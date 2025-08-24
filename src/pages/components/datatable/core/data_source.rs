use crate::pages::components::datatable::core::agg::AggMap;
use std::{future::Future, pin::Pin};

#[derive(Clone, Debug)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Clone, Debug)]
pub struct SortModel {
    pub col_id: String,
    pub sort: SortOrder,
    pub sort_index: Option<usize>,
}

#[derive(Clone, Debug, Default)]
pub struct FilterModel {
    // placeholder: fill with column -> condition list
    pub quick_text: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct GroupModel {
    pub group_cols: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct PivotModel {
    pub pivot_cols: Vec<String>,
    pub value_cols: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Query {
    pub start_row: usize,
    pub end_row: usize,
    pub sort: Vec<SortModel>,
    pub filter: FilterModel,
    pub group: GroupModel,
    pub pivot: Option<PivotModel>,
}

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub trait DataSource<T: 'static> {
    fn get_rows<'a>(&'a self, _q: Query) -> BoxFuture<'a, Result<RowsResult<T>, DataError>>;
}

#[derive(Clone, Debug)]
pub struct RowsResult<T> {
    pub rows: Vec<T>,
    pub total: Option<usize>,
    pub aggregates: Option<AggMap>,
}

#[derive(thiserror::Error, Debug)]
pub enum DataError {
    #[error("network")]
    Network,
    #[error("server: {0}")]
    Server(String),
    #[error("other")]
    Other,
}

impl<T> RowsResult<T> {
    pub fn empty() -> Self {
        Self {
            rows: Vec::new(),
            total: Some(0),
            aggregates: None,
        }
    }
}
