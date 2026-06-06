use common::pal::{PalInfo, PalMetrics, PalPlayerList};
use leptos::prelude::*;

use crate::components::{
    card::{Card, CardVariant},
    elevated_button::{ButtonSize, ButtonVariant, ElevatedButton},
    pal_player_list::PalPlayerList,
    tag::{Tag, TagColor, TagSize},
};

fn version_icon() -> AnyView {
    view! {
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zM8 2a6 6 0 1 0 0 12A6 6 0 0 0 8 2z"/>
            <path d="M9.5 5.5a.5.5 0 0 1 0 1h-3a.5.5 0 0 1 0-1h3zM9.5 7.5a.5.5 0 0 1 0 1h-3a.5.5 0 0 1 0-1h3zM6.5 9.5a.5.5 0 0 1 0 1h1a.5.5 0 0 1 0-1h-1z"/>
        </svg>
    }.into_any()
}

fn globe_icon() -> AnyView {
    view! {
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1zM2.52 5h2.3a10.6 10.6 0 0 1 1.37 7H3.27A6 6 0 0 1 2.52 5zm8.66 0h2.3a6 6 0 0 1-2.92 7h-0.75a10.6 10.6 0 0 0 1.37-7zM6.19 5h3.62a9.57 9.57 0 0 0-1.24 7H7.43A9.57 9.57 0 0 0 6.19 5zM8 2a6 6 0 0 1 4.26 2H3.74A6 6 0 0 1 8 2zm0 12a6 6 0 0 1-4.26-2h8.52A6 6 0 0 1 8 14z"/>
        </svg>
    }.into_any()
}

fn fps_icon() -> AnyView {
    view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1zM2 8a6 6 0 1 1 12 0A6 6 0 0 1 2 8z"/>
            <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z"/>
        </svg>
    }.into_any()
}

fn players_icon() -> AnyView {
    view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M7 14s-1 0-1-1 1-4 5-4 5 3 5 4-1 1-1 1H7zm4-6a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"/>
            <path fill-rule="evenodd" d="M5.216 14A2.238 2.238 0 0 1 5 13c0-1.355.68-2.75 1.936-3.72A6.325 6.325 0 0 0 5 9c-4 0-5 3-5 4s1 1 1 1h4.216z"/>
            <path d="M4.5 8a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5z"/>
        </svg>
    }.into_any()
}

fn clock_icon() -> AnyView {
    view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zM8 2a6 6 0 1 0 0 12A6 6 0 0 0 8 2z"/>
            <path d="M7.5 4a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 1 .5-.5zM8 11a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5z"/>
        </svg>
    }.into_any()
}

fn camp_icon() -> AnyView {
    view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M6.5 14.5v-4h3v4h3.5l-5 5-5-5h3.5z"/>
            <path d="M5.5 7V2h-3l5.5 5.5L13.5 2h-3v5H14l-6 6-6-6h3.5z"/>
        </svg>
    }
    .into_any()
}

fn days_icon() -> AnyView {
    view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M3.5 0a.5.5 0 0 1 .5.5V1h8V.5a.5.5 0 0 1 1 0V1h1a2 2 0 0 1 2 2v11a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2h1V.5a.5.5 0 0 1 .5-.5zM1 4v10a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V4H1z"/>
        </svg>
    }.into_any()
}

fn frame_icon() -> AnyView {
    view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M0 1a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H1a1 1 0 0 1-1-1V1zm4 3.5a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7a.5.5 0 0 1-.5-.5zm0 2a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5zm0 2a.5.5 0 0 1 .5-.5h6a.5.5 0 0 1 0 1h-6a.5.5 0 0 1-.5-.5z"/>
        </svg>
    }.into_any()
}

fn format_uptime(secs: u64) -> String {
    let d = secs / 86_400;
    let h = (secs % 86_400) / 3_600;
    if d > 0 {
        format!("{}d {}h", d, h)
    } else {
        format!("{}h", h)
    }
}

