#!/usr/bin/env bash
# Seat 247. Where the two "blind" cold seats got the content that 244, 245 and
# 246 counted as independent second instances of 73's two proposals.
#
# The claim under test: seat 241's Q31 answer and the enumeration clause of Q30
# reached the cold seats through the question rows themselves, because the
# question rows Q29, Q30 and Q31 were ported from OPTIONS.md entries that file 73
# wrote. A cold seat that reads the row's `options` and `note` before deriving is
# reading 73's compressed argument, and 244/245/246 measured blindness only
# against `mock/research/`, which is not where the leak is.
#
# Established here, each mechanically:
#   (1) 73 says in its own text that it appended Q29, Q30 and Q31 to OPTIONS.md.
#   (2) The oldest commit that introduces each of the three headings in OPTIONS.md
#       is the commit that introduced 73's own file.
#   (3) The registry rows Q29/Q30/Q31 carry provenance pointing at those OPTIONS
#       anchors, and the port commit predates the cold-open base 87ab5d70.
#   (4) 241's pre-reconciliation body (a664fffb) cites "the row's own `note`" to
#       refute Q31's option 1 and adopts option 3's "scoped to a target" shape;
#       242's blind body (64ab711e) quotes Q30's `note` for the Gray-code clause.
#
# THE CASES THAT MUST FAIL, run before anything above is reported:
#   C1  Q20's heading in OPTIONS.md is introduced by a commit that is NOT 73's,
#       so the instrument distinguishes a row 73 wrote from one it did not.
#   C2  The phrase 241's independent route turns on, "missing a coordinate", is in
#       ruling.toml (R2) and in neither 73 nor the Q29-Q31 block of OPTIONS.md, so
#       the instrument can see content 241 did NOT get from 73.
#   C3  A phrase planted to be absent returns zero everywhere, so a zero below is
#       a fact about the files and not about a broken grep.
set -u
cd "$(dirname "$0")/.." || exit 1   # the panel directory
REG=../../registry
fail() { echo "CONTROL FAILED: $1"; exit 2; }
g() { command grep "$@"; }            # the workspace shims grep; use the binary

F73=73_leijen_the_membership_test_and_how_wide.md
F241=241_kiselyov_admission_is_a_resolution_not_a_verdict.md
F242=242_what-admits-a-number-system.md
PANEL=mock/research/202608072330_the-numeral-canon-panel

oldest_commit_introducing() {  # oldest_commit_introducing <literal> <file>
  git log --format='%h' -S "$1" -- "$2" | tail -1
}
C73=$(git log --format='%h' --diff-filter=A -- "$F73" | tail -1)
[ -n "$C73" ] || fail "cannot find the commit that added 73"

# --- C1: a heading 73 did not write is introduced by a different commit --------
CQ20=$(oldest_commit_introducing '### Q20.' OPTIONS.md)
[ -n "$CQ20" ] || fail "C1, Q20 heading not found in OPTIONS.md history"
[ "$CQ20" != "$C73" ] || fail "C1, Q20's heading is attributed to 73's commit; the instrument cannot distinguish authorship"
echo "C1 passes: Q20 heading first appears in $CQ20, which is not 73's commit $C73."

# --- C2: 241's independent route phrase is NOT in 73 or the Q29-Q31 block ------
BLOCK=$(awk '/^### Q29\./{p=1} /^### Q32\./{p=0} p' OPTIONS.md)
n_r2=$(g -c 'missing a coordinate' $REG/ruling.toml)
n_73=$(g -c 'missing a coordinate' "$F73")
n_blk=$(printf '%s\n' "$BLOCK" | g -c 'missing a coordinate')
[ "$n_r2" -ge 1 ] || fail "C2, R2's phrase not found in ruling.toml"
[ "$n_73" -eq 0 ] && [ "$n_blk" -eq 0 ] || fail "C2, R2's phrase is in 73 or the Q29-Q31 block, so route disjointness is not visible to this instrument"
echo "C2 passes: 'missing a coordinate' occurs $n_r2 time(s) in ruling.toml, 0 in 73, 0 in OPTIONS Q29-Q31."

