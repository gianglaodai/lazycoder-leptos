use leptos::attr::Scope;
use leptos::prelude::AnyView;
use crate::pages::components::datatable::core::render_value::Value;
use super::base::ICellRenderer;


pub struct ProgressRenderer;
impl ProgressRenderer { pub fn new() -> Self { Self } }


impl<T> ICellRenderer<T> for ProgressRenderer {
    fn view(&self, _cx: Scope, value: &Value, _row: &T) -> AnyView {
        unimplemented!()
    }
}