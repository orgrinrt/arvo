#!/usr/bin/env bash
# Which const-parameter gate admits the door's own type, and at what price.
#
# The question's second option names `min_adt_const_params`, which is the name
# the compiler's help text emits on this pin. The workspace register carries a
# row for `adt_const_params` in its Allowed tier and none for the other. These
# four arms say what each of the two actually does to a `repr(transparent)`
# newtype whose field is private, which is what the door's types are.
#
# `a0` is the control: with no gate at all the position is refused, which is the
# refusal `arvo-format/tests/ui/an_arvo_type_as_a_const_parameter.rs` already
# pins. Without it a later arm building would say nothing, because it could be
# building for a reason unrelated to the feature.
#
# Compiled to a real path rather than /dev/null: `rustc` wants a temp directory
# beside its output and refuses on that device.
set -u
cd "$(dirname "$0")"
mkdir -p out
trap 'rm -rf out' EXIT

for f in a0_no_gate a1_min_adt_const_params a2_adt_const_params a3_public_field; do
    echo "=== $f ==="
    # Captured rather than piped: `grep | sed` reports sed's status, which is
    # always zero, so an `|| echo` after it never fires.
    g=$(grep -m1 -E "^#!\[feature" "$f.rs")
    if [ -n "$g" ]; then echo "    gate: $g"; else echo "    gate: none"; fi
    rustc --edition 2021 --crate-type bin -o "out/$f" "$f.rs" 2>&1 \
      | grep -E '^error|less visible|only supported types' | head -3 | sed 's/^/    /'
    if [ -f "out/$f" ]; then echo "    BUILT."; else echo "    refused."; fi
    echo
done

echo "=== verdict ==="
echo "The gate the second option names refuses a newtype whose field is private,"
echo "so taking it costs the encapsulation the door's types exist for as well as"
echo "the feature. The gate the workspace register carries a row for takes the"
echo "same type with the field still private."
