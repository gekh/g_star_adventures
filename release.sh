cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/midota.wasm

# wasm-opt -O -ol 100 -s 100 -o ./out/output.wasm ./out/midota_bg.wasm
# wasm-opt -O3 -o ./out/output.wasm ./out/midota_bg.wasm

# mv ./out/output.wasm ./out/midota_bg.wasm

rsync -r out midota_test:midota/.

rsync -r assets midota_test:midota/.
