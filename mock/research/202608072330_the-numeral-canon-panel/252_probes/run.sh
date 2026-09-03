#!/usr/bin/env bash
# Does arvo's unstable machinery reach a consumer?
#
# Three arms and three controls, each a two-crate compile with `rustc` directly
# so nothing about cargo, features or a workspace can be the cause. The library
# is built first, then a consumer crate that declares NO feature attribute of
# any kind is built against it.
#
# The question is one thing per arm: does the consumer need the gate?
#
#   A  a `generic_const_exprs` bound in a public signature
#   B  a stack-owned type at a const generic parameter position
#   C  the coordinate set at associated consts, which is what the canon ratifies
#
# Every arm has a control in the same crate under the same conditions, so a
# failure that is really "this dependency cannot be consumed at all" is
# distinguishable from "this position leaks the gate".
#
# Exit status is not the result. The result is the captured output under `out/`.
set -u

here="$(cd "$(dirname "$0")" && pwd)"
out="$here/out"
work="$here/.work"
rm -rf "$work" "$out"
mkdir -p "$work" "$out"

echo "toolchain: $(rustc --version)" | tee "$out/00_toolchain.txt"

build() { # <label> <crate-name> <source> [extern...]
    local label="$1" name="$2" src="$3"; shift 3
    local args=()
    for e in "$@"; do args+=(--extern "$e"); done
    echo "--- $label"
    rustc --edition 2024 --crate-type lib --crate-name "$name" \
        -L "$work" --out-dir "$work" "${args[@]}" "$src" 2>&1
    echo "[exit $?]"
}

{
    echo "=============================================================="
    echo "ARM A  generic_const_exprs in a public signature"
    echo "=============================================================="
    build "A.lib  leaky (feature enabled here)" leaky "$here/a_generic_const_exprs/leaky.rs"
    build "A.control  consumer naming the signature with NO const expression" acontrol \
        "$here/a_generic_const_exprs/control_user.rs" "leaky=$work/libleaky.rlib"
    build "A.test  consumer naming the signature that CARRIES the const expression" auser \
        "$here/a_generic_const_exprs/user.rs" "leaky=$work/libleaky.rlib"
} 2>&1 | tee "$out/a_generic_const_exprs.txt"

{
    echo "=============================================================="
    echo "ARM B  a stack-owned type at a const generic parameter"
    echo "=============================================================="
    build "B.lib  door (feature enabled here)" door "$here/b_adt_const_params/door.rs"
    build "B.control  consumer naming the item with no ADT const parameter" bcontrol \
        "$here/b_adt_const_params/control_user.rs" "door=$work/libdoor.rlib"
    build "B.test  consumer naming the declaration whose const parameter is an ADT" buser \
        "$here/b_adt_const_params/user.rs" "door=$work/libdoor.rlib"
} 2>&1 | tee "$out/b_adt_const_params.txt"

rm -f "$work/libdoor.rlib"

{
    echo "=============================================================="
    echo "ARM C  the coordinate set at associated consts, no feature anywhere"
    echo "=============================================================="
    build "C.lib  door (NO feature attribute)" door "$here/c_assoc_const/door.rs"
    build "C.test  outside crate declaring a format, no machine type, no gate" cuser \
        "$here/c_assoc_const/user.rs" "door=$work/libdoor.rlib"
    build "C.negative  the value a u32-shaped door cannot hold, which MUST fail" cneg \
        "$here/c_assoc_const/negative_control.rs" "door=$work/libdoor.rlib"
} 2>&1 | tee "$out/c_assoc_const.txt"

echo
echo "results are under $out"
