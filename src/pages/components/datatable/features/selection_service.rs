#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectionMode { #[default] Single, Multi }


pub struct SelectionService {
    pub mode: SelectionMode,
}


impl SelectionService {
    pub fn new() -> Self { Self { mode: SelectionMode::Single } }
    pub fn set_mode(&mut self, _mode: SelectionMode) {
        unimplemented!()
    }
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
    pub fn selected_ids(&self) -> Vec<String> { Vec::new() }
}