# rs-wasi



[rust](https://wasmedge.org/docs/develop/rust/setup/)

```sh

curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

rustup target add wasm32-wasi
RUSTFLAGS="--cfg wasmedge --cfg tokio_unstable" cargo build --target wasm32-wasi --release

cargo build --target wasm32-wasi --release
wasmedge target/wasm32-wasi/release/rs-wasi.wasm
wasmedge --reactor target/wasm32-wasi/release/rs-wasi.wasm add 2 5

wasmedge compile target/wasm32-wasi/release/rs-wasi.wasm rs-wasi.wasm
wasmedge rs-wasi.wasm second state
wasmedge --reactor rs-wasi.wasm add 2 5
```