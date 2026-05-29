use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Start,
    Stop,
    Restart,
}

impl ButtonVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Start => "elevated-btn--start",
            ButtonVariant::Stop => "elevated-btn--stop",
            ButtonVariant::Restart => "elevated-btn--restart",
        }
    }
}

#[component]
pub fn ElevatedButton(
    #[prop(into)] label: String,
    variant: ButtonVariant,
    #[prop(into)] on_click: Callback<()>,
    #[prop(optional)] icon: Option<AnyView>,
) -> impl IntoView {
    let class = format!("elevated-btn {}", variant.class());

    view! {
        <button class=class on:click=move |_| on_click.run(())>
            {icon}
            {label}
        </button>
    }
}
