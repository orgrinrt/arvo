#!/usr/bin/env bash
# How much of the canon is settled, by topic, and by whom.
#
# Op's bar is that a full design and then a full implementation can be done
# from the canon. A designer working from a row needs to know how hard to hold
# it, and the registry says: a `ruling` is op with a human in the loop, a
# `proposal` is the panel and op has not seen it. `mockspace.toml` on the
# proposal namespace: "It is canon only once a ruling stamps it."
#
# So the question this answers is not how many rows there are. It is how many
# of them anybody has blessed, and which subjects have nothing from op at all.
#
# Process rulings are excluded from the per-topic count on purpose. They govern
# how the panel is run rather than what arvo is, and counting them as coverage
# of a subject would be the same collapse as counting a proposal as met.
#
# Control: `--control` plants one ratified ruling carrying a `ratifies` edge and
# one non-process ruling on a topic that has none, and both must move.
set -euo pipefail
root="$(cd "$(dirname "$0")" && pwd)"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
R="$root/mock/registry"

extra_r=""; extra_p=""
if [ "${1:-}" = "--control" ]; then
  extra_r=$'\n[[ruling]]\nid = "PLANTED_RATIFIER"\nkind = "ruling"\nrung = "ratified"\ntopic = "the_format"\nsays = "x"\nratifies = ["PLANTED_CLAIM"]\nprovenance = ["panel::x"]\nkeywords = ["x"]\n'
  echo "  [control] planted one ratified ruling on the_format carrying a ratifies edge"
fi

rulings=$(mktemp); trap 'rm -f "$rulings"' EXIT
{ cat "$R/ruling.toml"; printf '%s' "$extra_r"; } > "$rulings"

echo
echo "######## the whole canon, by what has a human in the loop"
printf '  rulings          %s\n' "$(grep -c '^\[\[ruling\]\]' "$rulings")"
printf '    of those, process (how the panel runs, not what arvo is)  %s\n' "$(awk '/^kind = "process"/{n++} END{print n+0}' "$rulings")"
printf '    at rung ratified %s   in_force %s   stated %s   open %s\n' \
  "$(awk '/^rung = "ratified"/{n++} END{print n+0}' "$rulings")" \
  "$(awk '/^rung = "in_force"/{n++} END{print n+0}' "$rulings")" \
  "$(awk '/^rung = "stated"/{n++} END{print n+0}' "$rulings")" \
  "$(awk '/^rung = "open"/{n++} END{print n+0}' "$rulings")"
printf '  proposals        %s\n' "$(cat "$R"/proposal*.toml | grep -c '^\[\[proposal\]\]')"
printf '    ratified by a ruling (`ratifies` edges in the whole canon)  %s\n' "$(grep -c '^ratifies = ' "$rulings" || true)"
printf '    normative, i.e. a design decision rather than a measurement  %s\n' "$(cat "$R"/proposal*.toml | grep -c '^sentence_kind = "normative"')"
printf '    resting on one expert  %s\n' "$(cat "$R"/proposal*.toml | grep -c '^standing = "one_expert"')"

echo
echo "######## per topic: op rows that are about arvo, against proposals"
printf '  %-26s %-8s %s\n' TOPIC OP-ROWS PROPOSALS
awk '/^\[\[topic\]\]/{id=""} /^id = /{gsub(/^id = "|"$/,"");print}' "$R/topic.toml" | while read -r t; do
  o=$(awk -v t="$t" '/^\[\[ruling\]\]/{k="";tp=""} /^kind = /{gsub(/^kind = "|"$/,"");k=$0} /^topic = /{gsub(/^topic = "|"$/,"");tp=$0; if(k!="process" && tp==t) n++} END{print n+0}' "$rulings")
  p=$(cat "$R"/proposal*.toml | grep -c "^topic = \"$t\"" || true)
  flag=""; [ "$o" -eq 0 ] && [ "$p" -gt 0 ] && flag="   <- nothing from op, $p claims"
  printf '  %-26s %-8s %s%s\n' "$t" "$o" "$p" "$flag"
done
