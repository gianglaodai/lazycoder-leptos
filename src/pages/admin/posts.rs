use crate::pages::admin::guard::AdminGuard;
use crate::pages::components::button::{ButtonIntent, ButtonVariant};
use crate::pages::components::{
    Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, DialogTrigger, Form, FormControl, FormField, FormItem, FormLabel, FormMessage,
    Input,
};
use crate::pages::rest::auth_api::UserTO;
use crate::pages::rest::post_api::{create_post, delete_post, update_post, PostTO};
use crate::pages::rest::post_info_api::{count_post_infos, load_post_infos};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::{use_navigate, use_query_map};

#[component]
fn NewPostDialog() -> impl IntoView {
    let user_ctx: RwSignal<Option<UserTO>> = expect_context();
    let navigate = use_navigate();

    let title = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());

    let create_action = Action::new(move |t: &String| {
        let title_val = t.clone();
        let user_id = user_ctx.get().map(|u| u.id).unwrap_or(0);
        async move { create_post(title_val, user_id).await }
    });

    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            if let Some(Ok(post)) = create_action.value().get() {
                error.set(String::new());
                title.set(String::new());
                navigate(&format!("/admin/posts/{}", post.id), Default::default());
            } else if let Some(Err(e)) = create_action.value().get() {
                error.set(e.to_string());
            }
        }
    });

    let disabled =
        Signal::derive(move || title.get().trim().is_empty() || create_action.pending().get());

    // Map error String -> Option<String> for FormField context
    let error_sig: Signal<Option<String>> = Signal::derive({
        let error = error.clone();
        move || {
            let e = error.get();
            if e.is_empty() {
                None
            } else {
                Some(e)
            }
        }
    });

    view! {
        <Dialog>
            <DialogTrigger variant=ButtonVariant::Primary intent=ButtonIntent::Primary>New Post</DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>New Post</DialogTitle>
                    <DialogDescription>Enter a title to create a new post. You can edit details afterward.</DialogDescription>
                </DialogHeader>
                <Form prevent_default=true on_submit=Callback::new({
                    let disabled = disabled.clone();
                    let create_action = create_action.clone();
                    let title = title.clone();
                    move |_| { if !disabled.get() { create_action.dispatch(title.get()); } }
                })>
                    <FormField name="title".to_string() error=error_sig>
                        <FormItem>
                            <FormLabel>Title</FormLabel>
                            <FormControl>
                                <Input placeholder="Title" value=title on_input=Callback::new(move |ev: leptos::ev::Event| title.set(event_target_value(&ev))) />
                            </FormControl>
                            <FormMessage>{move || error.get()}</FormMessage>
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
                        on_click=Callback::new(move |_| { if !disabled.get() { create_action.dispatch(title.get()); } })
                    >
                        {move || if create_action.pending().get() { "Creating...".to_string() } else { "Create".to_string() }}
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}

