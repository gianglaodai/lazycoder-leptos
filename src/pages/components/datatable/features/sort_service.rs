use crate::pages::components::datatable::core::data_source::{SortModel, SortOrder};
use crate::pages::components::datatable::core::render_value::Value;
use crate::pages::components::datatable::core::row::RowNode;
use std::cmp::Ordering;
use std::collections::HashMap;

pub type Comparator = fn(a: &Value, b: &Value) -> Ordering;

#[derive(Clone, Default)]
pub struct SortService {
    model: Vec<SortModel>,
    comparators: HashMap<String, Comparator>, // by column id
}

impl SortService {
    pub fn new() -> Self {
        Self { model: Vec::new(), comparators: HashMap::new() }
    }

    /// Replace the current sort model. The incoming list may contain sort_index hints.
    pub fn set_model(&mut self, model: Vec<SortModel>) {
        self.model = model;
    }

    /// Return the current sort model.
    pub fn get_model(&self) -> Vec<SortModel> {
        self.model.clone()
    }

    /// Register a comparator for a given column id.
    pub fn register_comparator(&mut self, col_id: &str, cmp: Comparator) {
        self.comparators.insert(col_id.to_string(), cmp);
    }

    /// Sort a slice of RowNode<serde_json::Value> in place according to the current model.
    /// This is a best-effort client-side sort for JSON-like row data; when server-side sorting
    /// is enabled, callers should not use this.
    pub fn sort_slice(&self, rows: &mut [RowNode<serde_json::Value>]) {
        if self.model.is_empty() || rows.len() <= 1 {
            return;
        }
        let ordered = self.ordered_model();
        // Use stable sort for deterministic ordering
        rows.sort_by(|a, b| {
            for sm in ordered.iter() {
                let va = extract_value(&sm.col_id, &a.data);
                let vb = extract_value(&sm.col_id, &b.data);
                let ord = if let Some(cmp) = self.comparators.get(&sm.col_id) {
                    cmp(&va, &vb)
                } else {
                    compare_value(&va, &vb)
                };
                let ord = match sm.sort {
                    SortOrder::Asc => ord,
                    SortOrder::Desc => ord.reverse(),
                };
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            Ordering::Equal
        });
    }

    fn ordered_model(&self) -> Vec<SortModel> {
        let mut v = self.model.clone();
        v.sort_by(|a, b| match (a.sort_index, b.sort_index) {
            (Some(x), Some(y)) => x.cmp(&y),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a.col_id.cmp(&b.col_id),
        });
        v
    }
}

fn extract_value(col_id: &str, data: &serde_json::Value) -> Value {
    // Expect an object with fields whose keys match col_id.
    match data {
        serde_json::Value::Object(map) => {
            if let Some(v) = map.get(col_id) {
                json_to_value(v)
            } else {
                Value::Empty
            }
        }
        _ => Value::Empty,
    }
}

fn json_to_value(v: &serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Empty,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() { Value::Number(f) } else { Value::Empty }
        }
        serde_json::Value::String(s) => Value::Text(s.clone()),
        other => Value::Text(other.to_string()),
    }
}

fn compare_value(a: &Value, b: &Value) -> Ordering {
    use Value::*;
    match (a, b) {
        (Empty, Empty) => Ordering::Equal,
        (Empty, _) => Ordering::Less,
        (_, Empty) => Ordering::Greater,
        (Number(x), Number(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
        (Bool(x), Bool(y)) => x.cmp(y),
        (Date(x), Date(y)) => x.cmp(y), // lexicographic assume ISO
        (Text(x), Text(y)) => x.cmp(y),
        // Fallback to string comparison when types differ
        _ => a.to_string().cmp(&b.to_string()),
    }
}
