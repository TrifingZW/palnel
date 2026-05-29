use common::sysinfo::SystemMetrics;
use leptos::prelude::*;

use crate::components::{
    bar_progress::BarProgress,
    card::{Card, CardVariant},
    circular_progress::CircularProgress,
    tag::{Tag, TagColor, TagSize},
};

fn format_gb(bytes: u64) -> String {
    format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
}

fn format_rate(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{:.1} KB", bytes as f64 / 1_024.0)
    } else {
        format!("{} B", bytes)
    }
}

/// 仪表盘视图，展示系统信息、CPU、内存、网络与磁盘状态。
#[component]
pub fn DashboardView() -> impl IntoView {
    let metrics = SystemMetrics {
        cpu_usage: 0.23,
        cpu_cores: 16,
        cpu_model: "AMD Ryzen 9 5950X 16-Core Processor".into(),
        memory_total: 17_179_869_184,
        memory_used: 9_437_000_000,
        swap_total: 8_589_934_592,
        swap_used: 2_147_000_000,
        os_name: "Windows 11 Pro".into(),
        os_version: "10.0.22621".into(),
        kernel_version: "NT 10.0.22621".into(),
        hostname: "DESKTOP-A1B2C3D".into(),
        collected_at: "14:30:00".into(),
        networks: vec![
            common::sysinfo::NetworkInfo {
                name: "eth0".into(),
                rx_bytes: 45_823_456_789,
                tx_bytes: 12_345_678_901,
            },
            common::sysinfo::NetworkInfo {
                name: "eth1".into(),
                rx_bytes: 8_456_789_012,
                tx_bytes: 3_210_987_654,
            },
            common::sysinfo::NetworkInfo {
                name: "wlan0".into(),
                rx_bytes: 234_567_890_123,
                tx_bytes: 98_765_432_109,
            },
        ],
        disks: vec![
            common::sysinfo::DiskInfo {
                name: "C:".into(),
                mount_point: "/mnt/c".into(),
                total: 512_110_190_592,
                used: 287_000_000_000,
            },
            common::sysinfo::DiskInfo {
                name: "D:".into(),
                mount_point: "/mnt/d".into(),
                total: 1_099_511_627_776,
                used: 654_000_000_000,
            },
        ],
        ..Default::default()
    };

    let (cpu_usage, _set_cpu_usage) = signal(metrics.cpu_usage);
    let (mem_usage, _set_mem_usage) =
        signal(metrics.memory_used as f32 / metrics.memory_total as f32);
    let (swap_usage, _set_swap_usage) = signal(if metrics.swap_total > 0 {
        metrics.swap_used as f32 / metrics.swap_total as f32
    } else {
        0.0
    });

    let os_icon = view! {
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
            <path d="M1 3.5A1.5 1.5 0 0 1 2.5 2h11A1.5 1.5 0 0 1 15 3.5v8a1.5 1.5 0 0 1-1.5 1.5h-11A1.5 1.5 0 0 1 1 11.5v-8zM2.5 3a.5.5 0 0 0-.5.5v8a.5.5 0 0 0 .5.5h11a.5.5 0 0 0 .5-.5v-8a.5.5 0 0 0-.5-.5h-11z"/>
        </svg>
    }.into_any();

    let ver_icon = view! {
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1zM2 8a6 6 0 1 1 12 0A6 6 0 0 1 2 8z"/>
            <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z"/>
        </svg>
    }.into_any();

    let krn_icon = view! {
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
            <path d="M4 4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2H4zm0 1h8a1 1 0 0 1 1 1v4a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1z"/>
            <path d="M5 6.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5z"/>
        </svg>
    }.into_any();

    let host_icon = view! {
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 1a3 3 0 1 0 0 6 3 3 0 0 0 0-6zM4.5 7A2.5 2.5 0 0 0 2 9.5V14h12V9.5A2.5 2.5 0 0 0 11.5 7h-7z"/>
        </svg>
    }.into_any();

    let time_icon = view! {
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zM8 2a6 6 0 1 0 0 12A6 6 0 0 0 8 2z"/>
            <path d="M7.5 4a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 1 .5-.5zM8 11a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5z"/>
        </svg>
    }.into_any();

    view! {
        <div class="dashboard">
            <Card variant=CardVariant::Banner class={ "sysbar".to_string() }>
                <Tag text=metrics.os_name.clone() size=TagSize::Small color=TagColor::Accent icon=os_icon />
                <Tag text=metrics.os_version.clone() size=TagSize::Small color=TagColor::Purple icon=ver_icon />
                <Tag text=metrics.kernel_version.clone() size=TagSize::Small color=TagColor::Teal icon=krn_icon />
                <Tag text=metrics.hostname.clone() size=TagSize::Small color=TagColor::Success icon=host_icon />
                <Tag text=metrics.collected_at.clone() size=TagSize::Small color=TagColor::Warning icon=time_icon />
            </Card>

            <div class="dashboard__grid">
                <Card class={ "cpu-card".to_string() }>
                    <CircularProgress value=cpu_usage size=120 stroke_width=10 />
                    <Tag
                        text=format!("{} 核心", metrics.cpu_cores)
                        size=TagSize::Small
                        color=TagColor::Info
                    />
                    <span class="cpu-card__model">{metrics.cpu_model.clone()}</span>
                </Card>

                <Card class={ "memory-card".to_string() }>
                    <div class="memory-card__section">
                        <span class="memory-card__label">
                            <span class="memory-card__accent memory-card__accent--success"></span>
                            "内存"
                        </span>
                        <BarProgress value=mem_usage fill_color="var(--color-success)".to_string() />
                        <span class="memory-card__stat">
                            {format!("{} / {}", format_gb(metrics.memory_used), format_gb(metrics.memory_total))}
                        </span>
                    </div>
                    <hr class="memory-card__divider" />
                    <div class="memory-card__section">
                        <span class="memory-card__label">
                            <span class="memory-card__accent memory-card__accent--warning"></span>
                            "交换区"
                        </span>
                        <BarProgress value=swap_usage fill_color="var(--color-warning)".to_string() />
                        <span class="memory-card__stat">
                            {format!("{} / {}", format_gb(metrics.swap_used), format_gb(metrics.swap_total))}
                        </span>
                    </div>
                </Card>
            </div>

            <Card class={ "network-card".to_string() }>
                {metrics.networks.iter().enumerate().map(|(i, net)| {
                    let variant = match i % 3 {
                        0 => "info",
                        1 => "teal",
                        _ => "success",
                    };
                    let class = format!("network-card__iface network-card__iface--{}", variant);
                    view! {
                        <div class=class>
                            <span class="network-card__name">{net.name.clone()}</span>
                            <div class="network-card__traffic">
                                <Tag text=format_rate(net.rx_bytes) size=TagSize::Small color=TagColor::Success
                                    icon=view! {
                                        <svg width="10" height="10" viewBox="0 0 16 16" fill="currentColor"><path d="M8 12L3 7h3V2h4v5h3l-5 5z"/></svg>
                                    }.into_any()
                                />
                                <Tag text=format_rate(net.tx_bytes) size=TagSize::Small color=TagColor::Accent
                                    icon=view! {
                                        <svg width="10" height="10" viewBox="0 0 16 16" fill="currentColor"><path d="M8 4l5 5h-3v5H6V9H3l5-5z"/></svg>
                                    }.into_any()
                                />
                            </div>
                        </div>
                    }
                }).collect_view()}
            </Card>

            <Card class={ "disk-card".to_string() }>
                {metrics.disks.iter().enumerate().map(|(i, disk)| {
                    let usage = if disk.total > 0 {
                        disk.used as f32 / disk.total as f32
                    } else {
                        0.0
                    };
                    let (usage_signal, _) = signal(usage);
                    let variant = match i % 4 {
                        0 => "info",
                        1 => "teal",
                        2 => "success",
                        _ => "warning",
                    };
                    let item_class = format!("disk-card__item disk-card__item--{}", variant);
                    let bar_class = format!("disk-card__bar disk-card__bar--{}", variant);
                    view! {
                        <div class=item_class>
                            <span class=bar_class></span>
                            <div class="disk-card__body">
                                <div class="disk-card__header">
                                    <span class="disk-card__name">
                                        <span class="disk-card__icon">
                                            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor"><path d="M2 3a1 1 0 0 1 1-1h3.586a1 1 0 0 1 .707.293l1.414 1.414A1 1 0 0 0 9.414 4H13a1 1 0 0 1 1 1v1H2V3zm0 3h12v6a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V6z"/></svg>
                                        </span>
                                        {disk.name.clone()}
                                    </span>
                                    <span class="disk-card__pct">{format!("{:.0}%", usage * 100.0)}</span>
                                </div>
                                <BarProgress value=usage_signal fill_color="var(--color-accent)".to_string() />
                                <div class="disk-card__stat">
                                    <span>{format_gb(disk.used)}</span>
                                    <span>{disk.mount_point.clone()}</span>
                                    <span>{format_gb(disk.total)}</span>
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </Card>
        </div>
    }
}
