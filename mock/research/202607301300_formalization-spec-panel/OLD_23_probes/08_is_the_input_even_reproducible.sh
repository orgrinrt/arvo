#!/bin/bash
# PROBE 8: is the verifier's input reproducible, and the false alarm on the way.
#
# Everything in probes 1 through 6 assumes that reading a check build tells you
# something about the source. This asks the prior question: three identical cold
# builds of unchanged source, do they contain the same monomorphisations.
#
# The first attempt measured `--all-targets` and found large variance (1490 to
# 1629 strategy-carrying symbols, sets differing by 675 lines) and I nearly
# reported it as a property of codegen-unit partitioning, with a conclusion
# about gating hanging off it. It is not that. That build FAILS, exit 101, and
# the variance is a parallel build racing to different stopping points before
# the failure aborts the rest. Part 1 shows the failure. Part 2 asks the
# question again on a build that succeeds.
set -u
cd "$(dirname "$0")/../../.." || exit 1   # arvo/mock
[ -f Cargo.toml ] || { echo "not in arvo/mock, refusing to report on nothing"; exit 1; }

echo "=== 1. the release all-targets build does not compile ==="
CARGO_TARGET_DIR=/tmp/23p8_at cargo +nightly-2026-05-28 build --release --all-targets \
  > /dev/null 2>/tmp/23p8_err.txt
echo "  exit: $?"
grep -m2 -A2 '^error' /tmp/23p8_err.txt | sed 's/^/    /'
echo
echo "  The cause is a deliberate design choice, not a regression:"
grep -n -B1 'const NAME: ' crates/arvo-strategy/src/lib.rs | head -4 | sed 's/^/    /'
echo "  NAME is gated to debug for an empty .rodata in release, and"
echo "  crates/arvo/tests/cross_width.rs:20 uses it. That test target exists in"
echo "  debug and not in release, cargo test runs debug and stays green at 654"
echo "  passed, and nothing in the suite says the release profile does not build."
rm -rf /tmp/23p8_at

echo
echo "=== 2. the same question on a build that succeeds ==="
for run in 1 2 3; do
  T=/tmp/23p8_$run
  rm -rf $T
  CARGO_TARGET_DIR=$T RUSTFLAGS="--emit=llvm-ir -Cno-prepopulate-passes -Zinline-mir=no" \
    cargo +nightly-2026-05-28 build --release -q 2>/dev/null
  st=$?
  find $T -name '*.ll' -exec cat {} + | grep -o '^define[^@]*@_R[A-Za-z0-9_.$]*' |
    sed 's/.*@//' | sed 's/Cs[0-9A-Za-z]*_//g' | sort > /tmp/23p8_s$run.txt
  printf '  run %s  exit=%s files=%-4s defines=%-6s generic=%s\n' "$run" "$st" \
    "$(find $T -name '*.ll' | wc -l | tr -d ' ')" \
    "$(wc -l < /tmp/23p8_s$run.txt | tr -d ' ')" \
    "$(grep -c '^_RIN' /tmp/23p8_s$run.txt)"
  rm -rf $T
done
echo "  1 vs 2 differing lines: $(diff /tmp/23p8_s1.txt /tmp/23p8_s2.txt | grep -c '^[<>]')"
echo "  1 vs 3 differing lines: $(diff /tmp/23p8_s1.txt /tmp/23p8_s3.txt | grep -c '^[<>]')"
echo
echo "  Zero differences means the input IS reproducible and a scanner over it"
echo "  can gate. Nonzero means it cannot, and the one-line assertion in probe 7"
echo "  is the only shape that can, because it names what it compares."
