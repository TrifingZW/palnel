use common::pal::PalguardProcessStatus;
use leptos::prelude::*;
use rpc::palguard::{PalguardRestart, PalguardStart, PalguardStop};

use crate::{
    components::{
        avatar::AvatarArea,
        elevated_button::{ButtonSize, ButtonVariant, ElevatedButton},
        tab_bar::{PanelTab, TabBar},
        tag::{Tag, TagColor, TagSize},
    },
    sse::create_sse,
    views::{
        dashboard::DashboardView, map::MapView, palworld::PalWorldView, saves::SaveView,
        settings::SettingsView,
    },
};

/// 主面板，组合导航、状态栏、操作按钮及内容区。
#[component]
pub fn Panel() -> impl IntoView {
    let sse = create_sse();
    let (active_tab, set_active_tab) = signal(PanelTab::PalWorld);

    let status = Memo::new(move |_| match sse.palguard_process.get() {
        PalguardProcessStatus::Stopped => "未启动".to_string(),
        PalguardProcessStatus::Running {
            ..
        } => "运行中".to_string(),
        PalguardProcessStatus::Crashed {
            ..
        } => "异常退出".to_string(),
    });

    let status_color = Memo::new(move |_| match sse.palguard_process.get() {
        PalguardProcessStatus::Stopped => TagColor::Danger,
        PalguardProcessStatus::Running {
            ..
        } => TagColor::Success,
        PalguardProcessStatus::Crashed {
            ..
        } => TagColor::Warning,
    });

    let status_icon = Memo::new(move |_| match sse.palguard_process.get() {
        PalguardProcessStatus::Running {
            ..
        } => "status-dot status-dot--pulse".to_string(),
        _ => "status-dot".to_string(),
    });

    let sub_status = Memo::new(move |_| match sse.palguard_process.get() {
        PalguardProcessStatus::Stopped => "PID: 0".to_string(),
        PalguardProcessStatus::Running {
            pid,
            ..
        } => format!("PID: {}", pid),
        PalguardProcessStatus::Crashed {
            exit_code,
            ..
        } => format!("ExitCode: {}", exit_code.unwrap_or_default()),
    });

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

    // 1) 三个 ServerAction，分别对应启动、关闭、重启
    let start_action = ServerAction::<PalguardStart>::new();
    let stop_action = ServerAction::<PalguardStop>::new();
    let restart_action = ServerAction::<PalguardRestart>::new();

    // 2) 标记提交完成但 SSE 尚未反馈的等待阶段
    let (awaiting_sse, set_awaiting_sse) = signal(false);

    // 3) 消费 action 返回值：仅在提交成功时进入等待 SSE 阶段
    Effect::new(move || {
        if let Some(Ok(())) = start_action.value().get() {
            start_action.value().set(None);
            set_awaiting_sse.set(true);
        } else if start_action.value().get().is_some() {
            start_action.value().set(None);
        }
    });
    Effect::new(move || {
        if let Some(Ok(())) = stop_action.value().get() {
            stop_action.value().set(None);
            set_awaiting_sse.set(true);
        } else if stop_action.value().get().is_some() {
            stop_action.value().set(None);
        }
    });
    Effect::new(move || {
        if let Some(Ok(())) = restart_action.value().get() {
            restart_action.value().set(None);
            set_awaiting_sse.set(true);
        } else if restart_action.value().get().is_some() {
            restart_action.value().set(None);
        }
    });

    // 4) SSE 状态变更时解除等待
    Effect::new(move || {
        sse.palguard_process.track();
        set_awaiting_sse.set(false);
    });

    // 5) 各按钮的禁用逻辑：已达到目标状态 或 其他操作进行中
    let start_loading = move || start_action.pending().get();
    let stop_loading = move || stop_action.pending().get();
    let restart_loading = move || restart_action.pending().get();

    let start_disabled = Signal::derive(move || {
        matches!(sse.palguard_process.get(), PalguardProcessStatus::Running { .. })
            || stop_action.pending().get()
            || restart_action.pending().get()
            || awaiting_sse.get()
    });

    let stop_disabled = Signal::derive(move || {
        !matches!(sse.palguard_process.get(), PalguardProcessStatus::Running { .. })
            || start_action.pending().get()
            || restart_action.pending().get()
            || awaiting_sse.get()
    });

    let restart_disabled = Signal::derive(move || {
        matches!(sse.palguard_process.get(), PalguardProcessStatus::Stopped)
            || start_action.pending().get()
            || stop_action.pending().get()
            || awaiting_sse.get()
    });

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
                        <Tag text=status size=TagSize::Large color=status_color icon=view! {
                            <span class=status_icon />
                        }.into_any()/>
                        <Tag text=sub_status size=TagSize::Large color=TagColor::Info />
                    </div>
                    <div class="panel__actions">
                        <ActionForm action=start_action>
                            <ElevatedButton
                                label="启动"
                                loading_label="启动中…"
                                variant=ButtonVariant::Success
                                size=ButtonSize::Large
                                icon=start_icon
                                disabled=start_disabled
                                loading=start_loading
                                button_type="submit"
                            />
                        </ActionForm>
                        <ActionForm action=stop_action>
                            <ElevatedButton
                                label="关闭"
                                loading_label="关闭中…"
                                variant=ButtonVariant::Danger
                                size=ButtonSize::Large
                                icon=stop_icon
                                disabled=stop_disabled
                                loading=stop_loading
                                button_type="submit"
                            />
                        </ActionForm>
                        <ActionForm action=restart_action>
                            <ElevatedButton
                                label="重启"
                                loading_label="重启中…"
                                variant=ButtonVariant::Accent
                                size=ButtonSize::Large
                                icon=restart_icon
                                disabled=restart_disabled
                                loading=restart_loading
                                button_type="submit"
                            />
                        </ActionForm>
                    </div>
                </div>
            </header>
            <hr class="panel__divider" />
            <main class="panel__content">
                {move || match active_tab.get() {
                    PanelTab::PalWorld => view! {
                        <PalWorldView
                            pal_info=sse.pal_info
                            pal_metrics=sse.pal_metrics
                            pal_player_list=sse.pal_player_list
                        />
                    }.into_any(),
                    PanelTab::Dashboard => view! { <DashboardView sys_metrics=sse.sys_metrics /> }.into_any(),
                    PanelTab::Saves => view! { <SaveView /> }.into_any(),
                    PanelTab::Map => view! { <MapView /> }.into_any(),
                    PanelTab::Settings => view! { <SettingsView /> }.into_any(),
                }}
            </main>
        </div>
    }
}
