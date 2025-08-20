use leptos::ev;
use leptos::prelude::*;

fn base_classes() -> &'static str {
    // Based on shadcn/ui input styles
    "flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50"
}

#[component]
pub fn Input(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] r#type: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional, default = MaybeSignal::from(String::new()))] value: MaybeSignal<String>,
    #[prop(into, optional)] on_input: Option<Callback<ev::Event, ()>>,
    #[prop(into, optional)] on_change: Option<Callback<ev::Event, ()>>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        if class.is_empty() {
            base_classes().to_string()
        } else {
            format!("{} {}", base_classes(), class)
        }
    };

    let t = r#type.unwrap_or_else(|| "text".to_string());

    view! {
        <input
            class=classes()
            id=id
            name=name
            r#type=t
            disabled=disabled
            required=required
            placeholder=placeholder
            prop:value=value
            on:input=move |ev| if let Some(cb) = on_input { cb.run(ev) }
            on:change=move |ev| if let Some(cb) = on_change { cb.run(ev) }
        />
    }
}
