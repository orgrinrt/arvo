#!/usr/bin/env bash
# For every probe the coverage map names, its `standing` and the first sentence
# of its `control`, so the edges that will be refused are visible before any is
# written.
#
# The gate refuses a `measured` or `enumeration` row citing a probe at
# `defective` or `withdrawn`, or one whose `control` reads as an admission that
# none was run. An edge can be right about which instrument produced a number
# and still be refused, and that refusal is a statement about the claim rather
# than about the edge.
#
# The verdict column reimplements `names_no_control` from `shape.rs` rather
# than calling it, on purpose: two independent readings of the same field is
# the only way to find out whether the opening-word rule agrees with a person
# reading the sentence. Where the two disagree, that is the second read the
# rule's own doc comment says is owed.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
reg="$(cd "$here/../../../registry" && pwd)"

cut -f2 "$here/../192_probes/p4_coverage_map.tsv" | sort -u | while read -r p; do
  [ "$p" = "NONE" ] && continue
  row=$("$here/row.sh" probe "$p" 2>/dev/null) || { printf '%-62s MISSING\n' "$p"; continue; }
  st=$(printf '%s' "$row" | sed -n 's/^standing = "\(.*\)"$/\1/p')
  ct=$(printf '%s' "$row" | sed -n 's/^control = "\(.*\)"$/\1/p' | head -1)
  [ -n "$ct" ] || ct=$(printf '%s' "$row" | awk '/^control = (\047\047\047|""")/{f=1;next} f&&/^(\047\047\047|""")/{exit} f' | tr '\n' ' ')
  low=$(printf '%s' "$ct" | tr 'A-Z' 'a-z')
  verdict=ok
  case "$low" in
    *"no control"*|*"nothing was run"*|*"no case that had to fail"*) verdict=ADMITS_NONE ;;
    none[!a-z0-9]*|none)
      case "$low" in
        *fired*|*disagreed*|*refused*|*failed*|*reported*|*caught*|*flagged*) verdict=ok ;;
        *) verdict=ADMITS_NONE ;;
      esac ;;
  esac
  [ "$st" = defective ] || [ "$st" = withdrawn ] && verdict="STANDING_$st"
  printf '%-62s %-10s %-12s %s\n' "$p" "$st" "$verdict" "$(printf '%s' "$ct" | cut -c1-70)"
done
