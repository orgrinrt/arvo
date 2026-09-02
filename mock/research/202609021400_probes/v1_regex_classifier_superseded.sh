#!/usr/bin/env bash
# Seat q31a. Does an `ADMITTED` assertion refuse a candidate on mathematics, or
# on what this implementation's carriers hold?
#
# Q31 asks whether the canon needs one word or two for "is a number system" and
# "can arvo carry it". If the shipped crate's admission obligations are all one
# kind of condition, one word costs nothing and the question is idle. If they
# are two kinds welded into one const, the crate is already living the confusion
# the question names, and it has no word to say which is which.
#
# The classifier, and it is deliberately mechanical rather than a reading. An
# assertion is HOSTING when its expression or its refusal message names a
# machine type of this implementation, or a bit count of one. Otherwise it is
# CONCEPT: it refuses a candidate that fails to describe values at all.
#
# Two controls, because a classifier that cannot come out the other way is not a
# classifier:
#   - a planted CONCEPT-shaped assertion must classify CONCEPT;
#   - a planted HOSTING-shaped assertion must classify HOSTING.
# Both are run before the real files and the script exits non-zero if either
# comes out wrong.
set -uo pipefail
cd "$(dirname "$0")/../../.." || exit 1
SRC=mock/crates/arvo-format/src

# A machine type of this implementation, or a bit count of one. `Width`,
# `Exponent`, `Slot` and `MagnitudeCount` are this crate's own newtypes and are
# NOT in the pattern: what matters is whether a *machine* bound is being named.
MACHINE='i8|i16|i32|i64|i128|u8|u16|u32|u64|u128|isize|usize|64-bit|32-bit|2\^63|2\^31'

classify() { # $1 = the whole assert!(...) text
  if printf '%s' "$1" | grep -Eq "$MACHINE"; then echo HOSTING; else echo CONCEPT; fi
}

echo "== controls =="
C_CONCEPT='assert!(Self::WIDTH.count() >= 1, "a declared width of zero bits admits no values");'
C_HOSTING='assert!(Self::WIDTH.count() <= 62, "2^63 does not fit a signed 64-bit integer");'
ok=1
[ "$(classify "$C_CONCEPT")" = CONCEPT ] || { echo "CONTROL FAILED: concept arm classified $(classify "$C_CONCEPT")"; ok=0; }
[ "$(classify "$C_HOSTING")" = HOSTING ] || { echo "CONTROL FAILED: hosting arm classified $(classify "$C_HOSTING")"; ok=0; }
echo "  planted concept arm -> $(classify "$C_CONCEPT")"
echo "  planted hosting arm -> $(classify "$C_HOSTING")"
[ "$ok" = 1 ] || { echo "controls did not separate; the classifier proves nothing"; exit 1; }

echo
echo "== the four ADMITTED blocks =="
total=0; hosting=0; concept=0
for f in ambient format quantum slots; do
  file="$SRC/$f.rs"
  # The block runs from `const ADMITTED` to the closing `};` at the same indent.
  block=$(awk '/const ADMITTED: \(\) = \{/{on=1} on{print} on&&/^    \};$/{exit}' "$file")
  n=$(printf '%s\n' "$block" | grep -c 'assert!(')
  echo "-- $f.rs : $n assertion(s)"
  # Split the block on `assert!(` and classify each piece.
  printf '%s\n' "$block" \
    | tr '\n' ' ' \
    | sed 's/assert!(/\n@@/g' \
    | grep '^@@' \
    | while IFS= read -r a; do
        k=$(classify "$a")
        # the first quoted string in the assertion is its message
        msg=$(printf '%s' "$a" | sed -n 's/.*"\(.\{0,70\}\).*/\1/p')
        printf '   %-7s %s\n' "$k" "${msg:-<no message>}"
      done
  # counted separately, because the pipe above runs in a subshell
  for a in $(printf '%s\n' "$block" | tr '\n' ' ' | sed 's/assert!(/\n@@/g' | grep -c '^@@'); do :; done
  total=$((total + n))
done

echo
echo "== totals, recounted outside the subshell =="
allh=0; allc=0
for f in ambient format quantum slots; do
  block=$(awk '/const ADMITTED: \(\) = \{/{on=1} on{print} on&&/^    \};$/{exit}' "$SRC/$f.rs")
  while IFS= read -r a; do
    [ -z "$a" ] && continue
    if [ "$(classify "$a")" = HOSTING ]; then allh=$((allh+1)); else allc=$((allc+1)); fi
  done <<< "$(printf '%s\n' "$block" | tr '\n' ' ' | sed 's/assert!(/\n@@/g' | grep '^@@')"
done
echo "  assertions total : $((allh+allc))"
echo "  CONCEPT          : $allc"
echo "  HOSTING          : $allh"
echo
if [ "$allh" -gt 0 ] && [ "$allc" -gt 0 ]; then
  echo "RESULT: the obligations are two kinds of condition under one word."
else
  echo "RESULT: the obligations are one kind of condition."
fi
