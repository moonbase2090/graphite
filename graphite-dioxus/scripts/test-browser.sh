#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
manifest="$repo_root/graphite-dioxus/Cargo.toml"
out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT

rustup target add --toolchain 1.88.0 wasm32-unknown-unknown
cargo +1.88.0 build --manifest-path "$manifest" --target wasm32-unknown-unknown --features browser-test --bin browser-contract
wasm-bindgen "$repo_root/graphite-dioxus/target/wasm32-unknown-unknown/debug/browser-contract.wasm" --target web --out-dir "$out" --out-name browser_contract
cp "$repo_root/graphite-dioxus/tests/browser-contract.html" "$out/browser-contract.html"
cp "$repo_root/graphite-dioxus/tests/browser-contract-entry.js" "$out/browser-contract-entry.js"
node "$repo_root/graphite-dioxus/tests/browser.cjs" "$out"
