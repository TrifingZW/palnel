use leptos::prelude::*;

/// 卡片视觉变体。
#[derive(Clone, Copy, PartialEq)]
pub enum CardVariant {
    Default,
    Banner,
}

impl CardVariant {
    pub fn class(&self) -> Option<&'static str> {
        match self {
            CardVariant::Default => None,
            CardVariant::Banner => Some("card--banner"),
        }
    }
}

/// 通用卡片容器 —— 纯容器，提供统一的表面、圆角和内边距。
#[component]
pub fn Card(
    #[prop(default = CardVariant::Default)] variant: CardVariant,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let mut classes = vec!["card".to_string()];
    if let Some(v) = variant.class() {
        classes.push(v.to_string());
    }
    if let Some(c) = class {
        classes.push(c);
    }
    let class = classes.join(" ");

    view! {
        <article class=class>{children()}</article>
    }
}
