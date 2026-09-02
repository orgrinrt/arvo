#!/usr/bin/env bash
# A3's discriminating control: is the const-expression bound actually checked in
# an ungated consumer, or was A3 clean because nothing was checked?
set -u
here="$(cd "$(dirname "$0")" && pwd)"
out="$here/out"; work="$here/.work4"
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
    echo "A3-control  is the bound evaluated in an ungated consumer at all"
    echo "=============================================================="
    build "lib       bound_lib2, Small implemented at 0, 7 and 13 only" bound_lib2 \
        "$here/a_generic_const_exprs/bound_lib2.rs"
    build "positive  6+7=13, implemented, ungated consumer, must build" a3ok \
        "$here/a_generic_const_exprs/bound_user_ok.rs" "bound_lib2=$work/libbound_lib2.rlib"
    build "negative  6+6=12, not implemented, ungated consumer, MUST fail" a3bad \
        "$here/a_generic_const_exprs/bound_user_bad.rs" "bound_lib2=$work/libbound_lib2.rlib"
} 2>&1 | tee "$out/f_bound_is_checked.txt"
