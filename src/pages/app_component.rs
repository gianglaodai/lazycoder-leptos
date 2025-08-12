use crate::pages::about::AboutMePage;
use crate::pages::admin::home::AdminHomePage;
use crate::pages::articles::ArticlesPage;
use crate::pages::components::{Footer, Navigation};
use crate::pages::forbidden::ForbiddenPage;
use crate::pages::home::HomePage;
use crate::pages::login::LoginPage;
use crate::pages::newsletter::NewsletterPage;
use crate::pages::not_found::NotFoundPage;
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
                    <Route path=path!("/403") view=ForbiddenPage/>
                    <Route path=path!("/admin/home") view=AdminHomePage/>
                    <Route path=path!("/about") view=AboutMePage/>
                    <Route path=path!("/articles") view=ArticlesPage ssr=SsrMode::OutOfOrder/>
                    <Route path=path!("/newsletter") view=NewsletterPage/>
                    <Route path=WildcardSegment("any") view=NotFoundPage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
