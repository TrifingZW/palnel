use leptos::prelude::*;

/// 标签颜色变体。
#[derive(Clone, Copy, PartialEq)]
pub enum TagColor {
    Success,
    Warning,
    Danger,
    Info,
    Accent,
    Purple,
    Teal,
}

impl TagColor {
    pub fn class(&self) -> &'static str {
        match self {
            TagColor::Success => "tag--success",
            TagColor::Warning => "tag--warning",
            TagColor::Danger => "tag--danger",
            TagColor::Info => "tag--info",
            TagColor::Accent => "tag--accent",
            TagColor::Purple => "tag--purple",
            TagColor::Teal => "tag--teal",
        }
    }
}

/// 标签尺寸变体。
#[derive(Clone, Copy, PartialEq)]
pub enum TagSize {
    Small,
    Medium,
    Large,
}

impl TagSize {
    pub fn class(&self) -> &'static str {
        match self {
            TagSize::Small => "tag--sm",
            TagSize::Medium => "tag--md",
            TagSize::Large => "tag--lg",
        }
    }
}

/// 内联标签，支持图标、多色、多尺寸。
#[component]
pub fn Tag(
    #[prop(into)] text: Signal<String>,
    #[prop(into, default = TagSize::Medium.into())] size: Signal<TagSize>,
    #[prop(into, default = TagColor::Warning.into())] color: Signal<TagColor>,
    #[prop(into, optional)] icon: Option<AnyView>,
) -> impl IntoView {
    let class = move || format!("tag {} {}", size.get().class(), color.get().class());

    view! {
        <span class=class>
            {icon}
            {text}
        </span>
    }
}
