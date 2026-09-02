#!/usr/bin/env nutshell
# What is actually in the probe corpus, counted rather than remembered.
#
# Required outcomes, written before the run:
#
#   C1  the directory count must be non-zero and must match `ls -d *_probes` by
#       hand. An extractor that walks the wrong tree reports zero and reads as
#       "no probes", which is the failure mode that matters here.
#   C2  a directory known to hold a defect-marked artifact by filename
#       (175_probes/clause23, which holds clause23_v1_CONTROLS_FAILED.out) must
#       appear in the marked list. If it does not, the marker regex is wrong and
#       every "no defect marked" verdict below is worthless.
#   C3  a directory known to hold NO defect marker (147_probes, two files) must
#       NOT appear in the marked list. Without this arm the regex could be
#       matching everything.
#
# C2 and C3 are the pair. C2 alone is satisfied by a regex matching every line.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
panel="$(cd "$here/.." && pwd)"
out="$here/p2_census.out"

# Filename markers this corpus uses to flag a run that did not stand.
# Established by reading names rather than by guessing: CONTROLS_FAILED,
# REFUTED, defective, first_attempt, _corrected, run1.
marker='CONTROLS_FAILED|REFUTED|defective|DEFECTIVE|first_attempt|_corrected|run1|_wrong|_broken|superseded'

{
  printf '=== p2 census, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  printf '## directories\n'
  dirs=$(cd "$panel" && ls -d *_probes 2>/dev/null | sort)
  n_dirs=$(printf '%s\n' "$dirs" | grep -c . || true)
  printf 'probe directories: %s\n' "$n_dirs"
  printf 'artifact files under them: %s\n' \
    "$(cd "$panel" && find . -path './*_probes/*' -type f | wc -l | tr -d ' ')"
  printf 'sketch directories: %s\n' \
    "$(ls -d "$panel"/../sketches/*/ 2>/dev/null | wc -l | tr -d ' ')"
  printf 'bench artifacts (csv): %s\n' \
    "$(ls "$panel"/../../benches/*.csv 2>/dev/null | wc -l | tr -d ' ')"
  printf '\n'

  printf '## extensions across the probe tree\n'
  ( cd "$panel" && find . -path './*_probes/*' -type f | sed 's/.*\.//' | sort | uniq -c | sort -rn )
  printf '\n'

  printf '## directories whose filenames flag a run that did not stand\n'
  marked=$(cd "$panel" && find . -path './*_probes/*' -type f \
    | grep -E "$marker" | sed 's|^\./||' | sort)
  printf '%s\n' "$marked"
  printf 'count: %s files in %s directories\n\n' \
    "$(printf '%s\n' "$marked" | grep -c . || true)" \
    "$(printf '%s\n' "$marked" | cut -d/ -f1 | sort -u | grep -c . || true)"

  printf '## C2: 175_probes/clause23 must be marked\n'
  if printf '%s\n' "$marked" | grep -q '^175_probes/clause23/'; then
    printf 'C2 PASS: found\n'
  else
    printf 'C2 FAIL: the marker regex does not see a file this corpus names CONTROLS_FAILED\n'
  fi
  printf '\n'

  printf '## C3: 147_probes must NOT be marked\n'
  if printf '%s\n' "$marked" | grep -q '^147_probes/'; then
    printf 'C3 FAIL: the regex matches a directory with no defect marker, so it matches too much\n'
  else
    printf 'C3 PASS: not present\n'
  fi
  printf '\n'

  printf '## per directory: files, and whether committed output sits beside the source\n'
  printf '%-28s %5s %5s %s\n' dir files outs langs
  for d in $dirs; do
    files=$(find "$panel/$d" -type f | wc -l | tr -d ' ')
    outs=$(find "$panel/$d" -type f \( -name '*.out' -o -name '*out*.txt' -o -name '*output*' -o -name '*.log' \) | wc -l | tr -d ' ')
    langs=$(find "$panel/$d" -type f -name '*.*' | sed 's/.*\.//' \
      | grep -E '^(py|rs|sh|s|awk|c|cpp|zig|toml)$' | sort -u | tr '\n' ',' | sed 's/,$//')
    printf '%-28s %5s %5s %s\n' "$d" "$files" "$outs" "${langs:--}"
  done
} 2>&1 | tee "$out"
