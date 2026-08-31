#!/usr/bin/env bash
# `the_committed_canon_defines_no_term_twice` is green. This asks what it is
# green over, and the answer turned out to be two different nothings.
#
# CASE THAT MUST FAIL: relabel the rival strategy rows on a COPY so the check's
# selection is non-empty, and watch the finding appear. **It did not appear**,
# and that failure is the second half of this probe rather than a reason to
# tune it: the check skips any row carrying a `supersedes`, and one of the two
# rivals supersedes a third, unrelated row.
set -uo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
P="$ROOT/mock/registry/proposal.toml"
S="$ROOT/mock/checks/src/shape.rs"

echo "### PART ONE: the selection is empty"
printf '  rows with sentence_kind = "definition" : %s\n' "$(grep -c 'sentence_kind = "definition"' "$P" || true)"
printf '  rows carrying a `defines` field        : %s\n' "$(grep -c '^defines = ' "$P" || true)"
printf '  total proposal rows                    : %s\n' "$(grep -c '^\[\[proposal\]\]' "$P")"
echo "  The check reads the defines field on rows marked definition. There are none of"
echo "  either, so the green is over an empty selection."
echo
echo "  The kind postdates the rows, so this is not the porting seat's error:"
git -C "$ROOT" log -1 --format='    kind added:  %h %ci %s' -- "$S"
git -C "$ROOT" log -1 --format='    rows landed: %h %ci %s' -- mock/registry/proposal.toml

echo
echo "### PART TWO: the term the corpus does define more than once"
awk '/^\[\[proposal\]\]/{id="";sup="(none)"} /^id = /{s=$0;sub(/^id = "/,"",s);sub(/"$/,"",s);id=s}
     /^supersedes = /{sup="yes"} /^keywords/{ if (id ~ /^(a_strategy_is|the_named_strategies)/) printf "  %-84s supersedes: %s\n", id, sup }' "$P"
echo
echo "  Three of those four state what a strategy is. One supersedes another."
echo "  Two are live rivals: the declared-semantics row and the preference row."

echo
echo "### PART THREE: the case that had to fail, and did not fire"
T=$(mktemp -d)
sed -e '/^id = "a_strategy_is_a_declared_semantics_together_with_a_weighting_over_the_arms_that_realise_it"/,/^keywords/ s/^sentence_kind = "normative"/sentence_kind = "definition"\ndefines = "strategy"/' \
    -e '/^id = "a_strategy_is_a_preference_over_measurements_resolved_as_a_compile_time_argmin"/,/^keywords/ s/^sentence_kind = "normative"/sentence_kind = "definition"\ndefines = "strategy"/' \
    "$P" > "$T/proposal.toml"
echo "  substitutions landed: $(grep -c 'defines = "strategy"' "$T/proposal.toml" || true)  (must be 2)"
echo
echo "  what the check would then see, applying its own selection by hand:"
awk '/^\[\[proposal\]\]/{id="";def="";sup=""} /^id = /{s=$0;sub(/^id = "/,"",s);sub(/"$/,"",s);id=s}
     /^defines = /{def=$0} /^supersedes = /{sup="yes"}
     /^keywords/{ if(def!="") printf "    %-82s supersedes=%-5s -> %s\n", id, (sup==""?"no":sup), (sup==""?"COUNTED":"SKIPPED") }' "$T/proposal.toml"
rm -rf "$T"
echo
echo "  One row counted for the term, so no finding. The skip is unconditional:"
sed -n '213,216p' "$S" | sed 's/^/    /'
echo
echo "  A row may supersede one definition and still be a live rival to a third."
echo "  That is the committed shape exactly: the declared-semantics row supersedes"
echo "  the pair row and rivals the preference row, and the skip discards it on the"
echo "  strength of the first fact while the second is what the check is for."
echo
echo "  So the check has two independent reasons it cannot see this corpus's one"
echo "  doubly-defined term, and fixing the labels alone would not surface it."
