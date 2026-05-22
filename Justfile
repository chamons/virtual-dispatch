build:
    cargo build --target wasm32-unknown-unknown  --no-default-features
    cp ./target/wasm32-unknown-unknown/debug/virtual-dispatch.wasm web/virtual-dispatch.wasm

build-release:
    cargo build --release --target wasm32-unknown-unknown --no-default-features
    cp ./target/wasm32-unknown-unknown/release/virtual-dispatch.wasm web/virtual-dispatch.wasm


serve:
    basic-http-server web/
