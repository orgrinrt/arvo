#!/usr/bin/env bash
# Groups every question, ruling and proposal by topic, so a sweep for edges runs
# over one subject at a time instead of over the cross product of 78 questions
# and 91 candidate answers.
#
# Topic is the right join because the schema says so: `proposal.topic` is "the
# same enumeration a ruling uses, because the subjects are the same whoever is
# speaking". It is a starting filter and never the test. Two rows sharing a
# topic are not thereby connected, which is the whole failure mode this pass is
# supposed to avoid.
#
# The control is in `control_runs.txt`: a topic naming no row prints nothing,
# and a row with no topic prints under `-` rather than vanishing.
set -euo pipefail
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"

emit() { # emit <namespace> <file...>
  local ns=$1; shift
  awk -v ns="$ns" -v hdr="[[$1x]]" '
    /^\[\[/ { if (id != "") print topic "\t" ns "\t" id; id=""; topic="-"; inrow = ($0 == "[[" ns "]]"); next }
    !inrow  { next }
    /^id = /    { gsub(/^id = "|"$/, ""); id = $0 }
    /^topic = / { gsub(/^topic = "|"$/, ""); topic = $0 }
    END { if (id != "") print topic "\t" ns "\t" id }
  ' "${@:2}"
}

{
  emit question x "$reg/question.toml"
  emit ruling   x "$reg/ruling.toml"
  emit proposal x "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml"
} | sort | awk -F'\t' '
  $1 != last { print ""; print "=== " $1 " ==="; last = $1 }
  { printf "  %-9s %s\n", $2, $3 }
'
