#!/usr/bin/env nutshell
# File `35` derived what the layers above need from the numeral and measured it
# with three independent instruments. Which of its figures reached the registry?
#
# `35` is the panel's own cold derivation of the surface above the numeral. It
# is 884 lines, it names ten requirements, and sections 3.4, 3.5 and 3.5a carry
# the numbers. If the canon reaches the algorithm surface anywhere, these
# figures are where it would show.
#
# Every figure below is quoted from `35` at the line given, so a reader can
# check both sides. The question is one-directional and mechanical: does the
# string occur anywhere in `mock/registry/`?
#
# CONTROLS, three, and the run does not count without all three.
#   POSITIVE-A `476` and `897`, the coherence law's witness counts. They are in
#     `law.toml` and must report present. If they do not, the grep is broken.
#   POSITIVE-B `12.6`, which `35` measured and which is quoted in a retirement
#     row. Must report present, and it is the one that makes the result
#     interesting rather than an artifact of formatting: it proves a `35` figure
#     *can* survive, so the absences are about selection and not about notation.
#   NEGATIVE `999999999`, a figure in neither. Must report absent.
#
# The formatting hazard is real and is why POSITIVE-B matters: `35` writes
# `5,414,255` with separators and a registry row could carry `5414255` without.
# So every multi-digit figure is searched in both spellings.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
[ -f "$root/mockspace.toml" ] || { echo "run me from inside the repository" >&2; exit 2; }
reg="$root/mock/registry"
src="$root/mock/research/202608072330_the-numeral-canon-panel/35_mcsherry_what_the_layers_above_need_from_the_numeral.md"
[ -f "$src" ] || { echo "35 not found" >&2; exit 2; }

echo "### registry: $reg"
echo "### source:   $(basename "$src")"
echo

check() {
  label="$1"; needle="$2"; kind="$3"
  bare=$(printf '%s' "$needle" | tr -d ',')
  n=$({ grep -roh -- "$needle" "$reg" 2>/dev/null || true; } | grep -c . || true)
  m=0
  [ "$bare" = "$needle" ] || m=$({ grep -roh -- "$bare" "$reg" 2>/dev/null || true; } | grep -c . || true)
  tot=$((n + m))
  # in `35` itself, to confirm the figure is really that file's
  s=$({ grep -c -- "$needle" "$src" || true; })
  if [ "$tot" -eq 0 ]; then verdict="ABSENT "; else verdict="present"; fi
  printf "  %-9s %-8s %-14s registry=%-3s (in 35: %s)  %s\n" "$kind" "$verdict" "$needle" "$tot" "$s" "$label"
}

echo "######## 3.4  absorption: the top absorbs under saturation, never under wrapping"
check "saturation absorbs at every cell"      "63 of 63"    "finding"
check "wrapping absorbs at no cell"           "0 of 63"     "finding"
echo
echo "######## 3.5  monotonicity, the other half min-plus needs"
check "holds every cell under saturation"     "33 of 33"    "finding"
check "worst-case triples under wrapping"     "33.07"       "finding"
echo
echo "######## 3.5  the end-to-end DAG run, with its in-range control"
check "shortest path wrong, w=3 wrapping"     "5,414,255"   "finding"
check "shortest path wrong, w=4 wrapping"     "407,293,133" "finding"
check "in-range instances, longest w=4"       "736,300,800" "finding"
check "in-range instances, shortest w=4"      "832,398,764" "finding"
check "percent wrong, w=3"                    "45.4"        "finding"
check "percent wrong, w=4"                    "48.9"        "finding"
echo
echo "######## 3.5a the shape 35 proposed and killed"
check "reserved top, percent still wrong"     "12.6"        "CTRL+B"
check "reserved top, monotonicity failures"   "560 of 2176" "finding"
check "wrapping monotonicity failures"        "680 of 2176" "finding"
check "reserved top, instances"               "78.2"        "finding"
echo
echo "######## controls"
check "coherence law witness, signed sat"     "476"         "CTRL+A"
check "coherence law witness, mutant"         "897"         "CTRL+A"
check "a figure in neither document"          "999999999"   "CTRL-N"
