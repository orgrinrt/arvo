#!/usr/bin/env bash
# Compile the arms on their own and commit every stderr.
#
# Three pairs, each a refusal and a control. A control that also refused would
# void its pair, because the difference between the two arms would then not be
# the thing the pair is about.
#
#   A. the shipped family at 64 against the same at 62. Is the width the bound?
#   B. an outside declarer writing an unsigned 64-bit range against the same at
#      62. Is the shipped impl list the bound, or the coordinate's arithmetic?
#   C. an outside declarer writing a *signed* 64-bit range, whose every constant
#      fits the coordinate, against the same declaration with the obligation not
#      forced. Is the escape the diagnostic names actually open?
#
# Run from this directory, after `cargo build`, which is what puts the dependency
# rlibs where `--extern` can find them.
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

run_one the_sixty_four_bit_width.rs the_sixty_four_bit_width.stderr
A_REFUSAL=$?
run_one the_sixty_two_bit_width.rs the_sixty_two_bit_width.stderr
A_CONTROL=$?

run_one the_outside_implementor.rs the_outside_implementor.stderr
B_REFUSAL=$?
run_one the_outside_implementor_control.rs the_outside_implementor_control.stderr
B_CONTROL=$?

run_one the_outside_signed_sixty_four.rs the_outside_signed_sixty_four.stderr
C_REFUSAL=$?
run_one the_outside_signed_sixty_four_control.rs the_outside_signed_sixty_four_control.stderr
C_CONTROL=$?

echo
echo "pair A, the shipped family:       refusal exit $A_REFUSAL (non-zero), control exit $A_CONTROL (zero)"
echo "pair B, an outside unsigned 64:   refusal exit $B_REFUSAL (non-zero), control exit $B_CONTROL (zero)"
echo "pair C, an outside signed 64:     refusal exit $C_REFUSAL (non-zero), control exit $C_CONTROL (zero)"

FAIL=0
{ [ "$A_REFUSAL" -ne 0 ] && [ "$A_CONTROL" -eq 0 ]; } || FAIL=1
{ [ "$B_REFUSAL" -ne 0 ] && [ "$B_CONTROL" -eq 0 ]; } || FAIL=1
{ [ "$C_REFUSAL" -ne 0 ] && [ "$C_CONTROL" -eq 0 ]; } || FAIL=1

if [ "$FAIL" -eq 0 ]; then
    echo "RESULT: 64 bits is refused inside the shipped family, refused to an outside"
    echo "        unsigned declarer by the coordinate's arithmetic, and refused to an"
    echo "        outside signed declarer by the ADMITTED obligation, which caps every"
    echo "        implementor at 62 while the diagnostic tells them the trait is open."
    exit 0
fi
echo "RESULT: void. A pair did not separate, so nothing here establishes anything."
exit 1
