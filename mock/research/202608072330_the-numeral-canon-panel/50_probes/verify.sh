#!/usr/bin/env bash
# Rebuild and rerun every probe in 50_probes/ from committed source, on the pinned toolchain.
#
#   ./verify.sh
#
# Two files are EXPECTED TO FAIL TO COMPILE. Their committed .err is the result.
set -u
cd "$(dirname "$0")"
mkdir -p bin

RUSTC="rustc +nightly-2026-05-28 --edition 2021"
echo "toolchain: $(rustc +nightly-2026-05-28 --version)"
echo "python:    $(python3 --version)"
echo "feature gates in this directory: $(grep -c '^#!\[feature' *.rs | grep -v ':0' | wc -l | tr -d ' ') files with any"
echo

echo "=== p1 the criterion as a fixpoint equation, solved exhaustively (python) ==="
python3 p1_criterion_fixpoints.py > /tmp/p1.$$ && diff -q /tmp/p1.$$ p1_criterion_fixpoints.out \
  && echo "  reproduces committed .out" || echo "  DIFFERS from committed .out"
rm -f /tmp/p1.$$

echo "=== p2 Reading A loses the declared width (rust, compiles) ==="
$RUSTC -O p2_reading_a_loses_the_width.rs -o bin/p2 && ./bin/p2 > /tmp/p2.$$ \
  && diff -q /tmp/p2.$$ p2_reading_a_loses_the_width.out \
  && echo "  reproduces committed .out" || echo "  DIFFERS from committed .out"
rm -f /tmp/p2.$$

echo "=== p3 a site recomputing the stride, and what an open strategy set does to it (rust, compiles) ==="
$RUSTC -O p3_site_recomputes_the_stride.rs -o bin/p3 && ./bin/p3 > /tmp/p3.$$ \
  && diff -q /tmp/p3.$$ p3_site_recomputes_the_stride.out \
  && echo "  reproduces committed .out" || echo "  DIFFERS from committed .out"
rm -f /tmp/p3.$$

echo "=== p4 the access width is keyed on the stride, not on the width alone (python) ==="
python3 p4_access_width_is_keyed_on_the_stride.py > /tmp/p4.$$ \
  && diff -q /tmp/p4.$$ p4_access_width_is_keyed_on_the_stride.out \
  && echo "  reproduces committed .out" || echo "  DIFFERS from committed .out"
rm -f /tmp/p4.$$

echo "=== p5 three facts, two slots, both assignments (rust, compiles) ==="
$RUSTC -O p5_three_facts_two_slots.rs -o bin/p5 && ./bin/p5 > /tmp/p5.$$ \
  && diff -q /tmp/p5.$$ p5_three_facts_two_slots.out \
  && echo "  reproduces committed .out" || echo "  DIFFERS from committed .out"
rm -f /tmp/p5.$$

echo "=== p5b EXPECTED REFUSAL: the negative controls for p5 (rust, must not compile) ==="
if $RUSTC --crate-type lib p5b_negctl_three_facts.rs 2> /tmp/p5b.$$; then
  echo "  UNEXPECTED: it compiled. the result is void."
else
  echo "  refused, as expected. errors:"
  grep -c '^error' /tmp/p5b.$$ | sed 's/^/    grep ^error count (includes the aborting line): /'
  grep -c 'generic_const_exprs' /tmp/p5b.$$ | sed 's/^/    naming the forbidden generic_const_exprs: /'
  grep -c 'SameType<u32>' /tmp/p5b.$$ | sed 's/^/    the non-vacuity control (u16: SameType<u32>): /'
  diff -q /tmp/p5b.$$ p5b_negctl_three_facts.err \
    && echo "    reproduces committed .err" || echo "    DIFFERS from committed .err"
fi
rm -f /tmp/p5b.$$

echo "=== p6 what each Precise design admits, by chain length (python) ==="
python3 p6_precise_fork_is_not_a_fork.py > /tmp/p6.$$ \
  && diff -q /tmp/p6.$$ p6_precise_fork_is_not_a_fork.out \
  && echo "  reproduces committed .out" || echo "  DIFFERS from committed .out"
rm -f /tmp/p6.$$

echo "=== p7 readout of a committed bench (python, reads mock/benches/, runs nothing) ==="
python3 p7_bench_readout.py > /tmp/p7.$$ \
  && diff -q /tmp/p7.$$ p7_bench_readout.out \
  && echo "  reproduces committed .out" || echo "  DIFFERS from committed .out"
rm -f /tmp/p7.$$

echo
echo "=== feature gates, per file (all should be 0) ==="
grep -c '^#!\[feature' *.rs
