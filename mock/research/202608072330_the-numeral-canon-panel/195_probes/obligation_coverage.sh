#!/usr/bin/env bash
# The eleven obligations, each met / proposed / nothing, counted apart.
#
# The split is the schema's and it is the line this whole report turns on.
# `mockspace.toml` on the field: "A proposal alone does not meet one: a proposal
# is proposed rather than met, and reporting it otherwise closes a gap op has
# never seen."
#
#   met       a ruling names it. Op said something that discharges it.
#   proposed  only a proposal names it, and nothing has stamped that proposal.
#   nothing   no row in any namespace carries an edge to it.
#
# Only `ruling` and `proposal` can carry the edge, which is a fact about the
# schema and worth stating, because a `retirement` reaching an obligation has
# nowhere to say so and this corpus has several that do.
#
# Control: `--control` plants one ruling edge and one proposal edge against two
# obligations that have neither. Both must move out of `nothing` and into the
# right column, or an all-nothing report is a fact about the parser.
set -euo pipefail
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"

edges=$(mktemp); trap 'rm -f "$edges"' EXIT
awk '
  /^\[\[/ { ns = substr($0, 3, length($0) - 4); next }
  /^id = / { gsub(/^id = "|"$/, ""); id = $0 }
  /^obligation = / {
    line = $0; gsub(/^obligation = \[|\]$/, "", line); gsub(/"/, "", line)
    n = split(line, t, ",")
    for (i = 1; i <= n; i++) { gsub(/^ +| +$/, "", t[i]); if (t[i] != "") print ns "\t" t[i] "\t" id }
  }
' "$reg/ruling.toml" "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml" > "$edges"

if [ "${1:-}" = "--control" ]; then
  printf 'ruling\ta_content_hash\tPLANTED_RULING\n'     >> "$edges"
  printf 'proposal\ta_cost_dynamic_program\tPLANTED_PROP\n' >> "$edges"
  echo "  [control] planted a ruling edge at a_content_hash and a proposal edge at a_cost_dynamic_program"
fi

met=0; prop=0; none=0
printf '  %-52s %-9s %s\n' OBLIGATION STATE 'REACHED BY'
awk '/^\[\[obligation\]\]/{next} /^id = /{gsub(/^id = "|"$/,"");print}' "$reg/obligation.toml" | while read -r ob; do
  r=$(awk -F'\t' -v o="$ob" '$1=="ruling"   && $2==o {print $3}' "$edges" | paste -sd, -)
  p=$(awk -F'\t' -v o="$ob" '$1=="proposal" && $2==o {print $3}' "$edges" | paste -sd, -)
  if   [ -n "$r" ]; then state=met;      by="$r${p:+ (+$p proposed)}"
  elif [ -n "$p" ]; then state=proposed; by="$p"
  else                   state=NOTHING;  by="-"
  fi
  printf '  %-52s %-9s %s\n' "$ob" "$state" "$by"
done

echo
echo "  totals, counted apart because a proposal is proposed rather than met:"
awk -F'\t' 'NR==FNR{if($1=="ruling")R[$2]=1; if($1=="proposal")P[$2]=1; next}
  /^id = /{gsub(/^id = "|"$/,"");
    if(R[$0]) m++; else if(P[$0]) q++; else n++}
  END{printf "    met %d   proposed %d   nothing %d   of %d\n", m, q, n, m+q+n}
' "$edges" "$reg/obligation.toml"
