use leptos::prelude::*;
use leptos::{component, view, IntoView};

/// 403 - Forbidden
#[component]
pub fn ForbiddenPage() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        // set HTTP status 403 during SSR
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::FORBIDDEN);
    }

    view! {
        <div class="container-page py-10 font-serif">
            <h1 class="text-3xl font-bold mb-4">"403 - Forbidden"</h1>
            <p>"You do not have permission to access this page."</p>
        </div>
    }
}
