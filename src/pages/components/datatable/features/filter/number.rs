use super::{ColumnFilterModel, IFilter};
use leptos::prelude::*;

pub struct NumberFilter;

impl NumberFilter {
    pub fn new() -> Self {
        Self
    }
}

impl IFilter for NumberFilter {
    fn view(&self) -> impl IntoView {
        unimplemented!()
    }
    fn as_model(&self) -> ColumnFilterModel {
        unimplemented!()
    }
}
