#!/usr/bin/env bash
# Q31 attack probe. Is a clause carrier-derived, or is it a literal somebody moved?
#
# The carrier-perturbation classifier says a residue clause is one whose verdict
# moves when a carrier type changes. This splits that mutation in two and runs
# both halves separately:
#
#   NAIVE  only the edits the compiler refuses to build without. The carrier type
#          and the literals written in it. Nothing else.
#   FULL   the naive set plus the bound literals: `<= 62` and `i64::MAX` for the
#          slot index, `i32::MIN`/`i32::MAX` for the exponent. Those compile
#          unchanged under the wider carrier, so moving them is a choice.
#
# If NAIVE moves nothing and FULL moves the row, then the row moved because of the
# chosen edit and not because of the carrier, and the classifier is reporting the
# experimenter's annotation back to itself.
#
# Controls: the baseline vector must be reproduced after every restore, and every
# arm must reproduce the rows that have nothing to do with the perturbed carrier.

set -uo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
SRC="$ROOT/mock/crates/arvo-format/src"
BAK="$(mktemp -d)"
OUT="$HERE/output_carrier_dependence.txt"

FILES="slots.rs quantum.rs apply.rs format.rs lib.rs standards.rs"
for f in $FILES; do cp "$SRC/$f" "$BAK/$f"; done
restore() { for f in $FILES; do cp "$BAK/$f" "$SRC/$f"; touch "$SRC/$f"; done; }
trap restore EXIT

sub() { perl -0777 -i -pe "s/\Q$2\E/$3/g or die 'NOT FOUND in $1: $2'" "$SRC/$1"; }

battery() { ( cd "$HERE/carrier_dependence" && cargo run --quiet --bin battery 2>&1 | tail -1 ); }

# --- the two carriers, each split into forced and chosen ----------------------

slot_forced() {
  sub slots.rs 'pub struct Slot(i64);'            'pub struct Slot(i128);'
  sub slots.rs 'pub const fn at(index: i64)'      'pub const fn at(index: i128)'
  sub slots.rs 'pub const fn index(self) -> i64'  'pub const fn index(self) -> i128'
  sub slots.rs 'pub struct SlotCount(i64);'       'pub struct SlotCount(i128);'
  sub slots.rs 'pub const fn of(count: i64)'      'pub const fn of(count: i128)'
  sub slots.rs 'pub const fn count(self) -> i64'  'pub const fn count(self) -> i128'
  sub slots.rs '1i64 <<'                          '1i128 <<'
  # forced too, and in three other files: the carrier leaks out of slots.rs
  sub apply.rs  'Slot::at(slot.index() + whole)'  'Slot::at(slot.index() + whole as i128)'
  sub apply.rs  'Slot::at(slot as i64)'          'Slot::at(slot as i128)'
  sub apply.rs  '.rem_euclid(span)) as i64)'     '.rem_euclid(span)) as i128)'
  sub format.rs 'Slot::at(slot as i64)'          'Slot::at(slot as i128)'
}
slot_chosen() {
  sub slots.rs 'Self::WIDTH.count() <= 62,'  'Self::WIDTH.count() <= 126,'
  sub slots.rs '&& S::WIDTH.count() <= 62'   '&& S::WIDTH.count() <= 126'
  sub slots.rs '< i64::MAX as i128'          '< i128::MAX'
}

exponent_forced() {
  sub quantum.rs 'pub struct Exponent(i32);'          'pub struct Exponent(i64);'
  sub quantum.rs 'pub const fn of(power: i32)'        'pub const fn of(power: i64)'
  sub quantum.rs 'pub const fn power(self) -> i32'    'pub const fn power(self) -> i64'
  sub quantum.rs '(magnitude.0 as i32)'               '(magnitude.0 as i64)'
  sub quantum.rs 'Constant<const EXP: i32>'           'Constant<const EXP: i64>'
  sub quantum.rs 'impl<const EXP: i32>'               'impl<const EXP: i64>'
  sub quantum.rs 'Indexed<const MIN_EXP: i32, const COUNT: u32>' 'Indexed<const MIN_EXP: i64, const COUNT: u32>'
  sub quantum.rs 'impl<const MIN_EXP: i32, const COUNT: u32>'    'impl<const MIN_EXP: i64, const COUNT: u32>'
  # forced too: the exponent carrier is replicated as a bare const-generic
  # parameter in fourteen public positions across two other files.
  sub lib.rs       'const FRAC: i32'    'const FRAC: i64'
  sub lib.rs       'const EXP: i32'     'const EXP: i64'
  sub lib.rs       'const MIN_EXP: i32' 'const MIN_EXP: i64'
  sub standards.rs 'const F: i32'       'const F: i64'
}
exponent_chosen() {
  sub quantum.rs 'reach >= (i32::MIN as i128) && reach <= (i32::MAX as i128)' \
                 'reach >= (i64::MIN as i128) && reach <= (i64::MAX as i128)'
}

# --- the replacement: write the bound as a computation over the carrier --------
#
# `<= 62` and `i64::MAX` are the carrier's range spelled as constants. Spell them
# as functions of the carrier instead and the classifier stops needing a hand
# edit: one token moves and the verdict moves with it. Two arms, and the first is
# the control that says the rewrite changed no behaviour at the shipped carrier.

