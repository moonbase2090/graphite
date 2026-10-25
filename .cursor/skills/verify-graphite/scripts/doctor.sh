#!/bin/sh
# Read-only check that this checkout can be driven.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/../../../.." && pwd)
cd "$root"
rustc_version=$(rustc --version)
case "$rustc_version" in
  *1.8[5-9]*|*1.9[0-9]*|*1.[1-9][0-9][0-9]*|*[2-9].[0-9]*) ;;
  *)
    printf 'doctor: rustc is older than 1.85: %s\n' "$rustc_version" >&2
    exit 1
    ;;
esac
pinned=$(sed -n 's/^rust-version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
if [ "$pinned" != "1.85" ]; then
  printf 'doctor: Cargo.toml rust-version is %s, expected 1.85\n' "$pinned" >&2
  exit 1
fi
name=$(cargo metadata --no-deps --format-version 1 | python3 -c 'import json,sys; print(json.load(sys.stdin)["packages"][0]["name"])')
if [ "$name" != "graphite-core" ]; then
  printf 'doctor: package name is %s, expected graphite-core\n' "$name" >&2
  exit 1
fi
printf 'doctor: ok rustc=%s package=%s rust-version=%s\n' "$rustc_version" "$name" "$pinned"
