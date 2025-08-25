use std::sync::Arc;

use crate::pages::components::datatable::core::column::{ColumnState, Pinned};
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::Update;

pub enum PinSide {
    Left,
    Right,
}

pub struct PinService<T: Send + Sync + 'static> {
    state: Arc<TableState<T>>,
}

impl<T: Send + Sync + 'static> PinService<T> {
    pub fn new(state: Arc<TableState<T>>) -> Self {
        Self { state }
    }
    /// Pin a column to the given side by updating the runtime column_state map.
    pub fn pin(&self, col_id: &str, side: PinSide) {
        let id = col_id.to_string();
        let pin_val = match side {
            PinSide::Left => Pinned::Left,
            PinSide::Right => Pinned::Right,
        };
        self.state.column_state.update(|m| {
            let entry = m
                .entry(id.clone())
                .or_insert_with(|| ColumnState { id: id.clone(), ..Default::default() });
            entry.pinned = Some(pin_val);
        });
    }
    /// Unpin a column (explicitly set Pinned::None so it overrides any column default).
    pub fn unpin(&self, col_id: &str) {
        let id = col_id.to_string();
        self.state.column_state.update(|m| {
            let entry = m
                .entry(id.clone())
                .or_insert_with(|| ColumnState { id: id.clone(), ..Default::default() });
            entry.pinned = Some(Pinned::None);
        });
    }
}
