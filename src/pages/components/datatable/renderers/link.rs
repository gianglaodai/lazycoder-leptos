use super::base::ICellRenderer;
use crate::pages::components::datatable::core::render_value::Value;
use leptos::attr::Scope;
use leptos::prelude::AnyView;
use leptos::prelude::*;

pub struct LinkRenderer;
impl LinkRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl<T> ICellRenderer<T> for LinkRenderer {
    fn view(&self, cx: Scope, value: &Value, _row: &T) -> AnyView {
        let txt = value.to_string();
        // naive URL detection
        let is_url = txt.starts_with("http://") || txt.starts_with("https://");
        if is_url {
            view! { <a href={txt.clone()} target="_blank" class="text-blue-600 hover:underline truncate">{txt.clone()}</a> }.into_any()
        } else {
            view! { <span class="truncate">{txt}</span> }.into_any()
        }
    }
}
