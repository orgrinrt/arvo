#!/usr/bin/env bash
# 108 p0. The test gate, run per crate, greping EVERY result line.
#
# Three known ways this corpus produces a meaningless green, all recorded by
# earlier members and all avoided here:
#   - `cargo test --workspace` from mock/benches reaches only the driver (102, 103).
#   - `bitpack-write-contend-shared` hangs without --test-threads=1 (96, 100).
#   - `tail -4` reads the doc-test result block, not the unit-test one (106).
# So: per crate, --test-threads=1 everywhere, and grep every line matching
# "test result:" rather than positional extraction.
#
# Run from mock/benches/variants.
set -u
cd "$(dirname "$0")/../../../benches/variants" || exit 1

total=0
crates=0
for d in *-shared; do
  out=$(cargo test --manifest-path "$d/Cargo.toml" -- --test-threads=1 2>&1)
  # every result line, unit tests AND doc tests
  echo "== $d"
  echo "$out" | grep -E '^test result:' | sed 's/^/   /'
  n=$(echo "$out" | grep -E '^test result:' | grep -oE '[0-9]+ passed' | grep -oE '^[0-9]+' | paste -sd+ - | bc)
  f=$(echo "$out" | grep -E '^test result:' | grep -oE '[0-9]+ failed' | grep -oE '^[0-9]+' | paste -sd+ - | bc)
  echo "   crate passed=$n failed=$f"
  total=$((total + n))
  crates=$((crates + 1))
  if [ "$f" != "0" ]; then echo "   !! FAILURES IN $d"; fi
done
echo
echo "crates=$crates total_passed=$total"
