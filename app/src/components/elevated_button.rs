use leptos::prelude::*;

/// 按钮颜色变体。
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Success,
    Danger,
    Accent,
}

impl ButtonVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Success => "elevated-btn--success",
            ButtonVariant::Danger => "elevated-btn--danger",
            ButtonVariant::Accent => "elevated-btn--accent",
        }
    }
}

/// 按钮尺寸变体。
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl ButtonSize {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonSize::Small => "elevated-btn--sm",
            ButtonSize::Medium => "elevated-btn--md",
            ButtonSize::Large => "elevated-btn--lg",
        }
    }
}

/// 悬浮按钮，支持图标、变体色、三种尺寸。
#[component]
pub fn ElevatedButton(
    #[prop(into)] label: String,
    variant: ButtonVariant,
    #[prop(into)] on_click: Callback<()>,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(optional)] icon: Option<AnyView>,
) -> impl IntoView {
    let class = format!("elevated-btn {} {}", variant.class(), size.class());

    view! {
        <button class=class on:click=move |_| on_click.run(())>
            {icon}
            {label}
        </button>
    }
}
