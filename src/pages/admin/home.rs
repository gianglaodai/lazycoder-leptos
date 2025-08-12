use crate::pages::rest::auth_api::{auth_me, UserRole};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_navigate;

#[component]
pub fn AdminHomePage() -> impl IntoView {
    // server-side backed guard: ask backend who we are and redirect accordingly
    {
        Effect::new(move |_| {
            spawn_local(async move {
                let navigate = use_navigate();
                match auth_me().await {
                    Ok(UserRole::ADMIN) => {}
                    Ok(UserRole::USER) => {
                        let _ = navigate("/403", Default::default());
                    }
                    Err(_) => {
                        let _ = navigate("/login", Default::default());
                    }
                }
            });
        });
    }

    view! {
        <div class="container-page py-10 font-serif">
            <h1 class="text-3xl font-bold mb-4">"Admin Dashboard"</h1>
            <p>"Welcome, Admin!"</p>
        </div>
    }
}
