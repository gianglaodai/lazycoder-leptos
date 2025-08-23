use leptos::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::pages::components::{
    button::ButtonIntent, button::ButtonVariant, Button, Checkbox, Input, Paginator, Table,
    TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow,
};

/// DataTable (shadcn style)
///
/// Structure:
/// - Top toolbar (search input placeholder for future filtering)
/// - Table (headers from field_definitions, rows from HashMap)
/// - Footer with right-aligned pagination
///
/// Props remain compatible with previous implementation for minimal integration changes.
#[component]
pub fn DataTable(
    field_definitions: Vec<(String, String)>,
    rows: Vec<HashMap<String, String>>,
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
        .map(|(_, label)| label.clone())
        .collect();
    let fields: Vec<String> = field_definitions
        .iter()
        .map(|(name, _)| name.clone())
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
        move |_| row_ids.iter().any(|id| id.as_ref().is_some_and(|k| selected.get().contains(k)))
    });
    let all_selected = Memo::new({
        let selected = selected.read_only();
        let row_ids = row_ids.clone();
        move |_| {
            if all_count == 0 { return false; }
            row_ids.iter().filter_map(|id| id.as_ref()).all(|k| selected.get().contains(k))
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
    // Then data headers
    header_cells.extend(
        headers
            .into_iter()
            .map(|label| view! { <TableHead>{label}</TableHead> }.into_any())
    );
    if editable || deletable {
        header_cells.push(view! { <TableHead class="text-right">{"Action"}</TableHead> }.into_any());
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
                        "Delete Selected"
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
