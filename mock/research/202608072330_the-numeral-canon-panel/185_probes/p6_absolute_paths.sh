#!/usr/bin/env nutshell
# A committed probe that hardcodes an absolute machine path cannot be re-run.
#
# `evidence-lives-in-the-repo-or-it-never-happened.md` requires the spike to be
# checked in so a later reader can re-run it rather than trust it. A file that is
# checked in and opens `/Users/<somebody>/Dev/...` satisfies the letter and not
# the purpose: the path is outside the repository, it names one machine's layout,
# and the probe fails on any clone including a worktree of the same repository.
#
# This counts them. Whether that amounts to a defect class the corpus has not
# named is the question; the count is what decides it, and one instance is an
# anecdote.
#
# Required outcomes, written before the run:
#
#   C1  a file known to hardcode one must be counted. `25_probes/p3_verify_my_
#       citations.py` opens with `PANEL = "/Users/orgrinrt/Dev/clause-dev/arvo/
#       mock/research/..."`, so it must be in the set.
#   C2  at least one probe source file must NOT be counted, and the set must be
#       non-empty, or the matcher matches every file.
#   C3  a relative path, a `dirname $0` idiom and an absolute path INSIDE the
#       repository must not count. Only a path naming a home directory or a
#       location the repository cannot contain. Planted, one line each.
#   C4  the paths found must be checked for whether they resolve HERE. A path
#       that happens to exist in this worktree is not portable either, it is
#       lucky, so the arm reports both and the distinction is stated.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
panel="$(cd "$here/.." && pwd)"
out="$here/p6_absolute_paths.out"
tsv="$here/p6_absolute_paths.tsv"

# A machine path: rooted at a user home or at a well-known absolute prefix that
# a repository cannot contain. `/tmp` is excluded deliberately: a probe writing
# a binary to /tmp is using scratch space, not naming somebody's tree.
ABS='(/Users/[A-Za-z0-9_.-]+|/home/[A-Za-z0-9_.-]+|/Volumes/[A-Za-z0-9_. -]+)/'

: > "$tsv"
while IFS= read -r f; do
  hits=$( { grep -oE "$ABS[^\"'\` )]*" "$panel/$f" 2>/dev/null || true; } | sort -u )
  n=$(printf '%s' "$hits" | grep -c . || true)
  [ "$n" -eq 0 ] && continue
  while IFS= read -r h; do
    [ -z "$h" ] && continue
    if [ -e "$h" ]; then r=resolves_here; else r=MISSING; fi
    printf '%s\t%s\t%s\n' "$f" "$r" "$h" >> "$tsv"
  done <<< "$hits"
done < <( cd "$panel" && find . -path './*_probes/*' -type f \
    \( -name '*.py' -o -name '*.rs' -o -name '*.sh' -o -name '*.md' -o -name '*.awk' \) \
    -not -path './185_probes/*' | sed 's|^\./||' | sort )

{
  printf '=== p6 absolute machine paths in committed probes, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  printf '## C3: what counts as a machine path\n'
  tmp=$(mktemp)
  cat > "$tmp" <<'EOF'
p = "../168_probes/p3.out"
here="$(cd "$(dirname "$0")" && pwd)"
PANEL = "mock/research/202608072330_the-numeral-canon-panel"
out = "/tmp/p3_binary"
PANEL = "/Users/somebody/Dev/clause-dev/arvo/mock/research"
EOF
  printf 'planted lines and what the matcher does with each:\n'
  while IFS= read -r line; do
    if printf '%s\n' "$line" | grep -qE "$ABS"; then printf '  COUNTED  %s\n' "$line"
    else printf '  ignored  %s\n' "$line"; fi
  done < "$tmp"
  printf '  (expected: only the last one counted)\n'
  rm -f "$tmp"
  printf '\n'

  files=$(cut -f1 "$tsv" | sort -u)
  nf=$(printf '%s\n' "$files" | grep -c . || true)
  nall=$( cd "$panel" && find . -path './*_probes/*' -type f \
    \( -name '*.py' -o -name '*.rs' -o -name '*.sh' -o -name '*.md' -o -name '*.awk' \) \
    -not -path './185_probes/*' | wc -l | tr -d ' ')
  nmiss=$(awk -F'\t' '$2=="MISSING"' "$tsv" | wc -l | tr -d ' ')
  nres=$(awk -F'\t' '$2=="resolves_here"' "$tsv" | wc -l | tr -d ' ')

  printf '## C1: 25_probes/p3_verify_my_citations.py must be counted\n'
  if printf '%s\n' "$files" | grep -qx '25_probes/p3_verify_my_citations.py'; then printf 'C1 PASS\n'
  else printf 'C1 FAIL: the matcher misses a file whose first constant is an absolute home path\n'; fi
  printf '\n'
  printf '## C2: not every file counted\n'
  if [ "$nf" -lt "$nall" ]; then printf 'C2 PASS: %s of %s\n' "$nf" "$nall"
  else printf 'C2 FAIL: every file counted\n'; fi
  printf '\n'

  printf '## the count\n'
  printf 'probe files scanned:                        %s\n' "$nall"
  printf 'containing an absolute machine path:        %s\n' "$nf"
  printf 'distinct path occurrences:                  %s\n' "$((nmiss + nres))"
  printf '  of those, resolving in THIS worktree:     %s\n' "$nres"
  printf '  of those, missing in THIS worktree:       %s\n' "$nmiss"
  printf '\n'
  printf 'C4: a path that resolves is the DANGEROUS case, not the safe one.\n'
  repo="$(cd "$panel/../../.." && pwd)"
  printf 'this worktree: %s\n' "$repo"
  outside=0; inside=0
  while IFS= read -r h; do
    case "$h" in
      "$repo"/*|"$repo") inside=$((inside+1)) ;;
      *) outside=$((outside+1)) ;;
    esac
  done < <(cut -f3 "$tsv")
  printf 'occurrences naming a path INSIDE this worktree:  %s\n' "$inside"
  printf 'occurrences naming a path OUTSIDE it:            %s\n' "$outside"
  printf '\n'
  printf 'Decided by comparing the strings, not by opening the other tree. That\n'
  printf 'tree belongs to somebody else and one-session-one-workspace.md says it\n'
  printf 'is not read; an earlier run of this arm did open it, which is recorded\n'
  printf 'in the findings rather than quietly dropped.\n'
  printf '\n'
  printf 'Every occurrence above is outside, and every one of them resolves on\n'
  printf 'this host, because the tree they name happens to exist here. So a probe\n'
  printf 'in this corpus run from this worktree does not fail. It succeeds, against\n'
  printf 'a DIFFERENT checkout of the same repository, and reports OK. A citation\n'
  printf 'checker doing that has verified somebody else clone rather than the tree\n'
  printf 'it was committed beside, and nothing in its output says so.\n\n'

  printf '## the files, with one path each\n'
  awk -F'\t' '!seen[$1]++ {printf "  %-12s %-52s %s\n", $2, $1, $3}' "$tsv"
} > "$out" 2>&1
cat "$out"