#[component]
pub fn AdminPostsPage() -> impl IntoView {
    let query = use_query_map();
    let first_result = move || {
        query.with(|q| {
            q.get("first_result")
                .and_then(|p| p.parse::<i64>().ok())
                .unwrap_or(0)
        })
    };
    let max_results = move || {
        query.with(|q| {
            q.get("max_results")
                .and_then(|p| p.parse::<i32>().ok())
                .unwrap_or(5)
        })
    };

    let delete_action = Action::new(move |post_id: &i32| {
        let id = *post_id;
        async move { delete_post(id.to_owned()).await }
    });

    let reload = RwSignal::new(0u32);

    // Build datatable state and columns
    use crate::pages::components::datatable::core::column::{ColumnDef, Pinned};
    use crate::pages::components::datatable::core::render_value::Value;
    use crate::pages::components::datatable::core::row::RowNode;
    use crate::pages::components::datatable::core::state::TableState;
    use crate::pages::components::datatable::DataTable;
    use std::sync::Arc;

    let table_state: Arc<TableState<crate::pages::rest::post_info_api::PostInfoTO>> = Arc::new(TableState::new());

    // Initialize pagination from query
    table_state.page_size.set(max_results() as usize);
    let initial_cp = {
        let ps = table_state.page_size.get_untracked().max(1) as i64;
        let fr = first_result();
        ((fr / ps) + 1).max(1) as usize
    };
    table_state.current_page.set(initial_cp);

    let posts_and_total_resource = Resource::new(
        {
            let table_state = table_state.clone();
            move || {
                let sort: Option<String> = query.with(|q| q.get("sort").map(|s| s.to_string()));
                let search: Option<String> = query.with(|q| q.get("search").map(|s| s.to_string()));
                let (p_filters, a_filters): (Option<Vec<String>>, Option<Vec<String>>) = {
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Some(w) = leptos::web_sys::window() {
                            if let Ok(search_str) = w.location().search() {
                                let qs = search_str.trim_start_matches('?');
                                let mut pfs: Vec<String> = vec![];
                                let mut afs: Vec<String> = vec![];
                                for kv in qs.split('&') {
                                    if let Some((k, v)) = kv.split_once('=') {
                                        if k == "p_filters" {
                                            pfs.push(v.to_string());
                                        }
                                        if k == "a_filters" {
                                            afs.push(v.to_string());
                                        }
                                    }
                                }
                                (
                                    if pfs.is_empty() { None } else { Some(pfs) },
                                    if afs.is_empty() { None } else { Some(afs) },
                                )
                            } else {
                                (None, None)
                            }
                        } else {
                            (None, None)
                        }
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let pfs = query.with(|q| {
                            let mut items: Vec<String> = Vec::new();
                            if let Some(v) = q.get("p_filters") {
                                items.push(v.to_string());
                            }
                            if let Some(v) = q.get("p_filters[]") {
                                items.push(v.to_string());
                            }
                            if items.is_empty() {
                                None
                            } else {
                                Some(items)
                            }
                        });
                        let afs = query.with(|q| {
                            let mut items: Vec<String> = Vec::new();
                            if let Some(v) = q.get("a_filters") {
                                items.push(v.to_string());
                            }
                            if let Some(v) = q.get("a_filters[]") {
                                items.push(v.to_string());
                            }
                            if items.is_empty() {
                                None
                            } else {
                                Some(items)
                            }
                        });
                        (pfs, afs)
                    }
                };
                let ps = table_state.page_size.get();
                let cp = table_state.current_page.get();
                let first_result_i32 = ((cp.saturating_sub(1) * ps) as i32).max(0);
                let max_results_i32 = ps as i32;
                (
                    first_result_i32,
                    max_results_i32,
                    sort,
                    search,
                    p_filters,
                    a_filters,
                    reload.get(),
                )
            }
        },
        |(first_result_i32, max_results_i32, sort, search, p_filters, a_filters, _)| async move {
            let fut_posts = load_post_infos(
                first_result_i32 as i64,
                max_results_i32,
                sort.clone(),
                search.clone(),
                p_filters.clone(),
                a_filters.clone(),
            );
            let fut_count = count_post_infos(search.clone(), p_filters.clone(), a_filters.clone());
            let (posts_res, total_res) = futures::join!(fut_posts, fut_count);
            match (posts_res, total_res) {
                (Ok(posts), Ok(total)) => Ok((posts, total)),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        },
    );

    Effect::new({
        let reload = reload.clone();
        move |_| {
            if let Some(Ok(_)) = delete_action.value().get() {
                reload.update(|v| *v += 1);
            }
        }
    });

    // Define columns (AG Grid inspired)
    table_state.columns.set(vec![
        ColumnDef {
            id: "id",
            header_name: "ID",
            value_getter: Some(Arc::new(|p: &crate::pages::rest::post_info_api::PostInfoTO| Value::Number(p.id as f64))),
            value_formatter: None,
            cell_renderer: None,
            cell_editor: None,
            sortable: true,
            filterable: true,
            resizable: true,
            movable: false,
            pinned: Pinned::None,
            width: 80,
            min_width: 60,
            max_width: Some(140),
            groupable: false,
            aggregate: None,
            comparator: None,
            field: Some("id"),
            data_type: Some(crate::pages::components::datatable::core::column::DataType::Int),
        },
        ColumnDef {
            id: "title",
            header_name: "Title",
            value_getter: Some(Arc::new(|p: &crate::pages::rest::post_info_api::PostInfoTO| Value::Text(p.title.clone()))),
            value_formatter: None,
            cell_renderer: None,
            cell_editor: None,
            sortable: true,
            filterable: true,
            resizable: true,
            movable: true,
            pinned: Pinned::None,
            width: 260,
            min_width: 120,
            max_width: None,
            groupable: false,
            aggregate: None,
            comparator: None,
            field: Some("title"),
            data_type: Some(crate::pages::components::datatable::core::column::DataType::Text),
        },
        ColumnDef {
            id: "slug",
            header_name: "Slug",
            value_getter: Some(Arc::new(|p: &crate::pages::rest::post_info_api::PostInfoTO| Value::Text(p.slug.clone()))),
            value_formatter: None,
            cell_renderer: None,
            cell_editor: None,
            sortable: true,
            filterable: true,
            resizable: true,
            movable: true,
            pinned: Pinned::None,
            width: 200,
            min_width: 120,
            max_width: None,
            groupable: false,
            aggregate: None,
            comparator: None,
            field: Some("slug"),
            data_type: Some(crate::pages::components::datatable::core::column::DataType::Text),
        },
        ColumnDef {
            id: "status",
            header_name: "Status",
            value_getter: Some(Arc::new(|p: &crate::pages::rest::post_info_api::PostInfoTO| Value::Text(p.status.clone()))),
            value_formatter: None,
            cell_renderer: None,
            cell_editor: None,
            sortable: true,
            filterable: true,
            resizable: true,
            movable: true,
            pinned: Pinned::None,
            width: 120,
            min_width: 80,
            max_width: Some(200),
            groupable: false,
            aggregate: None,
            comparator: None,
            field: Some("status"),
            data_type: Some(crate::pages::components::datatable::core::column::DataType::Text),
        },
        ColumnDef {
            id: "username",
            header_name: "Author",
            value_getter: Some(Arc::new(|p: &crate::pages::rest::post_info_api::PostInfoTO| Value::Text(p.username.clone()))),
            value_formatter: None,
            cell_renderer: None,
            cell_editor: None,
            sortable: true,
            filterable: true,
            resizable: true,
            movable: true,
            pinned: Pinned::None,
            width: 160,
            min_width: 100,
            max_width: None,
            groupable: false,
            aggregate: None,
            comparator: None,
            field: Some("username"),
            data_type: Some(crate::pages::components::datatable::core::column::DataType::Text),
        },
        ColumnDef {
            id: "email",
            header_name: "Email",
            value_getter: Some(Arc::new(|p: &crate::pages::rest::post_info_api::PostInfoTO| Value::Text(p.email.clone()))),
            value_formatter: None,
            cell_renderer: None,
            cell_editor: None,
            sortable: true,
            filterable: true,
            resizable: true,
            movable: true,
            pinned: Pinned::None,
            width: 220,
            min_width: 140,
            max_width: None,
            groupable: false,
            aggregate: None,
            comparator: None,
            field: Some("email"),
            data_type: Some(crate::pages::components::datatable::core::column::DataType::Text),
        },
        ColumnDef {
            id: "created_at",
            header_name: "Created",
            value_getter: Some(Arc::new(|p: &crate::pages::rest::post_info_api::PostInfoTO| Value::Text(p.created_at.to_string()))),
            value_formatter: None,
            cell_renderer: None,
            cell_editor: None,
            sortable: true,
            filterable: true,
            resizable: true,
            movable: true,
            pinned: Pinned::None,
            width: 180,
            min_width: 140,
            max_width: None,
            groupable: false,
            aggregate: None,
            comparator: None,
            field: Some("created_at"),
            data_type: Some(crate::pages::components::datatable::core::column::DataType::DateTime),
        },
        ColumnDef {
            id: "updated_at",
            header_name: "Updated",
            value_getter: Some(Arc::new(|p: &crate::pages::rest::post_info_api::PostInfoTO| Value::Text(p.updated_at.to_string()))),
            value_formatter: None,
            cell_renderer: None,
            cell_editor: None,
            sortable: true,
            filterable: false,
            resizable: true,
            movable: true,
            pinned: Pinned::None,
            width: 180,
            min_width: 140,
            max_width: None,
            groupable: false,
            aggregate: None,
            comparator: None,
            field: Some("updated_at"),
            data_type: Some(crate::pages::components::datatable::core::column::DataType::DateTime),
        }
    ]);

    // When resource resolves, populate rows and total
    Effect::new({
        let table_state = table_state.clone();
        move |_| {
            if let Some(Ok((posts, total))) = posts_and_total_resource.get() {
                let rows: Vec<RowNode<_>> = posts.into_iter().map(|p| RowNode::new(p.id.to_string(), p)).collect();
                table_state.set_rows(rows);
                table_state.set_total_rows(Some(total as usize));
            }
        }
    });

    view! {
        <AdminGuard>
            <div class="container-page py-10 font-serif">
                <div class="flex items-center justify-between mb-6">
                    <h1 class="text-3xl font-bold">Manage Posts</h1>
                    <NewPostDialog />
                </div>
                <div class="mt-4">
                    <DataTable state=table_state.clone() height="600px".to_string() row_height=36 />
                </div>
            </div>
        </AdminGuard>
    }
}

