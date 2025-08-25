use crate::pages::components::datatable::core::column::ColumnDef;
use crate::pages::components::datatable::core::row::RowNode;
use crate::pages::components::datatable::core::state::TableState;
use crate::pages::components::datatable::core::render_value::Value as LCValue;
use leptos::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

fn compare_value(a: &LCValue, b: &LCValue) -> Ordering {
    use LCValue::*;
    match (a, b) {
        (Empty, Empty) => Ordering::Equal,
        (Empty, _) => Ordering::Less,
        (_, Empty) => Ordering::Greater,
        (Number(x), Number(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
        (Bool(x), Bool(y)) => x.cmp(y),
        (Date(x), Date(y)) => x.cmp(y), // assume ISO format for lexicographic order
        (Text(x), Text(y)) => x.cmp(y),
        _ => a.to_string().cmp(&b.to_string()),
    }
}

#[component]
pub fn VirtualizedBody<T: Clone + Send + Sync + 'static>(
    #[prop(into)] state: Arc<TableState<T>>,
    row_height: i32,
) -> impl IntoView {
    // Compute visible columns merging runtime column_state (hidden/width)
    let cols_sig = state.columns;
    let col_state_sig = state.column_state;
    let visible_cols = move || {
        cols_sig.with(|cols| {
            col_state_sig.with(|m| {
                cols
                    .iter()
                    .filter(|c| !m.get(c.id).and_then(|cs| cs.hidden).unwrap_or(false))
                    .map(|c| {
                        let mut cc = c.clone();
                        if let Some(w) = m.get(c.id).and_then(|cs| cs.width) {
                            cc.width = w;
                        }
                        cc
                    })
                    .collect::<Vec<_>>()
            })
        })
    };
    let template_style = move || {
        let widths = visible_cols()
            .into_iter()
            .map(|c| format!("{}px", c.width))
            .collect::<Vec<_>>()
            .join(" ");
        format!("grid-template-columns:{};", widths)
    };
    let rows_sig = state.rows;
    let quick = state.quick_filter;
    let page_size = state.page_size;
    let current_page = state.current_page;
    let total_rows_sig = state.total_rows;
    let sort_model_sig = state.sort_model;
    let columns_sig = state.columns;

    // Helper: filter rows by quick text across visible columns.
    let state_for_filter = state.clone();
    let filtered_rows = move || {
        let q = quick.get_untracked().to_lowercase();
        let rows = rows_sig.with(|v| v.clone());
        let col_filters = state_for_filter.filter_model.with(|fm| fm.column_text.clone());
        let cols = visible_cols();
        let apply_quick = |text: &str| q.is_empty() || text.to_lowercase().contains(&q);
        if q.is_empty() && col_filters.is_empty() {
            rows
        } else {
            rows.into_iter()
                .filter(|rn| {
                    // Quick filter across any visible column
                    let quick_ok = if q.is_empty() { true } else {
                        cols.iter().any(|c| {
                            let val = if let Some(getter) = &c.value_getter { getter(&rn.data) } else { LCValue::Empty };
                            let txt = if let Some(fmt) = &c.value_formatter { fmt(&val) } else { val.to_string() };
                            apply_quick(&txt)
                        })
                    };
                    if !quick_ok { return false; }
                    // Per-column filters (contains)
                    for (cid, needle) in col_filters.iter() {
                        if let Some(c) = cols.iter().find(|c| &c.id == cid) {
                            let val = if let Some(getter) = &c.value_getter { getter(&rn.data) } else { LCValue::Empty };
                            let txt = if let Some(fmt) = &c.value_formatter { fmt(&val) } else { val.to_string() };
                            if !txt.to_lowercase().contains(&needle.to_lowercase()) {
                                return false;
                            }
                        }
                    }
                    true
                })
                .collect::<Vec<_>>()
        }
    };

    // Helper: sort rows based on sort_model.
    let sorted_rows = move || {
        let mut rows = filtered_rows();
        let model = sort_model_sig.with(|m| m.clone());
        if model.is_empty() || rows.len() <= 1 {
            return rows;
        }
        // Map columns by id for quick lookup of getters and comparators
        let cols = columns_sig.with(|c| c.clone());
        let col_map: HashMap<String, ColumnDef<T>> = cols.into_iter().map(|c| (c.id.to_string(), c)).collect();
        // Order model by sort_index (if any)
        let mut ordered = model.clone();
        ordered.sort_by(|a, b| match (a.sort_index, b.sort_index) {
            (Some(x), Some(y)) => x.cmp(&y),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.col_id.cmp(&b.col_id),
        });
        rows.sort_by(|ra, rb| {
            for sm in ordered.iter() {
                if let Some(col) = col_map.get(&sm.col_id) {
                    let va = if let Some(getter) = &col.value_getter { getter(&ra.data) } else { LCValue::Empty };
                    let vb = if let Some(getter) = &col.value_getter { getter(&rb.data) } else { LCValue::Empty };
                    let ord = if let Some(cmp) = &col.comparator {
                        cmp(&va, &vb)
                    } else {
                        compare_value(&va, &vb)
                    };
                    let ord = match sm.sort {
                        crate::pages::components::datatable::core::data_source::SortOrder::Asc => ord,
                        crate::pages::components::datatable::core::data_source::SortOrder::Desc => ord.reverse(),
                    };
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
            }
            Ordering::Equal
        });
        rows
    };

    // Helper: page the sorted rows.
    let paged_rows = move || {
        let ps = page_size.get_untracked().max(1);
        let cp = current_page.get_untracked().max(1);
        let fr = sorted_rows();
        // If total_rows is provided (server-side pagination) and the current rows are already a single page,
        // avoid client-side slicing to prevent double pagination.
        let total_opt = total_rows_sig.get_untracked();
        if total_opt.is_some() && fr.len() <= ps {
            return fr;
        }
        let start = ps.saturating_mul(cp.saturating_sub(1));
        let end = start + ps;
        fr.into_iter().enumerate().filter_map(|(idx, r)| if idx >= start && idx < end { Some(r) } else { None }).collect::<Vec<_>>()
    };

    view! {
        <div class="lc-dt-rows">
            <For
                each=move || paged_rows()
                key=|r| r.id.clone()
                children=move |row| { view!{
                    <div class="lc-dt-row grid items-center border-b border-gray-100" style=move || {
                        let widths = visible_cols().into_iter().map(|c| format!("{}px", c.width)).collect::<Vec<_>>().join(" ");
                        format!("grid-template-columns:{};height:{}px;", widths, row_height)
                    } on:click={
                        let state = state.clone();
                        let row_id = row.id.clone();
                        move |_| {
                            // simple single-selection toggle
                            state.selection.update(|sel| {
                                if sel.selected_row_ids.iter().any(|id| id == &row_id) {
                                    sel.selected_row_ids.retain(|id| id != &row_id);
                                } else {
                                    sel.selected_row_ids.clear();
                                    sel.selected_row_ids.push(row_id.clone());
                                    sel.last_clicked_row_id = Some(row_id.clone());
                                }
                            });
                        }
                    } class=("bg-blue-50", { let s2 = state.clone(); move || s2.selection.with(|s| s.selected_row_ids.iter().any(|id| id == &row.id)) }) >
                        <For
                            each=visible_cols
                            key=|c| c.id
                            children=move |col| { view!{
                                {
                                    use leptos::prelude::AnyView;
                                    use crate::pages::components::datatable::core::render_value::Value as LCValue;
                                    let value = if let Some(getter) = &col.value_getter {
                                        getter(&row.data)
                                    } else { LCValue::Empty };
                                    let text = if let Some(fmt) = &col.value_formatter { fmt(&value) } else { value.to_string() };
                                    let inner: AnyView = view!{ <span class="truncate">{text}</span> }.into_any();
                                    view!{ <div class="lc-dt-cell px-3 py-2 border-r border-gray-100 overflow-hidden">{inner}</div> }
                                }
                            }}
                        />
                    </div>
                }}
            />
        </div>
    }
}

#[component]
pub fn RowRenderer<T: Clone + Send + Sync + 'static>(
    _row: RowNode<T>,
    _cols: Vec<ColumnDef<T>>,
    row_height: i32,
) -> impl IntoView {
    // Simplified placeholder: actual row rendering is done inline in VirtualizedBody for now
    view! {
        <div class="lc-dt-row-inner" style=move || format!("height:{}px;", row_height)></div>
    }
}

#[component]
pub fn CellHost<T: Clone + Send + Sync + 'static>(
    row: RowNode<T>,
    col: ColumnDef<T>,
) -> impl IntoView {
    use leptos::prelude::AnyView;
    use crate::pages::components::datatable::core::render_value::Value as LCValue;
    let value = if let Some(getter) = &col.value_getter { getter(&row.data) } else { LCValue::Empty };
    let text = if let Some(fmt) = &col.value_formatter { fmt(&value) } else { value.to_string() };
    let inner: AnyView = view! { <span class="truncate">{text}</span> }.into_any();
    view! {
        <div class="lc-dt-cell px-3 py-2 border-r border-gray-100 overflow-hidden">{inner}</div>
    }
}

pub fn handle_row_click<T>(_row: &RowNode<T>) {
    // placeholder: selection handling will be wired with state later
}
pub fn handle_cell_click<T>(_row: &RowNode<T>, _col: &ColumnDef<T>) {
    // placeholder: cell click handling (edit/navigation) goes here later
}
