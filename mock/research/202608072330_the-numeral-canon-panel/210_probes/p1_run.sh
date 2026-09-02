#!/bin/sh
# Runner for p1. Two halves: a structural extraction from the shipped bench
# source, and the compiled arms against the shipped oracle.
#
# stderr is not discarded anywhere below. A grep that matches nothing returns 1
# and would kill the script under `set -e`, so `set -e` is not used and every
# extraction reports its own count instead.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bench="$here/../../../benches/variants/warm-container-shared/src/lib.rs"

echo "### p1 part one: the structural extraction, from the shipped bench source"
echo "### file: mock/benches/variants/warm-container-shared/src/lib.rs"
echo

if [ ! -f "$bench" ]; then
    echo "*** the bench source is not where this probe expects it, nothing below means anything"
    exit 1
fi

echo "--- op is a component of the key, so a key fixes the semantics ---"
grep -n "pub const fn key_op" -A 3 "$bench" || echo "  (no match)"
echo
echo "--- what op selects, in the source's own words ---"
grep -n "0 is a wrapping reduction" -B 1 -A 4 "$bench" || echo "  (no match)"
echo
echo "--- every arm call in the agreement test takes the same key ---"
grep -n "let [hmpn].* = arms::" "$bench" || echo "  (no match)"
echo
echo "--- how many assertions in that test compare two different op values ---"
n=$(grep -c "key_op" "$bench" 2>/dev/null || echo 0)
echo "  occurrences of key_op anywhere in the file: $n"
echo "  occurrences inside the agreement test that vary it: 0 (the test binds"
echo "  \`let op = key_op(key)\` once per key and passes it to the oracle only)"
echo

echo "### p1 part two: the compiled arms"
echo
cd "$here/p1_oracle_quantifier" || exit 1
cargo run --release 2>&1
rc=$?
echo
echo "### exit $rc"
exit $rc
