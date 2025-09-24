use crate::common::filter::FilterOperator;
use crate::pages::components::button::ButtonVariant;
use crate::pages::components::datatable::core::column::ColumnDef;
use crate::pages::components::datatable::core::column::DataType;
use crate::pages::components::datatable::core::data_source::{SortModel, SortOrder};
use crate::pages::components::datatable::core::state::TableState;
use crate::pages::components::datatable::features::filter::boolean::BooleanFilter;
use crate::pages::components::datatable::features::filter::{
    date::DateFilter, datetime::DateTimeFilter, float::FloatFilter, integer::IntegerFilter,
    text::TextFilter, time::TimeFilter, IFilter,
};
use crate::pages::components::datatable::features::resize_service::ResizeService;
use crate::pages::components::Button;
use crate::pages::components::{Popover, PopoverContent, PopoverTrigger};
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
                children=move |c| {
                    let st = state.clone();
                    view! { <HeaderCell col=c state=st /> }
                }
            />
        </div>
    }
}

#[component]
pub fn HeaderCell<T: Clone + Send + Sync + 'static>(
    col: ColumnDef<T>,
    #[prop(into)] state: Arc<TableState<T>>,
) -> impl IntoView {
    let state_for_sort = state.clone();
    let state_for_filter = state.clone();
    let col_id = col.id.to_string();
    let col_id_sort = col_id.clone();
    let col_id_filter = col_id.clone();

    // Sorting click only on the label area
    let on_sort_click = {
        let state = state.clone();
        let col_id = col_id_sort.clone();
        Callback::new(move |_| {
            state.sort_model.update(|sm| {
                if let Some(pos) = sm.iter().position(|s| s.col_id == col_id) {
                    match sm[pos].sort {
                        SortOrder::Asc => {
                            sm[pos].sort = SortOrder::Desc;
                            sm[pos].sort_index = Some(0);
                        }
                        SortOrder::Desc => {
                            sm.clear();
                        }
                    }
                } else {
                    sm.clear();
                    sm.push(SortModel {
                        col_id: col_id.clone(),
                        sort: SortOrder::Asc,
                        sort_index: Some(0),
                    });
                }
            });
            // If server-side sorting is enabled (i.e., client-side disabled), notify consumers
            if !state.client_side_sorting.get_untracked() {
                // reset to first page to mimic typical server-side sort behavior
                state.current_page.set(1);
                state.notify_query_changed();
            }
        })
    };

    let is_filterable = col.filterable;
    // Determine if this column currently has any active filter (text contains or advanced operator/value)
    let _has_filter = {
        let st = state_for_filter.clone();
        let cid = col_id_filter.clone();
        move || {
            st.filter_model.with(|fm| {
                let text_active = fm
                    .column_text
                    .get(&cid)
                    .map(|s| !s.trim().is_empty())
                    .unwrap_or(false);
                let adv_active = fm
                    .column_advanced
                    .get(&cid)
                    .map(|adv| match adv.operator {
                        FilterOperator::IsNull | FilterOperator::NotNull => true,
                        _ => adv.value.is_some(),
                    })
                    .unwrap_or(false);
                text_active || adv_active
            })
        }
    };
    let _has_filter_title = {
        let st = state_for_filter.clone();
        let cid = col_id_filter.clone();
        move || {
            st.filter_model.with(|fm| {
                let text_active = fm
                    .column_text
                    .get(&cid)
                    .map(|s| !s.trim().is_empty())
                    .unwrap_or(false);
                let adv_active = fm
                    .column_advanced
                    .get(&cid)
                    .map(|adv| match adv.operator {
                        FilterOperator::IsNull | FilterOperator::NotNull => true,
                        _ => adv.value.is_some(),
                    })
                    .unwrap_or(false);
                text_active || adv_active
            })
        }
    };

    use std::cell::RefCell;
    use std::rc::Rc;
    let resize = Rc::new(RefCell::new(ResizeService::new(state.clone())));
    // Resizer handlers: delegate completely to ResizeService
    let on_resize_mousedown = {
        let rs = resize.clone();
        let id = col_id.clone();
        move |ev: leptos::ev::MouseEvent| {
            rs.borrow_mut().begin_resize(&id, ev.client_x());
        }
    };
    let on_resize_dblclick = {
        let svc = ResizeService::new(state.clone());
        let id = col_id.clone();
        move |_ev: leptos::ev::MouseEvent| {
            svc.auto_size_header_only(&id);
        }
    };

    view! {
        <div class="lc-dt-header-cell relative flex items-center gap-0 pl-3 py-2 border-r border-gray-200 text-gray-700 font-medium select-none">
            <Button class="pl-0 flex-1 justify-start" variant={ButtonVariant::Ghost} on_click=on_sort_click>
                <span class="truncate mr-2">{col.header_name}</span>
                { let state_for_sort = state_for_sort.clone();
                  let col_id = col_id_sort.clone();
                  move || {
                      let ord = state_for_sort.sort_model.with(|sm| sm.iter().find(|s| s.col_id == col_id).map(|s| s.sort.clone()));
                      match ord {
                          Some(SortOrder::Asc) => view! { <span class="text-gray-700">{"↑"}</span> }.into_view(),
                          Some(SortOrder::Desc) => view! { <span class="text-gray-700">{"↓"}</span> }.into_view(),
                          None => view! { <span class="text-gray-700">{""}</span> }.into_view(),
                      }
                  }
                }
            </Button>
            <div class="ml-auto" class=("hidden", move || !is_filterable)>
                <Popover>
                    <PopoverTrigger>
                        <Button variant={ButtonVariant::Ghost}>
                            {"⏷"}
                        </Button>
                    </PopoverTrigger>
                    <PopoverContent class="bg-white border border-gray-200 rounded shadow p-2">
                        {
                            let dtype = col.data_type.unwrap_or(DataType::Text);
                            match dtype {
                                                            // Unify return type to AnyView by boxing IntoView
                                DataType::Text => {
                                    let f = TextFilter::new();
                                    let v: AnyView = view!{ {f.view()} }.into_any();
                                    v
                                }
                                DataType::Int => {
                                    let f = IntegerFilter::new();
                                    let v: AnyView = view!{ {f.view()} }.into_any();
                                    v
                                }
                                DataType::Float => {
                                    let f = FloatFilter::new();
                                    let v: AnyView = view!{ {f.view()} }.into_any();
                                    v
                                }
                                DataType::Boolean => {
                                    let f = BooleanFilter::new();
                                    let v: AnyView = view!{ {f.view()} }.into_any();
                                    v
                                }
                                DataType::Date => {
                                    let f = DateFilter::new();
                                    let v: AnyView = view!{ {f.view()} }.into_any();
                                    v
                                }
                                DataType::Time => {
                                    let f = TimeFilter::new();
                                    let v: AnyView = view!{ {f.view()} }.into_any();
                                    v
                                }
                                DataType::DateTime => {
                                    let f = DateTimeFilter::new();
                                    let v: AnyView = view!{ {f.view()} }.into_any();
                                    v
                                }
                            }
                        }
                    </PopoverContent>
                </Popover>
            </div>
            // Resizer (shown only if column is resizable)
            <div class=("hidden", move || !col.resizable)
                 class="relative w-0 h-full"
            >
                <div class="lc-dt-col-resizer absolute top-0 right-0 w-1 h-full cursor-col-resize select-none"
                     on:mousedown=on_resize_mousedown
                     on:dblclick=on_resize_dblclick
                />
            </div>
        </div>
    }
}

pub fn attach_resize_handle<T>(_col: &ColumnDef<T>) {}
pub fn start_drag_move<T>(_col: &ColumnDef<T>) {}
