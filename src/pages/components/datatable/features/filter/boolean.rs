use super::{ColumnFilterModel, IFilter};
use leptos::prelude::*;

pub struct BooleanFilter;

impl BooleanFilter {
    pub fn new() -> Self {
        Self
    }
}

impl IFilter for BooleanFilter {
    fn view(&self) -> impl IntoView {
        // Minimal, non-interactive number filter UI to avoid panics until fully wired
        view! {
            <div class="lc-filter-number flex items-center gap-2">
                <select class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 bg-white">
                    <option value="is">"Is"</option>
                    <option value="=null">"Is null"</option>
                    <option value="!null">"Not null"</option>
                </select>
                <input type="checkbox" class="h-4 w-4 border-gray-300 rounded text-blue-600 focus:ring-blue-500" />
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
