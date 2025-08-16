use crate::pages::about::AboutMePage;
use crate::pages::admin::home::AdminHomePage;
use crate::pages::admin::posts::AdminPostsPage;
use crate::pages::admin::post_form::{AdminPostNewPage, AdminPostEditPage};
use crate::pages::articles::ArticlesPage;
use crate::pages::components::{Footer, Navigation};
use crate::pages::home::HomePage;
use crate::pages::login::LoginPage;
use crate::pages::newsletter::NewsletterPage;
use crate::pages::not_found::NotFoundPage;
use crate::pages::register::RegisterPage;
use crate::pages::rest::auth_api::{current_user, UserTO};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, SsrMode, StaticSegment, WildcardSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Provide global user context (hydrated from server session)
    let user_ctx: RwSignal<Option<UserTO>> = RwSignal::new(None);
    provide_context(user_ctx);

    // Create a shared Resource preloaded during SSR that reads current user from session
    let current_user_res = Resource::new(|| (), |_| async move { current_user().await });
    // Provide the resource globally so admin pages can read it for rendering
    provide_context(current_user_res);

    // On hydration, sync the resource value into the simple signal for guards/navigation
    {
        let res = current_user_res.clone();
        let user_ctx = user_ctx.clone();
        Effect::new(move |_| {
            if let Some(Ok(u)) = res.get() {
                user_ctx.set(u);
            }
        });
    }

    view! {
        <Stylesheet id="leptos" href="/pkg/lazycoder_leptos.css"/>
        <Title text="LazyCoder — Modern CSS-inspired Blog"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" attr:crossorigin="anonymous" />
        <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Patrick+Hand&family=Inter:wght@400;500;600;700&display=swap" />

        <Router>
            <Navigation/>
            <main class="min-h-screen bg-[--color-bg] text-[--color-ink]">
                <Routes fallback=move || view! { <NotFoundPage/> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=path!("/home") view=HomePage/>
                    <Route path=path!("/login") view=LoginPage/>
                    <Route path=path!("/register") view=RegisterPage/>
                    <Route path=path!("/about") view=AboutMePage/>
                    <Route path=path!("/articles") view=ArticlesPage ssr=SsrMode::OutOfOrder/>
                    <Route path=path!("/newsletter") view=NewsletterPage/>
                    <Route path=path!("/admin/home") view=AdminHomePage/>
                    <Route path=path!("/admin/posts") view=AdminPostsPage/>
                    <Route path=path!("/admin/posts/new") view=AdminPostNewPage/>
                    <Route path=path!("/admin/posts/:id/edit") view=AdminPostEditPage/>
                    <Route path=WildcardSegment("any") view=NotFoundPage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
