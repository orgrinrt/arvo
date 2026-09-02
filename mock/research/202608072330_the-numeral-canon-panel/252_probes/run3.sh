#!/usr/bin/env bash
# Arm A3: the bound shape, which is the wording the obligation row actually uses.
set -u
here="$(cd "$(dirname "$0")" && pwd)"
out="$here/out"; work="$here/.work3"
rm -rf "$work"; mkdir -p "$work" "$out"
build() {
    local label="$1" name="$2" src="$3"; shift 3
    local args=(); for e in "$@"; do args+=(--extern "$e"); done
    echo "--- $label"
    rustc --edition 2024 --crate-type lib --crate-name "$name" \
        -L "$work" --out-dir "$work" "${args[@]}" "$src" 2>&1
    echo "[exit $?]"
}
{
    echo "toolchain: $(rustc --version)"
    echo "=============================================================="
    echo "A3  a generic_const_exprs bound in a public signature"
    echo "=============================================================="
    build "A3.lib      bound_lib" bound_lib "$here/a_generic_const_exprs/bound_lib.rs"
    build "A3.test     consumer with NO feature" a3user \
        "$here/a_generic_const_exprs/bound_user.rs" "bound_lib=$work/libbound_lib.rlib"
    build "A3.control  same consumer, feature ON" a3gated \
        "$here/a_generic_const_exprs/bound_gated_user.rs" "bound_lib=$work/libbound_lib.rlib"
} 2>&1 | tee "$out/e_bound_shape.txt"
