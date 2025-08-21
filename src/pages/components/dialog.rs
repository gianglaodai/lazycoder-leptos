use leptos::html::Div;
use leptos::prelude::*;

#[derive(Clone)]
pub struct DialogContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool, ()>,
}

fn overlay_classes() -> &'static str {
    "fixed inset-0 z-50 bg-black/80"
}

fn content_classes() -> &'static str {
    "fixed left-1/2 top-1/2 z-50 grid w-full max-w-lg -translate-x-1/2 -translate-y-1/2 gap-4 border bg-background p-6 shadow-lg rounded-lg focus:outline-none"
}

fn header_classes() -> &'static str {
    "flex flex-col space-y-1.5 text-center sm:text-left"
}

fn footer_classes() -> &'static str {
    "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2"
}

fn title_classes() -> &'static str {
    "text-lg font-semibold leading-none tracking-tight"
}

fn description_classes() -> &'static str {
    "text-sm text-muted-foreground"
}

#[component]
pub fn Dialog(
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool, ()>>,
    children: Children,
) -> impl IntoView {
    let (uncontrolled_open, set_uncontrolled_open) = signal(default_open.unwrap_or(false));

    let (is_open, setter) = match open {
        Some(sig) => {
            let cb = on_open_change.unwrap_or_else(|| Callback::new(|_| {}));
            (sig, cb)
        }
        None => {
            let cb = Callback::new(move |value: bool| set_uncontrolled_open.set(value));
            (uncontrolled_open.into(), cb)
        }
    };

    provide_context(DialogContext {
        open: is_open,
        set_open: setter,
    });

    view! { {children()} }
}

#[component]
pub fn DialogTrigger(children: Children) -> impl IntoView {
    let ctx = expect_context::<DialogContext>();
    view! {
        <button
            class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 disabled:pointer-events-none disabled:opacity-50 h-9 px-4 py-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            on:click=move |_| { ctx.set_open.run(true); }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DialogOverlay() -> impl IntoView {
    let ctx = expect_context::<DialogContext>();
    let on_click = move |_| ctx.set_open.run(false);

    let classes = move || {
        let hidden = if ctx.open.get() { "" } else { "hidden" };
        format!("{} {}", overlay_classes(), hidden)
    };

    view! { <div class=classes on:click=on_click></div> }
}

#[component]
pub fn DialogContent(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<DialogContext>();
    let node_ref: NodeRef<Div> = NodeRef::new();

    let _key_listener = window_event_listener(leptos::ev::keydown, move |ev| {
        if ctx.open.get_untracked() {
            if ev.key() == "Escape" {
                ctx.set_open.run(false);
            }
        }
    });

    let on_backdrop_click = move |_| ctx.set_open.run(false);
    let stop = move |ev: leptos::ev::MouseEvent| ev.stop_propagation();

    let extra = class.unwrap_or_default();

    let classes = move || {
        if extra.is_empty() {
            content_classes().to_string()
        } else {
            format!("{} {}", content_classes(), extra)
        }
    };

    let root_classes = move || {
        let hidden = if ctx.open.get() { "" } else { "hidden" };
        format!("{} {}", "fixed inset-0 z-50", hidden)
    };

    view! {
        <div class=root_classes on:click=on_backdrop_click>
            <div class=overlay_classes()></div>
            <div node_ref=node_ref class=classes role="dialog" aria-modal="true" on:click=stop>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn DialogHeader(children: Children) -> impl IntoView {
    view! { <div class=header_classes()>{children()}</div> }
}

#[component]
pub fn DialogFooter(children: Children) -> impl IntoView {
    view! { <div class=footer_classes()>{children()}</div> }
}

#[component]
pub fn DialogTitle(children: Children) -> impl IntoView {
    view! { <h2 class=title_classes()>{children()}</h2> }
}

#[component]
pub fn DialogDescription(children: Children) -> impl IntoView {
    view! { <p class=description_classes()>{children()}</p> }
}

#[component]
pub fn DialogClose(children: Children) -> impl IntoView {
    let ctx = expect_context::<DialogContext>();
    view! {
        <button
            class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 disabled:pointer-events-none disabled:opacity-50 h-9 px-4 py-2 bg-secondary text-secondary-foreground hover:bg-secondary/80"
            on:click=move |_| { ctx.set_open.run(false); }
        >
            {children()}
        </button>
    }
}
