#!/usr/bin/env bash
# --emit=metadata sweeps, same shape as files 36/41/42/46: two counts, min of 3,
# difference quotient. Not a bench; a compile-time fact on the pin + host.
set -u
cd "$(dirname "$0")"
TOWER=../libtower.rlib
run() {
  local kind=$1 n=$2 f=out_${1}_${2}.rs
  python3 gen.py "$kind" "$n" > "$f"
  local best=999999
  for i in 1 2 3; do
    local t0=$(python3 -c 'import time;print(time.time())')
    rustc --edition 2021 --crate-type lib --emit=metadata --extern tower="$TOWER" \
      -o "meta_${kind}_${n}.rmeta" "$f" 2>/dev/null || { echo "$kind $n COMPILE FAILED"; return 1; }
    local t1=$(python3 -c 'import time;print(time.time())')
    local ms=$(python3 -c "print(($t1-$t0)*1000)")
    best=$(python3 -c "print(min($best,$ms))")
  done
  local bytes=$(wc -c < "meta_${kind}_${n}.rmeta")
  echo "$kind $n ${best} $bytes"
}
for kind in alias_table alias_table_bare grade_projected grade_declared; do
  run "$kind" 0
  run "$kind" 400
done
