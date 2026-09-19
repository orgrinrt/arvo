#!/usr/bin/env bash
# Builds every arm of both options and records the exit code and the diagnostic.
# Run from this directory; the pinned toolchain resolves through the
# `rust-toolchain.toml` beside it. Metadata goes to a throwaway directory, so
# nothing is written here but `output.txt`.
set -u
out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT

arm() {
    local file="$1" target="$2" cfg="$3" expect="$4"
    local args=(--edition 2024 --crate-type lib --emit=metadata --out-dir "$out" --target "$target")
    if [ -n "$cfg" ]; then args+=(--cfg "$cfg"); fi
    local log
    log="$(rustc "${args[@]}" "$file" 2>&1)"
    local code=$?
    local verdict=builds
    if [ "$code" -ne 0 ]; then verdict=refused; fi
    local ok=as-expected
    if [ "$verdict" != "$expect" ]; then ok=UNEXPECTED; fi
    echo "== $file target=$target cfg=${cfg:-none} exit=$code $verdict expected=$expect $ok"
    if [ "$code" -ne 0 ]; then echo "$log" | grep -E '^error' | sort -u; fi
}

rustc --version

arm z_an_i64_index_restated.rs aarch64-apple-darwin "" builds
arm z_an_i64_index_restated.rs aarch64-apple-darwin controls builds
arm z_an_i64_index_restated.rs aarch64-apple-darwin signed_64_counted refused
arm z_an_i64_index_restated.rs aarch64-apple-darwin unsigned_64_counted refused
arm z_an_i64_index_restated.rs aarch64-apple-darwin unsigned_64_exclusive refused
arm z_an_i64_index_restated.rs aarch64-apple-darwin signed_64_exclusive refused

for t in aarch64-apple-darwin x86_64-unknown-linux-gnu i686-unknown-linux-gnu thumbv6m-none-eabi wasm32-unknown-unknown; do
    arm y_an_i128_index.rs "$t" "" builds
done
arm y_an_i128_index.rs aarch64-apple-darwin past_the_ladder refused
