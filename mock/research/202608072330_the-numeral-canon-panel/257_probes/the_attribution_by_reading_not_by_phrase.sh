#!/usr/bin/env bash
# Seat 257. Whether 242 or 243 carries the two instructions four briefs have
# attributed to seat 242: that its reading of admission not be merged with
# 241's, and that the tier count be recorded as contested.
#
# 244, 245 and 256 each answered this with a phrase grep. A phrase grep
# establishes that a phrase is absent, never that an instruction is absent, so
# this arm is the weakest of the four and is run only for its controls. The
# instrument this file actually rests on is a read of 241, 242 and 243 end to
# end, reported in 257 section 1; a script cannot carry that and this one does
# not pretend to.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

SRC="242_what-admits-a-number-system.md 243_seat242_the_resolution_has_no_second_arm.md"
PAT='merg|combin|synthes|kept apart|read as one|not be (read|joined|merged)|do not (merge|combine)|record.{0,20}contested|as contested'

echo "== ARM 1: the merge-and-contest instruction over 242 and 243 =="
for f in $SRC; do
    printf '  %-58s ' "$f"
    grep -cE "$PAT" "$f"
done

echo
echo "== ARM 1 hits, shown so a reader can judge polarity rather than count =="
grep -nE "$PAT" $SRC || echo "  (none)"

echo
echo "== NEGATIVE CONTROL: a planted instruction of the exact shape must fire =="
PLANT=$(mktemp)
printf 'x\ntheir two readings must not be merged\ny\nthe tier count is recorded as contested\nz\n' > "$PLANT"
printf '  planted lines matching (must be 2): '
grep -cE "$PAT" "$PLANT"
rm -f "$PLANT"

echo
echo "== POSITIVE CONTROL: the instrument can see these two files at all =="
for w in resolution tier coordinate; do
    for f in $SRC; do
        printf '  %-12s in %-58s ' "$w" "$f"
        grep -c "$w" "$f"
    done
done

echo
echo "== ARM 2: what 243 says about 241, which is the opposite polarity =="
grep -n "lands on me\|Where 241 beat me\|withdraw my version\|survives my attack" \
    243_seat242_the_resolution_has_no_second_arm.md || echo "  (none)"
