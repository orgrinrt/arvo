#!/usr/bin/env bash
# The two discriminating controls the first run showed were owed.
#
# A2  the same consumer as arm A with the feature turned on in the consumer.
#     Separates "the consumer needs the gate" from "an unevaluated const never
#     normalises across a crate boundary for anybody".
# B2  arm B's consumer forcing the ADT const parameter through a const
#     assertion, positive and negative. Separates "it compiled" from "it was
#     actually evaluated here".
set -u

here="$(cd "$(dirname "$0")" && pwd)"
out="$here/out"
work="$here/.work2"
rm -rf "$work"; mkdir -p "$work" "$out"

build() {
    local label="$1" name="$2" src="$3"; shift 3
    local args=()
    for e in "$@"; do args+=(--extern "$e"); done
    echo "--- $label"
    rustc --edition 2024 --crate-type lib --crate-name "$name" \
        -L "$work" --out-dir "$work" "${args[@]}" "$src" 2>&1
    echo "[exit $?]"
}

{
    echo "toolchain: $(rustc --version)"
    echo "=============================================================="
    echo "A2  does turning the feature ON in the consumer fix arm A"
    echo "=============================================================="
    build "A2.lib   leaky" leaky "$here/a_generic_const_exprs/leaky.rs"
    build "A2.test  same consumer, feature enabled in the CONSUMER" agated \
        "$here/a_generic_const_exprs/gated_user.rs" "leaky=$work/libleaky.rlib"

    echo "=============================================================="
    echo "B2  was the ADT const parameter actually evaluated in the consumer"
    echo "=============================================================="
    build "B2.lib       door" door "$here/b_adt_const_params/door.rs"
    build "B2.positive  asserts DECLARED == 13, must build" bpos \
        "$here/b_adt_const_params/forced_user.rs" "door=$work/libdoor.rlib"
    build "B2.negative  asserts DECLARED == 12, MUST fail" bneg \
        "$here/b_adt_const_params/forced_user_wrong.rs" "door=$work/libdoor.rlib"
} 2>&1 | tee "$out/d_discriminating_controls.txt"
