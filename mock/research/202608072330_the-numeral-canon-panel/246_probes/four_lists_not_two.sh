#!/usr/bin/env bash
# Seat 246. Is the tier ambiguity 241 found the question Q19 asks?
#
# 242 section 9 files the one-word-two-tiers defect against
# question::are_the_level_hierarchies_the_same_cut, and 244's C2 carries that as
# "the tier count", calling 241's `coordinate` finding "the same seam from the
# other side". If those are two names for one question, answering Q19 closes it.
# If they are two questions, then the sitting's second contested item points at
# a row that decides nothing about it.
#
# Four candidate lists are in play and this instrument prints all four from
# their own sources rather than from anybody's summary of them:
#
#   A  65's three levels                     (Q19's first option)
#   B  66's five levels                      (Q19's second option)
#   C  67's five chain components, carried by 74 section 3.1
#   D  R3's ten identity coordinates, as the shipped traits declare them
#
# Q19 is A against B. 241's seam is C against D. If A, B, C and D are four
# distinct lists then Q19's pair is not 241's pair.
#
# THE CASE THAT MUST FAIL: a set comparison that calls everything distinct is
# useless. C1 feeds the comparison one list against itself and requires it to
# report identity; C2 feeds it two lists known to share a member and requires
# the shared member to be reported. Without both, "four distinct lists" is a
# fact about the comparison.

set -u
cd "$(dirname "$0")/.." || exit 1   # the panel directory
fail() { echo "CONTROL FAILED: $1"; exit 2; }

A="system representation format"
B="number system representation_scheme format container"
C="ambient_domain representable_set reduction encoding container"

echo "=== the four lists, each quoted from its own source ==="
echo
echo "A. 65's three levels, from OPTIONS.md's statement of Q19:"
grep -n "65\` proposes three levels" OPTIONS.md | sed 's/^/    /'
echo "B. 66's five levels, same line:"
grep -n "proposes$" OPTIONS.md >/dev/null 2>&1 || true
sed -n '1598,1599p' OPTIONS.md | sed 's/^/    /'
echo
echo "   and the registry row's own options, which is what a reader gates on:"
awk 'BEGIN{RS="\\[\\[question\\]\\]"} /id = "are_the_level_hierarchies_the_same_cut"/{print}' \
  ../../registry/question.toml | awk '/^options = \[/,/^\]/' | sed 's/^/    /'
echo
echo "C. 67's five chain components, from 74 section 3.1:"
sed -n '119,123p' 74_giesen_consolidation_the_number_system_concept.md | sed 's/^/    /'
echo
echo "D. R3's ten identity coordinates, from the shipped trait declarations."
echo "   ADMITTED is excluded: it carries a default body, so no candidate chooses it."
D=""
for f in ambient quantum slots format; do
  # Declarations only. Doctest bodies inside /// lines are indented differently and
  # are dropped by the leading-four-spaces anchor; ADMITTED is dropped by name.
  cs=$(grep -oE '^    const [A-Z_]+:' "../../crates/arvo-format/src/$f.rs" \
       | sed 's/    const //;s/://' | grep -v '^ADMITTED$' | sort -u | tr '\n' ' ')
  printf '    %-10s %s\n' "$f.rs" "$cs"
  D="$D $cs"
done
nd=$(printf '%s\n' $D | sort -u | grep -c .)
printf '    total, deduplicated : %d\n' "$nd"
[ "$nd" -eq 10 ] || fail "the trait declarations do not come to ten; the extraction is wrong or the crate moved."
echo

echo "=== the comparison ==="
common() { comm -12 <(printf '%s\n' $1 | sort) <(printf '%s\n' $2 | sort) | tr '\n' ' '; }
printf '  A n B : %s\n' "$(common "$A" "$B")"
printf '  A n C : %s\n' "$(common "$A" "$C")"
printf '  B n C : %s\n' "$(common "$B" "$C")"
echo
echo "  D is a list of associated constants and A, B and C are lists of concept"
echo "  levels, so a set intersection between them is not defined at the level of"
echo "  the names. What relates D to C is stated by ratified rows, below."
echo

# C1: the comparison must report identity on a list against itself.
self=$(common "$C" "$C")
[ "$(printf '%s\n' $self | grep -c .)" -eq 5 ] || fail "C1, a list compared with itself did not come back whole."
echo "  C1 passes: C against itself returns all five members, so the comparison can see a match."
# C2: two lists known to share a member must report it.
[ "$(common "$B" "$C")" = "container " ] || fail "C2, B and C share 'container' and the comparison did not report exactly that."
echo "  C2 passes: B and C share exactly 'container', which the comparison reports."
echo
echo "  VERDICT: Q19 asks about A against B. 241's seam is C against D."
echo "  A and C share nothing. B and C share one word, 'container', which is why"
echo "  the two pairs are easy to conflate and why they are not the same pair."
echo

echo "=== what the ratified rows already say about C against D ==="
echo "  R1, ruling::the_format_spine_is_canon, ratifies:"
awk 'BEGIN{RS="\\[\\[proposal\\]\\]"} /id = "a_format_is_identified_by_its_ambient_domain_and_its_representable_set"/{print}' \
  ../../registry/proposal.toml | grep '^says' | fold -s -w 92 | sed 's/^/    /'
echo
echo "  R2, ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule:"
awk 'BEGIN{RS="\\[\\[ruling\\]\\]"} /id = "the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule"/{print}' \
  ../../registry/ruling.toml | grep '^says' | fold -s -w 92 | sed 's/^/    /' | head -3
echo
echo "  question::adaptation_in_identity_or_realisation, its answered field:"
awk 'BEGIN{RS="\\[\\[question\\]\\]"} /id = "adaptation_in_identity_or_realisation"/{print}' \
  ../../registry/question.toml | grep '^answered' | fold -s -w 92 | sed 's/^/    /' | head -4
echo
echo "  So C's component 3 (the reduction, which is the adaptation) and component"
echo "  4 (the encoding) are ratified out of identity by R1, and component 5 (the"
echo "  container) is ratified as placement rather than semantics by R2. That"
echo "  leaves components 1 and 2 as identity, and D refines exactly those two."
echo "  The relation between C and D is therefore already settled by ratified"
echo "  text. What is not settled is which of the two a canon sentence uses."
echo
echo "=== is there a row asking which vocabulary a canon sentence uses ==="
echo "  every the_number_system row whose text contains any of the phrases the"
echo "  question would be written in, with what that row actually asks, because a"
echo "  bare count would not let a reader see that the hit is incidental:"
awk 'BEGIN{RS="\\[\\[question\\]\\]"} /topic = "the_number_system"/{
  if ($0 ~ /identity coordinate|chain component|which vocabulary|coordinate.*means/) {
    match($0,/id = "[a-z0-9_]+"/); i=substr($0,RSTART+6,RLENGTH-7);
    match($0,/asks = "[^"]*"/); a=substr($0,RSTART+8,RLENGTH-9);
    printf "    %s\n      asks: %s\n", i, substr(a,1,88)
  }
}' ../../registry/question.toml
ctl=$(awk 'BEGIN{RS="\\[\\[question\\]\\]"} /topic = "the_number_system"/{print}' ../../registry/question.toml \
    | grep -ciE 'level' || true)
[ "$ctl" -gt 0 ] || fail "C3, the same grep finds no mention of 'level' either, so its result says nothing."
echo "  C3 passes: the same grep over the same rows finds 'level' $ctl times, so it can find a phrase."
