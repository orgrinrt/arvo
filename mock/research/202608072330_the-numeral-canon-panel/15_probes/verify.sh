#!/usr/bin/env bash
# Reruns every probe in this directory and reports whether each matched its
# expected outcome. One command, no arguments.
#
#   cd 15_probes && ./verify.sh
#
# Toolchain: nightly-2026-05-28 = rustc 1.98.0-nightly (57d06900f 2026-05-27).
# Zero feature gates and no -Z flags anywhere; `grep -c '#!\[feature' *.rs` is
# part of the run below.

set -u
cd "$(dirname "$0")"
TC=nightly-2026-05-28
mkdir -p build
fail=0
unexpected=0

say() { printf '%-46s %s\n' "$1" "$2"; }

expect_ok() { # name, command...
  local n="$1"; shift
  if "$@" >/dev/null 2>&1; then say "$n" "ok (expected ok)"; else
    say "$n" "FAIL (expected ok)"; unexpected=$((unexpected+1)); fi
}
expect_err() { # name, command...
  local n="$1"; shift
  if "$@" >/dev/null 2>&1; then
    say "$n" "COMPILED (expected refusal)"; unexpected=$((unexpected+1));
  else say "$n" "refused (expected refusal)"; fi
}

echo "== toolchain =="
rustc +$TC --version
echo
echo "== feature gates across every probe (must all be 0) =="
grep -c '#!\[feature' *.rs | sed 's/^/  /'
echo

echo "== python instruments =="
expect_ok  "q01 negative-width recount"        python3 q01_negative_width_recount.py
expect_ok  "q02 (W,F) coordinates"             python3 q02_wf_coordinates.py
expect_ok  "q03 tight addition"                python3 q03_tight_addition.py
expect_ok  "q03b reconcile 461 vs 476"         python3 q03b_reconcile_461_vs_476.py
echo

echo "== generated matrices =="
python3 gen.py >/dev/null 2>&1
expect_ok  "q05 subtraction whole matrix"      rustc +$TC --edition 2024 --crate-type lib q05_subtraction_matrix.rs --out-dir build
expect_ok  "q06 shape rules whole matrix"      rustc +$TC --edition 2024 --crate-type lib q06_shape_matrix.rs --out-dir build
expect_err "q06 negative control"              rustc +$TC --edition 2024 --crate-type lib q06_negctl.rs --out-dir build
echo

echo "== the three-input map =="
expect_ok  "q07 map compiles"                  rustc +$TC --edition 2024 --crate-type lib q07_three_input_map.rs --out-dir build
expect_ok  "q08 map whole matrix, 1608 triples" rustc +$TC --edition 2024 --crate-type lib q08_map_matrix.rs --out-dir build
expect_err "q08 negative control, Cold stride" rustc +$TC --edition 2024 --crate-type lib q08_negctl.rs --out-dir build
expect_err "q08 negative control, Hot padding" rustc +$TC --edition 2024 --crate-type lib q08_negctl2.rs --out-dir build
echo

echo "== the door, erasure, Cold =="
expect_ok  "q09 door + derivation"             rustc +$TC --edition 2024 -O q09_door_and_erasure.rs --out-dir build
expect_ok  "q12 erasure asm"                   rustc +$TC --edition 2024 -O --emit asm --crate-type lib q12_erasure_asm.rs --out-dir build
expect_ok  "q13 Cold packed column"            rustc +$TC --edition 2024 -O q13_cold_packed.rs --out-dir build
echo

echo "== the diagnostic tag =="
expect_err "q10 three diag arms (all fail)"    rustc +$TC --edition 2024 --crate-type lib q10_diag_tag.rs --out-dir build
expect_err "q11 t1, computed vs written"       rustc +$TC --edition 2024 --crate-type lib q11_tag_costs.rs --out-dir build
expect_ok  "q11 without t1"                    rustc +$TC --edition 2024 --cfg no_t1 -O --crate-type bin q11_tag_costs.rs --out-dir build
expect_err "q14 too-narrow declaration"        rustc +$TC --edition 2024 --crate-type lib q14_declared_plus_tag.rs --out-dir build
expect_ok  "q14 adequate declaration"          rustc +$TC --edition 2024 --cfg no_d2 --crate-type lib q14_declared_plus_tag.rs --out-dir build
echo

echo "== the ceiling and the alias site =="
expect_ok  "q15 c1, four multiplies past the table" rustc +$TC --edition 2024 --crate-type lib q15_ceiling_and_alias_site.rs --out-dir build
expect_err "q15 c2, undeclared width"          rustc +$TC --edition 2024 --cfg c2 --crate-type lib q15_ceiling_and_alias_site.rs --out-dir build
echo

echo "unexpected outcomes: $unexpected"
exit $(( unexpected > 0 ))
