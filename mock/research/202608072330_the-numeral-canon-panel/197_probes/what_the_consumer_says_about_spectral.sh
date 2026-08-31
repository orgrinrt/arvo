#!/usr/bin/env bash
# What hilavitkutin's own design says arvo owes for the spectral step.
#
# `a_spectral_partition_of_a_dependency_graph` was reworded to ask arvo for
# "the Laplacian over an adjacency the consumer already holds, the iteration
# that finds the Fiedler vector, and the bisection and k-way split over it",
# and its `gap` justifies the widening by saying the consumer's "foundations
# table gives arvo the Laplacian construction, the power iteration, the Fiedler
# vector and both split forms".
#
# An obligation is definitionally the consumer's need in the consumer's terms,
# so that claim is checkable against the consumer. This prints the two lines it
# could rest on, from a committed ref rather than from a working tree, because
# the clone is on a recovery branch and what a seat happens to have checked out
# is not what the consumer says.
#
# Both refs are printed. If they agree, which ref anybody read does not matter.
set -euo pipefail
# Resolved from this script's own location rather than written down. The repo
# root is the nearest ancestor holding `mockspace.toml` and the consumer is a
# sibling clone of it in the workspace. A committed probe naming somebody's
# checkout does not fail elsewhere: where that checkout exists it succeeds and
# reports about a different tree. `a_probe_reads_the_tree_it_sits_in` refused
# this file at the first version and was right to.
root="$(cd "$(dirname "$0")" && pwd)"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
[ -f "$root/mockspace.toml" ] || { echo "run me from inside the repository" >&2; exit 2; }
repo="${1:-$(dirname "$root")/hilavitkutin}"
[ -d "$repo" ] || { echo "no consumer clone at $repo" >&2; exit 2; }

for ref in origin/dev recover/unlanded-rounds-and-benches; do
  echo "######## $ref"
  echo "--- mock/DESIGN.md.tmpl, the foundations-crate sentence"
  git -C "$repo" show "$ref:mock/DESIGN.md.tmpl" 2>/dev/null \
    | grep -n 'foundations crate dependency' | fold -s -w 100 | sed 's/^/  /'
  echo "--- mock/crates/hilavitkutin/DESIGN.md.tmpl, the spectral step"
  git -C "$repo" show "$ref:mock/crates/hilavitkutin/DESIGN.md.tmpl" 2>/dev/null \
    | grep -n -A3 'Spectral partitioning' | sed 's/^/  /'
  echo
done

echo "######## the words the rewording attributes to that table"
for w in "Laplacian construction" "power iteration" "both split forms" "bisection"; do
  n=0
  for ref in origin/dev recover/unlanded-rounds-and-benches; do
    for f in mock/DESIGN.md.tmpl mock/crates/hilavitkutin/DESIGN.md.tmpl; do
      c=$(git -C "$repo" show "$ref:$f" 2>/dev/null | grep -ci -- "$w" || true)
      n=$((n + c))
    done
  done
  printf '  %-24s occurrences across both refs and both files: %s\n' "$w" "$n"
done

echo
echo "######## control: a phrase that IS there, and one that is not"
for w in "engine-local" "Fiedler partition step" "ZZZ_NOT_IN_ANY_DESIGN"; do
  c=$(git -C "$repo" show origin/dev:mock/DESIGN.md.tmpl 2>/dev/null | grep -ci -- "$w" || true)
  d=$(git -C "$repo" show origin/dev:mock/crates/hilavitkutin/DESIGN.md.tmpl 2>/dev/null | grep -ci -- "$w" || true)
  printf '  %-24s top-level=%s per-crate=%s\n' "$w" "$c" "$d"
done
