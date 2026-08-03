#!/usr/bin/env bash
# File 46's price check: does sealing Adjustment (one private trait plus
# one blanket impl whose bound duplicates the Adjustment impl's own) cost
# anything over file 42's sealed tower? Same methodology as
# 42_probes/price (min-of-1, 8-bit operands, --emit=metadata, baseline at
# count 0 subtracted), two kinds: alias_sealed (42's tower, the baseline)
# and alias_sealed_adj (this file's tower). Scope honestly stated:
# min-of-1, 8-bit only, two counts (0, 400); a slope over one non-zero
# count is a difference quotient, not a fit.
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
for count in 0 400; do
  for kind in alias_sealed alias_sealed_adj; do
    src="gen/${kind}_8_${count}.rs"
    python3 gen.py "$kind" "$count" 8 "$src"
    ms=$(time_ms rustc --edition 2021 --crate-type lib --emit=metadata "$src" --out-dir out)
    echo "$kind,8,$count,$ms" >> results.csv
    echo "$kind count=$count: ${ms} ms"
  done
done
