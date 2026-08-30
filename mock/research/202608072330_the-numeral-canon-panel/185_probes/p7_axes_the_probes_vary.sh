#!/usr/bin/env nutshell
# What the instruments actually varied, read off their outputs.
#
# A previous port censused the axes the panel's PROSE predicates over and found
# ten families no `dimension` row declares. This asks a different question with a
# different instrument: what did the probes themselves sweep? Probe outputs print
# one line per cell with the varied parameters as `key=value`, so the keys are
# the axes, stated by the code rather than by the author.
#
# The two questions can disagree, and where they do the probe is the better
# witness: prose says what somebody believed the region was, an output line says
# what the loop ran over.
#
# THIS IS NOT A PROPOSAL TO DECLARE ANYTHING. The axis set is known incomplete
# and extending it needs two independent readings; the dispatch forbids adding a
# row and is right to, because an axis declared today silently rewrites the
# negative space of every predicate already committed.
#
# Required outcomes, written before the run:
#
#   C1  `W` and `F` must appear, and high. They are integer width and fraction
#       width, both declared, both swept everywhere. Zero for either means the
#       extractor is not reading output lines at all.
#   C2  at least one frequent key must map to NO declared dimension. If every key
#       maps, there is no finding and the instrument should say so rather than
#       being tuned until one appears.
#   C3  a result column must not be mistaken for an axis. `differ=0` and
#       `ops=192` are outputs, not parameters. They cannot be separated
#       mechanically from a key alone, so the report carries the distinct-value
#       cardinality per key and the classification below is stated as a reading.
#   C4  the mapping to declared slugs is read from `dimension.toml` rather than
#       typed, so a slug that does not exist cannot be claimed as a match.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
panel="$(cd "$here/.." && pwd)"
repo="$(cd "$panel/../../.." && pwd)"
out="$here/p7_axes_the_probes_vary.out"
tsv="$here/p7_axes_the_probes_vary.tsv"

# The declared axes, read rather than typed (C4).
declared=$(grep '^id = ' "$repo/mock/registry/dimension.toml" | sed 's/id = "//; s/"//' | sort)

# Every `key=value` in every committed probe output, with the directory it came
# from. Keys are lowercased so `W` and `w` are one axis.
# Extraction in awk rather than a shell loop per line. The shell form was
# written first and did not finish: it spawns two processes per `key=value` over
# roughly a thousand output files. `timeout` is not installed on this host, which
# is one of the six meaningless greens `107` recorded, so there was nothing to
# cap it with either.
: > "$tsv"
( cd "$panel" && find . -path './*_probes/*' -type f \
    \( -name '*.out' -o -name '*out*.txt' -o -name '*output*' \) \
    -not -path './185_probes/*' | sed 's|^\./||' | sort ) \
| ( cd "$panel" && xargs awk '
    FNR==1 { d = FILENAME; sub(/\/.*/, "", d) }
    {
      line = $0
      while (match(line, /[A-Za-z_][A-Za-z_0-9]{0,24} ?= ?[^ ,;)]+/)) {
        kv = substr(line, RSTART, RLENGTH)
        line = substr(line, RSTART + RLENGTH)
        eq = index(kv, "=")
        k = substr(kv, 1, eq - 1); v = substr(kv, eq + 1)
        gsub(/ /, "", k); gsub(/ /, "", v)
        print d "\t" tolower(k) "\t" v
      }
    }' ) >> "$tsv"

