use super::ICellEditor;

pub struct TextEditor;
impl TextEditor {
    pub fn new() -> Self {
        Self
    }
}
impl<T> ICellEditor<T> for TextEditor {}

pub struct NumberEditor;
impl NumberEditor {
    pub fn new() -> Self {
        Self
    }
}
impl<T> ICellEditor<T> for NumberEditor {}

pub struct SelectEditor;
impl SelectEditor {
    pub fn new() -> Self {
        Self
    }
}
impl<T> ICellEditor<T> for SelectEditor {}
