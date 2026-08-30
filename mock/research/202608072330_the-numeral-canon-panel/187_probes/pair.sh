#!/usr/bin/env bash
# Prints the questions of one topic with their options, then the rulings and
# proposals of the same topic with their claims. Both sides of every candidate
# edge, on one screen, which is the only way to test an edge rather than guess
# at it from a slug.
#
# usage: pair.sh <topic>
#
# Deliberately drops `note`, `provenance` and `keywords`: those are what make a
# row findable and are not what an edge is a claim about. Where a note turns out
# to matter, `row.sh` prints the row whole.
set -euo pipefail
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"
topic=$1

show() { # show <namespace> <fields-regex> <file...>
  local ns=$1 fields=$2; shift 2
  awk -v ns="$ns" -v want="topic = \"$topic\"" -v fields="$fields" '
    /^\[\[/ { if (hit) printf "%s", buf; inrow = ($0 == "[[" ns "]]"); buf=""; hit=0; keep=0; next }
    !inrow  { next }
    /^id = /    { gsub(/^id = "|"$/, ""); buf = "\n--- " ns " :: " $0 "\n"; keep=0 }
    $0 == want { hit = 1 }
    $0 ~ fields { keep = 1; buf = buf $0 "\n"; next }
    /^[a-z_]+ = / { keep = 0 }
    keep { buf = buf $0 "\n" }
    END { if (hit) printf "%s", buf }
  ' "$@"
}

echo "################ QUESTIONS in $topic"
show question '^(asks|options|decider) = ' "$reg/question.toml"
echo
echo "################ RULINGS in $topic"
show ruling '^(kind|rung|says|instead) = ' "$reg/ruling.toml"
echo
echo "################ PROPOSALS in $topic"
show proposal '^(kind|says|instead) = ' "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml"
