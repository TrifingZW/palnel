use leptos::prelude::*;

/// 通用下拉菜单容器，提供遮罩层、定位与进入动画。
///
/// `show` 信号控制显示/隐藏；点击遮罩关闭菜单并触发 `on_close`。
/// 内容通过 `children` 自由组合，引用 `.menu__info`、`.menu__actions`
/// 等 BEM 类名构建布局。
#[component]
pub fn Menu(
    show: RwSignal<bool>,
    children: Children,
    #[prop(optional)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let dismiss = move |_| {
        show.set(false);
        if let Some(ref cb) = on_close {
            cb.run(());
        }
    };

    view! {
        <div
            class="menu-backdrop"
            class:menu-backdrop--visible=move || show.get()
            on:click=dismiss
        ></div>
        <div class="menu" class:menu--visible=move || show.get()>
            {children()}
        </div>
    }
}

/// 菜单操作按钮，支持常规与危险两种色调。
#[component]
pub fn MenuAction(
    #[prop(into)] label: String,
    #[prop(into)] on_click: Callback<()>,
    #[prop(default = false)] danger: bool,
    #[prop(optional)] icon: Option<AnyView>,
) -> impl IntoView {
    view! {
        <button
            class="menu__action-btn"
            class:menu__action-btn--danger=danger
            on:click=move |_| on_click.run(())
        >
            {icon}
            {label}
        </button>
    }
}
