#!/usr/bin/env nutshell
# p2. Whether any multi-arrival row has MECHANICAL independence evidence.
#
# `proposal::a_strategy_is_an_assignment_and_a_weighting` states the panel's own
# instrument for independence and its limit in one sentence: "The commit
# ordering establishes the within-file half for each and nothing about the
# between-file half, and for one member it runs the wrong way: its phase one
# landed two minutes after the other member's file was already in the tree."
#
# So the test is: for a row citing two author files, was the second one already
# able to read the first when it landed? If yes, blindness rests on the members'
# own coverage statements and on nothing a reader can check.
#
# The window is deliberately generous. A member commits its blind phase and then
# reads; the interesting quantity is the gap between the two ADD commits, so
# anything under a working session is no evidence at all.
#
# THE CASES THAT MUST FAIL, STATED BEFORE THE RUN.
#
# C1. Consolidation 74 compresses member files 65 and 66, so it MUST land after
#     both. If the comparison reports 74 first, the ordering arithmetic is
#     inverted and every verdict below is backwards.
# C2. A file that does not exist must report MISSING. A `git log` over a bad
#     path prints nothing and an unguarded pipeline reads that as epoch zero,
#     which would make every pair look enormously separated.
# C3. The gaps over the whole cited corpus must not be constant. If every pair
#     comes out at the same number this is reading one commit for all of them.
use log

ROOT="$(git rev-parse --show-toplevel)"
P="mock/research/202608072330_the-numeral-canon-panel"

# epoch of the commit that ADDED a panel file, or MISSING
added_at() {
  local stem="$1" path
  path=$(ls "$ROOT/$P/${stem}"*.md 2>/dev/null | head -1)
  [ -z "$path" ] && { echo "MISSING"; return; }
  local out
  out=$(git -C "$ROOT" log --diff-filter=A --format='%ct %H' -- "$path" | tail -1)
  [ -z "$out" ] && { echo "MISSING"; return; }
  echo "$out"
}

pair_report() {
  local a="$1" b="$2" label="$3"
  local ra rb ta tb ha hb
  ra=$(added_at "$a"); rb=$(added_at "$b")
  if [ "$ra" = "MISSING" ] || [ "$rb" = "MISSING" ]; then
    printf "  %-46s %-46s  UNRESOLVED (%s / %s)\n" "$a" "$b" "$ra" "$rb"; return
  fi
  ta=${ra%% *}; ha=${ra##* }
  tb=${rb%% *}; hb=${rb##* }
  local gap=$(( tb - ta )); local absgap=${gap#-}
  local verdict
  if [ "$ha" = "$hb" ]; then verdict="SAME COMMIT: no ordering at all"
  elif [ "$absgap" -lt 3600 ]; then verdict="under an hour apart: the later one could read the earlier"
  else verdict="over an hour apart"
  fi
  printf "  %-46s %-46s  gap=%ss  %s\n" "$a" "$b" "$absgap" "$verdict"
  echo "$absgap" >> /tmp/p2_gaps.txt
}

: > /tmp/p2_gaps.txt

echo "== C1 control: a consolidation must land after what it compresses =="
T65=$(added_at 65_knuth_number_systems_derived_cold); T74=$(added_at 74_giesen_consolidation_the_number_system_concept)
echo "  65 added at ${T65%% *}   74 added at ${T74%% *}"
if [ "${T74%% *}" -gt "${T65%% *}" ]; then echo "  PASS C1 74 is later than 65"; c1=0; else echo "  FAIL C1 ordering is inverted"; c1=1; fi
echo
echo "== C2 control: a nonexistent file =="
R=$(added_at 999_nobody_wrote_this)
if [ "$R" = "MISSING" ]; then echo "  PASS C2 reports MISSING rather than a number"; c2=0; else echo "  FAIL C2 got '$R'"; c2=1; fi
echo

echo "== the two multi-arrival rows p1 found that reach two authors =="
pair_report 65_knuth_number_systems_derived_cold 66_dolan_number_systems_derived_cold "the_concept_is_closed_and_the_inventory_is_open"
pair_report 76_willsey_derived_laws_derived_cold 77_amin_derived_laws_derived_cold "whether_an_operation_needs_chain_machinery"
echo

echo "== every other cold pair the corpus names, as the population C3 needs =="
pair_report 09_persona_checkpoint 10_lattner_fresh_eyes_on_the_container_derivation ""
pair_report 65_knuth_number_systems_derived_cold 74_giesen_consolidation_the_number_system_concept ""
pair_report 109_bellard_the_primitive_derived_cold 110_willsey_the_primitive_derived_cold ""
pair_report 125_knuth_rounding_cold_derivation 126_wronski_what_rounding_actually_selects ""
pair_report 65_knuth_number_systems_derived_cold 126_wronski_what_rounding_actually_selects ""
echo

echo "== C3 control: the gaps are not constant =="
DISTINCT=$(sort -u /tmp/p2_gaps.txt | wc -l | tr -d ' ')
echo "  distinct gap values observed: $DISTINCT"
if [ "$DISTINCT" -ge 2 ]; then echo "  PASS C3 the instrument discriminates"; c3=0; else echo "  FAIL C3 every pair reports the same gap"; c3=1; fi
echo
echo "== what this establishes =="
echo "  A cold pair whose two files land minutes apart has no MECHANICAL"
echo "  independence evidence: at the moment the second landed, the first was"
echo "  already in the tree and readable. Blindness then rests on the members'"
echo "  own coverage statements, which is what the corpus already says."
exit $(( c1 + c2 + c3 ))
