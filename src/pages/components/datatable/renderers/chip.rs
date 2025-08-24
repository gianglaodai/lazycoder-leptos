use leptos::attr::Scope;
use leptos::IntoView;
use crate::pages::components::datatable::core::render_value::Value;
use super::base::ICellRenderer;


pub struct ChipRenderer;
impl ChipRenderer { pub fn new() -> Self { Self } }


impl<T> ICellRenderer<T> for ChipRenderer {
    fn view(&self, _cx: Scope, value: &Value, _row: &T) -> impl IntoView {
        unimplemented!()
    }
}