# rust-wasm

A minimal Rust → WebAssembly project that runs in the browser with no bundler.

## Prerequisites

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

## Build & run

```sh
wasm-pack build --target web     # outputs ./pkg
python3 -m http.server 8080      # any static server works
```

Open http://localhost:8080.

(Opening `index.html` directly via `file://` won't work — browsers
require wasm to be served over HTTP.)

## Layout

- `src/lib.rs` — pure Rust logic (`greet`, `fibonacci`, `count_primes`)
- `src/dom.rs` — DOM wiring written in Rust via `web-sys`; its
  `#[wasm_bindgen(start)]` function runs automatically on load
- `index.html` — static markup plus a one-line `init()` that loads the wasm

## Tests

```sh
cargo test
```
