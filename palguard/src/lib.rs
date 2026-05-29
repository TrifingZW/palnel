//! # Palguard — 外部进程守护器
//!
//! 管理外部服务器进程的完整生命周期，包括启动、停止、标准输入输出收集与状态监控。
//!
//! ## 设计原则
//!
//! - **幂等操作**：启动/停止操作可安全重复调用
//! - **超时保护**：所有阻塞操作均设置超时，防止死锁
//! - **优雅降级**：先尝试优雅退出，超时后强制终止
//! - **线程安全**：内部使用 `Arc<Mutex<>>`，可跨任务安全共享
//!
//! ## 使用示例
//!
//! ```ignore
//! use palguard::Palguard;
//!
//! let guard = Palguard::new(
//!     "/path/to/server".into(),
//!     "/path/to/workdir".into(),
//!     vec!["--port".into(), "8080".into()],
//! );
//!
//! guard.start().await?;
//! assert!(matches!(guard.status().await, ProcessStatus::Running { .. }));
//!
//! guard.stop().await?;
//! assert_eq!(guard.status().await, ProcessStatus::Stopped);
//! ```

use std::collections::VecDeque;
use std::fmt;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

/// 进程运行状态。
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProcessStatus {
    /// 未启动
    Stopped,
    /// 运行中，携带 PID 和启动时间戳
    Running {
        pid: u32,
        started_at: i64,
    },
    /// 异常退出，携带退出码
    Crashed {
        exit_code: Option<i32>,
    },
}

impl ProcessStatus {
    /// 是否处于运行中状态。
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running { .. })
    }
}

/// 进程输出行，带全局递增序号便于增量拉取。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OutputLine {
    pub index: u64,
    pub content: String,
}

struct Inner {
    executable_path: String,
    working_directory: String,
    args: Vec<String>,
    state: Mutex<State>,
    stdout_buf: Arc<Mutex<LineBuffer>>,
    stderr_buf: Arc<Mutex<LineBuffer>>,
}

struct State {
    child: Option<Child>,
    stdin: Option<ChildStdin>,
    status: ProcessStatus,
}

/// 固定容量的环形行缓冲区。
struct LineBuffer {
    lines: VecDeque<OutputLine>,
    next_index: u64,
    capacity: usize,
}

impl LineBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            lines: VecDeque::new(),
            next_index: 0,
            capacity,
        }
    }

    fn push(&mut self, content: String) {
        let line = OutputLine {
            index: self.next_index,
            content,
        };
        self.next_index = self.next_index.wrapping_add(1);
        self.lines.push_back(line);
        while self.lines.len() > self.capacity {
            self.lines.pop_front();
        }
    }

    fn read(&self, offset: u64, limit: u64) -> Vec<OutputLine> {
        self.lines.iter().skip_while(|l| l.index < offset).take(limit as usize).cloned().collect()
    }
}

/// 进程守护器——管理外部服务器进程的生命周期与标准输入输出。
///
/// `Clone` 实现基于内部 `Arc`，可安全注入到应用状态中跨请求共享。
#[derive(Clone)]
pub struct Palguard {
    inner: Arc<Inner>,
}

impl fmt::Debug for Palguard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Palguard")
            .field("executable_path", &self.inner.executable_path)
            .finish_non_exhaustive()
    }
}

impl Palguard {
    /// 创建新的守护器实例，进程初始状态为 [`ProcessStatus::Stopped`]。
    pub fn new(executable_path: String, working_directory: String, args: Vec<String>) -> Self {
        Self {
            inner: Arc::new(Inner {
                executable_path,
                working_directory,
                args,
                state: Mutex::new(State {
                    child: None,
                    stdin: None,
                    status: ProcessStatus::Stopped,
                }),
                stdout_buf: Arc::new(Mutex::new(LineBuffer::new(10_000))),
                stderr_buf: Arc::new(Mutex::new(LineBuffer::new(10_000))),
            }),
        }
    }

