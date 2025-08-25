use crate::pages::components::datatable::core::render_value::Value;
use leptos::attr::Scope;
use leptos::prelude::AnyView;
use leptos::prelude::*;

pub mod editors;

pub trait ICellEditor<T> {
    fn start_value(&self, v: &Value) -> Value {
        v.clone()
    }
    fn view(&self, cx: Scope, _row: &T) -> AnyView {
        // Safe no-op editor view to avoid panics when invoked before editing is fully wired.
        view! { <span></span> }.into_any()
    }
    fn get_value(&self) -> Value {
        Value::Empty
    }
    fn on_key(&self, _key: &str) {
        // no-op by default
    }
}

pub struct EditService;

impl EditService {
    pub fn new() -> Self {
        Self
    }
    /// Begin editing a cell; currently a safe no-op placeholder.
    pub fn begin_edit<T>(&mut self, _row_id: &str, _col_id: &str) {
        // editing pipeline not yet wired; avoid panic by doing nothing
    }
    /// Commit current edit; currently a safe no-op placeholder.
    pub fn commit<T>(&mut self) {
        // no-op for now
    }
    /// Cancel current edit; currently a safe no-op placeholder.
    pub fn cancel<T>(&mut self) {
        // no-op for now
    }
}
