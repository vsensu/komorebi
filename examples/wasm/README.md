# WASM

## Setup

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

## Build & Run

The first command will build the example for the wasm target, creating a binary.

```bash
cargo build --release --example empty --target wasm32-unknown-unknown
```

Then, wasm-bindgen-cli is used to create javascript bindings to this wasm file, which can be loaded using this example HTML file.

```bash
wasm-bindgen --out-name wasm_example --out-dir examples/wasm/target --target web target/wasm32-unknown-unknown/release/examples/empty.wasm
```

Then serve examples/wasm directory to browser. i.e.

```bash
# cargo install basic-http-server
basic-http-server examples/wasm
```
