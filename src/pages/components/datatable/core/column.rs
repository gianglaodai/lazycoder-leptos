use std::sync::Arc;

use crate::pages::components::datatable::core::agg::AggregateFn;
use crate::pages::components::datatable::core::data_source::{SortModel, SortOrder};
use crate::pages::components::datatable::core::render_value::Value;
use crate::pages::components::datatable::core::state::TableState;
use crate::pages::components::datatable::renderers::base::ICellRenderer;
use leptos::prelude::{Set, Update, With};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pinned {
    None,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataType {
    Text,
    Int,
    Float,
    Boolean,
    Date,
    Time,
    DateTime,
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
    pub data_type: Option<DataType>,
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

pub struct ColumnApi<T: Send + Sync + 'static> {
    state: std::sync::Arc<TableState<T>>,
}

impl<T: Send + Sync + 'static> ColumnApi<T> {
    pub fn new(state: std::sync::Arc<TableState<T>>) -> Self {
        Self { state }
    }
    pub fn set_visible(&self, col_id: &str, visible: bool) {
        let id = col_id.to_string();
        self.state.column_state.update(|m| {
            let entry = m.entry(id.clone()).or_insert_with(|| ColumnState {
                id: id.clone(),
                ..Default::default()
            });
            entry.hidden = Some(!visible);
        });
    }
    pub fn set_width(&self, col_id: &str, width: i32) {
        let id = col_id.to_string();
        // clamp against column def min/max
        let (min, max_opt) = self.state.columns.with(|cols| {
            if let Some(c) = cols.iter().find(|c| c.id == id.as_str()) {
                (c.min_width.max(0), c.max_width)
            } else {
                (0, None)
            }
        });
        let clamped = if let Some(max) = max_opt {
            width.clamp(min, max)
        } else {
            width.max(min)
        };
        self.state.column_state.update(|m| {
            let entry = m.entry(id.clone()).or_insert_with(|| ColumnState {
                id: id.clone(),
                ..Default::default()
            });
            entry.width = Some(clamped);
        });
    }
    pub fn move_column(&self, from: usize, to: usize) {
        self.state.columns.update(|cols| {
            if from < cols.len() && to < cols.len() && from != to {
                let col = cols.remove(from);
                cols.insert(to, col);
            }
        });
    }
    pub fn set_column_state(&self, state_list: Vec<ColumnState>) {
        // Merge column states
        let mut new_map: HashMap<String, ColumnState> = self.state.column_state.with(|m| m.clone());
        for s in state_list.iter() {
            new_map
                .entry(s.id.clone())
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
        self.state.column_state.set(new_map);
        // Also update sort_model based on provided states
        let mut sort_model: Vec<SortModel> = state_list
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
        // Ensure stable order by sort_index when provided
        sort_model.sort_by(|a, b| match (a.sort_index, b.sort_index) {
            (Some(x), Some(y)) => x.cmp(&y),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.col_id.cmp(&b.col_id),
        });
        if !sort_model.is_empty() {
            self.state.sort_model.set(sort_model);
        }
    }
}
