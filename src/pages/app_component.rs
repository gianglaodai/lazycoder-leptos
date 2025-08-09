use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet, Title, Link};
use leptos_router::{components::{Route, Router, Routes}, path, SsrMode, StaticSegment, WildcardSegment};
use crate::pages::about::AboutMePage;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFoundPage;
use crate::pages::articles::ArticlesPage;
use crate::pages::components::Navigation;
use crate::pages::newsletter::NewsletterPage;

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
                    <Route path=path!("/about") view=AboutMePage/>
                    <Route path=path!("/articles") view=ArticlesPage ssr=SsrMode::OutOfOrder/>
                    <Route path=path!("/newsletter") view=NewsletterPage/>
                    <Route path=WildcardSegment("any") view=NotFoundPage/>
                </Routes>
            </main>
        </Router>
    }
}