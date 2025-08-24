use leptos::prelude::*;
use super::{IFilter, ColumnFilterModel};


pub struct DateFilter;


impl DateFilter { pub fn new() -> Self { Self } }


impl IFilter for DateFilter {
    fn view(&self) -> impl IntoView {
        unimplemented!()
    }
    fn as_model(&self) -> ColumnFilterModel {
        unimplemented!()
    }
}