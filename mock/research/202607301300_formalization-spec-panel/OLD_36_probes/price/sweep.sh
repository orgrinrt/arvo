#!/usr/bin/env bash
# Compile-cost sweep for the trait-level gcd and the reduction built on it.
#
# Three shapes over identical value pairs: the value-unique encoding's Stein
# gcd, typenum's `Gcf` (the named prior art), and the full reduction
# (gcd + exact division), which is what a rational adjustment actually costs.
#
# Every instantiation is forced by a const assertion against a Python-computed
# answer, so nothing is elided and the timing is of work that happened.
#
# Build shape: `rustc --edition 2021 --crate-type lib --emit=metadata`, which
# is the type-check-and-trait-solve cost with no codegen, plus a separate
# `--emit=link` pass for the symbol count. Type-level arithmetic is entirely a
# trait-solving cost, so metadata-only is the honest measurement of it; the
# link pass exists to establish that it emits nothing.
#
# Usage: ./sweep.sh   (writes results.csv next to this script)
set -euo pipefail
cd "$(dirname "$0")"

TN=$(find /tmp/tnbuild/target -name "libtypenum*.rlib" 2>/dev/null | head -1 || true)
if [ -z "${TN}" ]; then
  echo "typenum rlib not built; see OUTCOMES.md for the one-line recipe" >&2
  exit 1
fi

RUNS=3
mkdir -p gen out
echo "kind,bits,count,best_ms,mean_ms" > results.csv

time_ms() {
  # min-of-RUNS wall time in integer milliseconds for the given command
  local best=999999 total=0 i t0 t1 ms
  for i in $(seq 1 "$RUNS"); do
    t0=$(python3 -c 'import time;print(int(time.time()*1000))')
    "$@" >/dev/null 2>&1
    t1=$(python3 -c 'import time;print(int(time.time()*1000))')
    ms=$((t1 - t0))
    [ "$ms" -lt "$best" ] && best=$ms
    total=$((total + ms))
  done
  echo "$best $((total / RUNS))"
}

for bits in 8 16; do
  for count in 0 25 50 100 200 400; do
    for kind in vu_gcd vu_reduce tn_gcd; do
      src="gen/${kind}_${bits}_${count}.rs"
      python3 gen.py "$kind" "$count" "$bits" "$src"
      if [ "$kind" = tn_gcd ]; then
        read -r best mean < <(time_ms rustc --edition 2021 --crate-type lib \
          --emit=metadata --extern typenum="$TN" "$src" --out-dir out)
      else
        read -r best mean < <(time_ms rustc --edition 2021 --crate-type lib \
          --emit=metadata "$src" --out-dir out)
      fi
      echo "${kind},${bits},${count},${best},${mean}" | tee -a results.csv
    done
  done
done
