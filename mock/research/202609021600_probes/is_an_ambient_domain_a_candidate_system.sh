#!/usr/bin/env bash
# Q31 attack probe. Both seats' leading argument needs one unstated premise: that
# an ambient domain is itself a candidate for the predicate "is a number system",
# so that folding residue into that predicate would make "the rationals" a
# non-system and empty the ratified sentences that name one.
#
# This asks the registry, through the harness's own reader of `canon_paths`,
# what an ambient domain is filed as. It is a reading of returned text and is
# marked as one; what the probe supplies is that the text is the registry's and
# is quoted rather than remembered.
#
# Controls, both required:
#   POSITIVE  a phrase known to be in the registry must come back non-zero, or the
#             query is not reaching the rows and every zero below is about the tool.
#   NEGATIVE  a phrase known to be absent must come back zero, or the matcher is
#             matching everything.

set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
OUT="$HERE/output_is_an_ambient_domain_a_candidate_system.txt"
q() { ( cd "$ROOT" && cargo mock query "$1" 2>&1 | grep -v '^ *Finished' ); }

{
echo "probe: is an ambient domain a candidate system, or a coordinate of one?"
echo "tree:  $(git -C "$ROOT" rev-parse --short HEAD)"
echo

echo "--- controls ---"
printf 'POSITIVE  proposal says~"ambient domain"        -> %s (must be > 0)\n' "$(q 'proposal.where(says~ambient domain).count()')"
printf 'POSITIVE  proposal says~"representable set"     -> %s (must be > 0)\n' "$(q 'proposal.where(says~representable set).count()')"
printf 'NEGATIVE  proposal says~"the surreal numbers"   -> %s (must be 0)\n'   "$(q 'proposal.where(says~the surreal numbers).count()')"
printf 'NEGATIVE  proposal says~"quaternion"            -> %s (must be 0)\n'   "$(q 'proposal.where(says~quaternion).count()')"
echo

echo "--- every row whose says names an ambient domain, with its standing ---"
q 'proposal.where(says~ambient domain).select(id,standing,topic)'
echo

echo "--- the two the ratified spine carries, in full ---"
for r in arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation \
         a_format_is_identified_by_its_ambient_domain_and_its_representable_set; do
  echo "== proposal::$r"
  q "proposal::$r::says"
  echo
done

echo "--- what the concept's own sequence puts the ambient domain at ---"
echo "== proposal::the_numeral_concept_is_a_dependent_sequence_of_choices"
q 'proposal::the_numeral_concept_is_a_dependent_sequence_of_choices::says'
echo
echo "== proposal::a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts"
q 'proposal::a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts::says'
echo
echo "== dimension::ambient_domain, what and grammar"
q 'dimension::ambient_domain::what'
q 'dimension::ambient_domain::grammar'
echo

echo "--- the row that ratifies the spine, so the tier is checkable ---"
q 'ruling.where(id=the_format_spine_is_canon).select(id,rung,ratified_by,topic)'
q 'ruling::the_format_spine_is_canon::ratifies'
} | tee "$OUT"
