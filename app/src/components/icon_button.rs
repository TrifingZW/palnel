use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};

/// Google Material 风格图标按钮 — 圆形、透明底、点击波浪动画。
#[component]
pub fn IconButton(#[prop(into)] on_click: Callback<()>, children: Children) -> impl IntoView {
    let (ripple, set_ripple) = signal(false);

    let handle_click = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        on_click.run(());
        if ripple.get() {
            return;
        }
        set_ripple.set(true);
        let set_ripple = set_ripple;
        let cb = Closure::wrap(Box::new(move || {
            set_ripple.set(false);
        }) as Box<dyn FnMut()>);
        let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            450,
        );
        cb.forget();
    };

    view! {
        <button
            class="icon-btn"
            class:icon-btn--ripple=ripple
            on:click=handle_click
        >
            {children()}
        </button>
    }
}
