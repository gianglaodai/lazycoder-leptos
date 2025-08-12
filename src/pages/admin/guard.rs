use crate::pages::rest::auth_api::{UserRole, UserTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_navigate;

#[component]
pub fn AdminGuard(children: leptos::children::ChildrenFn) -> impl IntoView {
    let allowed = RwSignal::new(false);

    // Check client-side user context first (hydrated from session). Avoid extra me() calls.
    {
        let allowed = allowed.clone();
        Effect::new(move |_| {
            let navigate = use_navigate();
            if let Some(user_ctx) = use_context::<RwSignal<Option<UserTO>>>() {
                match user_ctx.get() {
                    Some(user) => match user.role {
                        UserRole::ADMIN => {
                            allowed.set(true);
                        }
                        UserRole::USER => {
                            let _ = navigate("/403", Default::default());
                        }
                    },
                    None => {
                        // Not authenticated
                        let _ = navigate("/login", Default::default());
                    }
                }
            } else {
                // No user context available – be safe and redirect to login
                let _ = navigate("/login", Default::default());
            }
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
