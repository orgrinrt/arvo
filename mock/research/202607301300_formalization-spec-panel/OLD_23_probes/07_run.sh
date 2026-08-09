#!/bin/bash
# PROBE 7 runner: the inert-axis check with no tool at all, and the exact
# optimisation levels at which it is valid.
set -u
cd "$(dirname "$0")"
N="+nightly-2026-05-28"
RS=07_no_tool_at_all.rs

echo "One assertion, in the ordinary suite, on the ordinary build:"
echo
echo "    assert_ne!(dot::<Strict> as usize, dot::<Relaxed> as usize);"
echo
echo "It passes when the axis changes generated code and fails when it does"
echo "not, because two monomorphisations that compile to identical code get"
echo "folded to one address. Where that folding runs is measured, not assumed."
echo
printf '%-26s %-24s %s\n' "flags" "live axis: one address?" "inert axis: one address?"
while IFS= read -r fl; do
  a=$(rustc $N $fl $RS -o /tmp/23p7a 2>/dev/null && /tmp/23p7a | grep 'same address' | grep -oE '(true|false)')
  b=$(rustc $N $fl --cfg inert $RS -o /tmp/23p7b 2>/dev/null && /tmp/23p7b | grep 'same address' | grep -oE '(true|false)')
  printf '%-26s %-24s %s\n' "$fl" "$a" "$b"
done <<EOF
-Copt-level=0
-Copt-level=1
-Copt-level=2
-Copt-level=3
-Copt-level=3 -Clto=fat
-Copt-level=s
EOF

echo
echo "Correct from -Copt-level=2 upward, including under fat LTO and under -Os."
echo "Wrong below it, in the safe direction only: at 0 and 1 an inert axis reads"
echo "as live, so the assertion is gated to a release-profile test rather than"
echo "trusted everywhere. It never reports a live axis as inert at any level."
echo
echo "=== the variant that looks cruder and is actually unusable ==="
printf '%-16s %-38s %s\n' "flags" "live: first 128 bytes" "inert: first 128 bytes"
for fl in -Copt-level=0 -Copt-level=1 -Copt-level=2 -Copt-level=3; do
  a=$(rustc $N $fl $RS -o /tmp/23p7a 2>/dev/null && /tmp/23p7a | sed -n 6p)
  b=$(rustc $N $fl --cfg inert $RS -o /tmp/23p7b 2>/dev/null && /tmp/23p7b | sed -n 6p)
  printf '%-16s %-38s %s\n' "$fl" "$(echo "$a" | cut -c1-36)" "$(echo "$b" | cut -c1-36)"
done
echo
echo "Comparing the machine code directly is wrong in BOTH directions: at 0 it"
echo "calls a live axis inert, and at 1 it calls an inert axis live. The address"
echo "is the better observation precisely because the compiler computed it."
