use crate::pages::admin::guard::AdminGuard;
use crate::pages::admin::layout::AdminSidebar;
use crate::pages::components::sidebar::SidebarProvider;
use crate::pages::components::{Button, Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger, Form, FormControl, FormField, FormItem, FormLabel, FormMessage, Input, Select};
use crate::pages::rest::post_collection_api::{load_post_collection_infos, create_post_collection, PostCollectionInfoTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn AdminPostCollectionsDashboardPage() -> impl IntoView {
    let infos = Resource::new(|| (), |_| async move {
        load_post_collection_infos(Some(0), Some(100), Some("title".to_string()), None, None).await
    });

    // Create dialog state
    let slug = RwSignal::new(String::new());
    let title = RwSignal::new(String::new());
    let visibility = RwSignal::new("PUBLIC".to_string());
    let create_action = Action::new(|(s, t, v): &(String, String, String)| {
        let s = s.clone();
        let t = t.clone();
        let v = v.clone();
        async move { create_post_collection(s, t, v).await }
    });

    // Refresh list after create
    Effect::new({
        let infos = infos.clone();
        move |_| {
            if let Some(Ok(_)) = create_action.value().get() {
                slug.set(String::new());
                title.set(String::new());
                visibility.set("PUBLIC".to_string());
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
                                <h1 class="text-3xl font-bold">"Post Collections"</h1>
                                <Dialog>
                                    <DialogTrigger>
                                        <Button intent=crate::pages::components::button::ButtonIntent::Primary>"New Collection"</Button>
                                    </DialogTrigger>
                                    <DialogContent>
                                        <DialogHeader>
                                            <DialogTitle>"Create Collection"</DialogTitle>
                                        </DialogHeader>
                                        <Form on_submit=Callback::new({
                                            let slug = slug.clone();
                                            let title = title.clone();
                                            let visibility = visibility.clone();
                                            let create_action = create_action.clone();
                                            move |_| {
                                                let _ = create_action.dispatch((slug.get_untracked(), title.get_untracked(), visibility.get_untracked()));
                                            }
                                        })>
                                            <FormField name="slug">
                                                <FormItem>
                                                    <FormLabel>"Slug"</FormLabel>
                                                    <FormControl>
                                                        <Input placeholder="collection-slug" value=slug on_input=Callback::new(move |ev: leptos::ev::Event| slug.set(event_target_value(&ev))) />
                                                    </FormControl>
                                                </FormItem>
                                            </FormField>
                                            <FormField name="title">
                                                <FormItem>
                                                    <FormLabel>"Title"</FormLabel>
                                                    <FormControl>
                                                        <Input placeholder="Collection Title" value=title on_input=Callback::new(move |ev: leptos::ev::Event| title.set(event_target_value(&ev))) />
                                                    </FormControl>
                                                </FormItem>
                                            </FormField>
                                            <FormField name="visibility">
                                                <FormItem>
                                                    <FormLabel>"Visibility"</FormLabel>
                                                    <FormControl>
                                                        <Select value=visibility on_change=Callback::new(move |ev| visibility.set(event_target_value(&ev)))>
                                                            <option value="PUBLIC">PUBLIC</option>
                                                            <option value="PRIVATE">PRIVATE</option>
                                                        </Select>
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
                                                        <th class="text-left px-4 py-2">"Slug"</th>
                                                        <th class="text-left px-4 py-2">"Title"</th>
                                                        <th class="text-left px-4 py-2">"Visibility"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {items.into_iter().map(|it: PostCollectionInfoTO| view!{
                                                        <tr class="border-t">
                                                            <td class="px-4 py-2">{it.id}</td>
                                                            <td class="px-4 py-2">{it.slug}</td>
                                                            <td class="px-4 py-2">{it.title}</td>
                                                            <td class="px-4 py-2">{it.visibility}</td>
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
