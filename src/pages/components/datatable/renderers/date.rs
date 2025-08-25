use super::base::ICellRenderer;
use crate::pages::components::datatable::core::render_value::Value;
use leptos::attr::Scope;
use leptos::prelude::AnyView;
use leptos::prelude::*;

pub struct DateRenderer;
impl DateRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl<T> ICellRenderer<T> for DateRenderer {
    fn view(&self, cx: Scope, value: &Value, _row: &T) -> AnyView {
        // For now assume Value::Date is an ISO-like string and just display it
        let text = match value {
            Value::Date(s) => s.clone(),
            _ => value.to_string(),
        };
        view! { <span class="truncate">{text}</span> }.into_any()
    }
}
