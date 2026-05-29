use leptos::prelude::*;

/// 通用卡片容器 —— 纯容器，提供统一的表面、圆角和内边距。
#[component]
pub fn Card(
    /// 附加 CSS 类名
    #[prop(optional)]
    class: Option<String>,
    /// 卡片内容
    children: Children,
) -> impl IntoView {
    let class = class.map(|c| format!("card {}", c)).unwrap_or_else(|| "card".to_string());

    view! {
        <article class=class>{children()}</article>
    }
}
