use leptos::prelude::*;

/// Dialog 风格变体，影响确认按钮色彩。
#[derive(Clone, Copy, PartialEq)]
pub enum DialogVariant {
    Default,
    Danger,
}

impl DialogVariant {
    pub fn class(&self) -> &'static str {
        match self {
            DialogVariant::Default => "dialog--default",
            DialogVariant::Danger => "dialog--danger",
        }
    }
}

/// 模态确认弹窗，支持标题、正文、图标及确认/取消操作。
#[component]
pub fn Dialog(
    #[prop(into)] title: String,
    #[prop(into)] message: String,
    show: RwSignal<bool>,
    #[prop(default = DialogVariant::Default)] variant: DialogVariant,
    #[prop(optional)] confirm_label: Option<String>,
    #[prop(optional)] cancel_label: Option<String>,
    #[prop(optional)] on_confirm: Option<Callback<()>>,
    #[prop(optional)] on_cancel: Option<Callback<()>>,
    #[prop(optional)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let class = format!("dialog {}", variant.class());

    let dismiss = move || {
        show.set(false);
        if let Some(ref cb) = on_close {
            cb.run(());
        }
    };

    let dialog_view = view! {
        <div
            class=class
            class:dialog--visible=move || show.get()
            class:dialog--hidden=move || !show.get()
            on:click=move |ev: leptos::ev::MouseEvent| {
                if ev.target() == ev.current_target() {
                    dismiss();
                }
            }
        >
            <div class="dialog__card">
                <div class="dialog__header">
                    <div class="dialog__title-row">
                        {view! {
                            <svg class="dialog__icon" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                {match variant {
                                    DialogVariant::Default => view! {
                                        <circle cx="12" cy="12" r="10"/>
                                        <path d="M12 16v-4M12 8h0"/>
                                    },
                                    DialogVariant::Danger => view! {
                                        <circle cx="12" cy="12" r="10"/>
                                        <path d="M12 8v4M12 16h0"/>
                                    },
                                }}
                            </svg>
                        }}
                        <h2 class="dialog__title">{title}</h2>
                    </div>
                    <button
                        class="dialog__close"
                        on:click=move |_| dismiss()
                        aria-label="关闭"
                    >
                        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                            <path d="M1 1l12 12M13 1L1 13"/>
                        </svg>
                    </button>
                </div>
                <div class="dialog__body">
                    <p class="dialog__message">{message}</p>
                </div>
                <div class="dialog__footer">
                    {if let Some(label) = cancel_label {
                        let on_cancel = on_cancel.clone();
                        view! {
                            <button
                                class="dialog__btn dialog__btn--cancel"
                                on:click=move |_| {
                                    if let Some(ref cb) = on_cancel {
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
                    {if let Some(label) = confirm_label {
                        let on_confirm = on_confirm.clone();
                        view! {
                            <button
                                class="dialog__btn dialog__btn--confirm"
                                on:click=move |_| {
                                    if let Some(ref cb) = on_confirm {
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
                </div>
            </div>
        </div>
    };

    dialog_view
}
