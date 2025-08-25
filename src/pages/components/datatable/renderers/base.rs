use leptos::prelude::*;
use leptos::prelude::AnyView;
use leptos::attr::Scope;
use crate::pages::components::datatable::core::render_value::Value;

pub trait ICellRenderer<T> {
    fn view(&self, _cx: Scope, _value: &Value, _row: &T) -> AnyView {
        unimplemented!()
    }
}


#[derive(Clone, Default)]
pub struct RendererParams {
    pub class: Option<String>,
}