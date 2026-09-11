#!/bin/sh
# Lowers arm.rs at -O for each precompiled target and keeps the assembly, so the
# claim that the width predicate folds is a file a reader can open.
cd "$(dirname "$0")" || exit 2
for target in aarch64-apple-darwin x86_64-unknown-linux-gnu i686-unknown-linux-gnu thumbv6m-none-eabi wasm32-unknown-unknown; do
  rustc --edition 2024 --crate-type lib -O --emit asm --target "$target" -o "$target.s" arm.rs 2> "$target.err" && echo "$target ok" || echo "$target FAILED"
done
