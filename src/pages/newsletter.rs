use leptos::prelude::*;
use leptos_router::components::A;
use crate::pages::components::Button;
use crate::pages::components::button::ButtonVariant;

#[component]
pub fn NewsletterPage() -> impl IntoView {
    // State for form inputs
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (is_subscribed, set_is_subscribed) = signal(false);
    let (is_loading, set_loading) = signal(false);

    // Handle form submission
    let handle_submit = move |_| {
        set_loading.set(true);
        // Simulate API call
        set_timeout(
            move || {
                set_is_subscribed.set(true);
                set_loading.set(false);
            },
            std::time::Duration::from_secs(1),
        );
    };

    // Sample newsletter issues
    let issues = vec![
        (
            "Understanding Rust's Ownership Model",
            "2023-10-15",
            "A deep dive into one of Rust's most powerful features that ensures memory safety without garbage collection.",
            "/newsletter/understanding-rust-ownership"
        ),
        (
            "Building Scalable Microservices with Actix-Web",
            "2023-09-28",
            "Learn how to build and deploy high-performance microservices using Rust's Actix-Web framework.",
            "/newsletter/actix-web-microservices"
        ),
        (
            "The Future of WebAssembly",
            "2023-09-10",
            "Exploring how WebAssembly is changing the landscape of web development and what it means for Rust developers.",
            "/newsletter/wasm-future"
        ),
    ];

    view! {
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            {/* Hero Section */}
            <div class="text-center mb-16">
                <h1 class="text-4xl font-bold text-gray-900 mb-4">
                    <span class="text-blue-600">"Newsletter"</span>
                    " & Updates"
                </h1>
                <div class="w-24 h-1 bg-blue-500 mx-auto mb-8"></div>
                <p class="text-xl text-gray-600 max-w-2xl mx-auto">
                    "Stay updated with the latest articles, tutorials, and insights about Rust, Web Development, and more."
                </p>
            </div>

            // Subscription Form
            <div class="bg-white rounded-xl shadow-md overflow-hidden mb-16">
                <div class="p-8">
                    <Show
                        when=move || !is_subscribed.get()
                        fallback= move || view! {
                            <div class="text-center py-8">
                                <div class="text-green-500 text-6xl mb-4">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                                    </svg>
                                </div>
                                <h2 class="text-2xl font-bold text-gray-900 mb-2">"Thank You for Subscribing!"</h2>
                                <p class="text-gray-600 mb-6">
                                    "You've been successfully added to our newsletter. Check your email for the confirmation."
                                </p>
                                <Button variant=ButtonVariant::Link on_click=Callback::new(move |_| set_is_subscribed.set(false))>
                                    "Back to form"
                                </Button>
                            </div>
                        }
                    >
                        <h2 class="text-2xl font-bold text-gray-900 mb-6 text-center">"Subscribe to Our Newsletter"</h2>
                        <form on:submit=move |ev| {
                            ev.prevent_default();
                            handle_submit(ev);
                        } class="space-y-6">
                            <div>
                                <label for="name" class="block text-sm font-medium text-gray-700 mb-1">
                                    "Name"
                                </label>
                                <input
                                    type="text"
                                    id="name"
                                    prop:value=name
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                    required
                                    class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                                    placeholder="Your name"
                                />
                            </div>
                            <div>
                                <label for="email" class="block text-sm font-medium text-gray-700 mb-1">
                                    "Email address"
                                </label>
                                <input
                                    type="email"
                                    id="email"
                                    prop:value=email
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                    required
                                    class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                                    placeholder="you@example.com"
                                />
                            </div>
                            <div class="flex items-start">
                                <div class="flex items-center h-5">
                                    <input
                                        id="terms"
                                        name="terms"
                                        type="checkbox"
                                        required
                                        class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                    />
                                </div>
                                <div class="ml-3 text-sm">
                                    <label for="terms" class="font-medium text-gray-700">
                                        "I agree to the "
                                        <a href="/privacy" class="text-blue-600 hover:text-blue-500">
                                            "Privacy Policy"
                                        </a>
                                    </label>
                                </div>
                            </div>
                            <div>
                                <Button class="w-full" r#type="submit".to_string() disabled=is_loading.get()>Subscribe</Button>
                            </div>
                        </form>
                    </Show>
                </div>
            </div>

            {/* Past Issues */}
            <div>
                <h2 class="text-2xl font-bold text-gray-900 mb-8 text-center">"Past Issues"</h2>
                <div class="space-y-6">
                    {issues.into_iter().map(|(title, date, excerpt, link)| {
                        view! {
                            <div class="bg-white rounded-lg shadow overflow-hidden hover:shadow-md transition-shadow">
                                <div class="p-6">
                                    <div class="flex justify-between items-start">
                                        <div>
                                            <h3 class="text-lg font-semibold text-gray-900">
                                                <A href=link attr:class="hover:text-blue-600">{title}</A>
                                            </h3>
                                            <p class="mt-1 text-sm text-gray-500">{date}</p>
                                        </div>
                                        <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                                            "Read"
                                        </span>
                                    </div>
                                    <p class="mt-3 text-gray-600">
                                        {excerpt}
                                    </p>
                                    <div class="mt-4">
                                        <A
                                            href=link
                                            attr:class="text-sm font-medium text-blue-600 hover:text-blue-500"
                                        >
                                            "Read full issue "
                                            <span aria-hidden="true">"→"</span>
                                        </A>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            {/* FAQ Section */}
            <div class="mt-16">
                <h2 class="text-2xl font-bold text-gray-900 mb-8 text-center">"Frequently Asked Questions"</h2>
                <div class="bg-white shadow overflow-hidden sm:rounded-lg">
                    <div class="px-4 py-5 sm:p-6">
                        <dl class="space-y-8">
                            <div>
                                <dt class="text-lg font-medium text-gray-900">
                                    "How often will I receive the newsletter?"
                                </dt>
                                <dd class="mt-2 text-base text-gray-600">
                                    "We send out our newsletter bi-weekly, typically on Monday mornings. Occasionally, we might send special updates for important announcements."
                                </dd>
                            </div>
                            <div>
                                <dt class="text-lg font-medium text-gray-900">
                                    "Can I unsubscribe at any time?"
                                </dt>
                                <dd class="mt-2 text-base text-gray-600">
                                    "Absolutely! Every newsletter includes an unsubscribe link at the bottom. You can also update your preferences or unsubscribe through your account settings."
                                </dd>
                            </div>
                            <div>
                                <dt class="text-lg font-medium text-gray-900">
                                    "What kind of content can I expect?"
                                </dt>
                                <dd class="mt-2 text-base text-gray-600">
                                    "Our newsletter includes the latest articles, tutorials, industry news, and exclusive content about Rust, Web Development, and related technologies. We also share tips, tools, and resources to help you stay ahead in your development journey."
                                </dd>
                            </div>
                        </dl>
                    </div>
                </div>
            </div>
        </div>
    }
}
