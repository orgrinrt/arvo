#!/usr/bin/env nutshell
# Are the five plan-chain obligations five needs, or one crate list rewritten?
# And which plan step falls through the gap that question opens?
#
# WHY THIS RUNS. `184` states its own method: "every row below is written as a
# need and none of them names an arvo crate. Where a consumer's document names
# only the crate and never the use, no row is written and the gap is recorded."
# The rows honour the letter of that; none contains a crate name.
#
# But `184`'s source for the plan-chain half is ONE SENTENCE, which it quotes,
# and that sentence is a crate-to-purpose mapping naming FIVE crates. `184` then
# writes "Five needs." If each obligation is that sentence's clause with the
# crate name struck out, the canon's demand side is the DELETED CRATE TREE'S
# DECOMPOSITION with the names filed off, which is the one thing this unit was
# told not to reason from. `191` names that shape and rejects it as its
# alternative 3, then builds its class B on one of these rows.
#
# The consequence is sharper than the provenance. The chain has twelve canonical
# steps and the sentence names five crates, so SEVEN steps have no crate. A
# demand side derived from the sentence cannot see them, and neither `184` nor
# `191` has a row or a class for the one that is not index bookkeeping.
#
# ARM 1  the chain, its own count, and what the obligation rows say it is
# ARM 2  five obligations against five crate clauses, in order
# ARM 3  a_cost_dynamic_program against what the consumer says arvo-comb does
# ARM 4  the twelve canonical stages against 191 section 2.2's five classes
#
# THE CASE THAT MUST FAIL, and the run does not count without all four.
#   POS-1 the twelve arrow-separated stages must parse, or ARM 4 measures nothing
#   POS-2 the dependency sentence must be found, or ARM 2 has no left-hand side
#   NEG   the clause splitter must match an invented `arvo-zzz for ...` clause,
#         proving it is not keyed on the five real names and ARM 2 is not circular
#   DISC  if ARM 3 shows the obligation carrying a purpose the consumer states
#         independently of the crate, the rewrite reading is WRONG for that row.
#         The arm prints every statement of that crate's purpose so the outcome
#         is reachable.
#
# PATHS. The consumer is resolved from this repository's parent, never from a
# home path: the corpus's tenth instrument-defect class is a committed probe
# that reads somebody else's checkout and reports about a different tree.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
[ -f "$root/mockspace.toml" ] || { echo "run me from inside the repository" >&2; exit 2; }
reg="$root/mock/registry"
hila="$(dirname "$root")/hilavitkutin"
[ -d "$hila" ] || { echo "hilavitkutin not cloned beside this repo; clone it and re-run" >&2; exit 2; }
top="$hila/mock/DESIGN.md.tmpl"
echo "### registry: $reg"
echo "### consumer: $hila"
echo

echo "======== ARM 1. the chain, and what the obligations say it is ========"
echo "   the consumer's own heading:"
{ grep -n '^### Plan algorithm chain' "$top" || true; } | sed 's/^/      /'
stages=$( { grep -o 'build_dag →[^`]*' "$top" || true; } | head -1 | tr '→' '\n' | sed 's/^ *//; s/ *$//' | grep -c . || true)
echo "   arrow-separated canonical stages: $stages"
if [ "$stages" -lt 2 ]; then echo "   POS-1 FAIL *** stages did not parse ***"; exit 3; fi
echo "   what the obligation rows say:"
{ grep -n 'twelve-step\|twelve plan' "$reg/obligation.toml" || true; } | cut -c1-130 | sed 's/^/      /'
echo

echo "======== ARM 2. five obligations against five crate clauses ========"
clauses=$( { grep 'Each step names its foundations crate dependency' "$top" || true; } | { grep -oE 'arvo-[a-z]+.? for [^,.`]*' || true; })
echo "   the sentence's clauses:"
printf '%s\n' "$clauses" | sed 's/^/      /'
echo "   clause count: $(printf '%s\n' "$clauses" | grep -c . || true)"
echo
echo "   the plan-chain obligation ids, in file order, first five:"
awk '/^\[\[obligation\]\]/{f=1;next} f && /^id = "/{print "      " $0; f=0}' "$reg/obligation.toml" | head -5
echo

echo "======== ARM 3. what the consumer says arvo-comb is for ========"
echo "   the obligation's need, as the canon carries it:"
{ grep -A2 'id = "a_cost_dynamic_program"' "$reg/obligation.toml" || true; } | grep '^need' | cut -c1-170 | sed 's/^/      /'
echo "   every statement of that crate's purpose in the consumer, lockfiles and"
echo "   imported prior art excluded:"
{ grep -rn 'arvo-comb' "$hila/mock" 2>/dev/null || true; } \
  | grep -v 'Cargo.lock\|Cargo.toml\|research/imported\|/target/\|Binary file' \
  | grep -iE 'DP|dynamic|greedy|bin-pack|matrix|grouping' \
  | sed "s|$hila/||" | cut -c1-175 | sed 's/^/      /'
echo

echo "======== ARM 4. the twelve stages against 191's five classes ========"
# 191 section 2.2 assigns by step number:
#   A = 1 2 4 5 6 11 12 13   B = 3 8   C = 7   D = 9   E = the content hash
cls() { case "$1" in 1|2|4|5|6|11|12) echo A;; 3|8) echo B;; 7) echo C;; 9) echo D;; *) echo "-- UNASSIGNED";; esac; }
i=0
printf '%s\n' "$( { grep -o 'build_dag →[^`]*' "$top" || true; } | head -1 | tr '→' '\n' | sed 's/^ *//; s/ *$//' )" | while read -r s; do
  [ -z "$s" ] && continue
  i=$((i+1))
  printf "   %2s  %-16s  class %s\n" "$i" "$s" "$(cls $i)"
done
echo
echo "   the unassigned stage's own vocabulary, in the consumer:"
{ grep -rn 'MAX_FUSE' "$hila/mock" 2>/dev/null || true; } | grep -i 'strategy\|picks among\|config selection' | sed "s|$hila/||" | cut -c1-165 | sed 's/^/      /'
echo

echo "======== CONTROLS ========"
if printf '%s\n' "$clauses" | grep -q 'arvo-bitmask'; then
  echo "   POS-2  PASS  the dependency sentence parsed"
else
  echo "   POS-2  FAIL  *** ARM 2 has no left-hand side ***"; exit 3
fi
if printf 'x: `arvo-bitmask` for set ops, `arvo-zzz` for the nothing step.\n' | { grep -oE 'arvo-[a-z]+.? for [^,.`]*' || true; } | grep -q 'arvo-zzz'; then
  echo "   NEG    PASS  the splitter matches an invented clause, so ARM 2's five are"
  echo "                the document's five and not a list this probe carried in"
else
  echo "   NEG    FAIL  *** splitter is keyed on the real names; ARM 2 is circular ***"; exit 3
fi
