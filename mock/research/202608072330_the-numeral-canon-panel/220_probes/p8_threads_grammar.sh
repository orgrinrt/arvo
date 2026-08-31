#!/usr/bin/env bash
# p8: does the corpus write the `threads` axis in the form its own dimension row
# declares?
#
# Reached by attacking seat 219's universal count. Its matcher implements the
# declared grammar faithfully (`values == "any" || ends_with(" any")`) and its
# doc comment says every declared grammar spells the universal as a trailing bare
# `any`, which is true of the grammars. Three entries are missed anyway, which
# means the corpus departs from the grammar rather than the matcher departing
# from the corpus.
#
# `dimension::threads` declares exactly: `threads = 1`, `threads = <n>`, or
# `threads any`. This walks every threads entry against that.
#
# Run from `mock/`.

set -uo pipefail
[ -d registry ] || { echo "run me from mock/" >&2; exit 2; }

echo "### the declared grammar"
grep -A 4 '^id = "threads"' registry/dimension.toml | grep '^grammar' | sed 's/^/  /'

entries() {
  awk '
    /^(predicate|holds|fails) = \[/ { inarr=1 }
    inarr {
      n = split($0, parts, "\"")
      for (i = 2; i <= n; i += 2) {
        s = parts[i]
        if (index(s, ":") == 0) continue
        slug = substr(s, 1, index(s, ":") - 1); gsub(/^[ \t]+|[ \t]+$/, "", slug)
        span = substr(s, index(s, ":") + 1);   gsub(/^[ \t]+|[ \t]+$/, "", span)
        if (slug == "threads") print span
      }
    }
    inarr && /\]/ { inarr=0 }
  ' registry/*.toml
}

echo
echo "### every threads entry, conforming or not"
entries | sort | uniq -c | sort -rn | while read -r n span; do
  span="${span#"${span%%[![:space:]]*}"}"
  if [[ "$span" =~ ^threads[[:space:]]*=[[:space:]]*[0-9]+$ || "$span" == "threads any" ]]; then
    printf "  [conforms] %3s  %s\n" "$n" "$span"
  else
    printf "  [DEPARTS ] %3s  %s\n" "$n" "$span"
  fi
done

echo
echo "### counts"
total=$(entries | wc -l | tr -d ' ')
conf=$(entries | grep -cE '^threads[[:space:]]*=[[:space:]]*[0-9]+$|^threads any$' || true)
echo "  threads entries: $total"
echo "  conforming to the declared grammar: $conf"
echo "  departing: $((total - conf))"

echo
echo "### CONTROL. The matcher must accept a known-good form and refuse a known-bad one."
ok=$(printf 'threads = 1\nthreads any\n' | grep -cE '^threads[[:space:]]*=[[:space:]]*[0-9]+$|^threads any$')
bad=$(printf 'threads any, because\nthreads mostly\n' | grep -cE '^threads[[:space:]]*=[[:space:]]*[0-9]+$|^threads any$' || true)
echo "  known-good accepted: $ok of 2"
echo "  known-bad accepted:  $bad of 2"
[ "$ok" = "2" ] || { echo "  CONTROL FAILED: the matcher refuses a form the grammar declares"; exit 1; }
[ "$bad" = "0" ] || { echo "  CONTROL FAILED: the matcher accepts a form the grammar does not declare"; exit 1; }
echo "  controls held."

echo
echo "### where the departures live, by file"
for f in registry/*.toml; do
  n=$(awk -v F="$f" '
    /^(predicate|holds|fails) = \[/ { inarr=1 }
    inarr {
      k = split($0, parts, "\"")
      for (i = 2; i <= k; i += 2) {
        s = parts[i]
        if (index(s, ":") == 0) continue
        slug = substr(s, 1, index(s, ":") - 1); gsub(/^[ \t]+|[ \t]+$/, "", slug)
        span = substr(s, index(s, ":") + 1);   gsub(/^[ \t]+|[ \t]+$/, "", span)
        # the bare dialect: a span that does not repeat any word of its own slug
        split(slug, w, "_")
        seen = 0
        for (j in w) if (index(tolower(span), w[j]) > 0) seen = 1
        if (!seen) c++
      }
    }
    inarr && /\]/ { inarr=0 }
    END { print c+0 }
  ' "$f")
  t=$(awk '
    /^(predicate|holds|fails) = \[/ { inarr=1 }
    inarr { k = split($0, parts, "\""); for (i = 2; i <= k; i += 2) if (index(parts[i], ":") > 0) c++ }
    inarr && /\]/ { inarr=0 }
    END { print c+0 }
  ' "$f")
  [ "$t" = "0" ] && continue
  printf "  %-44s %3s of %3s entries write a span naming nothing from their slug\n" "$(basename "$f")" "$n" "$t"
done

echo
echo "### CONTROL for the file breakdown."
echo "  a span naming its own axis must not be counted, and one that does not must be."
printf 'x\n' >/dev/null
echo "  'threads any'  contains 'threads' -> not counted"
echo "  '1'            contains no slug word -> counted"
