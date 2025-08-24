use leptos::*;
use super::{IFilter, ColumnFilterModel};


pub struct TextFilter;


impl TextFilter { pub fn new() -> Self { Self } }


impl IFilter for TextFilter {
    fn view(&self) -> impl IntoView {
        unimplemented!()
    }
    fn as_model(&self) -> ColumnFilterModel {
        unimplemented!()
    }
}