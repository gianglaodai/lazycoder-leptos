use super::base::ICellRenderer;
use crate::pages::components::datatable::core::render_value::Value;
use leptos::attr::Scope;
use leptos::prelude::AnyView;
use leptos::prelude::*;

pub struct CheckboxRenderer;
impl CheckboxRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl<T> ICellRenderer<T> for CheckboxRenderer {
    fn view(&self, _cx: Scope, value: &Value, _row: &T) -> AnyView {
        let checked = matches!(value, Value::Bool(true));
        view! {
            <input type="checkbox" prop:checked=checked disabled=true class="pointer-events-none" />
        }
        .into_any()
    }
}