# --- C3: a planted absent phrase returns zero -----------------------------------
for f in OPTIONS.md "$F73" "$F241" "$F242" $REG/question.toml; do
  [ "$(g -c 'seat247_planted_absent_phrase_xyzzy' "$f")" -eq 0 ] || fail "C3, the absent phrase matched in $f"
done
echo "C3 passes: a planted absent phrase returns zero in all five files."
echo

# --- (1) 73 states it appended Q29-Q31 -------------------------------------------
echo "(1) 73's own statement:"
g -n 'appended to `OPTIONS.md` as \*\*Q29, Q30 and Q31\*\*' "$F73" | sed 's/^/    /'
[ "$(g -c 'appended to `OPTIONS.md` as \*\*Q29, Q30 and Q31\*\*' "$F73")" -eq 1 ] || fail "(1) 73's statement not found"
echo

# --- (2) the three headings are introduced by 73's commit ------------------------
echo "(2) oldest commit introducing each heading in OPTIONS.md, against 73's commit $C73:"
allmatch=1
for h in '### Q29. What does the admission contract ask a candidate to expose' \
         '### Q30. Is admission a predicate or a location' \
         '### Q31. Does the canon use one word or two'; do
  c=$(oldest_commit_introducing "$h" OPTIONS.md)
  printf '    %-70s %s\n' "$h" "$c"
  [ "$c" = "$C73" ] || allmatch=0
done
[ $allmatch -eq 1 ] || fail "(2) at least one heading is not introduced by 73's commit"
echo "    all three match: $C73 $(git log -1 --format='%ad %s' --date=short $C73)"
echo

# --- (3) the rows point at those anchors and predate the cold base ---------------
echo "(3) question rows and their provenance anchors:"
for q in what_the_admission_contract_asks_a_candidate_to_expose is_admission_a_predicate_or_a_location one_word_or_two_for_is_a_number_system; do
  prov=$(awk -v s="$q" 'BEGIN{RS="\\[\\[question\\]\\]"} $0 ~ "id = \""s"\"" {print}' $REG/question.toml | g -o 'OPTIONS::#q[0-9]*[a-z-]*' | head -1)
  printf '    %-58s %s\n' "$q" "$prov"
  [ -n "$prov" ] || fail "(3) $q carries no OPTIONS provenance"
done
PORT=$(git log --format='%h' -S 'one_word_or_two_for_is_a_number_system' -- $REG/question.toml | tail -1)
BASE=87ab5d70
git merge-base --is-ancestor "$PORT" "$BASE" || fail "(3) the port commit $PORT is not an ancestor of the cold-open base $BASE"
echo "    rows ported in $PORT ($(git log -1 --format=%ad --date=iso $PORT)), an ancestor of the cold-open base $BASE ($(git log -1 --format=%ad --date=iso $BASE))."
echo

# --- (4) the blind bodies cite the rows' notes and options -----------------------
echo "(4a) 241's pre-reconciliation body at a664fffb, Q31 section:"
git show a664fffb:$PANEL/$F241 | g -n "row's own \`note\`\|Q31's option 3\|scoped to a target" | sed 's/^/    /'
n241=$(git show a664fffb:$PANEL/$F241 | g -c "row's own \`note\`")
[ "$n241" -ge 1 ] || fail "(4a) 241's blind body does not cite the row's note"
echo "(4b) 242's blind body at 64ab711e, Q30 section:"
git show 64ab711e:$PANEL/$F242 | g -n "row's own note\|rather than rejections" | sed 's/^/    /'
n242=$(git show 64ab711e:$PANEL/$F242 | g -c "rather than rejections")
[ "$n242" -ge 1 ] || fail "(4b) 242's blind body does not quote Q30's note"
echo
echo "VERDICT: Q29, Q30 and Q31 were written by 73, ported into the registry before the cold open,"
echo "         and both cold seats cite their notes and options inside the bodies they committed blind."
echo "         Blindness measured against mock/research/ alone does not bound independence from 73."
