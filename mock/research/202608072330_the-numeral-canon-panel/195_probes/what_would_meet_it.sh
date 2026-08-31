#!/usr/bin/env bash
# For one search term, every registry row in any namespace whose narrative text
# mentions it, with the namespace and the field named.
#
# Direction matters and this is the reason the script exists. Starting from
# rows and asking which obligation each is near produces topical adjacency,
# which is the failure `187` section 5 catalogued. So the terms handed to this
# come from the obligation's own `need` sentence, written down before the
# search runs, and every hit is then read on both sides.
#
# It searches all twelve registry files and every narrative field, because
# `187`'s instrument read `says` in three files and six namespaces went
# unsearched. `191` measured what that missed: `retirement.toml` alone carries
# nine of the hits for one obligation and four for another.
#
# usage: what_would_meet_it.sh <term> [<term> ...]
#
# Control: `--control` prepends a synthetic record carrying a phrase from the
# obligation nothing in the corpus reaches. It must surface, or an empty report
# is a fact about the grep.
set -euo pipefail
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"

flat=$(mktemp); trap 'rm -f "$flat"' EXIT
# One line per (namespace, row, field, text). Block strings are folded onto the
# field that opened them, so a paragraph note is searchable as one unit.
awk '
  function emit() { if (ns && id && f) print ns "\t" id "\t" f "\t" tolower(buf) ; f=""; buf="" }
  /^\[\[/ && !inblock { emit(); ns = substr($0, 3, length($0) - 4); id=""; next }
  /^id = / && !inblock { emit(); gsub(/^id = "|"$/, ""); id = $0; next }
  /^[a-z_]+ = (\047\047\047|""")[ \t]*$/ { emit(); split($0, a, " "); f = a[1]; inblock = 1; next }
  inblock && /^(\047\047\047|""")[ \t]*$/ { emit(); inblock = 0; next }
  inblock { buf = buf " " $0; next }
  /^(says|claim|why|asks|instead|replacement|note|need|gap|establishes|statement|control|because|what|intent|defect|options|witness|holds|fails) = / {
    emit(); split($0, a, " "); f = a[1]; buf = $0; next
  }
  /^  "/ { buf = buf " " $0; next }
  { emit() }
  END { emit() }
' "$reg"/*.toml > "$flat"

if [ "${1:-}" = "--control" ]; then
  shift
  printf 'PLANTED\tCONTROL_ROW\tsays\ta fiedler partition, a topological order, a compressed sparse adjacency and a stable content hash\n' >> "$flat"
fi

for t in "$@"; do
  echo "---- term: $t"
  { grep -i -- "$t" "$flat" || true; } | awk -F'\t' '{printf "  %-11s %-58s %s\n", $1, $2, $3}' | sort -u
done
