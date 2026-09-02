#!/bin/bash
# PROBE 2 runner: four builds of one model, two checkers, one table.
set -u
cd "$(dirname "$0")"
N="+nightly-2026-05-28"
RS=02_declared_against_generated.rs
CHECK="-O -Cno-prepopulate-passes -Zinline-mir=no -Cpanic=abort"

echo "The model declares two axes. Three cfg arms plant one defect each, and"
echo "none of them changes a type, a bound, or a declared axis. All four compile."
echo
printf '%-14s %-38s %s\n' "build" "03, rules over instructions" "05, rule-free"
for cfg in "" "--cfg underclaim" "--cfg overclaim" "--cfg swapped"; do
  tag=$(echo "${cfg:-clean}" | tr -d ' -')
  ll=/tmp/23p2_$tag.ll
  rustc $N $CHECK $cfg --emit=llvm-ir --crate-type=lib $RS -o $ll 2>/dev/null
  a=$(python3 03_the_whole_verifier.py --depth 1 --scope _02 < $ll |
      grep -oE '(UNDER|OVER)-CLAIM \([0-9]+\)' | tr '\n' ' ')
  b=$(python3 05_the_axis_that_generated_nothing.py < $ll | grep -c 'CHANGES NOTHING')
  printf '%-14s %-38s %s inert axes\n' "$tag" "$a" "$b"
done

echo
echo "=== what each catches, stated ==="
echo "  underclaim  dot fuses whatever Policy says.   both catch it."
echo "  overclaim   load ignores Layout entirely.     both catch it."
echo "  swapped     the liberty goes to Strict.       only the rules catch it,"
echo "              because the axis still changes generated code."
echo
echo "=== full report on the one the cheap check misses ==="
python3 03_the_whole_verifier.py --depth 1 --scope _02 < /tmp/23p2_cfgswapped.ll
echo
python3 05_the_axis_that_generated_nothing.py < /tmp/23p2_cfgswapped.ll

echo
echo "=== the two flags, on this model ==="
rustc $N -O -Cno-prepopulate-passes -Cpanic=abort --emit=llvm-ir --crate-type=lib $RS -o /tmp/23p2_onlyone.ll 2>/dev/null
printf '  with -Cno-prepopulate-passes alone: %s dot/load defines\n' \
  "$(grep -cE '^define.*(3dot|4load)' /tmp/23p2_onlyone.ll)"
printf '  with -Zinline-mir=no as well:       %s\n' \
  "$(grep -cE '^define.*(3dot|4load)' /tmp/23p2_clean.ll)"
echo "  No difference here, because both operations are too large for rustc's"
echo "  own MIR inliner. Probe 1 measures a small operation, where the same"
echo "  comparison is 0 against 12: rustc inlines before LLVM is handed"
echo "  anything, so -Cno-prepopulate-passes alone is not the check build."
