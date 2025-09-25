use crate::pages::admin::guard::AdminGuard;
use crate::pages::admin::layout::AdminSidebar;
use crate::pages::components::sidebar::SidebarProvider;
use crate::pages::components::{Input, Button, Select};
use crate::pages::rest::user_api::{load_user_by_id, update_user, UserTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_params_map;

#[component]
pub fn AdminUserEditPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|v| v.parse::<i32>().ok())).unwrap_or(0);

    let entity_res = Resource::new(move || id(), |id| async move { load_user_by_id(id).await });

    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let role = RwSignal::new("USER".to_string());

    Effect::new({
        let entity_res = entity_res.clone();
        move |_| {
            if let Some(Ok(e)) = entity_res.get() {
                username.set(e.username.clone());
                email.set(e.email.clone());
                role.set(e.role.clone());
            }
        }
    });

    let save_action = Action::new(move |(u, e, p, r): &(String, String, String, String)| {
        let mut current = entity_res.get().and_then(|r| r.ok()).unwrap_or(UserTO{
            id: id(),
            uid: String::new(),
            version: 0,
            created_at: time::OffsetDateTime::UNIX_EPOCH,
            updated_at: time::OffsetDateTime::UNIX_EPOCH,
            username: String::new(),
            email: String::new(),
            password: String::new(),
            role: String::new(),
        });
        current.username = u.clone();
        current.email = e.clone();
        if !p.is_empty() { current.password = p.clone(); }
        current.role = r.clone();
        async move { update_user(current).await }
    });

    view! {
        <AdminGuard>
            <SidebarProvider default_open=true>
                <div class="flex gap-0">
                    <AdminSidebar />
                    <main class="flex-1 min-h-screen">
                        <div class="container-page py-10 font-serif">
                            <h1 class="text-3xl font-bold mb-6">"Edit User"</h1>
                            <Suspense fallback=move || view!{<div>"Loading..."</div>}>
                                {move || match entity_res.get() {
                                    Some(Ok(_e)) => view!{
                                        <div class="grid gap-3 max-w-xl">
                                            <Input placeholder="Username" value=username on_input=Callback::new(move |ev| username.set(event_target_value(&ev))) />
                                            <Input placeholder="Email" value=email on_input=Callback::new(move |ev| email.set(event_target_value(&ev))) />
                                            <Input r#type="password" placeholder="New Password (optional)" value=password on_input=Callback::new(move |ev| password.set(event_target_value(&ev))) />
                                            <Select value=role on_change=Callback::new(move |ev| role.set(event_target_value(&ev)))>
                                                <option value="USER">USER</option>
                                                <option value="ADMIN">ADMIN</option>
                                            </Select>
                                            <div>
                                                <Button on_click=Callback::new({ let username=username.clone(); let email=email.clone(); let password=password.clone(); let role=role.clone(); let save=save_action.clone(); move |_| { let _= save.dispatch((username.get_untracked(), email.get_untracked(), password.get_untracked(), role.get_untracked())); } }) loading_signal=Signal::derive(move || save_action.pending().get())>
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
