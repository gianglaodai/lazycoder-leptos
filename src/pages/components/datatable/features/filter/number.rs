use super::{ColumnFilterModel, IFilter};
use leptos::prelude::*;

pub struct NumberFilter;

impl NumberFilter {
    pub fn new() -> Self {
        Self
    }
}

impl IFilter for NumberFilter {
    fn view(&self) -> impl IntoView {
        // Minimal, non-interactive number filter UI to avoid panics until fully wired
        view! {
            <div class="lc-filter-number flex items-center gap-2">
                <select class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 bg-white">
                    <option value="equals">"="</option>
                    <option value="notEqual">"≠"</option>
                    <option value="lt">"<"</option>
                    <option value="lte">"≤"</option>
                    <option value="gt">">"</option>
                    <option value="gte">"≥"</option>
                </select>
                <input type="number" class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 w-24" />
            </div>
        }
    }
    fn as_model(&self) -> ColumnFilterModel {
        // Safe default model; real wiring will fill values from inputs later
        ColumnFilterModel {
            col_id: String::new(),
            operator: "equals".into(),
            value: String::new(),
        }
    }
}
