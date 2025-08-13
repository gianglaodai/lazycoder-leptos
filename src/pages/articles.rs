use crate::pages::components::Pagination;
use crate::pages::rest::post_api::{count_posts, load_posts, PostTO};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;
use time::format_description;
use leptos::control_flow::Show;
use crate::pages::rest::auth_api::UserTO;

#[component]
pub fn ArticlesPage() -> impl IntoView {
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

    let posts_resource = Resource::new(
        move || (first_result(), max_results()),
        |(first_result, max_results)| async move { load_posts(first_result, max_results).await },
    );

    let total_posts_resource = Resource::new(
        move || (first_result(), max_results()),
        |(_, _)| async { count_posts().await },
    );

    view! {
        <div class="container-page py-10">
            <h1 class="text-3xl font-serif font-semibold mb-8">Bài viết mới nhất</h1>

            <Suspense fallback=move || view! {<div class="text-center py-8">Loading posts...</div>}>
                {move || match posts_resource.get() {
                    Some(Ok(posts)) => view! {
                        <div class="space-y-6">
                            {posts.into_iter()
                                .map(|post| view! { <Article post=post/> })
                                .collect_view()
                            }

                        </div>
                    }.into_any(),
                    Some(Err(e)) => view! {
                        <div class="text-red-600">
                            Error loading posts: {e.to_string()}
                        </div>
                    }.into_any(),
                    None => view! {
                        <div class="text-center py-8">Loading...</div>
                    }.into_any()
                }}
            </Suspense>

            <div>
            <Suspense fallback=move || view! {<div>Loading total...</div>}>
                {move || match total_posts_resource.get() {
                        Some(Ok(total_posts)) => view! {
                            <Pagination
                                first_result=first_result()
                                total_entities=total_posts
                                max_results=5
                                max_visible_pages=5
                            />
                    }.into_any(),
                        Some(Err(_e)) => view!{<div>Error loading total</div>}.into_any(),
                        None => view!{<div>Loading...</div>}.into_any()}
                }
            </Suspense>
            </div>
        </div>
    }
}

#[component]
fn Article(post: PostTO) -> impl IntoView {
    let user_ctx: RwSignal<Option<UserTO>> = expect_context();
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let formatted_date = post
        .created_at
        .format(&format)
        .unwrap_or_else(|_| "Unknown date".to_string());

    view! {
        <article class="mb-8 p-6 bg-white rounded-2xl border border-stone-200 shadow-sm/20 hover:shadow transition-shadow">
            <div class="flex justify-between items-start gap-4">
                <h2 class="text-xl font-semibold mb-2 hover:text-stone-900 transition-colors">
                    {post.title}
                </h2>
                <div class="flex gap-2">
                    <Show when={move || user_ctx.get().map(|u| u.id == post.user_id).unwrap_or(false)}>
                        <button class="inline-flex items-center rounded-full bg-stone-800 text-white px-4 py-2 text-xs font-medium shadow-sm hover:bg-stone-900 transition-colors">Edit</button>
                        <button class="inline-flex items-center rounded-full bg-red-600 text-white px-4 py-2 text-xs font-medium shadow-sm hover:bg-red-700 transition-colors">Delete</button>
                    </Show>
                </div>
            </div>

            {if !formatted_date.is_empty() {
                view! {
                    <p class="text-sm text-stone-500 mb-4">
                        <time datetime=formatted_date.clone()>
                            {formatted_date.clone()}
                        </time>
                    </p>
                }.into_any()
            } else {
                view! {}.into_any()
            }}

            <p class="text-stone-700 mb-4 leading-7">
                {post.summary}
            </p>

            <A
                href=format!("/articles/{}", post.slug)
                attr:class="inline-flex items-center font-medium underline decoration-brand-300/0 hover:decoration-brand-400 underline-offset-4 text-stone-800 hover:text-stone-900"
            >
                Read more
                <svg class="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                </svg>
            </A>
        </article>
    }
}
