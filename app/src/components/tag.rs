use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TagColor {
    Success,
    Warning,
    Danger,
}

impl TagColor {
    pub fn class(&self) -> &'static str {
        match self {
            TagColor::Success => "tag--success",
            TagColor::Warning => "tag--warning",
            TagColor::Danger => "tag--danger",
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
) -> impl IntoView {
    let class = format!("tag {} {}", size.class(), color.class());

    view! {
        <span class=class>{text}</span>
    }
}
