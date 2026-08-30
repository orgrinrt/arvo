#!/usr/bin/env nutshell
# The same question asked of `43`, the panel's cold derivation of what a
# composition is. One file losing its results is an accident; two is a pattern
# in the port, and the pattern is the finding.
#
# `43` is 934 lines and answers the aggregate sense of "composition", which is
# the sense op's I11 uses. Its figures are quoted from its section 0.
#
# DEFECT IN VERSION ONE, kept here because it is the reason this arm is shaped
# the way it is. The first run used a bare substring grep and reported `4096`,
# `58` and `94` present. All three were false: `4096` matched a law's "4096
# triples", `58` matched the digits inside `12,582,912`, and `94` matched a file
# citation `94:887`. A short decimal is a substring of the corpus's other
# numbers and of its own citations, so a substring grep on one cannot answer
# this question at all. Every match is therefore printed with its surrounding
# text, and the verdict for a short figure is `CHECK` rather than `present`
# until a reader has looked at the line. The transcript of the bad run is in
# `which_of_43s_figures_survived_v1_substring.out`.
#
# CONTROLS, three.
#   POSITIVE-A `476` / `897`, present in `law.toml`, must report matches.
#   POSITIVE-B none available: no figure of `43`'s survived, so there is no
#     in-file positive to plant. Stated rather than hidden, and it is the result.
#   NEGATIVE `999999999`, must report zero.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
reg="$root/mock/registry"
src="$root/mock/research/202608072330_the-numeral-canon-panel/43_rompf_what_a_composition_is.md"
[ -f "$src" ] || { echo "43 not found" >&2; exit 2; }

check() {
  label="$1"; needle="$2"; kind="$3"
  bare=$(printf '%s' "$needle" | tr -d ',')
  hits=$({ grep -rohE "[^0-9,]($needle|$bare)[^0-9]" "$reg" 2>/dev/null || true; } | grep -c . || true)
  ctx=$({ grep -rohE ".{50}[^0-9,]($needle|$bare)[^0-9].{20}" "$reg" 2>/dev/null || true; })
  s=$({ grep -c -- "$needle" "$src" || true; })
  short=$(printf '%s' "$bare" | wc -c | tr -d ' ')
  if [ "$hits" -eq 0 ]; then v="ABSENT"; elif [ "$short" -le 5 ]; then v="CHECK "; else v="present"; fi
  printf "  %-8s %-7s %-16s matches=%-3s (in 43: %s)  %s\n" "$kind" "$v" "$needle" "$hits" "$s" "$label"
  [ "$hits" -eq 0 ] || printf '%s\n' "$ctx" | sed 's/^/           | /'
}

echo "######## 43 section 0, the figures its instruments produced"
check "two-level shapes one bit wide, s3"      "1201"    "finding"
check "two-level shape space, s3"              "4096"    "finding"
check "negative control overflow witness, s3"  "6502"    "finding"
check "grids matching the prediction, s5"      "8 of 18" "finding"
check "grids at zero bias, s5"                 "9 of 9"  "finding"
check "assembly lines, clamped subslice, p7"   "58"      "finding"
check "assembly lines, enforced run, p7"       "94"      "finding"
echo
echo "######## controls"
check "coherence law witness, signed sat"      "476"       "CTRL+A"
check "coherence law witness, mutant"          "897"       "CTRL+A"
check "a figure in neither document"           "999999999" "CTRL-N"
echo
echo "######## and the concepts, not the numbers: does any row state them?"
for t in "capacity against count" "len <= capacity" "binding-time distinction" "flattening" "traversal order" "shared scale" "frame of reference" "per-aggregate"; do
  n=$({ grep -ric -- "$t" "$reg" 2>/dev/null || true; } | awk -F: '{s+=$2} END{print s+0}')
  printf "  %-28s registry=%s\n" "$t" "$n"
done
