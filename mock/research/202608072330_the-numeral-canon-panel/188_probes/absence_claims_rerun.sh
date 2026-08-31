#!/usr/bin/env bash
# Absence claims, re-run last and separately.
#
# A structural claim degrades when the world moves; an absence claim INVERTS,
# silently, because its own text never changes. And an absence claim names no
# place, so it passes every citation check by construction. The only way to test
# one is to execute the search again rather than re-read its output.
#
# CASE THAT MUST FAIL: every arm below carries a positive control, a search of
# the same shape over the same corpus whose answer is known to be non-empty. An
# arm whose control returns nothing has a broken search and its zero says
# nothing.
set -uo pipefail
cd "$(dirname "$0")/.."

hdr() { printf '\n########## %s\n' "$1"; }

hdr "1. the_multiplicative_guard... gap: \"No bench harness ran on any of it and every member says so\""
echo "  members of the format unit that the guard claim rests on: 60, 62"
for n in 60 62; do
  f=$(ls ${n}_*.md | head -1)
  printf '  %-52s bench-harness mentions: %s\n' "$f" "$(grep -ciE 'bench harness|mock/benches|criterion|ad-hoc quick spike|nothing is timed|no bench' "$f" || true)"
  grep -inE 'ad-hoc quick spike|nothing is timed|no bench (ran|harness)|not a bench' "$f" | sed 's/^/      /' | cut -c1-160
done
echo "  CONTROL: a phrase certainly present in both, to show the grep reaches them"
for n in 60 62; do f=$(ls ${n}_*.md|head -1); printf '    %-50s "probe" hits: %s\n' "$f" "$(grep -ci probe "$f" || true)"; done

hdr "2. a_chain_is_exact_operations... : \"nobody in the unit attacked it\""
echo "  the unit is the format topic: 55 55b 56 57 57b 58 60 61 62. Searching for an attack"
echo "  on the closed-operations / fused-adaptation claim."
for n in 55 55b 56 57 57b 58 60 61 62; do
  f=$(ls ${n}_*.md 2>/dev/null | head -1); [ -z "$f" ] && continue
  h=$(grep -ciE 'chain accuracy|chain-accuracy|unstatable|fuses invisibly|closed over' "$f" || true)
  [ "$h" -gt 0 ] && printf '  %-52s %s hit(s)\n' "$f" "$h"
done
echo "  CONTROL: the same sweep for a term every file in the unit uses"
for n in 55 57 62; do f=$(ls ${n}_*.md|head -1); printf '    %-50s "format" hits: %s\n' "$f" "$(grep -ci format "$f" || true)"; done

hdr "3. an_additive_verdict... : \"nobody in the first five files of the unit had named it\""
echo "  first five files of the format unit: 55 55b 56 57 57b. Searching for same-scale / mixed-scale."
for n in 55 55b 56 57 57b; do
  f=$(ls ${n}_*.md 2>/dev/null | head -1); [ -z "$f" ] && continue
  printf '  %-52s same-scale/common-scale/mixed-scale hits: %s\n' "$f" \
    "$(grep -ciE 'same scale|same-scale|common scale|common-scale|mixed scale|mixed-scale' "$f" || true)"
done
echo "  and where it IS named, for contrast:"
for n in 58 60 61 62 63; do
  f=$(ls ${n}_*.md 2>/dev/null | head -1); [ -z "$f" ] && continue
  h=$(grep -ciE 'same scale|same-scale|common scale|common-scale|mixed scale|mixed-scale' "$f" || true)
  [ "$h" -gt 0 ] && printf '  %-52s %s hit(s)\n' "$f" "$h"
done
echo "  CONTROL: a pattern that must return zero everywhere"
for n in 55 58 63; do f=$(ls ${n}_*.md|head -1); printf '    %-50s zzq hits: %s\n' "$f" "$(grep -ci zzq_no_such "$f" || true)"; done
