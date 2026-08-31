#!/usr/bin/env nutshell
# The ratio, read from `standing` rather than from prose, and the gap between them.
#
# `185_probes/p8` counted admissions by looking at whether a `control` field
# opens with the word None. That was the only thing available then. It is now
# wrong, and by a measurable amount, which is the argument for the new value in
# one line: prose said seven, the data says nine.
#
# This does not rerun `185`'s instrument. Rerunning it overwrites its committed
# output, which is a hazard `192` recorded and then hit again; this reads the
# registry directly.
#
# Required outcomes, written before the run:
#
#   C1  the data count and the prose count must DIFFER, or the new value bought
#       nothing measurable and this file should say so.
#   C2  the four standing values must partition the rows exactly.
#   C3  the prose count must be a SUBSET of the data count, not merely a
#       different number. If prose flags a row the data does not, the triage
#       missed one and that is a defect rather than a demonstration.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../../.." && pwd)"
toml="$root/mock/registry/probe.toml"
out="$here/p4_the_ratio_from_data.out"

rows() {
  awk '
    function flush(){ if(id!=""){ printf "%s\t%s\t%s\n", id, st, ctl } id="";st="";ctl="" }
    /^\[\[probe\]\]/{ flush(); next }
    /^id = /{ id=$0; sub(/id = "/,"",id); sub(/"$/,"",id); next }
    /^standing = /{ st=$0; sub(/.*= "/,"",st); sub(/"$/,"",st); next }
    /^control = /{ ctl=$0; sub(/control = "/,"",ctl); sub(/"$/,"",ctl); next }
    END{ flush() }
  ' "$toml"
}

{
  printf '=== p4 the ratio from data, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  total=$(rows | wc -l | tr -d ' ')
  printf '## standing, read from the field\n'
  rows | cut -f2 | sort | uniq -c | sed 's/^/  /'
  printf '  total %s\n\n' "$total"

  data=$(rows | awk -F'\t' '$2=="uncontrolled"{print $1}' | sort)
  nd=$(printf '%s\n' "$data" | grep -c . || true)
  prose=$(rows | awk -F'\t' 'tolower($3) ~ /^none[^a-z0-9]/ {print $1}' | sort)
  np=$(printf '%s\n' "$prose" | grep -c . || true)

  printf '## C1: prose and data must differ\n'
  printf 'rows whose control OPENS with the word none:  %s\n' "$np"
  printf 'rows whose standing IS uncontrolled:          %s\n' "$nd"
  if [ "$np" -ne "$nd" ]; then printf 'C1 PASS: the value carries %s the prose cannot see\n' "$((nd - np))"
  else printf 'C1 FAIL: prose and data agree, so the value bought nothing measurable here\n'; fi
  printf '\n'
  printf 'the ones prose cannot see:\n'
  comm -23 <(printf '%s\n' "$data") <(printf '%s\n' "$prose") | sed 's/^/  /'
  printf '\n'

  printf '## C3: prose must be a subset of data\n'
  extra=$(comm -13 <(printf '%s\n' "$data") <(printf '%s\n' "$prose"))
  if [ -z "$extra" ]; then printf 'C3 PASS: every prose hit is also `uncontrolled`\n'
  else printf 'C3 FAIL, the triage missed these:\n'; printf '%s\n' "$extra" | sed 's/^/  /'; fi
  printf '\n'

  printf '## C2: the values partition\n'
  sum=$(rows | cut -f2 | grep -cE '^(sound|uncontrolled|defective|withdrawn)$' || true)
  if [ "$sum" -eq "$total" ]; then printf 'C2 PASS: %s of %s\n' "$sum" "$total"
  else printf 'C2 FAIL: %s of %s\n' "$sum" "$total"; fi
  printf '\n'

  printf '## the ratio, stated the way it should now be stated\n'
  awk -v n="$nd" -v t="$total" 'BEGIN{printf "instruments that ran with no case that had to fail: %d of %d (%.0f%%)\n", n, t, 100*n/t}'
  awk -v n="$nd" -v t="$total" 'BEGIN{printf "instruments carrying one:                           %d of %d (%.0f%%)\n", t-n-3, t, 100*(t-n-3)/t}'
  printf 'plus two `defective` and one `withdrawn`, which are neither.\n'
} > "$out" 2>&1
cat "$out"
