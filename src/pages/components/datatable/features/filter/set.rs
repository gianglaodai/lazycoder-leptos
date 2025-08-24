use super::{ColumnFilterModel, IFilter};
use leptos::prelude::*;

pub struct SetFilter;

impl SetFilter {
    pub fn new() -> Self {
        Self
    }
}

impl IFilter for SetFilter {
    fn view(&self) -> impl IntoView {
        unimplemented!()
    }
    fn as_model(&self) -> ColumnFilterModel {
        unimplemented!()
    }
}
