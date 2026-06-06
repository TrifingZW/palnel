use leptos::prelude::*;

/// 通用文本输入字段，支持标签、前置图标、页脚插槽与双向绑定。
#[component]
pub fn TextField(
    #[prop(into)] label: String,
    #[prop(into)] name: String,
    #[prop(into)] placeholder: String,
    #[prop(into, optional)] icon: Option<AnyView>,
    #[prop(into, optional)] value: Option<RwSignal<String>>,
    #[prop(default = "text")] input_type: &'static str,
    #[prop(optional)] autocomplete: Option<&'static str>,
    #[prop(default = false)] required: bool,
    #[prop(optional)] footer: Option<AnyView>,
    #[prop(optional)] on_input: Option<Callback<String, ()>>,
) -> impl IntoView {
    let id = format!("tf-{}", name);

    let input_class = if icon.is_some() {
        "text-field__input text-field__input--icon"
    } else {
        "text-field__input"
    };

    view! {
        <div class="text-field">
            <label class="text-field__label" for=id.clone()>{label}</label>
            <div class="text-field__wrap">
                <span class="text-field__leading">{icon}</span>
                <input
                    id=id
                    class=input_class
                    type=input_type
                    name=name
                    placeholder=placeholder
                    required=required
                    autocomplete=autocomplete.unwrap_or("off")
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        if let Some(sig) = value {
                            sig.set(val.clone());
                        }
                        if let Some(ref cb) = on_input {
                            cb.run(val);
                        }
                    }
                />
            </div>
            {footer}
        </div>
    }
}
