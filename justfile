build-web:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen \
        --no-typescript \
        --target web \
        --out-dir ./web/wasm/ \
        --out-name "greeting_card" \
        ./target/wasm32-unknown-unknown/release/greeting_card.wasm
    wasm-opt \
        -Oz \
        -o ./web/wasm/greeting_card_bg.wasm \
        ./web/wasm/greeting_card_bg.wasm
