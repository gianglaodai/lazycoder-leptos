use super::{ColumnFilterModel, IFilter};
use leptos::prelude::*;

pub struct SetFilter;

impl SetFilter {
    pub fn new() -> Self {
        Self
    }
}

impl IFilter for SetFilter {
    fn view(&self) -> impl IntoView {
        // Minimal, non-interactive set filter UI: placeholder to avoid panics
        // In a full implementation, this would list unique values with checkboxes.
        view! {
            <div class="lc-filter-set flex items-center gap-2">
                <input type="text" class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 w-40" placeholder="Comma-separated values" />
            </div>
        }
    }
    fn as_model(&self) -> ColumnFilterModel {
        // Safe default: using operator "in" and empty value (to be populated later)
        ColumnFilterModel {
            col_id: String::new(),
            operator: "in".into(),
            value: String::new(),
        }
    }
}
