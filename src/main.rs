use lazycoder_leptos::{app, config, db};

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init_env();
    let pool = db::init_pool().await.expect("Failed to connect DB");
    db::run_migrations(&pool).await.expect("Failed to run migrations");
    app::run(pool).await
}


#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use lazycoder_leptos::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}