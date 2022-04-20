#/bin/bash
VER=2021-11-01
rustup toolchain install stable-$VER
rustup default stable-$VER
rustup target add wasm32-unknown-unknown
cargo build -p session_vault --target wasm32-unknown-unknown --release