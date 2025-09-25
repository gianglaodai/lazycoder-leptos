use crate::pages::admin::guard::AdminGuard;
use crate::pages::admin::layout::AdminSidebar;
use crate::pages::components::sidebar::SidebarProvider;
use crate::pages::components::{Input, Button, Select};
use crate::pages::rest::post_collection_api::{load_post_collection_by_id, update_post_collection, PostCollectionTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_params_map;

#[component]
pub fn AdminPostCollectionEditPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|v| v.parse::<i32>().ok())).unwrap_or(0);

    let entity_res = Resource::new(move || id(), |id| async move { load_post_collection_by_id(id).await });

    let slug = RwSignal::new(String::new());
    let title = RwSignal::new(String::new());
    let visibility = RwSignal::new(String::new());

    Effect::new({
        let entity_res = entity_res.clone();
        move |_| {
            if let Some(Ok(e)) = entity_res.get() {
                slug.set(e.slug.clone());
                title.set(e.title.clone());
                visibility.set(e.visibility.clone());
            }
        }
    });

    let save_action = Action::new(move |(s, t, v): &(String, String, String)| {
        let mut current = entity_res.get().and_then(|r| r.ok()).unwrap_or(PostCollectionTO{
            id: id(),
            uid: String::new(),
            version: 0,
            created_at: time::OffsetDateTime::UNIX_EPOCH,
            updated_at: time::OffsetDateTime::UNIX_EPOCH,
            slug: String::new(),
            title: String::new(),
            visibility: "PUBLIC".to_string(),
            description: None,
        });
        current.slug = s.clone();
        current.title = t.clone();
        current.visibility = v.clone();
        async move { update_post_collection(current).await }
    });

    view! {
        <AdminGuard>
            <SidebarProvider default_open=true>
                <div class="flex gap-0">
                    <AdminSidebar />
                    <main class="flex-1 min-h-screen">
                        <div class="container-page py-10 font-serif">
                            <h1 class="text-3xl font-bold mb-6">"Edit Post Collection"</h1>
                            <Suspense fallback=move || view!{<div>"Loading..."</div>}>
                                {move || match entity_res.get() {
                                    Some(Ok(_e)) => view!{
                                        <div class="grid gap-3 max-w-xl">
                                            <Input placeholder="Slug" value=slug on_input=Callback::new(move |ev| slug.set(event_target_value(&ev))) />
                                            <Input placeholder="Title" value=title on_input=Callback::new(move |ev| title.set(event_target_value(&ev))) />
                                            <Select value=visibility on_change=Callback::new(move |ev| visibility.set(event_target_value(&ev)))>
                                                <option value="PUBLIC">PUBLIC</option>
                                                <option value="PRIVATE">PRIVATE</option>
                                            </Select>
                                            <div>
                                                <Button on_click=Callback::new({ let slug=slug.clone(); let title=title.clone(); let visibility=visibility.clone(); let save=save_action.clone(); move |_| { let _= save.dispatch((slug.get_untracked(), title.get_untracked(), visibility.get_untracked())); } }) loading_signal=Signal::derive(move || save_action.pending().get())>
                                                    "Save"
                                                </Button>
                                            </div>
                                        </div>
                                    }.into_any(),
                                    Some(Err(e)) => view!{<div class="text-red-600">{"Error: "}{e.to_string()}</div>}.into_any(),
                                    None => view!{<div/>}.into_any(),
                                }}
                            </Suspense>
                        </div>
                    </main>
                </div>
            </SidebarProvider>
        </AdminGuard>
    }
}
