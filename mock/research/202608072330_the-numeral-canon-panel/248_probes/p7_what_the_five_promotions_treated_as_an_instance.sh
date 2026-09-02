#!/usr/bin/env nutshell
# p7. What the corpus's own expert promotions treated as an independent instance.
#
# Written after the blind commit, for the reconciliation.
#
# `246` and `247` both hold that the canon does not say what an instance is when
# the claim was in the row the seat answered, and `247` names ONE precedent, R3,
# and finds it does not cover the case. There are five rulings at
# `ratified_by = "experts"`, every one carries a `promotion` field, and every one
# of those fields states the test it used. This enumerates them and greps each
# for the instrument it names.
#
# THE CASES THAT MUST FAIL, STATED BEFORE THE RUN.
#
# C1. The count of `ratified_by = "experts"` rulings and the count of
#     `promotion` fields must both be five and must be equal. If they differ, a
#     promotion sits on a ruling this does not read, or vice versa, and the
#     enumeration is not the enumeration.
# C2. Each of the four instrument phrases must be found in at least one
#     `promotion` field. A phrase found nowhere is one I invented while reading.
# C3. A phrase that must NOT be there: `standing`, as a justification for a
#     promotion. If a promotion field says it promoted on a `standing` value, the
#     claim that these promote on the gate and the instruments is wrong.
# C4. The grep must return zero for a phrase known to be absent, and non-zero for
#     one known present, on the same field set. Without both, an empty result and
#     a broken extractor read alike.
use log

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT" || exit 1
R="mock/registry/ruling.toml"

# The `promotion` fields, one per line, which is how they are written.
grep '^promotion = ' "$R" > /tmp/p7_promotions.txt
NP=$(wc -l < /tmp/p7_promotions.txt | tr -d ' ')
NE=$(grep -c '^ratified_by = "experts"' "$R")

echo "== the namespace =="
echo "  rulings at ratified_by = \"experts\":  $NE"
echo "  rulings carrying a promotion field:  $NP"
echo
echo "== which rulings they are =="
grep -B14 '^ratified_by = "experts"' "$R" | grep '^id = ' | sed 's/^/  /'
echo

echo "== the instruments each promotion names =="
show() { # phrase label
  local n; n=$(grep -c "$1" /tmp/p7_promotions.txt)
  printf "  %-58s x%s\n" "$2" "$n"
}
show 'phase-one commit precedes'                    "blind commit lands before the reading commit"
show 'not an ancestor'                              "mutual non-ancestry of the two branches"
show 'on the gate rather than on the count'         "promoted on the gate, not on the count"
show 'read the same paragraph rather than by two instruments' "shared-reading agreement excluded"
show 'no longer serve as a blind instance'          "a leaked seat withdraws as a blind instance"
show 'checkable rather than asserted'               "the general form: checkable, not asserted"
echo

fail=0
chk() { if [ "$2" = "$3" ]; then echo "  PASS $1"; else echo "  FAIL $1: expected $2, got $3"; fail=1; fi }

echo "== the cases that had to fail =="
chk "C1a five experts-ratified rulings" 5 "$NE"
chk "C1b five promotion fields"         5 "$NP"
chk "C1c the two counts agree"          "$NE" "$NP"

miss=0
for p in 'phase-one commit precedes' 'not an ancestor' 'on the gate rather than on the count' 'read the same paragraph rather than by two instruments'; do
  [ "$(grep -c "$p" /tmp/p7_promotions.txt)" -ge 1 ] || { echo "    absent: $p"; miss=1; }
done
[ "$miss" -eq 0 ] && r=yes || r=no
chk "C2 every instrument phrase is present in some promotion field" yes "$r"

SG=$(grep -ci 'promoted on .*standing\|because its standing' /tmp/p7_promotions.txt)
chk "C3 no promotion justifies itself by a standing value" 0 "$SG"

ABSENT=$(grep -c 'zzz_no_such_phrase_zzz' /tmp/p7_promotions.txt)
PRESENT=$(grep -c 'Two' /tmp/p7_promotions.txt)
[ "$ABSENT" -eq 0 ] && [ "$PRESENT" -ge 1 ] && r=yes || r=no
chk "C4 the extractor gives 0 for an absent phrase and $PRESENT for a present one" yes "$r"
echo

echo "== what this establishes =="
echo "  Five promotions, five stated tests, and every one of them is an ORDERING"
echo "  or an EXCLUSION test rather than a judgement of how good an argument is:"
echo "  a blind commit landing before the reading commit; mutual non-ancestry;"
echo "  promotion on the gate rather than the count; agreement traceable to a"
echo "  shared reading excluded from what the promotion rests on; and a seat whose"
echo "  blindness leaked withdrawing as a blind instance while its earlier"
echo "  commits still count."
echo
echo "  The fourth of those covers the case 246 and 247 hand back. A seat that"
echo "  agreed with a conclusion because it read the conclusion is agreement from"
echo "  a shared reading, and the corpus already excludes that from a promotion."
exit $fail
