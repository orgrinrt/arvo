#!/bin/sh
# Test gate for dispatch 160. Runs the thirteen -shared bench variant crates,
# each separately, at --release, serialising bitpack-write-contend-shared per
# the standing instruction (its default-runner hang and soundness bug are
# handled elsewhere; the crate is not touched).
#
# Negative control (the case that must fail, declared before the run): a crate
# whose run produces no parseable "N passed" line prints MISSING OR ZERO, so a
# silently failed invocation cannot read as green. The control is exercised by
# the CONTROL line at the bottom, which runs a nonexistent crate and must print
# MISSING OR ZERO.
cd "$(dirname "$0")/../../../benches" || exit 1
total=0
for c in bitpack-carrier-shared bitpack-contend-shared bitpack-footprint-shared \
         bitpack-plan-shared bitpack-shared bitpack-wide-shared \
         quantiser-fadd-shared quantiser-radix-shared satfold-shared \
         warm-clamp-shared warm-container-shared wide-rung-shared; do
  out=$(cargo test --offline --release --manifest-path "variants/$c/Cargo.toml" 2>/dev/null | grep -E '^test result:')
  n=$(printf '%s' "$out" | grep -oE '[0-9]+ passed' | grep -oE '[0-9]+' | paste -sd+ - | bc)
  if [ -z "$n" ] || [ "$n" -eq 0 ]; then echo "$c : MISSING OR ZERO"; else echo "$c : $n passed"; total=$((total+n)); fi
done
c=bitpack-write-contend-shared
out=$(cargo test --offline --release --manifest-path "variants/$c/Cargo.toml" -- --test-threads=1 2>/dev/null | grep -E '^test result:')
n=$(printf '%s' "$out" | grep -oE '[0-9]+ passed' | grep -oE '[0-9]+' | paste -sd+ - | bc)
if [ -z "$n" ] || [ "$n" -eq 0 ]; then echo "$c : MISSING OR ZERO"; else echo "$c (serial) : $n passed"; total=$((total+n)); fi
echo "total : $total"
out=$(cargo test --offline --release --manifest-path "variants/does-not-exist/Cargo.toml" 2>/dev/null | grep -E '^test result:')
n=$(printf '%s' "$out" | grep -oE '[0-9]+ passed' | grep -oE '[0-9]+' | paste -sd+ - | bc)
if [ -z "$n" ] || [ "$n" -eq 0 ]; then echo "CONTROL nonexistent crate : MISSING OR ZERO (control fires)"; else echo "CONTROL FAILED"; fi
