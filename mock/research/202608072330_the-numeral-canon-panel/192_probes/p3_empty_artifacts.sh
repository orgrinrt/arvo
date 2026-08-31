#!/usr/bin/env nutshell
# A committed artifact with zero bytes cannot be cited, and how many there are.
#
# Found by hitting it: a `lives` entry naming `06_probes/p3_asm.out` is refused
# as `unresolvable-provenance`, "points past the end (1 requested)", because
# every probe citation carries a terminal `::1` and a zero-byte file has no line
# one. So the evidence that a compile emitted nothing is the one kind of evidence
# the citation grammar cannot point at.
#
# The second question is whether an empty output is a defect. Mostly it is not:
# a captured stdout from a clean build is legitimately empty. This separates the
# two rather than reporting a scary count.
#
# Required outcomes, written before the run:
#
#   C1  a non-empty artifact at line 1 must resolve, and an empty one must be
#       refused. Both planted through the real lint, because the claim is about
#       the engine rather than about my reading of it.
#   C2  the empty set must be non-empty and must not be everything.
#   C3  the split between build-or-compile captures and other outputs must be
#       computed, or "56 empty outputs" is a number with no reading attached.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
panel="$(cd "$here/.." && pwd)"
root="$(cd "$panel/../../.." && pwd)"
plant="$root/mock/registry/zzz_planted_empty_artifact.toml"
out="$here/p3_empty_artifacts.out"

cleanup() { rm -f "$plant"; }
trap cleanup EXIT

try_cite() {
  local label="$1" target="$2"
  cat > "$plant" <<TOML
[[probe]]
id = "planted_empty_artifact_arm"
establishes = "nothing; this row exists to see whether the citation resolves."
lives = ["panel::202608072330_the-numeral-canon-panel::${target}::1"]
control = "this row IS the control."
standing = "sound"
TOML
  local r
  # Filtered to the planted row's own id. Run one took `head -1` of every error
  # in the file and a pre-existing broken citation elsewhere in probe.toml
  # answered for both arms, so the control and the case it controlled reported
  # the same thing and the arm established nothing.
  r=$( cd "$root" && cargo mock --lint-only 2>&1 \
        | grep 'planted_empty_artifact_arm' | grep -oE 'ERROR \[[a-z-]+\]' | head -1 || true )
  printf '  %-28s %-46s %s\n' "$label" "$target" "${r:-resolves}"
}

{
  printf '=== p3 empty artifacts, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  printf '## C1: can an empty artifact be cited at all\n'
  try_cite 'non-empty (must resolve)' '06_probes/p1.out'
  try_cite 'zero bytes (must refuse)' '06_probes/p3_asm.out'
  printf '\n'

  empty=$( cd "$panel" && find . -path './*_probes/*' -type f -empty | sed 's|^\./||' | sort )
  ne=$(printf '%s\n' "$empty" | grep -c . || true)
  allf=$( cd "$panel" && find . -path './*_probes/*' -type f | wc -l | tr -d ' ')
  printf '## C2: the empty set is a subset\n'
  if [ "$ne" -gt 0 ] && [ "$ne" -lt "$allf" ]; then
    printf 'C2 PASS: %s empty of %s artifact files\n' "$ne" "$allf"
  else
    printf 'C2 FAIL: %s of %s\n' "$ne" "$allf"
  fi
  printf '\n'

  printf '## C3: what the empty ones are\n'
  build=$(printf '%s\n' "$empty" | grep -cE '(build|compile|link|meta|asm)' || true)
  other=$((ne - build))
  printf 'naming build, compile, link, meta or asm: %s\n' "$build"
  printf 'everything else:                         %s\n' "$other"
  printf '\n'
  printf 'the ones that are NOT a build or compile capture:\n'
  printf '%s\n' "$empty" | grep -vE '(build|compile|link|meta|asm)' | sed 's/^/  /'
  printf '\n'
  printf 'READING. A captured stdout from a clean build is legitimately empty and\n'
  printf 'most of these are that. The count is not a defect count and is not\n'
  printf 'reported as one. What it bounds is how much committed evidence the\n'
  printf 'citation grammar cannot reach.\n'
} 2>&1 | tee "$out"
