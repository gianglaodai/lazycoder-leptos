use leptos::attr::Scope;
use leptos::IntoView;
use crate::pages::components::datatable::core::render_value::Value;
use super::base::ICellRenderer;


pub struct MoneyRenderer;
impl MoneyRenderer { pub fn new() -> Self { Self } }


impl<T> ICellRenderer<T> for MoneyRenderer {
    fn view(&self, _cx: Scope, value: &Value, _row: &T) -> impl IntoView {
        unimplemented!()
    }
}