use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct RowNode<T> {
    pub id: String,
    pub data: Arc<T>,
    pub depth: usize,
    pub parent: Option<String>,
    pub is_group: bool,
    pub expanded: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SelectionState {
    pub selected_row_ids: Vec<String>,
    pub last_clicked_row_id: Option<String>,
    pub range_anchor: Option<(String, String)>,
}

impl<T> RowNode<T> {
    pub fn new(_id: impl Into<String>, _data: T) -> Self {
        Self {
            id: _id.into(),
            data: Arc::new(_data),
            depth: 0,
            parent: None,
            is_group: false,
            expanded: false,
        }
    }
}

impl SelectionState {
    pub fn clear(&mut self) {
        self.selected_row_ids.clear();
        self.last_clicked_row_id = None;
        self.range_anchor = None;
    }
    pub fn select(&mut self, row_id: &str) {
        if !self.selected_row_ids.iter().any(|id| id == row_id) {
            self.selected_row_ids.push(row_id.to_string());
        }
        self.last_clicked_row_id = Some(row_id.to_string());
    }
    pub fn deselect(&mut self, row_id: &str) {
        self.selected_row_ids.retain(|id| id != row_id);
    }
    pub fn toggle(&mut self, row_id: &str) {
        if self.is_selected(row_id) {
            self.deselect(row_id);
        } else {
            self.select(row_id);
        }
    }
    pub fn is_selected(&self, row_id: &str) -> bool {
        self.selected_row_ids.iter().any(|id| id == row_id)
    }
}
