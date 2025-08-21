use crate::pages::components::button::ButtonVariant;
use crate::pages::components::Button;
use crate::pages::rest::auth_api::{logout, UserTO};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

#[component]
pub fn Navigation() -> impl IntoView {
    let user_ctx: RwSignal<Option<UserTO>> = expect_context();
    let show_menu = RwSignal::new(false);
    let _navigate = use_navigate();
    let username = Memo::new(move |_| user_ctx.get().map(|u| u.username).unwrap_or_default());

    let logout = move |_| {
        // Close menu, logout, clear user, and navigate home
        show_menu.set(false);
        let user_ctx = user_ctx.clone();
        leptos::task::spawn_local(async move {
            let _ = logout().await; // ignore error gracefully
            user_ctx.set(None);
            let _ = leptos::prelude::window().location().set_href("/");
        });
    };

    // Close dropdown when clicking outside
    let on_click_outside = move |_| show_menu.set(false);

    view! {
        <nav class="font-serif bg-[--color-bg]/80 backdrop-blur supports-[backdrop-filter]:bg-[--color-bg]/80 mb-10 px-9 py-8 shadow-[0_0_2em_rgba(0,0,0,0.1)] sm:mx-0" on:click=on_click_outside>
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
                            <div class="flex items-center">
                                <Button variant=ButtonVariant::Link class="text-xl font-bold underline underline-offset-4 decoration-1 hover:no-underline hover:text-black transition-colors" on_click=Callback::new(move |ev: leptos::ev::MouseEvent| { ev.stop_propagation(); show_menu.update(|v| *v = !*v); })>
                                    {move || format!("Welcome, {}", username.get())}
                                </Button>
                                <Show when=move || show_menu.get() fallback=|| ()>
                                    <div class="absolute right-0 mt-2 w-48 rounded-md bg-white shadow-lg ring-1 ring-black/5 focus:outline-none z-50" on:click=|ev| ev.stop_propagation()>
                                        <div class="py-1 text-[--color-ink]">
                                            <A href="/settings" attr:class="block px-4 py-2 text-sm hover:bg-gray-100">Settings</A>
                                            <Button class="w-full text-left block px-4 py-2 text-sm hover:bg-gray-100" variant=ButtonVariant::Ghost on_click=Callback::new(move |_| logout(()))>Logout</Button>
                                        </div>
                                    </div>
                                </Show>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
        </nav>
    }
}
