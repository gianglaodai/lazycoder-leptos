use leptos::prelude::*;

#[derive(Clone)]
struct TooltipCtx {
    open: RwSignal<bool>,
}

#[component]
pub fn Tooltip(children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(TooltipCtx { open });
    view! { {children()} }
}

#[component]
pub fn TooltipTrigger(children: Children) -> impl IntoView {
    let ctx = expect_context::<TooltipCtx>();
    let on_enter = move |_| ctx.open.set(true);
    let on_leave = move |_| ctx.open.set(false);
    view! { <span data-slot="tooltip-trigger" on:mouseenter=on_enter on:mouseleave=on_leave>{children()}</span> }
}

#[component]
pub fn TooltipContent(
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] side_offset: i32,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<TooltipCtx>();
    let side_offset = if side_offset == 0 { 0 } else { side_offset };
    let classes = move || {
        let extra = class.clone().unwrap_or_default();
        let base = "bg-primary text-primary-foreground z-50 w-fit rounded-md px-3 py-1.5 text-xs";
        let hidden = if ctx.open.get() { "" } else { "hidden" };
        let offset = format!("mt-{}", side_offset);
        if extra.is_empty() {
            format!("{} {} {} absolute", base, offset, hidden)
        } else {
            format!("{} {} {} {} absolute", base, offset, hidden, extra)
        }
    };
    view! { <div data-slot="tooltip-content" class=classes>{children()}</div> }
}

#[component]
pub fn TooltipProvider(children: Children) -> impl IntoView {
    view! { {children()} }
}
