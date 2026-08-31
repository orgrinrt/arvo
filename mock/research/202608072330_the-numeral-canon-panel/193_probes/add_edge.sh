#!/usr/bin/env bash
# Adds one cross-namespace edge to one registry row, in place.
#
# usage: add_edge.sh <file> <namespace> <slug> <field> <target> [<target> ...]
#
# Why a tool rather than an editor. These rows sit in files of twelve hundred
# to fourteen hundred lines, and the alternative on offer was rewriting each
# file whole. A whole-file rewrite of a canon file cannot be checked: a dropped
# row or a mangled multi-line string looks the same as a clean write. A field
# insertion can be checked, because `git diff` on it is three lines and the
# lint pass afterwards resolves every slug it names.
#
# Placement. The edge goes immediately before the first tail field the row
# carries (`key`, `note`, `gap`, `provenance`, `keywords`, `supersedes`,
# `corrects`), which puts it where the schema declares it and keeps the reading
# order the other rows already have. Calling twice on one row appends in call
# order, so `answers` then `obligation` comes out in that order. A row carrying
# no tail field at all gets the edge at its end.
#
# Multi-line strings. `'''` and `"""` blocks are tracked, because a `note` in
# this corpus runs to a paragraph and a line inside one can begin with
# something that looks like a field. Without the tracking the insertion lands
# inside a quotation. The control run plants exactly that case.
#
# Refuses rather than guesses: an unknown row, or a row that already carries
# the field, exits non-zero and changes nothing.
#
# Controls in `control_runs.txt`, and they found two defects rather than none.
# A row with no tail field was printed back unchanged and reported as not
# found, which is the wrong diagnosis of a row it had. And a namespace matching
# no row at all **corrupted the file**, shifting every table header down by one
# row, while exiting 0 and printing success. Both are why only the matching
# namespace is buffered now: nothing outside it is held, so nothing outside it
# can be reordered.
set -euo pipefail

file=$1; ns=$2; slug=$3; field=$4; shift 4
list=$(printf '"%s", ' "$@"); list="[${list%, }]"

tmp=$(mktemp); trap 'rm -f "$tmp"' EXIT
awk -v ns="[[$ns]]" -v want="id = \"$slug\"" -v field="$field" -v value="$list" '
  function flush(  i) {
    if (at == 0) at = n + 1
    for (i = 1; i <= n; i++) {
      if (i == at) print field " = " value
      print buf[i]
    }
    if (at == n + 1) print field " = " value
    n = 0; at = 0
  }
  function drain(  i) { for (i = 1; i <= n; i++) print buf[i]; n = 0; at = 0 }

  /^\[\[/ && !inblock {
    if (inrow) { if (hit) flush(); else drain() }
    hit = 0
    if ($0 == ns) { inrow = 1; buf[++n] = $0 } else { inrow = 0; print }
    next
  }
  !inrow { print; next }
  {
    if (!inblock && $0 == want) hit = 1
    if (!inblock && hit && $0 ~ /^[a-z_]+ = /) {
      split($0, f, " ")
      if (f[1] == field) { print "DUPLICATE" > "/dev/stderr"; exit 3 }
      if (at == 0 && (f[1] == "key" || f[1] == "note" || f[1] == "gap" || f[1] == "provenance" || f[1] == "keywords" || f[1] == "supersedes" || f[1] == "corrects")) at = n + 1
    }
    buf[++n] = $0
    # a line opening or closing a multi-line string flips the block state
    if ($0 ~ /^[a-z_]+ = (\047\047\047|""")[ \t]*$/) inblock = 1
    else if (inblock && $0 ~ /^(\047\047\047|""")[ \t]*$/) inblock = 0
  }
  END { if (inrow) { if (hit) flush(); else drain() } }
' "$file" > "$tmp"

if ! grep -q . "$tmp"; then echo "refused: empty output for $ns::$slug" >&2; exit 1; fi
if diff -q "$file" "$tmp" >/dev/null; then
  echo "refused: no such row $ns::$slug in $file" >&2; exit 2
fi
cp "$tmp" "$file"
echo "added $field to $ns::$slug -> $list"
