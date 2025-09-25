use crate::pages::admin::guard::AdminGuard;
use crate::pages::admin::layout::AdminSidebar;
use crate::pages::components::sidebar::SidebarProvider;
use crate::pages::components::{Button, Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger, Form, FormControl, FormField, FormItem, FormLabel, FormMessage, Input};
use crate::pages::rest::post_taxonomy_api::{load_post_taxonomy_infos, create_post_taxonomy, PostTaxonomyInfoTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn AdminPostTaxonomiesDashboardPage() -> impl IntoView {
    let infos = Resource::new(|| (), |_| async move {
        load_post_taxonomy_infos(Some(0), Some(100), Some("name".to_string()), None, None).await
    });

    // Create dialog state
    let code = RwSignal::new(String::new());
    let name = RwSignal::new(String::new());
    let create_action = Action::new(|(c, n): &(String, String)| {
        let c = c.clone();
        let n = n.clone();
        async move { create_post_taxonomy(c, n).await }
    });

    // Refresh list after create
    Effect::new({
        let infos = infos.clone();
        move |_| {
            if let Some(Ok(_)) = create_action.value().get() {
                code.set(String::new());
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
                                <h1 class="text-3xl font-bold">"Taxonomies"</h1>
                                <Dialog>
                                    <DialogTrigger>
                                        <Button intent=crate::pages::components::button::ButtonIntent::Primary>"New Taxonomy"</Button>
                                    </DialogTrigger>
                                    <DialogContent>
                                        <DialogHeader>
                                            <DialogTitle>"Create Taxonomy"</DialogTitle>
                                        </DialogHeader>
                                        <Form on_submit=Callback::new({
                                            let code = code.clone();
                                            let name = name.clone();
                                            let create_action = create_action.clone();
                                            move |_| {
                                                let _ = create_action.dispatch((code.get_untracked(), name.get_untracked()));
                                            }
                                        })>
                                            <FormField name="code">
                                                <FormItem>
                                                    <FormLabel>"Code"</FormLabel>
                                                    <FormControl>
                                                        <Input placeholder="TAX_CODE" value=code on_input=Callback::new(move |ev: leptos::ev::Event| code.set(event_target_value(&ev))) />
                                                    </FormControl>
                                                </FormItem>
                                            </FormField>
                                            <FormField name="name">
                                                <FormItem>
                                                    <FormLabel>"Name"</FormLabel>
                                                    <FormControl>
                                                        <Input placeholder="Taxonomy Name" value=name on_input=Callback::new(move |ev: leptos::ev::Event| name.set(event_target_value(&ev))) />
                                                    </FormControl>
                                                </FormItem>
                                            </FormField>
                                            <DialogFooter>
                                                <Button r#type="submit".to_string() loading_signal=Signal::derive(move || create_action.pending().get())>"Create"</Button>
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
                                                        <th class="text-left px-4 py-2">"Code"</th>
                                                        <th class="text-left px-4 py-2">"Name"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {items.into_iter().map(|it: PostTaxonomyInfoTO| view!{
                                                        <tr class="border-t">
                                                            <td class="px-4 py-2">{it.id}</td>
                                                            <td class="px-4 py-2">{it.code}</td>
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
