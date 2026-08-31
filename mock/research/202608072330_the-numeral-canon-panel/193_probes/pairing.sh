#!/usr/bin/env bash
# Prints one mapped pairing with both sides open: what the claim says and what
# the instrument establishes.
#
# An `evidence` edge says this instrument produced this claim's number. That is
# checkable only by reading the two together, and the failure mode is topical
# adjacency: a probe about rounding and a claim about rounding are not thereby
# connected. So this prints the claim's `says` and `predicate` against the
# probe's `establishes` and `lives`, and nothing else, because those are the
# four fields the question turns on.
#
# usage: pairing.sh <proposal-slug> <probe-slug>
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
echo "================================================================ CLAIM"
"$here/row.sh" proposal "$1" | awk '/^(says|sentence_kind|standing|predicate) = /,/^[a-z_]+ = /' \
  | grep -vE '^(because|note|gap|law|instead|topic|kind|keywords|provenance|evidence|answers|obligation|supersedes|id) = ' | head -20
echo "---------------------------------------------------------------- INSTRUMENT"
"$here/row.sh" probe "$2" | grep -vE '^(keywords|note|reproduced|defect) = ' | head -24
