use super::base::ICellRenderer;
use crate::pages::components::datatable::core::render_value::Value;
use leptos::attr::Scope;
use leptos::prelude::AnyView;
use leptos::prelude::*;

pub struct MoneyRenderer;
impl MoneyRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl<T> ICellRenderer<T> for MoneyRenderer {
    fn view(&self, _cx: Scope, value: &Value, _row: &T) -> AnyView {
        let text = match value {
            Value::Number(n) => format!("${:.2}", n),
            _ => value.to_string(),
        };
        view! { <span class="truncate tabular-nums">{text}</span> }.into_any()
    }
}
