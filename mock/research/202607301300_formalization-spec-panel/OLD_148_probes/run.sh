#!/usr/bin/env bash
# Probes for 148. Run from this directory.  Toolchain pinned:
# rustc 1.98.0-nightly (57d06900f 2026-05-27).
set -u
TMP=$(mktemp -d)
RS="rustup run nightly-2026-05-28 rustc"
LIB="--edition 2021 --crate-type lib"

b() { echo; echo "=============== $* ==============="; }

b "p1  the inclusion conditions, Implicit, 331776 ordered pairs"
$RS -O p1_inclusion_conditions.rs -o /tmp/p1 && /tmp/p1

b "p2  the inclusion conditions, Ranged, 147456 ordered pairs"
$RS -O p2_ranged_inclusion.rs -o /tmp/p2 && /tmp/p2

b "p3  meets and joins, complete enumeration inside the bounding sets"
$RS -O p3_lattice_or_not.rs -o /tmp/p3 && /tmp/p3

b "p4a From by reference, condition on a trait, Peano widths (expect exit 0, no -Z, no features)"
$RS $LIB --out-dir "$TMP" p4a_from_ref_trait_bound.rs; echo "exit=$?"

b "p4b the same BY VALUE (expect E0119)"
$RS $LIB --out-dir "$TMP" p4b_by_value_control.rs; echo "exit=$?"

b "p4c the same over the design's binary width encoding (expect exit 0)"
$RS $LIB --out-dir "$TMP" p4c_binary_widths.rs; echo "exit=$?"

b "p4d negative controls, Peano (expect refusal; note the diagnostic size)"
out=$($RS $LIB --out-dir "$TMP" p4d_negative_controls.rs 2>&1); rc=$?; echo "$out" | head -20; echo "exit=$rc"

b "p4e negative control, binary, WITHOUT do_not_recommend (expect the internals to leak)"
out=$($RS $LIB --out-dir "$TMP" p4e_binary_negative.rs 2>&1); rc=$?; echo "$out" | head -12; echo "exit=$rc"

b "p4f the same WITH do_not_recommend (expect the design's own message at the top)"
out=$($RS $LIB --out-dir "$TMP" p4f_do_not_recommend.rs 2>&1); rc=$?; echo "$out" | head -14; echo "exit=$rc"

b "p4g strict order by value, binary encoding (expect E0119 on BOTH solvers)"
out=$($RS $LIB --out-dir "$TMP" p4g_strict_by_value.rs 2>&1); rc=$?; echo "$out" | head -10; echo "exit=$rc (default solver)"
out=$($RS $LIB -Znext-solver=globally --out-dir "$TMP" p4g_strict_by_value.rs 2>&1); rc=$?; echo "$out" | head -6; echo "exit=$rc (next solver)"

b "p4h TryFrom beside From (expect E0119)"
out=$($RS $LIB --out-dir "$TMP" p4h_tryfrom_beside.rs 2>&1); rc=$?; echo "$out" | head -12; echo "exit=$rc"

b "p5  erasure, and the by-value arm beside the by-reference one"
$RS $LIB -O --emit asm p5_erasure_and_by_value_arm.rs -o p5.s; echo "exit=$?"
grep -n "scalar_via_from = \|scalar_via_embed = " p5.s

b "re-check of 146's own f03 with and without the flag it needs"
( cd ../146_probes \
  && $RS $LIB --out-dir "$TMP" -Znext-solver=globally f03_ref_source_full.rs >/dev/null 2>&1; echo "f03 WITH -Znext-solver=globally: exit=$?" \
  ; $RS $LIB --out-dir "$TMP" f03_ref_source_full.rs >/dev/null 2>&1; echo "f03 WITHOUT it: exit=$?" )
