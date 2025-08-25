use crate::pages::components::datatable::core::column::{ColumnDef, ColumnState};
use crate::pages::components::datatable::core::data_source::{FilterModel, SortModel};
use crate::pages::components::datatable::core::row::{RowNode, SelectionState};
use leptos::prelude::{ReadUntracked, RwSignal, Set, Update};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ViewportState {
    pub first_row: usize,
    pub last_row: usize,
    pub first_col: usize,
    pub last_col: usize,
}

pub struct TableState<T: Send + Sync + 'static> {
    pub columns: RwSignal<Vec<ColumnDef<T>>>,
    pub column_state: RwSignal<HashMap<String, ColumnState>>, // per-column runtime state (width/hidden/pinned/sort)
    pub sort_model: RwSignal<Vec<SortModel>>,
    pub filter_model: RwSignal<FilterModel>,
    pub selection: RwSignal<SelectionState>,
    pub viewport: RwSignal<ViewportState>,
    pub quick_filter: RwSignal<String>,
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
    pub rows: RwSignal<Vec<RowNode<T>>>,
    pub total_rows: RwSignal<Option<usize>>,
    // Client-side pagination (optional):
    pub page_size: RwSignal<usize>,
    pub current_page: RwSignal<usize>,
}

impl<T: Send + Sync + 'static> TableState<T> {
    pub fn new() -> Self {
        Self {
            columns: RwSignal::new(Vec::new()),
            column_state: RwSignal::new(HashMap::new()),
            sort_model: RwSignal::new(Vec::new()),
            filter_model: RwSignal::new(FilterModel::default()),
            selection: RwSignal::new(SelectionState::default()),
            viewport: RwSignal::new(ViewportState::default()),
            quick_filter: RwSignal::new(String::new()),
            loading: RwSignal::new(false),
            error: RwSignal::new(None),
            rows: RwSignal::new(Vec::new()),
            total_rows: RwSignal::new(None),
            page_size: RwSignal::new(50),
            current_page: RwSignal::new(1),
        }
    }

    pub fn set_rows(&self, rows: Vec<RowNode<T>>) {
        self.rows.set(rows);
    }
    pub fn set_total_rows(&self, total: Option<usize>) {
        self.total_rows.set(total);
    }
    pub fn set_loading(&self, b: bool) {
        self.loading.set(b);
    }
    pub fn set_error(&self, msg: Option<String>) {
        self.error.set(msg);
    }
    pub fn push_row(&self, row: RowNode<T>) {
        self.rows.update(|v| v.push(row));
    }
    pub fn rows_en(&self) -> usize {
        // Return the current number of row nodes tracked by the table state.
        // This uses get_untracked to avoid creating reactive dependencies here.
        self.rows.read_untracked().len()
    }
}
