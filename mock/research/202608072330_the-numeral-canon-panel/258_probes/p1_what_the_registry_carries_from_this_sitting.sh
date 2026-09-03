#!/usr/bin/env bash
# Seat 258. What the registry carries from seats 241 through 257.
#
# 257 section 4 measured zero rows carrying a provenance reference to seats 241
# through 256, with seat 240 as its control. This is a fourth arrival on that
# and it is run rather than inherited for one reason: 257's tree was `bd2916f3`
# and the trunk has moved twice since, so "does it still hold" is a question 257
# could not answer. It also widens the range to 257 and prints the per-seat
# counts rather than the maximum alone, because a maximum hides a hole in the
# middle of a range.
#
# THE CASES THAT MUST FAIL, run before the census is reported:
#   C1  A planted registry file citing seat 999 must come back in the census.
#       Without it a broken extractor and an empty result are the same output.
#   C2  Seat 240 must come back with a nonzero count, which is 257's control and
#       is what says the pattern matches the citation form the rows actually use.
#   C3  A seat number that cannot be cited because no such file exists, 998, must
#       come back zero while C1's 999 comes back nonzero in the same run.
set -u
cd "$(dirname "$0")/../../../.." || exit 1 # the repository root
fail() {
	echo "CONTROL FAILED: $1"
	exit 2
}

PAT='the-numeral-canon-panel::[0-9]*'
PLANT=mock/registry/zz_258_control.toml

echo "tree: $(git rev-parse HEAD)"
echo "root: $(pwd)"
echo

# --- C1 and C3: plant, measure, remove -----------------------------------------
cat >"$PLANT" <<'EOF'
# Seat 258's control plant. Removed by the probe that wrote it.
[[probe]]
id = "zz_258_control"
provenance = ["panel::202608072330_the-numeral-canon-panel::999_control_plant::#1"]
EOF
planted=$(grep -roh "$PAT" mock/registry/ | sed 's/.*:://' | grep -c '^999$' || true)
absent=$(grep -roh "$PAT" mock/registry/ | sed 's/.*:://' | grep -c '^998$' || true)
rm -f "$PLANT"
[ "$planted" -ge 1 ] || fail "C1: the planted seat 999 citation was not seen; the extractor is broken"
[ "$absent" -eq 0 ] || fail "C3: seat 998 was reported present and no such citation was planted"
echo "C1 planted seat 999 seen        : $planted   (must be >= 1)"
echo "C3 unplanted seat 998 seen      : $absent   (must be 0)"
[ -f "$PLANT" ] && fail "the control plant was not removed"

# --- C2: the control seat ------------------------------------------------------
c240=$(grep -roh "$PAT" mock/registry/ | sed 's/.*:://' | grep -c '^240$' || true)
[ "$c240" -ge 1 ] || fail "C2: seat 240 returns zero, so the pattern does not match the citation form"
echo "C2 control, seat 240 citations  : $c240   (must be >= 1)"
echo

# --- the census ----------------------------------------------------------------
echo "citations per seat, 241 through 258:"
total=0
for n in $(seq 241 258); do
	c=$(grep -roh "$PAT" mock/registry/ | sed 's/.*:://' | grep -c "^${n}\$" || true)
	total=$((total + c))
	printf '  %s : %s\n' "$n" "$c"
done
echo
echo "total citations of seats 241..258 : $total"
echo "highest seat cited anywhere in the registry:"
grep -roh "$PAT" mock/registry/ | sed 's/.*:://' | sort -n | tail -1
echo
echo "total citations of this panel in the registry, all seats:"
grep -roh "$PAT" mock/registry/ | wc -l | tr -d ' '
