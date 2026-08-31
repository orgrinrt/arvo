#!/usr/bin/env bash
# How far back can a reader of a row actually get?
#
# A row cites a consolidation at a line. That line may itself cite a member file
# or a probe, in which case the trail continues, or it may cite nothing, in
# which case the row's evidence ends at a compression. This counts which.
#
# CASE THAT MUST FAIL: control 1 asks the same question of a line window shifted
# 400 lines. If the "carries an onward citation" rate is the same there, the
# measure is about the density of citations in the file rather than about the
# lines the rows chose, and it says nothing about the port. Control 2 runs the
# extractor over a line that provably carries an onward citation and one that
# provably carries none.
set -uo pipefail
cd "$(dirname "$0")/.."
REG=../../registry
OFF=0
[ "${1:-}" = "--control" ] && OFF=400

onward() { # $1 file, $2 line -> prints onward citations in a +/-2 window
  local lo=$(( $2 - 2 )); [ "$lo" -lt 1 ] && lo=1
  sed -n "${lo},$(( $2 + 2 ))p" "$1" 2>/dev/null \
    | grep -oE '`[0-9]+(_probes/[A-Za-z0-9_.-]+)?[:`]|[0-9]+_probes/[A-Za-z0-9_.-]+' \
    | tr -d '`:' | sort -u | tr '\n' ' '
}

tot=0; withref=0
: > /tmp/depth_rows.txt
grep -ohE 'panel::[A-Za-z0-9_.#-]+::[A-Za-z0-9_.#-]+::[0-9]+' "$REG"/proposal.toml "$REG"/law.toml \
  | sort -u | while IFS= read -r ref; do
  dir=$(printf '%s' "$ref" | awk -F'::' '{print $2}')
  file=$(printf '%s' "$ref" | awk -F'::' '{print $3}')
  line=$(printf '%s' "$ref" | awk -F'::' '{print $4}')
  path=$(find "../$dir" -maxdepth 1 -name "$file*" | head -1)
  o=$(onward "$path" $(( line + OFF )))
  printf '%s:%s\t%s\n' "$file" "$line" "${o:-NONE}" >> /tmp/depth_rows.txt
done
tot=$(wc -l < /tmp/depth_rows.txt | tr -d ' ')
withref=$(grep -vc "	NONE$" /tmp/depth_rows.txt || true)
echo "=== distinct line citations from the rows: $tot ==="
echo "of those, the cited window carries an onward panel citation: $withref"
echo "of those, the trail ends at the consolidation:               $(( tot - withref ))"
echo
echo "--- the ones whose trail ends, in full ---"
grep "	NONE$" /tmp/depth_rows.txt | cut -f1 | tr '\n' ' '; echo
echo
echo "--- distinct onward targets recoverable this way ---"
grep -v "	NONE$" /tmp/depth_rows.txt | cut -f2 | tr ' ' '\n' | grep -E '^[0-9]' | sort -u | tr '\n' ' '; echo
cp /tmp/depth_rows.txt "188_probes/citation_depth_rows${OFF}.tsv" 2>/dev/null || cp /tmp/depth_rows.txt "citation_depth_rows${OFF}.tsv"

echo
echo "=== CONTROL 2: the extractor on a line known to carry an onward citation, and one known not to ==="
echo "  90:122 (its window names 76 and 76_probes, so must be non-empty): [$(onward 90_giesen_consolidation_derived_algebraic_laws.md 122)]"
echo "  63:1   (a title line, must be empty):                             [$(onward 63_spj_consolidation_the_format_concept.md 1)]"
