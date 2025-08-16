use crate::pages::components::Button;
use crate::pages::rest::auth_api::register;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_navigate;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirm = RwSignal::new(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let u = username.get();
        let e = email.get();
        let p = password.get();
        let c = confirm.get();

        if p != c {
            #[cfg(target_arch = "wasm32")]
            {
                let _ = window().alert_with_message("Passwords do not match");
            }
            return;
        }

        spawn_local(async move {
            match register(u, e, p).await {
                Ok(_) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let _ = window().alert_with_message("Registration successful. Please sign in.");
                    }
                    let navigate = use_navigate();
                    let _ = navigate("/login", Default::default());
                }
                Err(e) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let _ = window().alert_with_message(&format!("Registration failed: {}", e));
                    }
                }
            }
        });
    };

    view! {
        <div class="container-page max-w-md mx-auto py-10 font-serif">
            <h1 class="text-3xl font-bold mb-6 text-center">"Register"</h1>
            <form class="bg-white p-6 rounded-xl shadow space-y-4" on:submit=on_submit>
                <div>
                    <label class="block mb-1 font-semibold">"Username"</label>
                    <input
                        class="w-full border rounded px-3 py-2"
                        type="text"
                        placeholder="Enter username"
                        prop:value=move || username.get()
                        on:input=move |ev| username.set(event_target_value(&ev))
                        required
                    />
                </div>
                <div>
                    <label class="block mb-1 font-semibold">"Email"</label>
                    <input
                        class="w-full border rounded px-3 py-2"
                        type="email"
                        placeholder="Enter email"
                        prop:value=move || email.get()
                        on:input=move |ev| email.set(event_target_value(&ev))
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
                <div>
                    <label class="block mb-1 font-semibold">"Confirm Password"</label>
                    <input
                        class="w-full border rounded px-3 py-2"
                        type="password"
                        placeholder="Confirm password"
                        prop:value=move || confirm.get()
                        on:input=move |ev| confirm.set(event_target_value(&ev))
                        required
                    />
                </div>
                <Button class="w-full" r#type="submit".to_string()>CREATE ACCOUNT</Button>
                <p class="text-sm text-stone-500">Already have an account? <a href="/login" class="text-blue-600 hover:underline">Sign in</a></p>
            </form>
        </div>
    }
}
