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
        to_html_with_options(&content.get(), &options).unwrap_or_else(|_| "Error rendering markdown".to_string())
    };

    view! {
        <div class="max-w-6xl mx-auto p-6">
            <h2 class="text-2xl font-bold text-gray-800 mb-6">Markdown Editor</h2>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                // Input section
                <div class="space-y-4">
                    <h3 class="text-lg font-semibold text-gray-700">Editor</h3>
                    <textarea
                        class="w-full h-96 p-4 border border-gray-300 rounded-lg resize-none focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm"
                        placeholder="Type your markdown here..."
                        prop:value=content
                        on:input=move |ev| {
                            set_content.set(event_target_value(&ev));
                        }
                    />
                </div>
                
                // Preview section
                <div class="space-y-4">
                    <h3 class="text-lg font-semibold text-gray-700">Preview</h3>
                    <article
                        class="w-full h-96 p-4 border border-gray-300 rounded-lg bg-gray-50 overflow-auto prose prose-lg max-w-none prose-code:font-mono prose-pre:font-mono"
                        inner_html=rendered_html
                    />
                </div>
            </div>
            
            // Submit button
            <div class="mt-4 flex justify-end">
                <button
                    class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
                    on:click=move |_| {
                        on_submit.run(content.get());
                    }
                >
                    "Submit Changes"
                </button>
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