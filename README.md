# Rust + WASM testing
Sandboxing for WASM binaries

## Build
Compile Rust to WASM
```
rustup target add wasm32-unknown-unknown
cargo build --target=wasm32-unknown-unknown
```

## Deploy
A very simple deployment... to the public directory
```
cp target/wasm32-unknown-unknown/release/rust_wasm.wasm public
```

## Interface
Via a web browser
```
npm test
```
Visit `http://localhost:8000/index.htm`