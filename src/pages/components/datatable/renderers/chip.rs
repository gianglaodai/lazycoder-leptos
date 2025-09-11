use super::base::ICellRenderer;
use crate::pages::components::datatable::core::render_value::Value;
use leptos::attr::Scope;
use leptos::prelude::AnyView;
use leptos::prelude::*;

pub struct ChipRenderer;
impl ChipRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl<T> ICellRenderer<T> for ChipRenderer {
    fn view(&self, _cx: Scope, value: &Value, _row: &T) -> AnyView {
        let text = value.to_string();
        view! { <span class="px-2 py-0.5 rounded-full bg-blue-50 text-blue-700 text-xs">{text}</span> }.into_any()
    }
}
