use std::collections::HashMap;
use std::rc::Rc;
use super::base::ICellRenderer;


/// Registry gắn col_id -> renderer
pub struct RendererRegistry<T> {
    map: HashMap<String, Rc<dyn ICellRenderer<T>>>,
}


impl<T> RendererRegistry<T> {
    pub fn new() -> Self { Self { map: HashMap::new() } }
    pub fn register(&mut self, _col_id: impl Into<String>, _r: Rc<dyn ICellRenderer<T>>) {
        unimplemented!()
    }
    pub fn get(&self, _col_id: &str) -> Option<Rc<dyn ICellRenderer<T>>> { None }
}