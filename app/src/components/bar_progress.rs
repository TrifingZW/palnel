use leptos::prelude::*;

/// 横向条形进度条。
#[component]
pub fn BarProgress(
    /// 进度值 0.0 ~ 1.0
    #[prop(into)]
    value: Signal<f32>,
    /// 填充色 CSS 值
    #[prop(into, optional)]
    fill_color: Option<String>,
) -> impl IntoView {
    let pct = move || {
        let v = value.get().clamp(0.0, 1.0);
        format!("{}%", v * 100.0)
    };

    let fill_style = move || {
        let color = fill_color.clone().unwrap_or_else(|| "var(--color-accent)".to_string());
        format!("width:{};background:{}", pct(), color)
    };

    view! {
        <div class="bar-progress">
            <div class="bar-progress__fill" style=fill_style></div>
        </div>
    }
}
