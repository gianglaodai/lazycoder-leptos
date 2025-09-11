use crate::pages::components::datatable::core::render_value::Value;
use leptos::attr::Scope;
use leptos::prelude::AnyView;
use leptos::prelude::*;

pub trait ICellRenderer<T> {
    fn view(&self, _cx: Scope, value: &Value, _row: &T) -> AnyView {
        // Safe default: render value as text
        let text = value.to_string();
        view! { <span class="truncate">{text}</span> }.into_any()
    }
}

#[derive(Clone, Default)]
pub struct RendererParams {
    pub class: Option<String>,
}
