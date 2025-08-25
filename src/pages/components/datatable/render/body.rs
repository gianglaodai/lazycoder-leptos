use crate::pages::components::datatable::core::column::ColumnDef;
use crate::pages::components::datatable::core::row::RowNode;
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::*;
use std::sync::Arc;

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

    // Helper: filter rows by quick text across visible columns.
    let filtered_rows = move || {
        let q = quick.get_untracked().to_lowercase();
        let rows = rows_sig.with(|v| v.clone());
        if q.is_empty() {
            rows
        } else {
            // build a vector of visible columns once
            let cols = visible_cols();
            rows.into_iter()
                .filter(|rn| {
                    cols.iter().any(|c| {
                        let val = if let Some(getter) = &c.value_getter { getter(&rn.data) } else { crate::pages::components::datatable::core::render_value::Value::Empty };
                        let txt = if let Some(fmt) = &c.value_formatter { fmt(&val) } else { val.to_string() };
                        txt.to_lowercase().contains(&q)
                    })
                })
                .collect::<Vec<_>>()
        }
    };

    // Helper: page the filtered rows.
    let paged_rows = move || {
        let ps = page_size.get_untracked().max(1);
        let cp = current_page.get_untracked().max(1);
        let start = ps.saturating_mul(cp.saturating_sub(1));
        let end = start + ps;
        let fr = filtered_rows();
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
