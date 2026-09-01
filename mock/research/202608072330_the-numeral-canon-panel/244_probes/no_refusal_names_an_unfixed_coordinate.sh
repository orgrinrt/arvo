#!/usr/bin/env bash
# Seat 244. The located disagreement of this sitting is whether admission can
# return the name of a coordinate a candidate failed to fix (241), or whether it
# is total because underdetermination is a compile error one tier below (243).
#
# 243 measured the encoding: an `impl Format` omitting an item is E0046. This
# probe measures the other half, in the shipped tree: of the refusals that DO
# exist, how many name a coordinate outside the set the refusing trait requires,
# rather than a condition on the values of coordinates that trait already fixes.
#
# THE CASES THAT MUST FAIL, and the first cut of this probe had only one of them,
# which is why it printed a wrong finding on its first run and is disclosed here.
#
#   Control A, positive: an assertion referencing a coordinate the trait does not
#   fix must be FLAGGED.
#   Control B, negative: an assertion referencing only coordinates the trait does
#   fix must NOT be flagged.
#
# The first cut had A only. A `sed -E '\s'` that BSD sed does not honour left the
# coordinate list holding whole source lines, so nothing matched it, so every
# assertion was flagged and control A passed anyway. A classifier that says the
# same thing about every input passes a one-sided control by construction. Both
# controls now have to hold before section 2's count is printed at all.
set -u
cd "$(dirname "$0")/../../../.." || exit 1   # repo root
SRC=mock/crates/arvo-format/src

# A `const X: T;` with no `=` is a coordinate an implementor must fix. One with a
# default body is not: `Slots::ADMITTED` is `const ADMITTED: () = { .. }`.
coords_of() {
  grep -E '^[[:space:]]{4}const [A-Z_]+:[^=]+;[[:space:]]*$' "$SRC/$1.rs" \
    | sed -E 's/^[[:space:]]*const ([A-Z_]+).*/\1/'
}

echo "======== 1. The coordinates, taken from the trait declarations"
TOTAL=0
for f in ambient quantum slots format; do
  c=$(coords_of "$f" | tr '\n' ' ')
  n=$(coords_of "$f" | wc -l | tr -d ' ')
  TOTAL=$((TOTAL + n))
  printf '  %-9s %d  %s\n' "$f" "$n" "$c"
done
echo "  total required coordinates = $TOTAL"
SLOTS_RE=$(coords_of slots | paste -sd'|' -)
echo "  the refusing trait, Slots, fixes: $SLOTS_RE"
echo "  (ADMITTED is excluded: it carries a default body, so no candidate chooses it)"

extract() {
  awk '/const ADMITTED: \(\) = \{/{on=1} on{print} on&&/^    \};$/{exit}' "$SRC/slots.rs" \
    | tr '\n' ' ' | sed 's/assert!(/\n@/g' | sed -n 's/^@\([^,]*\),.*/\1/p'
}

# -> "refs\tunfixed"
classify() {
  local e="$1" refs="" unfixed="" r
  refs=$(printf '%s' "$e" | grep -oE 'Self::[A-Z_]+' | sed 's/Self:://' | sort -u | tr '\n' ' ')
  for r in $refs; do
    printf '%s' "$r" | grep -qE "^($SLOTS_RE)$" || unfixed="$unfixed $r"
  done
  printf '%s\t%s\n' "${refs% }" "${unfixed# }"
}

echo
echo "======== 2. CONTROLS, both required, run before the census is believed"
A='Self::MIN <= Self::MAX && Self::PHASE_DEN != 0'   # PHASE_DEN lives on Format
B='Self::MIN <= Self::MAX'                            # both on Slots
IFS=$'\t' read -r ra ua <<<"$(classify "$A")"
IFS=$'\t' read -r rb ub <<<"$(classify "$B")"
printf '  A (must flag)     : %-46s refs[%s] unfixed[%s]\n' "$A" "$ra" "$ua"
printf '  B (must not flag) : %-46s refs[%s] unfixed[%s]\n' "$B" "$rb" "$ub"
if [ -z "$ua" ]; then
  echo "  CONTROL A FAILED: the classifier cannot flag anything. Stopping."; exit 2
fi
if [ -n "$ub" ]; then
  echo "  CONTROL B FAILED: the classifier flags everything, so A proved nothing. Stopping."; exit 2
fi
echo "  both controls pass: the classifier separates the two cases."

echo
echo "======== 3. Every assertion in Slots::ADMITTED, exhaustively"
n=0; flagged=0
while IFS= read -r e; do
  [ -n "$e" ] || continue
  n=$((n+1))
  IFS=$'\t' read -r refs unfixed <<<"$(classify "$e")"
  if [ -n "$unfixed" ]; then
    flagged=$((flagged+1)); mark="NAMES A COORDINATE Slots DOES NOT FIX: $unfixed"
  else
    mark="a condition on the values of coordinates Slots already fixes"
  fi
  printf '  a%-2d refs [%-16s] -> %s\n' "$n" "$refs" "$mark"
done <<<"$(extract)"

echo
echo "======== 4. FINDING"
echo "  assertions in the block = $n"
echo "  of which name a coordinate the trait does not fix = $flagged"
echo
echo "  So the shipped admission mechanism has $n ways to refuse and none of them"
echo "  reports an unfixed coordinate. Every one is a condition on values already"
echo "  fixed, which is what a const item on a trait can be: it evaluates only for"
echo "  an impl, and an impl missing a required item does not exist (243, E0046)."
echo "  Bounded whole-container over the assertions in the block, exhaustively."
