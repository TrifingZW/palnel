use leptos::{portal::Portal, prelude::*};
use rpc::{login::Login, user::get_user};

use crate::components::{snackbar::SnackbarVariant, snackbar_state::SnackbarState};

/// 登录弹窗，集成 RPC 调用、加载状态、表单验证与全局反馈。
#[component]
pub fn LoginDialog(show: RwSignal<bool>, is_authenticated: RwSignal<bool>) -> impl IntoView {
    let auth_trigger = RwSignal::new(0);
    let user_resource = Resource::new(
        move || auth_trigger.get(),
        move |_| async move { get_user().await.unwrap_or(None) },
    );
    Effect::new(move || {
        if let Some(res) = user_resource.get() {
            is_authenticated.set(res.is_some());
        }
    });

    let login = ServerAction::<Login>::new();
    let is_loading = move || login.pending().get();
    let snackbar = SnackbarState::use_state();

    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    Effect::new(move || {
        if let Some(res) = login.value().get() {
            match res {
                Ok(_) => {
                    is_authenticated.set(true);
                    show.set(false);
                    snackbar.show("认证成功。", SnackbarVariant::Success);
                }
                Err(e) => {
                    let msg = e.to_string();
                    let user_msg = if msg.contains("User err") {
                        "用户不存在。"
                    } else if msg.contains("Invalid credentials") {
                        "密码错误。"
                    } else if msg.contains("cannot be empty") {
                        "用户名与密码不能为空。"
                    } else {
                        "登录失败，请稍后重试。"
                    };
                    snackbar.show(user_msg, SnackbarVariant::Danger);
                }
            }
            login.value().set(None);
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
                                <svg class="dialog__icon" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/>
                                    <polyline points="10 17 15 12 10 7"/>
                                    <line x1="15" y1="12" x2="3" y2="12"/>
                                </svg>
                            }}
                            <h2 class="dialog__title">"登录"</h2>
                        </div>
                        <button class="dialog__close" on:click=move |_| dismiss() aria-label="关闭">
                            <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                                <path d="M1 1l12 12M13 1L1 13"/>
                            </svg>
                        </button>
                    </div>

                    <ActionForm action=login>
                        <div class="dialog__body">
                            <div class="login-form">
                                <div class="login-field">
                                    <label class="login-field__label" for="login-username">"用户名"</label>
                                    <div class="login-field__input-wrap">
                                        <span class="login-field__leading">
                                            {view! {
                                                <svg class="login-field__leading-svg" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                                                </svg>
                                            }}
                                        </span>
                                        <input
                                            id="login-username"
                                            class="login-field__input login-field__input--icon"
                                            type="text"
                                            name="username"
                                            placeholder="请输入用户名"
                                            autocomplete="username"
                                            required
                                            on:input=move |ev| username.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>
                                <div class="login-field">
                                    <label class="login-field__label" for="login-password">"密码"</label>
                                    <div class="login-field__input-wrap">
                                        <span class="login-field__leading">
                                            {view! {
                                                <svg class="login-field__leading-svg" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                                                </svg>
                                            }}
                                        </span>
                                        <input
                                            id="login-password"
                                            class="login-field__input login-field__input--icon"
                                            type="password"
                                            name="password"
                                            placeholder="请输入密码"
                                            autocomplete="current-password"
                                            required
                                            on:input=move |ev| password.set(event_target_value(&ev))
                                        />
                                    </div>
                                    <div class="login-field__footer">
                                        <a href="#" class="login-field__link">"忘记凭证？"</a>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="dialog__footer dialog__footer--stack">
                            <button
                                class="dialog__btn dialog__btn--confirm dialog__btn--block"
                                type="submit"
                                disabled=is_loading
                            >
                                {move || {
                                    if is_loading() {
                                        view! {
                                            <span class="dialog__spinner"></span>
                                            <span>"验证中…"</span>
                                        }.into_any()
                                    } else {
                                        view! { <span>"登录"</span> }.into_any()
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
