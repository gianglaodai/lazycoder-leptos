use crate::pages::components::MarkdownEditor;
use crate::pages::components::MarkdownViewer;
use leptos::prelude::*;
use leptos::svg::title;
use leptos::web_sys;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Feature items with icons and descriptions
    let features = vec![
        ("⚡", "Lightning Fast", "Built with Rust for maximum performance",),
        ("🔒", "Secure", "Enterprise-grade security out of the box"),
        ("🌐", "Responsive", "Looks great on any device"),
        ("🚀", "Modern", "Built with the latest web technologies"),
    ];

    let handle_submit = Callback::new(move |markdown: String| {
        web_sys::console::log_1(&markdown.into());
    });
    view! {
        // Hero Section
        <section class="py-20 text-center">
            <h1 class="text-5xl font-bold text-gray-900 mb-6">
                "Welcome to "
                <span class="text-blue-600">"LazyCoder"</span>
            </h1>
            <p class="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
                "The ultimate platform for lazy coders who want to build amazing things "
                "without all the hassle."
            </p>
            <div class="space-x-4">
                <button class="bg-blue-600 text-white px-6 py-3 rounded-lg font-medium hover:bg-blue-700 transition-colors">
                    "Get Started"
                </button>
                <button class="border border-gray-300 px-6 py-3 rounded-lg font-medium text-gray-700 hover:bg-gray-50 transition-colors">
                    "Learn More"
                </button>
            </div>
        </section>

        // Features Section
        <section class="py-16 bg-gray-50">
            <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
                <h2 class="text-3xl font-bold text-center text-gray-900 mb-12">
                    "Why Choose LazyCoder?"
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                    {features.into_iter().map(|(icon, title, description)| {
                        view! {
                            <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                                <div class="text-4xl mb-4">{icon}</div>
                                <h3 class="text-xl font-semibold mb-2">{title}</h3>
                                <p class="text-gray-600">{description}</p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>

        // CTA Section
        <section class="py-20 text-center">
            <div class="max-w-3xl mx-auto px-4">
                <h2 class="text-3xl font-bold text-gray-900 mb-6">
                    "Ready to get started?"
                </h2>
                <p class="text-xl text-gray-600 mb-8">
                    "Join thousands of developers who are already building amazing things with LazyCoder."
                </p>
                <button class="bg-blue-600 text-white px-8 py-3 rounded-lg font-medium hover:bg-blue-700 transition-colors text-lg">
                    "Create Your Account"
                </button>
            </div>
        </section>
    }
}
