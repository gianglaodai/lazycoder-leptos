use crate::pages::components::datatable::core::data_source::SortModel;
use crate::pages::components::datatable::core::render_value::Value;
use crate::pages::components::datatable::core::row::RowNode;

pub type Comparator = fn(a: &Value, b: &Value) -> std::cmp::Ordering;


pub struct SortService {
    // registry by column id -> comparator
}


impl SortService {
    pub fn new() -> Self { Self { } }
    pub fn set_model(&mut self, _model: Vec<SortModel>) {
        unimplemented!()
    }
    pub fn get_model(&self) -> Vec<SortModel> { Vec::new() }
    pub fn register_comparator(&mut self, _col_id: &str, _cmp: Comparator) {
        unimplemented!()
    }
    pub fn sort_slice(&self, _rows: &mut [RowNode<serde_json::Value>]) {
        unimplemented!()
    }
}