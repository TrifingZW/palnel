use serde::{Deserialize, Serialize};

/// 系统核心指标的聚合快照。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub cpu_cores: usize,
    pub cpu_model: String,
    pub memory_total: u64,
    pub memory_used: u64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub uptime: u64,
    pub load_avg_one: f64,
    pub load_avg_five: f64,
    pub load_avg_fifteen: f64,
    pub disks: Vec<DiskInfo>,
    pub networks: Vec<NetworkInfo>,
    pub process_count: usize,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,
    /// 数据采集时刻 (HH:MM:SS 本地时间)。
    pub collected_at: String,
}

/// 单块磁盘的容量与使用信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
}

/// 单张网卡的收发统计。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NetworkInfo {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}
