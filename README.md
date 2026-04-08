# Aftershock

A full-stack blog application built with [Leptos](https://leptos.dev/) (frontend) and [Axum](https://github.com/tokio-rs/axum) (storage backend).

## Prerequisites

- Rust nightly toolchain
- `wasm32-unknown-unknown` target
- `cargo-leptos`
- `diesel_cli` (for database migrations)
- Node.js (for end-to-end tests)

```sh
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install --locked cargo-leptos
cargo binstall diesel_cli  # or: cargo install diesel_cli
```

## Development

### Backend (Storage Server)

Runs on `http://127.0.0.1:3030` by default (configurable via `AFTERSHOCK_DB_PORT`).

```sh
cargo run --bin aftershock_storage
```

### Frontend (Leptos SSR)

Runs on `http://127.0.0.1:3000`.

```sh
# Development with hot-reload
cargo leptos watch

# Production-like serve (no file watching)
cargo leptos serve
```

### Build

```sh
cargo leptos build           # dev build
cargo leptos build --release # release build
```

## End-to-End Tests

The project includes a comprehensive Playwright test suite under `crates/aftershock/end2end/`.

### Setup

```sh
cd crates/aftershock/end2end
npm install
npx playwright install  # download browser binaries
```

### Running Tests

**Option 1: Manual** — start both servers, then run tests:

```sh
# Terminal 1: start backend
cargo run --bin aftershock_storage

# Terminal 2: start frontend
cargo leptos serve

# Terminal 3: run tests
cd crates/aftershock/end2end
npx playwright test
```

On Windows, use `npx.cmd` instead of `npx` if running outside of npm scripts.

**Option 2: `cargo leptos end-to-end`** — starts the frontend automatically, but the backend must be started manually first:

```sh
# Terminal 1: start backend
cargo run --bin aftershock_storage

# Terminal 2: build + run e2e
cargo leptos end-to-end
```

**Option 3: npm scripts** (from `crates/aftershock/end2end/`):

```sh
npm test               # headless
npm run test:headed    # with browser UI
```

## License

BSD 3-Clause License
