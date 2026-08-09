#!/bin/sh
# The consumer compile-cost measurement. One row per (arm, profile).
#
# Run from inside the repository tree so `rust-toolchain.toml` resolves. A bare
# `rustc` outside the tree resolves to stable (1.94.0 on this machine) and
# reports E0554 on every gate, which is a silent way to measure the wrong
# compiler entirely.
#
#   ./bench.sh <arm> <numerals> <capacities> [ceiling] [emit] [runs]
#
# `metadata` is what a consumer's `cargo check` pays and is the default.
# `link` at opt-level 0 is the `cargo build` figure.
#
# hyperfine reports mean +- sigma over `runs` timed runs after 2 warmups. The
# warmups matter: the first run pays filesystem cache costs that no steady-state
# consumer pays, and reporting it would flatter nothing and mislead everything.
set -e
ARM=$1; N=$2; M=$3; CEIL=${4:-64}; EMIT=${5:-metadata}; RUNS=${6:-5}
HERE=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$HERE/gen" "$HERE/out"
SRC="$HERE/gen/arm_${ARM}_n${N}_m${M}_c${CEIL}.rs"
python3 "$HERE/gen_consumer.py" "$ARM" "$N" "$M" "$CEIL" > "$SRC"
rustc --edition 2024 --crate-type=lib --emit="$EMIT" --out-dir "$HERE/out" "$SRC"
hyperfine --warmup 2 --runs "$RUNS" --style none \
  --export-json "$HERE/out/${ARM}_n${N}_m${M}_c${CEIL}_${EMIT}.json" \
  "rustc --edition 2024 --crate-type=lib --emit=$EMIT --out-dir $HERE/out $SRC" > /dev/null
python3 - "$HERE/out/${ARM}_n${N}_m${M}_c${CEIL}_${EMIT}.json" "$ARM" "$N" "$M" "$CEIL" "$EMIT" "$SRC" <<'PY'
import json, sys, os
j = json.load(open(sys.argv[1]))["results"][0]
arm, n, m, c, emit, src = sys.argv[2:8]
print(f"{arm},{n},{m},{c},{emit},{os.path.getsize(src)},"
      f"{j['mean']*1000:.0f},{j['stddev']*1000:.0f},{j['min']*1000:.0f},{j['max']*1000:.0f}")
PY
