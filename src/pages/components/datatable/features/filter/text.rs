use super::{ColumnFilterModel, IFilter};
use leptos::prelude::*;

pub struct TextFilter;

impl TextFilter {
    pub fn new() -> Self {
        Self
    }
}

impl IFilter for TextFilter {
    fn view(&self) -> impl IntoView {
        // Minimal, non-interactive text filter UI to avoid panics until fully wired
        view! {
            <div class="lc-filter-text flex items-center gap-2">
                <select class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 bg-white">
                    <option value="contains">"contains"</option>
                    <option value="notContains">"not contains"</option>
                    <option value="equals">"equals"</option>
                    <option value="startsWith">"starts with"</option>
                    <option value="endsWith">"ends with"</option>
                </select>
                <input type="text" class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 w-32" placeholder="Filter..." />
            </div>
        }
    }
    fn as_model(&self) -> ColumnFilterModel {
        // Safe default model; real wiring will fill values from inputs later
        ColumnFilterModel {
            col_id: String::new(),
            operator: "contains".into(),
            value: String::new(),
        }
    }
}
