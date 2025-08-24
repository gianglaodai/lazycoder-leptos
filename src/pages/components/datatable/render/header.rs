use leptos::*;
use leptos::prelude::*;
use std::rc::Rc;
use crate::pages::components::datatable::core::column::ColumnDef;
use crate::pages::components::datatable::core::state::TableState;

#[component]
pub fn HeaderRow<T: 'static>(#[prop(into)] state: Rc<TableState<T>>) -> impl IntoView {
    unimplemented!()
}

#[component]
pub fn HeaderCell<T: 'static>(col: ColumnDef<T>) -> impl IntoView {
    unimplemented!()
}

pub fn attach_resize_handle<T>(_col: &ColumnDef<T>) {
    unimplemented!()
}
pub fn start_drag_move<T>(_col: &ColumnDef<T>) {
    unimplemented!()
}
