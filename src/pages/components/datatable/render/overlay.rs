// render/overlay.rs
use leptos::*;
use leptos::prelude::*;
use std::rc::Rc;
use crate::pages::components::datatable::core::state::TableState;

#[component]
pub fn LoadingOverlay<T: Send + Sync + 'static>(#[prop(into)] state: Rc<TableState<T>>) -> impl IntoView {
    unimplemented!()
}

#[component]
pub fn EmptyOverlay<T: Send + Sync + 'static>(#[prop(into)] state: Rc<TableState<T>>) -> impl IntoView {
    unimplemented!()
}

#[component]
pub fn ErrorOverlay<T: Send + Sync + 'static>(#[prop(into)] state: Rc<TableState<T>>) -> impl IntoView {
    unimplemented!()
}
