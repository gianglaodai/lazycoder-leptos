use leptos::prelude::ServerFnError;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostTO {
    pub id: Option<i32>,
    pub uid: Option<String>,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: String,
}
#[server]
pub async fn load_posts() -> Result<Vec<PostTO>, ServerFnError> {
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    let result = state
        .post_service
        .get_all(vec![])
        .await
        .map(|posts| {
            posts
                .into_iter()
                .map(|post| PostTO {
                    id: post.id,
                    uid: post.uid.map(|u| u.to_string()),
                    created_at: post.created_at,
                    updated_at: post.updated_at,
                    slug: post.slug,
                    title: post.title,
                    summary: post.summary,
                    content: post.content,
                    status: post.status.as_str().to_string(),
                })
                .collect::<Vec<PostTO>>()
        })
        .map_err(|e| ServerFnError::ServerError(e.to_string()));
    match &result {
        Ok(v) => log::info!("{:?}", v),
        Err(e) => log::error!("{:?}", e),
    };
    result
}

#[server]
async fn load_count_posts() -> Result<i64, ServerFnError> {
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    state
        .post_service
        .count(vec![])
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn ArticlesPage() -> impl IntoView {
    let (page, _) = signal(1);
    let posts_resource =
        Resource::new(move || page.get(), |page| async move { load_posts().await });
    let total_posts_resource = Resource::new(move || (), |_| async { load_count_posts().await });
    view! {
        <div>Article hello</div>
        <Suspense fallback=move || view! {<div>Loading posts...</div>}>
            {move || match posts_resource.get(){
                Some(Ok(posts)) => view! {
                    {posts.into_iter().map(|post| view! {<Article post=post/>}).collect_view().into_any()}
                },
                Some(Err(e)) => view! {<div>Error loading posts {e.to_string()}</div>}.into_any(),
                None => view! {<div>Loading...</div>}.into_any()}
            }
        </Suspense>

        <div>
        <Suspense fallback=move || view! {<div>Loading total...</div>}>
            {move || match total_posts_resource.get() {
                    Some(Ok(total_posts)) => view! {<div>"Total: " {total_posts}</div>}.into_any(),
                    Some(Err(_e)) => view!{<div>Error loading total</div>}.into_any(),
                    None => view!{<div>Loading...</div>}.into_any()}
            }
        </Suspense>
        </div>
    }
}

#[component]
fn Article(post: PostTO) -> impl IntoView {
    view! {
        <div>{post.title}</div>
        <div>{post.summary}</div>
    }
}
