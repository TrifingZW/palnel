use std::time::Duration;

use common::sysinfo::{DiskInfo, NetworkInfo, SystemMetrics};
use sysinfo::{Disks, Networks, System};

/// 执行 sysinfo 采集，返回聚合快照（由后台轮询任务调用）。
pub async fn collect_system_metrics() -> SystemMetrics {
    let mut sys = System::new_all();
    tokio::time::sleep(Duration::from_millis(200)).await;
    sys.refresh_cpu_all();

    let cpu_usage: f32;
    let cpu_cores: usize;
    let cpu_model: String;
    {
        let cpus = sys.cpus();
        cpu_usage = if cpus.is_empty() {
            0.0
        } else {
            (cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32 * 10.0).round()
                / 10.0
        };
        cpu_model = cpus.first().map(|c| c.brand().to_string()).unwrap_or_default();
        cpu_cores = cpus.len();
    }

    sys.refresh_memory();
    let memory_total = sys.total_memory();
    let memory_used = sys.used_memory();
    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();

    let disks = Disks::new_with_refreshed_list();
    let disk_list: Vec<DiskInfo> = disks
        .list()
        .iter()
        .map(|d| DiskInfo {
            name: d.name().to_string_lossy().into_owned(),
            mount_point: d.mount_point().to_string_lossy().into_owned(),
            total: d.total_space(),
            used: d.total_space().saturating_sub(d.available_space()),
        })
        .collect();

    let networks = Networks::new_with_refreshed_list();
    let net_list: Vec<NetworkInfo> = networks
        .iter()
        .map(|(name, data)| NetworkInfo {
            name: name.clone(),
            rx_bytes: data.received(),
            tx_bytes: data.transmitted(),
        })
        .collect();

    let load_avg = System::load_average();
    let collected_at = {
        let now = time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc());
        format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
    };

    SystemMetrics {
        cpu_usage: cpu_usage.clamp(0.0, 100.0),
        cpu_cores,
        cpu_model,
        memory_total,
        memory_used,
        swap_total,
        swap_used,
        uptime: System::uptime(),
        load_avg_one: (load_avg.one * 100.0).round() / 100.0,
        load_avg_five: (load_avg.five * 100.0).round() / 100.0,
        load_avg_fifteen: (load_avg.fifteen * 100.0).round() / 100.0,
        disks: disk_list,
        networks: net_list,
        process_count: sys.processes().len(),
        os_name: System::name().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
        kernel_version: System::kernel_version().unwrap_or_default(),
        hostname: System::host_name().unwrap_or_default(),
        collected_at,
    }
}
