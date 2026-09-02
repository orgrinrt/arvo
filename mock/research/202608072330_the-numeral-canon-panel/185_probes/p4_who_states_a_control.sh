#!/usr/bin/env nutshell
# How much of this probe corpus states a control at all.
#
# The headline ratio the dispatch asks for. It is a weak measure and is reported
# as one: what it counts is a source file whose own text names a case that had
# to come out a particular way. It CANNOT tell whether that case fired, whether
# it could have fired, or whether it tested the mechanism rather than the scope.
# Every one of those has to be read. What it can do is bound the population, and
# a file that never uses the vocabulary certainly did not write one down.
#
# Required outcomes, written before the run:
#
#   C1  a file known to state a control must be counted. `168_probes/p5_which_
#       mask_blocks_the_vectoriser.rs` carries "THE CASE THAT MUST DIFFER, or
#       this instrument reports nothing", so it must be in the stated set.
#   C2  at least one probe source file must come back with NO control language,
#       and the set must be non-empty. If every file is counted the matcher is
#       matching the corpus rather than the property.
#   C3  the matcher must not fire on `controller`, `controlled` or `control
#       flow`, which are ordinary words in this material. Planted directly.
#   C4  the two sets must partition: stated + unstated == total source files.
#       A file counted twice or dropped makes the ratio arithmetic on nothing.
#
# C2 and C3 are the pair that matter. C1 alone is satisfied by a matcher that
# says yes to everything.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
panel="$(cd "$here/.." && pwd)"
out="$here/p4_who_states_a_control.out"
tsv="$here/p4_who_states_a_control.tsv"

# Source files only. An output file quoting the word proves nothing about the
# instrument, and a .txt of results is not where a control is declared.
sources() {
  ( cd "$panel" && find . -path './*_probes/*' -type f \
      \( -name '*.py' -o -name '*.rs' -o -name '*.sh' -o -name '*.awk' -o -name '*.zig' \) \
      -not -path './185_probes/*' | sed 's|^\./||' | sort )
}

# The vocabulary this corpus uses to write a control down. `control` must be a
# standalone word (C3), and the alternatives are the phrasings that appear
# without it.
# `control flow` is ordinary programming prose and is not a control. Run one
# counted it and C3 fired, which is the arm doing its job; 426 of 1133 was that
# run's figure and `p4_run1_control_flow_overmatch.out` keeps it. `controller`
# and `controlled` never matched, because the trailing `[^A-Za-z]` already
# excluded them, so the arm as first written tested three words at once and
# could not say which broke it. It is three arms now.
CTL='(^|[^A-Za-z])[Cc]ontrols?([^A-Za-z]|$)|MUST (FAIL|DIFFER|BE|NOT|MATCH|REPORT)|must fail|must differ|must be True|must be true|negative control|required outcome|planted|sanity check|had to fail'
NOTCTL='[Cc]ontrol[- ](flow|depend|charact|regist|structur|word)'
matches() { grep -E "$CTL" "$1" 2>/dev/null | grep -vE "$NOTCTL" | grep -q . ; }

: > "$tsv"
while IFS= read -r f; do
  if matches "$panel/$f"; then
    printf '%s\tstated\n' "$f" >> "$tsv"
  else
    printf '%s\tunstated\n' "$f" >> "$tsv"
  fi
done < <(sources)

{
  printf '=== p4 who states a control, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  total=$(wc -l < "$tsv" | tr -d ' ')
  stated=$(awk -F'\t' '$2=="stated"' "$tsv" | wc -l | tr -d ' ')
  unstated=$(awk -F'\t' '$2=="unstated"' "$tsv" | wc -l | tr -d ' ')

  printf '## C1: a file known to state a control must be counted\n'
  if grep -q '^168_probes/p5_which_mask_blocks_the_vectoriser.rs	stated$' "$tsv"; then
    printf 'C1 PASS\n'
  else
    printf 'C1 FAIL: the matcher misses a file that writes its control in capitals\n'
  fi
  printf '\n'

  printf '## C2: the unstated set must be non-empty\n'
  if [ "$unstated" -gt 0 ]; then
    printf 'C2 PASS: %s of %s source files use none of the vocabulary\n' "$unstated" "$total"
  else
    printf 'C2 FAIL: every file counted, so the matcher matches the corpus not the property\n'
  fi
  printf '\n'

  printf '## C3: three planted lines, one arm each, so a fail names the word\n'
  tmp=$(mktemp)
  for probe in 'the controller resets the state' \
               'the value is controlled by the caller' \
               'this branch changes control flow' \
               'a control-dependency on the mask'; do
    printf '%s\n' "$probe" > "$tmp"
    if matches "$tmp"; then printf '  FAIL  counted: %s\n' "$probe"
    else printf '  pass  ignored: %s\n' "$probe"; fi
  done
  printf '   the positive half, so C3 is not passing by matching nothing:\n'
  for probe in 'the control must fail here' \
               'C1: the wide arm MUST DIFFER' \
               'planted absent fragment not found'; do
    printf '%s\n' "$probe" > "$tmp"
    if matches "$tmp"; then printf '  pass  counted: %s\n' "$probe"
    else printf '  FAIL  missed:  %s\n' "$probe"; fi
  done
  rm -f "$tmp"
  printf '\n'

  printf '## C4: the two sets must partition\n'
  if [ "$((stated + unstated))" -eq "$total" ]; then
    printf 'C4 PASS: %s + %s = %s\n' "$stated" "$unstated" "$total"
  else
    printf 'C4 FAIL: %s + %s != %s\n' "$stated" "$unstated" "$total"
  fi
  printf '\n'

  printf '## the ratio\n'
  printf 'probe source files (py, rs, sh, awk, zig), excluding this dispatch: %s\n' "$total"
  printf 'naming a case that had to come out a particular way:                %s\n' "$stated"
  printf 'naming none:                                                        %s\n' "$unstated"
  awk -v s="$stated" -v t="$total" 'BEGIN{printf "share stating one: %.1f%%\n", 100*s/t}'
  printf '\n'
  printf 'WHAT THIS DOES NOT SAY: that the stated ones fired, that they could\n'
  printf 'have fired, or that they tested the mechanism rather than the scope.\n'
  printf 'Four defects in this corpus were controls that could not fail, and this\n'
  printf 'instrument counts every one of them as stated.\n\n'

  printf '## the unstated files, in full\n'
  awk -F'\t' '$2=="unstated"{print "  " $1}' "$tsv"
  printf '\n'
  printf '## per directory, stated / total\n'
  awk -F'\t' '{split($1,a,"/"); d=a[1]; t[d]++; if($2=="stated") s[d]++}
    END{for(d in t) printf "%-30s %3d / %3d\n", d, s[d]+0, t[d]}' "$tsv" | sort
} > "$out" 2>&1
cat "$out"
