use leptos::prelude::*;
use leptos::web_sys::window;
use leptos_router::hooks::{use_navigate, use_query_map};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::pages::components::{
    button::ButtonIntent, button::ButtonVariant, Button, Checkbox, ColumnFilter, Input, Paginator,
    Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow,
};
use crate::value_data_type::ValueDataType;

/// DataTable (shadcn style)
///
/// Structure:
/// - Top toolbar (search input placeholder for future filtering)
/// - Table (headers from field_definitions, rows from HashMap)
/// - Footer with right-aligned pagination
///
/// Props remain compatible with previous implementation for minimal integration changes.
type FieldName = String;
type FieldLabel = String;
type FieldValue = String;
type Row = HashMap<FieldName, FieldValue>;
#[component]
pub fn DataTable(
    field_definitions: Vec<(FieldName, FieldLabel, ValueDataType)>,
    rows: Vec<Row>,
    total_entities: i64,
    #[prop(optional, default = 0)] first_result: i64,
    #[prop(optional, default = 5)] max_results: i64,
    #[prop(optional, default = 7)] max_visible_pages: i64,
    #[prop(optional, default = false)] editable: bool,
    #[prop(optional, default = false)] deletable: bool,
    #[prop(into, optional)] on_edit: Option<Callback<HashMap<String, String>, ()>>,
    #[prop(into, optional)] on_delete: Option<Callback<HashMap<String, String>, ()>>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] caption: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let caption = caption.unwrap_or_default();

    // Prepare header labels and field order
    let headers: Vec<String> = field_definitions
        .iter()
        .map(|(_, label, _)| label.clone())
        .collect();
    let fields: Vec<String> = field_definitions
        .iter()
        .map(|(name, _, _)| name.clone())
        .collect();

    // Selection state keyed by a unique field (id preferred, else first field)
    let rows_rc: Arc<Vec<HashMap<String, String>>> = Arc::new(rows.clone());
    let id_field = if fields.iter().any(|f| f == "id") {
        "id".to_string()
    } else {
        fields.get(0).cloned().unwrap_or_else(|| "id".to_string())
    };
    let selected: RwSignal<HashSet<String>> = RwSignal::new(HashSet::new());

    // Helpers
    let get_row_id = {
        let id_field = id_field.clone();
        move |row: &HashMap<String, String>| -> Option<String> { row.get(&id_field).cloned() }
    };

    // Precompute ids for current rows
    let row_ids: Vec<Option<String>> = rows_rc.iter().map(|r| get_row_id(r)).collect();

    // Header select-all checkbox state
    let all_count = row_ids.iter().filter(|id| id.is_some()).count();
    let any_selected = Memo::new({
        let selected = selected.read_only();
        let row_ids = row_ids.clone();
        move |_| {
            row_ids
                .iter()
                .any(|id| id.as_ref().is_some_and(|k| selected.get().contains(k)))
        }
    });
    let all_selected = Memo::new({
        let selected = selected.read_only();
        let row_ids = row_ids.clone();
        move |_| {
            if all_count == 0 {
                return false;
            }
            row_ids
                .iter()
                .filter_map(|id| id.as_ref())
                .all(|k| selected.get().contains(k))
        }
    });
    let indeterminate = Memo::new({
        let any_selected = any_selected.clone();
        let all_selected = all_selected.clone();
        move |_| any_selected.get() && !all_selected.get()
    });

    // Build header cells (prepend checkbox; append Action if needed)
    let mut header_cells: Vec<AnyView> = Vec::new();
    // Selection header
    header_cells.push(
        view! { <TableHead class="w-8"> <Checkbox checked=all_selected indeterminate=indeterminate on_change=Callback::new(move |checked| {
            // Toggle select all visible
            let mut set = selected.get();
            if checked {
                for id in row_ids.iter().filter_map(|x| x.clone()) { set.insert(id); }
            } else {
                for id in row_ids.iter().filter_map(|x| x.clone()) { set.remove(&id); }
            }
            selected.set(set);
        }) /> </TableHead> }.into_any()
    );

    // Query handling owned by DataTable
    let query = use_query_map();
    let navigate = use_navigate();
    let current_sort =
        move || query.with(|q| q.get("sort").map(|s| s.to_string()).unwrap_or_default());
    let current_search =
        move || query.with(|q| q.get("search").map(|s| s.to_string()).unwrap_or_default());
    fn get_query_search_string() -> String {
        window()
            .and_then(|w| Some(w.location()))
            .and_then(|loc| loc.search().ok())
            .unwrap_or_default()
    }
    fn parse_all_pfilters_from_search(search: &str) -> Vec<String> {
        let qs = search.trim_start_matches('?');
        qs.split('&')
            .filter_map(|kv| {
                let mut it = kv.splitn(2, '=');
                let k = it.next()?;
                let v = it.next().unwrap_or("");
                if k == "p_filters" {
                    Some(v.to_string())
                } else {
                    None
                }
            })
            .collect()
    }
    fn find_field_filter(filters: &[String], field: &str) -> Option<(String, String, String)> {
        for f in filters {
            let parts: Vec<&str> = f.splitn(4, ':').collect();
            if parts.len() >= 3 && parts[0] == field {
                let op = parts.get(1).copied().unwrap_or("=").to_string();
                let val = parts.get(2).copied().unwrap_or("").to_string();
                let dt = parts.get(3).copied().unwrap_or("").to_string();
                return Some((op, val, dt));
            }
        }
        None
    }
    fn build_url_with(
        max_results: i64,
        sort: String,
        search: String,
        filters: &[String],
    ) -> String {
        let mut url = format!("?first_result=0&max_results={}", max_results);
        if !sort.is_empty() {
            url.push_str(&format!("&sort={}", sort));
        }
        if !search.is_empty() {
            url.push_str(&format!("&search={}", search));
        }
        for f in filters {
            if !f.is_empty() {
                url.push_str(&format!("&p_filters={}", f));
            }
        }
        url
    }

    // Then data headers (with ColumnFilter per header)
    header_cells.extend({
        let field_defs = field_definitions.clone();
        let max_results_copy = max_results;
        let current_sort_fn = current_sort.clone();
        let current_search_fn = current_search.clone();
        let navigate_clone = navigate.clone();
        field_defs
            .into_iter()
            .map(move |(name, label, dtype)| {
                // derive signals
                let field_for_sig = name.clone();
                let op_sig: Signal<String> = Signal::derive({
                    let field_for_op = field_for_sig.clone();
                    move || {
                        let search = get_query_search_string();
                        let filters = parse_all_pfilters_from_search(&search);
                        if let Some((op, _v, _)) = find_field_filter(&filters, &field_for_op) { op } else { "=".to_string() }
                    }
                });
                let val_sig: Signal<String> = Signal::derive({
                    let field_for_val = field_for_sig.clone();
                    move || {
                        let search = get_query_search_string();
                        let filters = parse_all_pfilters_from_search(&search);
                        if let Some((_op, v, _)) = find_field_filter(&filters, &field_for_val) { v } else { String::new() }
                    }
                });
                let on_op = {
                    let name = name.clone();
                    let navigate = navigate_clone.clone();
                    let current_sort = current_sort_fn.clone();
                    let current_search = current_search_fn.clone();
                    Callback::new(move |op: String| {
                        let search = get_query_search_string();
                        let mut filters = parse_all_pfilters_from_search(&search);
                        let vcur = find_field_filter(&filters, &name).map(|(_, v, _)| v).unwrap_or_default();
                        filters.retain(|f| f.splitn(4, ':').next().unwrap_or("") != name);
                        if !vcur.is_empty() { filters.push(format!("{}:{}:{}:{}", name, op, vcur, dtype.to_code())); }
                        let url = build_url_with(max_results_copy, current_sort(), current_search(), &filters);
                        let _ = navigate(&url, Default::default());
                    })
                };
                let on_val = {
                    let name = name.clone();
                    let navigate = navigate_clone.clone();
                    let current_sort = current_sort_fn.clone();
                    let current_search = current_search_fn.clone();
                    Callback::new(move |v: String| {
                        let search = get_query_search_string();
                        let mut filters = parse_all_pfilters_from_search(&search);
                        let op_cur = find_field_filter(&filters, &name).map(|(op, _v, _)| op).unwrap_or("=".to_string());
                        filters.retain(|f| f.splitn(4, ':').next().unwrap_or("") != name);
                        if !v.is_empty() { filters.push(format!("{}:{}:{}:{}", name, op_cur, v, dtype.to_code())); }
                        let url = build_url_with(max_results_copy, current_sort(), current_search(), &filters);
                        let _ = navigate(&url, Default::default());
                    })
                };
                view! {
                    <TableHead>
                        <div class="space-y-1">
                            <div>{label.clone()}</div>
                            <ColumnFilter field_name=name.clone() field_datatype=dtype operator=op_sig value=val_sig on_operator_change=on_op on_value_change=on_val class="mt-1" />
                        </div>
                    </TableHead>
                }.into_any()
            })
    });
    if editable || deletable {
        header_cells
            .push(view! { <TableHead class="text-right">{"Action"}</TableHead> }.into_any());
    }

    // Build body rows
    let body_rows = rows_rc
        .iter()
        .cloned()
        .map({
            let fields = fields.clone();
            let on_edit = on_edit.clone();
            let on_delete = on_delete.clone();
            let selected = selected.clone();
            let get_row_id = get_row_id.clone();
            move |row| {
                let mut cells: Vec<AnyView> = Vec::new();
                // Selection cell
                let rid = get_row_id(&row).unwrap_or_default();
                let is_checked = Memo::new({
                    let selected = selected.read_only();
                    let rid = rid.clone();
                    move |_| selected.get().contains(&rid)
                });
                let rid_for_toggle = rid.clone();
                cells.push(
                    view! { <TableCell class="w-8">
                        <Checkbox checked=is_checked on_change=Callback::new(move |checked| {
                            let mut set = selected.get();
                            if checked { set.insert(rid_for_toggle.clone()); } else { set.remove(&rid_for_toggle); }
                            selected.set(set);
                        }) />
                    </TableCell> }.into_any(),
                );
                // Data cells
                for name in fields.iter() {
                    let value = row.get(name).cloned().unwrap_or_default();
                    cells.push(view! { <TableCell>{value}</TableCell> }.into_any());
                }
                // Action cells
                if editable || deletable {
                    let mut action_children: Vec<AnyView> = Vec::new();
                    if editable {
                        if let Some(cb) = on_edit.clone() {
                            let row_clone = row.clone();
                            action_children.push(
                                view! { <Button variant=ButtonVariant::Outline intent=ButtonIntent::Primary on_click=Callback::new(move |_| { cb.run(row_clone.clone()); })>"Edit"</Button> }.into_any()
                            );
                        } else {
                            action_children.push(
                                view! { <Button variant=ButtonVariant::Outline intent=ButtonIntent::Primary disabled=true>"Edit"</Button> }.into_any()
                            );
                        }
                    }
                    if deletable {
                        if let Some(cb) = on_delete.clone() {
                            let row_clone = row.clone();
                            action_children.push(
                                view! { <Button variant=ButtonVariant::Outline intent=ButtonIntent::Destructive on_click=Callback::new(move |_| { cb.run(row_clone.clone()); })>"Delete"</Button> }.into_any()
                            );
                        } else {
                            action_children.push(
                                view! { <Button variant=ButtonVariant::Outline intent=ButtonIntent::Destructive disabled=true>"Delete"</Button> }.into_any()
                            );
                        }
                    }
                    cells.push(view! { <TableCell class="text-right flex flex-row-reverse gap-2">{view! {{action_children}}}</TableCell> }.into_any());
                }
                view! {
                    <TableRow>
                        {view! {{cells}}}
                    </TableRow>
                }
                .into_any()
            }
        })
        .collect_view();

    // Bulk delete button enabled when any selected
    let toolbar_right = {
        if deletable {
            if let Some(cb) = on_delete.clone() {
                let selected = selected.clone();
                let rows_rc = rows_rc.clone();
                let id_field = id_field.clone();
                let any_sel = any_selected.clone();
                view! {
                    <Button variant=ButtonVariant::Outline intent=ButtonIntent::Destructive disabled_signal=Signal::derive(move || !any_sel.get()) on_click=Callback::new(move |_| {
                        let sel = selected.get();
                        for row in rows_rc.iter() {
                            if let Some(id) = row.get(&id_field) { if sel.contains(id) { cb.run(row.clone()); } }
                        }
                        // clear selection
                        selected.set(HashSet::new());
                    })>
                        "Delete"
                    </Button>
                }
                .into_any()
            } else {
                view! { <Button variant=ButtonVariant::Outline intent=ButtonIntent::Destructive disabled=true>"Delete Selected"</Button> }.into_any()
            }
        } else {
            view! { <div></div> }.into_any()
        }
    };

    // Render shadcn-like layout
    view! {
        <div class=move || crate::cn!("space-y-4", class.clone())>
            <div class="flex items-center justify-between">
                <div class="flex-1">
                    <Input placeholder="Filter..." class="max-w-sm" />
                </div>
                <div class="flex items-center gap-2">{toolbar_right}</div>
            </div>

            <Table>
                <TableHeader>
                    <TableRow>
                        {view! {{header_cells}}}
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {view! {{body_rows}}}
                </TableBody>
                {if caption.is_empty() { view! { <TableCaption>{""}</TableCaption> }.into_any() } else { view! { <TableCaption>{caption.clone()}</TableCaption> }.into_any() }}
            </Table>

            <div class="flex items-center justify-end">
                <Paginator
                    first_result=first_result
                    total_entities=total_entities
                    max_results=max_results
                    max_visible_pages=max_visible_pages
                />
            </div>
        </div>
    }
}
