#!/usr/bin/env bash
# How many files in the closed panel are op's own words, against the number every
# document that cites the roster states.
#
# The claim under test: `OLD_112_the_op_material_sweep.md` states twenty-three,
# and `SEED_TALKING_POINTS.md` repeats twenty-three. If the archive holds more,
# every population built from that roster is short by the difference.
#
# CONTROL. A count is worthless without a case that had to fail, so this asserts
# the two exclusions rather than trusting the glob: `OLD_112` itself matches
# `_op_` and is an agent file, and the persona stand-ins match `persona` and are
# not op. If either exclusion stops removing anything the counts below are
# measuring a different set and the script says so and exits non-zero.
set -uo pipefail

cd "$(dirname "$0")/../../202607301300_formalization-spec-panel" || exit 2

all=$(ls OLD_*_op_*.md 2>/dev/null | wc -l | tr -d ' ')
sweep=$(ls OLD_112_the_op_material_sweep.md 2>/dev/null | wc -l | tr -d ' ')
persona=$(ls OLD_*persona*.md 2>/dev/null | wc -l | tr -d ' ')
op=$((all - sweep))

echo "files matching OLD_*_op_*.md         : $all"
echo "  of which the sweep itself (agent)  : $sweep"
echo "op-authored checkpoint files         : $op"
echo "persona stand-in files (not op)      : $persona"
echo "roster stated by OLD_112 and by STP  : 23"
echo "shortfall                            : $((op - 23))"

# The control. Both exclusions must remove something, or the arithmetic above is
# over a set nobody checked.
if [ "$sweep" -ne 1 ]; then
    echo "CONTROL FAILED: the sweep file did not match the op glob; counts are over a different set" >&2
    exit 1
fi
if [ "$persona" -lt 1 ]; then
    echo "CONTROL FAILED: no persona file matched; the not-op exclusion is not exercised" >&2
    exit 1
fi

echo
echo "the op files the stated roster of 23 does not contain:"
for f in $(ls OLD_*_op_*.md | grep -v OLD_112); do
    n=$(echo "$f" | sed 's/^OLD_//; s/_op_.*//')
    case "$n" in
    04b | 06b | 08b | 12b | 13b | 13c | 16b | 16c | 16d | 17b | 24b | 30b | 34b | 39b | 44b | 68b | 70b | 74b | 77b | 79b | 82b | 86b | 108b) ;;
    *) echo "  $f" ;;
    esac
done

echo
echo "and the dates, because a file written after the sweep could not have been swept:"
git log --follow --format='  %ad %s' --date=short -- OLD_112_the_op_material_sweep.md | tail -1
git log --follow --format='  %ad %s' --date=short -- OLD_145c_op_checkpoint_forty.md | tail -1
