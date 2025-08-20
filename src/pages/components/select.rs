use leptos::ev;
use leptos::prelude::*;

fn base_classes() -> &'static str {
    // Using a native select styled similarly to shadcn form controls
    "h-9 w-full rounded-md border border-input bg-background px-3 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50"
}

#[component]
pub fn Select(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(into, optional, default = MaybeSignal::from(String::new()))] value: MaybeSignal<String>,
    #[prop(into, optional)] on_change: Option<Callback<ev::Event, ()>>,
    children: Children,
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
        <select
            class=classes()
            id=id
            name=name
            disabled=disabled
            required=required
            prop:value=value
            on:change=move |ev| if let Some(cb) = on_change { cb.run(ev) }
        >
            {children()}
        </select>
    }
}
