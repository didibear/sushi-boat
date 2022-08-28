cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./dist/target --target web ./target/wasm32-unknown-unknown/release/stacker.wasm
cp -r assets dist
zip dist.zip dist/**/*
