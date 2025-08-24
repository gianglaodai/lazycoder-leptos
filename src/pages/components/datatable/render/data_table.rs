// render/table.rs
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct DataTableProperties<T: 'static> {
    pub state: Rc<TableState<T>>,
    pub height: Option<String>,
    pub row_height: Option<i32>,
}

#[component]
pub fn DataTable<T: Clone + 'static>(
    #[prop(into)] state: Rc<TableState<T>>,
    #[prop(optional)] height: Option<String>,
    #[prop(optional)] row_height: Option<i32>,
) -> impl IntoView {
    unimplemented!()
}
