#!/usr/bin/env bash
# Seat 246. Is 241 an independent second instance for the two standing
# proposals it is claimed to second, and by what route.
#
# Two promotions are in question:
#   L2  (244, and endorsed by 245): 241's Q30 answer seconds
#       proposal::admission_returns_a_coordinate_rather_than_a_verdict
#   L2' (245, uninspected by 244):  241's Q31 answer seconds
#       proposal::membership_and_hosting_are_two_questions
# Both proposals stand at one_expert with provenance 74 and 73.
#
# Independence is not a thing anybody can assert about their own reading. Three
# things about it ARE mechanically checkable and this instrument checks them:
#
#   1. ROUTE DISJOINTNESS BY DATE. 241's route runs through R2, whose provenance
#      is 225 and 226. If 73 and 74 landed before 225 and 226, then 73's author
#      could not have used R2, whatever either of them remembers. Dates, not
#      claims.
#   2. THE BLIND BODY IS UNTOUCHED. 241 asserts its pre-reconciliation body was
#      committed before it opened anything under mock/research/. Reading leaves
#      no trace, so that assertion is not checkable. What IS checkable is the
#      weaker and still decisive thing: whether 241 edited that body after the
#      commit in which it says the reading began. If the body is byte-identical
#      from the last blind commit onward, then whatever it read did not change
#      the text, so the text on the page is the blind text.
#   3. WHAT 241 SAYS IT READ. 241's reconciliation names its reading list. If
#      the passage a proposal's provenance points at is not on that list, then
#      241 did not inherit it even in the post-blind phase.
#
# THE CASES THAT MUST FAIL, each planted and shown before its number counts:
#   C1  the date comparison must be able to report the opposite order.
#   C2  the section-hash instrument must report a CHANGE where one exists. If it
#       calls everything unchanged, "section 6 is unchanged" means nothing.
#   C3  the citation grep must find a citation that is there, and must find a
#       planted one, or a zero from it is a fact about the grep.

set -u
cd "$(dirname "$0")/../../../.." || exit 1   # repo root
P=mock/research/202608072330_the-numeral-canon-panel
F=$P/241_kiselyov_admission_is_a_resolution_not_a_verdict.md
fail() { echo "CONTROL FAILED: $1"; exit 2; }

added() { git log --diff-filter=A --format='%ct' -1 -- "$1"; }
addeds() { git log --diff-filter=A --format='%ci' -1 -- "$1"; }

f73=$P/73_leijen_the_membership_test_and_how_wide.md
f74=$P/74_giesen_consolidation_the_number_system_concept.md
f225=$P/225_peyton_jones_the_container_premise.md
f226=$P/226_lattner_the_derivation_outputs.md

echo "=== 1. route disjointness, by the dates the files landed ==="
for f in "$f73" "$f74" "$f225" "$f226"; do
  printf '  %-58s %s\n' "$(basename "$f")" "$(addeds "$f")"
done
t73=$(added "$f73"); t74=$(added "$f74"); t225=$(added "$f225"); t226=$(added "$f226")
older=$(( t73 < t225 && t74 < t225 && t73 < t226 && t74 < t226 ))
[ "$older" -eq 1 ] || fail "the provenance files of the proposals are not older than R2's provenance files; the disjointness argument does not run."
echo "  73 and 74 both precede 225 and 226, so neither could have used R2."
# C1: the comparison must be able to come out the other way.
rev=$(( t225 < t73 ))
[ "$rev" -eq 0 ] || fail "C1, the comparison reports both orders at once."
echo "  C1 passes: the same comparison run the other way round returns false, so it is a test and not a constant."
echo "  R2's own provenance field, quoted from the row:"
awk 'BEGIN{RS="\\[\\[ruling\\]\\]"} /id = "the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule"/{print}' \
  mock/registry/ruling.toml | grep '^provenance' | sed 's/^/    /'
echo

echo "=== 2. was 241's pre-reconciliation body edited after the reading began ==="
# Sections, and the commits, in order.
COMMITS=$(git log --reverse --format='%h' -- "$F")
echo "  commits touching 241, oldest first:"
for c in $COMMITS; do printf '    %s  %s\n' "$c" "$(git log -1 --format='%ci %s' "$c")"; done
echo
sect() { # sect <commit> <start-heading-regex> <end-heading-regex>
  git show "$1:$F" | awk -v s="$2" -v e="$3" '$0 ~ s {f=1} $0 ~ e {f=0} f'
}
hash_of() { shasum | cut -c1-12; }
echo "  per-commit hash of section 6 (Q31, the claim L2' turns on):"
for c in $COMMITS; do
  h=$(sect "$c" '^## 6\. Q31' '^## 7\.' | hash_of)
  n=$(sect "$c" '^## 6\. Q31' '^## 7\.' | grep -c .)
  printf '    %s  %s  (%s lines)\n' "$c" "$h" "$n"
done
echo "  per-commit hash of section 8 (Q30, the claim L2 turns on):"
for c in $COMMITS; do
  h=$(sect "$c" '^## 8\. Q30' '^## 9\.' | hash_of)
  printf '    %s  %s\n' "$c" "$h"
done
echo "  per-commit hash of section 5 (Q22), included as the control:"
for c in $COMMITS; do
  h=$(sect "$c" '^## 5\. Q22' '^## 6\.' | hash_of)
  printf '    %s  %s\n' "$c" "$h"
