use crate::cn;
use leptos::html::Div;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemVariant {
    Default,
    Destructive,
}

impl Default for ItemVariant {
    fn default() -> Self {
        ItemVariant::Default
    }
}

#[derive(Clone)]
struct DropdownMenuCtx {
    open: RwSignal<bool>,
    root_ref: NodeRef<Div>,
}

/// Root container that provides open/close state for the Dropdown Menu.
#[component]
pub fn DropdownMenu(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let root_ref = NodeRef::new();

    provide_context(DropdownMenuCtx { open, root_ref });

    // Close when clicking outside
    let on_window_click = {
        let open = open.clone();
        let root_ref = root_ref.clone();
        move |ev: web_sys::MouseEvent| {
            if !open.get_untracked() {
                return;
            }
            if let Some(root) = root_ref.get_untracked() {
                if let Some(target) = ev.target() {
                    if let Some(node) = target.dyn_ref::<web_sys::Node>() {
                        if !root.contains(Some(node)) {
                            open.set(false);
                        }
                    }
                }
            }
        }
    };

    view! {
        <div node_ref=root_ref class=cn!("relative inline-block text-left", class) on:click:window=on_window_click>
            {children()}
        </div>
    }
}

/// The element that toggles the menu. Typically a button.
#[component]
pub fn DropdownMenuTrigger(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<DropdownMenuCtx>();
    let toggle = move |_| ctx.open.update(|v| *v = !*v);

    view! {
        <button type="button" class=cn!(class) on:click=toggle aria-haspopup="menu" aria-expanded=move || ctx.open.get().to_string()>
            {children()}
        </button>
    }
}

/// The floating content panel. Positioned relative to the root.
#[component]
pub fn DropdownMenuContent(
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] side_offset: i32,
    children: Children,
) -> impl IntoView {
    let side_offset = if side_offset == 0 { 4 } else { side_offset };

    let ctx = expect_context::<DropdownMenuCtx>();

    let base = "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 min-w-[8rem] origin-[var(--radix-dropdown-menu-content-transform-origin)] overflow-x-hidden overflow-y-auto rounded-md border p-1 shadow-md";

    // Simple positioning (bottom-start)
    let pos_cls = move || format!("absolute left-0 mt-{} {}", side_offset, base);

    view! {
        <div
            role="menu"
            data-slot="dropdown-menu-content"
            class=move || cn!(pos_cls(), class.clone(), (!ctx.open.get()).then_some("hidden"))
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuGroup(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! { <div data-slot="dropdown-menu-group" class=cn!(class)>{children()}</div> }
}

#[component]
pub fn DropdownMenuLabel(
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] inset: bool,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        cn!(
            "px-2 py-1.5 text-sm font-medium",
            (inset).then_some("pl-8"),
            class.clone()
        )
    };

    view! { <div data-slot="dropdown-menu-label" class=classes>{children()}</div> }
}

#[component]
pub fn DropdownMenuSeparator(#[prop(into, optional)] class: Option<String>) -> impl IntoView {
    view! { <div role="separator" data-slot="dropdown-menu-separator" class=cn!("bg-border -mx-1 my-1 h-px", class)></div> }
}

#[component]
pub fn DropdownMenuShortcut(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! { <span data-slot="dropdown-menu-shortcut" class=cn!("text-muted-foreground ml-auto text-xs tracking-widest", class)>{children()}</span> }
}

#[component]
pub fn DropdownMenuItem(
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] inset: bool,
    #[prop(optional)] variant: ItemVariant,
    #[prop(into, optional)] on_click: Option<Callback<web_sys::MouseEvent, ()>>,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        let destructive = matches!(variant, ItemVariant::Destructive);
        cn!(
            "focus:bg-accent focus:text-accent-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
            (inset).then_some("pl-8"),
            (destructive).then_some("text-destructive focus:bg-destructive/10 dark:focus:bg-destructive/20 focus:text-destructive [&_*[svg]]:!text-destructive"),
            class.clone(),
        )
    };

    view! {
        <div
            role="menuitem"
            tabindex="0"
            data-slot="dropdown-menu-item"
            data-inset=move || inset.then_some("true")
            data-variant=move || if matches!(variant, ItemVariant::Destructive) { Some("destructive") } else { Some("default") }
            class=classes
            on:click=move |ev| if let Some(cb) = on_click.clone() { cb.run(ev) }
        >
            {children()}
        </div>
    }
}
