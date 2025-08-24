<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Starter Template

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
