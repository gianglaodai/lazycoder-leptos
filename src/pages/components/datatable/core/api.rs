use crate::pages::components::datatable::core::column::ColumnState;
use crate::pages::components::datatable::core::data_source::{FilterModel, SortModel};
use crate::pages::components::datatable::core::render_value::Value;
use crate::pages::components::datatable::core::row::RowNode;
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::{ReadUntracked, Set, Update, With};
use std::rc::Rc;
use std::sync::Arc;

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

impl<T: Clone + Send + Sync + 'static> GridApi<T> {
    pub fn new(state: Arc<TableState<T>>) -> Self {
        let state_for_refresh = state.clone();
        let refresh_cells = Rc::new(move |_opts: CellRefreshOpts| {
            // Nudge a reactive update by touching rows (no structural change)
            state_for_refresh.rows.update(|_| {});
        });
        let set_quick_filter = {
            let state = state.clone();
            Rc::new(move |q: String| {
                state.quick_filter.set(q.clone());
                state.filter_model.update(|f| f.quick_text = Some(q));
            })
        };
        let set_column_state = {
            let state = state.clone();
            Rc::new(move |list: Vec<ColumnState>| {
                // Merge into column_state map
                state.column_state.update(|m| {
                    for s in list.iter() {
                        m.entry(s.id.clone())
                            .and_modify(|e| {
                                if let Some(w) = s.width {
                                    e.width = Some(w);
                                }
                                if let Some(p) = s.pinned {
                                    e.pinned = Some(p);
                                }
                                if let Some(h) = s.hidden {
                                    e.hidden = Some(h);
                                }
                                if let Some(sort) = &s.sort {
                                    e.sort = Some(sort.clone());
                                }
                                if let Some(idx) = s.sort_index {
                                    e.sort_index = Some(idx);
                                }
                            })
                            .or_insert_with(|| s.clone());
                    }
                });
                // Also reflect sort into sort_model
                let mut sm: Vec<SortModel> = list
                    .iter()
                    .filter_map(|s| {
                        s.sort
                            .clone()
                            .map(|sort| (s.id.clone(), sort, s.sort_index))
                    })
                    .map(|(id, sort, idx)| SortModel {
                        col_id: id,
                        sort,
                        sort_index: idx,
                    })
                    .collect();
                sm.sort_by(|a, b| match (a.sort_index, b.sort_index) {
                    (Some(x), Some(y)) => x.cmp(&y),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.col_id.cmp(&b.col_id),
                });
                if !sm.is_empty() {
                    state.sort_model.set(sm);
                }
            })
        };
        let state_for_export = state.clone();
        let export_csv = Rc::new(move |opts: ExportOpts| {
            // Build CSV from visible columns and current (unfiltered) rows.
            let delimiter = if opts.delimiter == '\0' { ',' } else { opts.delimiter };
            let include_headers = opts.include_headers;
            let file_name = opts.file_name.clone().unwrap_or_else(|| "export.csv".to_string());

            let (cols, col_state) = (state_for_export.columns.read_untracked(), state_for_export.column_state.read_untracked());
            let visible_cols = cols
                .iter()
                .filter(|c| !col_state.get(c.id).and_then(|cs| cs.hidden).unwrap_or(false))
                .cloned()
                .collect::<Vec<_>>();

            let mut out = String::new();
            if include_headers {
                for (i, c) in visible_cols.iter().enumerate() {
                    if i > 0 { out.push(delimiter); }
                    let mut h = c.header_name.to_string();
                    // simple CSV escaping for headers
                    if h.contains(delimiter) || h.contains('\n') || h.contains('"') {
                        h = h.replace('"', "\"\"");
                        out.push('"'); out.push_str(&h); out.push('"');
                    } else {
                        out.push_str(&h);
                    }
                }
                out.push('\n');
            }
            let rows = state_for_export.rows.read_untracked();
            for rn in rows.iter() {
                for (i, c) in visible_cols.iter().enumerate() {
                    if i > 0 { out.push(delimiter); }
                    let val = if let Some(getter) = &c.value_getter { getter(&rn.data) } else { crate::pages::components::datatable::core::render_value::Value::Empty };
                    let s = if let Some(fmt) = &c.value_formatter { fmt(&val) } else { val.to_string() };
                    if s.contains(delimiter) || s.contains('\n') || s.contains('"') {
                        let escaped = s.replace('"', "\"\"");
                        out.push('"'); out.push_str(&escaped); out.push('"');
                    } else {
                        out.push_str(&s);
                    }
                }
                out.push('\n');
            }
            #[cfg(target_arch = "wasm32")]
            {
                // WASM: skip direct download to avoid web_sys/js_sys deps; log length instead.
                // A proper download can be reintroduced later via a helper that depends on web-sys.
                let _ = (&file_name, out.len());
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                // Non-wasm (SSR/dev): just log the CSV length
                println!("CSV export generated: {} bytes", out.len());
            }
        });
        let state_for_selected = state.clone();
        let get_selected_rows = Rc::new(move || {
            let rows = state_for_selected.rows.read_untracked();
            let sel = state_for_selected.selection.read_untracked();
            rows.iter()
                .filter(|rn| sel.selected_row_ids.iter().any(|id| id == &rn.id))
                .map(|rn| (*rn.data).clone())
                .collect::<Vec<_>>()
        });
        Self {
            refresh_cells,
            set_quick_filter,
            export_csv,
            set_column_state,
            get_selected_rows,
        }
    }
}
