#!/usr/bin/env nutshell
# The two lists this dispatch works from, regenerated rather than taken on trust.
#
# LIST A. Proposal rows whose `sentence_kind` is one the checks call RAN_SOMETHING
# (`measured` or `enumeration`) and whose `evidence` is empty. The committed
# ceiling says 21 and a ceiling is not a list, so this extracts them with their
# topic and the file they sit in.
#
# LIST B. Probe rows whose own `control` field the checker reads as an admission
# that none was run. That set is now load-bearing: `measurements_resting_on_an_
# unusable_instrument` reports any RAN_SOMETHING claim citing one. Reading it
# matters to me specifically, because the phrase list is a substring match and
# a row of mine describing a control that DID fire could contain one by accident.
#
# Required outcomes, written before the run:
#
#   C1  list A must have the same length as the committed ceiling, 21. A
#       different number means either the extractor is wrong or the registry
#       moved, and both need saying before anything is built on the list.
#   C2  list A must be non-empty AND must exclude at least one RAN_SOMETHING row
#       that DOES carry evidence. Without the second half the extractor could be
#       selecting on sentence_kind alone.
#   C3  the phrase matcher for list B must reproduce the checker's own five
#       phrases exactly, read out of `shape.rs` rather than typed here, or list B
#       is my guess at what the checker does.
#   C4  a probe row describing a control that fired must NOT land in list B.
#       Planted from a real row.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
reg="$repo/mock/registry"
out="$here/p1_what_is_blocked.out"

# C3: the phrases, read from the source of truth.
phrases=$(sed -n '/fn names_no_control/,/^}/p' "$repo/mock/checks/src/shape.rs" \
  | grep -oE '"[a-z ]+"' | tr -d '"')

# One record per proposal row: id, file, sentence_kind, topic, has_evidence.
rows=$(awk '
  /^\[\[proposal\]\]/ { if (id != "") print id "\t" f "\t" sk "\t" tp "\t" (ev ? "yes" : "no");
                        id=""; sk=""; tp=""; ev=0; f=FILENAME; sub(/.*\//,"",f) }
  /^id = /            { id=$0; sub(/id = "/,"",id); sub(/"$/,"",id) }
  /^sentence_kind = / { sk=$0; sub(/sentence_kind = "/,"",sk); sub(/"$/,"",sk) }
  /^topic = /         { tp=$0; sub(/topic = "/,"",tp); sub(/"$/,"",tp) }
  /^evidence = \[/    { if ($0 !~ /\[\]/) ev=1 }
  END { if (id != "") print id "\t" f "\t" sk "\t" tp "\t" (ev ? "yes" : "no") }
' "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml")

{
  printf '=== p1 what is blocked, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  printf '## C3: the five phrases the checker reads, taken from shape.rs\n'
  printf '%s\n' "$phrases" | sed 's/^/  /'
  n_ph=$(printf '%s\n' "$phrases" | grep -c . || true)
  if [ "$n_ph" -eq 5 ]; then printf 'C3 PASS: five phrases extracted\n'
  else printf 'C3 FAIL: extracted %s, so the function shape changed and list B is a guess\n' "$n_ph"; fi
  printf '\n'

  printf '## LIST A: ran something, names no instrument\n'
  blocked=$(printf '%s\n' "$rows" | awk -F'\t' '($3=="measured" || $3=="enumeration") && $5=="no"')
  na=$(printf '%s\n' "$blocked" | grep -c . || true)
  printf '%s\n' "$blocked" | awk -F'\t' '{printf "  %-52s %-32s %-12s %s\n", $1, $4, $3, $2}'
  printf 'count: %s\n\n' "$na"

  ceiling=$(grep -oE 'WITHOUT_AN_INSTRUMENT: usize = [0-9]+' \
    "$repo/mock/checks/tests/what_one_field_obliges_another_to_carry.rs" | grep -oE '[0-9]+')
  printf '## C1: must equal the committed ceiling\n'
  if [ "$na" -eq "$ceiling" ]; then printf 'C1 PASS: %s against a ceiling of %s\n' "$na" "$ceiling"
  else printf 'C1 FAIL: %s extracted against a ceiling of %s\n' "$na" "$ceiling"; fi
  printf '\n'

  printf '## C2: the extractor must also exclude a RAN_SOMETHING row that has evidence\n'
  withev=$(printf '%s\n' "$rows" | awk -F'\t' '($3=="measured" || $3=="enumeration") && $5=="yes"' | grep -c . || true)
  if [ "$withev" -gt 0 ]; then
    printf 'C2 PASS: %s such rows exist and are correctly absent from list A\n' "$withev"
    printf '%s\n' "$rows" | awk -F'\t' '($3=="measured" || $3=="enumeration") && $5=="yes" {printf "  excluded: %s\n", $1}' 
  else
    printf 'C2 FAIL: no RAN_SOMETHING row carries evidence, so the evidence filter is untested\n'
  fi
  printf '\n'

  printf '## LIST B: probe rows the checker reads as having no control\n'
  # The phrase list goes through a file rather than `awk -v`. Run one passed it
  # with -v and awk refused it as "newline in string", printing the error once
  # per input line and producing an empty list B, which reads exactly like "no
  # row has an uncontrolled instrument".
  phfile=$(mktemp); printf '%s\n' "$phrases" > "$phfile"
  awk -v PF="$phfile" '
    BEGIN { while ((getline line < PF) > 0) if (line != "") ph[++np] = line }
    /^\[\[probe\]\]/ { id=""; ctl="" }
    /^id = /      { id=$0; sub(/id = "/,"",id); sub(/"$/,"",id) }
    /^control = / { ctl=tolower($0)
                    for (i=1; i<=np; i++) if (index(ctl, ph[i])) {
                      printf "  %-56s matched: %s\n", id, ph[i]; break } }
  ' "$reg/probe.toml"
  rm -f "$phfile"
  printf '\n'

  printf '## C4: a row describing a control that fired must not be in list B\n'
  tmp=$(mktemp)
  printf 'control = "Three cases stated in the header before the run and all three fired."\n' > "$tmp"
  hit=no
  while IFS= read -r p; do
    [ -z "$p" ] && continue
    grep -qi -- "$p" "$tmp" && hit=yes
  done <<< "$phrases"
  if [ "$hit" = no ]; then printf 'C4 PASS: not matched\n'
  else printf 'C4 FAIL: a fired-control sentence is read as an admission\n'; fi
  printf '   and the positive half:\n'
  printf 'control = "None was run, and the shape admits no obvious one."\n' > "$tmp"
  hit=no
  while IFS= read -r p; do
    [ -z "$p" ] && continue
    grep -qi -- "$p" "$tmp" && hit=yes
  done <<< "$phrases"
  if [ "$hit" = yes ]; then printf '   PASS: an admission is matched\n'
  else printf '   FAIL: the matcher matches nothing at all\n'; fi
  rm -f "$tmp"
} > "$out" 2>&1
cat "$out"
