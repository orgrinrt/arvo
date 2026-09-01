#!/usr/bin/env bash
# Seat 242. One arm per invocation, `build` and then `run`, because the one
# obligation that exists fires at codegen and `cargo check` skips it.
set -u
for arm in "" phase_den_zero radix_one magnitudes_zero inverted_slots; do
  if [ -z "$arm" ]; then label="positive (no features)"; flags=(); else label="$arm"; flags=(--features "$arm"); fi
  echo "======== ARM: $label"
  out=$(cargo run --quiet "${flags[@]}" 2>&1)
  status=$?
  echo "$out"
  echo "-------- exit: $status"
done
