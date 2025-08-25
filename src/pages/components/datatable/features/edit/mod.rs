use crate::pages::components::datatable::core::render_value::Value;
use leptos::prelude::*;
use leptos::prelude::AnyView;

pub mod editors;

pub trait ICellEditor<T> {
    fn start_value(&self, _v: &Value) -> Value {
        Value::Empty
    }
    fn view(&self, _row: &T) -> AnyView {
        unimplemented!()
    }
    fn get_value(&self) -> Value {
        Value::Empty
    }
    fn on_key(&self, _key: &str) {
        unimplemented!()
    }
}

pub struct EditService;

impl EditService {
    pub fn new() -> Self {
        Self
    }
    pub fn begin_edit<T>(&mut self, _row_id: &str, _col_id: &str) {
        unimplemented!()
    }
    pub fn commit<T>(&mut self) {
        unimplemented!()
    }
    pub fn cancel<T>(&mut self) {
        unimplemented!()
    }
}
