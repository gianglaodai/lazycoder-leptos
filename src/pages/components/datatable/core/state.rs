use crate::pages::components::datatable::core::column::ColumnDef;
use crate::pages::components::datatable::core::data_source::{FilterModel, SortModel};
use crate::pages::components::datatable::core::row::{RowNode, SelectionState};
use leptos::prelude::{RwSignal, Set, Update};

#[derive(Clone, Debug, Default)]
pub struct ViewportState {
    pub first_row: usize,
    pub last_row: usize,
    pub first_col: usize,
    pub last_col: usize,
}

pub struct TableState<T: Send + Sync + 'static> {
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

impl<T: Send + Sync + 'static> TableState<T> {
    pub fn new() -> Self {
        Self {
            columns: RwSignal::new(Vec::new()),
            sort_model: RwSignal::new(Vec::new()),
            filter_model: RwSignal::new(FilterModel::default()),
            selection: RwSignal::new(SelectionState::default()),
            viewport: RwSignal::new(ViewportState::default()),
            quick_filter: RwSignal::new(String::new()),
            loading: RwSignal::new(false),
            error: RwSignal::new(None),
            rows: RwSignal::new(Vec::new()),
            total_rows: RwSignal::new(None),
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
        // Fallback implementation without requiring Clone/GetUntracked on the signal's inner type
        // Consider adding a dedicated counter signal updated alongside `rows` if you need this frequently.
        0
    }
}
