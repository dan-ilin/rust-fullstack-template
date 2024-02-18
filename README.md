# rust-fullstack-template

## Introduction

A fully functional template project for fullstack applications written in Rust.
The project consists of 3 modules:
- backend
  - HTTP server providing a REST API
- frontend
  - Web frontend for user interaction
- common
  - API models and other code shared by both backend and frontend

## Getting Started

### Setup

Install dependencies:
`cargo build`

Install trunk WASM application bundler following directions [here](https://trunkrs.dev/#install).

### Running tests

To run tests:
`cargo test`

### Run application

To start the backend:
`cargo run --bin backend`

To compile and serve the frontend locally:
```
trunk serve
```

Proxying backend requests is required to avoid CORS issues when running the application locally.

### Useful References

- [Rust](https://www.rust-lang.org/)
- [Trunk](https://trunkrs.dev/)
- [Yew](https://github.com/yewstack/yew)
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen/tree/main)
- [Tokio](https://github.com/tokio-rs/tokio)
- [axum](https://github.com/tokio-rs/axum)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [serde](https://github.com/serde-rs/serde)
- [log](https://github.com/rust-lang/log)
- [lazy-static](https://github.com/rust-lang-nursery/lazy-static.rs)
- [toml](https://github.com/toml-rs/toml)
