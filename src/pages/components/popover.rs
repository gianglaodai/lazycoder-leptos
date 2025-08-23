use leptos::html::Div;
use leptos::prelude::*;
use leptos::web_sys;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct PopoverContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool, ()>,
    pub container: NodeRef<Div>,
}

fn content_base_classes() -> &'static str {
    // shadcn/ui popover content baseline
    "z-50 min-w-[8rem] rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none"
}

fn container_classes() -> &'static str {
    // container that positions the popover relative to trigger
    "relative inline-block"
}

fn arrow_classes() -> &'static str {
    "absolute -top-2 left-1/2 -translate-x-1/2 h-2 w-2 rotate-45 bg-popover border-l border-t"
}

#[component]
pub fn Popover(
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

    let container_ref: NodeRef<Div> = NodeRef::new();

    provide_context(PopoverContext {
        open: is_open,
        set_open: setter,
        container: container_ref.clone(),
    });

    view! { <div node_ref=container_ref class=container_classes()>{children()}</div> }
}

#[component]
pub fn PopoverTrigger(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    // We render a span/button-like element that toggles the popover open state.
    let extra = class.unwrap_or_default();
    let classes = move || crate::cn!("cursor-pointer", extra.clone());

    view! {
        <span
            class=classes
            role="button"
            attr:aria-haspopup="dialog"
            attr:aria-expanded=move || if ctx.open.get() { "true" } else { "false" }
            on:click=move |_| {
                let next = !ctx.open.get();
                ctx.set_open.run(next);
            }
        >
            {children()}
        </span>
    }
}

#[component]
pub fn PopoverContent(
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] show_arrow: bool,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    // Close on Escape
    let _key_listener = {
        let ctx = ctx.clone();
        window_event_listener(leptos::ev::keydown, move |ev| {
            if ev.key() == "Escape" && ctx.open.get_untracked() {
                ctx.set_open.run(false);
            }
        })
    };

    // Close when clicking outside of the popover container (which wraps trigger + content)
    let _click_listener = {
        let ctx = ctx.clone();
        window_event_listener(leptos::ev::mousedown, move |ev: leptos::ev::MouseEvent| {
            if !ctx.open.get_untracked() { return; }
            if let Some(container_div) = ctx.container.get_untracked() {
                if let Some(target) = ev.target() {
                    if let Ok(target_el) = target.dyn_into::<web_sys::Element>() {
                        let container_el: web_sys::Element = container_div.unchecked_into();
                        if !container_el.contains(Some(&target_el)) {
                            ctx.set_open.run(false);
                        }
                    }
                }
            }
        })
    };

    let extra = class.unwrap_or_default();

    let panel_classes = move || crate::cn!(
        content_base_classes(),
        extra.clone(),
        // position classes: below & centered to trigger
        "absolute left-1/2 top-full mt-2 -translate-x-1/2"
    );

    let classes = move || {
        let mut s = panel_classes();
        if !ctx.open.get() { s.push_str(" hidden"); }
        s
    };

    view! {
        <div class=classes role="dialog" aria-modal="false" on:click=|ev| ev.stop_propagation()>
            {children()}
            <Show when=move || show_arrow fallback=|| ()>
                <div class=arrow_classes()></div>
            </Show>
        </div>
    }
}
