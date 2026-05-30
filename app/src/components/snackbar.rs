use leptos::prelude::*;

/// Snackbar 通知颜色变体。
#[derive(Clone, Copy, PartialEq)]
pub enum SnackbarVariant {
    Success,
    Info,
    Warning,
    Danger,
}

impl SnackbarVariant {
    /// 返回 BEM 修饰符类名。
    pub fn class(&self) -> &'static str {
        match self {
            SnackbarVariant::Success => "snackbar--success",
            SnackbarVariant::Info => "snackbar--info",
            SnackbarVariant::Warning => "snackbar--warning",
            SnackbarVariant::Danger => "snackbar--danger",
        }
    }
}

/// 底部弹出式消息通知，支持多色变体、图标、操作按钮与自动关闭。
///
/// `show` 信号控制显示状态；设为 `true` 后将在 `duration_ms` 毫秒内自动隐藏。
/// 用户也可通过关闭按钮或操作按钮（搭配 `on_close` 回调）手动关闭。
#[component]
pub fn Snackbar(
    #[prop(into)] message: String,
    show: RwSignal<bool>,
    #[prop(default = SnackbarVariant::Info)] variant: SnackbarVariant,
    #[prop(default = 0)] duration_ms: u64,
    #[prop(optional)] icon: Option<AnyView>,
    #[prop(optional)] action_label: Option<String>,
    #[prop(optional)] on_action: Option<Callback<()>>,
    #[prop(optional)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let class = format!("snackbar {}", variant.class());

    let dismiss = move || {
        show.set(false);
        if let Some(ref cb) = on_close {
            cb.run(());
        }
    };

    let default_icon = view! {
        <svg class="snackbar__icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            {match variant {
                SnackbarVariant::Success => view! { <path d="M5 13l4 4L19 7"/> }.into_any(),
                SnackbarVariant::Info => view! {
                    <circle cx="12" cy="12" r="10"/>
                    <path d="M12 16v-4M12 8h0"/>
                }.into_any(),
                SnackbarVariant::Warning => view! {
                    <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
                    <path d="M12 9v4M12 17h0"/>
                }.into_any(),
                SnackbarVariant::Danger => view! {
                    <circle cx="12" cy="12" r="10"/>
                    <path d="M15 9l-6 6M9 9l6 6"/>
                }.into_any(),
            }}
        </svg>
    }
    .into_any();

    let icon_view = icon.unwrap_or(default_icon);

    Effect::new(move || {
        if show.get() {
            if duration_ms > 0 {
                let dismiss = dismiss.clone();
                set_timeout(move || dismiss(), std::time::Duration::from_millis(duration_ms));
            }
        }
    });

    view! {
        <div class=class class:snackbar--visible=move || show.get() class:snackbar--hidden=move || !show.get()>
            <div class="snackbar__body">
                {icon_view}
                <span class="snackbar__message">{message}</span>
            </div>
            <div class="snackbar__actions">
                {if let Some(label) = action_label {
                    let on_action = on_action.clone();
                    let dismiss = dismiss.clone();
                    view! {
                        <button
                            class="snackbar__action-btn"
                            on:click=move |_| {
                                if let Some(ref cb) = on_action {
                                    cb.run(());
                                }
                                dismiss();
                            }
                        >
                            {label}
                        </button>
                    }.into_any()
                } else {
                    ().into_any()
                }}
                <button
                    class="snackbar__close"
                    on:click=move |_| dismiss()
                    aria-label="关闭"
                >
                    <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                        <path d="M1 1l12 12M13 1L1 13"/>
                    </svg>
                </button>
            </div>
        </div>
    }
}
