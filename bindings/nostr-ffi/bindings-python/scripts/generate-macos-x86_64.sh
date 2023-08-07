#!/usr/bin/env bash

set -euo pipefail
python3 --version
pip install --user -r requirements.txt

echo "Generating nostr.py..."
cd ../
cargo build --release
cargo run -p uniffi-bindgen generate --lib-file ../../target/release/libnostr_ffi.a src/nostr.udl --language python --no-format -o bindings-python/src/nostr/

echo "Generating native binaries..."
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin

echo "Copying libraries libnostr_ffi.dylib..."
cp ../../target/x86_64-apple-darwin/release/libnostr_ffi.dylib bindings-python/src/nostr/

echo "All done!"
