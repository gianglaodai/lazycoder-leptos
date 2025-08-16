use crate::pages::admin::guard::AdminGuard;
use crate::pages::components::MarkdownEditor;
use crate::pages::components::Button;
use crate::pages::components::button::ButtonVariant;
use crate::pages::rest::auth_api::UserTO;
use crate::pages::rest::post_api::{create_post, load_post_by_id, update_post, PostCreateTO, PostTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::{use_navigate, use_params_map};
use time::format_description;

#[derive(Clone, Debug)]
pub struct PostFormValues {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: String,
}

#[component]
pub fn AdminPostForm(
    heading: String,
    show_slug: bool,
    initial_slug: String,
    initial_title: String,
    initial_summary: String,
    initial_content: String,
    initial_status: String,
    meta: Option<String>,
    submit_label: String,
    cancel_href: String,
    on_submit: Callback<PostFormValues, ()>,
) -> impl IntoView {
    let slug = RwSignal::new(initial_slug);
    let title = RwSignal::new(initial_title);
    let summary = RwSignal::new(initial_summary);
    let content = RwSignal::new(initial_content);
    let status = RwSignal::new(initial_status);

    let submit = move || {
        on_submit.run(PostFormValues {
            slug: slug.get_untracked(),
            title: title.get_untracked(),
            summary: summary.get_untracked(),
            content: content.get_untracked(),
            status: status.get_untracked(),
        })
    };

    view! {
        <div class="container-page py-10 font-serif">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-3xl font-bold">{heading.clone()}</h1>
            </div>
            <div class="p-4 bg-white rounded-lg border border-stone-200">
                {move || meta.clone().map(|m| view!{ <div class="text-sm text-stone-500 mb-3">{m}</div> }.into_any()).unwrap_or_else(|| view!{<div/>}.into_any())}
                <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                    {move || if show_slug { view!{
                        <input class="input" placeholder="Slug" prop:value=move || slug.get() on:input=move |ev| slug.set(event_target_value(&ev)) />
                    }.into_any() } else { view!{<div/>}.into_any() }}
                    <input class="input" placeholder="Title" prop:value=move || title.get() on:input=move |ev| title.set(event_target_value(&ev)) />
                    <input class="input md:col-span-2" placeholder="Summary" prop:value=move || summary.get() on:input=move |ev| summary.set(event_target_value(&ev)) />
                    <div class="md:col-span-2">
                        <MarkdownEditor
                            initial_content=content.get()
                            on_submit=Callback::new({
                                let content = content.clone();
                                move |md: String| content.set(md)
                            })
                        />
                    </div>
                    <select class="input" prop:value=move || status.get() on:change=move |ev| status.set(event_target_value(&ev))>
                        <option value="DRAFT">DRAFT</option>
                        <option value="REVIEW">REVIEW</option>
                        <option value="PUBLISHED">PUBLISHED</option>
                        <option value="ARCHIVED">ARCHIVED</option>
                        <option value="DELETED">DELETED</option>
                    </select>
                    <div class="flex gap-2">
                        <Button href=cancel_href.clone() variant=ButtonVariant::Outline>Cancel</Button>
                        <Button on_click=Callback::new(move |_| submit())>{submit_label.clone()}</Button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn AdminPostNewPage() -> impl IntoView {
    let user_ctx: RwSignal<Option<UserTO>> = expect_context();
    let navigate = use_navigate();

    let submit = Action::new(move |vals: &PostFormValues| {
        let payload = PostCreateTO {
            slug: vals.slug.clone(),
            title: vals.title.clone(),
            summary: vals.summary.clone(),
            content: vals.content.clone(),
            status: vals.status.clone(),
            user_id: user_ctx.get().map(|u| u.id).unwrap(),
        };
        async move { create_post(payload).await }
    });

    Effect::new(move |_| {
        if let Some(Ok(_)) = submit.value().get() {
            navigate("/admin/posts", Default::default());
        }
    });

    view! {
        <AdminGuard>
            <AdminPostForm
                heading="New Post".to_string()
                show_slug=true
                initial_slug="".to_string()
                initial_title="".to_string()
                initial_summary="".to_string()
                initial_content="".to_string()
                initial_status="DRAFT".to_string()
                meta=None
                submit_label="Create".to_string()
                cancel_href="/admin/posts".to_string()
                on_submit=Callback::new({
                    let submit = submit.clone();
                    move |vals: PostFormValues| {
                        let _ = submit.dispatch(vals);
                    }
                })
            />
        </AdminGuard>
    }
}

#[component]
pub fn AdminPostEditPage() -> impl IntoView {
    let params = use_params_map();

    let id = move || {
        params
            .with(|p| p.get("id").and_then(|v| v.parse::<i32>().ok()))
            .unwrap_or(0)
    };

    let post_res = Resource::new(move || id(), |id| async move { load_post_by_id(id).await });

    let update_action = Action::new(move |vals: &PostFormValues| {
        let mut to = match post_res.get() {
            Some(Ok(p)) => p.clone(),
            _ => PostTO {
                id: id(),
                uid: String::new(),
                version: 0,
                created_at: time::OffsetDateTime::UNIX_EPOCH,
                updated_at: time::OffsetDateTime::UNIX_EPOCH,
                slug: String::new(),
                title: String::new(),
                summary: String::new(),
                content: String::new(),
                status: String::new(),
                user_id: 0,
            },
        };
        to.title = vals.title.clone();
        to.summary = vals.summary.clone();
        to.content = vals.content.clone();
        to.status = vals.status.clone();
        async move { update_post(to).await }
    });

    view! {
        <AdminGuard>
            <Suspense fallback=move || view!{<div class="text-center py-8">Loading post...</div>}>
                {move || match post_res.get() {
                    Some(Ok(post)) => {
                        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
                        let meta = format!(
                            "Slug: {} - Status: {} - Created: {} - Updated: {}",
                            post.slug,
                            post.status,
                            post.created_at.format(&format).unwrap_or_default(),
                            post.updated_at.format(&format).unwrap_or_default()
                        );
                        view!{
                            <AdminPostForm
                                heading="Edit Post".to_string()
                                show_slug=false
                                initial_slug=post.slug.clone()
                                initial_title=post.title.clone()
                                initial_summary=post.summary.clone()
                                initial_content=post.content.clone()
                                initial_status=post.status.clone()
                                meta=Some(meta)
                                submit_label="Save".to_string()
                                cancel_href="/admin/posts".to_string()
                                on_submit=Callback::new({
                                    let update_action = update_action.clone();
                                    move |vals: PostFormValues| {
                                        let _ = update_action.dispatch(vals);
                                    }
                                })
                            />
                        }.into_any()
                    }
                    Some(Err(e)) => view!{<div class="text-red-600">Error: {e.to_string()}</div>}.into_any(),
                    None => view!{<div/>}.into_any()
                }}
            </Suspense>
        </AdminGuard>
    }
}
