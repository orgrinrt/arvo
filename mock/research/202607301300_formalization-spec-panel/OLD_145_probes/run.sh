#!/usr/bin/env bash
# Probes for 145. Run from this directory. Output captured in output.txt.
#
#   ./run.sh 2>&1 | tee output.txt
#
# Toolchain is pinned: rustc 1.98.0-nightly (57d06900f 2026-05-27).
set -u
RS="rustup run nightly-2026-05-28 rustc"
GCA="--edition 2021 --crate-type lib -Znext-solver=globally"
BARE="--edition 2021 --crate-type lib"

banner() { echo; echo "=============== $* ==============="; }

banner "o1_order.rs  (expected: all zero failures)"
$RS -O o1_order.rs -o o1_order && ./o1_order

banner "e1_from_overlap.rs  (EXPECTED TO FAIL: E0119 against core's From<T> for T)"
$RS $BARE e1_from_overlap.rs; echo "exit=$?"

banner "e2_from_conditioned.rs  (expected: exit 0, a closed witness rescues coherence)"
$RS $BARE e2_from_conditioned.rs; echo "exit=$?"

banner "e3_embed_gca.rs  (EXPECTED TO FAIL: a computed witness does NOT rescue coherence)"
$RS $GCA e3_embed_gca.rs; echo "exit=$?"

banner "e4_routes_after_refusal.rs  (expected: exit 0, three routes)"
$RS $GCA e4_routes_after_refusal.rs; echo "exit=$?"

banner "e5_refused.rs  (EXPECTED TO FAIL: the antichain pair, both directions)"
$RS $GCA e5_refused.rs; echo "exit=$?"

banner "e6_named_verdict.rs  (EXPECTED TO FAIL: same, with the tag number removed)"
$RS $GCA e6_named_verdict.rs; echo "exit=$?"

banner "h1_heterogeneous.rs  (expected: exit 0, under the gates)"
$RS $GCA h1_heterogeneous.rs; echo "exit=$?"

banner "h2_heterogeneous_gatefree.rs  (expected: exit 0, NO gates, NO next-solver)"
$RS $BARE h2_heterogeneous_gatefree.rs; echo "exit=$?"

banner "h3_negative.rs  (EXPECTED TO FAIL: two wrong output annotations)"
$RS $BARE h3_negative.rs; echo "exit=$?"

banner "x1_cv1_gap.rs  (expected: exit 0, which is the finding)"
$RS $BARE x1_cv1_gap.rs; echo "exit=$?"

banner "q1_quantise.rs  (expected: C1 C2 C3 zero failures, C4 many disagreements)"
$RS -O q1_quantise.rs -o q1_quantise && ./q1_quantise

banner "q2_retag.rs  (expected: many non-commuting, zero on the diagonal and Warm<->Cold)"
$RS -O q2_retag.rs -o q2_retag && ./q2_retag
rm -f o1_order q1_quantise q2_retag *.rlib
