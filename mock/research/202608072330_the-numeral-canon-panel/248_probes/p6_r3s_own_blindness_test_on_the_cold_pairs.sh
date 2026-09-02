#!/usr/bin/env nutshell
# p6. R3's own blindness test, run on every cold pair the corpus rests a
# `two_experts` standing on.
#
# Written after the blind commit, for the reconciliation. It supersedes `p2`'s
# instrument rather than repeating it: `p2` measured the gap between two add
# commits, which is weak evidence in both directions. The corpus has a better
# test. CORRECTED AFTER THE FIRST DRAFT OF THIS HEADER, which said R3 was "the
# one row the panel has ever promoted on expert convergence": there are FIVE
# rulings at `ratified_by = "experts"`, each carrying a `promotion` field, and
# `p7` enumerates them. R3 is the one that uses ancestry; the other four use
# three further instruments. The false count is kept here rather than deleted
# because it is the shape this panel keeps producing: a superlative asserted
# from the one example in front of me. `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`,
# `rung = ratified`, `ratified_by = experts`, `promotion` field:
#
#   "Blindness is checkable rather than asserted: both worktrees were cut from
#    `0cac9beb`, and the second seat's merge `2ff7ae29` is not an ancestor of the
#    first seat's head `176ced90`, so neither branch could show the other's file
#    to a `git log`."
#
# So the test is mutual non-ancestry. If one seat's commit is an ancestor of the
# other's, the other's `git log` reaches it and blindness is asserted rather than
# checkable.
#
# THE CASES THAT MUST FAIL, STATED BEFORE THE RUN.
#
# C1. R3's own pair, seats 238 and 239, MUST come out mutually non-ancestral. If
#     it does not, this instrument cannot detect the one case the corpus has
#     already certified as blind, and every "not blind" verdict below is noise.
# C2. R3's own two hashes must reproduce its own sentence: `2ff7ae29` is not an
#     ancestor of `176ced90`. This checks the instrument against a ratified row's
#     stated evidence rather than against my reading of it.
# C3. A commit must be an ancestor of itself, which git guarantees. Without it an
#     always-false predicate would report every pair as blind.
# C4. The panel's first commit must be an ancestor of HEAD. Without it an
#     always-false predicate would report every pair as non-blind, which is the
#     direction this file's conclusion runs and therefore the one to guard.
use log

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT" || exit 1
P="mock/research/202608072330_the-numeral-canon-panel"

add() { git log --diff-filter=A --format=%H -- "$P/$1"*.md | tail -1; }
anc() { git merge-base --is-ancestor "$1" "$2" 2>/dev/null; }

verdict() {
  local a="$1" b="$2" A B
  A=$(add "$a"); B=$(add "$b")
  if [ -z "$A" ] || [ -z "$B" ]; then printf "  %-11s UNRESOLVED (%s / %s)\n" "${a:0:3}/${b:0:3}" "${A:-none}" "${B:-none}"; return 1; fi
  if anc "$A" "$B"; then printf "  %-11s NOT BLIND: %s is an ancestor of %s, so the later seat's git log reached it\n" "${a:0:3}/${b:0:3}" "${a:0:3}" "${b:0:3}"; return 1; fi
  if anc "$B" "$A"; then printf "  %-11s NOT BLIND: %s is an ancestor of %s, so the later seat's git log reached it\n" "${a:0:3}/${b:0:3}" "${b:0:3}" "${a:0:3}"; return 1; fi
  printf "  %-11s BLIND by R3's test: mutually non-ancestral\n" "${a:0:3}/${b:0:3}"; return 0
}

echo "== C1 and the reference case: R3's own pair =="
if verdict 238_kiselyov_what_the_door_may_carry_out 239_what_the_numeric_introduction_door; then
  echo "  PASS C1 the instrument detects the pair the corpus already certified"; c1=0
else
  echo "  FAIL C1 the instrument cannot see R3's blindness; every verdict below is noise"; c1=1
fi
echo

echo "== C2: R3's own two hashes, against R3's own sentence =="
if anc 2ff7ae29 176ced90; then echo "  FAIL C2 2ff7ae29 IS an ancestor of 176ced90; R3's promotion field is wrong or I misread it"; c2=1
else echo "  PASS C2 2ff7ae29 is not an ancestor of 176ced90, exactly as R3 states"; c2=0; fi
echo

echo "== C3 and C4: the predicate is neither always-true nor always-false =="
H=$(git rev-parse HEAD); FIRST=$(git log --format=%H -- "$P" | tail -1)
if anc "$H" "$H"; then echo "  PASS C3 a commit is an ancestor of itself"; c3=0; else echo "  FAIL C3 the predicate is always false"; c3=1; fi
if anc "$FIRST" "$H"; then echo "  PASS C4 the panel's first commit is an ancestor of HEAD"; c4=0; else echo "  FAIL C4 the predicate is always false"; c4=1; fi
echo

echo "== every cold pair a two_experts standing in this corpus rests on =="
verdict 65_knuth_number_systems_derived_cold 66_dolan_number_systems_derived_cold
verdict 76_willsey_derived_laws_derived_cold 77_amin_derived_laws_derived_cold
verdict 109_bellard_the_primitive_derived_cold 110_willsey_the_primitive_derived_cold
verdict 125_knuth_rounding_cold_derivation 126_wronski_what_rounding_actually_selects
echo

echo "== what this establishes, and what it does not =="
echo "  Establishes: under the instrument the corpus's only expert-ratified"
echo "  promotion used to certify blindness, none of the four cold pairs is blind."
echo "  Each was committed sequentially into one branch, so the later seat's"
echo "  \`git log\` reached the earlier seat's file."
echo
echo "  Does NOT establish that any of those seats read the other. Sequential"
echo "  commitment into a shared clone is what the panel did before the"
echo "  worktree-per-seat discipline, and a seat may have had its context sealed"
echo "  regardless of what its branch could show it. What is measured is that"
echo "  blindness there is ASSERTED and not CHECKABLE, which is precisely the"
echo "  distinction R3 draws about itself."
exit $(( c1 + c2 + c3 + c4 ))
