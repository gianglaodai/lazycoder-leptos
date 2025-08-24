use crate::pages::admin::guard::AdminGuard;
use crate::pages::components::button::{ButtonIntent, ButtonVariant};
use crate::pages::components::{
    Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter,
    DialogHeader, DialogTitle, DialogTrigger, Form, FormControl, FormField,
    FormItem, FormLabel, FormMessage, Input,
};
use crate::pages::rest::auth_api::UserTO;
use crate::pages::rest::post_api::{create_post, delete_post, update_post, PostTO};
use crate::pages::rest::post_info_api::{count_post_infos, load_post_infos };
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
            <DialogTrigger>New Post</DialogTrigger>
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

    let posts_and_total_resource = Resource::new(
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
            (
                first_result() as i32,
                max_results(),
                sort,
                search,
                p_filters,
                a_filters,
                reload.get(),
            )
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

    view! {
        <AdminGuard>
            <div class="container-page py-10 font-serif">
                <div class="flex items-center justify-between mb-6">
                    <h1 class="text-3xl font-bold">Manage Posts</h1>
                    <NewPostDialog />
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
