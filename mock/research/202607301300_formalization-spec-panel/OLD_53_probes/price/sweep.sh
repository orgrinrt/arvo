#!/usr/bin/env bash
# Aggregate compile-cost sweep for file 53. Min-of-2 per point (better
# than 41/42's min-of-1, still short of file 36's min-of-3; wall-clock
# budget). Build shape identical to 36/41/42's sweeps:
#   rustc --edition 2021 --crate-type lib --emit=metadata
# Toolchain: resolved from the repo pin (verified 1.98.0-nightly
# 57d06900f from this directory before the run; the file-52 gotcha of a
# bare rustc resolving to stable applies only OUTSIDE the repo tree).
set -euo pipefail
cd "$(dirname "$0")"

mkdir -p gen out
echo "kind,count,ms_run1,ms_run2,ms_min" > results.csv

time_ms() {
  local t0 t1
  t0=$(python3 -c 'import time;print(int(time.time()*1000))')
  "$@" >/dev/null 2>&1
  t1=$(python3 -c 'import time;print(int(time.time()*1000))')
  echo $((t1 - t0))
}

run_point() {
  local kind=$1 count=$2
  local src="gen/${kind}_${count}.rs"
  python3 gen.py "$kind" "$count" "$src"
  local a b m
  a=$(time_ms rustc --edition 2021 --crate-type lib --emit=metadata "$src" --out-dir out)
  b=$(time_ms rustc --edition 2021 --crate-type lib --emit=metadata "$src" --out-dir out)
  m=$(( a < b ? a : b ))
  echo "${kind},${count},${a},${b},${m}" | tee -a results.csv
}

for count in 0 25 50 100 200; do run_point dyadic "$count"; done
run_point distinct16 5
run_point distinct16 50
run_point distinct16 100
run_point repeat16 100
run_point headline 3
run_point chained 1
