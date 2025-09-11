#![cfg(feature = "ssr")]

use crate::routes::config;
use actix_files::NamedFile;
use actix_web::middleware::NormalizePath;
use actix_web::web::Data;
use actix_web::{get, Result};
use leptos::config::LeptosOptions;
use leptos_actix::handle_server_fns;
use sqlx::PgPool;

pub async fn run(pool: PgPool) -> std::io::Result<()> {
    use crate::pages::app::App;
    use crate::state::new_app_state;
    use actix_files::Files;
    use actix_session::storage::CookieSessionStore;
    use actix_web::cookie::Key;
    use actix_web::middleware::Logger;
    use actix_web::{App, HttpServer};
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos::prelude::{AutoReload, GlobalAttributes, HydrationScripts};
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    let secret_key = Key::from(
        std::env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set")
            .as_bytes(),
    );
    let state = new_app_state(pool.clone()).await;
    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        println!("listening on http://{}", &addr);

        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .wrap({
                use actix_session::SessionMiddleware;
                use actix_web::cookie::SameSite;
                let session_mw = SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    // Make cookie HTTP-only to mitigate XSS
                    .cookie_http_only(true)
                    // Lax is a good default for session cookies protecting from CSRF on cross-site POST
                    .cookie_same_site(SameSite::Lax)
                    // Set a custom, stable cookie name
                    .cookie_name("lazycoder.sid".into())
                    // Set secure only in non-DEV environments (requires HTTPS)
                    .cookie_secure(std::env::var("RUST_ENV").map(|v| v != "DEV").unwrap_or(false))
                    .build();
                session_mw
            })
            .wrap(NormalizePath::trim())
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", &site_root))
            .service(favicon)
            .route("/api/{tail:.*}", handle_server_fns())
            .configure(config)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
        .app_data(Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
        .bind(&addr)?
        .run()
        .await
}

#[get("favicon.ico")]
async fn favicon(leptos_options: Data<LeptosOptions>) -> Result<NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(NamedFile::open(format!("{site_root}/favicon.ico"))?)
}
