use leptos::prelude::*;

/// 紧凑内联状态指示，图标 + 文字。
#[component]
pub fn InlineStat(
    #[prop(optional)] icon: Option<AnyView>,
    #[prop(into)] text: String,
) -> impl IntoView {
    view! {
        <span class="inline-stat">
            {icon}
            <span class="inline-stat__text">{text}</span>
        </span>
    }
}
