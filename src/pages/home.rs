use leptos::prelude::*;

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

    view! {
        // Hero Section
        <section class="py-24">
            <div class="container-page text-center">
                <h1 class="text-4xl sm:text-5xl lg:text-6xl font-serif font-semibold tracking-tight mb-6">
                    "Xây Blog hiện đại, kiểu Kevin Powell"
                </h1>
                <p class="text-lg sm:text-xl text-stone-600 mb-10 max-w-2xl mx-auto prose-balanced">
                    "Đơn giản, tinh tế, tập trung vào nội dung. Dùng Tailwind để có trải nghiệm mượt mà, sạch sẽ và dễ đọc."
                </p>
                <div class="flex items-center justify-center gap-4">
                    <button class="bg-brand-600 text-white px-6 py-3 rounded-full font-medium hover:bg-brand-700 transition-colors shadow-sm">
                        "Bắt đầu ngay"
                    </button>
                    <button class="px-6 py-3 rounded-full font-medium border border-stone-300 text-stone-800 hover:bg-stone-100 transition-colors">
                        "Tìm hiểu thêm"
                    </button>
                </div>
            </div>
        </section>

        // Features Section
        <section class="py-16 bg-white/60">
            <div class="container-page">
                <h2 class="text-2xl sm:text-3xl font-serif font-semibold text-center mb-12">
                    "Vì sao chọn LazyCoder?"
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 lg:gap-8">
                    {features.into_iter().map(|(icon, title, description)| {
                        view! {
                            <div class="bg-white p-6 rounded-2xl border border-stone-200 shadow-sm/20 hover:shadow transition-shadow">
                                <div class="text-4xl mb-3">{icon}</div>
                                <h3 class="text-lg font-semibold mb-1">{title}</h3>
                                <p class="text-stone-600 text-sm leading-6">{description}</p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>

        // CTA Section
        <section class="py-20">
            <div class="container-page text-center">
                <h2 class="text-2xl sm:text-3xl font-serif font-semibold mb-6">
                    "Sẵn sàng bắt đầu?"
                </h2>
                <p class="text-lg text-stone-600 mb-8">
                    "Tham gia cộng đồng developer đang xây dựng blog đẹp và hiệu quả với LazyCoder."
                </p>
                <button class="bg-brand-600 text-white px-8 py-3 rounded-full font-medium hover:bg-brand-700 transition-colors text-base shadow-sm">
                    "Tạo tài khoản"
                </button>
            </div>
        </section>
    }
}
