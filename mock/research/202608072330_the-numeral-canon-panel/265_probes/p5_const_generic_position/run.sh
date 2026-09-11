#!/bin/sh
# Compiles each of the three files as a library for every precompiled target
# (the 16-bit target has no precompiled core and needs cargo's build-std, which
# a bare rustc call does not do; the position rule is not about the target).
cd "$(dirname "$0")" || exit 2
for f in p5a_bare_usize p5b_newtype_no_feature p5c_newtype_with_feature; do
  for target in aarch64-apple-darwin x86_64-unknown-linux-gnu i686-unknown-linux-gnu thumbv6m-none-eabi wasm32-unknown-unknown; do
    out="out/$f.$target.txt"
    rustc --edition 2024 --crate-type lib --target "$target" -o "out/$f.$target.rlib" "$f.rs" > "$out.tmp" 2>&1
    code=$?
    { echo "exit=$code"; echo "target=$target $(rustc --print cfg --target "$target" | grep target_pointer_width)"; echo "---"; cat "$out.tmp"; } > "$out"
    rm -f "$out.tmp" "out/$f.$target.rlib"
    echo "$f $target -> exit=$code"
  done
done
