use leptos::prelude::*;
use leptos::*;

pub mod date;
pub mod float;
pub mod set;
pub mod text;
pub mod integer;
pub mod time;
pub mod datetime;
pub mod boolean;

#[derive(Clone, Debug, Default)]
pub struct ColumnFilterModel {
    pub col_id: String,
    pub operator: String,
    pub value: String,
}

pub trait IFilter {
    fn view(&self) -> impl IntoView; // UI skeleton (can be replaced with component fn)
    fn as_model(&self) -> ColumnFilterModel; // serialize to model
}

pub struct FilterService;

impl FilterService {
    pub fn new() -> Self {
        Self
    }
    pub fn set_model(&mut self, _filters: Vec<ColumnFilterModel>) { /* empty */
    }
    pub fn get_model(&self) -> Vec<ColumnFilterModel> {
        Vec::new()
    }
}