done
# C2: the instrument must report a change somewhere, or "unchanged" is vacuous.
first=$(echo "$COMMITS" | head -1); last=$(echo "$COMMITS" | tail -1)
h5a=$(sect "$first" '^## 5\. Q22' '^## 6\.' | hash_of)
h5b=$(sect "$last"  '^## 5\. Q22' '^## 6\.' | hash_of)
[ "$h5a" != "$h5b" ] || fail "C2, the section-hash instrument reports no change anywhere, so 'unchanged' carries no information."
echo "  C2 passes: section 5 differs between the first and last commit, so the instrument can see a change."
echo
LASTBLIND=a664fffb   # 241's stage 3, the last commit before its reconciliation commit
echo "  whole pre-reconciliation body (everything above the reconciliation rule), $LASTBLIND against HEAD-of-file:"
# everything above the reconciliation heading, with the trailing separator rule
# and blank lines stripped, since a rule appended below the body is not an edit
# to the body. Both forms are reported.
body_raw()  { git show "$1:$F" | awk '/^# Reconciliation, written after the blind phase/{exit} {print}'; }
body() { body_raw "$1" | awk '{a[NR]=$0} END{n=NR; while (n>0 && (a[n]=="" || a[n]=="---")) n--; for(i=1;i<=n;i++) print a[i]}'; }
ra=$(body_raw "$LASTBLIND" | hash_of); rb=$(body_raw "$last" | hash_of)
ba=$(body "$LASTBLIND" | hash_of);     bb=$(body "$last" | hash_of)
printf '    raw, including any appended separator: %s : %s / %s : %s\n' "$LASTBLIND" "$ra" "$last" "$rb"
printf '    separator rule and trailing blanks stripped: %s : %s / %s : %s\n' "$LASTBLIND" "$ba" "$last" "$bb"
if [ "$ra" != "$rb" ]; then
  echo "    the raw difference, in full:"
  diff <(body_raw "$LASTBLIND") <(body_raw "$last") | sed 's/^/      /'
fi
if [ "$ba" = "$bb" ]; then
  echo "    IDENTICAL. 241 added the reconciliation and the erratum and edited nothing above them."
else
  echo "    DIFFERS. The blind body was edited after the reading began; see the diff."
  diff <(body "$LASTBLIND") <(body "$last") | head -40
fi
echo

echo "=== 3. what 241 says it read, and whether it reaches the proposals' provenance ==="
echo "  241's own reading list, from its reconciliation:"
awk '/^What I read:/{f=1} f{print "    "$0} /^## What I would not change/{exit}' "$F" | head -12
echo
echo "  the provenance the two proposals name:"
for id in admission_returns_a_coordinate_rather_than_a_verdict membership_and_hosting_are_two_questions; do
  echo "    $id:"
  awk -v want="id = \"$id\"" 'BEGIN{RS="\\[\\[proposal\\]\\]"} index($0, want){print}' mock/registry/proposal.toml \
    | awk '/^provenance = \[/,/^\]/' | sed 's/^/      /'
done
echo
echo "  73's line 667 and 705, which are the two provenance targets inside 73:"
sed -n '667p;705p' "$f73" | cut -c1-100 | sed 's/^/    /'
echo
echo "  does 241 cite 73 by section, and which section:"
BT=$(printf '\140')
grep -oE "${BT}73[^${BT}]*${BT}[^.]{0,40}" "$F" | sed 's/^/    /'
# C3: positive and planted controls on the citation grep.
grep -q 'section 7' "$F" || fail "C3, the grep cannot find 'section 7' in 241, which is present."
echo "  C3a passes: the grep finds text that is present in 241."
planted=$(mktemp -t s246cite); trap 'rm -f "$planted"' EXIT
{ cat "$F"; echo 'and 73 section 1 with its two-by-two, which I read.'; } > "$planted"
grep -q '73 section 1' "$planted" || fail "C3b, a planted citation of 73 section 1 was not found."
grep -q '73 section 1' "$F" && { echo "  NOTE: 241 DOES cite 73 section 1."; } || \
  echo "  C3b passes: a planted '73 section 1' is found in the planted copy and is absent from 241 itself."

echo
echo "=== 4. what backs the two sections of 73 the proposals come from ==="
# M1 rests on 73 section 1, M5 on 73 section 7, per 73's own "Rests on:" lines.
# If neither section cites an instrument, then neither proposal varies anything,
# and the dimension intersection with 241 is empty because there is no
# instrument on either side rather than because two sweeps disagreed.
echo "  73's own committed instruments:"
ls "$P/73_probes" | sed 's/^/    /'
echo
echo "  which section of 73 cites each of its own probes:"
awk '/^## /{sec=$0} /73_probes\//{print "    " sec}' "$f73" | sort -u
echo
for s in 1 7; do
  n=$(awk -v s="^## $s\\\\." '$0 ~ s {f=1; next} /^## /{f=0} f' "$f73" | grep -c '73_probes/')
  printf '  section %s cites its own probes %d times\n' "$s" "$n"
  [ "$n" -eq 0 ] || echo "    NOTE: section $s does cite an instrument, which changes the reading above."
done
# C4: the counter must be able to return nonzero, or a zero above means nothing.
n2=$(awk '/^## 2\./{f=1; next} /^## /{f=0} f' "$f73" | grep -c '73_probes/')
[ "$n2" -gt 0 ] || fail "C4, the per-section counter returns zero for section 2 as well, so its zeros say nothing."
echo "  C4 passes: the same counter returns $n2 for section 2, so a zero is a fact about the section."
echo
echo "  73's own 'Rests on' lines for M1 and M5:"
grep -n 'Rests on: section 1\|Rests on: section 7' "$f73" | sed 's/^/    /'
