#!/usr/bin/env bash
# Rebuild src/generated from model/runtime-graph-v1.json with the
# omml-xml-codegen binary of the lutaml-rs revision pinned in Cargo.toml.
# Usage: regenerate.sh OUTPUT_DIR
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
out=$(realpath -m "$1")
line=$(grep -E '^lutaml-rs = ' "$root/Cargo.toml")
repo=$(sed -E 's/.*git = "([^"]+)".*/\1/' <<<"$line")
rev=$(sed -E 's/.*rev = "([0-9a-f]{40})".*/\1/' <<<"$line")
src="$root/target/lutaml-rs-src"
rm -rf "$src" "$out"
git init -q "$src"
git -C "$src" fetch -q --depth 1 "$repo" "$rev"
git -C "$src" checkout -q FETCH_HEAD
mkdir -p "$out"
CARGO_TARGET_DIR="$root/target/codegen-target" cargo run --quiet --locked --release \
  --manifest-path "$src/Cargo.toml" --features generator,xml --bin omml-xml-codegen -- \
  "$root/model/runtime-graph-v1.json" "$out"
