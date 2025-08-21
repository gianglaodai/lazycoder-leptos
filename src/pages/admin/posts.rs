use crate::pages::admin::guard::AdminGuard;
use crate::pages::components::button::ButtonVariant;
use crate::pages::components::{
    Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, DialogTrigger, Input, Pagination, PaginationContent, PaginationEllipsis,
    PaginationItem, PaginationLink, PaginationNext, PaginationPrevious, Table, TableBody,
    TableCaption, TableCell, TableHead, TableHeader, TableRow,
};
use crate::pages::rest::auth_api::UserTO;
use crate::pages::rest::post_api::{create_post, delete_post, update_post, PostTO};
use crate::pages::rest::post_info_api::{count_post_infos, load_post_infos};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::{use_navigate, use_query_map};
use time::format_description;
use tokio::join;

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

    // Navigate on success
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
                    <button
                        class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 disabled:pointer-events-none disabled:opacity-50 h-9 px-4 py-2 bg-primary text-primary-foreground hover:bg-primary/90"
                        disabled=Signal::derive(move || disabled.get())
                        on:click=move |_| { if !disabled.get() { create_action.dispatch(title.get()); } }
                    >
                        {move || if create_action.pending().get() { "Creating...".to_string() } else { "Create".to_string() }}
                    </button>
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
                .unwrap_or(10)
        })
    };

    let reload = RwSignal::new(0u32);

    let posts_and_total_resource = Resource::new(
        move || (first_result(), max_results(), reload.get()),
        |(first_result, max_results, _)| async move {
            let fut_posts = load_post_infos(first_result, max_results);
            let fut_count = count_post_infos();
            let (posts_res, total_res) = join!(fut_posts, fut_count);
            match (posts_res, total_res) {
                (Ok(posts), Ok(total)) => Ok((posts, total)),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        },
    );

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
                            <Table>
                                <TableHeader>
                                    <TableRow>
                                        <TableHead>Title</TableHead>
                                        <TableHead>Slug</TableHead>
                                        <TableHead>Status</TableHead>
                                        <TableHead>Created</TableHead>
                                        <TableHead>Updated</TableHead>
                                        <TableHead class="text-right">Actions</TableHead>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                    {posts.into_iter().map(|post| {
                                        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
                                        let created = post.created_at.format(&format).unwrap_or_else(|_| "".to_string());
                                        let updated = post.updated_at.format(&format).unwrap_or_else(|_| "".to_string());
                                        let post_id = post.id;
                                        let slug = post.slug.clone();
                                        let title = post.title.clone();
                                        let status = post.status.clone();
                                        view! {
                                            <TableRow>
                                                <TableCell>{title}</TableCell>
                                                <TableCell class="font-mono text-xs">{slug}</TableCell>
                                                <TableCell>{status}</TableCell>
                                                <TableCell class="whitespace-nowrap text-xs text-stone-600">{created}</TableCell>
                                                <TableCell class="whitespace-nowrap text-xs text-stone-600">{updated}</TableCell>
                                                <TableCell class="text-right">
                                                    <Button href=format!("/admin/posts/{}/edit", post_id) variant=ButtonVariant::Outline>Edit</Button>
                                                </TableCell>
                                            </TableRow>
                                        }
                                    }).collect_view()}
                                </TableBody>
                                <TableCaption>A list of your posts.</TableCaption>
                            </Table>
                            <div class="mt-6">
                                <Pagination>
                                    <PaginationContent>
                                        <PaginationItem>
                                            {
                                                let fr = first_result();
                                             let max = max_results();
                                             let max_i64 = max as i64;
                                             let total_i64 = total as i64;
                                             let total_pages = if max_i64 <= 0 { 1 } else { ((total_i64 + max_i64 - 1) / max_i64).max(1) };
                                             let current_page = if max_i64 > 0 { fr / max_i64 } else { 0 };
                                             let prev_fr = if current_page > 0 { Some(fr - max_i64) } else { None };
                                             match prev_fr {
                                                Some(pf) => view! { <PaginationPrevious href=format!("?first_result={}&max_results={}", pf, max) /> }.into_any(),
                                                None => view! { <PaginationPrevious /> }.into_any(),
                                             }
                                            }
                                        </PaginationItem>

                                        {let fr = first_result();
                                         let max = max_results();
                                         let max_i64 = max as i64;
                                         let total_i64 = total as i64;
                                         let total_pages = if max_i64 <= 0 { 1 } else { ((total_i64 + max_i64 - 1) / max_i64).max(1) };
                                         let current_page = if max_i64 > 0 { fr / max_i64 } else { 0 };
                                         let mut v: Vec<leptos::prelude::AnyView> = Vec::new();
                                         if total_pages <= 7 {
                                             for i in 0..total_pages {
                                                 let is_active = i == current_page;
                                                 let first = i * max_i64;
                                                 v.push(view! {
                                                     <PaginationItem>
                                                         <PaginationLink is_active=is_active href=format!("?first_result={}&max_results={}", first, max)>
                                                             { (i + 1).to_string() }
                                                         </PaginationLink>
                                                     </PaginationItem>
                                                 }.into_any());
                                             }
                                         } else {
                                             // First page
                                             v.push(view! {
                                                 <PaginationItem>
                                                     <PaginationLink is_active={current_page==0} href=format!("?first_result={}&max_results={}", 0, max)>
                                                         { "1" }
                                                     </PaginationLink>
                                                 </PaginationItem>
                                             }.into_any());
                                             // Left ellipsis
                                             if current_page > 2 { v.push(view! { <PaginationEllipsis /> }.into_any()); }
                                             // Middle pages: current-1, current, current+1
                                             let set: std::collections::BTreeSet<i64> = [current_page.saturating_sub(1), current_page, (current_page+1).min(total_pages-1)].into_iter().collect();
                                             for i in set {
                                                 if i > 0 && i < total_pages-1 {
                                                     let first = i * max_i64;
                                                     let is_active = i == current_page;
                                                     v.push(view! {
                                                         <PaginationItem>
                                                             <PaginationLink is_active=is_active href=format!("?first_result={}&max_results={}", first, max)>
                                                                 { (i + 1).to_string() }
                                                             </PaginationLink>
                                                         </PaginationItem>
                                                     }.into_any());
                                                 }
                                             }
                                             // Right ellipsis
                                             if current_page + 3 < total_pages { v.push(view! { <PaginationEllipsis /> }.into_any()); }
                                             // Last page
                                             v.push(view! {
                                                 <PaginationItem>
                                                     <PaginationLink is_active={current_page==total_pages-1} href=format!("?first_result={}&max_results={}", (total_pages-1)*max_i64, max)>
                                                         { total_pages.to_string() }
                                                     </PaginationLink>
                                                 </PaginationItem>
                                             }.into_any());
                                         }
                                         view! { {v} }.into_any()
                                        }

                                        <PaginationItem>
                                            {let fr = first_result();
                                             let max = max_results();
                                             let max_i64 = max as i64;
                                             let total_i64 = total as i64;
                                             let total_pages = if max_i64 <= 0 { 1 } else { ((total_i64 + max_i64 - 1) / max_i64).max(1) };
                                             let current_page = if max_i64 > 0 { fr / max_i64 } else { 0 };
                                             let next_fr = if (current_page + 1) < total_pages { Some(fr + max_i64) } else { None };
                                             match next_fr {
                                                Some(nf) => view! { <PaginationNext href=format!("?first_result={}&max_results={}", nf, max) /> }.into_any(),
                                                None => view! { <PaginationNext /> }.into_any(),
                                             }
                                            }
                                        </PaginationItem>
                                    </PaginationContent>
                                </Pagination>
                            </div>
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
                        <Button href=format!("/admin/posts/{}/edit", post_id) variant=ButtonVariant::Outline>Edit</Button>
                        <Button variant=ButtonVariant::Destructive on_click=Callback::new(move |_| { delete_action.dispatch(()); })>Delete</Button>
                    </div>
                </div>
            </div>
        </AdminGuard>
    }
}
