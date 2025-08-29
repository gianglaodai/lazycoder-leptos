use leptos::prelude::*;

#[component]
pub fn Skeleton(#[prop(into, optional)] class: Option<String>) -> impl IntoView {
    let classes = move || {
        let extra = class.clone().unwrap_or_default();
        let base = "bg-accent animate-pulse rounded-md";
        if extra.is_empty() {
            base.to_string()
        } else {
            format!("{} {}", base, extra)
        }
    };
    view! { <div data-slot="skeleton" class=classes></div> }
}
