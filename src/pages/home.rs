use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Feature items with icons and descriptions
    let features = vec![
        ("⚡", "Nhanh chóng", "Code xong trước deadline, còn thời gian order trà sữa."),
        ("🔒", "An toàn", "Code sạch, bug ít – khỏi bận tâm fix giữa đêm."),
        ("🤸", "Linh hoạt", "Dự án scale mượt, dev rảnh đi phượt."),
        ("🚀", "Luôn mới", "Tận dụng công nghệ mới để… không phải code lại."),
    ];

    view! {
        <div class="font-serif">
        // Hero Section
        <section class="py-24 bg-gradient-to-b from-white to-stone-100">
            <div class="container-page text-center">
                <h1 class="text-4xl sm:text-5xl lg:text-6xl font-serif font-semibold tracking-tight mb-6">
                    "Lazy để busy"
                </h1>
                <p class="text-lg sm:text-xl text-stone-600 mb-10 max-w-2xl mx-auto prose-balanced">
                    "Máy chạy, dev chơi – Code ít, hiệu quả nhiều.
                    Đây là nơi tôi chia sẻ bí kíp 'lười đúng cách' để bạn code gọn, bug ít, và có thêm thời gian cho cà phê, game, hay ngủ trưa."
                </p>
                <a href="#features" class="px-6 py-3 bg-blue-600 text-white rounded-lg shadow hover:bg-blue-700 transition">
                    "LAZY LIKE A PRO"
                </a>
            </div>
        </section>

        // Features Section
        <section id="features" class="py-24 bg-gradient-to-tr from-stone-50 via-sky-100 to-stone-200">
            <div class="container-page">
                <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-8 sm:gap-10 items-stretch">
                    {features.into_iter().map(|(icon, title, desc)| {
                        view! {
                            <div class="group h-full text-center rounded-2xl bg-gradient-to-br from-stone-100 via-stone-50 to-stone-300 border border-stone-200/80 shadow-md hover:shadow-2xl transition-all duration-300 p-6 sm:p-8 hover:-translate-y-1 overflow-hidden">
                                <div class="mx-auto mb-5 grid w-16 h-16 place-content-center rounded-full bg-gradient-to-tr from-indigo-600 to-sky-400 text-white shadow-xl ring-8 ring-white/60 hover:scale-105 transition-transform duration-300">
                                    <span class="text-3xl">{icon}</span>
                                </div>
                                <h3 class="text-lg sm:text-xl font-serif font-semibold mb-2 text-stone-900">{title}</h3>
                                <p class="text-stone-600 leading-relaxed">{desc}</p>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>

        // About Section
        <section class="py-20 bg-gradient-to-tr from-blue-800 to-emerald-500 text-white **:text-white">
            <div class="container-page text-center max-w-3xl mx-auto">
                <h2 class="text-3xl font-serif font-semibold mb-6">
                    "Vì sao 'lười' lại là siêu năng lực?"
                </h2>
                <p class="text-lg text-stone-600 mb-8">
                    "Trong lập trình, 'lười' không phải là tránh việc – mà là tìm cách làm ít việc nhất để đạt kết quả tốt nhất.
                    Tôi chia sẻ tips tối ưu code, tự động hóa task lặp, và tư duy thiết kế giúp bạn tiết kiệm thời gian, công sức… để làm điều bạn thích (kể cả là ngủ)."
                </p>
                <a href="#mentorship" class="px-6 py-3 bg-green-600 text-white rounded-lg shadow hover:bg-green-700 transition">
                    "JUST DO IT"
                </a>
            </div>
        </section>

        // Mentorship Section
        <section id="mentorship" class="py-20 bg-gradient-to-b from-white via-stone-50 to-emerald-50 text-stone-900">
            <div class="container-page text-center max-w-3xl mx-auto">
                <h2 class="text-3xl font-serif font-semibold mb-6">
                    "Muốn máy chạy, dev chơi?"
                </h2>
                <p class="text-lg text-stone-700 mb-8">
                    "Tôi mentor lập trình viên mới ra trường, giúp họ đi từ ‘code cho xong’ sang ‘code như hacker lão luyện’.
                    Không giáo trình nhàm chán, chỉ có tình huống thực tế, giải pháp gọn nhẹ và tư duy xịn."
                </p>
                <a href="/contact" class="px-6 py-3 bg-blue-600 text-white rounded-lg shadow hover:bg-blue-700 transition">
                    "CONTACT ME"
                </a>
            </div>
        </section>

        // Footer CTA
        <section class="py-20 bg-stone-800 text-stone-100 text-center">
            <div class="container-page">
                <h2 class="text-3xl font-serif font-semibold mb-4">
                    "Lazy để busy – Máy chạy, dev chơi"
                </h2>
                <p class="mb-8 text-stone-300">"Muốn trở thành Lazy Coder chưa?"</p>
                <a href="/start" class="px-6 py-3 bg-emerald-600 text-white rounded-lg shadow hover:bg-emerald-700 transition">
                    "JUST DO IT"
                </a>
            </div>
        </section>
        </div>
    }
}
