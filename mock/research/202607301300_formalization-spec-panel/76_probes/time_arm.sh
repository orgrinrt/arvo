#!/bin/sh
# Time one arm at one consumer profile. Median of three, wall clock.
#
# Run from inside the repository tree so `rust-toolchain.toml` resolves. A bare
# `rustc` outside the tree picks up stable and reports E0554 on every gate,
# which invalidated one member's results until they caught it.
#
#   ./time_arm.sh <arm> <numerals> <capacities> [ceiling] [emit]
#
# emit defaults to `metadata`, which is what a consumer's `cargo check` pays.
# Pass `link` for the full codegen figure.
set -e
ARM=$1; N=$2; M=$3; CEIL=${4:-64}; EMIT=${5:-metadata}
HERE=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$HERE/gen"
SRC="$HERE/gen/arm_${ARM}_${N}_${M}_${CEIL}.rs"
python3 "$HERE/gen_consumer.py" "$ARM" "$N" "$M" "$CEIL" > "$SRC"
BYTES=$(wc -c < "$SRC" | tr -d ' ')
OUT="$HERE/out"
mkdir -p "$OUT"
BEST=""
for _ in 1 2 3; do
  S=$(python3 -c 'import time;print(time.time())')
  rustc --edition 2024 --crate-type=lib --emit="$EMIT" --out-dir "$OUT" "$SRC" 2>/dev/null
  E=$(python3 -c 'import time;print(time.time())')
  T=$(python3 -c "print(f'{($E-$S)*1000:.0f}')")
  BEST="$BEST $T"
done
MED=$(echo "$BEST" | tr ' ' '\n' | grep -v '^$' | sort -n | sed -n 2p)
echo "$ARM,$N,$M,$CEIL,$EMIT,$BYTES,$MED,$(echo $BEST | tr ' ' '/')"
# the generated source stays: it is the artifact the number was taken on.
