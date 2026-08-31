#!/usr/bin/env nutshell
# Which probe directories a live claim actually leans on, ranked.
#
# 135 directories and 2599 files is more than one dispatch reads. This decides
# what gets read, on the only criterion that is not taste: how many times the
# panel's own prose names the directory. A probe nobody cites establishes
# nothing anybody is currently standing on.
#
# Required outcomes, written before the run:
#
#   C1  a heavily cited directory must come back non-zero. `151_probes` is named
#       in 183 section 8 twice, so it must not be zero.
#   C2  at least one existing directory must come back ZERO. Without this arm a
#       counter that matches its own input returns a positive number for
#       everything and the ranking is an artifact of the matcher.
#   C3  self-citations from inside the probe tree are excluded, or a script that
#       names its own directory inflates itself.
#   C4  the matcher must not count a longer name as an occurrence of a shorter
#       one. `47_probes` is a substring of `147_probes`, and so is every
#       directory whose number is a suffix of another's. The arm: the naive
#       fixed-string count for `47_probes` must exceed the bounded count, and
#       the bounded count must exceed nothing it should not.
#
# C4 exists because run one did not have it and was wrong. C1, C2 and C3 all
# passed on a counter in which `47_probes` reported 94 mentions, every one of
# them a mention of `147_probes`, and in which `57_probes` topped the ranking at
# 205 while absorbing `157_probes`. A control establishes that an instrument
# measures what it points at. It cannot establish that it points at the right
# thing, and here the instrument pointed at a superstring of it.
# `p3_citedness_run1_substring_overcount.out` is that run, kept.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
panel="$(cd "$here/.." && pwd)"
repo="$(cd "$panel/../../.." && pwd)"
out="$here/p3_citedness.out"
tsv="$here/p3_citedness.tsv"

# A mention is the directory name not preceded by a word character. `147_probes`
# therefore does not count as `47_probes`, and `x47_probes` does not either.
bounded() { grep -oE "(^|[^0-9A-Za-z_])$1" 2>/dev/null || true; }

: > "$tsv"
for d in $(cd "$panel" && ls -d *_probes 2>/dev/null | sort); do
  # `|| true` on every one: a grep that matches nothing exits 1, and under
  # `set -e` that kills the loop on the first uncited directory, silently,
  # leaving no output file at all. Run one of this instrument did exactly that.
  # Only the panel's top-level .md files, which excludes the probe tree (C3).
  prose=$( { cd "$panel" && cat -- *.md 2>/dev/null; } | grep -cE "(^|[^0-9A-Za-z_])$d" || true)
  hits=$(  { cd "$panel" && cat -- *.md 2>/dev/null; } | bounded "$d" | wc -l | tr -d ' ')
  reg=$(   { cd "$repo/mock/registry" && cat -- *.toml 2>/dev/null; } | bounded "$d" | wc -l | tr -d ' ')
  printf '%s\t%s\t%s\t%s\n' "$d" "$hits" "$prose" "$reg" >> "$tsv"
done

{
  printf '=== p3 citedness, %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  printf 'columns: dir, bounded mentions in panel .md, lines mentioning it, mentions in registry toml\n'
  printf 'the probe tree itself is excluded from every count (C3)\n\n'

  printf '## C1: 151_probes must be non-zero\n'
  c1=$(awk -F'\t' '$1=="151_probes"{print $2}' "$tsv")
  if [ "${c1:-0}" -gt 0 ]; then printf 'C1 PASS: %s mentions\n' "$c1"
  else printf 'C1 FAIL: zero for a directory 183 section 8 names twice\n'; fi
  printf '\n'

  printf '## C2: at least one existing directory must come back with zero mentions\n'
  zeros=$(awk -F'\t' '$2==0{n++} END{print n+0}' "$tsv")
  if [ "$zeros" -gt 0 ]; then
    printf 'C2 PASS: %s directories are cited nowhere in the panel prose\n' "$zeros"
    awk -F'\t' '$2==0{printf "  %s\n", $1}' "$tsv"
  else
    printf 'C2 FAIL: every directory came back positive; the matcher matches too much\n'
  fi
  printf '\n'

  printf '## C4: the matcher must not read 147_probes as 47_probes\n'
  naive=$( { cd "$panel" && cat -- *.md 2>/dev/null; } | grep -o -F '47_probes' | wc -l | tr -d ' ')
  fixed=$(awk -F'\t' '$1=="47_probes"{print $2}' "$tsv")
  long=$(awk -F'\t' '$1=="147_probes"{print $2}' "$tsv")
  printf 'naive fixed-string 47_probes: %s\n' "$naive"
  printf 'bounded 47_probes:            %s\n' "${fixed:-0}"
  printf 'bounded 147_probes:           %s\n' "${long:-0}"
  if [ "$naive" -gt "${fixed:-0}" ]; then
    printf 'C4 PASS: the bound removed %s phantom mentions from one directory alone\n' \
      "$((naive - ${fixed:-0}))"
  else
    printf 'C4 FAIL: the bound changed nothing, so either it is not applied or the corpus has no such pair\n'
  fi
  printf '\n'

  printf '## ranked by mentions\n'
  sort -t"$(printf '\t')" -k2,2nr "$tsv" | awk -F'\t' '{printf "%6s  %4s lines  %3s reg  %s\n", $2, $3, $4, $1}'
  printf '\n'

  printf '## totals\n'
  awk -F'\t' '{m+=$2; r+=$4; if($2>0)c++} END{printf "directories: %d\ncited at least once: %d\nuncited: %d\ntotal mentions: %d\nmentions in registry toml: %d\n", NR, c, NR-c, m, r}' "$tsv"
} > "$out" 2>&1
cat "$out"
