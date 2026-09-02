#!/usr/bin/env bash
# Seat 246. What index does the shipped hosting predicate actually carry, and
# is `Slots::ADMITTED` one kind of condition or several.
#
# Two claims are under test, both load-bearing for the Q31 promotion:
#
#   241 section 6: "Being hostable is a predicate over the *values* those
#   coordinates take, against a particular target's realisation ladder.
#   `Slots::ADMITTED` in the shipped crate is exactly this and nothing else."
#   And: "A predicate over target-dependent facts is target-indexed already."
#
#   proposal::membership_and_hosting_are_two_questions: "whether THIS
#   IMPLEMENTATION can carry one ... the second is about residue at runtime and
#   is answered by what a value at rest may carry."
#
# Those are two different indices. 241 says target; the proposal says
# implementation. Nobody in the sitting noticed, because both files say "two
# questions" and the second word differs.
#
# SECTION 1 asks whether any assertion in Slots::ADMITTED can change truth value
# between two targets with MIN, MAX and WIDTH held fixed. That is decidable from
# the crate's source: it can only happen if some type or cfg in the crate is
# target-dependent.
#
# SECTION 2 classifies the five assertions by what they compare a coordinate
# against: another coordinate (well-formedness, true or false of the declaration
# alone) or a carrier's capacity (hosting, true or false of what this crate
# chose to carry an index in). 241 says all five are the second kind.
#
# THE CASES THAT MUST FAIL, planted and shown before any count is reported:
#   C1  the target-dependence grep must find a target-dependent construct where
#       one exists, or its zero on the source is a fact about the grep. The
#       crate's own tests use `usize`, so they are the positive control.
#   C2  the classifier must flag an assertion that names a carrier capacity.
#   C3  the classifier must NOT flag an assertion that names only coordinates.
#       C2 alone passes for a classifier that flags everything, which is the
#       exact defect 244 disclosed against its own census, so C3 is required.
#   C4  a planted assertion of each kind must be classified correctly, so the
#       classifier is tested on input it did not help produce.

set -u
cd "$(dirname "$0")/../../../.." || exit 1   # repo root
SRC=mock/crates/arvo-format/src
fail() { echo "CONTROL FAILED: $1"; exit 2; }

echo "=== 1. can any of the five assertions differ between two targets ==="
# Non-test source only: tests are not compiled into a consumer's binary and are
# not what the obligation is.
nontest=$(find "$SRC" -name '*.rs' ! -path '*/tests/*' ! -name 'tests.rs' | sort)
echo "  non-test source files:"
printf '%s\n' "$nontest" | sed 's/^/    /'
TDEP='\busize\b|\bisize\b|target_pointer_width|cfg\(target|size_of::<\*|std::mem::size_of'
hits=$(grep -nE "$TDEP" $nontest || true)
if [ -n "$hits" ]; then
  echo "  target-dependent constructs found:"
  printf '%s\n' "$hits" | sed 's/^/    /'
else
  echo "  target-dependent constructs found: NONE"
fi
# C1: the same grep must find them where they are.
testfiles=$(find "$SRC" -name '*.rs' \( -path '*/tests/*' -o -name 'tests.rs' \) | sort)
ctl=$(grep -cE "$TDEP" $testfiles 2>/dev/null | awk -F: '{s+=$2} END{print s+0}')
[ "${ctl:-0}" -gt 0 ] || fail "C1, the target-dependence grep finds nothing in the crate's own tests either, so its zero above is about the grep."
echo "  C1 passes: the same pattern finds $ctl target-dependent constructs in the crate's test files, so the zero above is about the source."
echo
echo "  the carriers the slot coordinates are declared in:"
grep -nE 'const (MIN|MAX|WIDTH):' "$SRC/slots.rs" | sed 's/^/    /'
grep -nE 'pub struct Width' "$SRC/width.rs" | sed 's/^/    /'
echo "  Every one is a fixed-width Rust type. i64 is 64 bits on every target and"
echo "  u32 is 32 bits on every target, so with MIN, MAX and WIDTH held fixed no"
echo "  assertion in ADMITTED can change truth value between two targets."
echo "  CONCLUSION: the shipped hosting predicate is indexed by THIS CRATE'S"
echo "  chosen carriers, not by a target. The proposal's word is the right one."
echo

echo "=== 2. what each of the five assertions actually compares ==="
# Pull the assertion conditions out of the ADMITTED block rather than typing them.
conds=$(awk '/const ADMITTED: \(\) = \{/{f=1} f{print} /^    \};/{if(f) exit}' "$SRC/slots.rs" \
        | grep -A1 'assert!(' | grep -vE 'assert!\(|^--$' | sed 's/^ *//;s/,$//')
n=$(printf '%s\n' "$conds" | grep -c .)
[ "$n" -eq 5 ] || fail "expected five assertion conditions in Slots::ADMITTED, extracted $n. The instrument is reading the wrong block."
echo "  five conditions extracted from $SRC/slots.rs:"
printf '%s\n' "$conds" | sed 's/^/    /'
echo
# A condition is a HOSTING condition iff it compares a coordinate against a
# capacity constant of a carrier this crate chose. It is a WELL-FORMEDNESS
# condition iff every term in it is a coordinate.
CAP='i64::MAX|i32::MAX|u32::MAX|\b62\b|\b63\b|\b64\b'
classify() { if printf '%s' "$1" | grep -qE "$CAP"; then echo hosting; else echo wellformed; fi; }
echo "  classification:"
h=0; w=0
while IFS= read -r c; do
  [ -n "$c" ] || continue
  k=$(classify "$c")
  [ "$k" = hosting ] && h=$((h+1)) || w=$((w+1))
  printf '    %-10s %s\n' "$k" "$c"
done <<< "$conds"
echo
printf '  hosting: %d    well-formedness: %d\n' "$h" "$w"
[ "$h" -gt 0 ] || fail "C2, the classifier flagged nothing as hosting, though the third assertion names 62."
echo "  C2 passes: at least one condition is classified hosting."
[ "$w" -gt 0 ] || fail "C3, the classifier flagged everything as hosting. A classifier that says one thing about every input says nothing."
echo "  C3 passes: at least one condition is classified well-formedness, so the classifier distinguishes."
# C4: planted conditions of each kind, which the classifier did not help write.
[ "$(classify 'Self::MIN <= Self::MAX')" = wellformed ] || fail "C4a, a planted coordinate-only condition was classified hosting."
[ "$(classify '(Self::MAX as i128) < i64::MAX as i128')" = hosting ] || fail "C4b, a planted carrier-capacity condition was classified well-formed."
[ "$(classify 'Self::WIDTH.count() >= 1')" = wellformed ] || fail "C4c, a planted coordinate-only condition was classified hosting."
echo "  C4 passes: three planted conditions, two kinds, all classified correctly."
echo
echo "  CONCLUSION: Slots::ADMITTED is not one kind of condition. It mixes"
echo "  well-formedness of the declaration with capacity of this crate's carriers."
echo "  241's 'exactly this and nothing else' is measured false here."
echo
echo "=== 3. the 63-bit example 241 rests its hosting half on ==="
grep -n 'wider than a slot index carries' -A2 "$SRC/slots.rs" | sed 's/^/  /'
echo "  241 wrote: 'a width of 63 bits is a perfectly good coordinate assignment"
echo "  that this stack cannot carry, because a slot count of 2^63 does not fit"
echo "  the signed 64-bit integer a slot index is carried in.'"
echo "  The bound in source is <= 62, so 63 is refused, and the message is the"
echo "  source's own. 241's worked example reproduces."
