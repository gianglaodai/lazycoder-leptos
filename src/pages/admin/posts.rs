use crate::pages::admin::guard::AdminGuard;
use crate::pages::components::Pagination;
use crate::pages::components::Button;
use crate::pages::components::button::ButtonVariant;
use crate::pages::rest::post_api::{
    count_posts, delete_post, load_posts, update_post, PostTO,
};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_query_map;
use time::format_description;

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
                .unwrap_or(10)
        })
    };

    let reload = RwSignal::new(0u32);

    let posts_resource = Resource::new(
        move || (first_result(), max_results(), reload.get()),
        |(first_result, max_results, _)| async move { load_posts(first_result, max_results).await },
    );

    let total_posts_resource = Resource::new(
        move || (first_result(), max_results(), reload.get()),
        |(_, _, _)| async { count_posts().await },
    );

    view! {
        <AdminGuard>
            <div class="container-page py-10 font-serif">
                <div class="flex items-center justify-between mb-6">
                    <h1 class="text-3xl font-bold">Manage Posts</h1>
                    <Button href="/admin/posts/new">New Post</Button>
                </div>

                <Suspense fallback=move || view! {<div class="text-center py-8">Loading posts...</div>}>
                    {move || match posts_resource.get() {
                        Some(Ok(posts)) => view! {
                            <div class="space-y-4">
                                {posts.into_iter()
                                    .map(|post| view! { <AdminPostItem post=post reload=reload/> })
                                    .collect_view()
                                }
                            </div>
                        }.into_any(),
                        Some(Err(e)) => view! {
                            <div class="text-red-600">Error loading posts: {e.to_string()}</div>
                        }.into_any(),
                        None => view! {<div class="text-center py-8">Loading...</div>}.into_any()
                    }}
                </Suspense>

                <div class="mt-6">
                    <Suspense fallback=move || view! {<div>Loading total...</div>}>
                        {move || match total_posts_resource.get() {
                            Some(Ok(total_posts)) => view! {
                                <Pagination
                                    first_result=first_result()
                                    total_entities=total_posts
                                    max_results=10
                                    max_visible_pages=5
                                />
                            }.into_any(),
                            Some(Err(_)) => view!{<div>Error loading total</div>}.into_any(),
                            None => view!{<div>Loading...</div>}.into_any(),
                        }}
                    </Suspense>
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

    // Prepare values needed for display and actions before any potential moves
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
                        <Button href=format!("/admin/posts/{}/edit", post_id) variant=ButtonVariant::Outline>Edit</Button>
                        <Button variant=ButtonVariant::Destructive on_click=Callback::new(move |_| { delete_action.dispatch(()); })>Delete</Button>
                    </div>
                </div>
            </div>
        </AdminGuard>
    }
}
