use crate::pages::components::datatable::core::column::ColumnDef;
use crate::pages::components::datatable::core::data_source::{FilterModel, SortModel};
use crate::pages::components::datatable::core::row::{RowNode, SelectionState};
use leptos::prelude::RwSignal;

#[derive(Clone, Debug, Default)]
pub struct ViewportState {
    pub first_row: usize,
    pub last_row: usize,
    pub first_col: usize,
    pub last_col: usize,
}

pub struct TableState<T: 'static> {
    pub columns: RwSignal<Vec<ColumnDef<T>>>,
    pub sort_model: RwSignal<Vec<SortModel>>,
    pub filter_model: RwSignal<FilterModel>,
    pub selection: RwSignal<SelectionState>,
    pub viewport: RwSignal<ViewportState>,
    pub quick_filter: RwSignal<String>,
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
    pub rows: RwSignal<Vec<RowNode<T>>>,
    pub total_rows: RwSignal<Option<usize>>,
}

impl<T: 'static> TableState<T> {
    pub fn new() -> Self {
        Self {
            columns: RwSignal::new(vec![]),
            sort_model: RwSignal::new(vec![]),
            filter_model: RwSignal::new(FilterModel::default()),
            selection: RwSignal::new(SelectionState::default()),
            viewport: RwSignal::new(ViewportState::default()),
            quick_filter: RwSignal::new(String::new()),
            loading: RwSignal::new(false),
            error: RwSignal::new(None),
            rows: RwSignal::new(vec![]),
            total_rows: RwSignal::new(None),
        }
    }

    pub fn set_rows(&self, _rows: Vec<RowNode<T>>) {
        unimplemented!()
    }
    pub fn set_total_rows(&self, _total: Option<usize>) {
        unimplemented!()
    }
    pub fn set_loading(&self, _b: bool) {
        unimplemented!()
    }
    pub fn set_error(&self, _msg: Option<String>) {
        unimplemented!()
    }
    pub fn push_row(&self, _row: RowNode<T>) {
        unimplemented!()
    }
    pub fn rows_en(&self) -> usize {
        unimplemented!()
    }
}
