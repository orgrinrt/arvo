#!/usr/bin/env bash
# Decimal-shaped bias compositions against file 53's two priced controls.
# Build shape identical to 36/41/42/53: rustc --edition 2021 --crate-type lib --emit=metadata
# Min-of-3 per point.
set -uo pipefail
cd "$(dirname "$0")"
mkdir -p gen out
echo "kind,count,ms_run1,ms_run2,ms_run3,ms_min,status" > bias_results.csv
for kind in dyadic distinct16 dec_quantum dec_slope dec_wide; do
  for count in 0 20 40; do
    src="gen/bias_${kind}_${count}.rs"
    python3 bias_gen.py "$kind" "$count" "$src"
    ts=(); rc=0
    for i in 1 2 3; do
      t0=$(python3 -c 'import time;print(int(time.time()*1000))')
      rustc --edition 2021 --crate-type lib --emit=metadata "$src" --out-dir out >/dev/null 2>&1 || rc=1
      t1=$(python3 -c 'import time;print(int(time.time()*1000))')
      ts+=($((t1-t0)))
    done
    m=${ts[0]}; for t in "${ts[@]}"; do [ "$t" -lt "$m" ] && m=$t; done
    st=ok; [ $rc -ne 0 ] && st=REFUSED
    echo "${kind},${count},${ts[0]},${ts[1]},${ts[2]},${m},${st}" | tee -a bias_results.csv
  done
done
