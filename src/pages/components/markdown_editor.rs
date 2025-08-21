use crate::pages::components::Textarea;
use leptos::prelude::*;
use markdown::{to_html_with_options, Options};

#[component]
pub fn MarkdownEditor(
    #[prop(optional)] initial_content: Option<String>,
    on_submit: Callback<String>,
) -> impl IntoView {
    let (content, set_content) = signal(initial_content.unwrap_or_default());

    let rendered_html = move || {
        let options = Options::gfm();
        to_html_with_options(&content.get(), &options)
            .unwrap_or_else(|_| "Error rendering markdown".to_string())
    };

    view! {
        <div>
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="space-y-4">
                    <Textarea
                        class="h-96 resize-none font-mono text-sm"
                        placeholder="Type your markdown here..."
                        value=content
                        on_input=Callback::new(move |ev: leptos::ev::Event| {
                            set_content.set(event_target_value(&ev));
                        })
                        on_blur=Callback::new(move |_: leptos::ev::FocusEvent| {
                            on_submit.run(content.get());
                        })
                    />
                </div>

                <div class="space-y-4">
                    <article
                        class="w-full h-96 p-4 border border-gray-300 rounded-lg bg-gray-50 overflow-auto prose prose-lg max-w-none prose-code:font-mono prose-pre:font-mono"
                        inner_html=rendered_html
                    />
                </div>
            </div>


            // Toolbar with common markdown shortcuts
            <div class="mt-6 p-4 bg-gray-100 rounded-lg">
                <h4 class="text-sm font-semibold text-gray-600 mb-2">Quick Reference:</h4>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-2 text-xs text-gray-600">
                    <div>"**bold**" -> <strong>bold</strong></div>
                    <div>"*italic*" -> <em>italic</em></div>
                    <div>"# Header" -> Header</div>
                    <div>"[link](url)" -> link</div>
                </div>
            </div>
        </div>
    }
}
