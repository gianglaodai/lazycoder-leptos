use crate::pages::components::Pagination;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;
use time::format_description;
use crate::pages::rest::post_api::{count_posts, load_posts, PostTO};

#[component]
pub fn ArticlesPage() -> impl IntoView {
    let query = use_query_map();
    let first_result = move || {
        query.with(|q| {
            q.get("first_result")
                .and_then(|p| p.parse::<i64>().ok())
                .unwrap_or(1)
        })
    };

    let posts_resource = Resource::new(first_result, |first_result| async move {
        load_posts(first_result, 5).await
    });

    let total_posts_resource = Resource::new(|| (), |_| async { count_posts().await });

    view! {
        <div class="max-w-4xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">Latest Articles</h1>

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
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let formatted_date = post
        .created_at
        .map(|dt| {
            dt.format(&format)
                .unwrap_or_else(|_| "Unknown date".to_string())
        })
        .unwrap_or_else(|| "".to_string());

    view! {
        <article class="mb-8 p-6 bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow duration-300">
            <h2 class="text-2xl font-bold text-gray-800 mb-2 hover:text-blue-600 transition-colors">
                {post.title}
            </h2>

            {if !formatted_date.is_empty() {
                view! {
                    <p class="text-sm text-gray-500 mb-4">
                        <time datetime=formatted_date.clone()>
                            {formatted_date.clone()}
                        </time>
                    </p>
                }.into_any()
            } else {
                view! {}.into_any()
            }}

            <p class="text-gray-600 mb-4">
                {post.summary}
            </p>

            <A
                href=format!("/articles/{}", post.slug)
                attr:class="inline-flex items-center text-blue-600 hover:text-blue-800 font-medium"
            >
                Read more
                <svg class="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                </svg>
            </A>
        </article>
    }
}
