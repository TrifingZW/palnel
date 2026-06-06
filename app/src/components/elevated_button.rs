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

/// 悬浮按钮，支持图标、变体色、三种尺寸、加载与禁用过渡。
#[component]
pub fn ElevatedButton(
    #[prop(into)] label: Signal<String>,
    #[prop(into)] loading_label: Signal<String>,
    variant: ButtonVariant,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(into, optional)] icon: Option<AnyView>,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(into, default = Signal::derive(|| false))] loading: Signal<bool>,
    #[prop(default = "button")] button_type: &'static str,
) -> impl IntoView {
    let base_class = format!("elevated-btn {} {}", variant.class(), size.class());

    let is_disabled = Memo::new(move |_| disabled.get() || loading.get());

    let label = StoredValue::new(label);
    let loading_label = StoredValue::new(loading_label);

    view! {
        <button
            type=button_type
            class=move || {
                let mut cls = base_class.clone();
                if loading.get() {
                    cls.push_str(" elevated-btn--loading");
                } else if disabled.get() {
                    cls.push_str(" elevated-btn--disabled");
                }
                cls
            }
            disabled=move || is_disabled.get()
            on:click=move |ev| {
                if loading.get() || disabled.get() {
                    ev.prevent_default();
                } else if let Some(ref cb) = on_click {
                    cb.run(());
                }
            }
        >
            <span
                class="elevated-btn__icon"
                class:elevated-btn__icon_hidden=loading
            >
                {icon}
            </span>
            {move || {
                if loading.get() {
                    view! {
                        <span class="elevated-btn__spinner"></span>
                        {loading_label.with_value(|l| l.clone())}
                    }.into_any()
                } else {
                    view! {
                        {label.with_value(|l| l.clone())}
                    }.into_any()
                }
            }}
        </button>
    }
}
