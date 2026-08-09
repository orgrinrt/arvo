#!/usr/bin/env bash
# File 54's radix-axis compile-cost sweep. Same build shape as every prior sweep in this
# review (36, 41, 42, 53):
#   rustc --edition 2021 --crate-type lib --emit=metadata
# Min-of-3 per point. Toolchain resolved from the repo pin; verified from this directory
# before the run, per file 52's finding that a bare rustc outside the tree is stable.
set -uo pipefail
cd "$(dirname "$0")"

mkdir -p gen out
echo "kind,kmax,ms_run1,ms_run2,ms_run3,ms_min,status" > results.csv

time_ms() {
  local t0 t1
  t0=$(python3 -c 'import time;print(int(time.time()*1000))')
  "$@" >/dev/null 2>&1
  local rc=$?
  t1=$(python3 -c 'import time;print(int(time.time()*1000))')
  echo "$((t1 - t0)) $rc"
}

run_point() {
  local kind=$1 kmax=$2
  local src="gen/${kind}_${kmax}.rs"
  python3 gen.py "$kind" "$kmax" "$src"
  local a b c m rc out
  out=$(time_ms rustc --edition 2021 --crate-type lib --emit=metadata "$src" --out-dir out); a=${out% *}; rc=${out#* }
  out=$(time_ms rustc --edition 2021 --crate-type lib --emit=metadata "$src" --out-dir out); b=${out% *}
  out=$(time_ms rustc --edition 2021 --crate-type lib --emit=metadata "$src" --out-dir out); c=${out% *}
  m=$a; [ "$b" -lt "$m" ] && m=$b; [ "$c" -lt "$m" ] && m=$c
  local status=ok; [ "$rc" != "0" ] && status=REFUSED
  echo "${kind},${kmax},${a},${b},${c},${m},${status}" | tee -a results.csv
}

# baseline: the tower alone, no numerals
run_point radix_exp 0 2>/dev/null || true
for k in 1 2 4 8 12 16 19 20 24; do
  run_point radix_exp "$k"
  run_point absorbed "$k"
done
