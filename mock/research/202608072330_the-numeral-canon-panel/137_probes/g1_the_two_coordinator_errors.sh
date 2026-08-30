#!/usr/bin/env bash
# g1: the two coordinator errors, grepped by me rather than taken from 136.
#   (a) the fabricated figure 21,204 of 32,768
#   (b) the test-gate count "123 across 13" attributed to 125
# 136 section 0.2 claims (b) sits in five places across four files, naming
# 131:48, 131:52, 132:43, 134:13, 135:10. Both the count and the LOCATIONS
# are checked, and the class question is whether the correction reached every
# instance or only the reported ones.
set -u
cd "$(dirname "$0")/.." || exit 1

echo "=== (a) the fabricated figure, across the whole panel ==="
echo "-- any file containing 21,204 or 21204 --"
grep -rln '21,204\|21204' --include='*.md' --include='*.py' --include='*.txt' --include='*.rs' . | sort
echo "-- in 125 and its probes specifically (136 claims zero) --"
grep -rc '21,204\|21204' 125_knuth_rounding_cold_derivation.md 125_probes/ 2>/dev/null | grep -v ':0$' || echo "   zero, as 136 claims"
echo "-- and where the RETRACTIONS live (these are legitimate mentions) --"
grep -rln '21,204\|21204' --include='*.md' . | sort | sed 's/^/     /'

echo
echo "=== (b) every place attributing a completed gate count to 125 ==="
echo "-- every line mentioning '123 across 13' or '123 across thirteen' --"
grep -rn '123 across 13\|123 across thirteen' --include='*.md' . | sed 's/^\.\///' | sort
echo
echo "-- of those, which ATTRIBUTE it to 125 --"
grep -rn '123 across 13' --include='*.md' . | grep -E '125' | sed 's/^\.\///' | sort
echo
echo "-- what 125 section 10 actually records --"
sed -n '463,466p' 125_knuth_rounding_cold_derivation.md
echo
echo "-- and 122's own artifact, which 136 says is the real source --"
tail -3 122_probes/u0_test_gate_run.txt 2>/dev/null || echo "   u0 NOT FOUND"
