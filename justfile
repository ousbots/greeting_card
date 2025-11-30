build-web:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen \
        --no-typescript \
        --target web \
        --out-dir ./web/wasm/ \
        --out-name "greeting_card" \
        ./target/wasm32-unknown-unknown/release/greeting_card.wasm

[working-directory: "web"]
run-web: build-web
    python3 -m http.server 8080
