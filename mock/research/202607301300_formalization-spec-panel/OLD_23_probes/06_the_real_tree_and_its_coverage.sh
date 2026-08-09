#!/bin/bash
# PROBE 6: the cost and the coverage, on the real arvo tree rather than a model.
#
# Three numbers decide whether any of this is worth building: what a check build
# costs, what the scan costs, and how much of the design it can see. The third
# is the one that bounds the whole idea, and it is not a cost, it is a ceiling.
set -u
cd "$(dirname "$0")"
MOCK=../../../../mock
N="+nightly-2026-05-28"
T=/tmp/23_arvo_all

echo "=== 1. the check build, all targets, cold ==="
# Debug, not release: probe 8 measures that `--release --all-targets` does not
# compile, because Strategy::NAME is cfg(debug_assertions) and
# crates/arvo/tests/cross_width.rs:20 uses it. A check build is not a shipping
# build, so debug is the right profile here anyway.
rm -rf $T
( cd $MOCK && CARGO_TARGET_DIR=$T \
  RUSTFLAGS="--emit=llvm-ir -Cno-prepopulate-passes -Zinline-mir=no" \
  /usr/bin/time -p cargo $N build --all-targets -q ) 2>&1 | grep -E '^(real|user)'
echo "  IR emitted: $(find $T -name '*.ll' -exec cat {} + | wc -c | tr -d ' ') bytes"

echo
echo "=== 2. what is in it ==="
IR() { find $T -name '*.ll' -exec cat {} + ; }
echo "  generic monomorphisations:            $(IR | grep -c '^define[^@]*@_RIN')"
echo "  carrying an arvo strategy marker:     $(IR | grep -o '^define[^@]*@_R[A-Za-z0-9_.$]*' | grep -cE '(3Hot|4Warm|4Cold|7Precise)')"
echo "  by marker:"
for m in 3Hot 4Warm 4Cold 7Precise; do
    printf '    %-9s %s\n' "$m" "$(IR | grep -o '^define[^@]*@_R[A-Za-z0-9_.$]*' | grep -oE "$m" | wc -l | tr -d ' ')"
  done

echo
echo "=== 3. the scan, over all of it ==="
/usr/bin/time -p sh -c "find $T -name '*.ll' -exec cat {} + | python3 05_the_axis_that_generated_nothing.py --scope arvo_ > /dev/null" 2>&1 | grep -E '^(real|user)'

echo
echo "=== 4. the ceiling ==="
echo "  A verifier sees the compositions a build INSTANTIATES. That is roughly"
echo "  700 here, and it is not a property of arvo, it is a property of what"
echo "  and every one exists because a test or a bench constructed it. The"
echo "  release library build alone contains FOUR. The spec's quantisation axis alone"
echo "  is 6*6*6 directions by 9*9 range resolutions, which is 17496 instances"
echo "  before any other axis is chosen. The verifier's coverage is"
echo "  test-suite-shaped, never type-system-shaped, and no amount of work on"
echo "  the tool changes that."

echo
echo "=== 5. and one real symbol, to show the whole composition survives ==="
IR | grep -o '^define[^@]*@_R[A-Za-z0-9_.$]*' | grep '6UFixed' | sed 's/.*@//' |
  sort -u | head -1 | fold -w 100 | sed 's/^/  /'
echo "  Const arguments of ADT const-param type are in there structurally, with"
echo "  their newtype names. So are backreferences (B3_, B5_), which is why a"
echo "  real reader is rustc-demangle and a tree walk, never a regex."