{
  printf '=== p7 axes the probes actually varied, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  printf '## the declared axis set, read from dimension.toml\n'
  printf '%s\n' "$declared" | tr '\n' ' '; printf '\n'
  printf 'declared count: %s\n\n' "$(printf '%s\n' "$declared" | grep -c . || true)"

  printf '## C1: w and f must appear\n'
  for k in w f; do
    n=$(awk -F'\t' -v k="$k" '$2==k{print $1}' "$tsv" | sort -u | wc -l | tr -d ' ')
    if [ "$n" -gt 0 ]; then printf 'C1 pass: %s appears in %s probe directories\n' "$k" "$n"
    else printf 'C1 FAIL: %s appears nowhere, so the extractor is not reading output lines\n' "$k"; fi
  done
  printf '\n'

  printf '## keys ranked by how many probe directories they appear in\n'
  printf '%-24s %5s %8s %s\n' key dirs distinct declared
  awk -F'\t' '{dirs[$2"\t"$1]=1; vals[$2"\t"$3]=1}
    END{for (x in dirs){split(x,a,"\t"); nd[a[1]]++}
        for (x in vals){split(x,a,"\t"); nv[a[1]]++}
        for (k in nd) printf "%s\t%d\t%d\n", k, nd[k], nv[k]}' "$tsv" \
  | sort -t"$(printf '\t')" -k2,2nr | head -60 \
  | while IFS=$(printf '\t') read -r k nd nv; do
      if printf '%s\n' "$declared" | grep -qx "$k"; then m=declared; else m='-'; fi
      printf '%-24s %5s %8s %s\n' "$k" "$nd" "$nv" "$m"
    done
  printf '\n'

  printf '## the synonym map, and why the column above is not the finding\n'
  printf 'The probes name their axes with the short symbols the corpus argues in.\n'
  printf 'dimension.toml names them with long descriptive slugs. So `w` reads as\n'
  printf 'undeclared against `integer_width` and `f` against `fraction_width`, and\n'
  printf 'the C2 arm below would have passed on that alone, which is the arm\n'
  printf 'cheating rather than finding anything. Nothing mechanical bridges the two\n'
  printf 'spellings, which is itself worth stating: no check can confirm that a\n'
  printf 'predicate naming `integer_width` corresponds to a probe that swept `W`.\n'
  printf 'The map below is hand-written and is a reading, not a measurement.\n\n'
  SYN='w=integer_width f=fraction_width width=total_width policy=overflow_policy
overflow=overflow_policy op=operation mode=rounding signed=signedness
signedness=signedness rounding=rounding arity=arity operation=operation
threads=threads container=container strategy=strategy alignment=alignment
profile=build_profile optlevel=build_profile opt=build_profile lto=build_profile
assertions=build_profile build=build_profile depth=chain_length steps=chain_length
chain=chain_length'
  printf '%s\n' "$SYN" | tr ' ' '\n' | grep . | sed 's/^/  /'
  printf '\n'

  printf '## C2: after the map, at least one frequent key must still map to nothing\n'
  unmapped=$(awk -F'\t' '{dirs[$2"\t"$1]=1} END{for (x in dirs){split(x,a,"\t"); nd[a[1]]++}
    for (k in nd) if (nd[k] >= 5) print k}' "$tsv" | sort \
    | while IFS= read -r k; do
        printf '%s\n' "$declared" | grep -qx "$k" && continue
        printf '%s\n' "$SYN" | tr ' ' '\n' | grep -q "^$k=" && continue
        printf '%s\n' "$k"
      done)
  nu=$(printf '%s\n' "$unmapped" | grep -c . || true)
  if [ "$nu" -gt 0 ]; then
    printf 'C2 pass: %s keys appear in five or more directories, match no declared slug,\n' "$nu"
    printf '        and are not a spelling of one under the map above\n'
  else
    printf 'C2 FAIL: every frequent key maps, so this instrument found nothing and says so\n'
  fi
  printf '\n'
  printf 'those keys, with the directory count, unclassified:\n'
  printf '%s\n' "$unmapped" | while IFS= read -r k; do
    [ -z "$k" ] && continue
    n=$(awk -F'\t' -v k="$k" '$2==k{print $1}' "$tsv" | sort -u | wc -l | tr -d ' ')
    printf '  %-24s %s\n' "$k" "$n"
  done
  printf '\n'
  printf '## the toolchain, which is the residue arm worth a number\n'
  printf 'The pinned nightly is a condition on every compiled result in this\n'
  printf 'corpus and is not a declared axis. `build_profile` is the optimisation\n'
  printf 'settings; the compiler version is a different thing, and a result at one\n'
  printf 'nightly is not established at another.\n'
  pop=$( cd "$panel" && ls -d *_probes 2>/dev/null | wc -l | tr -d ' ')
  naming=$( { cd "$panel" && grep -rl 'nightly-2026-05-28' . 2>/dev/null || true; } \
    | grep '_probes' | sed 's|^\./||; s|/.*||' | sort -u | grep -c . || true)
  printf 'probe directories:                        %s\n' "$pop"
  printf 'naming the pinned nightly anywhere:       %s\n' "$naming"
  if [ "$naming" -le "$pop" ]; then
    printf 'C5 PASS: the count does not exceed its own population\n'
  else
    printf 'C5 FAIL: %s of %s is impossible, so the path split is wrong\n' "$naming" "$pop"
  fi
  printf 'C5 exists because a first hand-run of this count returned 202 of 135.\n'
  printf 'It split the path on the wrong field and counted directory names that\n'
  printf 'were not probe directories. A count exceeding its own denominator is the\n'
  printf 'cheapest control available and it is available on every count.\n\n'

  printf 'C3: a key here is not necessarily an axis. `differ` and `ops` are result\n'
  printf 'columns and appear in this list; nothing mechanical separates a parameter\n'
  printf 'from an output, so the distinct-value count above is the only signal and\n'
  printf 'the classification in the findings file is a reading rather than a count.\n'
} > "$out" 2>&1
cat "$out"
