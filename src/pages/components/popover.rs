use leptos::html::Div;
use leptos::prelude::*;
use leptos::web_sys;
use wasm_bindgen::JsCast;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Clone)]
pub struct PopoverContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool, ()>,
    pub container: NodeRef<Div>,
    pub click_x: RwSignal<f64>,
    pub click_y: RwSignal<f64>,
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
    let click_x: RwSignal<f64> = RwSignal::new(0.0f64);
    let click_y: RwSignal<f64> = RwSignal::new(0.0f64);

    provide_context(PopoverContext {
        open: is_open,
        set_open: setter,
        container: container_ref.clone(),
        click_x,
        click_y,
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
            on:click=move |ev: leptos::ev::MouseEvent| {
                ctx.click_x.set(ev.client_x() as f64);
                ctx.click_y.set(ev.client_y() as f64);
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

    // Track whether this component is still mounted to avoid accessing disposed signals
    let alive = Arc::new(AtomicBool::new(true));
    let alive_on_cleanup = alive.clone();
    on_cleanup(move || {
        alive_on_cleanup.store(false, Ordering::Relaxed);
    });

    // Close on Escape
    let _key_listener = {
        let ctx = ctx.clone();
        let alive = alive.clone();
        window_event_listener(leptos::ev::keydown, move |ev| {
            if !alive.load(Ordering::Relaxed) {
                return;
            }
            if ev.key() == "Escape" && ctx.open.get_untracked() {
                ctx.set_open.run(false);
            }
        })
    };

    // Close when clicking outside of the popover container (which wraps trigger + content)
    let _click_listener = {
        let ctx = ctx.clone();
        let alive = alive.clone();
        window_event_listener(leptos::ev::mousedown, move |ev: leptos::ev::MouseEvent| {
            if !alive.load(Ordering::Relaxed) {
                return;
            }
            if !ctx.open.get_untracked() {
                return;
            }
            if let Some(container_div) = ctx.container.get_untracked() {
                if let Some(target) = ev.target() {
                    if let Ok(target_el) = target.dyn_into::<web_sys::Element>() {
                        let container_el: web_sys::Element = container_div.unchecked_into();
                        if !container_el.contains(Some(target_el.as_ref())) {
                            ctx.set_open.run(false);
                        }
                    }
                }
            }
        })
    };

    // Track viewport size
    let vw = RwSignal::new(0.0f64);
    let vh = RwSignal::new(0.0f64);
    let update_vp = {
        let vw = vw.clone();
        let vh = vh.clone();
        move || {
            #[cfg(target_arch = "wasm32")]
            if let Some(w) = web_sys::window() {
                if let Ok(wv) = w.inner_width() {
                    if let Some(inner_w) = wv.as_f64() {
                        vw.set(inner_w);
                    }
                }
                if let Ok(hv) = w.inner_height() {
                    if let Some(inner_h) = hv.as_f64() {
                        vh.set(inner_h);
                    }
                }
            }
        }
    };
    update_vp();
    let _resize_listener = window_event_listener(leptos::ev::resize, move |_| update_vp());

    // Measure content size when open
    let content_ref: NodeRef<Div> = NodeRef::new();
    let cw = RwSignal::new(0.0f64);
    let ch = RwSignal::new(0.0f64);
    Effect::new({
        let cw = cw.clone();
        let ch = ch.clone();
        move |_| {
            if ctx.open.get() {
                if let Some(el) = content_ref.get_untracked() {
                    let elem: web_sys::HtmlElement = el.unchecked_into();
                    let w = elem.offset_width() as f64;
                    let h = elem.offset_height() as f64;
                    if w > 0.0 {
                        cw.set(w);
                    }
                    if h > 0.0 {
                        ch.set(h);
                    }
                }
            }
        }
    });

    // Use last click position and clamp to viewport to avoid overflow
    let left = Signal::derive({
        let ctx = ctx.clone();
        move || {
            let margin = 8.0f64;
            let x = ctx.click_x.get();
            let panel_w = cw.get();
            let vp_w = vw.get();
            // If placing the panel with its left at x would overflow the right edge,
            // align the panel's right edge to the trigger/click position so the
            // popover's top-right corner meets the trigger when near the right side.
            if x + panel_w + margin > vp_w {
                (x - panel_w).max(margin)
            } else {
                let max_left = (vp_w - margin - panel_w).max(margin);
                x.clamp(margin, max_left)
            }
        }
    });
    let top = Signal::derive({
        let ctx = ctx.clone();
        move || {
            let margin = 8.0f64;
            let below = ctx.click_y.get() + margin;
            let max_top = (vh.get() - margin - ch.get()).max(margin);
            below.min(max_top)
        }
    });

    let extra = class.unwrap_or_default();

    let panel_classes = move || {
        crate::cn!(
            content_base_classes(),
            extra.clone(),
            // position fixed relative to viewport, no centering transform
            "fixed mt-2"
        )
    };

    let classes = move || {
        let mut s = panel_classes();
        if !ctx.open.get() {
            s.push_str(" hidden");
        }
        s
    };

    let style_attr = move || format!("left:{}px; top:{}px;", left.get(), top.get());

    view! {
        <div node_ref=content_ref class=classes style=style_attr role="dialog" aria-modal="false" on:click=|ev| ev.stop_propagation()>
            {children()}
            <Show when=move || show_arrow fallback=|| ()>
                <div class=arrow_classes()></div>
            </Show>
        </div>
    }
}
