#!/usr/bin/env bash
# Seat 253. How many coordinates the four format contracts declare on this tree.
#
# `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
# speaks of "the ten associated constants". Seat 247 reports nine on its tree and
# says a later reader reconstructing "the ten" from HEAD will not get the set
# seats 238 and 239 counted. This is a second instance of that, by a different
# instrument: extract the associated consts from each trait body rather than
# counting from a commit message.
#
# Control: the same extractor is run against a planted trait body carrying one
# extra const, so a count that could not move would be visible.
set -u
SRC=${SRC:-../../../../crates/arvo-format/src}

consts_in_trait() {   # $1 = file, $2 = trait name
  awk -v t="pub trait $2" '
    $0 ~ t {inside=1}
    inside && /^}/ {inside=0}
    inside && /^    const [A-Z_]+:/ {
      line=$0; sub(/^ *const /,"",line); sub(/:.*/,"",line); print line
    }
  ' "$1"
}

total=0
for spec in "ambient.rs Ambient" "quantum.rs Quantum" "slots.rs Slots" "format.rs Format"; do
  set -- $spec
  names=$(consts_in_trait "$SRC/$1" "$2" | grep -v '^ADMITTED$' || true)
  n=$(printf '%s\n' "$names" | grep -c . || true)
  total=$(( total + n ))
  printf "  %-9s %s : %s\n" "$2" "$n" "$(printf '%s' "$names" | tr '\n' ' ')"
done
echo "  ---------------------------------"
echo "  coordinates on this tree (ADMITTED excluded, it carries no value): $total"
echo
echo "  Operation::ARITY, which one commit message counted as the tenth:"
grep -n 'const ARITY' "$SRC/adapt.rs" | sed 's/^/    /'
echo
echo "======== CONTROL: the extractor against a planted trait carrying one more"
tmp=$(mktemp -d)
sed 's|^    const PHASE: Phase;|    const PHASE: Phase;\n\n    const PLANTED_CONTROL: Bool;|' "$SRC/format.rs" > "$tmp/format.rs"
echo "  Format with one planted const: $(consts_in_trait "$tmp/format.rs" Format | grep -v ADMITTED | grep -c .)"
echo "  (if this is not 2 the count above proves nothing)"
rm -rf "$tmp"
