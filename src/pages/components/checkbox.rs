use leptos::prelude::*;
use leptos::html;
use leptos::web_sys;
use wasm_bindgen::JsCast;

fn checkbox_cls() -> &'static str {
    // Approximate shadcn/ui checkbox styles
    "h-4 w-4 shrink-0 rounded-sm border border-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
}

#[component]
pub fn Checkbox(
    // Controlled checked state
    #[prop(optional, into)] checked: MaybeSignal<bool>,
    // Optional visual indeterminate state (for header select-all)
    #[prop(optional, into)] indeterminate: MaybeSignal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(into, optional)] class: Option<String>,
    // Callback with new checked value
    #[prop(into, optional)] on_change: Option<Callback<bool, ()>>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    // Keep the indeterminate visual state in sync
    let indet = indeterminate.clone();
    Effect::new(move |_| {
        if let Some(input) = input_ref.get() {
            // web-sys HtmlInputElement has set_indeterminate
            input.set_indeterminate(indet.get());
        }
    });

    let on_input = move |ev: web_sys::Event| {
        let target = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok());
        if let (Some(cb), Some(input)) = (on_change.clone(), target) {
            cb.run(input.checked());
        }
    };

    view! {
        <input
            node_ref=input_ref
            type="checkbox"
            role="checkbox"
            class=move || crate::cn!(checkbox_cls(), class.clone())
            prop:checked=checked
            prop:disabled=disabled
            on:input=on_input
        />
    }
}
