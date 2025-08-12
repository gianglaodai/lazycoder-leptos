use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="font-serif bg-[--color-bg]/80 backdrop-blur supports-[backdrop-filter]:bg-[--color-bg]/80 mb-10 px-9 py-8 shadow-[0_0_2em_rgba(0,0,0,0.1)] sm:mx-0">
            <div class="container-page">
                <div class="flex flex-wrap items-center justify-center py-3">
                    <div class="flex items-center justify-center gap-8">
                        <div class="flex flex-wrap items-center justify-center gap-4 sm:gap-6 text-xl [&_a]:inline-flex [&_a]:items-center [&_a]:font-bold [&_a]:underline [&_a]:decoration-current [&_a]:underline-offset-4 [&_a]:decoration-1 [&_a:hover]:no-underline [&_a:hover]:text-black [&_a]:transition-colors [&_a[aria-current=page]]:decoration-4">
                            <A href="/" exact=true attr:class="text-rose-600">HOME</A>
                            <A href="/articles" attr:class="text-emerald-600">ARTICLES</A>
                            <A href="/about" attr:class="text-indigo-600">ABOUT ME</A>
                            <A href="/newsletter" attr:class="text-amber-600">NEWSLETTER</A>
                            <A href="/login" attr:class="text-blue-600">LOGIN</A>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
