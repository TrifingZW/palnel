use leptos::prelude::*;

use crate::components::{
    login_dialog::LoginDialog,
    menu::{Menu, MenuAction},
};

/// 盾牌图标 SVG。
fn shield_svg() -> impl IntoView {
    view! {
        <svg
            width="18"
            height="18"
            viewBox="0 0 20 20"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M10 2L3 5v5c0 4.418 2.91 8.087 7 9 4.09-.913 7-4.582 7-9V5l-7-3z"
                fill="currentColor"
            />
        </svg>
    }
}

/// 登录箭头图标 SVG。
fn login_icon_svg() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/>
            <polyline points="10 17 15 12 10 7"/>
            <line x1="15" y1="12" x2="3" y2="12"/>
        </svg>
    }
}

/// 登出箭头图标 SVG。
fn logout_icon_svg() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
            <polyline points="16 17 21 12 16 7"/>
            <line x1="21" y1="12" x2="9" y2="12"/>
        </svg>
    }
}

/// 头像区域，含认证状态切换、下拉菜单与登录弹窗。
#[component]
pub fn AvatarArea() -> impl IntoView {
    let show_menu = RwSignal::new(false);
    let show_login = RwSignal::new(false);
    let is_authenticated = RwSignal::new(false);

    let toggle_menu = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        show_menu.update(|v| *v = !*v);
    };

    let close_menu = Callback::new(move |()| show_menu.set(false));

    let open_login = Callback::new(move |()| {
        show_menu.set(false);
        show_login.set(true);
    });

    let handle_logout = Callback::new(move |()| {
        is_authenticated.set(false);
        show_menu.set(false);
    });

    view! {
        <div class="avatar-area">
            <button class="avatar-area__btn" on:click=toggle_menu>
                <span class="avatar-area__circle" aria-hidden="true">
                    {shield_svg()}
                </span>
                <span class="avatar-area__label">
                    {move || if is_authenticated.get() { "Authenticated" } else { "Unauthenticated" }}
                </span>
            </button>

            <Menu show=show_menu on_close=close_menu>
                <div class="menu__info">
                    <span class="menu__avatar" aria-hidden="true">
                        {shield_svg()}
                    </span>
                    <span class="menu__name">
                        {move || if is_authenticated.get() { "Authenticated" } else { "Unauthenticated" }}
                    </span>
                    <span class="menu__ip">"127.0.0.1"</span>
                </div>
                <div class="menu__actions">
                    {move || {
                        if is_authenticated.get() {
                            view! {
                                <MenuAction
                                    label="Logout"
                                    on_click=handle_logout
                                    icon=logout_icon_svg().into_any()
                                />
                            }.into_any()
                        } else {
                            view! {
                                <MenuAction
                                    label="Login"
                                    on_click=open_login
                                    icon=login_icon_svg().into_any()
                                />
                            }.into_any()
                        }
                    }}
                </div>
            </Menu>
        </div>

        <LoginDialog show=show_login is_authenticated=is_authenticated />
    }
}
