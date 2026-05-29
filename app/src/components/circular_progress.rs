use leptos::prelude::*;

/// SVG 环形进度条，中心显示百分比数值。
#[component]
pub fn CircularProgress(
    /// 进度值 0.0 ~ 1.0
    #[prop(into)]
    value: Signal<f32>,
    /// 圆环直径 (px)
    #[prop(default = 100usize)]
    size: usize,
    /// 圆环描边宽度 (px)
    #[prop(default = 8usize)]
    stroke_width: usize,
) -> impl IntoView {
    let radius = (size - stroke_width) as f64 / 2.0;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let center = (size / 2) as f64;

    let dashoffset = move || {
        let progress = value.get().clamp(0.0, 1.0) as f64;
        circumference * (1.0 - progress)
    };

    let percentage = move || format!("{:.0}%", value.get().clamp(0.0, 1.0) * 100.0);

    view! {
        <div class="circular-progress" style=move || format!("width:{}px;height:{}px", size, size)>
            <svg class="circular-progress__svg" viewBox=move || format!("0 0 {} {}", size, size)>
                <circle
                    class="circular-progress__track"
                    cx=center
                    cy=center
                    r=radius
                    stroke-width=stroke_width
                />
                <circle
                    class="circular-progress__bar"
                    cx=center
                    cy=center
                    r=radius
                    stroke-width=stroke_width
                    stroke-dasharray=circumference
                    stroke-dashoffset=dashoffset
                    stroke-linecap="round"
                />
            </svg>
            <span class="circular-progress__value">{percentage}</span>
        </div>
    }
}
