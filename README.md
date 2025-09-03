<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# LazyCoder Leptos

This project is a Leptos + Actix full‑stack app (server-side rendering + hydration) with TailwindCSS. Below are the commands and required environment variables to run it locally and in production.

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

`cargo install cargo-leptos --locked`

Then run

`cargo leptos new --git leptos-rs/start-actix`

to generate a new project template (you will be prompted to enter a project name).

`cd {projectname}`

to go to your newly created project.

Of course, you should explore around the project structure, but the best place to start with your application code is in `src/app.rs`.

## Running your project

1. Copy .env.example to .env and fill in DATABASE_URL and SECRET_KEY.
2. Install Rust toolchain and cargo-leptos (see below), plus Node/Bun deps: npm i or bun i.
3. In one terminal, build Tailwind once or run watch:
   - npm run start
4. In another terminal, run the SSR dev server:
   - npm run watch:ssr

By default, you can access your local project at http://localhost:3000

`cargo leptos watch`  
By default, you can access your local project at `http://localhost:3000`

## Testing

Use cargo-leptos to run tests across the appropriate targets:

- Run tests: `cargo leptos test`
- Alternatively, via package script: `npm test`
- Run SSR-targeted tests explicitly: `cargo leptos test --bin-features ssr --lib-features hydrate --bin-default-features false --lib-default-features false` (or `npm run test:ssr`)

Note: This replaces plain `cargo test`. If you only want to run library/unit tests for the hydration (wasm) target, cargo-leptos handles that automatically. If you encounter SSR compilation issues during cargo-leptos test, run plain `cargo test` for quick local unit checks and open an issue to track SSR test configuration.

## Running with SSR features

cargo-leptos requires a subcommand (e.g., serve, watch, build, test). The top‑level flag form `cargo leptos --features=ssr` is not accepted by cargo-leptos and will fail with "unexpected argument '--features'".

Instead, either rely on the project defaults defined in Cargo.toml [package.metadata.leptos] (bin-features = ["ssr"], lib-features = ["hydrate"]) or pass features to a specific subcommand, for example:

- Serve with SSR: `cargo leptos serve --bin-features ssr --lib-features hydrate --bin-default-features false --lib-default-features false`
- Watch with SSR: `cargo leptos watch --bin-features ssr --lib-features hydrate --bin-default-features false --lib-default-features false`

You can also use the npm scripts added for convenience:

- `npm run serve:ssr`
- `npm run watch:ssr`

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future)

## Environment Variables

Create a .env file in the project root (you can start from .env.example):

Required
- DATABASE_URL: PostgreSQL connection string used by sqlx on the server.
  Example: postgres://USER:PASSWORD@localhost:5432/DATABASE
- SECRET_KEY: 32-byte secret used by Actix SessionMiddleware for signing/encrypting cookies.
  Generate securely, e.g.:
  - openssl rand -base64 32
  - node -e "console.log(require('crypto').randomBytes(32).toString('hex'))"

Optional (Leptos runtime config; usually not needed locally because Cargo.toml already sets these)
- LEPTOS_OUTPUT_NAME: default lazycoder_leptos
- LEPTOS_SITE_ROOT: default target/site
- LEPTOS_SITE_PKG_DIR: default pkg
- LEPTOS_SITE_ADDR: default 127.0.0.1:3000
- LEPTOS_RELOAD_PORT: default 3001

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
leptos_start
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="leptos_start"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Notes about CSR and Trunk:
Although it is not recommended, you can also run your project without server integration using the feature `csr` and `trunk serve`:

`trunk serve --open --features csr`

This may be useful for integrating external tools which require a static site, e.g. `tauri`.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
