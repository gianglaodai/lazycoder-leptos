use leptos::prelude::*;
use markdown::{to_html_with_options, Options};

#[component]
pub fn MarkdownViewer(
    #[prop(into)] content: String,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let rendered_html = move || {
        let options = Options::gfm();
        to_html_with_options(&content, &options).unwrap_or_else(|_| "Error rendering markdown".to_string())
    };

    let container_class = format!(
        "max-w-4xl mx-auto p-6 {}",
        class.unwrap_or_default()
    );

    view! {
        <div class=container_class>
            {title.map(|t| view! {
                <h2 class="text-2xl font-bold text-gray-800 mb-6">{t}</h2>
            })}

            <div
                class="prose prose-lg max-w-none prose-headings:text-gray-800 prose-p:text-gray-700 prose-a:text-blue-600 prose-strong:text-gray-900 prose-code:text-pink-600 prose-code:bg-gray-100 prose-code:px-1 prose-code:rounded prose-pre:bg-gray-900 prose-pre:text-gray-100"
                inner_html=rendered_html
            />
        </div>
    }
}