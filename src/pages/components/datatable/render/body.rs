use crate::pages::components::datatable::core::column::ColumnDef;
use crate::pages::components::datatable::core::row::RowNode;
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::*;
use leptos::*;
use std::rc::Rc;

#[component]
pub fn VirtualizedBody<T: Clone + 'static>(
    #[prop(into)] state: Rc<TableState<T>>,
    row_height: i32,
) -> impl IntoView {
    unimplemented!()
}

#[component]
pub fn RowRenderer<T: Clone + 'static>(
    row: RowNode<T>,
    cols: Vec<ColumnDef<T>>,
    row_height: i32,
) -> impl IntoView {
    unimplemented!()
}

#[component]
pub fn CellHost<T: Clone + 'static>(row: RowNode<T>, col: ColumnDef<T>) -> impl IntoView {
    unimplemented!()
}

pub fn handle_row_click<T>(_row: &RowNode<T>) {
    unimplemented!()
}
pub fn handle_cell_click<T>(_row: &RowNode<T>, _col: &ColumnDef<T>) {
    unimplemented!()
}