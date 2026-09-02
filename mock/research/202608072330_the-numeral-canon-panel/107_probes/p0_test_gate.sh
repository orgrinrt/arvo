#!/usr/bin/env bash
# p0: the test gate, run per crate rather than taken from five files.
#
# It also found a FIFTH way this environment produces a meaningless green,
# beside the four 106 section 0.2 enumerates, and it caught me the same way
# 106's tail -4 caught 106:
#
#   `timeout` is not installed on this host. A runner written as
#   `timeout 600 cargo test ... | grep '^test result'` produces NO output and
#   exits 0, for all thirteen crates. It reads as a completed gate.
#
# And a sixth, milder: wide-rung-shared alone takes 107 seconds, so a batch
# runner under a two-minute cap reports twelve of thirteen and looks complete.
set -u
cd /Users/orgrinrt/Dev/clause-dev/arvo/mock/benches/variants || exit 1

echo "=== the fifth meaningless green: timeout is absent on this host ==="
command -v timeout >/dev/null && echo "  timeout present" || echo "  timeout NOT FOUND"
cd bitpack-shared || exit 1
echo -n "  'timeout 600 cargo test | grep ^test result' lines : "
timeout 600 cargo test -- --test-threads=1 2>/dev/null | grep -cE '^test result'
echo "  exit=$?  <- reads as a clean run of a crate that has three tests"
cd ..

echo
echo "=== the gate, per crate, with the runner that works ==="
tot=0
for d in *-shared; do
  r=$( cd "$d" && cargo test -- --test-threads=1 2>/dev/null | grep -E '^test result' | head -1 )
  n=$(printf '%s' "$r" | sed -n 's/.*ok\. \([0-9]*\) passed.*/\1/p')
  tot=$(( tot + ${n:-0} ))
  printf "  %-32s %s\n" "$d" "${r:-NO RESULT}"
done
echo "  ---"
echo "  total passed: $tot"
