# Setup Guide

## Preparetion

- `rustup default nightly`
- `rustup target add wasm32-unknown-unknown`
- `cargo install --locked cargo-leptos`

## Frontend

Just use `cargo leptos watch`.

## Backend

Install diesel_cli using `cargo binstall diesel_cli` or `cargo install diesel_cli` first.

Then `cargo run --bin aftershock_storage`