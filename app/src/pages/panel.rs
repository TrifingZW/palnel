use leptos::prelude::*;

use crate::{
    components::{
        avatar::AvatarArea,
        elevated_button::{ButtonSize, ButtonVariant, ElevatedButton},
        tab_bar::{PanelTab, TabBar},
        tag::{Tag, TagColor, TagSize},
    },
    views::{
        dashboard::DashboardView, map::MapView, palworld::PalWorldView, saves::SaveView,
        settings::SettingsView,
    },
};

/// 主面板，组合导航、状态栏、操作按钮及内容区。
#[component]
pub fn Panel() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(PanelTab::PalWorld);
    let (running, _set_running) = signal(true);
    let (pid, _set_pid) = signal(12345u32);

    let start_icon = view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path d="M4 2.5v11l9-5.5-9-5.5z"/>
        </svg>
    }.into_any();

    let stop_icon = view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <rect x="3" y="3" width="10" height="10" rx="1.5"/>
        </svg>
    }.into_any();

    let restart_icon = view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg">
            <path d="M2 8a6 6 0 0 1 10.47-4M14 8a6 6 0 0 1-10.47 4"/>
            <path d="M14 2v4h-4M2 14v-4h4"/>
        </svg>
    }.into_any();

    view! {
        <div class="panel">
            <header class="panel__header">
                <div class="panel__header-row">
                    <div class="panel__title-group">
                        <h1 class="panel__title">"Palnel"</h1>
                        <Tag text="v0.1.0"/>
                    </div>
                    <AvatarArea />
                </div>
                <div class="panel__header-row">
                    <TabBar active_tab=active_tab on_change=move |tab| set_active_tab.set(tab) />
                    <div class="panel__status">
                        {move || {
                            if running.get() {
                                view! {
                                    <span class="tag tag--lg tag--success">
                                        <span class="status-dot status-dot--pulse"></span>
                                        "运行中"
                                    </span>
                                }.into_any()
                            } else {
                                view! {
                                    <span class="tag tag--lg tag--danger">
                                        <span class="status-dot"></span>
                                        "已停止"
                                    </span>
                                }.into_any()
                            }
                        }}
                        {move || {
                            view! {
                                <Tag text=format!("PID: {}", pid.get()) size=TagSize::Large color=TagColor::Info />
                            }
                        }}
                    </div>
                    <div class="panel__actions">
                        <ElevatedButton label="启动" variant=ButtonVariant::Success size=ButtonSize::Large icon=start_icon on_click=move |_| {} />
                        <ElevatedButton label="关闭" variant=ButtonVariant::Danger size=ButtonSize::Large icon=stop_icon on_click=move |_| {} />
                        <ElevatedButton label="重启" variant=ButtonVariant::Accent size=ButtonSize::Large icon=restart_icon on_click=move |_| {} />
                    </div>
                </div>
            </header>
            <hr class="panel__divider" />
            <main class="panel__content">
                {move || match active_tab.get() {
                    PanelTab::PalWorld => view! { <PalWorldView /> }.into_any(),
                    PanelTab::Dashboard => view! { <DashboardView /> }.into_any(),
                    PanelTab::Saves => view! { <SaveView /> }.into_any(),
                    PanelTab::Map => view! { <MapView /> }.into_any(),
                    PanelTab::Settings => view! { <SettingsView /> }.into_any(),
                }}
            </main>
        </div>
    }
}
