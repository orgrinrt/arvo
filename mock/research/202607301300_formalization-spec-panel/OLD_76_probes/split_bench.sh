#!/bin/sh
# The figure a consumer actually recognises.
#
# The measurements in bench.sh compile the machinery and the consumer together,
# which is what a single generated file does and is NOT the consumer situation:
# arvo is a dependency, so its machinery is compiled once, in arvo, and a
# consumer crate pays only for its own instantiations against an already-built
# rlib.
#
# This splits the same generated program in two at the machinery boundary,
# builds the machinery crate once, and times ONLY the consumer crate against it.
#
#   ./split_bench.sh <arm> <numerals> <capacities> [ceiling] [runs]
set -e
ARM=$1; N=$2; M=$3; CEIL=${4:-64}; RUNS=${5:-5}
HERE=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$HERE/gen" "$HERE/out"
python3 "$HERE/gen_consumer.py" "$ARM" "$N" "$M" "$CEIL" > "$HERE/gen/whole_${ARM}.rs"
python3 "$HERE/split_consumer.py" "$HERE/gen/whole_${ARM}.rs" \
  "$HERE/gen/mach_${ARM}.rs" "$HERE/gen/user_${ARM}_n${N}_m${M}_c${CEIL}.rs"
rustc --edition 2024 --crate-type=lib --crate-name mach \
  --out-dir "$HERE/out" "$HERE/gen/mach_${ARM}.rs"
rustc --edition 2024 --crate-type=lib --emit=metadata --extern mach="$HERE/out/libmach.rlib" \
  --out-dir "$HERE/out" "$HERE/gen/user_${ARM}_n${N}_m${M}_c${CEIL}.rs"
hyperfine --warmup 2 --runs "$RUNS" --style none --export-json "$HERE/out/split_${ARM}.json" \
  "rustc --edition 2024 --crate-type=lib --emit=metadata --extern mach=$HERE/out/libmach.rlib --out-dir $HERE/out $HERE/gen/user_${ARM}_n${N}_m${M}_c${CEIL}.rs" > /dev/null
python3 - "$HERE/out/split_${ARM}.json" "$ARM" "$N" "$M" "$CEIL" <<'PY'
import json, sys
j = json.load(open(sys.argv[1]))["results"][0]
print(f"{sys.argv[2]},{sys.argv[3]},{sys.argv[4]},{sys.argv[5]},consumer-only,"
      f"{j['mean']*1000:.0f},{j['stddev']*1000:.0f},{j['min']*1000:.0f},{j['max']*1000:.0f}")
PY
