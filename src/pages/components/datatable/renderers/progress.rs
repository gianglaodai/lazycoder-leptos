use super::base::ICellRenderer;
use crate::pages::components::datatable::core::render_value::Value;
use leptos::attr::Scope;
use leptos::prelude::AnyView;
use leptos::prelude::*;

pub struct ProgressRenderer;
impl ProgressRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl<T> ICellRenderer<T> for ProgressRenderer {
    fn view(&self, _cx: Scope, value: &Value, _row: &T) -> AnyView {
        let pct = match value {
            Value::Number(n) => (*n).max(0.0).min(100.0),
            _ => 0.0,
        };
        let w = format!("{}%", pct);
        view! {
            <div class="w-full h-2 bg-gray-100 rounded">
                <div class="h-2 bg-green-500 rounded" style=move || format!("width:{};", w.clone())></div>
            </div>
        }.into_any()
    }
}
