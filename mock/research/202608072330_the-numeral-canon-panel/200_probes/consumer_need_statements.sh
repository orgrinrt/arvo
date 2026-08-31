#!/usr/bin/env bash
# The consumer statements this pass re-derived the demand side from, quoted
# from the consumer rather than from a summary of it.
#
# Clones the four consumers if they are not already in this worktree's ignored
# build directory, so the probe is runnable rather than a record of a read.
# They go under `mock/target/` because that is gitignored: a read-only copy of
# somebody else's repository is disposable, and reaching into a sibling clone
# somebody may be mid-edit in is the thing this avoids.
#
# The interesting output is which consumers say something at all. `184` read
# three at one level; `191` re-read one and reported kolli at zero.
set -euo pipefail
root="$(cd "$(dirname "$0")" && pwd)"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
C="$root/mock/target/consumers"
mkdir -p "$C"
for r in hilavitkutin vehje kolli tarina; do
  [ -d "$C/$r" ] || git clone -q --depth 1 --single-branch --branch dev \
    "git@github.com:orgrinrt/$r.git" "$C/$r" 2>/dev/null || true
done

echo "######## how many lines under mock/ name arvo, per consumer"
for r in hilavitkutin vehje kolli tarina; do
  n=$(grep -rhi 'arvo' "$C/$r/mock" --include='*.md' --include='*.tmpl' 2>/dev/null | wc -l | tr -d ' ')
  f=$(grep -rli 'arvo' "$C/$r/mock" --include='*.md' --include='*.tmpl' 2>/dev/null | wc -l | tr -d ' ')
  printf '  %-14s lines=%-6s files=%s\n' "$r" "$n" "$f"
done

echo
echo "######## the statements, quoted"
q() { printf '\n  -- %s:%s\n' "$1" "$2"; sed -n "${2}p" "$C/$1" | sed 's/^/     /'; }
q kolli/mock/DESIGN.md.tmpl 106
q kolli/mock/DESIGN.md.tmpl 109
q kolli/mock/DESIGN.md.tmpl 110
q kolli/mock/DESIGN.md.tmpl 114
q tarina/mock/research/canon/03-algebra.md 236
q tarina/mock/research/canon/03-algebra.md 238
q tarina/mock/research/canon/03-algebra.md 244
q hilavitkutin/mock/crates/hilavitkutin-api/DESIGN.md.tmpl 155
q hilavitkutin/mock/crates/hilavitkutin/DESIGN.md.tmpl 36
q hilavitkutin/mock/crates/hilavitkutin/DESIGN.md.tmpl 39

echo
echo "######## controls"
printf '  a phrase that IS in kolli:      %s\n' "$(grep -c 'unstable machinery' "$C/kolli/mock/DESIGN.md.tmpl" || true)"
printf '  a phrase that is in no consumer: %s\n' "$(grep -rc 'ZZZ_NOT_IN_ANY_CONSUMER' "$C"/*/mock 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')"