#[component]
fn AdminPostItem(post: PostTO, reload: RwSignal<u32>) -> impl IntoView {
    let editing = RwSignal::new(false);

    let title = RwSignal::new(post.title.clone());
    let summary = RwSignal::new(post.summary.clone());
    let content = RwSignal::new(post.content.clone());
    let status = RwSignal::new(post.status.clone());

    let format = time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let created = post
        .created_at
        .format(&format)
        .unwrap_or_else(|_| "".to_string());
    let updated = post
        .updated_at
        .format(&format)
        .unwrap_or_else(|_| "".to_string());
    let slug = post.slug.clone();
    let post_id = post.id;
    let post_for_update = post.clone();

    let update_action = Action::new(move |_: &()| {
        let mut to = post_for_update.clone();
        to.title = title.get_untracked();
        to.summary = summary.get_untracked();
        to.content = content.get_untracked();
        to.status = status.get_untracked();
        async move { update_post(to).await }
    });

    let delete_action = Action::new(move |_: &()| async move { delete_post(post_id).await });

    Effect::new({
        let reload = reload.clone();
        move |_| {
            if let Some(Ok(_)) = update_action.value().get() {
                editing.set(false);
                reload.update(|v| *v += 1);
            }
            if let Some(Ok(_)) = delete_action.value().get() {
                reload.update(|v| *v += 1);
            }
        }
    });

    view! {
        <AdminGuard>
            <div class="p-4 bg-white rounded-lg border border-stone-200">
                <div class="flex justify-between items-start">
                    <div>
                        <div class="text-lg font-semibold">{move || title.get()}</div>
                        <div class="text-sm text-stone-500">Slug: {slug.clone()} - Status: {move || status.get()} - Created: {created.clone()} - Updated: {updated.clone()}</div>
                    </div>
                    <div class="flex gap-2">
                        <Button href=format!("/admin/posts/{}", post_id) variant=ButtonVariant::Outline intent=ButtonIntent::Primary>Edit</Button>
                        <Button variant=ButtonVariant::Outline intent=ButtonIntent::Destructive on_click=Callback::new(move |_| { delete_action.dispatch(()); })>Delete</Button>
                    </div>
                </div>
            </div>
        </AdminGuard>
    }
}
