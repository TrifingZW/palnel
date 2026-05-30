use common::pal::{PalInfo, PalMetrics};
use leptos::prelude::*;

use crate::components::{
    card::{Card, CardVariant},
    tag::{Tag, TagColor, TagSize},
};

/// 帕鲁世界视图，展示服务器基本信息与实时指标。
#[component]
pub fn PalWorldView() -> impl IntoView {
    let info = PalInfo {
        version: "v0.4.11".into(),
        server_name: "My Palworld Server".into(),
        description: "PVE 休闲服".into(),
        world_guid: "A1B2C3D4-E5F6-7890-ABCD-EF1234567890".into(),
    };

    let metrics = PalMetrics {
        server_fps: 60,
        current_player_num: 8,
        max_player_num: 32,
        server_frame_time: 16.67,
        uptime: 97_200,
        base_camp_num: 5,
        days: 42,
    };

    let version_icon = view! {
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zM8 2a6 6 0 1 0 0 12A6 6 0 0 0 8 2z"/>
            <path d="M9.5 5.5a.5.5 0 0 1 0 1h-3a.5.5 0 0 1 0-1h3zM9.5 7.5a.5.5 0 0 1 0 1h-3a.5.5 0 0 1 0-1h3zM6.5 9.5a.5.5 0 0 1 0 1h1a.5.5 0 0 1 0-1h-1z"/>
        </svg>
    }.into_any();

    let globe_icon = view! {
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1zM2.52 5h2.3a10.6 10.6 0 0 1 1.37 7H3.27A6 6 0 0 1 2.52 5zm8.66 0h2.3a6 6 0 0 1-2.92 7h-0.75a10.6 10.6 0 0 0 1.37-7zM6.19 5h3.62a9.57 9.57 0 0 0-1.24 7H7.43A9.57 9.57 0 0 0 6.19 5zM8 2a6 6 0 0 1 4.26 2H3.74A6 6 0 0 1 8 2zm0 12a6 6 0 0 1-4.26-2h8.52A6 6 0 0 1 8 14z"/>
        </svg>
    }.into_any();

    let fps_icon = view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1zM2 8a6 6 0 1 1 12 0A6 6 0 0 1 2 8z"/>
            <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z"/>
        </svg>
    }.into_any();

    let players_icon = view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M7 14s-1 0-1-1 1-4 5-4 5 3 5 4-1 1-1 1H7zm4-6a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"/>
            <path fill-rule="evenodd" d="M5.216 14A2.238 2.238 0 0 1 5 13c0-1.355.68-2.75 1.936-3.72A6.325 6.325 0 0 0 5 9c-4 0-5 3-5 4s1 1 1 1h4.216z"/>
            <path d="M4.5 8a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5z"/>
        </svg>
    }.into_any();

    let clock_icon = view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zM8 2a6 6 0 1 0 0 12A6 6 0 0 0 8 2z"/>
            <path d="M7.5 4a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 1 .5-.5zM8 11a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5z"/>
        </svg>
    }.into_any();

    let camp_icon = view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M6.5 14.5v-4h3v4h3.5l-5 5-5-5h3.5z"/>
            <path d="M5.5 7V2h-3l5.5 5.5L13.5 2h-3v5H14l-6 6-6-6h3.5z"/>
        </svg>
    }
    .into_any();

    let days_icon = view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M3.5 0a.5.5 0 0 1 .5.5V1h8V.5a.5.5 0 0 1 1 0V1h1a2 2 0 0 1 2 2v11a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2h1V.5a.5.5 0 0 1 .5-.5zM1 4v10a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V4H1z"/>
        </svg>
    }.into_any();

    let frame_icon = view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M0 1a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H1a1 1 0 0 1-1-1V1zm4 3.5a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7a.5.5 0 0 1-.5-.5zm0 2a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5zm0 2a.5.5 0 0 1 .5-.5h6a.5.5 0 0 1 0 1h-6a.5.5 0 0 1-.5-.5z"/>
        </svg>
    }.into_any();

    fn format_uptime(secs: u64) -> String {
        let d = secs / 86_400;
        let h = (secs % 86_400) / 3_600;
        if d > 0 {
            format!("{}d {}h", d, h)
        } else {
            format!("{}h", h)
        }
    }

    let metric_cards: [(String, &str, &str, AnyView); 6] = [
        (metrics.server_fps.to_string(), "FPS", "pink", fps_icon),
        (
            format!("{} / {}", metrics.current_player_num, metrics.max_player_num),
            "玩家",
            "purple",
            players_icon,
        ),
        (format!("{:.1} ms", metrics.server_frame_time), "帧时间", "teal", frame_icon),
        (format_uptime(metrics.uptime), "运行时长", "success", clock_icon),
        (metrics.days.to_string(), "游戏天数", "warning", days_icon),
        (metrics.base_camp_num.to_string(), "基地数", "info", camp_icon),
    ];

    view! {
        <div class="palworld">
            <Card variant=CardVariant::Banner class={ "palbar".to_string() }>
                <div class="palbar__row">
                    <h2 class="palbar__title">{info.server_name.clone()}</h2>
                    <Tag text=info.version.clone() size=TagSize::Small color=TagColor::Purple icon=version_icon />
                    <Tag text=info.world_guid[..8].to_string() size=TagSize::Small color=TagColor::Teal icon=globe_icon />
                </div>
                <p class="palbar__desc">{info.description.clone()}</p>
            </Card>

            <div class="palmetrics">
                {metric_cards.into_iter().map(|(value, label, accent, icon)| {
                    view! {
                        <Card class={ format!("palmetrics__card palmetrics__card--{}", accent) }>
                            <span class="palmetrics__icon">{icon}</span>
                            <span class="palmetrics__value">{value}</span>
                            <span class="palmetrics__label">{label}</span>
                        </Card>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
