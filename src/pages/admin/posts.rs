use crate::pages::admin::guard::AdminGuard;
use crate::pages::components::button::{ButtonIntent, ButtonVariant};
use crate::pages::components::{
    Button, DataTable, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter,
    DialogHeader, DialogTitle, DialogTrigger, Input,
};
use crate::pages::rest::auth_api::UserTO;
use crate::pages::rest::post_api::{create_post, delete_post, update_post, PostTO};
use crate::pages::rest::post_info_api::{count_post_infos, load_post_infos};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::{use_navigate, use_query_map};
use std::collections::HashMap;
use time::format_description;

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

    view! {
        <Dialog>
            <DialogTrigger>New Post</DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>New Post</DialogTitle>
                    <DialogDescription>Enter a title to create a new post. You can edit details afterward.</DialogDescription>
                </DialogHeader>
                <div class="space-y-2">
                    <Input placeholder="Title" value=title on_input=Callback::new(move |ev: leptos::ev::Event| title.set(event_target_value(&ev))) />
                    {move || if !error.get().is_empty() { view!{ <div class="text-sm text-red-600">{error.get()}</div> }.into_any() } else { view!{<div/>}.into_any() }}
                </div>
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

    let posts_and_total_resource = Resource::new(
        move || (first_result(), max_results(), reload.get()),
        |(first_result, max_results, _)| async move {
            let fut_posts = load_post_infos(first_result, max_results);
            let fut_count = count_post_infos();
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

    let navigate = use_navigate();
    let on_edit = Callback::new(move |row: HashMap<String, String>| {
        if let Some(id_str) = row.get("id") {
            if let Ok(id) = id_str.parse::<i32>() {
                navigate(&format!("/admin/posts/{}", id), Default::default());
            }
        }
    });
    let delete_action = delete_action.clone();
    let on_delete = Callback::new(move |row: HashMap<String, String>| {
        if let Some(id_str) = row.get("id") {
            if let Ok(id) = id_str.parse::<i32>() {
                delete_action.dispatch(id);
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

                <Suspense fallback=move || view! {<div class="text-center py-8">Loading posts...</div>}>
                    {move || match posts_and_total_resource.get() {
                        Some(Ok((posts, total))) => view! {
                            {
                                let field_definitions = vec![
                                    ("id".to_string(), "ID".to_string()),
                                    ("uid".to_string(), "UID".to_string()),
                                    ("title".to_string(), "Title".to_string()),
                                    ("slug".to_string(), "Slug".to_string()),
                                    ("status".to_string(), "Status".to_string()),
                                    ("created_at".to_string(), "Created".to_string()),
                                    ("updated_at".to_string(), "Updated".to_string()),
                                ];
                                let rows: Vec<HashMap<String, String>> = posts
                                    .into_iter()
                                    .map(|p| {
                                        p
                                        .to_field_map()
                                        .into_iter()
                                        .map(|(k, v)| (k.to_string(), v))
                                        .collect()
                                    })
                                    .collect();
                                {
                                    view! {
                                        <DataTable
                                            field_definitions=field_definitions.clone()
                                            rows=rows
                                            total_entities=total as i64
                                            first_result=first_result()
                                            max_results=max_results() as i64
                                            max_visible_pages=3
                                            editable=true
                                            deletable=true
                                            on_edit=on_edit
                                            on_delete=on_delete
                                            caption="A list of your posts.".to_string()
                                        />
                                    }
                                }
                            }
                        }.into_any(),
                        Some(Err(e)) => view! {
                            <div class="text-red-600">Error loading posts: {e.to_string()}</div>
                        }.into_any(),
                        None => view! {<div class="text-center py-8">Loading...</div>}.into_any()
                    }}
                </Suspense>
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

    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
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
