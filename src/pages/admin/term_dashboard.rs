use crate::pages::admin::guard::AdminGuard;
use crate::pages::admin::layout::AdminSidebar;
use crate::pages::components::sidebar::SidebarProvider;
use crate::pages::components::{Button, Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger, Form, FormControl, FormField, FormItem, FormLabel, FormMessage, Input, Select};
use crate::pages::rest::term_api::{load_term_infos, create_term, TermInfoTO};
use crate::pages::rest::post_taxonomy_api::{load_post_taxonomy_infos, PostTaxonomyInfoTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn AdminTermsDashboardPage() -> impl IntoView {
    let infos = Resource::new(|| (), |_| async move {
        load_term_infos(Some(0), Some(100), Some("name".to_string()), None, None).await
    });

    // Create dialog state
    let taxonomy_id = RwSignal::new(None::<i32>);
    let slug = RwSignal::new(String::new());
    let name = RwSignal::new(String::new());
    let create_action = Action::new(|(tx, s, n): &(i32, String, String)| {
        let tx = *tx;
        let s = s.clone();
        let n = n.clone();
        async move { create_term(tx, s, n).await }
    });

    // Taxonomy options
    let taxonomies_res = Resource::new(|| (), |_| async move {
        load_post_taxonomy_infos(Some(0), Some(100), Some("name".to_string()), None, None).await
    });

    // Refresh list after create
    Effect::new({
        let infos = infos.clone();
        move |_| {
            if let Some(Ok(_)) = create_action.value().get() {
                taxonomy_id.set(None);
                slug.set(String::new());
                name.set(String::new());
                infos.refetch();
            }
        }
    });

    view! {
        <AdminGuard>
            <SidebarProvider default_open=true>
                <div class="flex gap-0">
                    <AdminSidebar />
                    <main class="flex-1 min-h-screen">
                        <div class="container-page py-10 font-serif">
                            <div class="flex items-center justify-between mb-6">
                                <h1 class="text-3xl font-bold">"Terms"</h1>
                                <Dialog>
                                    <DialogTrigger>
                                        <Button intent=crate::pages::components::button::ButtonIntent::Primary>"New Term"</Button>
                                    </DialogTrigger>
                                    <DialogContent>
                                        <DialogHeader>
                                            <DialogTitle>"Create Term"</DialogTitle>
                                        </DialogHeader>
                                        <Form on_submit=Callback::new({
                                            let taxonomy_id = taxonomy_id.clone();
                                            let slug = slug.clone();
                                            let name = name.clone();
                                            let create_action = create_action.clone();
                                            move |_| {
                                                if let Some(tx) = taxonomy_id.get_untracked() {
                                                    let _ = create_action.dispatch((tx, slug.get_untracked(), name.get_untracked()));
                                                }
                                            }
                                        })>
                                            <FormItem>
                                                <FormLabel>"Taxonomy"</FormLabel>
                                                <FormControl>
                                                    <Select value=Signal::derive({ let taxonomy_id=taxonomy_id.clone(); move || taxonomy_id.get().map(|v| v.to_string()).unwrap_or_default() }) on_change=Callback::new(move |ev| taxonomy_id.set(event_target_value(&ev).parse().ok()))>
                                                        <option value="">-- select taxonomy --</option>
                                                        <Suspense fallback=move || view!{<option value="">"Loading..."</option>}>
                                                            {move || {
                                                                let mut opts = Vec::new();
                                                                if let Some(Ok(items)) = taxonomies_res.get() {
                                                                    opts = items
                                                                        .into_iter()
                                                                        .map(|it: PostTaxonomyInfoTO| view!{ <option value={it.id.to_string()}>{format!("{} ({})", it.name, it.code)}</option> })
                                                                        .collect();
                                                                }
                                                                opts
                                                            }}
                                                        </Suspense>
                                                    </Select>
                                                </FormControl>
                                            </FormItem>
                                            <FormField name="slug">
                                                <FormItem>
                                                    <FormLabel>"Slug"</FormLabel>
                                                    <FormControl>
                                                        <Input placeholder="term-slug" value=slug on_input=Callback::new(move |ev: leptos::ev::Event| slug.set(event_target_value(&ev))) />
                                                    </FormControl>
                                                </FormItem>
                                            </FormField>
                                            <FormField name="name">
                                                <FormItem>
                                                    <FormLabel>"Name"</FormLabel>
                                                    <FormControl>
                                                        <Input placeholder="Term Name" value=name on_input=Callback::new(move |ev: leptos::ev::Event| name.set(event_target_value(&ev))) />
                                                    </FormControl>
                                                </FormItem>
                                            </FormField>
                                            <DialogFooter>
                                                <Button r#type="submit".to_string() loading_signal=Signal::derive(move || create_action.pending().get()) disabled_signal=Signal::derive(move || taxonomy_id.get().is_none())>"Create"</Button>
                                            </DialogFooter>
                                        </Form>
                                    </DialogContent>
                                </Dialog>
                            </div>

                            <Suspense fallback=move || view!{<div>"Loading..."</div>}> 
                                {move || match infos.get() {
                                    Some(Ok(items)) => view!{
                                        <div class="overflow-x-auto bg-white border border-stone-200 rounded-lg">
                                            <table class="min-w-full text-sm">
                                                <thead class="bg-stone-100">
                                                    <tr>
                                                        <th class="text-left px-4 py-2">"ID"</th>
                                                        <th class="text-left px-4 py-2">"Taxonomy"</th>
                                                        <th class="text-left px-4 py-2">"Slug"</th>
                                                        <th class="text-left px-4 py-2">"Name"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {items.into_iter().map(|it: TermInfoTO| view!{
                                                        <tr class="border-t">
                                                            <td class="px-4 py-2">{it.id}</td>
                                                            <td class="px-4 py-2">{format!("{} ({})", it.taxonomy_name, it.taxonomy_code)}</td>
                                                            <td class="px-4 py-2">{it.slug}</td>
                                                            <td class="px-4 py-2">{it.name}</td>
                                                        </tr>
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        </div>
                                    }.into_any(),
                                    Some(Err(e)) => view!{<div class="text-red-600">{"Error: "}{e.to_string()}</div>}.into_any(),
                                    None => view!{<div/>}.into_any()
                                }}
                            </Suspense>
                        </div>
                    </main>
                </div>
            </SidebarProvider>
        </AdminGuard>
    }
}
