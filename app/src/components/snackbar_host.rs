use leptos::prelude::*;

use crate::components::snackbar_state::SnackbarState;

/// 全局 Snackbar 渲染宿主，订阅 `SnackbarState` 并在底部渲染当前通知。
///
/// 支持点击关闭按钮手动关闭，或 `duration_ms` 毫秒后自动消失。
/// 关闭后自动调用 `state.dismiss()` 清除状态。
#[component]
pub fn SnackbarHost(#[prop(default = 4000)] duration_ms: u64) -> impl IntoView {
    let state = SnackbarState::use_state();
    let visible = RwSignal::new(false);

    Effect::new(move || {
        if state.current.get().is_some() {
            visible.set(true);
            if duration_ms > 0 {
                let state = state;
                set_timeout(
                    move || {
                        visible.set(false);
                        state.dismiss();
                    },
                    std::time::Duration::from_millis(duration_ms),
                );
            }
        }
    });

    let handle_dismiss = move || {
        visible.set(false);
        state.dismiss();
    };

    view! {
        <div class="snackbar-host">
            {move || {
                if let Some((text, variant)) = state.current.get() {
                    if visible.get() {
                        let class = format!("snackbar {}", variant.class());
                        view! {
                            <div class=class class:snackbar--visible=visible class:snackbar--hidden=move || !visible.get()>
                                <div class="snackbar__body">
                                    <svg class="snackbar__icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        {match variant {
                                            crate::components::snackbar::SnackbarVariant::Success => view! { <path d="M5 13l4 4L19 7"/> }.into_any(),
                                            crate::components::snackbar::SnackbarVariant::Info => view! {
                                                <circle cx="12" cy="12" r="10"/>
                                                <path d="M12 16v-4M12 8h0"/>
                                            }.into_any(),
                                            crate::components::snackbar::SnackbarVariant::Warning => view! {
                                                <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
                                                <path d="M12 9v4M12 17h0"/>
                                            }.into_any(),
                                            crate::components::snackbar::SnackbarVariant::Danger => view! {
                                                <circle cx="12" cy="12" r="10"/>
                                                <path d="M15 9l-6 6M9 9l6 6"/>
                                            }.into_any(),
                                        }}
                                    </svg>
                                    <span class="snackbar__message">{text}</span>
                                </div>
                                <div class="snackbar__actions">
                                    <button
                                        class="snackbar__close"
                                        on:click=move |_| handle_dismiss.clone()()
                                        aria-label="关闭"
                                    >
                                        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                                            <path d="M1 1l12 12M13 1L1 13"/>
                                        </svg>
                                    </button>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        ().into_any()
                    }
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}
