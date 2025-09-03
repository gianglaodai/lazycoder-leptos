use crate::pages::admin::guard::AdminGuard;
use crate::pages::components::datatable::core::column::{ColumnDef, Pinned};
use crate::pages::components::datatable::core::render_value::Value;
use crate::pages::components::datatable::core::row::RowNode;
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::{use_navigate, use_query_map};
use std::sync::Arc;

#[component]
fn DataTableCtx() -> impl IntoView {
    use crate::pages::components::datatable::DataTable;
    let ts: Arc<TableState<crate::pages::rest::term_info_api::TermInfoTO>> = expect_context();
    view! { <DataTable state=ts height="600px".to_string() row_height=36 /> }
}

#[component]
pub fn AdminTermsPage() -> impl IntoView {
    let query = use_query_map();

    let table_state: Arc<TableState<crate::pages::rest::term_info_api::TermInfoTO>> = Arc::new(TableState::new());
    provide_context(table_state.clone());

    table_state.client_side_sorting.set(false);
    table_state.client_side_filtering.set(false);

    table_state.columns.set(vec![
        ColumnDef { id: "id", header_name: "ID", value_getter: Some(Arc::new(|p: &crate::pages::rest::term_info_api::TermInfoTO| Value::Number(p.id as f64))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: true, resizable: true, movable: false, pinned: Pinned::None, width: 80, min_width: 60, max_width: Some(140), groupable: false, aggregate: None, comparator: None, field: Some("id"), data_type: Some(crate::pages::components::datatable::core::column::DataType::Int), },
        ColumnDef { id: "taxonomy_code", header_name: "Taxonomy", value_getter: Some(Arc::new(|p: &crate::pages::rest::term_info_api::TermInfoTO| Value::Text(p.taxonomy_code.clone()))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: true, resizable: true, movable: true, pinned: Pinned::None, width: 160, min_width: 100, max_width: None, groupable: false, aggregate: None, comparator: None, field: Some("taxonomy_code"), data_type: Some(crate::pages::components::datatable::core::column::DataType::Text), },
        ColumnDef { id: "parent_slug", header_name: "Parent", value_getter: Some(Arc::new(|p: &crate::pages::rest::term_info_api::TermInfoTO| Value::Text(p.parent_slug.clone().unwrap_or_default()))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: true, resizable: true, movable: true, pinned: Pinned::None, width: 160, min_width: 100, max_width: None, groupable: false, aggregate: None, comparator: None, field: Some("parent_slug"), data_type: Some(crate::pages::components::datatable::core::column::DataType::Text), },
        ColumnDef { id: "slug", header_name: "Slug", value_getter: Some(Arc::new(|p: &crate::pages::rest::term_info_api::TermInfoTO| Value::Text(p.slug.clone()))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: true, resizable: true, movable: true, pinned: Pinned::None, width: 220, min_width: 120, max_width: None, groupable: false, aggregate: None, comparator: None, field: Some("slug"), data_type: Some(crate::pages::components::datatable::core::column::DataType::Text), },
        ColumnDef { id: "name", header_name: "Name", value_getter: Some(Arc::new(|p: &crate::pages::rest::term_info_api::TermInfoTO| Value::Text(p.name.clone()))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: true, resizable: true, movable: true, pinned: Pinned::None, width: 220, min_width: 120, max_width: None, groupable: false, aggregate: None, comparator: None, field: Some("name"), data_type: Some(crate::pages::components::datatable::core::column::DataType::Text), },
        ColumnDef { id: "updated_at", header_name: "Updated", value_getter: Some(Arc::new(|p: &crate::pages::rest::term_info_api::TermInfoTO| Value::Text(p.updated_at.to_string()))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: false, resizable: true, movable: true, pinned: Pinned::None, width: 180, min_width: 140, max_width: None, groupable: false, aggregate: None, comparator: None, field: Some("updated_at"), data_type: Some(crate::pages::components::datatable::core::column::DataType::DateTime), },
    ]);

    table_state.page_size.set(10);

    let resource = Resource::new(
        {
            let table_state = table_state.clone();
            move || {
                let sort: Option<String> = query.with(|q| q.get("sort").map(|s| s.to_string()));
                let search: Option<String> = query.with(|q| q.get("search").map(|s| s.to_string()));
                let ps = table_state.page_size.get();
                let cp = table_state.current_page.get();
                let first_result_i32 = ((cp.saturating_sub(1) * ps) as i32).max(0);
                let max_results_i32 = ps as i32;
                (first_result_i32, max_results_i32, sort, search)
            }
        },
        |(first_result_i32, max_results_i32, sort, search)| async move {
            use crate::pages::rest::term_info_api::{load_term_infos, count_term_infos};
            let (a, b) = futures::join!(
                load_term_infos(first_result_i32 as i64, max_results_i32, sort.clone(), search.clone(), None, None),
                count_term_infos(search.clone(), None, None)
            );
            match (a, b) { (Ok(items), Ok(total)) => Ok((items, total)), (Err(e), _) => Err(e), (_, Err(e)) => Err(e) }
        }
    );

    {
        use crate::pages::components::datatable::core::query_sync::{sync_table_query_to_url, SyncOptions};
        let nav = use_navigate();
        let st = table_state.clone();
        sync_table_query_to_url(st, move |qs| { nav(&qs, leptos_router::NavigateOptions { replace: true, ..Default::default() }); }, SyncOptions { include_sort: true, include_p_filters: false, include_a_filters: false, include_search: false, ..Default::default() });
    }

    Effect::new({
        let table_state = table_state.clone();
        move |_| {
            if let Some(Ok((items, total))) = resource.get() {
                let rows: Vec<RowNode<_>> = items.into_iter().map(|p| RowNode::new(p.id.to_string(), p)).collect();
                table_state.set_rows(rows);
                table_state.set_total_rows(Some(total as usize));
            }
        }
    });

    view! {
        <AdminGuard>
            <crate::pages::components::sidebar::SidebarProvider default_open=true>
                <div class="flex gap-0">
                    <crate::pages::admin::layout::AdminSidebar />
                    <main class="flex-1 min-h-screen">
                        <div class="container-page py-10 font-serif">
                            <div class="flex items-center justify-between mb-6">
                                <h1 class="text-3xl font-bold">Terms</h1>
                            </div>
                            <div class="mt-4">
                                <DataTableCtx />
                            </div>
                        </div>
                    </main>
                </div>
            </crate::pages::components::sidebar::SidebarProvider>
        </AdminGuard>
    }
}