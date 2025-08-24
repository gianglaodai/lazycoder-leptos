use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct RowNode<T> {
    pub id: String,
    pub data: Rc<T>,
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
            data: Rc::new(_data),
            depth: 0,
            parent: None,
            is_group: false,
            expanded: false,
        }
    }
}

impl SelectionState {
    pub fn clear(&mut self) {
        unimplemented!()
    }
    pub fn select(&mut self, _row_id: &str) {
        unimplemented!()
    }
    pub fn deselect(&mut self, _row_id: &str) {
        unimplemented!()
    }
    pub fn toggle(&mut self, _row_id: &str) {
        unimplemented!()
    }
    pub fn is_selected(&self, _row_id: &str) -> bool {
        false
    }
}
