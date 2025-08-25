use std::sync::Arc;

use crate::pages::components::datatable::core::agg::AggregateFn;
use crate::pages::components::datatable::core::data_source::SortOrder;
use crate::pages::components::datatable::core::render_value::Value;
use crate::pages::components::datatable::renderers::base::ICellRenderer;

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
    pub value_getter: Option<Arc<dyn Fn(&T) -> Value + Send + Sync>>,
    pub value_formatter: Option<Arc<dyn Fn(&Value) -> String + Send + Sync>>,
    pub cell_renderer: Option<Arc<dyn ICellRenderer<T> + Send + Sync>>,
    pub cell_editor: Option<Arc<dyn std::any::Any + Send + Sync>>,
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
    pub comparator: Option<Arc<dyn Fn(&Value, &Value) -> std::cmp::Ordering + Send + Sync>>,
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
