use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SheetSide {
    Top,
    Right,
    Bottom,
    Left,
}
impl Default for SheetSide {
    fn default() -> Self {
        SheetSide::Right
    }
}

#[derive(Clone)]
struct SheetCtx {
    open: RwSignal<bool>,
}

#[component]
pub fn Sheet(
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool, ()>>,
    children: Children,
) -> impl IntoView {
    let (u_open, u_set_open) = signal(default_open.unwrap_or(false));
    let (is_open, setter) = match open {
        Some(sig) => {
            let cb = on_open_change.unwrap_or_else(|| Callback::new(|_| {}));
            (sig, cb)
        }
        None => {
            let cb = Callback::new(move |v: bool| u_set_open.set(v));
            (u_open.into(), cb)
        }
    };
    provide_context(SheetCtx {
        open: RwSignal::new(is_open.get_untracked()),
    });
    // keep reactive by mirroring signal
    let ctx = expect_context::<SheetCtx>();
    Effect::new(move |_| {
        ctx.open.set(is_open.get());
    });
    view! { {children()} }
}

#[component]
pub fn SheetTrigger(children: Children) -> impl IntoView {
    let ctx = expect_context::<SheetCtx>();
    view! { <button data-slot="sheet-trigger" on:click=move |_| ctx.open.set(true)>{children()}</button> }
}

#[component]
pub fn SheetClose(children: Children) -> impl IntoView {
    let ctx = expect_context::<SheetCtx>();
    view! { <button data-slot="sheet-close" on:click=move |_| ctx.open.set(false)>{children()}</button> }
}

#[component]
pub fn SheetContent(
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] side: SheetSide,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SheetCtx>();
    let side = side;
    let overlay_classes = "fixed inset-0 z-50 bg-black/50";
    let side_classes = move || match side {
        SheetSide::Right => "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm",
        SheetSide::Left => "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm",
        SheetSide::Top => "inset-x-0 top-0 h-auto border-b",
        SheetSide::Bottom => "inset-x-0 bottom-0 h-auto border-t",
    };
    let base = "bg-background fixed z-50 flex flex-col gap-4 shadow-lg transition ease-in-out";
    let classes = move || {
        let extra = class.clone().unwrap_or_default();
        let hidden = if ctx.open.get() { "" } else { "hidden" };
        if extra.is_empty() {
            format!("{} {} {}", base, side_classes(), hidden)
        } else {
            format!("{} {} {} {}", base, side_classes(), hidden, extra)
        }
    };
    let stop = |ev: leptos::ev::MouseEvent| ev.stop_propagation();
    view! {
        <div class=move || if ctx.open.get() { "fixed inset-0 z-50" } else { "hidden" } on:click=move |_| ctx.open.set(false)>
            <div class=overlay_classes></div>
            <div data-slot="sheet-content" class=classes on:click=stop>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn SheetHeader(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let base = "flex flex-col gap-1.5 p-4";
    let classes = if extra.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, extra)
    };
    view! { <div data-slot="sheet-header" class=classes>{children()}</div> }
}

#[component]
pub fn SheetFooter(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let base = "mt-auto flex flex-col gap-2 p-4";
    let classes = if extra.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, extra)
    };
    view! { <div data-slot="sheet-footer" class=classes>{children()}</div> }
}

#[component]
pub fn SheetTitle(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let base = "text-foreground font-semibold";
    let classes = if extra.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, extra)
    };
    view! { <h3 data-slot="sheet-title" class=classes>{children()}</h3> }
}

#[component]
pub fn SheetDescription(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let base = "text-muted-foreground text-sm";
    let classes = if extra.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, extra)
    };
    view! { <p data-slot="sheet-description" class=classes>{children()}</p> }
}
