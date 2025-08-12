use crate::pages::rest::auth_api::{auth_login, UserRole};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_navigate;

#[component]
pub fn LoginPage() -> impl IntoView {
    let username_or_email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let remember = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let input = username_or_email.get();
        let pwd = password.get();
        let remember_flag = remember.get();
        spawn_local(async move {
            match auth_login(input, pwd, remember_flag).await {
                Ok(role) => {
                    let navigate = use_navigate();
                    match role {
                        UserRole::ADMIN => {
                            let _ = navigate("/admin/home", Default::default());
                        }
                        UserRole::USER => {
                            let _ = navigate("/home", Default::default());
                        }
                    }
                }
                Err(_e) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let _ = window().alert_with_message(
                            "Invalid credentials. Try admin/admin or user/password",
                        );
                    }
                }
            }
        });
    };

    view! {
        <div class="container-page max-w-md mx-auto py-10 font-serif">
            <h1 class="text-3xl font-bold mb-6 text-center">"Login"</h1>
            <form class="bg-white p-6 rounded-xl shadow space-y-4" on:submit=on_submit>
                <div>
                    <label class="block mb-1 font-semibold">"Username or Email"</label>
                    <input
                        class="w-full border rounded px-3 py-2"
                        type="text"
                        placeholder="Enter username or email"
                        prop:value=move || username_or_email.get()
                        on:input=move |ev| username_or_email.set(event_target_value(&ev))
                        required
                    />
                </div>
                <div>
                    <label class="block mb-1 font-semibold">"Password"</label>
                    <input
                        class="w-full border rounded px-3 py-2"
                        type="password"
                        placeholder="Enter password"
                        prop:value=move || password.get()
                        on:input=move |ev| password.set(event_target_value(&ev))
                        required
                    />
                </div>
                <div class="flex items-center gap-2">
                    <input id="remember" type="checkbox" class="h-4 w-4" prop:checked=move || remember.get() on:change=move |ev| remember.set(event_target_checked(&ev)) />
                    <label for="remember">"Remember me"</label>
                </div>
                <button type="submit" class="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700 transition">SIGN IN</button>
                <p class="text-sm text-stone-500">"Demo credentials: admin/admin or anyname/password"</p>
            </form>
        </div>
    }
}
