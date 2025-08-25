use std::collections::HashMap;
use std::sync::Arc;
use super::base::ICellRenderer;


pub struct RendererRegistry<T> {
    map: HashMap<String, Arc<dyn ICellRenderer<T> + Send + Sync>>,
}


impl<T> RendererRegistry<T> {
    pub fn new() -> Self { Self { map: HashMap::new() } }
    pub fn register(&mut self, col_id: impl Into<String>, r: Arc<dyn ICellRenderer<T> + Send + Sync>) {
        self.map.insert(col_id.into(), r);
    }
    pub fn get(&self, col_id: &str) -> Option<Arc<dyn ICellRenderer<T> + Send + Sync>> {
        self.map.get(col_id).cloned()
    }
}