use common::pal::PalInfo;
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
        </div>
    }
}
