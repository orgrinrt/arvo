#!/usr/bin/env bash
# The anchor set difference between the four consolidations and the rows ported
# out of them.
#
# The count alone is not the check. The set difference is, and every entry in it
# is a claim whose evidence went missing while the compression grew.
#
# TIER. These rows are canon candidates, so the anchors that must survive are
# the panel-internal and probe ones. Citations into the deleted crate tree do
# not count and are not to be restored, per the canon/design/code chain: a canon
# anchored to source cannot be the oracle for that source.
#
# CASE THAT MUST FAIL: the control at the bottom diffs each source against
# ITSELF. That must return empty. If it returns entries, the extractor is
# unstable (sort order, dedup, or the pattern matching differently on two
# passes) and every non-empty diff above it is noise.
set -uo pipefail
cd "$(dirname "$0")/.."
REG=../../registry
SRC="63_spj_consolidation_the_format_concept.md
74_giesen_consolidation_the_number_system_concept.md
90_giesen_consolidation_derived_algebraic_laws.md
106_giesen_consolidation_the_strategy_axis.md
AGREEMENTS.md"

# A panel-internal anchor as the corpus writes it in prose: `NN:LINE`,
# `NN_probes/file`, `FILE.md:LINE`. Normalised to the bare target so the two
# spellings of one thing compare equal.
extract_prose() {
  {
    grep -oE '`[0-9]+(_probes/[A-Za-z0-9_.-]+)?' "$1" | tr -d '`'
    grep -oE '[0-9]+_probes/[A-Za-z0-9_.*-]+' "$1"
    grep -oE '\b(AGREEMENTS|OPTIONS|DROPLIST|RULES|INTENTS|PRIOR_CALLS)\.md' "$1"
  } | sed 's|_probes/.*|_probes|' | sort -u
}

# A registry citation, normalised the same way: the panel file stem's leading
# number, or the probe directory.
extract_rows() {
  grep -ohE 'panel::[A-Za-z0-9_.#-]+::[A-Za-z0-9_.#-]+(::[A-Za-z0-9_.#-]+)*' "$REG"/proposal.toml "$REG"/law.toml \
    | awk -F'::' '{print $3; if ($4 ~ /_probes/) print $4}' \
    | sed -E 's/^([0-9]+)_.*/\1/; s|_probes.*|_probes|' \
    | sort -u
}

ROWS=$(mktemp); extract_rows > "$ROWS"

echo "=== anchors the rows carry (normalised targets) ==="
tr '\n' ' ' < "$ROWS"; echo; echo "count: $(wc -l < "$ROWS" | tr -d ' ')"
echo

for s in $SRC; do
  P=$(mktemp); extract_prose "$s" > "$P"
  tot=$(wc -l < "$P" | tr -d ' ')
  kept=$(comm -12 "$P" "$ROWS" | wc -l | tr -d ' ')
  echo "=== $s : $tot distinct anchors, $kept survive into a row ==="
  echo "--- DROPPED (the set difference, printed rather than described) ---"
  comm -23 "$P" "$ROWS" | tr '\n' ' '; echo
  echo
  rm -f "$P"
done

echo "=== CONTROL: each source diffed against itself, which must be empty ==="
for s in $SRC; do
  A=$(mktemp); B=$(mktemp)
  extract_prose "$s" > "$A"; extract_prose "$s" > "$B"
  d=$(comm -3 "$A" "$B" | wc -l | tr -d ' ')
  printf '%-56s self-diff = %s%s\n' "$s" "$d" "$( [ "$d" != 0 ] && printf '   *** UNSTABLE EXTRACTOR ***' )"
  rm -f "$A" "$B"
done

echo
echo "=== CONTROL 2: a target no source can contain must be absent from every kept set ==="
if printf 'zzz_not_a_panel_file\n' | comm -12 - "$ROWS" | grep -q .; then
  echo "*** the comm is matching things that are not there ***"
else
  echo "absent, as required"
fi
rm -f "$ROWS"
