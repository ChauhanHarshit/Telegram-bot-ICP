cargo build --release --target wasm32-unknown-unknown --package http_outcalls_backend

candid-extractor target/wasm32-unknown-unknown/release/http_outcalls_backend.wasm > src/http_outcalls_backend/http_outcalls_backend.did
