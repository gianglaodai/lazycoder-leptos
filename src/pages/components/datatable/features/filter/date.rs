use super::{ColumnFilterModel, IFilter};
use leptos::prelude::*;

pub struct DateFilter;

impl DateFilter {
    pub fn new() -> Self {
        Self
    }
}

impl IFilter for DateFilter {
    fn view(&self) -> impl IntoView {
        // Minimal, non-interactive UI skeleton to avoid panics until fully wired
        view! {
            <div class="lc-filter-date flex items-center gap-2">
                <select class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 bg-white">
                    <option value="equals">"="</option>
                    <option value="before">"<"</option>
                    <option value="after">">"</option>
                </select>
                <input type="date" class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700" />
            </div>
        }
    }
    fn as_model(&self) -> ColumnFilterModel {
        // Return a safe default model; real wiring will populate these from user input later
        ColumnFilterModel { col_id: String::new(), operator: "equals".into(), value: String::new() }
    }
}