    /// 启动进程并开始异步收集 stdout / stderr。
    ///
    /// 若已在运行中则返回错误，保证操作**幂等**。
    pub async fn start(&self) -> anyhow::Result<()> {
        let mut state = self.inner.state.lock().await;

        if matches!(state.status, ProcessStatus::Running { .. }) {
            return Err(anyhow::anyhow!("服务器已在运行中"));
        }

        let exe_path = PathBuf::from(&self.inner.executable_path);
        if !exe_path.exists() {
            return Err(anyhow::anyhow!("可执行文件不存在: {}", self.inner.executable_path));
        }

        let mut cmd = Command::new(&self.inner.executable_path);
        cmd.args(&self.inner.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(false);

        if !self.inner.working_directory.is_empty() {
            cmd.current_dir(&self.inner.working_directory);
        }

        let mut child = cmd.spawn().map_err(|e| anyhow::anyhow!("无法启动进程: {e}"))?;

        let pid = child.id().expect("子进程必须有 PID");
        let started_at = time::OffsetDateTime::now_utc().unix_timestamp();

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let stdin = child.stdin.take();

        spawn_line_reader(stdout, self.inner.stdout_buf.clone(), "stdout");
        spawn_line_reader(stderr, self.inner.stderr_buf.clone(), "stderr");

        state.child = Some(child);
        state.stdin = stdin;
        state.status = ProcessStatus::Running {
            pid,
            started_at,
        };

        info!("进程已启动 (PID: {pid})");
        Ok(())
    }

    /// 停止进程——优雅退出 + 超时强制终止。
    ///
    /// 1) 关闭 stdin
    /// 2) 轮询 30 秒等待优雅退出
    /// 3) 超时后调用 OS 原生方式强制终止进程树
    /// 4) 再等待 5 秒确认
    ///
    /// 注意：取出子进程句柄后立即释放锁再执行等待，避免阻塞 status() 查询。
    pub async fn stop(&self) -> anyhow::Result<()> {
        const GRACE_PERIOD: Duration = Duration::from_secs(30);
        const KILL_PERIOD: Duration = Duration::from_secs(5);

        let (mut child, pid) = {
            let mut state = self.inner.state.lock().await;

            state.stdin.take();

            let c = match state.child.take() {
                Some(c) => c,
                None => {
                    state.status = ProcessStatus::Stopped;
                    return Err(anyhow::anyhow!("服务器未在运行"));
                }
            };

            let p = match &state.status {
                ProcessStatus::Running {
                    pid,
                    ..
                } => *pid,
                _ => 0,
            };

            (c, p)
        };

        let deadline = tokio::time::Instant::now() + GRACE_PERIOD;
        loop {
            match child.try_wait() {
                Ok(Some(exit_status)) => {
                    info!("进程已退出 (exit: {exit_status:?})");
                    let mut state = self.inner.state.lock().await;
                    state.status = ProcessStatus::Stopped;
                    return Ok(());
                }
                Ok(None) => {
                    if tokio::time::Instant::now() >= deadline {
                        warn!("优雅退出超时，强制终止进程");
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
                Err(e) => {
                    error!("检查进程状态失败: {e}");
                    break;
                }
            }
        }

        kill_process_tree(pid);

        match tokio::time::timeout(KILL_PERIOD, child.wait()).await {
            Ok(Ok(status)) => info!("进程已强制终止 (exit: {status:?})"),
            Ok(Err(e)) => error!("等待强制终止出错: {e}"),
            Err(_) => error!("强制终止超时，进程可能仍在运行"),
        }

        let mut state = self.inner.state.lock().await;
        state.status = ProcessStatus::Stopped;
        Ok(())
    }

    /// 重启进程——等价于 `stop()` 后 `start()`。
    ///
    /// 忽略停止阶段的错误以保证幂等性（进程可能已不在运行）。
    pub async fn restart(&self) -> anyhow::Result<()> {
        let _ = self.stop().await;
        self.start().await
    }

    /// 获取当前进程状态。
    ///
    /// 内部执行 `try_wait` 检测进程是否意外退出并自动更新为 [`ProcessStatus::Crashed`]。
    pub async fn status(&self) -> ProcessStatus {
        let mut state = self.inner.state.lock().await;
        Self::refresh_inner(&mut state);
        state.status.clone()
    }

    /// 向进程的标准输入写入一行文本（自动追加换行符）。
    pub async fn write_stdin(&self, input: String) -> anyhow::Result<()> {
        let mut state = self.inner.state.lock().await;

        let stdin =
            state.stdin.as_mut().ok_or_else(|| anyhow::anyhow!("进程未启动或标准输入已关闭"))?;

        stdin.write_all(input.as_bytes()).await?;
        stdin.write_all(b"\n").await?;

        Ok(())
    }

    /// 读取已收集的 stdout 行（按 `offset` 增量拉取）。
    pub async fn read_stdout(&self, offset: u64, limit: u64) -> Vec<OutputLine> {
        self.inner.stdout_buf.lock().await.read(offset, limit)
    }

    /// 读取已收集的 stderr 行（按 `offset` 增量拉取）。
    pub async fn read_stderr(&self, offset: u64, limit: u64) -> Vec<OutputLine> {
        self.inner.stderr_buf.lock().await.read(offset, limit)
    }

    fn refresh_inner(state: &mut State) {
        let child = match state.child.as_mut() {
            Some(c) => c,
            None => return,
        };

        match child.try_wait() {
            Ok(Some(exit_status)) => {
                state.child = None;
                state.stdin = None;
                state.status = ProcessStatus::Crashed {
                    exit_code: exit_status.code(),
                };
                warn!("进程意外退出 (exit: {exit_status:?})");
            }
            Ok(None) => {}
            Err(e) => {
                error!("检查进程状态失败: {e}");
            }
        }
    }
}

/// 使用操作系统最可靠的方式强制终止进程及其所有子进程。
///
/// - **Windows**: `taskkill /F /T /PID <pid>` 终止整个进程树
/// - **Unix**: `kill -9 <pid>` 发送不可捕获的 SIGKILL
fn kill_process_tree(pid: u32) {
    if pid == 0 {
        return;
    }

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let result = std::process::Command::new("taskkill")
            .args(["/F", "/T", "/PID", &pid.to_string()])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        match result {
            Ok(s) if s.success() => info!("taskkill 成功终止进程树 (PID: {pid})"),
            Ok(s) => warn!("taskkill 返回非零退出码 (PID: {pid}, code: {:?})", s.code()),
            Err(e) => error!("taskkill 执行失败 (PID: {pid}): {e}"),
        }
    }

    #[cfg(not(windows))]
    {
        let result = std::process::Command::new("kill")
            .args(["-9", &pid.to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        match result {
            Ok(s) if s.success() => info!("SIGKILL 已发送 (PID: {pid})"),
            Ok(s) => warn!("kill 命令返回非零退出码 (PID: {pid}, code: {:?})", s.code()),
            Err(e) => error!("kill 命令执行失败 (PID: {pid}): {e}"),
        }
    }
}

/// 启动异步任务，持续从管道读取行并追加到缓冲区。
fn spawn_line_reader(
    stream: Option<impl tokio::io::AsyncRead + Unpin + Send + 'static>,
    buffer: Arc<Mutex<LineBuffer>>,
    label: &'static str,
) {
    let stream = match stream {
        Some(s) => s,
        None => return,
    };

    tokio::spawn(async move {
        let mut reader = BufReader::new(stream).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            buffer.lock().await.push(line);
        }
        info!("{label} 管道已关闭");
    });
}
