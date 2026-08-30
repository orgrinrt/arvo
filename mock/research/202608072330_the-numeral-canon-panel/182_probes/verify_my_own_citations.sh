#!/usr/bin/env bash
# Opens every file:line and file:range this deliverable names, and prints what is
# there, so a reader can see whether the sentence around it supports the claim.
#
# The registry's own citations are checked by `show_cited_lines.sh`. This checks
# the ones in the prose of `182`, which no linter reads at all. A prose citation
# is the least guarded thing in the corpus: a row's provenance is resolved by the
# lint, and a paragraph's `file:line` is resolved by nobody.
#
# Run from the repository root.
set -uo pipefail
D=mock/research/202608072330_the-numeral-canon-panel
M=mockspace.toml

show() { printf '%-34s %s\n' "$1:$2" "$(sed -n "${2}p" "$3" | cut -c1-92)"; }

echo "=== mockspace.toml, the ranges 182 names ==="
for l in 304 324 327 658 737 803 806 1000 1012 1119; do show mockspace.toml "$l" "$M"; done

echo
echo "=== 63, the lines and range endpoints 182 names ==="
for l in 184 190 217 223 364 368 370 380 413 496 515 534 535 683 691 731; do
  show 63 "$l" "$D/63_spj_consolidation_the_format_concept.md"; done

echo
echo "=== 74 ==="
for l in 654 698; do show 74 "$l" "$D/74_giesen_consolidation_the_number_system_concept.md"; done

echo
echo "=== 90 ==="
for l in 105 115 307 339 341 378 415 443 464 475 476 503 564; do
  show 90 "$l" "$D/90_giesen_consolidation_derived_algebraic_laws.md"; done

echo
echo "=== 106 ==="
for l in 63 394 400 1352 1358 1360 1385; do
  show 106 "$l" "$D/106_giesen_consolidation_the_strategy_axis.md"; done

echo
echo "=== arithmetic 182 states, recomputed ==="
w=$(wc -l < "$D/63_spj_consolidation_the_format_concept.md")
x=$(wc -l < "$D/74_giesen_consolidation_the_number_system_concept.md")
y=$(wc -l < "$D/90_giesen_consolidation_derived_algebraic_laws.md")
z=$(wc -l < "$D/106_giesen_consolidation_the_strategy_axis.md")
echo "lines in the four consolidations: $w + $x + $y + $z = $((w + x + y + z))   (182 section 1 says 4,333)"
echo "rows across both files this pass wrote: $(grep -c '^\[\[proposal\]\]' mock/registry/proposal.toml) + $(grep -c '^\[\[law\]\]' mock/registry/law.toml) = $(( $(grep -c '^\[\[proposal\]\]' mock/registry/proposal.toml) + $(grep -c '^\[\[law\]\]' mock/registry/law.toml) ))   (182 section 13 says 82)"
echo "predicate entries summed: $(grep -hcE '^  "[a-z_]+: ' mock/registry/proposal.toml mock/registry/law.toml | paste -sd+ - | bc)   (182 section 2 says 246)"

echo
echo "=== NEGATIVE CONTROL: a line past the end of a real file must print nothing ==="
printf 'expect an empty value after the colon: ['
printf '%s' "$(sed -n '99999p' "$D/63_spj_consolidation_the_format_concept.md")"
printf ']\n'
