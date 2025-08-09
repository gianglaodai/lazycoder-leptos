use leptos::prelude::*;
use time::OffsetDateTime;

#[component]
pub fn Footer() -> impl IntoView {
    let year = OffsetDateTime::now_utc().year();
    view! {
        <footer class="font-serif bg-[--color-bg]/80 backdrop-blur supports-[backdrop-filter]:bg-[--color-bg]/80 px-9 py-8 shadow-[0_0_2em_rgba(0,0,0,0.05)] sm:mx-0 border-t border-black/5">
            <div class="container-page">
                <div class="flex items-center justify-center">
                    <p class="text-lg text-[--color-ink]/80">
                        {format!("© {} LazyCoder's Blog. All rights reserved.", year)}
                    </p>
                </div>
            </div>
        </footer>
    }
}
