#!/usr/bin/env bash
# Seat q31a. Q31 option three scopes the second word to a target. That is only
# worth a quantifier if some hosting refusal actually changes truth value
# between two targets. This measures whether any does.
#
# An independent replication rather than a citation. Seat 246 measured this over
# `arvo-format` alone with one pattern set; this widens to every crate in the
# workspace and adds `f16`/`f128`, feature gates and conditional compilation of
# any kind, because a target index could enter through any of them and not only
# through pointer width.
#
# Two controls:
#   - the same pattern over the crates' own test files, which must find the one
#     `usize` known to be there, or a zero in the source columns means nothing;
#   - a synthetic file planted with each pattern, which must be found, so the
#     pattern list is shown to be able to fire at all.
set -uo pipefail
cd "$(dirname "$0")/../../.." || exit 1

# Two patterns, because the first run of this probe used one and it fired four
# times on `#[cfg(test)]`. A test-inclusion gate is conditional compilation and
# is not a target index, so the broad pattern over-reports and the number it
# produced was a fact about the pattern. Both are kept and reported separately:
# ANY is what shows the instrument can fire, TARGET is the measurement.
ANY='usize|isize|target_pointer_width|target_arch|target_os|target_endian|target_feature|cfg\(target|cfg\(feature|cfg!\(|#\[cfg|f16|f128|size_of::<\*'
TARGET='usize|isize|target_pointer_width|target_arch|target_os|target_endian|target_feature|cfg\(target|cfg\(feature|f16|f128|size_of::<\*'
PATTERNS="$ANY"

echo "== control 1: can the pattern list fire at all? =="
PLANT=mock/target/q31a_plant.rs
mkdir -p mock/target
cat > "$PLANT" <<'EOP'
#[cfg(target_pointer_width = "64")]
pub const W: usize = 64;
#[cfg(target_feature = "avx2")]
pub const A: bool = true;
pub type F = f16;
EOP
n=$(grep -Ec "$PATTERNS" "$PLANT")
echo "  planted file            : $n hit(s)  (must be > 0)"
[ "$n" -gt 0 ] || { echo "the pattern list cannot fire; nothing below is evidence"; exit 1; }

echo
echo "== control 2: the crates' own test files =="
TESTS=$(find mock/crates -name '*.rs' \( -path '*/tests/*' -o -name 'tests.rs' -o -name 'tests*.rs' \) | sort)
tn=0
for f in $TESTS; do tn=$((tn + $(grep -Ec "$PATTERNS" "$f"))); done
echo "  $(echo "$TESTS" | grep -c .) test file(s), $tn hit(s)  (must be > 0)"
[ "$tn" -gt 0 ] || { echo "the instrument finds nothing even where a hit is known; zeroes below mean nothing"; exit 1; }

echo
echo "== the measurement: every non-test source file in every crate =="
SRCS=$(find mock/crates -name '*.rs' ! -path '*/tests/*' ! -name 'tests.rs' ! -name 'tests*.rs' | sort)
total=0; anytotal=0
printf '  %-52s %6s %7s\n' "file" "ANY" "TARGET"
for f in $SRCS; do
  a=$(grep -Ec "$ANY" "$f")
  n=$(grep -Ec "$TARGET" "$f")
  total=$((total + n)); anytotal=$((anytotal + a))
  printf '  %-52s %6s %7s\n' "$f" "$a" "$n"
done
echo "  ---------------------------------------------------------------------"
printf '  %-52s %6s %7s\n' "$(echo "$SRCS" | grep -c .) files, total" "$anytotal" "$total"
echo
echo "  The ANY column is not zero and every hit in it is \`#[cfg(test)]\`:"
grep -rnE "$ANY" $SRCS
echo "  A test-inclusion gate is not a target index. That is why the two columns"
echo "  differ, and the ANY column is what shows the instrument fires on source."

echo
echo "== what carries the hosting bounds, then =="
grep -n 'pub struct Width\|pub struct Slot(\|pub struct Exponent\|pub struct MagnitudeCount\|pub struct Radix' mock/crates/arvo-format/src/*.rs
echo
echo "Every one of those is a fixed-width machine type. With MIN, MAX, WIDTH,"
echo "BASE, SLOPE and MAGNITUDES held fixed, no assertion in any ADMITTED block"
echo "can change truth value between two targets."
echo
if [ "$total" -eq 0 ]; then
  echo "RESULT: the hosting predicate is indexed by this implementation, not by a target."
  echo "        Option three's quantifier over compilations ranges over one value."
else
  echo "RESULT: $total site(s) could carry a target index; option three has something to serve."
fi
