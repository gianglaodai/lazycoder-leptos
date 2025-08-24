use crate::pages::components::datatable::core::column::ColumnState;
use crate::pages::components::datatable::core::data_source::{FilterModel, SortModel};
use crate::pages::components::datatable::core::render_value::Value;
use crate::pages::components::datatable::core::row::RowNode;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct CellRef {
    pub row_id: String,
    pub col_id: String,
}

#[derive(Clone, Debug)]
pub struct CellChange<T> {
    pub cell: CellRef,
    pub old_value: Value,
    pub new_value: Value,
    pub row: RowNode<T>,
}

#[derive(Clone, Debug)]
pub enum GridEvent<T> {
    CellClicked(CellRef),
    CellValueChanged(CellChange<T>),
    SortChanged(Vec<SortModel>),
    FilterChanged(FilterModel),
    SelectionChanged(usize),
    ColumnMoved { from: usize, to: usize },
    ColumnResized { id: String, width: i32 },
}

#[derive(Clone)]
pub struct GridApi<T> {
    pub refresh_cells: Rc<dyn Fn(CellRefreshOpts)>,
    pub set_quick_filter: Rc<dyn Fn(String)>,
    pub export_csv: Rc<dyn Fn(ExportOpts)>,
    pub set_column_state: Rc<dyn Fn(Vec<ColumnState>)>,
    pub get_selected_rows: Rc<dyn Fn() -> Vec<T>>,
}

#[derive(Clone, Debug, Default)]
pub struct CellRefreshOpts {
    pub row_ids: Option<Vec<String>>,
    pub col_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
pub struct ExportOpts {
    pub file_name: Option<String>,
    pub include_headers: bool,
    pub delimiter: char,
}

impl<T> GridApi<T> {
    pub fn new() -> Self {
        Self {
            refresh_cells: Rc::new(|_opts| unimplemented!()),
            set_quick_filter: Rc::new(|_q| unimplemented!()),
            export_csv: Rc::new(|_opts| unimplemented!()),
            set_column_state: Rc::new(|_state| unimplemented!()),
            get_selected_rows: Rc::new(|| Vec::new()),
        }
    }
}
