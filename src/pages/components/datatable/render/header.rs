use crate::pages::components::datatable::core::column::ColumnDef;
use crate::pages::components::datatable::core::data_source::{SortModel, SortOrder};
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::*;
use std::sync::Arc;

#[component]
pub fn HeaderRow<T: Clone + Send + Sync + 'static>(
    #[prop(into)] state: Arc<TableState<T>>,
) -> impl IntoView {
    let cols_sig = state.columns;
    let col_state_sig = state.column_state;
    // Compute template columns reactively from visible columns with width overrides
    let template_style = move || {
        let widths = cols_sig.with(|cols| {
            col_state_sig.with(|m| {
                cols.iter()
                    .filter(|c| !m.get(c.id).and_then(|cs| cs.hidden).unwrap_or(false))
                    .map(|c| m.get(c.id).and_then(|cs| cs.width).unwrap_or(c.width))
                    .map(|w| format!("{}px", w))
                    .collect::<Vec<_>>()
            })
        });
        format!("grid-template-columns:{};", widths.join(" "))
    };
    let visible_cols = move || {
        cols_sig.with(|cols| {
            col_state_sig.with(|m| {
                cols.iter()
                    .filter(|c| !m.get(c.id).and_then(|cs| cs.hidden).unwrap_or(false))
                    .cloned()
                    .collect::<Vec<_>>()
            })
        })
    };
    view! {
        <div class="lc-dt-header-row grid" style=template_style>
            <For
                each=visible_cols
                key=|c| c.id
                children=move |c| { view!{
                    // Determine current sort order for this column
                    { let state_for_sort = state.clone();
                      let col_id = c.id.to_string();
                      // compute sort order inline in closures below to avoid moves
                      let on_click = {
                        let state = state.clone();
                        let col_id = c.id.to_string();
                        move |_| {
                          // Cycle: None -> Asc -> Desc -> None, single-column sort
                          state.sort_model.update(|sm| {
                            if let Some(pos) = sm.iter().position(|s| s.col_id == col_id) {
                              match sm[pos].sort {
                                SortOrder::Asc => { sm[pos].sort = SortOrder::Desc; sm[pos].sort_index = Some(0); }
                                SortOrder::Desc => { sm.clear(); }
                              }
                            } else {
                              sm.clear();
                              sm.push(SortModel { col_id: col_id.clone(), sort: SortOrder::Asc, sort_index: Some(0) });
                            }
                          });
                        }
                      };
                      view!{
                        <div class="lc-dt-header-cell flex items-center gap-2 px-3 py-2 border-r border-gray-200 text-gray-700 font-medium select-none cursor-pointer" on:click=on_click>
                            <span class="truncate">{c.header_name}</span>
                            <span class="ml-auto inline-flex flex-col text-gray-400">
                                <span class="leading-none -mb-1" class=("text-gray-700", {
                                    let state_for_sort = state_for_sort.clone();
                                    let col_id = col_id.clone();
                                    move || {
                                        let ord = state_for_sort.sort_model.with(|sm| sm.iter().find(|s| s.col_id == col_id).map(|s| s.sort.clone()));
                                        matches!(ord, Some(SortOrder::Asc))
                                    }
                                })>{"↑"}</span>
                                <span class="leading-none" class=("text-gray-700", {
                                    let state_for_sort = state_for_sort.clone();
                                    let col_id = col_id.clone();
                                    move || {
                                        let ord = state_for_sort.sort_model.with(|sm| sm.iter().find(|s| s.col_id == col_id).map(|s| s.sort.clone()));
                                        matches!(ord, Some(SortOrder::Desc))
                                    }
                                })>{"↓"}</span>
                            </span>
                        </div>
                      }
                    }
                }}
            />
        </div>
    }
}

#[component]
pub fn HeaderCell<T: Send + Sync + 'static>(col: ColumnDef<T>) -> impl IntoView {
    // Simple AG Grid-like header cell: label + sort icons placeholders
    view! {
        <div class="lc-dt-header-cell flex items-center gap-2 px-3 py-2 border-r border-gray-200 text-gray-700 font-medium select-none">
            <span class="truncate">{col.header_name}</span>
            <span class="ml-auto inline-flex flex-col text-gray-400">
                <span class="leading-none -mb-1">{"↑"}</span>
                <span class="leading-none">{"↓"}</span>
            </span>
        </div>
    }
}

pub fn attach_resize_handle<T>(_col: &ColumnDef<T>) {}
pub fn start_drag_move<T>(_col: &ColumnDef<T>) {}
