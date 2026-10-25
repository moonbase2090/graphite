#!/bin/sh
# Run from the repository root, with the selected Rust toolchain on PATH.
set -eu
cargo test --locked --features terminal-palette
mkdir -p target
cargo run --quiet --locked --example terminal_palettes --features terminal-palette > target/terminal-palettes.md
diff -u docs/terminal-palettes.md target/terminal-palettes.md
cat target/terminal-palettes.md
