#!/usr/bin/env bash
# For each obligation, whose words its `provenance` actually resolves to.
#
# The namespace exists because "a check that walks the canon can only report
# that the canon agrees with itself, so the enumeration has to come from
# somewhere the canon does not reach". An obligation whose only citation is a
# panel file has not come from outside the canon. It has come from an agent's
# summary of outside, and a summary is where all three known wording errors in
# these rows were introduced.
#
# Three buckets:
#   consumer  cites a consumer repository
#   op        cites INTENTS or an op file, which is inside the repo and is
#             correct, because op is not outside the canon and never was
#   summary   cites a panel file that is neither
#
# THE DEFECT THIS SCRIPT SHIPPED WITH, kept because it is the point. The first
# version classified on any substring, so `hilavitkutin` matched inside the
# ANCHOR TEXT `#what-hilavitkutin-asks-for` on a citation whose target is
# `184`. It reported 8 of 13 as consumer-cited. A consumer's name inside an
# anchor is the summary naming who it read, which is the opposite of citing
# them, and the wrong answer was the flattering one. Classification is on the
# citation root now, and there is one rule rather than two.
set -euo pipefail
root="$(cd "$(dirname "$0")" && pwd)"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done

rows=$(awk '
  /^\[\[obligation\]\]/ { id=""; prov=""; next }
  /^id = /         { gsub(/^id = "|"$/, ""); id = $0 }
  /^provenance = / { prov = $0 }
  /^keywords = /   { if (id != "") { print id "\t" prov; id="" } }
' "$root/mock/registry/obligation.toml" | while IFS=$'\t' read -r id prov; do
  case "$prov" in
    *'"consumer::'*|*'"hilavitkutin::'*|*'"vehje::'*|*'"kolli::'*|*'"tarina::'*) b=consumer ;;
    *'INTENTS::'*|*_op_*)                                                       b=op ;;
    *'panel::'*)                                                                b=summary ;;
    *)                                                                          b=none ;;
  esac
  printf '%-68s %s\n' "$id" "$b"
done)

printf '%s\n' "$rows"
echo
echo "totals, tallied from the rows above rather than by a second rule:"
printf '%s\n' "$rows" | awk '{c[$NF]++} END {
  printf "  consumer %d   op %d   summary %d   none %d\n",
    c["consumer"], c["op"], c["summary"], c["none"] }'
