use leptos::{portal::Portal, prelude::*};
use rpc::palguard::PalguardAnnounce;

use crate::components::{
    snackbar::SnackbarVariant, snackbar_state::SnackbarState, text_field::TextField,
};

/// 公告弹窗，提供消息输入并通过 RPC 发送全服公告。
#[component]
pub fn AnnounceDialog(
    show: RwSignal<bool>,
    announce_action: ServerAction<PalguardAnnounce>,
    disabled: Signal<bool>,
    loading: Signal<bool>,
) -> impl IntoView {
    let snackbar = SnackbarState::use_state();

    Effect::new(move || {
        if let Some(res) = announce_action.value().get() {
            match res {
                Ok(_) => {
                    snackbar.show("公告已发送。", SnackbarVariant::Success);
                }
                Err(e) => {
                    snackbar.show(&format!("公告发送失败：{e}"), SnackbarVariant::Danger);
                }
            }
            announce_action.value().set(None);
            show.set(false);
        }
    });

    let dismiss = move || {
        show.set(false);
    };

    view! {
        <Portal>
            <div
                class="dialog dialog--default"
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
                                <svg width="22" height="22" viewBox="0 0 16 16" fill="currentColor">
                                    <path d="M13 2.5a1.5 1.5 0 0 1 3 0v11a1.5 1.5 0 0 1-3 0v-.214c-2.162-1.241-4.49-1.843-6.912-2.083l.405 2.712A1 1 0 0 1 5.51 15.1h-.548a1 1 0 0 1-.916-.599l-1.85-3.49a68.14 68.14 0 0 0-.202-.003A2.014 2.014 0 0 1 0 9V7a2.02 2.02 0 0 1 1.992-2.013 74.663 74.663 0 0 0 2.483-.075c3.043-.154 6.148-.849 8.525-2.199V2.5zm1 0v11a.5.5 0 0 0 1 0v-11a.5.5 0 0 0-1 0zm-1 1.35c-2.344 1.205-5.209 1.842-8 2.033v4.233c.18.01.359.022.537.036 2.568.189 5.093.744 7.4631.993V3.85zm-9 6.215v-4.13a95.09 95.09 0 0 1-1.992.052A1.02 1.02 0 0 0 1 7v2c0 .55.448 1.002 1.006 1.009A60.49 60.49 0 0 1 4 10.065zm-.657.975 1.609 3.037.01.024h.548l-.002-.014-.443-2.966a68.019 68.019 0 0 0-1.722-.082z"/>
                                </svg>
                            }}
                            <h2 class="dialog__title">"发送公告"</h2>
                        </div>
                        <button class="dialog__close" on:click=move |_| dismiss() aria-label="关闭">
                            <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                                <path d="M1 1l12 12M13 1L1 13"/>
                            </svg>
                        </button>
                    </div>

                    <ActionForm action=announce_action>
                        <div class="dialog__body">
                            <TextField
                                label="公告内容"
                                name="message"
                                placeholder="输入公告内容…"
                                icon=view! {
                                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
                                    </svg>
                                }.into_any()
                                required=true
                            />
                        </div>

                        <div class="dialog__footer dialog__footer--stack">
                            <button
                                class="dialog__btn dialog__btn--confirm dialog__btn--block"
                                type="submit"
                                disabled=move || loading.get() || disabled.get()
                            >
                                {move || {
                                    if loading.get() {
                                        view! {
                                            <span class="dialog__spinner"></span>
                                            <span>"发送中…"</span>
                                        }.into_any()
                                    } else {
                                        view! { <span>"发送"</span> }.into_any()
                                    }
                                }}
                            </button>
                            <button
                                class="dialog__btn dialog__btn--cancel dialog__btn--block"
                                type="button"
                                on:click=move |_| dismiss()
                            >
                                "取消"
                            </button>
                        </div>
                    </ActionForm>
                </div>
            </div>
        </Portal>
    }
}
