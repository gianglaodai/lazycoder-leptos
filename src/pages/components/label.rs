use leptos::prelude::*;

fn label_cls() -> &'static str {
    // shadcn/ui label styles
    "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
}

#[component]
pub fn Label(
    #[prop(into, optional)] class: Option<String>,
    // 'for' is a Rust keyword; use html_for to bind to the attribute
    #[prop(into, optional)] html_for: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();

    view! {
        <label
            class=move || crate::cn!(label_cls(), class.clone())
            prop:html_for=html_for
        >
            {children()}
        </label>
    }
}
