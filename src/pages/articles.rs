use crate::pages::components::{Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationPrevious, PaginationNext, PaginationEllipsis};
use crate::pages::rest::post_api::{count_posts, load_posts, PostTO};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;
use time::format_description;
use leptos::control_flow::Show;
use crate::pages::rest::auth_api::UserTO;
use crate::pages::components::Button;
use crate::pages::components::button::ButtonVariant;

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
                        <Pagination>
                            <PaginationContent>
                                <PaginationItem>
                                    {let fr = first_result();
                                     let max = max_results();
                                     let max_i64 = max as i64;
                                     let total = total_posts as i64;
                                     let total_pages = if max_i64 <= 0 { 1 } else { ((total + max_i64 - 1) / max_i64).max(1) };
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
                                 let total = total_posts as i64;
                                 let total_pages = if max_i64 <= 0 { 1 } else { ((total + max_i64 - 1) / max_i64).max(1) };
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
                                     let total = total_posts as i64;
                                     let total_pages = if max_i64 <= 0 { 1 } else { ((total + max_i64 - 1) / max_i64).max(1) };
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
                        <Button variant=ButtonVariant::Secondary size=crate::pages::components::button::ButtonSize::Sm>Edit</Button>
                        <Button variant=ButtonVariant::Destructive size=crate::pages::components::button::ButtonSize::Sm>Delete</Button>
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
