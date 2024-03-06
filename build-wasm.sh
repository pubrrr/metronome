#!/usr/bin/env bash

set -xe

echo "Building:"
cargo build --release --target wasm32-unknown-unknown

echo "Generating WASM JS:"
wasm-bindgen --out-name metronome_wasm --out-dir ./public/wasm --target web target/wasm32-unknown-unknown/release/metronome.wasm

echo "Copying assets:"
cp -r ./assets ./public/assets

echo "done building wasm"
