use leptos::prelude::*;

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

#[component]
pub fn Tag(
    #[prop(into)] text: String,
    #[prop(default = TagSize::Medium)] size: TagSize,
    #[prop(default = TagColor::Warning)] color: TagColor,
    #[prop(optional)] icon: Option<AnyView>,
) -> impl IntoView {
    let class = format!("tag {} {}", size.class(), color.class());

    view! {
        <span class=class>
            {icon}
            {text}
        </span>
    }
}
