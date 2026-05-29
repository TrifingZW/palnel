use leptos::prelude::*;

/// 下拉菜单，展示认证信息与登录/登出操作。
#[component]
pub fn Menu(
    is_authenticated: ReadSignal<bool>,
    #[prop(into)] on_toggle_auth: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="menu__info">
            <span class="menu__avatar" aria-hidden="true">
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
            </span>
            <span class="menu__name">
                {move || if is_authenticated.get() { "Authenticated" } else { "Unauthenticated" }}
            </span>
            <span class="menu__ip">"127.0.0.1"</span>
        </div>
        <div class="menu__actions">
            <button class="menu__action-btn" on:click=move |_| on_toggle_auth.run(())>
                {move || if is_authenticated.get() { "Logout" } else { "Login" }}
            </button>
        </div>
    }
}
