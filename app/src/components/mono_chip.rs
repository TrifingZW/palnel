use leptos::prelude::*;

/// 等宽文字芯片，适合展示 IP、坐标、ID 等技术信息。
#[component]
pub fn MonoChip(
    #[prop(optional)] icon: Option<AnyView>,
    #[prop(into)] text: String,
) -> impl IntoView {
    view! {
        <span class="mono-chip">
            {icon}
            <span class="mono-chip__text">{text}</span>
        </span>
    }
}
