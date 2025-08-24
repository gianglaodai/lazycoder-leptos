// render/footer.rs
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::*;
use leptos::*;
use std::rc::Rc;

#[component]
pub fn StatusBar<T: 'static>(#[prop(into)] state: Rc<TableState<T>>) -> impl IntoView {
    unimplemented!()
}

#[component]
pub fn Pagination<T: 'static>(#[prop(into)] state: Rc<TableState<T>>) -> impl IntoView {
    unimplemented!()
}

pub fn go_to_page(_page: usize) {unimplemented!()
}
pub fn set_page_size(_size: usize) {unimplemented!()
}
