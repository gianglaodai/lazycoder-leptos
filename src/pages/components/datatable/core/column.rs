use std::rc::Rc;

use crate::pages::components::datatable::core::agg::AggregateFn;
use crate::pages::components::datatable::core::data_source::SortOrder;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pinned {
    None,
    Left,
    Right,
}

#[derive(Clone)]
pub struct ColumnDef<T: 'static> {
    pub id: &'static str,
    pub header_name: &'static str,
    pub value_getter: Option<Rc<dyn Fn(&T) -> Value>>,
    pub value_formatter: Option<Rc<dyn Fn(&Value) -> String>>,
    pub cell_renderer: Option<Rc<dyn std::any::Any>>,
    pub cell_editor: Option<Rc<dyn std::any::Any>>,
    pub sortable: bool,
    pub filterable: bool,
    pub resizable: bool,
    pub movable: bool,
    pub pinned: Pinned,
    pub width: i32,
    pub min_width: i32,
    pub max_width: Option<i32>,
    pub groupable: bool,
    pub aggregate: Option<AggregateFn>,
    pub comparator: Option<Rc<dyn Fn(&Value, &Value) -> std::cmp::Ordering>>,
    pub field: Option<&'static str>,
}

#[derive(Clone, Debug, Default)]
pub struct ColumnState {
    pub id: String,
    pub width: Option<i32>,
    pub pinned: Option<Pinned>,
    pub hidden: Option<bool>,
    pub sort: Option<SortOrder>,
    pub sort_index: Option<usize>,
}

pub struct ColumnApi;

impl ColumnApi {
    pub fn set_visible(&self, _col_id: &str, _visible: bool) {
        unimplemented!()
    }
    pub fn set_width(&self, _col_id: &str, _width: i32) {
        unimplemented!()
    }
    pub fn move_column(&self, _from: usize, _to: usize) {
        unimplemented!()
    }
    pub fn set_column_state(&self, _state: Vec<ColumnState>) {
        unimplemented!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Text(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Date(String),
    Empty,
}
