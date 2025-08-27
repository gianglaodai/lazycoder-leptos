use super::{ColumnFilterModel, IFilter};
use leptos::prelude::*;

pub struct DateTimeFilter;

impl DateTimeFilter {
    pub fn new() -> Self {
        Self
    }
}

impl IFilter for DateTimeFilter {
    fn view(&self) -> impl IntoView {
        // Minimal, non-interactive UI skeleton to avoid panics until fully wired
        view! {
            <div class="lc-filter-date flex items-center gap-2">
                <select class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 bg-white">
                    <option value="=">"Equals"</option>
                    <option value="<">"Before"</option>
                    <option value=">">"After"</option>
                    <option value="=null">"Is null"</option>
                    <option value="!null">"Not null"</option>
                </select>
                <input type="datetime-local" class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700" />
            </div>
        }
    }
    fn as_model(&self) -> ColumnFilterModel {
        // Return a safe default model; real wiring will populate these from user input later
        ColumnFilterModel {
            col_id: String::new(),
            operator: "equals".into(),
            value: String::new(),
        }
    }
}
