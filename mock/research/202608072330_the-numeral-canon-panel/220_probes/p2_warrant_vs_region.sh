#!/usr/bin/env bash
# p2: does the row-level `sentence_kind` warrant agree with the region its
# predicate writes?
#
# The ruling names one failure to guard: a sweep relabelled a proof. The schema
# already carries a row-level warrant word (`sentence_kind`, with `theorem` and
# `measured` among its values), so the question this answers is whether that
# word is already being written over sweep-shaped regions, and how often.
#
# Row-aware: walks each registry file tracking the open `[[namespace]]`, its
# `id` and its `sentence_kind`, and attributes every predicate entry to it.
#
# Run from the repo root.

set -uo pipefail
REG="mock/registry"
[ -d "$REG" ] || { echo "run me from the repo root; no $REG here" >&2; exit 2; }

rows() {
  awk '
    function flush() {
      if (id != "") print ns "\t" id "\t" (sk == "" ? "-" : sk) "\t" (spans == "" ? "-" : spans)
      id = ""; sk = ""; spans = ""; inarr = 0
    }
    /^\[\[[a-z_]+\]\]/ { flush(); ns = $0; gsub(/[\[\]]/, "", ns); next }
    /^id = / { id = $0; sub(/^id = "/, "", id); sub(/"$/, "", id); next }
    /^sentence_kind = / { sk = $0; sub(/^sentence_kind = "/, "", sk); sub(/"$/, "", sk); next }
    /^(predicate|holds|fails) = \[/ { inarr = 1 }
    inarr {
      n = split($0, parts, "\"")
      for (i = 2; i <= n; i += 2) {
        s = parts[i]
        if (index(s, ":") == 0) continue
        spans = spans (spans == "" ? "" : " | ") s
      }
      if ($0 ~ /\]/) inarr = 0
    }
    END { flush() }
  ' "$1"
}

allrows() { for f in "$REG"/*.toml; do rows "$f"; done; }

# The width span a row writes, or `ABSENT`.
widthof() {
  awk -F'\t' '{
    span = "ABSENT"
    n = split($4, e, / \| /)
    for (i = 1; i <= n; i++) if (e[i] ~ /^ *total_width:/) { span = e[i]; sub(/^ *total_width: */, "", span) }
    print $1 "\t" $2 "\t" $3 "\t" span
  }'
}

shapeof() {
  awk -F'\t' '{
    s = $4
    if (s == "ABSENT")                       k = "ABSENT"
    else if (s ~ /(^| )any([ ,]|$)/)         k = "any"
    else if (s ~ /\.\.=/)                    k = "range"
    else if (s ~ /\{/)                       k = "set"
    else if (s ~ /(=|<|>)/)                  k = "fixed"
    else                                     k = "prose"
    print $3 "\t" k "\t" $2
  }'
}

echo "### 1. rows carrying a sentence_kind, by kind"
allrows | awk -F'\t' '$3 != "-" {print $3}' | sort | uniq -c | sort -rn

echo
echo "### 2. sentence_kind against the shape of the width span it writes"
allrows | widthof | shapeof | awk -F'\t' '$1 != "-" {print $1 "\t" $2}' | sort | uniq -c | sort -rn

echo
echo "### 3. every row whose sentence_kind is \`theorem\`, with its width span"
allrows | widthof | awk -F'\t' '$3 == "theorem" {printf "  %-72s %s\n", $2, $4}'

echo
echo "### 4. every row whose sentence_kind is \`argument\`, with its width span"
allrows | widthof | awk -F'\t' '$3 == "argument" {printf "  %-72s %s\n", $2, $4}'

echo
echo "### 5. rows with a predicate and no width axis at all, by sentence_kind"
allrows | widthof | awk -F'\t' '$4 == "ABSENT" && $3 != "-" && $3 != "normative" && $3 != "definition" {print $3}' | sort | uniq -c | sort -rn
echo "  (total such rows:)"
allrows | widthof | awk -F'\t' '$4 == "ABSENT" && $3 != "-" && $3 != "normative" && $3 != "definition"' | wc -l | tr -d " "

echo
echo "### 6. CONTROL. The reader must find a row it is known to contain, and not one it does not."
allrows | awk -F'\t' '$2 == "no_multiplicative_structure_survives_a_nonzero_fraction_width" {print "found: " $1 " / " $3}'
n=$(allrows | awk -F'\t' '$2 == "a_row_that_does_not_exist"' | wc -l | tr -d " ")
echo "rows named a_row_that_does_not_exist: $n"
[ "$n" = "0" ] || echo "CONTROL FAILED"

echo
echo "### 7. CONTROL. Planted: a theorem row whose width span is a three-width sweep must be"
echo "    classified theorem/range, and one writing \`any\` must be classified theorem/any."
printf 'proposal\tplanted_sweep\ttheorem\ttotal_width: W in 3..=7\nproposal\tplanted_free\ttheorem\ttotal_width: W any\n' \
  | shapeof
echo "expected: theorem range, theorem any"
