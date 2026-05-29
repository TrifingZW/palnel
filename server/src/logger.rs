use std::path::PathBuf;

use time::macros::format_description;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{
    EnvFilter, Registry, fmt, fmt::time::LocalTime, layer::SubscriberExt, util::SubscriberInitExt,
};

/// 初始化全局日志记录器
///
/// 包含控制台标准输出与按日滚动的文件输出。
/// 必须在程序主入口点保留返回的 `WorkerGuard`，以确保异步日志在程序退出前刷入磁盘。
pub fn init() -> WorkerGuard {
    // 1) 配置本地时间格式化器
    // 提取本地电脑真实时区，剔除小数点，强制输出人类可读的本地时间
    let timer =
        LocalTime::new(format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"));

    // 2) 配置环境变量过滤器
    // 默认使用 info 级别，可通过 RUST_LOG 环境变量动态覆盖 (例如: RUST_LOG=debug)
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 3) 配置按日滚动的文件追加器
    // 日志将自动存储在 "logs" 目录中，文件名前缀设定为网站域名 "asdfri.cn.log"
    let log_dir = PathBuf::from("logs");
    let file_appender = rolling::daily(log_dir, "palnel.log");

    // 4) 配置非阻塞的文件写入器
    // 提取 guard 以确保后台日志写入线程的生命周期与主线程安全绑定
    let (non_blocking_appender, guard) = non_blocking(file_appender);

    // 5) 配置文件输出层
    // 禁用 ANSI 转义字符以保证日志文件的纯文本可读性，包含线程 ID 以利于并发调试
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_timer(timer.clone())
        .with_writer(non_blocking_appender);

    // 6) 配置控制台输出层
    // 启用 ANSI 转义字符以增强终端视觉效果，精简目标与线程 ID 输出保持整洁
    let console_layer = fmt::layer()
        .with_ansi(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_timer(timer)
        .with_writer(std::io::stdout);

    // 7) 注册并初始化全局订阅者
    // 组合过滤器、文件层与控制台层，构建完整的遥测数据收集管道
    Registry::default().with(env_filter).with(file_layer).with(console_layer).init();

    // 8) 记录系统启动初始信息
    // 验证日志系统是否成功接管应用输出
    tracing::info!("System log initialization complete");

    guard
}
