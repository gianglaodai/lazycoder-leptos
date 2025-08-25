use std::sync::Arc;

use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::{ReadUntracked, Update, With, Set};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectionMode {
    #[default]
    Single,
    Multi,
}

pub struct SelectionService<T: Send + Sync + 'static> {
    state: Arc<TableState<T>>,
    pub mode: SelectionMode,
}

impl<T: Send + Sync + 'static> SelectionService<T> {
    pub fn new(state: Arc<TableState<T>>) -> Self {
        Self {
            state,
            mode: SelectionMode::Single,
        }
    }

    pub fn set_mode(&mut self, mode: SelectionMode) {
        self.mode = mode;
        if matches!(self.mode, SelectionMode::Single) {
            // If switching to Single, ensure at most one selection remains.
            self.state.selection.update(|sel| {
                if sel.selected_row_ids.len() > 1 {
                    if let Some(first) = sel.selected_row_ids.first().cloned() {
                        sel.selected_row_ids.clear();
                        sel.selected_row_ids.push(first.clone());
                        sel.last_clicked_row_id = Some(first);
                        sel.range_anchor = None;
                    }
                }
            });
        }
    }

    pub fn clear(&self) {
        self.state.selection.update(|sel| sel.clear());
    }

    pub fn select(&self, row_id: &str) {
        match self.mode {
            SelectionMode::Single => {
                let id = row_id.to_string();
                {
                    let mut s = crate::pages::components::datatable::core::row::SelectionState::default();
                    s.selected_row_ids.push(id.clone());
                    s.last_clicked_row_id = Some(id);
                    self.state.selection.set(s);
                }
            }
            SelectionMode::Multi => {
                let id = row_id.to_string();
                self.state.selection.update(|sel| {
                    if !sel.selected_row_ids.iter().any(|x| x == &id) {
                        sel.selected_row_ids.push(id.clone());
                    }
                    sel.last_clicked_row_id = Some(id);
                });
            }
        }
    }

    pub fn deselect(&self, row_id: &str) {
        let id = row_id.to_string();
        self.state.selection.update(|sel| {
            sel.selected_row_ids.retain(|x| x != &id);
            if sel.last_clicked_row_id.as_deref() == Some(row_id) {
                sel.last_clicked_row_id = None;
            }
            // keep range_anchor unchanged
        });
    }

    pub fn toggle(&self, row_id: &str) {
        match self.mode {
            SelectionMode::Single => {
                // In single mode, toggle means: if already selected, clear; else select only this one
                let already = self
                    .state
                    .selection
                    .with(|s| s.selected_row_ids.iter().any(|x| x == row_id));
                if already {
                    self.clear();
                } else {
                    self.select(row_id);
                }
            }
            SelectionMode::Multi => {
                let id = row_id.to_string();
                self.state.selection.update(|sel| {
                    if let Some(pos) = sel.selected_row_ids.iter().position(|x| x == &id) {
                        sel.selected_row_ids.remove(pos);
                    } else {
                        sel.selected_row_ids.push(id.clone());
                    }
                    sel.last_clicked_row_id = Some(id);
                });
            }
        }
    }

    pub fn selected_ids(&self) -> Vec<String> {
        self.state.selection.read_untracked().selected_row_ids.clone()
    }
}
