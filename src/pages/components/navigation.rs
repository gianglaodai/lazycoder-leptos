use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="bg-white shadow-sm">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between h-16">
                    <div class="flex">
                        <div class="flex-shrink-0 flex items-center">
                            <span class="text-xl font-bold text-gray-800">LazyCoder</span>
                        </div>
                        <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                            <A
                                href="/"
                                exact=true
                                attr:class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 aria-[current=page]:border-blue-500 aria-[current=page]:text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                            >
                                Home
                            </A>
                            <A
                                href="/articles"
                                attr:class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 aria-[current=page]:border-blue-500 aria-[current=page]:text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                            >
                                Articles
                            </A>
                            <A
                                href="/about"
                                attr:class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 aria-[current=page]:border-blue-500 aria-[current=page]:text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                            >
                                About Me
                            </A>
                            <A
                                href="/newsletter"
                                attr:class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 aria-[current=page]:border-blue-500 aria-[current=page]:text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                            >
                                Newsletter
                            </A>
                        </div>
                    </div>
                </div>
            </div>

            // Mobile menu
            <div class="sm:hidden" id="mobile-menu">
                <div class="pt-2 pb-3 space-y-1">
                    <A
                        href="/"
                        exact=true
                        attr:class="border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700 aria-[current=page]:bg-blue-50 aria-[current=page]:border-blue-500 aria-[current=page]:text-blue-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
                    >
                        Home
                    </A>
                    <A
                        href="/articles"
                        attr:class="border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700 aria-[current=page]:bg-blue-50 aria-[current=page]:border-blue-500 aria-[current=page]:text-blue-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
                    >
                        Articles
                    </A>
                    <A
                        href="/about"
                        attr:class="border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700 aria-[current=page]:bg-blue-50 aria-[current=page]:border-blue-500 aria-[current=page]:text-blue-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
                    >
                        About Me
                    </A>
                    <A
                        href="/newsletter"
                        attr:class="border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700 aria-[current=page]:bg-blue-50 aria-[current=page]:border-blue-500 aria-[current=page]:text-blue-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
                    >
                        Newsletter
                    </A>
                </div>
            </div>
        </nav>
    }
}
