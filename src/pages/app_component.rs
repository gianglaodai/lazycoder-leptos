use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
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
        <Stylesheet id="leptos" href="/pkg/lazycoder-leptos.css"/>
        <Title text="Welcome to LazyCoder"/>

        <Router>
            <Navigation/>
            <main class="min-h-screen bg-white">
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