use leptos::prelude::*;
use crate::pages::components::button::{Button, ButtonSize, ButtonVariant};

// Pagination root: <nav>
#[component]
pub fn Pagination(
    #[prop(into, optional)] class: Option<String>,
    // Allow passing arbitrary attributes via attr:*
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! {
        <nav
            role="navigation"
            aria-label="pagination"
            data-slot="pagination"
            class=move || {
                let base = "mx-auto flex w-full justify-center";
                if class.is_empty() { base.to_string() } else { format!("{base} {class}") }
            }
        >
            {children()}
        </nav>
    }
}

// PaginationContent: <ul>
#[component]
pub fn PaginationContent(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! {
        <ul
            data-slot="pagination-content"
            class=move || {
                let base = "flex flex-row items-center gap-1";
                if class.is_empty() { base.to_string() } else { format!("{base} {class}") }
            }
        >
            {children()}
        </ul>
    }
}

// PaginationItem: <li>
#[component]
pub fn PaginationItem(children: Children) -> impl IntoView {
    view! { <li data-slot="pagination-item">{children()}</li> }
}

// PaginationLink: anchor-like button
#[component]
pub fn PaginationLink(
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] is_active: bool,
    #[prop(optional, default = ButtonSize::Icon)] size: ButtonSize,
    #[prop(optional)] href: Option<String>,
    #[prop(optional)] aria_label: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let variant = if is_active { ButtonVariant::Outline } else { ButtonVariant::Ghost };

    // Render Button with or without href based on provided value to avoid type inference issue
    match href {
        Some(url) => view! {
            <Button
                attr:aria-current=move || if is_active { Some("page".to_string()) } else { None }
                attr:data-slot="pagination-link"
                attr:data-active=move || if is_active { Some("true".to_string()) } else { None }
                attr:aria-label=aria_label.clone()
                class=class.clone()
                variant=variant
                size=size
                href=url
            >
                {children()}
            </Button>
        }.into_any(),
        None => view! {
            <Button
                attr:aria-current=move || if is_active { Some("page".to_string()) } else { None }
                attr:data-slot="pagination-link"
                attr:data-active=move || if is_active { Some("true".to_string()) } else { None }
                attr:aria-label=aria_label.clone()
                class=class.clone()
                variant=variant
                size=size
            >
                {children()}
            </Button>
        }.into_any(),
    }
}

// Icon helpers (inline SVG) to avoid external deps
fn chevron_left_icon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
            <polyline points="15 18 9 12 15 6"></polyline>
        </svg>
    }
}

fn chevron_right_icon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
            <polyline points="9 18 15 12 9 6"></polyline>
        </svg>
    }
}

fn more_horizontal_icon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
            <circle cx="12" cy="12" r="1"></circle>
            <circle cx="19" cy="12" r="1"></circle>
            <circle cx="5" cy="12" r="1"></circle>
        </svg>
    }
}

// Previous link
#[component]
pub fn PaginationPrevious(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] href: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        let base = "gap-1 px-2.5 sm:pl-2.5";
        if class.is_empty() { base.to_string() } else { format!("{base} {class}") }
    };
    match href {
        Some(url) => view! {
            <PaginationLink
                aria_label="Go to previous page".to_string()
                size=ButtonSize::Default
                class=classes()
                href=url
            >
                {chevron_left_icon()}
                <span class="hidden sm:block">{"Previous"}</span>
            </PaginationLink>
        }.into_any(),
        None => view! {
            <PaginationLink
                aria_label="Go to previous page".to_string()
                size=ButtonSize::Default
                class=classes()
            >
                {chevron_left_icon()}
                <span class="hidden sm:block">{"Previous"}</span>
            </PaginationLink>
        }.into_any(),
    }
}

// Next link
#[component]
pub fn PaginationNext(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] href: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        let base = "gap-1 px-2.5 sm:pr-2.5";
        if class.is_empty() { base.to_string() } else { format!("{base} {class}") }
    };
    match href {
        Some(url) => view! {
            <PaginationLink
                aria_label="Go to next page".to_string()
                size=ButtonSize::Default
                class=classes()
                href=url
            >
                <span class="hidden sm:block">{"Next"}</span>
                {chevron_right_icon()}
            </PaginationLink>
        }.into_any(),
        None => view! {
            <PaginationLink
                aria_label="Go to next page".to_string()
                size=ButtonSize::Default
                class=classes()
            >
                <span class="hidden sm:block">{"Next"}</span>
                {chevron_right_icon()}
            </PaginationLink>
        }.into_any(),
    }
}

// Ellipsis span
#[component]
pub fn PaginationEllipsis(
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! {
        <span
            aria-hidden
            data-slot="pagination-ellipsis"
            class=move || {
                let base = "flex size-9 items-center justify-center";
                if class.is_empty() { base.to_string() } else { format!("{base} {class}") }
            }
        >
            {more_horizontal_icon()}
            <span class="sr-only">{"More pages"}</span>
        </span>
    }
}