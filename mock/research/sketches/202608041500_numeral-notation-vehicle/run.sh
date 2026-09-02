#!/usr/bin/env bash
# Rebuilds and re-runs every artifact in this sketch, in order. Run from
# this directory. Requires the pinned toolchain (rust-toolchain.toml at
# the repo root); a bare `rustc` outside the repo tree resolves to stable
# and will not accept `--edition 2024` proc-macro output the same way.
set -euo pipefail
cd "$(dirname "$0")/crates"

echo "== tower.rs, standalone =="
rustc --edition 2024 --crate-type lib tower.rs -o /tmp/libtower.rlib

echo "== numeral_pm.rs, proc-macro crate, no external deps =="
rustc --edition 2024 --crate-type proc-macro numeral_pm.rs -o /tmp/libnumeral_pm.dylib

echo "== consumer_matrix.rs (WORKS: 923 assertions, 900 exhaustive) =="
rustc --edition 2024 --extern numeral_pm=/tmp/libnumeral_pm.dylib consumer_matrix.rs -o /tmp/consumer_matrix
/tmp/consumer_matrix

echo "== consumer_ceiling_readout.rs (FAILS on purpose: 64-bit VAL readout wall) =="
rustc --edition 2024 --extern numeral_pm=/tmp/libnumeral_pm.dylib consumer_ceiling_readout.rs -o /tmp/x 2>&1 || true

echo "== consumer_ceiling_structural.rs (FAILS on purpose: u128 host-arithmetic wall) =="
rustc --edition 2024 --extern numeral_pm=/tmp/libnumeral_pm.dylib consumer_ceiling_structural.rs -o /tmp/x 2>&1 || true

echo "== consumer_diagnostic.rs (FAILS on purpose: face mismatch stays legible) =="
rustc --edition 2024 --extern numeral_pm=/tmp/libnumeral_pm.dylib consumer_diagnostic.rs -o /tmp/x 2>&1 || true

echo "== consumer_diagnostic_decay.rs (FAILS on purpose: decays to the nest one hop in) =="
rustc --edition 2024 --extern numeral_pm=/tmp/libnumeral_pm.dylib consumer_diagnostic_decay.rs -o /tmp/x 2>&1 || true

echo "== done =="
