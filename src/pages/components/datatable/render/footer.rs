// render/footer.rs
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::*;
use std::sync::Arc;

#[component]
pub fn StatusBar<T: Send + Sync + 'static>(
    #[prop(into)] state: Arc<TableState<T>>,
) -> impl IntoView {
    let rows = state.rows;
    let total = state.total_rows;
    let selected = state.selection;
    let page_size = state.page_size;
    let current_page = state.current_page;

    let selected_count = move || selected.with(|s| s.selected_row_ids.len());
    let rows_count = move || rows.with(|v| v.len());

    // Determine page count based on total_rows when provided, else based on local rows length.
    let page_count = move || {
        let ps = page_size.get().max(1);
        let total_opt = total.get();
        let total_rows = total_opt.unwrap_or_else(|| rows.with(|v| v.len()));
        let pages = (total_rows + ps - 1) / ps;
        pages.max(1)
    };

    let total_text = move || total.with(|t| t.map(|n| n.to_string()).unwrap_or_else(|| "unknown".into()));

    let can_prev = move || current_page.get() > 1;
    let can_next = move || current_page.get() < page_count();

    let goto_prev = {
        let current_page = current_page.clone();
        move |_| {
            if current_page.get_untracked() > 1 {
                current_page.update(|p| *p -= 1);
            }
        }
    };
    let goto_next = {
        let current_page = current_page.clone();
        move |_| {
            let pc = page_count();
            if current_page.get_untracked() < pc {
                current_page.update(|p| *p += 1);
            }
        }
    };

    view! {
        <div class="lc-dt-status flex items-center justify-between px-3 py-2 text-xs text-gray-600">
            <div class="flex items-center gap-3">
                <span>{move || format!("Rows: {}", rows_count())}</span>
                <span>{move || format!("Selected: {}", selected_count())}</span>
                <span>{move || format!("Total: {}", total_text())}</span>
            </div>
            <div class="lc-dt-pagination inline-flex items-center gap-2">
                <button class="px-2 py-1 border border-gray-200 rounded text-gray-700 disabled:text-gray-400" on:click=goto_prev disabled=move || !can_prev() >{"Prev"}</button>
                <span class="text-gray-500">{move || format!("Page {}/{}", current_page.get(), page_count())}</span>
                <button class="px-2 py-1 border border-gray-200 rounded text-gray-700 disabled:text-gray-400" on:click=goto_next disabled=move || !can_next() >{"Next"}</button>
            </div>
        </div>
    }
}

#[component]
pub fn Pagination<T: Send + Sync + 'static>(
    #[prop(into)] _state: Arc<TableState<T>>,
) -> impl IntoView {
    // Placeholder pagination controls (no real state yet)
    view! {
        <div class="lc-dt-pagination inline-flex items-center gap-2">
            <button class="px-2 py-1 border border-gray-200 rounded text-gray-500" disabled=true>{"Prev"}</button>
            <span class="text-gray-500">{"Page 1"}</span>
            <button class="px-2 py-1 border border-gray-200 rounded text-gray-500" disabled=true>{"Next"}</button>
        </div>
    }
}

pub fn go_to_page(_page: usize) {
    // no-op for now
}
pub fn set_page_size(_size: usize) {
    // no-op for now
}
