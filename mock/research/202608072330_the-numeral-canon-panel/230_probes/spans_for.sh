#!/usr/bin/env bash
# Print every predicate span in the eight span-bearing governing files that
# contains a given phrase, with the file it came from. This is a reading aid,
# not a measurement: it exists so a judgement about whether a phrase is an axis
# is made against the sentences the corpus actually wrote rather than against a
# key extracted from them.
#
# The span extractor is `183_probes/span_verdicts.sh`'s, reused verbatim so a
# reading here is over the same spans the ranking ranked.
#
# Control S1: a phrase nobody wrote must print nothing. Checked with
# `phase_of_the_moon`, the same non-axis 183's own controls use.
set -euo pipefail
cd "$(dirname "$0")"
PANEL=..

FILES="119_leroy_the_canon_candidate_for_the_realisation_map.md
122_leroy_the_candidate_revised_against_two_partial_signatures.md
132_leroy_the_canon_candidate_for_the_rounding_axis.md
136_leroy_the_candidate_revised_against_three_signatures.md
138_leroy_the_restoration_pass.md
146_leroy_the_canon_candidate_for_the_strategy_object.md
151_leroy_the_candidate_revised_against_four_signatures.md
178_leroy_the_restoration_pass.md"

spans() {
  awk 'BEGIN{RS="";ORS="\n"} {gsub(/\n/," "); print}' "$1" \
    | { grep -E '^\*[^*]|^>' || true; } \
    | { grep -E 'holds? for:' || true; } \
    | sed -E 's/^.*holds? for: //' \
    | sed -E 's/\*\*Argument kind.*//' \
    | sed -E 's/\*\*//g; s/`//g; s/\*//g'
}

want="${1:?usage: spans_for.sh <phrase>}"
n=0
for f in $FILES; do
  i=0
  while IFS= read -r span; do
    [ -z "$span" ] && continue
    i=$((i+1))
    case "$span" in *"$want"*) n=$((n+1)); printf '%s #%s\n  %s\n\n' "${f%%_*}" "$i" "$span" ;; esac
  done < <(spans "$PANEL/$f")
done
printf '### %s spans contain: %s\n' "$n" "$want"
