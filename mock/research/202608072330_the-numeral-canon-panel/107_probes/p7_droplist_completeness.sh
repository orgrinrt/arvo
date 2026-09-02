#!/usr/bin/env bash
# p7: 106 section 13.2 claims twelve options living in exactly one member file,
# and the brief asks the check to test whether that LIST IS COMPLETE rather
# than whether its entries are right.
#
# Method: take substantive one-file results from the members that are
# licence-shaped, bound-shaped or consequence-shaped, and grep 106 for any
# trace of each. A hit means carried somewhere; zero hits means dropped.
set -u
cd "$(dirname "$0")/.." || exit 1
T=106_giesen_consolidation_the_strategy_axis.md

probe() {
  local label="$1"; shift
  local n=0
  for pat in "$@"; do
    n=$(( n + $(grep -ciE "$pat" "$T") ))
  done
  if [ "$n" -eq 0 ]; then printf "  DROPPED  %s\n" "$label"
  else printf "  carried  %-62s (%s hits)\n" "$label" "$n"; fi
}

echo "=== is it twelve? counting the bold leads in 106 section 13.2 ==="
awk '/^### 13\.2/{f=1;next} /^### 13\.3/{f=0} f' "$T" | grep -c '^\*\*'

echo
echo "=== one-file results from the members, tested against 106 ==="

echo "-- from 97 --"
probe "F-H: a declared non-negative operand window RECOVERS three laws at signed saturating" \
      "operand window" "non-negative operand" "declared window"
probe "F-B: the gap is polynomial against exponential in the region count" \
      "polynomial" "hyperplane" "exponential in the number of regions"
probe "F-A2: rationalisability decidable at ANY number of cost dimensions" \
      "extreme ray" "extreme-ray"
probe "F-G: at F=0 unsigned saturating, distributivity over SUBTRACTION fails" \
      "mul_over_sub" "over subtraction" "45\.79"

echo "-- from 98 --"
probe "F-98-7: a hard bound on a measurement has NO weighting" \
      "hard bound" "threshold" "subject to"
probe "the five-rung ladder L0..L5, and L1 = L2 on structured data" \
      "ladder" "L0|L1|L2|L4|L5" "order-rationalis"
probe "F-98-5: the L4/L1 ratio varies 47x, so a value of it is a fact about ONE table" \
      "47x" "fact about one table" "varies by a factor"
probe "F-98-4: the union-graph acyclicity criterion for the order rung" \
      "union graph" "union-graph" "acyclic"
probe "F-98-1: 144 Pareto-admissible sections, the rung between L0 and L3" \
      "144" "Pareto-admissible"
probe "F-98-13: select the section by resample agreement, not the point estimate" \
      "modal section" "resample agreement" "most resamples agree"

echo "-- from 93 --"
probe "F1a: the F = 0 half extends to ANY width by proof rather than by sweep" \
      "F1a" "semiring congruence" "any width"
probe "F11 / P8: Q41 answered, with signedness as the separating predicate" \
      "Q41" "honoured law" "law sets nest"
probe "Q14 collapse: lexicographic and a stated rate are one option at two values" \
      "lexicographic" "priority order" "exchange rate.*infinite"
probe "section 2's six axes, including the five it does name" \
      "six axes" "five axes" "divergence from a reference"

echo "-- from 94 --"
probe "W4/W5: the retraction verdict differs PER OPERATION PAIR, 6 of 9 for saturation" \
      "retract" "add>sub" "per operation pair"
probe "R1/R2/R3 and W8" "W8" "ulp bound" "R3"

echo "-- from 101 --"
probe "F-101-2: 0 of 94 implement score_output; 82 of 82 call sites use timed!" \
      "timed!" "score_output"
probe "the six reachable dead columns and the distance to each" \
      "setup_ns" "first_ns" "reachable"

echo "-- from 102 --"
probe "F-102-7: rounding to nearest costs ONE hoisted instruction on aarch64" \
      "madd" "hoisted"

echo
echo "=== for contrast: things 106 does carry, as a control on the instrument ==="
probe "CONTROL the free join semilattice (should be carried)" "free join semilattice"
probe "CONTROL the 0.273% control-pair floor (should be carried)" "0\.273"
probe "CONTROL the 175/79 pre-wiring split (should be carried)" "175 of 254|175 before"
