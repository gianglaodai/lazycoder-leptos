use crate::pages::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger,
};
use crate::pages::rest::auth_api::{logout as api_logout, UserRole, UserTO};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navigation() -> impl IntoView {
    let user_ctx: RwSignal<Option<UserTO>> = expect_context();
    let username = Memo::new(move |_| user_ctx.get().map(|u| u.username).unwrap_or_default());
    let is_admin = Memo::new(move |_| {
        user_ctx
            .get()
            .map(|u| u.role == UserRole::ADMIN)
            .unwrap_or(false)
    });

    let do_logout = move |_| {
        let user_ctx = user_ctx.clone();
        leptos::task::spawn_local(async move {
            let _ = api_logout().await; // ignore error gracefully
            user_ctx.set(None);
            let _ = leptos::prelude::window().location().set_href("/");
        });
    };

    view! {
        <nav class="font-serif bg-[--color-bg]/80 backdrop-blur supports-[backdrop-filter]:bg-[--color-bg]/80 mb-10 px-9 py-8 shadow-[0_0_2em_rgba(0,0,0,0.1)] sm:mx-0">
            <div class="container-page">
                <div class="flex items-center justify-between py-3">
                    {/* Left navigation links */}
                    <div class="flex items-center gap-8">
                        <div class="flex flex-wrap items-center gap-4 sm:gap-6 text-xl [&_a]:inline-flex [&_a]:items-center [&_a]:font-bold [&_a]:underline [&_a]:decoration-current [&_a]:underline-offset-4 [&_a]:decoration-1 [&_a:hover]:no-underline [&_a:hover]:text-black [&_a]:transition-colors [&_a[aria-current=page]]:decoration-4">
                            <A href="/" exact=true attr:class="text-rose-600">HOME</A>
                            <A href="/articles" attr:class="text-emerald-600">ARTICLES</A>
                            <A href="/about" attr:class="text-indigo-600">ABOUT ME</A>
                            <A href="/newsletter" attr:class="text-amber-600">NEWSLETTER</A>
                        </div>
                    </div>

                    {/* Right side: auth controls */}
                    <div class="relative">
                        <Show
                            when=move || user_ctx.get().is_some()
                            fallback=move || view! {
                                <div class="flex items-center">
                                    <A href="/login" attr:class="text-xl font-bold text-blue-600 underline underline-offset-4 decoration-1 hover:no-underline hover:text-black transition-colors">LOGIN</A>
                                </div>
                            }
                        >
                            <DropdownMenu>
                                <DropdownMenuTrigger class="text-xl cursor-pointer font-bold underline underline-offset-4 decoration-1 hover:no-underline hover:text-black transition-colors">
                                    {move || format!("Welcome, {}", username.get())}
                                </DropdownMenuTrigger>
                                <DropdownMenuContent class="right-0 mt-2 w-56">
                                    <Show when=move || is_admin.get() fallback=|| view! { <></> }>
                                        <DropdownMenuItem>
                                            <A href="/admin/home" attr:class="block w-full px-2 py-1.5 text-sm">Dashboard</A>
                                        </DropdownMenuItem>
                                    </Show>
                                    <DropdownMenuItem>
                                        <A href="/settings" attr:class="block w-full px-2 py-1.5 text-sm">Settings</A>
                                    </DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem on_click=Callback::new(move |_| do_logout(()))>
                                        Logout
                                    </DropdownMenuItem>
                                </DropdownMenuContent>
                            </DropdownMenu>
                        </Show>
                    </div>
                </div>
            </div>
        </nav>
    }
}
