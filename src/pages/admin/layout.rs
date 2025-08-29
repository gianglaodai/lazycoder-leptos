use crate::pages::components::{
    Separator, Sidebar, SidebarContent, SidebarFooter, SidebarHeader, SidebarProvider,
    SidebarTrigger,
};
use crate::pages::rest::auth_api::logout as api_logout;
use crate::pages::rest::auth_api::UserTO;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn AdminSidebar() -> impl IntoView {
    let user_ctx: RwSignal<Option<UserTO>> = expect_context();
    let do_logout = move |_| {
        let user_ctx = user_ctx.clone();
        leptos::task::spawn_local(async move {
            let _ = api_logout().await;
            user_ctx.set(None);
            let _ = leptos::prelude::window().location().set_href("/");
        });
    };

    view! {
        <Sidebar class="min-h-screen">
            <SidebarHeader>
                <div class="flex items-center justify-between">
                    <A href="/" attr:class="text-xl font-bold hover:opacity-80 transition-colors">LazyCoder</A>
                    <SidebarTrigger>
                        <span class="text-sm text-muted-foreground">Toggle</span>
                    </SidebarTrigger>
                </div>
            </SidebarHeader>
            <Separator class="my-2"/>
            <SidebarContent>
                <nav class="flex flex-col gap-1 text-sm">
                    <A href="/admin" attr:class="rounded px-2 py-1 hover:bg-accent hover:text-accent-foreground">Dashboard</A>
                    <A href="/admin/posts" attr:class="rounded px-2 py-1 hover:bg-accent hover:text-accent-foreground">Posts</A>
                    <A href="/admin/users" attr:class="rounded px-2 py-1 hover:bg-accent hover:text-accent-foreground">Users</A>
                    <A href="/settings" attr:class="rounded px-2 py-1 hover:bg-accent hover:text-accent-foreground">Settings</A>
                    <Separator class="my-2" />
                    <button class="text-left rounded px-2 py-1 hover:bg-accent hover:text-accent-foreground" on:click=move |_| do_logout(())>Logout</button>
                </nav>
            </SidebarContent>
            <SidebarFooter>
                <div class="text-xs text-muted-foreground">v1</div>
            </SidebarFooter>
        </Sidebar>
    }
}

#[component]
pub fn AdminLayout(children: Children) -> impl IntoView {
    view! {
        <SidebarProvider default_open=true>
            <div class="flex gap-0">
                <AdminSidebar />
                <main class="flex-1 min-h-screen">
                    {children()}
                </main>
            </div>
        </SidebarProvider>
    }
}
