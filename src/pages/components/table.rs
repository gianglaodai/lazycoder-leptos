use leptos::prelude::*;

// Tailwind classes based on shadcn/ui Table
// https://ui.shadcn.com/docs/components/table

fn tbl() -> &'static str {
    "w-full caption-bottom text-sm"
}

fn thead_cls() -> &'static str {
    "[&_tr]:border-b"
}

fn tbody_cls() -> &'static str {
    "[&_tr:last-child]:border-0"
}

fn tfoot_cls() -> &'static str {
    "bg-primary font-medium text-primary-foreground"
}

fn tr_cls() -> &'static str {
    "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted"
}

fn th_cls() -> &'static str {
    "h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0"
}

fn td_cls() -> &'static str {
    "p-2 align-middle [&:has([role=checkbox])]:pr-0"
}

fn caption_cls() -> &'static str {
    "mt-4 text-sm text-muted-foreground"
}

#[component]
pub fn Table(#[prop(into, optional)] class: Option<String>, children: Children) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        if class.is_empty() {
            tbl().to_string()
        } else {
            format!("{} {}", tbl(), class)
        }
    };

    view! {
        // Wrap in overflow container similar to shadcn react example
        <div class="w-full overflow-auto">
            <table class=classes()>{children()}</table>
        </div>
    }
}

#[component]
pub fn TableHeader(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        if class.is_empty() {
            thead_cls().to_string()
        } else {
            format!("{} {}", thead_cls(), class)
        }
    };

    view! { <thead class=classes()>{children()}</thead> }
}

#[component]
pub fn TableBody(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        if class.is_empty() {
            tbody_cls().to_string()
        } else {
            format!("{} {}", tbody_cls(), class)
        }
    };

    view! { <tbody class=classes()>{children()}</tbody> }
}

#[component]
pub fn TableFooter(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        if class.is_empty() {
            tfoot_cls().to_string()
        } else {
            format!("{} {}", tfoot_cls(), class)
        }
    };

    view! { <tfoot class=classes()>{children()}</tfoot> }
}

#[component]
pub fn TableRow(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        if class.is_empty() {
            tr_cls().to_string()
        } else {
            format!("{} {}", tr_cls(), class)
        }
    };

    view! { <tr class=classes()>{children()}</tr> }
}

#[component]
pub fn TableHead(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        if class.is_empty() {
            th_cls().to_string()
        } else {
            format!("{} {}", th_cls(), class)
        }
    };

    view! { <th class=classes()>{children()}</th> }
}

#[component]
pub fn TableCell(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        if class.is_empty() {
            td_cls().to_string()
        } else {
            format!("{} {}", td_cls(), class)
        }
    };

    view! { <td class=classes()>{children()}</td> }
}

#[component]
pub fn TableCaption(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let classes = move || {
        if class.is_empty() {
            caption_cls().to_string()
        } else {
            format!("{} {}", caption_cls(), class)
        }
    };

    view! { <caption class=classes()>{children()}</caption> }
}
