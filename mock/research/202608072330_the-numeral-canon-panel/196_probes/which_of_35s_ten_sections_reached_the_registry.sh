#!/usr/bin/env nutshell
# `191` measured three of `35`'s ten requirement sections and reported the
# result as a verdict on `35`. What do the other seven say?
#
# WHY THIS RUNS. `191_probes/which_of_35s_figures_survived.sh` takes "fourteen
# figures from `35` sections 3.4, 3.5 and 3.5a". `35` states ten requirements,
# 3.1 through 3.10, and its own section 3 opening says so. Three of ten, all
# three from the tropical-and-order half, and `191` section 2.1 concludes from
# them that "the port kept those two files' retirements in full and dropped
# almost every positive result they established".
#
# That is a law asserted over a sample of the shapes the thing has. The three
# sampled sections are the ones whose subject is the ALGEBRA. The sections whose
# subject is the COMPOSITION CONTRACT, which is what I11's second half names and
# what the unit was dispatched about, are 3.1, 3.2 and 3.8, and none was
# sampled.
#
# So this widens the sample to all ten and reports per section. It is the same
# instrument shape as `191`'s and deliberately so: a distinctive string per
# section, searched over `mock/registry/` whole. Where `191`'s finding is
# reproduced this file says so.
#
# THE CASE THAT MUST FAIL, four controls, and the run does not count without all.
#   POS-A  `476` and `897`, the coherence witness counts, must be present. Same
#          control `191` used, so a divergence means the greps differ rather
#          than the corpora.
#   POS-B  `12.6` must be present. It is a `35` figure that survived, so it
#          proves a `35` figure CAN survive and the absences are about selection.
#   NEG    `999999999` must be absent.
#   REPRO  section 3.4's `63 of 63` must be ABSENT, reproducing `191`. If it
#          comes back present, this run disagrees with `191` on data rather than
#          on reading and the difference must be resolved before anything else.
#
# HONESTY BOUND, stated because it is the same bound `191`'s instrument had and
# it did not state it: a string search measures whether a FIGURE travelled. A
# fact re-established later by a different seat with a different instrument at
# different widths will read as ABSENT here and be present in the corpus. So
# every ABSENT below is a claim about `35`'s figure and NOT about the canon's
# knowledge, and section-level verdicts are read with that in mind. Where this
# file makes a claim about the canon's knowledge it opens the row.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
[ -f "$root/mockspace.toml" ] || { echo "run me from inside the repository" >&2; exit 2; }
reg="$root/mock/registry"
src="$root/mock/research/202608072330_the-numeral-canon-panel/35_mcsherry_what_the_layers_above_need_from_the_numeral.md"
[ -f "$src" ] || { echo "35 not found" >&2; exit 2; }
echo "### registry: $reg"
echo "### source:   $(basename "$src")"
echo

hits() { { grep -ro -- "$1" "$reg"/*.toml 2>/dev/null || true; } | wc -l | tr -d " "; }
where() { { grep -rn -- "$1" "$reg"/*.toml 2>/dev/null || true; } | sed "s|$reg/||" | cut -c1-96 | head -3; }

row() { # $1 label  $2 needle  $3 note
  n=$(hits "$2")
  if [ "$n" -gt 0 ]; then st="present"; else st="ABSENT "; fi
  printf "  %-8s %-7s %-26s reg=%-3s %s\n" "$1" "$st" "$2" "$n" "$3"
  if [ "$n" -gt 0 ]; then where "$2" | sed 's/^/             /'; fi
}

echo "######## 191 SAMPLED THESE THREE ########"
row 3.4 "63 of 63"    "top absorbs under saturation"
row 3.5 "33 of 33"    "monotonicity split"
row 3.5a "560 of 2176" "reserved top monotonicity failures"
echo
echo "######## 191 SAMPLED NONE OF THESE SEVEN ########"
row 3.1 "runtime trip count" "the fold/chain boundary as 35 words it"
row 3.2 "log2"          "the accumulator formula W + ceil(log2 C)"
row 3.2 "accfit"        "the bench arm 20 says wins at every arity"
row 3.3 "18 of 126"     "shapes with no multiplicative identity"
row 3.6 "strided"       "a split reduction changes association order"
row 3.7 "6 of 33"       "mul-assoc holds only at F == 0"
row 3.7 "87.5"          "worst distributivity failure rate"
row 3.8 "per-aggregate" "the two-output container derivation"
row 3.9 "49.6"          "retraction lost under saturation"
row 3.10 "EMA"          "the named downstream invariant"
echo
echo "######## CONTROLS ########"
for pair in "POS-A:476" "POS-A:897" "POS-B:12.6"; do
  lbl=${pair%%:*}; ndl=${pair#*:}
  n=$(hits "$ndl")
  if [ "$n" -gt 0 ]; then echo "  $lbl  PASS  $ndl present ($n)"
  else echo "  $lbl  FAIL  *** $ndl absent; the grep is broken ***"; exit 3; fi
done
n=$(hits "999999999")
if [ "$n" -eq 0 ]; then echo "  NEG    PASS  999999999 absent"
else echo "  NEG    FAIL  *** matches everything ***"; exit 3; fi
n=$(hits "63 of 63")
if [ "$n" -eq 0 ]; then echo "  REPRO  PASS  '63 of 63' absent, reproducing 191 section 2.1"
else echo "  REPRO  FAIL  *** disagrees with 191 on data, not on reading ***"; exit 3; fi
echo
echo "### Read the ABSENTs as claims about 35's figures. Section 3.2's substance"
echo "### is in the registry under other words: question.toml Q11 carries the"
echo "### capacity-keyed accumulator as an option, and"
echo "### the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion"
echo "### carries 'any accumulator statement derived from a capacity is an"
echo "### additive-only mechanism' with a predicate and an answers edge to Q11."
echo "### A figure search cannot see either. That is the bound above, live."
echo
echo "### A FALSE POSITIVE IN THIS FILE'S OWN OUTPUT, kept rather than tuned away."
echo "### Section 3.7's '87.5' reports present. It is NOT 35's distributivity"
echo "### failure rate. The hit is proposal.toml:1374, a gap field on a row about"
echo "### composed add-and-subtract associativity under box lifting, whose own"
echo "### keywords carry 21.98 and R8. A three-character decimal is not"
echo "### distinctive, which is the defect 191 found in its own 43 arm and fixed"
echo "### by printing context. The context is printed above; the reader has to"
echo "### read it. I did not, for one draft."
echo "###"
echo "### 35 section 3.7's result DID reach the canon, and not by its figures:"
echo "### law.toml:145 distributivity_of_multiplication_over_addition carries a"
echo "### holds region, a fails region, a witness at 47.72 and 34.52 from two"
echo "### independently written later models, and a note naming 35:311 as one of"
echo "### three places the F = 0 qualifier was lost. Open the row."
