#!/usr/bin/env bash
set -uo pipefail
cd "$(dirname "$0")"
mkdir -p gen out
for kind in dec_quantum6; do
  for count in 0 15 30; do
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
