#!/usr/bin/env bash
# p1. What `standing` actually rests on, measured over the committed registry.
#
# Four readings. Every one of them is preceded by the case that must fail,
# shown failing, because three instruments in this sitting returned a clean
# zero while broken.
#
#   A. standing x distinct-provenance-files, over every proposal row.
#   B. the standing of every proposal an expert ratification names.
#   C. registry-slug citation density in panel member files, split at 189.
#   D. question.toml's header against its own rows.
#
# Run from anywhere inside the repo. Writes only to stdout.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)"

HERE=mock/research/202608072330_the-numeral-canon-panel/249_probes
AWKF=$HERE/rows.awk
REG=mock/registry
PANEL=mock/research/202608072330_the-numeral-canon-panel
SLUG='`(proposal|question|ruling|law|probe|retirement|obligation|dimension)::'
T=$(mktemp -d)

echo "=================================================================="
echo "CONTROLS -- nothing below counts unless each of these behaves"
echo "=================================================================="

cat > "$T/plant.toml" <<'TOML'
[[proposal]]
id = "planted_two_anchors_one_file"
standing = "two_experts"
provenance = [
  "panel::p::74_giesen_consolidation::614",
  "panel::p::74_giesen_consolidation::251",
]

[[proposal]]
id = "planted_two_files"
standing = "two_experts"
provenance = [
  "panel::p::65_knuth_derived_cold::519",
  "panel::p::66_dolan_derived_cold::255",
]
TOML
echo "-- C1 two anchors into one file must read as ONE file, two files as TWO --"
awk -f "$AWKF" "$T/plant.toml"
echo "   expected: 1 for the first row, 2 for the second."

echo
echo "-- C2 the standing field is really being read: mutate it and watch it move --"
sed 's/standing = "two_experts"/standing = "three_or_more"/' "$T/plant.toml" > "$T/mut.toml"
awk -f "$AWKF" "$T/mut.toml" | cut -f1,2
echo "   expected: both rows now three_or_more. Same output as C1 would mean the"
echo "   extractor is printing a constant rather than reading the field."

echo
echo "-- C3 a pattern that must be ZERO on the real tree --"
printf '   zzznotanamespace:: hits: '
grep -rhoE '`zzznotanamespace::' "$PANEL"/[0-9]*_*.md 2>/dev/null | wc -l | tr -d ' '
echo "-- C4 the same instrument, the real pattern, must be NON-ZERO --"
printf '   registry-slug hits across the panel: '
grep -rhoE "$SLUG" "$PANEL"/[0-9]*_*.md 2>/dev/null | wc -l | tr -d ' '

echo
echo "=================================================================="
echo "A. standing against the number of DISTINCT files its provenance names"
echo "=================================================================="
awk -f "$AWKF" "$REG/proposal.toml" "$REG/proposal-the-later-topics.toml" > "$T/rows.tsv"
printf 'proposal rows read: '; wc -l < "$T/rows.tsv" | tr -d ' '
awk -F'\t' '{print $2"\tfiles="$3}' "$T/rows.tsv" | sort | uniq -c | sort -k2,2
printf '\nmulti-arrival standing resting on ONE file: '
awk -F'\t' '($2=="two_experts"||$2=="three_or_more"||$2=="cross_topic") && $3<2' "$T/rows.tsv" | wc -l | tr -d ' '
printf 'the ceiling the lint carries:               '
grep -oE 'const CEILING: usize = [0-9]+' mock/lints/a_standing_is_reachable_from_what_it_cites.rs

echo
echo "=================================================================="
echo "B. the proposals an expert ratification names, and their standing"
echo "=================================================================="
# Every quoted token appearing anywhere in ruling.toml, intersected with the set
# of real proposal ids. Loose on the left, exact on the right: prose cannot
# enter the list because prose is not a proposal id.
grep -ohE '"[a-z0-9_]{8,}"' "$REG/ruling.toml" | tr -d '"' | sort -u > "$T/cand.txt"
cut -f1 "$T/rows.tsv" | sort -u > "$T/ids.txt"
comm -12 "$T/cand.txt" "$T/ids.txt" > "$T/named.txt"
printf 'proposal ids named anywhere in ruling.toml: '; wc -l < "$T/named.txt" | tr -d ' '
while read -r s; do
  awk -F'\t' -v k="$s" '$1==k{printf "  %-60s %-14s files=%s\n", $1, $2, $3}' "$T/rows.tsv"
done < "$T/named.txt"
echo
echo "  of those, how many rest on one file:"
while read -r s; do awk -F'\t' -v k="$s" '$1==k{print $3}' "$T/rows.tsv"; done < "$T/named.txt" \
  | awk '$1<2{n++} END{print "  " n+0 " of " NR}'

echo
echo "=================================================================="
echo "C. registry-slug citations in panel member files, split at 189"
echo "=================================================================="
bf=0; af=0; bfiles=0; afiles=0; before=0; after=0
for f in "$PANEL"/[0-9]*_*.md; do
  b=$(basename "$f"); n=${b%%_*}; n=${n//[!0-9]/}
  case "$n" in 244|245|246|247|248|249) continue;; esac
  c=$(grep -cE "$SLUG" "$f"); c=${c:-0}
  if [ "$n" -lt 189 ]; then bf=$((bf+1)); before=$((before+c)); [ "$c" -gt 0 ] && bfiles=$((bfiles+1))
  else af=$((af+1)); after=$((after+c)); [ "$c" -gt 0 ] && afiles=$((afiles+1)); fi
done
echo "files numbered  < 189: $bf files, $bfiles cite the registry, $before citation lines"
echo "files numbered >= 189: $af files, $afiles cite the registry, $after citation lines"
echo "(244 through 249 excluded: seat 249 had not read them when this ran.)"

echo
echo "=================================================================="
echo "D. question.toml's header against its own rows"
echo "=================================================================="
grep -n 'No answer is recorded here' "$REG/question.toml"
printf 'question rows:      '; grep -c '^\[\[question\]\]' "$REG/question.toml"
printf 'carrying answered:  '; grep -c '^answered = '      "$REG/question.toml"
printf 'carrying bound:     '; grep -c '^bound = '         "$REG/question.toml"
printf 'carrying options:   '; grep -c '^options = '       "$REG/question.toml"
rm -rf "$T"
