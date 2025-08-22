use leptos::prelude::*;

fn tbl() -> &'static str {
    "w-full caption-bottom text-sm"
}

fn thead_cls() -> &'static str {
    "[&_tr]:border-b *:cursor-pointer"
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
    view! {
        <div class="w-full overflow-auto">
            <table class=move || crate::cn!(tbl(), class.clone())>{children()}</table>
        </div>
    }
}

#[component]
pub fn TableHeader(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! { <thead class=move || crate::cn!(thead_cls(), class.clone())>{children()}</thead> }
}

#[component]
pub fn TableBody(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! { <tbody class=move || crate::cn!(tbody_cls(), class.clone())>{children()}</tbody> }
}

#[component]
pub fn TableFooter(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! { <tfoot class=move || crate::cn!(tfoot_cls(), class.clone())>{children()}</tfoot> }
}

#[component]
pub fn TableRow(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! { <tr class=move || crate::cn!(tr_cls(), class.clone())>{children()}</tr> }
}

#[component]
pub fn TableHead(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! { <th class=move || crate::cn!(th_cls(), class.clone())>{children()}</th> }
}

#[component]
pub fn TableCell(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! { <td class=move || crate::cn!(td_cls(), class.clone())>{children()}</td> }
}

#[component]
pub fn TableCaption(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! { <caption class=move || crate::cn!(caption_cls(), class.clone())>{children()}</caption> }
}
