#!/usr/bin/env bash
# The same key extractor, over every panel file that writes a predicate, rather
# than over the twelve governing files of the five later topics.
#
# Why. `183_probes/axis_census.sh` sweeps twelve files and `unblock_value.sh`
# ranks the undeclared keys it finds. Four of those twelve produce zero spans
# (`161`, `164`, `173`, `176`), and `161:70` says why in its own words:
# "Predicates are carried at the establishing file and not restated in full
# here". So topics nine and ten are in the file list and contribute nothing to
# the ranking, and whatever axes their establishing files predicate over were
# never in the sweep at all. This measures how much that is.
#
# The extractor is `183_probes/axis_census.sh`'s, reused verbatim, so a
# difference in the output is a difference in the corpus read and not in the
# instrument.
#
# Controls, outcomes written before the run:
#   W1  the twelve-file arm must reproduce `183_probes/keys.txt` exactly. If it
#       does not, this script's extractor is not that one and nothing below
#       compares.
#   W2  the wide arm must be a strict superset in file count, and must find at
#       least one key the narrow arm does not. If it finds none, the narrow
#       sweep was complete after all and the objection above is void.
#   W3  `phase_of_the_moon` must appear in neither.
#   W4  a key present in the narrow arm must still be present in the wide one.
#       A wide sweep that loses keys is mis-globbing rather than widening.
set -euo pipefail
cd "$(dirname "$0")"
PANEL=..
REG=../../../registry
out=wide_out; rm -rf "$out"; mkdir -p "$out"

NARROW="119_leroy_the_canon_candidate_for_the_realisation_map.md
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

keys_from() {  # $@ = files, relative to PANEL
  for f in "$@"; do spans "$PANEL/$f"; done \
    | awk -f ../183_probes/split_predicate.awk \
    | sed -E 's/^ *//; s/ *$//' \
    | sed -E 's/^(and|plus|the|with|of) //' \
    | sed -E 's/ (=|in|any|>=).*//' \
    | sed -E 's/\.$//' | sed -E 's/ *$//' \
    | grep -v '^$' | sort | uniq -c | sort -rn
}

keys_from $NARROW > "$out/narrow.txt"
WIDE=$(cd "$PANEL" && grep -lE 'holds? for:' ./*.md | sed 's|^\./||' | sort)
keys_from $WIDE > "$out/wide.txt"

nf=$(printf '%s\n' $NARROW | grep -c .)
wf=$(printf '%s\n' "$WIDE" | grep -c .)
echo "### files swept: narrow $nf, wide $wf"
echo "### distinct keys: narrow $(grep -c . "$out/narrow.txt"), wide $(grep -c . "$out/wide.txt")"
echo

DECLARED=$(grep '^id = ' "$REG/dimension.toml" | sed 's/id = "//; s/"//')
is_declared() {  # $1 = key
  slug=$(printf '%s' "$1" | tr 'A-Z ' 'a-z_' | tr -d '()')
  for d in $DECLARED; do case "$slug" in "$d"|"${d}s") return 0 ;; esac; done
  case "$slug" in
    w|width|f|i|s|overflow|overflow_behaviour|range_policy|operations|fold_length|\
container_width|debug-assertions|opt_level|edition|crate_type) return 0 ;;
  esac
  return 1
}

awk '{n=$1;$1="";sub(/^ /,"");print $0}' "$out/narrow.txt" | sort > "$out/narrow_keys"
awk '{n=$1;$1="";sub(/^ /,"");print $0}' "$out/wide.txt"   | sort > "$out/wide_keys"
comm -13 "$out/narrow_keys" "$out/wide_keys" > "$out/only_wide"

echo "### keys the wide sweep finds that the twelve-file sweep does not,"
echo "### undeclared ones only, by occurrence in the wide sweep"
: > "$out/new_undeclared"
while IFS= read -r k; do
  is_declared "$k" && continue
  n=$(awk -v k="$k" '{c=$1;$1="";sub(/^ /,"");if($0==k)print c}' "$out/wide.txt")
  printf '%6s  %s\n' "${n:-0}" "$k" >> "$out/new_undeclared"
done < "$out/only_wide"
sort -rn "$out/new_undeclared" | head -60
echo "  ..."
echo "  total new undeclared keys: $(grep -c . "$out/new_undeclared")"

echo
echo "### W1, the narrow arm must reproduce 183_probes/keys.txt exactly"
if diff -q "$out/narrow.txt" ../183_probes/keys.txt > /dev/null; then
  echo "  PASS, byte identical"
else
  echo "  FAIL, this extractor is not 183's:"; diff "$out/narrow.txt" ../183_probes/keys.txt | head -10
fi
echo "### W2, the wide arm must sweep more files and find at least one new key"
if [ "$wf" -gt "$nf" ] && [ -s "$out/only_wide" ]; then
  echo "  PASS, $wf > $nf files and $(grep -c . "$out/only_wide") keys the narrow arm never saw"
else
  echo "  FAIL, the narrow sweep was complete"
fi
echo "### W3, a key nobody wrote appears in neither"
if grep -q phase_of_the_moon "$out/narrow.txt" "$out/wide.txt"; then echo "  FAIL"; else echo "  PASS"; fi
echo "### W4, no key present in the narrow arm is lost in the wide one"
lost=$(comm -23 "$out/narrow_keys" "$out/wide_keys" | grep -c . || true)
if [ "$lost" = "0" ]; then echo "  PASS, 0 lost"; else
  echo "  FAIL, $lost keys lost:"; comm -23 "$out/narrow_keys" "$out/wide_keys" | sed 's/^/    /' | head
fi
