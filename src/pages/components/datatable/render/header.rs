use crate::business::filter::{FilterOperator, FilterValue};
use crate::pages::components::datatable::core::column::ColumnDef;
use crate::pages::components::datatable::core::column::DataType;
use crate::pages::components::datatable::core::data_source::{SortModel, SortOrder};
use crate::pages::components::datatable::core::state::TableState;
use crate::pages::components::{Popover, PopoverContent, PopoverTrigger};
use leptos::prelude::event_target_value;
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

    // Sorting click only on the label area
    let on_sort_click = {
        let state = state.clone();
        let col_id = col_id.clone();
        move |_| {
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
        }
    };

    let is_filterable = col.filterable;

    view! {
        <div class="lc-dt-header-cell flex items-center gap-2 px-3 py-2 border-r border-gray-200 text-gray-700 font-medium select-none justify-between">
            <button class="flex items-center gap-1 min-w-0" on:click=on_sort_click>
                <span class="truncate">{col.header_name}</span>
                { let state_for_sort = state_for_sort.clone();
                  let col_id = col_id.clone();
                  move || {
                      let ord = state_for_sort.sort_model.with(|sm| sm.iter().find(|s| s.col_id == col_id).map(|s| s.sort.clone()));
                      match ord {
                          Some(SortOrder::Asc) => view! { <span class="text-gray-700">{"↑"}</span> }.into_view(),
                          Some(SortOrder::Desc) => view! { <span class="text-gray-700">{"↓"}</span> }.into_view(),
                          None => view! { <span class="text-gray-700">{""}</span> }.into_view(),
                      }
                  }
                }
            </button>
            <div class="ml-auto" class=("hidden", move || !is_filterable)>
                <Popover>
                    <PopoverTrigger>
                        <button class="text-gray-500 hover:text-gray-700" title="Filter">
                            {"⏷"}
                        </button>
                    </PopoverTrigger>
                    <PopoverContent class="bg-white border border-gray-200 rounded shadow p-2">
                        {
                            let operator = RwSignal::new(String::from(match col.data_type.unwrap_or(DataType::Text) {
                                DataType::Text => "contains",
                                DataType::Int | DataType::Float => "equals",
                                DataType::Boolean => "is",
                                DataType::Date | DataType::Time | DataType::DateTime => "equals",
                            }));
                            let value = RwSignal::new(String::new());
                            let to_op = |s: &str| -> FilterOperator {
                                match s {
                                    "contains" => FilterOperator::Like,
                                    "notContains" => FilterOperator::NotLike,
                                    "equals" => FilterOperator::Equal,
                                    "notEqual" => FilterOperator::NotEqual,
                                    "lt" => FilterOperator::LessThan,
                                    "lte" => FilterOperator::LessThanOrEqual,
                                    "gt" => FilterOperator::GreaterThan,
                                    "gte" => FilterOperator::GreaterThanOrEqual,
                                    "is" => FilterOperator::Is,
                                    "before" => FilterOperator::LessThan,
                                    "after" => FilterOperator::GreaterThan,
                                    _ => FilterOperator::Equal,
                                }
                            };
                            let parse_val = {
                                let dtype = col.data_type.unwrap_or(DataType::Text);
                                move |txt: &str| -> Option<FilterValue> {
                                    let t = txt.trim();
                                    if t.is_empty() { return None; }
                                    match dtype {
                                        DataType::Text => Some(FilterValue::String(t.to_string())),
                                        DataType::Int => t.parse::<i32>().ok().map(FilterValue::Int),
                                        DataType::Float => t.parse::<f64>().ok().map(FilterValue::Float),
                                        DataType::Boolean => {
                                            match t.to_lowercase().as_str() { "true"|"1"|"yes"|"y" => Some(FilterValue::Bool(true)), "false"|"0"|"no"|"n" => Some(FilterValue::Bool(false)), _ => None }
                                        }
                                        DataType::Date => {
                                            let fmt = time::macros::format_description!("[year]-[month]-[day]");
                                            time::Date::parse(t, &fmt).ok().map(FilterValue::Date)
                                        }
                                        DataType::Time => {
                                            let fmt1 = time::macros::format_description!("[hour]:[minute]");
                                            let fmt2 = time::macros::format_description!("[hour]:[minute]:[second]");
                                            time::Time::parse(t, &fmt1).ok().or_else(|| time::Time::parse(t, &fmt2).ok()).map(FilterValue::Time)
                                        }
                                        DataType::DateTime => {
                                            let fmt1 = time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]");
                                            let fmt2 = time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
                                            if let Ok(pdt) = time::PrimitiveDateTime::parse(t, &fmt1) { Some(FilterValue::DateTime(pdt.assume_utc())) }
                                            else if let Ok(pdt2) = time::PrimitiveDateTime::parse(t, &fmt2) { Some(FilterValue::DateTime(pdt2.assume_utc())) }
                                            else { None }
                                        }
                                    }
                                }
                            };
                            let state2 = state_for_filter.clone();
                            let col_id2 = col_id.clone();
                            let apply = std::rc::Rc::new(move || {
                                let op_s = operator.get();
                                let op = to_op(&op_s);
                                let val_opt = parse_val(&value.get());
                                state2.filter_model.update(|fm| {
                                    if val_opt.is_none() && !matches!(op, FilterOperator::Is | FilterOperator::IsNull | FilterOperator::NotNull) {
                                        fm.column_advanced.remove(&col_id2);
                                    } else {
                                        fm.column_advanced.insert(col_id2.clone(), crate::pages::components::datatable::core::data_source::AdvancedFilter { operator: op.clone(), value: val_opt.clone() });
                                    }
                                    if let Some(dtype) = col.data_type { if matches!(dtype, crate::pages::components::datatable::core::column::DataType::Text) && matches!(op, FilterOperator::Like) {
                                        let v = value.get();
                                        if v.trim().is_empty() { fm.column_text.remove(&col_id2); } else { fm.column_text.insert(col_id2.clone(), v); }
                                    } else {
                                        fm.column_text.remove(&col_id2);
                                    }}
                                });
                            });
                            let dtype = col.data_type.unwrap_or(DataType::Text);
                            view!{
                                <div class="flex flex-col items-center gap-2">
                                    <select class="w-full border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 bg-white" on:change={ let a = apply.clone(); move |ev| { operator.set(event_target_value(&ev)); a(); } }>
                                        <option value="~" class:hidden=move || !matches!(dtype, DataType::Text)>"Contains"</option>
                                        <option value="!~" class:hidden=move || !matches!(dtype, DataType::Text)>"Does not contains"</option>
                                        <option value="=">"Equals"</option>
                                        <option value="!=">"Does not equals"</option>
                                        <option value="=null">"Is null"</option>
                                        <option value="!null">"Not null"</option>
                                        <option value="<" class:hidden=move || matches!(dtype, DataType::Text | DataType::Boolean)>"Less than"</option>
                                        <option value="<=" class:hidden=move || matches!(dtype, DataType::Text | DataType::Boolean)>"Less than or equal to"</option>
                                        <option value=">" class:hidden=move || matches!(dtype, DataType::Text | DataType::Boolean)>"Greater than"</option>
                                        <option value=">=" class:hidden=move || matches!(dtype, DataType::Text | DataType::Boolean)>"Greater than or equal to"</option>
                                        <option value="<" class:hidden=move || !matches!(dtype, DataType::Date | DataType::Time | DataType::DateTime)>"Before"</option>
                                        <option value=">" class:hidden=move || !matches!(dtype, DataType::Date | DataType::Time | DataType::DateTime)>"After"</option>
                                        <option value="is" class:hidden=move || !matches!(dtype, DataType::Boolean)>"Is"</option>
                                    </select>
                                    <input type="text" class:hidden=move || !matches!(dtype, DataType::Text) class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 w-48" placeholder="Filter..." on:input={ let a = apply.clone(); move |ev| { value.set(event_target_value(&ev)); a(); } } />
                                    <input type="number" step="any" class:hidden=move || !matches!(dtype, DataType::Int | DataType::Float) class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700 w-32" on:input={ let a = apply.clone(); move |ev| { value.set(event_target_value(&ev)); a(); } } />
                                    <select class:hidden=move || !matches!(dtype, DataType::Boolean) class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700" on:change={ let a = apply.clone(); move |ev| { value.set(event_target_value(&ev)); a(); } }><option value="true">"true"</option><option value="false">"false"</option></select>
                                    <input type="date" class:hidden=move || !matches!(dtype, DataType::Date) class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700" on:input={ let a = apply.clone(); move |ev| { value.set(event_target_value(&ev)); a(); } } />
                                    <input type="time" class:hidden=move || !matches!(dtype, DataType::Time) class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700" on:input={ let a = apply.clone(); move |ev| { value.set(event_target_value(&ev)); a(); } } />
                                    <input type="datetime-local" class:hidden=move || !matches!(dtype, DataType::DateTime) class="border border-gray-300 rounded px-2 py-1 text-xs text-gray-700" on:input={ let a = apply.clone(); move |ev| { value.set(event_target_value(&ev)); a(); } } />
                                </div>
                            }
                        }
                    </PopoverContent>
                </Popover>
            </div>
        </div>
    }
}

pub fn attach_resize_handle<T>(_col: &ColumnDef<T>) {}
pub fn start_drag_move<T>(_col: &ColumnDef<T>) {}
