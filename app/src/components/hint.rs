use leptos::{portal::Portal, prelude::*};
use wasm_bindgen::JsCast;

/// 悬浮提示组件，通过 Portal 渲染至 body 避免被父容器裁剪。
#[component]
pub fn Hint(#[prop(into)] text: String, children: Children) -> impl IntoView {
    let (show, set_show) = signal(false);
    let (pos, set_pos) = signal((0.0, 0.0));

    let on_enter = move |ev: leptos::ev::MouseEvent| {
        if let Some(el) =
            ev.current_target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let rect = el.get_bounding_client_rect();
            set_pos.set((rect.left() + rect.width() / 2.0, rect.top()));
        }
        set_show.set(true);
    };

    let on_leave = move |_| {
        set_show.set(false);
    };

    let tooltip_style = move || {
        let (x, y) = pos.get();
        format!("left:{x}px;top:{y}px;transform:translate(-50%,calc(-100% - 6px))")
    };

    let text = StoredValue::new(text);

    view! {
        <span
            class="hint"
            on:mouseenter=on_enter
            on:mouseleave=on_leave
        >
            {children()}
        </span>
        <Portal>
            <span
                class="hint__tooltip"
                class:hint__tooltip--visible=show
                style=move || tooltip_style()
            >
                {text.with_value(|t| t.clone())}
            </span>
        </Portal>
    }
}
