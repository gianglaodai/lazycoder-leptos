use crate::pages::admin::guard::AdminGuard;
use crate::pages::admin::layout::AdminSidebar;
use crate::pages::components::sidebar::SidebarProvider;
use crate::pages::components::{Input, Button};
use crate::pages::rest::post_taxonomy_api::{load_post_taxonomy_by_id, update_post_taxonomy, PostTaxonomyTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_params_map;

#[component]
pub fn AdminPostTaxonomyEditPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|v| v.parse::<i32>().ok())).unwrap_or(0);

    let entity_res = Resource::new(move || id(), |id| async move { load_post_taxonomy_by_id(id).await });

    let code = RwSignal::new(String::new());
    let name = RwSignal::new(String::new());

    Effect::new({
        let entity_res = entity_res.clone();
        move |_| {
            if let Some(Ok(e)) = entity_res.get() {
                code.set(e.code.clone());
                name.set(e.name.clone());
            }
        }
    });

    let save_action = Action::new(move |(code_v, name_v): &(String, String)| {
        let mut current = entity_res.get().and_then(|r| r.ok()).unwrap_or(PostTaxonomyTO{
            id: id(),
            uid: String::new(),
            version: 0,
            created_at: time::OffsetDateTime::UNIX_EPOCH,
            updated_at: time::OffsetDateTime::UNIX_EPOCH,
            code: String::new(),
            name: String::new(),
        });
        current.code = code_v.clone();
        current.name = name_v.clone();
        async move { update_post_taxonomy(current).await }
    });

    view! {
        <AdminGuard>
            <SidebarProvider default_open=true>
                <div class="flex gap-0">
                    <AdminSidebar />
                    <main class="flex-1 min-h-screen">
                        <div class="container-page py-10 font-serif">
                            <h1 class="text-3xl font-bold mb-6">"Edit Taxonomy"</h1>
                            <Suspense fallback=move || view!{<div>"Loading..."</div>}>
                                {move || match entity_res.get() {
                                    Some(Ok(_e)) => view!{
                                        <div class="grid gap-3 max-w-xl">
                                            <Input placeholder="Code" value=code on_input=Callback::new(move |ev| code.set(event_target_value(&ev))) />
                                            <Input placeholder="Name" value=name on_input=Callback::new(move |ev| name.set(event_target_value(&ev))) />
                                            <div>
                                                <Button on_click=Callback::new({ let code=code.clone(); let name=name.clone(); let save=save_action.clone(); move |_| { let _= save.dispatch((code.get_untracked(), name.get_untracked())); } }) loading_signal=Signal::derive(move || save_action.pending().get())>
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
