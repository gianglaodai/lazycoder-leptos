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
        to_html_with_options(&content, &options)
            .unwrap_or_else(|_| "Error rendering markdown".to_string())
    };

    let container_class = format!("max-w-4xl mx-auto p-6 {}", class.unwrap_or_default());

    view! {
        <div class=container_class>
            {title.map(|t| view! {
                <h2 class="text-2xl font-bold text-foreground mb-6">{t}</h2>
            })}

            <div
                class="prose max-w-none prose-zinc dark:prose-invert prose-a:text-primary prose-strong:text-foreground prose-code:bg-muted prose-code:text-foreground prose-code:px-1 prose-code:rounded prose-pre:bg-muted"
                inner_html=rendered_html
            />
        </div>
    }
}
