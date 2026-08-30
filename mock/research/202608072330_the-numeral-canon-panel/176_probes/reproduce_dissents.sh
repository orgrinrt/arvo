#!/bin/sh
# 176: reruns every instrument the two signatures rest on and diffs against the
# committed outputs. CONTROL (must fail): 174 r1's output diffed against 175
# clause23's output must DIFFER; if it reports REPRODUCES the differ is broken.
cd "$(dirname "$0")/.."
S="${TMPDIR:-/tmp}/176repro.$$"; mkdir -p "$S"
r() { if diff -q "$2" "$3" >/dev/null 2>&1; then echo "$1 : REPRODUCES"; else echo "$1 : DIFFERS"; fi }

# 174 r1: finiteness against the deferral theorem (compiled)
rustc --edition 2021 -O -o "$S/r1" 174_probes/r1_finiteness_and_the_deferral_theorem.rs 2>/dev/null && "$S/r1" > "$S/a"
r "174 r1 (finiteness, 663/17/160)" "$S/a" 174_probes/r1_finiteness_and_the_deferral_theorem.out
# 174 r2: profile-dependent definedness (both flags)
( cd 174_probes && for m in off on; do
    rustc -Copt-level=1 -Cdebug-assertions=$m -o "$S/r2_$m" r2_clause1_needs_the_profile_bound.rs 2>/dev/null
    "$S/r2_$m"; echo
  done > "$S/b" )
r "174 r2 (profile moves definedness)" "$S/b" 174_probes/r2_clause1_needs_the_profile_bound.out
# 175 marks: the trailing convention is forced
python3 175_probes/marks/marks.py > "$S/c" 2>&1
r "175 marks (trailing convention forced)" "$S/c" 175_probes/marks/marks.out
# 175 clause23: the family of boundary functions
python3 175_probes/clause23/clause23.py > "$S/d" 2>&1
r "175 clause23 (boundary-function family)" "$S/d" 175_probes/clause23/clause23.out
# 175 partial: the (x*x)/x definedness construction, both flags
# NOTE first version omitted the runner's own "##########" header lines and
# reported DIFFERS against a byte-identical binary output: a harness defect in
# this script, the exact class 171 3.1 named, caught by reading the diff.
( cd 175_probes/partial && {
    echo "########## debug-assertions=off"
    rustc --edition 2024 -Copt-level=3 -Cdebug-assertions=off -o "$S/p3_off" partial3.rs 2>/dev/null
    "$S/p3_off"; echo
    echo "########## debug-assertions=on"
    rustc --edition 2024 -Copt-level=3 -Cdebug-assertions=on -o "$S/p3_on" partial3.rs 2>/dev/null
    "$S/p3_on"
  } > "$S/e" )
r "175 partial3 (split 1 in 4096, both profiles)" "$S/e" 175_probes/partial/partial3.out
# 175 options: the option census
sh 175_probes/options/options.sh > "$S/f" 2>&1
r "175 options (O-171-1 at zero)" "$S/f" 175_probes/options/options.out
# A6/B7: the stale strings, verified at source
h1=$(head -1 173_probes/anchor_accounting/dropped_anchors.txt)
h2=$(sed -n 2p 173_probes/anchor_accounting/count_anchors.py)
case "$h1$h2" in *"13 source files"*|*"thirteen source files"*) echo "A6 stale strings : CONFIRMED PRESENT";; *) echo "A6 stale strings : NOT FOUND (already repaired?)";; esac
# A3/B4: the disclaimer's presence and absence
c60=$(tr '\n' ' ' < 60_stam_the_chain_derived_cold.md | grep -c "statability argument, not a benchmark")
c173=$(tr '\n' ' ' < 173_leroy_the_canon_candidate_for_the_chain.md | grep -c "statability argument, not a benchmark")
echo "A3 disclaimer: in 60 = $c60 (want 1), in 173 = $c173 (want 0)"
echo "CONTROL (r1 vs clause23, must DIFFER):"
r "  control" "$S/a" "$S/d"
rm -rf "$S"
