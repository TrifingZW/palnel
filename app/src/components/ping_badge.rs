use leptos::prelude::*;

/// 延迟指示器，颜色随数值自动分级：绿 < 50ms < 橙 < 120ms < 红。
#[component]
pub fn PingBadge(#[prop(into)] ping: f64) -> impl IntoView {
    let (color, bg) = if ping < 50.0 {
        ("var(--color-success)", "var(--color-success-light)")
    } else if ping < 120.0 {
        ("var(--color-warning)", "var(--color-warning-light)")
    } else {
        ("var(--color-danger)", "var(--color-danger-light)")
    };

    view! {
        <span class="ping-badge" style=format!("color:{};background:{}", color, bg)>
            <span class="ping-badge__dot" style=format!("background:{}", color)></span>
            {format!("{:.0}ms", ping)}
        </span>
    }
}
