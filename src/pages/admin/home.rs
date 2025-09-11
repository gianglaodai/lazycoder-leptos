use crate::pages::admin::guard::AdminGuard;
use crate::pages::admin::layout::AdminLayout;
use crate::pages::rest::auth_api::{UserRole, UserTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_navigate;

#[component]
pub fn AdminHomePage() -> impl IntoView {
    // Client-side guard using global user context hydrated from session
    {
        Effect::new(move |_| {
            let navigate = use_navigate();
            if let Some(user_ctx) = use_context::<RwSignal<Option<UserTO>>>() {
                match user_ctx.get() {
                    Some(user) => match user.role {
                        UserRole::ADMIN => {}
                        UserRole::USER => {
                            let _ = navigate("/403", Default::default());
                        }
                    },
                    None => {
                        let _ = navigate("/login", Default::default());
                    }
                }
            } else {
                let _ = navigate("/login", Default::default());
            }
        });
    }

    view! {
        <AdminGuard>
            <AdminLayout>
                <div class="container-page py-10 font-serif">
                    <h1 class="text-3xl font-bold mb-4">"Admin Dashboard"</h1>
                    <p>"Welcome, Admin!"</p>
                    <div class="mt-6">
                        <a href="/admin/posts" class="inline-flex items-center rounded-full bg-stone-800 text-white px-4 py-2 text-sm font-medium shadow-sm hover:bg-stone-900 transition-colors">Manage Posts</a>
                    </div>
                </div>
            </AdminLayout>
        </AdminGuard>
    }
}
