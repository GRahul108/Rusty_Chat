<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Starter Template

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

`cargo install cargo-leptos`




## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly ` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future)


## Install TailwindCSS
Install TailwindCSS with `npm install -D tailwindcss`

## Download LLMA
After downloading the language model update the path in .env folder.

## Run The App
To run the project locally,

`run npx tailwindcss -i ./input.css -o ./style/output.css --watch ` in a terminal - this will build style/output.css and automatically rebuild when a change is detected in input.css

`cargo leptos watch` in the project directory.

In in your browser, navigate to `http://localhost:3000/?`


