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

/// 概览视图，展示系统信息栏、CPU 与内存状态。
#[component]
pub fn OverviewView() -> impl IntoView {
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
        <div class="overview">
            <Card variant=CardVariant::Banner class={ "sysbar".to_string() }>
                <Tag text=metrics.os_name.clone() size=TagSize::Small color=TagColor::Accent icon=os_icon />
                <Tag text=metrics.os_version.clone() size=TagSize::Small color=TagColor::Purple icon=ver_icon />
                <Tag text=metrics.kernel_version.clone() size=TagSize::Small color=TagColor::Teal icon=krn_icon />
                <Tag text=metrics.hostname.clone() size=TagSize::Small color=TagColor::Success icon=host_icon />
                <Tag text=metrics.collected_at.clone() size=TagSize::Small color=TagColor::Warning icon=time_icon />
            </Card>

            <div class="overview__grid">
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
        </div>
    }
}