/// 帕鲁世界视图，展示服务器基本信息与实时指标。
#[component]
pub fn PalWorldView(
    pal_info: RwSignal<PalInfo>,
    pal_metrics: RwSignal<PalMetrics>,
    pal_player_list: RwSignal<PalPlayerList>,
) -> impl IntoView {
    view! {
        <div class="palworld">
            <Card variant=CardVariant::Banner class={ "palbar".to_string() }>
                {move || {
                    let info = pal_info.get();
                    view! {
                        <div class="palbar__row">
                            <h2 class="palbar__title">{info.server_name.clone()}</h2>
                            <Tag text=info.version.clone() size=TagSize::Small color=TagColor::Purple icon=version_icon() />
                            <Tag text=info.world_guid[..8.min(info.world_guid.len())].to_string() size=TagSize::Small color=TagColor::Teal icon=globe_icon() />
                        </div>
                        <p class="palbar__desc">{info.description.clone()}</p>
                    }
                }}
            </Card>

            <div class="palmetrics">
                {move || {
                    let m = pal_metrics.get();
                    [
                        (m.server_fps.to_string(), "FPS", "pink", fps_icon()),
                        (
                            format!("{} / {}", m.current_player_num, m.max_player_num),
                            "玩家",
                            "purple",
                            players_icon(),
                        ),
                        (format!("{:.1} ms", m.server_frame_time), "帧时间", "teal", frame_icon()),
                        (format_uptime(m.uptime), "运行时长", "success", clock_icon()),
                        (m.days.to_string(), "游戏天数", "warning", days_icon()),
                        (m.base_camp_num.to_string(), "基地数", "info", camp_icon()),
                    ].into_iter().map(|(value, label, accent, icon)| {
                        view! {
                            <Card class={ format!("palmetrics__card palmetrics__card--{}", accent) }>
                                <span class="palmetrics__icon">{icon}</span>
                                <span class="palmetrics__value">{value}</span>
                                <span class="palmetrics__label">{label}</span>
                            </Card>
                        }
                    }).collect_view()
                }}
            </div>

            <div class="palworld__body">
                <PalPlayerList players=move || pal_player_list.get().players />
            </div>

            <div class="palworld__footer">
                {
                    let announce_icon = view! {
                        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                            <path d="M13 2.5a1.5 1.5 0 0 1 3 0v11a1.5 1.5 0 0 1-3 0v-.214c-2.162-1.241-4.49-1.843-6.912-2.083l.405 2.712A1 1 0 0 1 5.51 15.1h-.548a1 1 0 0 1-.916-.599l-1.85-3.49a68.14 68.14 0 0 0-.202-.003A2.014 2.014 0 0 1 0 9V7a2.02 2.02 0 0 1 1.992-2.013 74.663 74.663 0 0 0 2.483-.075c3.043-.154 6.148-.849 8.525-2.199V2.5zm1 0v11a.5.5 0 0 0 1 0v-11a.5.5 0 0 0-1 0zm-1 1.35c-2.344 1.205-5.209 1.842-8 2.033v4.233c.18.01.359.022.537.036 2.568.189 5.093.744 7.463 1.993V3.85zm-9 6.215v-4.13a95.09 95.09 0 0 1-1.992.052A1.02 1.02 0 0 0 1 7v2c0 .55.448 1.002 1.006 1.009A60.49 60.49 0 0 1 4 10.065zm-.657.975 1.609 3.037.01.024h.548l-.002-.014-.443-2.966a68.019 68.019 0 0 0-1.722-.082z"/>
                        </svg>
                    }.into_any();
                    view! { <ElevatedButton label="公告".to_string() loading_label="".to_string() variant=ButtonVariant::Accent size=ButtonSize::Large icon=announce_icon on_click=move |_| {} /> }
                }
                {
                    let save_icon = view! {
                        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                            <path d="M2 1a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H9.5a1 1 0 0 0-1 1v7.293l2.646-2.647a.5.5 0 0 1 .708.708l-3.5 3.5a.5.5 0 0 1-.708 0l-3.5-3.5a.5.5 0 1 1 .708-.708L7.5 9.293V2a2 2 0 0 1 2-2H14a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V2a2 2 0 0 1 2-2h2.5a.5.5 0 0 1 0 1H2z"/>
                        </svg>
                    }.into_any();
                    view! { <ElevatedButton label="存档".to_string() loading_label="".to_string() variant=ButtonVariant::Accent size=ButtonSize::Large icon=save_icon on_click=move |_| {} /> }
                }
                {
                    let shutdown_icon = view! {
                        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                            <path d="M7.938 1.016A7.04 7.04 0 0 0 1.242 3.55a6.972 6.972 0 0 0 3.477 11.867.5.5 0 0 0 .144-.99 5.972 5.972 0 0 1-2.98-10.174 6.04 6.04 0 0 1 5.739-2.174.5.5 0 0 0 .627-.462.5.5 0 0 0-.311-.6z"/>
                            <path d="M5.5 3.62a.5.5 0 0 1 .655.257 5.968 5.968 0 0 0 7.017 3.234.5.5 0 0 1 .585.585 7.044 7.044 0 0 1-4.61 6.03.5.5 0 0 1-.344-.94 6.044 6.044 0 0 0 3.953-5.172 6.968 6.968 0 0 1-7.513-6.339.5.5 0 0 1 .257-.655z"/>
                            <path d="M8 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0v-6A.5.5 0 0 1 8 0z"/>
                        </svg>
                    }.into_any();
                    view! { <ElevatedButton label="关服".to_string() loading_label="".to_string() variant=ButtonVariant::Danger size=ButtonSize::Large icon=shutdown_icon on_click=move |_| {} /> }
                }
                {
                    let force_icon = view! {
                        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                            <path d="M5.5 3.62a.5.5 0 0 1 .655.257 5.968 5.968 0 0 0 7.017 3.234.5.5 0 0 1 .585.585 7.044 7.044 0 0 1-4.61 6.03.5.5 0 0 1-.344-.94 6.044 6.044 0 0 0 3.953-5.172 6.968 6.968 0 0 1-7.513-6.339.5.5 0 0 1 .257-.655z"/>
                            <path d="M8 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0v-6A.5.5 0 0 1 8 0z"/>
                            <path d="M12.5 16a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7zm.354-5.354-1.5 1.5a.5.5 0 0 0 .708.708l.438-.438V14a.5.5 0 0 0 1 0v-1.584l.438.438a.5.5 0 0 0 .708-.708l-1.5-1.5a.5.5 0 0 0-.708 0z"/>
                            <path d="M8.5 11.5a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 0 1H9a.5.5 0 0 1-.5-.5z"/>
                        </svg>
                    }.into_any();
                    view! { <ElevatedButton label="强停".to_string() loading_label="".to_string() variant=ButtonVariant::Danger size=ButtonSize::Large icon=force_icon on_click=move |_| {} /> }
                }
            </div>
        </div>
    }
}
