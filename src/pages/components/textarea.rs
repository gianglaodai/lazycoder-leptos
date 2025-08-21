use leptos::ev;
use leptos::prelude::*;

fn base_classes() -> &'static str {
    // Based on shadcn/ui textarea styles
    "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50"
}

#[component]
pub fn Textarea(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(optional)] rows: Option<u32>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional, default = Signal::from(String::new()))] value: Signal<String>,
    #[prop(optional)] on_input: Option<Callback<ev::Event, ()>>,
    #[prop(optional)] on_blur: Option<Callback<ev::FocusEvent, ()>>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        if class.is_empty() {
            base_classes().to_string()
        } else {
            format!("{} {}", base_classes(), class)
        }
    };

    view! {
        <textarea
            class=classes()
            id=id
            name=name
            disabled=disabled
            required=required
            rows=rows
            placeholder=placeholder
            prop:value=value
            on:input=move |ev| if let Some(cb) = on_input { cb.run(ev) }
            on:blur=move |ev| if let Some(cb) = on_blur { cb.run(ev) }
        />
    }
}
