#!/usr/bin/env bash
# Compile-cost comparison: BiasMulPP (file 41's bare alias) against
# BiasMulGeneric (probe 5's generic trait), and BiasMulPP against the
# sealed tower (probe 3's fix), all at 8-bit operands, the width
# comparable to file 36's own 12.07 ms/composition Reduce headline and
# file 41's own comparable point.
#
# Build shape: `rustc --edition 2021 --crate-type lib --emit=metadata`,
# trait-solve-only, no codegen, matching file 36's and file 41's own
# sweep shape.
#
# Scope, honestly stated against file 36's own min-of-3, full 8/16-bit
# sweep: min-of-1, 8-bit only, three kinds instead of two. Single-dispatch
# wall-clock budget, per file 41's own precedent; a member with more
# budget re-running this at min-of-3 and at 16-bit would tighten the
# numbers, not change their order (the alias-vs-generic gap is small
# enough, and the sealed-vs-unsealed gap small enough, that noise at
# min-of-1 is a real risk; both are read as approximate, not exact).
#
# Usage: ./sweep.sh   (writes results.csv next to this script)
set -euo pipefail
cd "$(dirname "$0")"

mkdir -p gen out
echo "kind,bits,count,ms" > results.csv

time_ms() {
  local t0 t1
  t0=$(python3 -c 'import time;print(int(time.time()*1000))')
  "$@" >/dev/null 2>&1
  t1=$(python3 -c 'import time;print(int(time.time()*1000))')
  echo $((t1 - t0))
}

for bits in 8; do
  for count in 0 25 50 100 200 400; do
    for kind in alias generic alias_sealed; do
      src="gen/${kind}_${bits}_${count}.rs"
      python3 gen.py "$kind" "$count" "$bits" "$src"
      ms=$(time_ms rustc --edition 2021 --crate-type lib --emit=metadata "$src" --out-dir out)
      echo "${kind},${bits},${count},${ms}" | tee -a results.csv
    done
  done
done
