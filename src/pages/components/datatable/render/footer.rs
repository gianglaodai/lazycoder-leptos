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

    let total_text =
        move || total.with(|t| t.map(|n| n.to_string()).unwrap_or_else(|| "unknown".into()));

    let can_prev = move || current_page.get() > 1;
    let can_next = move || current_page.get() < page_count();

    let goto_prev = {
        let st = state.clone();
        move |_| {
            if st.current_page.get_untracked() > 1 {
                st.current_page.update(|p| *p -= 1);
                if !st.client_side_sorting.get_untracked()
                    || !st.client_side_filtering.get_untracked()
                {
                    st.notify_query_changed();
                }
            }
        }
    };
    let goto_first = {
        let st = state.clone();
        move |_| {
            go_to_page(&st, 1);
        }
    };
    let goto_next = {
        let st = state.clone();
        move |_| {
            let pc = page_count();
            if st.current_page.get_untracked() < pc {
                st.current_page.update(|p| *p += 1);
                if !st.client_side_sorting.get_untracked()
                    || !st.client_side_filtering.get_untracked()
                {
                    st.notify_query_changed();
                }
            }
        }
    };
    let goto_last = {
        let st = state.clone();
        move |_| {
            // Use a large number; go_to_page will clamp to the last page based on total_rows & page_size
            go_to_page(&st, usize::MAX);
            if !st.client_side_sorting.get_untracked() || !st.client_side_filtering.get_untracked()
            {
                st.notify_query_changed();
            }
        }
    };
    view! {
        <div class="lc-dt-status flex items-center justify-between px-3 py-2 text-xs text-gray-600">
            <div class="flex items-center gap-3">
                <span>{move || {
                    let ps = page_size.get().max(1);
                    let cp = current_page.get().max(1);
                    let total_rows = total.get().unwrap_or_else(|| rows.with(|v| v.len()));
                    if total_rows == 0 {
                        "0 to 0 of 0".to_string()
                    } else {
                        let start = (cp - 1) * ps + 1;
                        let end = std::cmp::min(start + ps - 1, total_rows);
                        format!("{} to {} of {}", start, end, total_rows)
                    }
                }}</span>
                <span>{move || format!("Selected: {}", selected_count())}</span>
                <div class="flex items-center gap-1">
                    <span class="text-gray-500">{"Page size:"}</span>
                    {
                        let st = state.clone();
                        view!{
                            <select class="w-20 border border-gray-200 rounded px-1 py-0.5 text-gray-700"
                                prop:value=move || page_size.get().to_string()
                                on:change=move |ev| {
                                    let val = leptos::prelude::event_target_value(&ev);
                                    if let Ok(num) = val.parse::<usize>() {
                                        set_page_size(&st, num);
                                    }
                                }
                            >
                                <option value="5">{"5"}</option>
                                <option value="10">{"10"}</option>
                                <option value="20">{"20"}</option>
                                <option value="50">{"50"}</option>
                                <option value="100">{"100"}</option>
                            </select>
                        }
                    }
                </div>
            </div>
            <div class="lc-dt-pagination inline-flex items-center gap-2">
                <button class="px-2 py-1 border border-gray-200 rounded text-gray-700 disabled:text-gray-400 transform rotate-270" on:click=goto_first disabled=move || !can_prev() >{"⌅"}</button>
                <button class="px-2 py-1 border border-gray-200 rounded text-gray-700 disabled:text-gray-400 transform rotate-270" on:click=goto_prev disabled=move || !can_prev() >{"⌃"}</button>
                <span class="text-gray-500">{move || format!("Page {} of {}", current_page.get(), page_count())}</span>
                <button class="px-2 py-1 border border-gray-200 rounded text-gray-700 disabled:text-gray-400 transform rotate-90" on:click=goto_next disabled=move || !can_next() >{"⌃"}</button>
                <button class="px-2 py-1 border border-gray-200 rounded text-gray-700 disabled:text-gray-400 transform rotate-90" on:click=goto_last disabled=move || !can_next() >{"⌅"}</button>
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

pub fn go_to_page<T: Send + Sync + 'static>(state: &Arc<TableState<T>>, page: usize) {
    let page_size = state.page_size.get_untracked().max(1);
    // Determine total rows from server-provided total when available; otherwise from local rows length.
    let total_rows = state
        .total_rows
        .get_untracked()
        .unwrap_or_else(|| state.rows.read_untracked().len());
    let max_pages = ((total_rows + page_size - 1) / page_size).max(1);
    let target = page.clamp(1, max_pages);
    state.current_page.set(target);
    if !state.client_side_sorting.get_untracked() || !state.client_side_filtering.get_untracked() {
        state.notify_query_changed();
    }
}

pub fn set_page_size<T: Send + Sync + 'static>(state: &Arc<TableState<T>>, size: usize) {
    let new_size = size.max(1);
    // Update page size first
    state.page_size.set(new_size);
    // Re-clamp current page against new page count
    let total_rows = state
        .total_rows
        .get_untracked()
        .unwrap_or_else(|| state.rows.read_untracked().len());
    let max_pages = ((total_rows + new_size - 1) / new_size).max(1);
    state.current_page.update(|cp| {
        if *cp > max_pages {
            *cp = max_pages;
        } else if *cp == 0 {
            *cp = 1;
        }
    });
    if !state.client_side_sorting.get_untracked() || !state.client_side_filtering.get_untracked() {
        state.notify_query_changed();
    }
}
