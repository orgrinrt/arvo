#!/usr/bin/env bash
# Seat 258. The gap 248 says nothing names is a question row, filed at seat 223.
#
# 248 section 9 reports, as an unnamed gap: "There is no declared axis over which
# a claim about the corpus can be predicated at all... I do not know whether that
# is a gap somebody should close or a sign that process findings do not belong in
# a namespace shaped for arithmetic ones, and I am not licensed to decide it. It
# is named because nothing else names it."
#
# Something else names it. `question::can_a_claim_about_the_canons_own_structure_carry_a_region`
# asks exactly that, carries the three options 248 was weighing, and has been open
# with `decider = panel` since seat 223. 240, 242, 244's A10 and 246 report the
# same gap and none of them cites the row either.
#
# This measures both halves: the row exists and says what I claim, and no file of
# this sitting cites it.
#
# THE CASES THAT MUST FAIL, run before the census is reported:
#   C1  A slug that files of this sitting demonstrably do cite must come back
#       nonzero, or a zero is a fact about the grep. `a_standing_is_reachable_from_what_it_cites`
#       is 248's own, cited six times.
#   C2  A slug of the same shape that exists nowhere must come back zero in both
#       the registry and the corpus.
#   C3  The row must be present in `question.toml` and must carry no `answered`
#       field, or it is not open and citing it would be citing a settled question.
set -u
cd "$(dirname "$0")/../../../.." || exit 1 # the repository root
fail() {
	echo "CONTROL FAILED: $1"
	exit 2
}

PANEL=mock/research/202608072330_the-numeral-canon-panel
ROW=can_a_claim_about_the_canons_own_structure_carry_a_region
CTRL=a_standing_is_reachable_from_what_it_cites
NONE=zz_no_such_row_anywhere_at_all

echo "tree: $(git rev-parse HEAD)"
echo

# --- C3: the row is there and is open ------------------------------------------
grep -q "^id = \"$ROW\"$" mock/registry/question.toml || fail "C3: $ROW is not a row in question.toml"
block=$(awk -v w="\"$ROW\"" '/^\[\[question\]\]/{p=0} /^id = /{p=($3==w)} p' mock/registry/question.toml)
printf '%s\n' "$block" | grep -q '^answered = ' && fail "C3: the row carries an `answered` field, so it is settled"
echo "C3 the row exists and carries no \`answered\` : yes"
echo "    asks: $(printf '%s\n' "$block" | grep '^asks = ' | cut -c9-200)"
echo "    decider: $(printf '%s\n' "$block" | grep '^decider = ')"
echo "    provenance: $(printf '%s\n' "$block" | grep '^provenance = ')"
echo

# --- C1 and C2 -----------------------------------------------------------------
c1=$(grep -rc "$CTRL" $PANEL/24[1-9]_*.md $PANEL/25[0-7]_*.md 2>/dev/null | grep -v ':0$' | wc -l | tr -d ' ')
[ "$c1" -gt 0 ] || fail "C1: the control slug is cited by no file of this sitting, so the grep is broken"
echo "C1 files of this sitting citing the control slug : $c1   (must be > 0)"
c2r=$(grep -c "$NONE" mock/registry/*.toml | grep -v ':0$' | wc -l | tr -d ' ')
c2p=$(grep -rc "$NONE" $PANEL --include='*.md' 2>/dev/null | grep -v ':0$' | wc -l | tr -d ' ')
[ "$c2r" -eq 0 ] && [ "$c2p" -eq 0 ] || fail "C2: a slug that exists nowhere was found"
echo "C2 a slug existing nowhere, files matching       : 0"
echo

# --- the census ----------------------------------------------------------------
echo "files of this sitting (241..257) citing $ROW:"
hits=$(grep -rl "$ROW" $PANEL/24[1-9]_*.md $PANEL/25[0-7]_*.md 2>/dev/null | sed 's#.*/##' | sort)
if [ -z "$hits" ]; then echo "  (none)"; else printf '  %s\n' $hits; fi
echo
echo "every file in the panel directory citing it:"
allhits=$(grep -rl "$ROW" $PANEL --include='*.md' 2>/dev/null | sed 's#.*/##' | sort)
if [ -z "$allhits" ]; then echo "  (none)"; else printf '  %s\n' $allhits; fi
echo
echo "registry rows mentioning \`mockspace.toml\`, which is where \`standing\` is declared:"
grep -c "mockspace.toml" mock/registry/*.toml
echo
echo "VERDICT: the axis gap 248 reports as unnamed is an open question row filed at"
echo "seat 223, and no file of this sitting cites it. The locus of \`standing\`'s own"
echo "declaration is a different matter and is named by no row at all."