slot_derived_common() {
  slot_forced_body_only
  sub slots.rs 'Self::WIDTH.count() <= 62,'  'Self::WIDTH.count() <= SlotIndex::BITS - 2,'
  sub slots.rs '&& S::WIDTH.count() <= 62'   '&& S::WIDTH.count() <= SlotIndex::BITS - 2'
  sub slots.rs '< i64::MAX as i128'          '< SlotIndex::MAX as i128'
}
# The carrier named once, as a type alias, which is the whole of the change.
slot_alias() { perl -0777 -i -pe "s/\Quse crate::width::\E/type SlotIndex = $1;\nuse crate::width::/" "$SRC/slots.rs"; }
slot_derived_at_i64()  { slot_alias i64;  sub slots.rs 'pub struct Slot(i64);' 'pub struct Slot(SlotIndex);'; slot_derived_common; }
slot_derived_at_i128() { slot_alias i128; sub slots.rs 'pub struct Slot(i64);' 'pub struct Slot(SlotIndex);'; slot_derived_common; }

# The forced set minus the struct line, which the derived arms write themselves.
slot_forced_body_only() {
  sub slots.rs 'pub const fn at(index: i64)'      'pub const fn at(index: SlotIndex)'
  sub slots.rs 'pub const fn index(self) -> i64'  'pub const fn index(self) -> SlotIndex'
  sub slots.rs 'pub struct SlotCount(i64);'       'pub struct SlotCount(SlotIndex);'
  sub slots.rs 'pub const fn of(count: i64)'      'pub const fn of(count: SlotIndex)'
  sub slots.rs 'pub const fn count(self) -> i64'  'pub const fn count(self) -> SlotIndex'
  sub slots.rs '1i64 <<'                          '(1 as SlotIndex) <<'
  sub slots.rs 'type SlotIndex'                   'pub type SlotIndex'
  sub apply.rs  'Slot::at(slot.index() + whole)'  'Slot::at(slot.index() + whole as SlotIndex)'
  sub apply.rs  'use crate::slots::'                'use crate::slots::SlotIndex;\nuse crate::slots::'
  sub apply.rs  'Slot::at(slot as i64)'          'Slot::at(slot as _)'
  sub apply.rs  '.rem_euclid(span)) as i64)'     '.rem_euclid(span)) as _)'
  sub format.rs 'Slot::at(slot as i64)'          'Slot::at(slot as _)'
}

arm() {
  local name="$1"; shift
  restore
  for step in "$@"; do
    if ! $step 2>/tmp/q31_carr_err; then
      printf '%-28s %s\n' "$name" "EDIT FAILED: $(head -1 /tmp/q31_carr_err)"
      return
    fi
  done
  local v; v="$(battery)"
  case "$v" in
    Grid63=*) printf '%-28s %s\n' "$name" "$v" ;;
    *)        printf '%-28s %s\n' "$name" "BUILD FAILED: $(echo "$v" | head -c 90)" ;;
  esac
}

{
  echo "probe: is a clause carrier-derived, or a literal somebody moved?"
  echo "tree:  $(git -C "$ROOT" rev-parse --short HEAD)"
  echo "tool:  $(rustc --version)"
  echo
  echo "Legend: 1 admitted, 0 refused, by the shipped verdict functions."
  echo "Rows that must never move: Grid8, WiderThanSpan (admitted), Inverted,"
  echo "WidthZero, SpanOverWidth, ZeroMagnitudes (refused), and the quantum rows"
  echo "under a slot arm, and the slot rows under an exponent arm."
  echo
  restore
  printf '%-28s %s\n' "baseline (unmutated)" "$(battery)"
  echo
  echo "--- slot index carrier: i64 -> i128 ---"
  arm "slot NAIVE  (forced only)"  slot_forced
  arm "slot FULL   (+ chosen)"     slot_forced slot_chosen
  echo
  echo "--- the same slot bound, written as a computation over the carrier ---"
  arm "derived @ i64  (control)"   slot_derived_at_i64
  arm "derived @ i128 (one token)" slot_derived_at_i128
  echo
  echo "--- exponent carrier: i32 -> i64 ---"
  arm "exp  NAIVE  (forced only)"  exponent_forced
  arm "exp  FULL   (+ chosen)"     exponent_forced exponent_chosen
  echo
  restore
  printf '%-28s %s\n' "baseline again (restore)" "$(battery)"
  echo
  echo "The chosen edits, verbatim, and note that each COMPILES unchanged under the"
  echo "wider carrier, which is what makes it a choice rather than a consequence:"
  echo "  slots.rs  Self::WIDTH.count() <= 62      -> <= 126"
  echo "  slots.rs  < i64::MAX as i128             -> < i128::MAX"
  echo "  quantum.rs reach in [i32::MIN, i32::MAX] -> [i64::MIN, i64::MAX]"
  echo
  for f in $FILES; do
    if diff -q "$BAK/$f" "$SRC/$f" >/dev/null; then echo "  $f restored"; else echo "  $f DIFFERS, restore failed"; fi
  done
} | tee "$OUT"
