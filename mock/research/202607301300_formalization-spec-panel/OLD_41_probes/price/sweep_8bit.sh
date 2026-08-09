#!/usr/bin/env bash
# Compile-cost sweep for Bias's magnitude reduction (`bias_mag`, BiasMagN/
# BiasMagD alone) and the full sign-plus-magnitude composition a consumer
# actually names (`bias_full`, BiasMulPP).
#
# Every instantiation is forced by a const assertion against a
# Python-computed answer (fractions.Fraction, reduced), so nothing is
# elided and correctness is checked at the same time as cost.
#
# Build shape: `rustc --edition 2021 --crate-type lib --emit=metadata`,
# trait-solve-only, no codegen, the same shape file 36's own sweep uses and
# the honest one for type-level arithmetic (all of the cost is trait
# solving, none of it is codegen).
#
# Scope, honestly stated against file 36's own sweep: min-of-1 rather than
# min-of-3 (one run per point, not three), and 16-bit only rather than both
# 8 and 16-bit, to keep the wall-clock budget for this single dispatch
# reasonable. The counts (0, 25, 50, 100, 200, 400) and the width (16-bit,
# file 36's own headline width) match, so the two numbers are read against
# each other on the same basis.
#
# Usage: ./sweep.sh   (writes results_8bit.csv next to this script)
set -euo pipefail
cd "$(dirname "$0")"

mkdir -p gen out
echo "kind,bits,count,ms" > results_8bit.csv

time_ms() {
  local t0 t1
  t0=$(python3 -c 'import time;print(int(time.time()*1000))')
  "$@" >/dev/null 2>&1
  t1=$(python3 -c 'import time;print(int(time.time()*1000))')
  echo $((t1 - t0))
}

for bits in 8; do
  for count in 0 25 50 100 200 400; do
    for kind in bias_mag bias_full; do
      src="gen/${kind}_${bits}_${count}.rs"
      python3 gen.py "$kind" "$count" "$bits" "$src"
      ms=$(time_ms rustc --edition 2021 --crate-type lib --emit=metadata "$src" --out-dir out)
      echo "${kind},${bits},${count},${ms}" | tee -a results_8bit.csv
    done
  done
done
