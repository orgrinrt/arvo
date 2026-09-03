#!/usr/bin/env bash
# Compile the storage-side arms and their controls, and commit every stderr.
#
# Two pairs, each a refusal and a control that differs from it in exactly one
# thing: whether the coordinate reads a value or a const. A control that also
# refused would say the refusal is the borrow, the lifetime or the trait shape
# rather than the value-dependence, and would void the pair.
#
#   A. an element whose quantum exponent is a field of the block it borrows.
#   B. the same, with the block in hand as an ordinary parameter, which is the
#      route a reader reaches for when A's diagnostic looks like a fact about
#      where `self` may be written.
#
# Run from this directory, after `cargo build`.
set -uo pipefail

DEPS="target/debug/deps"
[ -d "$DEPS" ] || { echo "run 'cargo build' first: no $DEPS"; exit 2; }

ARVO=$(ls "$DEPS"/libarvo_format-*.rlib 2>/dev/null | head -1)
[ -n "$ARVO" ] || { echo "no arvo_format rlib under $DEPS"; exit 2; }

mkdir -p target/refusal

run_one() {
    local src="$1" out="$2"
    printf '\n=== %s ===\n' "$src"
    rustc --edition 2024 --crate-type lib "src/$src" \
        --extern "arvo_format=$ARVO" \
        -L "dependency=$DEPS" \
        --out-dir target/refusal \
        2> "$out"
    local rc=$?
    printf 'exit %d, stderr %s bytes -> %s\n' "$rc" "$(wc -c < "$out" | tr -d ' ')" "$out"
    cat "$out"
    return $rc
}

run_one the_block_floating_point_element.rs the_block_floating_point_element.stderr
A_REFUSAL=$?
run_one the_block_floating_point_element_control.rs the_block_floating_point_element_control.stderr
A_CONTROL=$?

run_one the_block_exponent_in_hand.rs the_block_exponent_in_hand.stderr
B_REFUSAL=$?
run_one the_block_exponent_in_hand_control.rs the_block_exponent_in_hand_control.stderr
B_CONTROL=$?

echo
echo "pair A, the exponent as a field:     refusal exit $A_REFUSAL (non-zero), control exit $A_CONTROL (zero)"
echo "pair B, the block in hand:           refusal exit $B_REFUSAL (non-zero), control exit $B_CONTROL (zero)"

FAIL=0
{ [ "$A_REFUSAL" -ne 0 ] && [ "$A_CONTROL" -eq 0 ]; } || FAIL=1
{ [ "$B_REFUSAL" -ne 0 ] && [ "$B_CONTROL" -eq 0 ]; } || FAIL=1

if [ "$FAIL" -eq 0 ]; then
    echo "RESULT: the locus clause's criterion discriminates, and rustc states it in"
    echo "        the same words: a value set fixed by a sibling datum has no format,"
    echo "        and the same set fixed by a const does. A cfg-selected width is a"
    echo "        const, which is the arm that builds in lib.rs."
    exit 0
fi
echo "RESULT: void. A pair did not separate, so nothing here establishes anything."
exit 1
