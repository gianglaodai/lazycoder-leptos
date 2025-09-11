use super::base::ICellRenderer;
use crate::pages::components::datatable::core::render_value::Value;
use leptos::attr::Scope;
use leptos::prelude::AnyView;
use leptos::prelude::*;

pub struct BarRenderer;
impl BarRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl<T> ICellRenderer<T> for BarRenderer {
    fn view(&self, _cx: Scope, value: &Value, _row: &T) -> AnyView {
        let pct = match value {
            Value::Number(n) => (*n).max(0.0).min(100.0),
            _ => 0.0,
        };
        let w = format!("{}%", pct);
        view! {
            <div class="w-full h-2 bg-gray-200 rounded">
                <div class="h-2 bg-blue-500 rounded" style=move || format!("width:{};", w.clone())></div>
            </div>
        }.into_any()
    }
}
