#!/usr/bin/env nutshell
# `184` says it read hilavitkutin at `mock/DESIGN.md.tmpl` and
# `mock/crates/*/DESIGN.md.tmpl`, and vehje and kolli at `mock/DESIGN.md.tmpl`
# only. So for two of the three consumers the per-crate documents were never
# opened. What is in them?
#
# Per consumer: how many crates, how many arvo-naming lines at the top level,
# how many in the per-crate design documents, and then every one of those lines
# printed, so a reader sees the demands rather than taking a count on trust.
#
# NUTSHELL HAZARD, and version one died on it. An unmatched glob is an error
# here rather than a literal, so `"$d"/mock/crates/*/` under `set -e` kills the
# script at the first consumer with no crates directory. That consumer is
# `kolli`, the third in the list, so version one printed two of five and
# exited 1, and a truncated report reads exactly like a complete one if nobody
# checks the exit status. Transcript:
# `the_demand_side_below_the_top_level_v1_truncated.out`. Every glob is now
# guarded by its directory existing, and the run prints a terminator line so a
# truncation is visible in the output itself rather than only in `$?`.
#
# CONTROLS, two, in opposite directions.
#   `kolli` must report zero everywhere. `184` reports it at zero and this is
#     the one claim of `184`'s the instrument can confirm rather than extend.
#   `notko` is upstream of arvo, so a large count there would mean the matcher
#     is catching cross-references rather than dependencies.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
ws="$(dirname "$root")"
echo "### workspace: $ws"
echo

for repo in hilavitkutin vehje kolli tarina notko; do
  d="$ws/$repo"
  if [ ! -d "$d" ]; then
    echo "######## $repo  -- NOT CLONED. Contributes nothing, and that is a gap rather than a zero."
    echo
    continue
  fi
  topn=0
  [ -f "$d/mock/DESIGN.md.tmpl" ] && topn=$({ grep -ci "arvo" "$d/mock/DESIGN.md.tmpl" || true; })
  crates=0
  percrate=0
  if [ -d "$d/mock/crates" ]; then
    crates=$(find "$d/mock/crates" -mindepth 1 -maxdepth 1 -type d | grep -c . || true)
    percrate=$(find "$d/mock/crates" -mindepth 2 -maxdepth 2 -name 'DESIGN.md.tmpl' -exec grep -ci "arvo" {} \; 2>/dev/null | awk '{s+=$1} END{print s+0}')
  fi
  echo "######## $repo   crates=$crates   arvo-lines: top-level=$topn  per-crate DESIGN=$percrate"
  if [ "$percrate" -gt 0 ]; then
    find "$d/mock/crates" -mindepth 2 -maxdepth 2 -name 'DESIGN.md.tmpl' | sort | while read -r f; do
      n=$({ grep -ci "arvo" "$f" || true; })
      [ "$n" -eq 0 ] && continue
      echo "  --- $(basename "$(dirname "$f")")  ($n lines)"
      { grep -in "arvo" "$f" || true; } | cut -c1-185 | sed 's/^/      /'
    done
  fi
  echo
done
echo "### END OF RUN. If this line is missing the walk was truncated."
