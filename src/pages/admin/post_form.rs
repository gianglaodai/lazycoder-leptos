use crate::pages::admin::guard::AdminGuard;
use crate::pages::components::Input;
use crate::pages::components::MarkdownEditor;
use crate::pages::components::Select;
use crate::pages::rest::post_api::{load_post_by_id, update_post, PostTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use time::format_description;
use crate::pages::components::sidebar::SidebarProvider;
use crate::pages::admin::layout::AdminSidebar;

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
    saved_post: Signal<Option<PostTO>>,
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

    // Sync local fields with saved data from server after update
    Effect::new(move |_| {
        if let Some(p) = saved_post.get() {
            slug.set(p.slug.clone());
            title.set(p.title.clone());
            summary.set(p.summary.clone());
            content.set(p.content.clone());
            status.set(p.status.clone());
        }
    });

    view! {
        <div class="container-page py-10 font-serif">
            <div class="flex items-center justify-between mb-6">
                <div class="flex items-center gap-3">
                    <A href="/admin/posts" attr:class="text-sm text-stone-500 hover:text-stone-700">Back</A>
                    <h1 class="text-3xl font-bold">{heading.clone()}</h1>
                </div>
            </div>
            <div class="p-4 bg-white rounded-lg border border-stone-200">
                {move || meta.clone().map(|m| view!{ <div class="text-sm text-stone-500 mb-3">{m}</div> }.into_any()).unwrap_or_else(|| view!{<div/>}.into_any())}
                <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                    {move || if show_slug { view!{
                        <Input placeholder="Slug" value=slug on_input=Callback::new(move |ev: leptos::ev::Event| slug.set(event_target_value(&ev))) on_blur=Callback::new(move |_| submit()) />
                    }.into_any() } else { view!{}.into_any() }}
                    <Input placeholder="Title" value=title on_input=Callback::new(move |ev: leptos::ev::Event| title.set(event_target_value(&ev))) on_blur=Callback::new(move |_| submit()) />
                    <Input class="md:col-span-2" placeholder="Summary" value=summary on_input=Callback::new(move |ev: leptos::ev::Event| summary.set(event_target_value(&ev))) on_blur=Callback::new(move |_| submit()) />
                    <Select value=status on_change=Callback::new(move |ev: leptos::ev::Event| { status.set(event_target_value(&ev)); submit(); })>
                        <option value="DRAFT">DRAFT</option>
                        <option value="REVIEW">REVIEW</option>
                        <option value="PUBLISHED">PUBLISHED</option>
                        <option value="ARCHIVED">ARCHIVED</option>
                        <option value="DELETED">DELETED</option>
                    </Select>
                    <div class="md:col-span-2">
                        <MarkdownEditor
                            initial_content=content.get()
                            on_submit=Callback::new({
                                let content = content.clone();
                                let submit_cb = submit.clone();
                                move |md: String| {
                                    content.set(md);
                                    submit_cb();
                                }
                            })
                        />
                    </div>
                </div>
            </div>
        </div>
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
            <SidebarProvider default_open=true>
                <div class="flex gap-0">
                    <AdminSidebar />
                    <main class="flex-1 min-h-screen">
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
                                saved_post=Signal::derive({
                                    let v = update_action.value();
                                    move || v.get().and_then(|r| r.ok())
                                })
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
                    </main>
                </div>
            </SidebarProvider>
        </AdminGuard>
    }
}
