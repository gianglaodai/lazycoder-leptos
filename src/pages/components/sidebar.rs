use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SidebarState {
    Expanded,
    Collapsed,
}

#[derive(Clone)]
pub struct SidebarContext {
    pub state: RwSignal<SidebarState>,
    pub open_mobile: RwSignal<bool>,
    pub toggle: Callback<(), ()>,
}

#[component]
pub fn SidebarProvider(
    #[prop(optional)] default_open: Option<bool>,
    children: Children,
) -> impl IntoView {
    let state = RwSignal::new(if default_open.unwrap_or(true) {
        SidebarState::Expanded
    } else {
        SidebarState::Collapsed
    });
    let open_mobile = RwSignal::new(false);
    let toggle = Callback::new(move |_| {
        state.update(|s| {
            *s = match *s {
                SidebarState::Expanded => SidebarState::Collapsed,
                SidebarState::Collapsed => SidebarState::Expanded,
            }
        });
    });
    provide_context(SidebarContext {
        state,
        open_mobile,
        toggle,
    });
    view! { {children()} }
}

#[component]
pub fn Sidebar(#[prop(into, optional)] class: Option<String>, children: Children) -> impl IntoView {
    let ctx = expect_context::<SidebarContext>();
    let classes = move || {
        let extra = class.clone().unwrap_or_default();
        let base = "bg-background border-r flex flex-col";
        let width = match ctx.state.get() {
            SidebarState::Expanded => "w-64",
            SidebarState::Collapsed => "w-12",
        };
        if extra.is_empty() {
            format!("{} {}", base, width)
        } else {
            format!("{} {} {}", base, width, extra)
        }
    };
    view! { <aside data-slot="sidebar" class=classes>{children()}</aside> }
}

#[component]
pub fn SidebarHeader(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let base = "p-4";
    let classes = if extra.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, extra)
    };
    view! { <div data-slot="sidebar-header" class=classes>{children()}</div> }
}

#[component]
pub fn SidebarContent(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let base = "flex-1 p-2 overflow-auto";
    let classes = if extra.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, extra)
    };
    view! { <div data-slot="sidebar-content" class=classes>{children()}</div> }
}

#[component]
pub fn SidebarFooter(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let base = "p-2 border-t";
    let classes = if extra.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, extra)
    };
    view! { <div data-slot="sidebar-footer" class=classes>{children()}</div> }
}

#[component]
pub fn SidebarTrigger(children: Children) -> impl IntoView {
    let ctx = expect_context::<SidebarContext>();
    view! { <button data-slot="sidebar-trigger" on:click=move |_| ctx.toggle.run(())>{children()}</button> }
}
