use crate::pages::admin::guard::AdminGuard;
use crate::pages::admin::layout::AdminSidebar;
use crate::pages::components::sidebar::SidebarProvider;
use crate::pages::components::{Button, Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger, Form, FormControl, FormField, FormItem, FormLabel, FormMessage, Input};
use crate::pages::rest::user_api::{load_user_infos, create_user, UserInfoTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn AdminUsersDashboardPage() -> impl IntoView {
    let infos = Resource::new(|| (), |_| async move {
        load_user_infos(Some(0), Some(100), Some("username".to_string()), None, None).await
    });

    // Create dialog state
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let create_action = Action::new(|(u, e, p): &(String, String, String)| {
        let u = u.clone();
        let e = e.clone();
        let p = p.clone();
        async move { create_user(u, e, p).await }
    });

    // Refresh list after create
    Effect::new({
        let infos = infos.clone();
        move |_| {
            if let Some(Ok(_)) = create_action.value().get() {
                username.set(String::new());
                email.set(String::new());
                password.set(String::new());
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
                                <h1 class="text-3xl font-bold">"Users"</h1>
                                <Dialog>
                                    <DialogTrigger>
                                        <Button intent=crate::pages::components::button::ButtonIntent::Primary>"New User"</Button>
                                    </DialogTrigger>
                                    <DialogContent>
                                        <DialogHeader>
                                            <DialogTitle>"Create User"</DialogTitle>
                                        </DialogHeader>
                                        <Form on_submit=Callback::new({
                                            let username = username.clone();
                                            let email = email.clone();
                                            let password = password.clone();
                                            let create_action = create_action.clone();
                                            move |_| {
                                                let _ = create_action.dispatch((username.get_untracked(), email.get_untracked(), password.get_untracked()));
                                            }
                                        })>
                                            <FormField name="username">
                                                <FormItem>
                                                    <FormLabel>"Username"</FormLabel>
                                                    <FormControl>
                                                        <Input placeholder="username" value=username on_input=Callback::new(move |ev: leptos::ev::Event| username.set(event_target_value(&ev))) />
                                                    </FormControl>
                                                </FormItem>
                                            </FormField>
                                            <FormField name="email">
                                                <FormItem>
                                                    <FormLabel>"Email"</FormLabel>
                                                    <FormControl>
                                                        <Input placeholder="email@example.com" value=email on_input=Callback::new(move |ev: leptos::ev::Event| email.set(event_target_value(&ev))) />
                                                    </FormControl>
                                                </FormItem>
                                            </FormField>
                                            <FormField name="password">
                                                <FormItem>
                                                    <FormLabel>"Password"</FormLabel>
                                                    <FormControl>
                                                        <Input r#type="password" placeholder="********" value=password on_input=Callback::new(move |ev: leptos::ev::Event| password.set(event_target_value(&ev))) />
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
                                                        <th class="text-left px-4 py-2">"Username"</th>
                                                        <th class="text-left px-4 py-2">"Email"</th>
                                                        <th class="text-left px-4 py-2">"Role"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {items.into_iter().map(|it: UserInfoTO| view!{
                                                        <tr class="border-t">
                                                            <td class="px-4 py-2">{it.id}</td>
                                                            <td class="px-4 py-2">{it.username}</td>
                                                            <td class="px-4 py-2">{it.email}</td>
                                                            <td class="px-4 py-2">{it.role}</td>
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
