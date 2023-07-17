RUSTFLAGS="-C link-args=-s" cargo build --release --target=wasm32-unknown-unknown
mv target/wasm32-unknown-unknown/release/crossfund.wasm contracts
NO_DOCKER=1 yarn test
