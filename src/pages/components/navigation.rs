use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="font-serif bg-[--color-bg]/80 backdrop-blur supports-[backdrop-filter]:bg-[--color-bg]/80">
            <div class="container-page">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center gap-8">
                        <div class="flex-shrink-0">
                            <span class="text-2xl font-semibold font-serif tracking-tight">LazyCoder</span>
                        </div>
                        <div class="hidden sm:flex sm:items-center sm:gap-6 text-xl [&_a]:inline-flex [&_a]:items-center [&_a]:font-medium [&_a]:underline [&_a]:decoration-current [&_a]:underline-offset-4 [&_a]:decoration-1 [&_a:hover]:no-underline [&_a:hover]:text-black [&_a]:transition-colors [&_a[aria-current=page]]:decoration-4">
                            <A
                                href="/"
                                exact=true
                                attr:class="text-rose-600"
                            >
                                Home
                            </A>
                            <A
                                href="/articles"
                                attr:class="text-emerald-600"
                            >
                                Articles
                            </A>
                            <A
                                href="/about"
                                attr:class="text-indigo-600"
                            >
                                About Me
                            </A>
                            <A
                                href="/newsletter"
                                attr:class="text-amber-600"
                            >
                                Newsletter
                            </A>
                        </div>
                    </div>
                    <div class="hidden sm:block">
                        <A href="/newsletter" attr:class="inline-flex items-center rounded-full bg-brand-600 text-white px-4 py-2 font-medium shadow-sm hover:bg-brand-700 transition-colors">
                            Subscribe
                        </A>
                    </div>
                </div>
            </div>

            // Mobile menu
            <div class="sm:hidden" id="mobile-menu">
                <div class="pt-2 pb-3 space-y-1 [&_a]:block [&_a]:px-4 [&_a]:py-2 [&_a]:underline [&_a]:decoration-current [&_a]:underline-offset-4 [&_a]:decoration-1 [&_a:hover]:no-underline [&_a:hover]:text-black [&_a]:text-base [&_a]:font-medium [&_a[aria-current=page]]:decoration-4">
                    <A href="/" exact=true attr:class="text-rose-600">HOME</A>
                    <A href="/articles" attr:class="text-emerald-600">ARTICLES</A>
                    <A href="/about" attr:class="text-indigo-600">ABOUT ME</A>
                    <A href="/newsletter" attr:class="text-amber-600">NEWSLETTER</A>
                </div>
            </div>
        </nav>
    }
}
