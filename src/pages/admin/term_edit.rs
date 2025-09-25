use crate::pages::admin::guard::AdminGuard;
use crate::pages::admin::layout::AdminSidebar;
use crate::pages::components::sidebar::SidebarProvider;
use crate::pages::components::{Input, Button, Select};
use crate::pages::rest::term_api::{load_term_by_id, update_term, TermTO};
use crate::pages::rest::post_taxonomy_api::{load_post_taxonomy_infos, PostTaxonomyInfoTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_params_map;

#[component]
pub fn AdminTermEditPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|v| v.parse::<i32>().ok())).unwrap_or(0);

    let entity_res = Resource::new(move || id(), |id| async move { load_term_by_id(id).await });
    let taxonomies_res = Resource::new(|| (), |_| async move {
        load_post_taxonomy_infos(Some(0), Some(100), Some("name".to_string()), None, None).await
    });

    let taxonomy_id = RwSignal::new(0);
    let slug = RwSignal::new(String::new());
    let name = RwSignal::new(String::new());

    Effect::new({
        let entity_res = entity_res.clone();
        move |_| {
            if let Some(Ok(e)) = entity_res.get() {
                taxonomy_id.set(e.taxonomy_id);
                slug.set(e.slug.clone());
                name.set(e.name.clone());
            }
        }
    });

    let save_action = Action::new(move |(tx, s, n): &(i32, String, String)| {
        let mut current = entity_res.get().and_then(|r| r.ok()).unwrap_or(TermTO{
            id: id(),
            uid: String::new(),
            version: 0,
            created_at: time::OffsetDateTime::UNIX_EPOCH,
            updated_at: time::OffsetDateTime::UNIX_EPOCH,
            taxonomy_id: 0,
            slug: String::new(),
            name: String::new(),
            parent_id: None,
            description: None,
        });
        current.taxonomy_id = *tx;
        current.slug = s.clone();
        current.name = n.clone();
        async move { update_term(current).await }
    });

    view! {
        <AdminGuard>
            <SidebarProvider default_open=true>
                <div class="flex gap-0">
                    <AdminSidebar />
                    <main class="flex-1 min-h-screen">
                        <div class="container-page py-10 font-serif">
                            <h1 class="text-3xl font-bold mb-6">"Edit Term"</h1>
                            <Suspense fallback=move || view!{<div>"Loading..."</div>}>
                                {move || match entity_res.get() {
                                    Some(Ok(_e)) => view!{
                                        <div class="grid gap-3 max-w-xl">
                                            <Select value=Signal::derive({ let taxonomy_id=taxonomy_id.clone(); move || taxonomy_id.get().to_string() }) on_change=Callback::new(move |ev| taxonomy_id.set(event_target_value(&ev).parse().unwrap_or_default()))>
                                                <Suspense fallback=move || view!{<option value="0">"Loading..."</option>}>
                                                    {move || match taxonomies_res.get() {
                                                        Some(Ok(items)) => items.into_iter().map(|it: PostTaxonomyInfoTO| view!{ <option value={it.id.to_string()}>{format!("{} ({})", it.name, it.code)}</option> }).collect_view().into_any(),
                                                        _ => view!{<></>}.into_any(),
                                                    }}
                                                </Suspense>
                                            </Select>
                                            <Input placeholder="Slug" value=slug on_input=Callback::new(move |ev| slug.set(event_target_value(&ev))) />
                                            <Input placeholder="Name" value=name on_input=Callback::new(move |ev| name.set(event_target_value(&ev))) />
                                            <div>
                                                <Button on_click=Callback::new({ let taxonomy_id=taxonomy_id.clone(); let slug=slug.clone(); let name=name.clone(); let save=save_action.clone(); move |_| { let _= save.dispatch((taxonomy_id.get_untracked(), slug.get_untracked(), name.get_untracked())); } }) loading_signal=Signal::derive(move || save_action.pending().get())>
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
