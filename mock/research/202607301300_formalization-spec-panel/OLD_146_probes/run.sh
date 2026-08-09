#!/usr/bin/env bash
# Probes for 146. Run from this directory. Output captured in output.txt.
#
#   ./run.sh 2>&1 | tee output.txt
#
# Toolchain is pinned: rustc 1.98.0-nightly (57d06900f 2026-05-27).
set -u
RS="rustup run nightly-2026-05-28 rustc"
GCA="--edition 2021 --crate-type lib -Znext-solver=globally"
BARE="--edition 2021 --crate-type lib"

banner() { echo; echo "=============== $* ==============="; }

banner "o1_order_general.rs  (the order, generalised past the dyadic slice)"
$RS -O o1_order_general.rs -o o1_order_general && ./o1_order_general

banner "o2_ranged_order.rs  (the order for Ranged numerals, the third instance)"
$RS -O o2_ranged_order.rs -o o2_ranged_order && ./o2_ranged_order

banner "n1_quantise_key.rs  (the quantiser rebuilt, and the adjudicating strategy)"
$RS -O n1_quantise_key.rs -o n1_quantise_key && ./n1_quantise_key

banner "n1b_with_degenerate.rs  (same, with the degenerate shape, to match 145's counts)"
$RS -O n1b_with_degenerate.rs -o n1b && ./n1b | head -6

banner "f01_baseline.rs  (EXPECTED E0119: the naive blanket, by value)"
$RS $BARE f01_baseline.rs; echo "exit=$?"

banner "f02_ref_source.rs  (EXPECTED E0277 on plain_into only: the impl itself is coherent)"
$RS $GCA f02_ref_source.rs; echo "exit=$?"

banner "f03_ref_source_full.rs  (expected exit 0: the spelling that works)"
$RS $GCA f03_ref_source_full.rs; echo "exit=$?"

banner "f04_ref_negative.rs  (EXPECTED TO FAIL: the antichain pair, both directions)"
$RS $GCA f04_ref_negative.rs; echo "exit=$?"

banner "f05_peano_strict.rs  (EXPECTED E0119: structural order over type-level widths)"
$RS $GCA f05_peano_strict.rs; echo "exit=$? (next solver)"
$RS $BARE f05_peano_strict.rs; echo "exit=$? (default solver)"

banner "f06_inherent_into.rs  (expected exit 0: an inherent into shadows the trait method)"
$RS $GCA f06_inherent_into.rs; echo "exit=$?"

banner "f07_tryfrom_on_top.rs  (EXPECTED E0119: TryFrom cannot sit beside the From)"
$RS $GCA f07_tryfrom_on_top.rs; echo "exit=$?"

banner "f08_negative_impls.rs  (EXPECTED E0751 + E0119: negative reasoning without its coherence half)"
$RS $GCA f08_negative_impls.rs; echo "exit=$?"

banner "f09_tryfrom_ref_alone.rs  (expected exit 0: TryFrom alone is coherent)"
$RS $GCA f09_tryfrom_ref_alone.rs; echo "exit=$?"

banner "f10_tryfrom_value_alone.rs  (EXPECTED E0119: TryFrom by value is not)"
$RS $GCA f10_tryfrom_value_alone.rs; echo "exit=$?"

banner "f11_both_axes.rs  (expected exit 0: one blanket over numeral and strategy)"
$RS $GCA f11_both_axes.rs; echo "exit=$?"

banner "f12_codegen.rs  (the borrow must survive to nothing)"
$RS -O --emit asm --crate-type lib -Znext-solver=globally f12_codegen.rs -o f12_codegen.s
grep -E "^_(scalar|loop)_via_conversion = " f12_codegen.s

banner "f13_min_specialization.rs  (EXPECTED E0119)"
$RS $GCA f13_min_specialization.rs; echo "exit=$?"

banner "f14_strategy_projection.rs  (EXPECTED E0119)"
$RS $GCA f14_strategy_projection.rs; echo "exit=$?"

banner "f15_inherent_cost.rs  (EXPECTED E0308: what the inherent into costs)"
$RS $GCA f15_inherent_cost.rs; echo "exit=$?"

banner "f16_question_mark.rs  (EXPECTED E0277: ? does not reach a by-reference From)"
$RS $GCA f16_question_mark.rs; echo "exit=$?"

rm -f o1_order_general o2_ranged_order n1_quantise_key n1b *.rlib
