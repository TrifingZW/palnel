use leptos::prelude::*;

use crate::components::menu::Menu;

/// 头像区域，含认证状态切换与下拉菜单。
#[component]
pub fn AvatarArea() -> impl IntoView {
    let (show_menu, set_show_menu) = signal(false);
    let (is_authenticated, set_is_authenticated) = signal(false);

    let toggle_menu = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        set_show_menu.update(|v| *v = !*v);
    };

    let close_menu = move |_| set_show_menu.set(false);

    let toggle_auth = move |_| {
        set_is_authenticated.update(|v| *v = !*v);
        set_show_menu.set(false);
    };

    view! {
        <div class="avatar-area">
            <button class="avatar-area__btn" on:click=toggle_menu>
                <span class="avatar-area__circle" aria-hidden="true">
                    <svg
                        width="16"
                        height="16"
                        viewBox="0 0 20 20"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            d="M10 2L3 5v5c0 4.418 2.91 8.087 7 9 4.09-.913 7-4.582 7-9V5l-7-3z"
                            fill="currentColor"
                        />
                    </svg>
                </span>
                <span class="avatar-area__label">
                    {move || if is_authenticated.get() { "Authenticated" } else { "Unauthenticated" }}
                </span>
            </button>
            <div
                class="menu-backdrop"
                class:menu-backdrop--visible=move || show_menu.get()
                on:click=close_menu
            ></div>
            <div class="menu" class:menu--visible=move || show_menu.get()>
                <Menu is_authenticated=is_authenticated on_toggle_auth=toggle_auth />
            </div>
        </div>
    }
}
