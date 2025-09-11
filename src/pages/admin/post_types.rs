use crate::pages::admin::guard::AdminGuard;
use crate::pages::components::datatable::core::column::{ColumnDef, Pinned};
use crate::pages::components::datatable::core::render_value::Value;
use crate::pages::components::datatable::core::row::RowNode;
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::{use_navigate, use_query_map};
use std::sync::Arc;

use crate::pages::components::{
    Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, DialogTrigger, Form, FormControl, FormField, FormItem, FormLabel, FormMessage,
    Input,
};
use crate::pages::components::button::{ButtonIntent, ButtonVariant};

#[component]
fn DataTableCtx() -> impl IntoView {
    use crate::pages::components::datatable::DataTable;
    let ts: Arc<TableState<crate::pages::rest::post_type_info_api::PostTypeInfoTO>> = expect_context();
    view! { <DataTable state=ts height="600px".to_string() row_height=36 /> }
}

#[component]
fn NewPostTypeDialog(on_created: Callback<()>) -> impl IntoView {
    let code = RwSignal::new(String::new());
    let name = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());

    let create_action = Action::new(move |(c, n): &(String, String)| {
        let c = c.clone();
        let n = n.clone();
        async move { crate::pages::rest::post_type_api::create_post_type(c, n).await }
    });

    Effect::new({
        let on_created = on_created.clone();
        move |_| {
            if let Some(Ok(_rows)) = create_action.value().get() {
                error.set(String::new());
                code.set(String::new());
                name.set(String::new());
                on_created.run(())
            } else if let Some(Err(e)) = create_action.value().get() {
                error.set(e.to_string());
            }
        }
    });

    let disabled: Signal<bool> = Signal::derive({
        let code = code.clone();
        let name = name.clone();
        let pending = create_action.pending();
        move || code.get().trim().is_empty() || name.get().trim().is_empty() || pending.get()
    });

    let error_sig: Signal<Option<String>> = Signal::derive({
        let error = error.clone();
        move || {
            let e = error.get();
            if e.is_empty() { None } else { Some(e) }
        }
    });

    view! {
        <Dialog>
            <DialogTrigger variant=ButtonVariant::Primary intent=ButtonIntent::Primary>New Post Type</DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>New Post Type</DialogTitle>
                    <DialogDescription>Enter a code and name to create a new post type.</DialogDescription>
                </DialogHeader>
                <Form prevent_default=true on_submit=Callback::new({
                    let disabled = disabled.clone();
                    let create_action = create_action.clone();
                    let code = code.clone();
                    let name = name.clone();
                    move |_| {
                        if !disabled.get_untracked() {
                            create_action.dispatch((code.get_untracked(), name.get_untracked()));
                        }
                    }
                })>
                    <FormField name="code".to_string() error=error_sig>
                        <FormItem>
                            <FormLabel>Code</FormLabel>
                            <FormControl>
                                <Input placeholder="e.g. article, tutorial" value=code on_input=Callback::new(move |ev: leptos::ev::Event| code.set(event_target_value(&ev))) />
                            </FormControl>
                            <FormMessage>{move || error.get()}</FormMessage>
                        </FormItem>
                    </FormField>
                    <FormField name="name".to_string() error=Signal::derive(|| None::<String>)>
                        <FormItem>
                            <FormLabel>Name</FormLabel>
                            <FormControl>
                                <Input placeholder="Display name" value=name on_input=Callback::new(move |ev: leptos::ev::Event| name.set(event_target_value(&ev))) />
                            </FormControl>
                        </FormItem>
                    </FormField>
                </Form>
                <DialogFooter>
                    <DialogClose>Cancel</DialogClose>
                    <Button
                        variant=ButtonVariant::Outline
                        intent=ButtonIntent::Primary
                        disabled_signal=disabled
                        loading_signal=create_action.pending().into()
                        on_click=Callback::new(move |_| {
                            if !disabled.get_untracked() {
                                create_action.dispatch((code.get_untracked(), name.get_untracked()));
                            }
                        })
                    >
                        {move || if create_action.pending().get() { "Creating...".to_string() } else { "Create".to_string() }}
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}

