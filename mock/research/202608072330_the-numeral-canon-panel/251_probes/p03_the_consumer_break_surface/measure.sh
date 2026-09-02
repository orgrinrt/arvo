#!/usr/bin/env bash
# How much shipped consumer source names an arvo type arvo does not have.
#
# Reads the object store at a named ref, never a working tree, because a count
# over a corpus is a claim about which corpus and a working tree is nobody's
# stated ref. The ref each name resolved to is printed with the count.
#
# Run from anywhere: pass the workspace root that holds the consumer clones.
#   ./measure.sh /path/to/workspace-root
set -uo pipefail

ROOT="${1:?usage: measure.sh <workspace-root holding the consumer clones>}"

# repo:ref pairs. vehje and tarina have no dev branch on their remotes.
TREES="hilavitkutin:origin/dev kolli:origin/dev vehje:origin/main notko:origin/dev"

# The names the deleted crate tree exported, as consumers still write them.
NAMES="USize ISize Cap Bits UFixed IFixed FastFloat StrictFloat Mask64 Mask256"

# A name no crate anywhere defines. If this ever returns non-zero the pattern,
# the pathspec or the shell is lying and every number below is void.
CONTROL_NAME="ZZThisTypeHasNeverExisted"

printf '%-14s %-12s %s\n' repo ref commit
for pair in $TREES; do
    repo="${pair%%:*}"; ref="${pair##*:}"
    oid=$(git -C "$ROOT/$repo" rev-parse "$ref" 2>&1) || { echo "UNRESOLVED $repo $ref: $oid"; exit 2; }
    printf '%-14s %-12s %s\n' "$repo" "$ref" "$oid"
done

echo
printf '%-14s %-12s %-8s %s\n' repo name files occurrences
for pair in $TREES; do
    repo="${pair%%:*}"; ref="${pair##*:}"
    for n in $NAMES $CONTROL_NAME; do
        files=$(git -C "$ROOT/$repo" grep -l "\\b$n\\b" "$ref" -- 'mock/crates/*/src/*' 'src/*' 2>/dev/null | wc -l | tr -d ' ')
        occ=$(git -C "$ROOT/$repo" grep -o "\\b$n\\b" "$ref" -- 'mock/crates/*/src/*' 'src/*' 2>/dev/null | wc -l | tr -d ' ')
        [ "$files" = "0" ] && [ "$n" != "$CONTROL_NAME" ] && continue
        printf '%-14s %-12s %-8s %s\n' "$repo" "$n" "$files" "$occ"
    done
done

echo
echo "the control must read 0 everywhere above; a non-zero there voids the rest."
