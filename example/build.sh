
cd ../;
cargo component build --release;

cd example;
cargo component build --release;

wasm-tools compose -d ../target/wasm32-wasi/release/rekuest.wasm ./target/wasm32-wasi/release/spin_example.wasm -o service.wasm

