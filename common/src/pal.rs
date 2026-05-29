use serde::{Deserialize, Serialize};

/// Palworld 服务器进程状态，由 Palguard 原生监控并通过 SSE 推送至前端。
///
/// 该类型在 client / server 两侧均可序列化，避免直接依赖 `palguard` crate。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PalguardProcessStatus {
    /// 未启动
    Stopped,
    /// 运行中
    Running {
        pid: u32,
        /// Unix 时间戳 (秒)
        started_at: i64,
    },
    /// 异常退出
    Crashed {
        exit_code: Option<i32>,
    },
}

impl Default for PalguardProcessStatus {
    fn default() -> Self {
        Self::Stopped
    }
}

#[cfg(feature = "ssr")]
impl From<palguard::ProcessStatus> for PalguardProcessStatus {
    fn from(value: palguard::ProcessStatus) -> Self {
        match value {
            palguard::ProcessStatus::Stopped => Self::Stopped,
            palguard::ProcessStatus::Running {
                pid,
                started_at,
            } => Self::Running {
                pid,
                started_at,
            },
            palguard::ProcessStatus::Crashed {
                exit_code,
            } => Self::Crashed {
                exit_code,
            },
        }
    }
}

/// 服务器基本信息
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct PalInfo {
    pub version: String,
    pub server_name: String,
    pub description: String,
    pub world_guid: String,
}

/// 服务器实时运行指标
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PalMetrics {
    pub server_fps: u32,
    pub current_player_num: u32,
    pub max_player_num: u32,
    pub server_frame_time: f64,
    pub uptime: u64,
    pub base_camp_num: u32,
    pub days: u32,
}

/// 玩家详细信息
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PalPlayer {
    pub name: String,
    pub account_name: String,
    pub player_id: String,
    pub user_id: String,
    pub ip: String,
    pub ping: f64,
    pub location_x: f64,
    pub location_y: f64,
    pub level: u32,
    pub building_count: u32,
}

/// 玩家列表
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PalPlayerList {
    pub players: Vec<PalPlayer>,
}
