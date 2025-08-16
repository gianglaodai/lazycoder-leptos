use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::components::A;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl Default for ButtonVariant {
    fn default() -> Self { ButtonVariant::Default }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

impl Default for ButtonSize {
    fn default() -> Self { ButtonSize::Default }
}

fn variant_classes(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
        ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    }
}

fn size_classes(size: ButtonSize) -> &'static str {
    match size {
        ButtonSize::Default => "h-9 px-4 py-2",
        ButtonSize::Sm => "h-8 rounded-md px-3",
        ButtonSize::Lg => "h-10 rounded-md px-8",
        ButtonSize::Icon => "h-9 w-9",
    }
}

fn base_classes() -> &'static str {
    "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 disabled:pointer-events-none disabled:opacity-50"
}

#[component]
pub fn Button(
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] loading: bool,
    #[prop(into, optional)] href: Option<String>,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent, ()>>,
    #[prop(optional)] r#type: Option<String>,
    children: Children,
) -> impl IntoView {
    let variant = if matches!(variant, ButtonVariant::Link) && href.is_none() {
        // Ensure link variant is meaningful; still allow it on buttons but it's mostly for anchors
        ButtonVariant::Link
    } else {
        if let ButtonVariant::Link = variant { ButtonVariant::Link } else { variant }
    };

    let class = class.unwrap_or_default();

    let classes = move || {
        let mut parts = Vec::new();
        parts.push(base_classes());
        parts.push(size_classes(size));
        parts.push(variant_classes(variant));
        if !class.is_empty() { parts.push(&class); }
        parts.join(" ")
    };

    let content = move || {
        if loading {
            view! {
                <span class="mr-2 inline-block h-4 w-4 animate-spin rounded-full border-2 border-current border-r-transparent"></span>
                {children()}
            }.into_any()
        } else {
            children().into_any()
        }
    };

    // If href provided, render as link; otherwise as button element
    match href {
        Some(url) => view! {
            <A href=url attr:class=classes()>
                {content()}
            </A>
        }.into_any(),
        None => {
            let t = r#type.unwrap_or_else(|| "button".to_string());
            view! {
                <button
                    class=classes()
                    disabled=disabled || loading
                    r#type=t
                    on:click=move |ev| if let Some(cb) = on_click { cb.run(ev) }
                >
                    {content()}
                </button>
            }.into_any()
        }
    }
}
