use leptos::attr::Scope;
use leptos::prelude::AnyView;
use crate::pages::components::datatable::core::render_value::Value;
use super::base::ICellRenderer;


pub struct DateRenderer;
impl DateRenderer { pub fn new() -> Self { Self } }


impl<T> ICellRenderer<T> for DateRenderer {
    fn view(&self, _cx: Scope, value: &Value, _row: &T) -> AnyView {
        unimplemented!()
    }
}