use super::{ColumnFilterModel, IFilter};
use leptos::prelude::*;

pub struct IntegerFilter;

impl IntegerFilter {
    pub fn new() -> Self {
        Self
    }
}

impl IFilter for IntegerFilter {
    fn view(&self) -> impl IntoView {
        // Minimal, non-interactive number filter UI to avoid panics until fully wired
        view! {
            <div class="lc-filter-number flex items-center gap-2">
                <select class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 bg-white">
                    <option value="=">"Equals"</option>
                    <option value="!=">"Not equals"</option>
                    <option value="<">"Less than"</option>
                    <option value="<=">"Less than or equal to"</option>
                    <option value=">">"Greater than"</option>
                    <option value=">=">"Greater than or equal to"</option>
                    <option value="=null">"Is null"</option>
                    <option value="!null">"Not null"</option>
                </select>
                <input type="number" pattern="[0-9]*" class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 w-24" />
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
