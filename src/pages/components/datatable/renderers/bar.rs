use super::base::ICellRenderer;
use crate::pages::components::datatable::core::render_value::Value;
use leptos::attr::Scope;
use leptos::prelude::AnyView;

pub struct BarRenderer;
impl BarRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl<T> ICellRenderer<T> for BarRenderer {
    fn view(&self, _cx: Scope, value: &Value, _row: &T) -> AnyView {
        unimplemented!()
    }
}