#[component]
pub fn AdminPostTypesPage() -> impl IntoView {
    let query = use_query_map();

    let table_state: Arc<TableState<crate::pages::rest::post_type_info_api::PostTypeInfoTO>> = Arc::new(TableState::new());
    provide_context(table_state.clone());

    table_state.client_side_sorting.set(false);
    table_state.client_side_filtering.set(false);

    table_state.columns.set(vec![
        ColumnDef { id: "id", header_name: "ID", value_getter: Some(Arc::new(|p: &crate::pages::rest::post_type_info_api::PostTypeInfoTO| Value::Number(p.id as f64))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: true, resizable: true, movable: false, pinned: Pinned::None, width: 80, min_width: 60, max_width: Some(140), groupable: false, aggregate: None, comparator: None, field: Some("id"), data_type: Some(crate::pages::components::datatable::core::column::DataType::Int), },
        ColumnDef { id: "uid", header_name: "UID", value_getter: Some(Arc::new(|p: &crate::pages::rest::post_type_info_api::PostTypeInfoTO| Value::Text(p.uid.clone()))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: true, resizable: true, movable: false, pinned: Pinned::None, width: 80, min_width: 120, max_width: Some(140), groupable: false, aggregate: None, comparator: None, field: Some("uid"), data_type: Some(crate::pages::components::datatable::core::column::DataType::Text), },
        ColumnDef { id: "code", header_name: "Code", value_getter: Some(Arc::new(|p: &crate::pages::rest::post_type_info_api::PostTypeInfoTO| Value::Text(p.code.clone()))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: true, resizable: true, movable: true, pinned: Pinned::None, width: 200, min_width: 120, max_width: None, groupable: false, aggregate: None, comparator: None, field: Some("code"), data_type: Some(crate::pages::components::datatable::core::column::DataType::Text), },
        ColumnDef { id: "name", header_name: "Name", value_getter: Some(Arc::new(|p: &crate::pages::rest::post_type_info_api::PostTypeInfoTO| Value::Text(p.name.clone()))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: true, resizable: true, movable: true, pinned: Pinned::None, width: 260, min_width: 120, max_width: None, groupable: false, aggregate: None, comparator: None, field: Some("name"), data_type: Some(crate::pages::components::datatable::core::column::DataType::Text), },
        ColumnDef { id: "created_at", header_name: "Created", value_getter: Some(Arc::new(|p: &crate::pages::rest::post_type_info_api::PostTypeInfoTO| Value::Text(p.created_at.to_string()))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: false, resizable: true, movable: true, pinned: Pinned::None, width: 180, min_width: 140, max_width: None, groupable: false, aggregate: None, comparator: None, field: Some("updated_at"), data_type: Some(crate::pages::components::datatable::core::column::DataType::DateTime), },
        ColumnDef { id: "updated_at", header_name: "Updated", value_getter: Some(Arc::new(|p: &crate::pages::rest::post_type_info_api::PostTypeInfoTO| Value::Text(p.updated_at.to_string()))), value_formatter: None, cell_renderer: None, cell_editor: None, sortable: true, filterable: false, resizable: true, movable: true, pinned: Pinned::None, width: 180, min_width: 140, max_width: None, groupable: false, aggregate: None, comparator: None, field: Some("updated_at"), data_type: Some(crate::pages::components::datatable::core::column::DataType::DateTime), },
    ]);

    // pagination init
    table_state.page_size.set(10);

    // a reload signal for manual refresh after creation
    let reload = RwSignal::new(0u32);

    // resource
    let posts_and_total_resource = Resource::new(
        {
            let table_state = table_state.clone();
            let reload = reload.clone();
            move || {
                let sort: Option<String> = query.with(|q| q.get("sort").map(|s| s.to_string()));
                let search: Option<String> = query.with(|q| q.get("search").map(|s| s.to_string()));
                let ps = table_state.page_size.get();
                let cp = table_state.current_page.get();
                let first_result_i32 = ((cp.saturating_sub(1) * ps) as i32).max(0);
                let max_results_i32 = ps as i32;
                let _r = reload.get();
                (first_result_i32, max_results_i32, sort, search, _r)
            }
        },
        |(first_result_i32, max_results_i32, sort, search, _r)| async move {
            use crate::pages::rest::post_type_info_api::{load_post_type_infos, count_post_type_infos};
            let (a, b) = futures::join!(
                load_post_type_infos(first_result_i32 as i64, max_results_i32, sort.clone(), search.clone(), None, None),
                count_post_type_infos(search.clone(), None, None)
            );
            match (a, b) { (Ok(items), Ok(total)) => Ok((items, total)), (Err(e), _) => Err(e), (_, Err(e)) => Err(e) }
        }
    );

    // sync URL with table query
    {
        use crate::pages::components::datatable::core::query_sync::{sync_table_query_to_url, SyncOptions};
        let nav = use_navigate();
        let st = table_state.clone();
        sync_table_query_to_url(st, move |qs| { nav(&qs, leptos_router::NavigateOptions { replace: true, ..Default::default() }); }, SyncOptions { include_sort: true, include_p_filters: false, include_a_filters: false, include_search: false, ..Default::default() });
    }

    // apply resource results
    Effect::new({
        let table_state = table_state.clone();
        move |_| {
            if let Some(Ok((items, total))) = posts_and_total_resource.get() {
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
                                <h1 class="text-3xl font-bold">Post Types</h1>
                                <NewPostTypeDialog on_created=Callback::new(move |_| reload.update(|r| *r += 1)) />
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