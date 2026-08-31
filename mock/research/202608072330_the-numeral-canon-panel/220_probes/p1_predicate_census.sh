#!/usr/bin/env bash
# p1: what shapes the predicate corpus actually writes, per axis.
#
# Reads every entry of the three predicate-bearing fields (`proposal.predicate`,
# `law.holds`, `law.fails`) out of the committed registry and classifies the
# span each one gives its axis. The point is to find out which spans the two
# markers would have to sit beside, and whether a leading marker token can be
# read without touching the span grammars.
#
# Run from the repo root. Writes to stdout; the committed output beside this
# file is what it printed.

set -uo pipefail
REG="mock/registry"
[ -d "$REG" ] || { echo "run me from the repo root; no $REG here" >&2; exit 2; }

# Every predicate entry, as `slug<TAB>span`. An entry is a quoted string on its
# own line inside one of the three arrays, or inline in a single-line array.
entries() {
  awk '
    /^(predicate|holds|fails) = \[/ { inarr=1 }
    inarr {
      n = split($0, parts, "\"")
      for (i = 2; i <= n; i += 2) {
        s = parts[i]
        if (index(s, ":") == 0) continue
        slug = substr(s, 1, index(s, ":") - 1)
        span = substr(s, index(s, ":") + 1)
        gsub(/^[ \t]+|[ \t]+$/, "", slug)
        gsub(/^[ \t]+|[ \t]+$/, "", span)
        print slug "\t" span
      }
    }
    inarr && /\]/ { inarr=0 }
  ' "$REG"/*.toml
}

# The span shape classes, in the order a reader would try them.
classify() {
  awk -F'\t' '{
    span = $2
    if (span ~ /(^| )any([ ,]|$)/)                       k = "any"
    else if (span ~ /\.\.=/)                             k = "range"
    else if (span ~ /\{/)                                k = "set"
    else if (span ~ /(=|<|>)/)                           k = "fixed"
    else                                                 k = "prose"
    print $1 "\t" k
  }'
}

echo "### 1. total predicate entries"
entries | wc -l | tr -d " "

echo
echo "### 2. entries per axis"
entries | cut -f1 | sort | uniq -c | sort -rn

echo
echo "### 3. span shape per axis (axis, shape, count)"
entries | classify | sort | uniq -c | sort -rn

echo
echo "### 4. every entry whose span says \`any\`"
entries | awk -F'\t' '$2 ~ /(^| )any([ ,]|$)/ {print $1 "\t" $2}' | sort | uniq -c | sort -rn

echo
echo "### 5. every entry whose span is a range, with its bounds"
entries | awk -F'\t' '$2 ~ /\.\.=/ {print $1 "\t" $2}' | sort | uniq -c | sort -rn

echo
echo "### 6. entries carrying a second colon (would a \`::\` or \`:\` marker delimiter collide?)"
entries | awk -F'\t' '$2 ~ /:/ {print $1 "\t" $2}'
echo "count:"
entries | awk -F'\t' '$2 ~ /:/' | wc -l | tr -d " "

echo
echo "### 7. CONTROL. A planted entry of each shape must classify as that shape."
printf 'total_width\tW in 1..=64\nthreads\tthreads any\nsignedness\tsignedness = signed\noperation\toperations {add, mul}\ncontainer\tinterval numerals containing zero\n' \
  | classify
echo "expected: range, any, fixed, set, prose"

echo
echo "### 8. CONTROL. A pattern known absent returns zero, one known present does not."
absent=$(entries | grep -c "phase_of_the_moon" || true)
present=$(entries | grep -c "fraction_width" || true)
echo "absent(phase_of_the_moon)=$absent  present(fraction_width)=$present"
[ "$absent" = "0" ] || echo "CONTROL FAILED: the absent pattern matched"
[ "$present" != "0" ] || echo "CONTROL FAILED: the present pattern did not match; the reader is broken"
