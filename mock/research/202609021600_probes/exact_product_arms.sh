#!/usr/bin/env bash
# Q31 attack probe. The repaired form of the leading argument.
#
# The route both seats led with asks whether a 63-bit `Slots` impl is admitted.
# That is the wrong contract: `apply::adapt<S>(exact: Exact, dither) -> Slot` takes
# the exact value as an `Exact` and a target signature, and never asks the exact
# value to be a declared format, which is the whole point of factoring arithmetic
# into an exact operation plus an adaptation onto the target's set.
#
# The question the factoring actually poses is whether the exact value can be
# written down at all. E1 says no at the widest admitted width, E2 is the control
# at a width where it can, so E1 is about the width rather than about `Slot::at`.
#
# `cargo build` and never `cargo check`: nothing here is forced at check time.
set -uo pipefail
cd "$(dirname "$0")/carrier_dependence"
OUT="../output_exact_product_arms.txt"
mkdir -p ../out
{
  echo "probe: can the exact product of two admitted operands be written down?"
  echo "tool:  $(rustc --version)"
  echo
  echo "It encodes no expectation. What each arm was for is in its own header and"
  echo "its file name; what is printed is the exit code that came back."
  echo
  for f in arms/*.rs; do
    n=$(basename "$f" .rs)
    out=$(cargo build --bin "$n" 2>&1); code=$?
    printf '%-62s exit=%s\n' "$n" "$code"
    echo "$out" | grep -E '^error' | sed 's/^/    /' | head -3
  done
} | tee "$OUT"
