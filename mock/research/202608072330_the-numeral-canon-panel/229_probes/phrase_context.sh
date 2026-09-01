#!/usr/bin/env bash
# Print the full predicate span, with its file and index, for every span whose
# phrase list contains a given key.
#
# The ranking in 183_probes tells you a phrase blocks N spans. It does not tell
# you what the span SAID, and that is the only thing that decides whether the
# phrase names an axis. This prints the sentence the phrase was cut out of.
#
# Controls, written before the run:
#   P1  a key nobody wrote must print nothing (`phase_of_the_moon`).
#   P2  a key everybody wrote must print many (`threads`), and the count must
#       match the census tally for it, or the extractor here disagrees with the
#       extractor the ranking was built on.
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
161_leroy_the_canon_candidate_for_the_primitive.md
164_leroy_the_candidate_revised_against_two_signatures.md
173_leroy_the_canon_candidate_for_the_chain.md
176_leroy_the_candidate_revised_against_two_signatures.md
178_leroy_the_restoration_pass.md"

spans() {
  awk 'BEGIN{RS="";ORS="\n"} {gsub(/\n/," "); print}' "$1" \
    | { grep -E '^\*[^*]|^>' || true; } \
    | { grep -E 'holds? for:' || true; } \
    | sed -E 's/^.*holds? for: //' \
    | sed -E 's/\*\*Argument kind.*//' \
    | sed -E 's/\*\*//g; s/`//g; s/\*//g'
}

want="${1:?usage: phrase_context.sh <exact key>}"
n=0
for f in $FILES; do
  i=0
  while IFS= read -r span; do
    [ -z "$span" ] && continue
    i=$((i+1))
    hit=0
    while IFS= read -r phrase; do
      key=$(printf '%s' "$phrase" \
        | sed -E 's/^ *//; s/ *$//' \
        | sed -E 's/^(and|plus|the|with|of) //' \
        | sed -E 's/ (=|in|any|>=).*//' \
        | sed -E 's/\.$//' | sed -E 's/ *$//')
      [ "$key" = "$want" ] && hit=1
    done < <(printf '%s\n' "$span" | awk -f split_predicate.awk)
    if [ "$hit" = 1 ]; then
      n=$((n+1))
      printf '%s #%s\n    %s\n\n' "${f%%_*}" "$i" "$span"
    fi
  done < <(spans "$PANEL/$f")
done
printf '### spans carrying key %s: %s\n' "$want" "$n"
