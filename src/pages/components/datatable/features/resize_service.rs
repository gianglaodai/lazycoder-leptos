use std::sync::Arc;

use crate::pages::components::datatable::core::column::ColumnState;
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::{Update, With};

pub struct ResizeService<T: Send + Sync + 'static> {
    state: Arc<TableState<T>>,      // table state handle
    resizing: bool,                 // whether a resize is in progress
    col_id: Option<String>,         // id of the column being resized
    start_x: i32,                   // pointer x at begin
    current_x: i32,                 // latest pointer x
    start_width: i32,               // effective width at begin (taking overrides into account)
}

impl<T: Send + Sync + 'static> ResizeService<T> {
    pub fn new(state: Arc<TableState<T>>) -> Self {
        Self {
            state,
            resizing: false,
            col_id: None,
            start_x: 0,
            current_x: 0,
            start_width: 0,
        }
    }

    /// Begin resizing a column: record the starting pointer position and the starting width.
    pub fn begin_resize(&mut self, col_id: &str, start_x: i32) {
        self.resizing = true;
        self.col_id = Some(col_id.to_string());
        self.start_x = start_x;
        self.current_x = start_x;
        // Determine effective start width (column_state override or column def width)
        let id = col_id.to_string();
        let effective = {
            let def_width = self
                .state
                .columns
                .with(|cols| cols.iter().find(|c| c.id == id.as_str()).map(|c| c.width).unwrap_or(0));
            self.state
                .column_state
                .with(|m| m.get(&id).and_then(|cs| cs.width))
                .unwrap_or(def_width)
        };
        self.start_width = effective.max(0);
    }

    /// Update the current pointer X during resize (safe when not resizing).
    pub fn update_resize(&mut self, current_x: i32) {
        if self.resizing {
            self.current_x = current_x;
        }
    }

    /// Finish the resize operation, applying the new width into column_state (clamped to min/max).
    pub fn end_resize(&mut self) {
        if !self.resizing {
            return;
        }
        let delta = self.current_x - self.start_x;
        let id_opt = self.col_id.clone();
        if let Some(id) = id_opt {
            // Compute clamped width using column def min/max
            let (min_w, max_w_opt) = self.state.columns.with(|cols| {
                if let Some(c) = cols.iter().find(|c| c.id == id.as_str()) {
                    (c.min_width.max(0), c.max_width)
                } else {
                    (0, None)
                }
            });
            let mut new_w = self.start_width.saturating_add(delta);
            if let Some(maxw) = max_w_opt { new_w = new_w.clamp(min_w, maxw); } else { new_w = new_w.max(min_w); }
            // Write into column_state
            self.state.column_state.update(|m| {
                let entry = m
                    .entry(id.clone())
                    .or_insert_with(|| ColumnState { id: id.clone(), ..Default::default() });
                entry.width = Some(new_w);
            });
        }
        // reset internal state
        self.resizing = false;
        self.col_id = None;
        self.start_x = 0;
        self.current_x = 0;
        self.start_width = 0;
    }
}
