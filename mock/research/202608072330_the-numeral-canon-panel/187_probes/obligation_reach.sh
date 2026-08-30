#!/usr/bin/env bash
# For each obligation, prints every ruling and proposal whose `says` mentions
# any of that obligation's own keywords.
#
# This is a net, not a test. A hit means two rows share a word and nothing more,
# and every hit still has to be read on both sides before an edge is written.
# What it is actually for is the other direction: an obligation with **no** hit
# is one nothing in the corpus even talks about, and that is a measurement of
# how far the panel's 180 files reach toward the demand side.
#
# Keywords are the obligation's own `keywords` field rather than words chosen
# here, so the net is the row's own account of what it is about. Keys under
# three characters are dropped, since `DP` matches every word containing it.
#
# Run with `--control` to add a synthetic row carrying two keywords from the
# obligations nothing else reaches. It must appear under both. Without that, an
# all-empty report is indistinguishable from a grep that matches nothing ever.
# Transcript in `control_runs.txt`.
set -euo pipefail
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"

says=$(mktemp); trap 'rm -f "$says"' EXIT
awk '
  /^\[\[/    { ns = substr($0, 3, length($0) - 4); next }
  /^id = /   { gsub(/^id = "|"$/, ""); id = $0 }
  /^says = / { print ns "\t" id "\t" tolower($0) }
' "$reg/ruling.toml" "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml" > "$says"

if [ "${1:-}" = "--control" ]; then
  printf 'proposal\tPLANTED_CONTROL_ROW\tsays = "a fiedler partition of the dependency graph, and a stable content hash"\n' >> "$says"
fi

awk '
  /^\[\[obligation\]\]/ { id = "" }
  /^id = /       { gsub(/^id = "|"$/, ""); id = $0 }
  /^keywords = / { gsub(/^keywords = \[|\]$/, ""); gsub(/"/, ""); print id "\t" $0 }
' "$reg/obligation.toml" | while IFS=$'\t' read -r ob keys; do
  out=$(echo "$keys" | tr ',' '\n' | sed 's/^ *//; s/ *$//' | while read -r k; do
    [ ${#k} -ge 3 ] || continue
    # `|| true` is load-bearing. A keyword matching nothing exits 1, and under
    # `set -e` that killed the whole script: the first run printed nothing at
    # all, which reads exactly like a corpus that mentions none of this.
    { grep -i -- "$k" "$says" || true; } | awk -F'\t' -v k="$k" '{print $1 " :: " $2 "\t(" k ")"}'
  done | sort -u)
  n=$(printf '%s' "$out" | grep -c . || true)
  echo "################ $ob  --  $n rows mention any of its own keywords"
  [ "$n" -eq 0 ] || printf '%s\n' "$out" | sed 's/^/  /'
done
