use crate::pages::rest::auth_api::{auth_me, UserRole};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_navigate;

#[component]
pub fn AdminGuard(children: leptos::children::ChildrenFn) -> impl IntoView {
    let allowed = RwSignal::new(false);

    // Ask backend to validate session/role
    {
        let allowed = allowed.clone();
        Effect::new(move |_| {
            spawn_local(async move {
                let navigate = use_navigate();
                match auth_me().await {
                    Ok(UserRole::ADMIN) => {
                        allowed.set(true);
                    }
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

    let children_fn = RwSignal::new(children);

    view! {
        <Show when=move || allowed.get() fallback=|| view!{ <></> }>
            {move || {
                let f = children_fn.get();
                f()
            }}
        </Show>
    }
}
