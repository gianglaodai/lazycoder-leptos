use leptos::prelude::*;

#[component]
pub fn AboutMePage() -> impl IntoView {
    let skills = vec![
        ("Rust", 90),
        ("Web Development", 85),
        ("Systems Programming", 88),
        ("Cloud Architecture", 82),
        ("DevOps", 80),
        ("Machine Learning", 75),
    ];

    let experience = vec![
        (
            "Senior Software Engineer",
            "TechCorp Inc.",
            "2020 - Present",
            "Leading a team of developers in building scalable backend systems with Rust and microservices architecture."
        ),
        (
            "Full Stack Developer",
            "WebSolutions Ltd.",
            "2017 - 2020",
            "Developed and maintained multiple web applications using modern JavaScript frameworks and RESTful APIs."
        ),
        (
            "Junior Developer",
            "StartUp Innovations",
            "2015 - 2017",
            "Contributed to various projects, focusing on frontend development and user experience improvements."
        ),
    ];

    view! {
        <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            {/* Hero Section */}
            <div class="text-center mb-16">
                <h1 class="text-4xl font-bold text-gray-900 mb-4">
                    "About "
                    <span class="text-blue-600">"Me"</span>
                </h1>
                <div class="w-24 h-1 bg-blue-500 mx-auto mb-8"></div>
                <p class="text-xl text-gray-600 max-w-3xl mx-auto">
                    "Passionate software engineer with a love for building efficient, scalable, and maintainable systems."
                </p>
            </div>

            {/* Profile Section */}
            <div class="grid md:grid-cols-3 gap-12 mb-16">
                <div class="md:col-span-1 flex justify-center">
                    <div class="w-64 h-64 rounded-full overflow-hidden border-4 border-blue-500 shadow-lg">
                        <div class="w-full h-full bg-gray-200 flex items-center justify-center text-gray-400">
                            <svg class="w-32 h-32" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path>
                            </svg>
                        </div>
                    </div>
                </div>
                <div class="md:col-span-2">
                    <h2 class="text-2xl font-semibold text-gray-800 mb-4">"Hello, I'm John Doe"</h2>
                    <p class="text-gray-600 mb-4">
                        "I'm a passionate software engineer with over 8 years of experience in building web applications and systems.
                        My journey in programming started when I was 15, and I've been in love with coding ever since."
                    </p>
                    <p class="text-gray-600 mb-6">
                        "When I'm not coding, you can find me hiking in the mountains, reading science fiction, or experimenting
                        with new technologies and programming languages."
                    </p>
                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <p class="font-medium text-gray-800">"Email:"</p>
                            <p class="text-blue-600">"john.doe@example.com"</p>
                        </div>
                        <div>
                            <p class="font-medium text-gray-800">"Location:"</p>
                            <p class="text-gray-600">"San Francisco, CA"</p>
                        </div>
                        <div>
                            <p class="font-medium text-gray-800">"GitHub:"</p>
                            <a href="https://github.com/username" class="text-blue-600 hover:underline">"github.com/username"</a>
                        </div>
                        <div>
                            <p class="font-medium text-gray-800">"LinkedIn:"</p>
                            <a href="https://linkedin.com/in/username" class="text-blue-600 hover:underline">"linkedin.com/in/username"</a>
                        </div>
                    </div>
                </div>
            </div>

            {/* Skills Section */}
            <div class="mb-16">
                <h2 class="text-2xl font-semibold text-gray-800 mb-8 text-center">"My Skills"</h2>
                <div class="grid md:grid-cols-2 gap-6">
                    {skills.into_iter().map(|(skill, level)| {
                        view! {
                            <div>
                                <div class="flex justify-between mb-1">
                                    <span class="text-gray-700">{skill}</span>
                                    <span class="text-gray-500">{format!("{}%", level)}</span>
                                </div>
                                <div class="w-full bg-gray-200 rounded-full h-2.5">
                                    <div
                                        class="bg-blue-600 h-2.5 rounded-full"
                                        style=format!("width: {}%;", level)
                                    ></div>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            {/* Experience Section */}
            <div class="mb-16">
                <h2 class="text-2xl font-semibold text-gray-800 mb-8 text-center">"Work Experience"</h2>
                <div class="space-y-8">
                    {experience.into_iter().map(|(title, company, period, description)| {
                        view! {
                            <div class="bg-white p-6 rounded-lg shadow-sm hover:shadow-md transition-shadow">
                                <div class="flex flex-col md:flex-row md:justify-between md:items-center mb-2">
                                    <h3 class="text-xl font-semibold text-gray-800">{title}</h3>
                                    <span class="text-blue-600 font-medium">{company}</span>
                                </div>
                                <p class="text-gray-500 mb-4">{period}</p>
                                <p class="text-gray-600">{description}</p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            {/* Call to Action */}
            <div class="text-center bg-blue-50 p-8 rounded-lg">
                <h2 class="text-2xl font-semibold text-gray-800 mb-4">"Let's Work Together"</h2>
                <p class="text-gray-600 mb-6 max-w-2xl mx-auto">
                    "I'm always open to discussing new projects, creative ideas, or opportunities to be part of your vision."
                </p>
                <a
                    href="/contact"
                    class="inline-block bg-blue-600 text-white px-6 py-3 rounded-lg font-medium hover:bg-blue-700 transition-colors"
                >
                    "Get In Touch"
                </a>
            </div>
        </div>
    }
}
