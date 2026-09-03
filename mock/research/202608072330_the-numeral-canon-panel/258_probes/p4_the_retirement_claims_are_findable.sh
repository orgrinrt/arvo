#!/usr/bin/env bash
# Seat 258. Every claim seat 258 retires is findable in the corpus it was written in.
#
# `a-retirement-claim-can-be-found` refuses a `claim` shorter than five words on
# the ground that it cannot pin the sentence it retires. The schema asks for the
# sentence itself, "or close enough that a grep finds it". This runs that search
# for each claim before the row is written, so a row is never written against a
# sentence that is not there.
#
# Each entry is the distinguishing fragment of the claim rather than the whole
# `claim` field, because a `claim` field carries the sentence plus enough context
# to read it and the fragment is what has to match.
#
# **Whitespace is normalised per file and nothing else is.** The corpus is prose
# wrapped to a column, so a sentence-length fragment spans a line break in most
# files that carry it, and a fixed-string `grep` over the raw file reports it
# absent. That is not a hypothetical: this probe's own C2 failed on its first run
# for exactly that reason, naming 247 and not 245 for a sentence 245 wrote, and
# 248's p5 disclosed the same defect against itself on the same corpus. Three
# arrivals at one instrument defect, and it is the reason for the `norm` below.
#
# THE CASES THAT MUST FAIL, run before the census is reported:
#   C1  A planted fragment that is in no file must come back zero. Without it a
#       matcher that finds everything reports every claim findable.
#   C2  A fragment known to be in a named file, and known to span a line break
#       there, must come back naming that file. This is the control that caught
#       the wrap.
#   C2b The same fragment under the unnormalised search must NOT name that file,
#       or the normalisation is not what recovered it and C2 proves nothing.
#   C3  Every claim must come back nonzero, and the script exits non-zero if one
#       does not, so the row set cannot be written against a missing sentence.
set -u
cd "$(dirname "$0")/.." || exit 1 # the panel directory
fail() {
	echo "CONTROL FAILED: $1"
	exit 2
}

# Fixed-string search over each file with newlines and runs of blanks collapsed.
find_in_corpus() {
	local frag=$1 f
	for f in ./*.md; do
		if tr '\n' ' ' <"$f" | tr -s ' ' | grep -qF "$frag"; then printf '%s\n' "${f#./}"; fi
	done
}
find_raw() { grep -rlF "$1" . --include='*.md' 2>/dev/null | sed 's#^\./##' | sort; }

# --- C1 ------------------------------------------------------------------------
n=$(find_in_corpus "zzq the arity is fixed at eleven by a planted control zzq" | wc -l | tr -d ' ')
[ "$n" -eq 0 ] || fail "C1: a planted absent fragment was found in $n files"
echo "C1 planted absent fragment found in            : 0 files"

# --- C2 and C2b ----------------------------------------------------------------
WRAPPED="none of the growth touches the admission topic"
hit=$(find_in_corpus "$WRAPPED")
case "$hit" in
*245_ringer_entailment_check_on_the_admission_consolidation.md*) ;;
*) fail "C2: the known fragment did not name 245 under normalisation, it named: $hit" ;;
esac
echo "C2 known wrapped fragment reaches 245          : yes"
raw=$(find_raw "$WRAPPED")
case "$raw" in
*245_ringer_entailment_check_on_the_admission_consolidation.md*) fail "C2b: the raw search also found it, so normalisation is not the separator" ;;
*) ;;
esac
echo "C2b the same fragment, unnormalised            : does not reach 245"
echo "    (raw search names: ${raw:-nothing})"
echo

# --- the claims ----------------------------------------------------------------
bad=0
check() {
	local label=$1 frag=$2
	local files
	files=$(find_in_corpus "$frag")
	if [ -z "$files" ]; then
		echo "MISSING  $label"
		echo "         fragment: $frag"
		bad=$((bad + 1))
	else
		echo "found    $label"
		printf '         %s\n' $files
	fi
}

check "R1 the arity is fixed by a ratified count" \
	"the arity is fixed at one by a ratified count"
check "R2 the bound-carrying question row count of 24" \
	"24 of the registry's 105 question"
check "R3 the erratum's own diagnosis of its miscount" \
	"fields sitting where I read \`bound\` to be"
check "R4 neither reading gives 6" \
	"Neither reading gives \`6\`"
check "R5 three of four identifies the counting convention" \
	"Three of four reproduce exactly under plain substring counting"
check "R6 the sitting's one promotable result" \
	"the sitting's one promotable result is a promotion nobody proposed"
check "R7 245's gate sentence about the growth" \
	"none of the growth touches the admission topic"
check "R8 241 as a counter-instance in its own vocabulary" \
	"is a counter-instance in its own vocabulary"
check "R9 08 declined as the same persona" \
	"one persona agreeing with itself and I do not count it"
check "R10 the admission subject waits on the level cut" \
	"does not close until the level cut does"
check "R11 the state file's attribution to seat 242" \
	"is not to be merged with"
check "R12 256's owed list" \
	"has no second reader and its author says it is the load-bearing new argument"

echo
if [ "$bad" -gt 0 ]; then
	echo "VERDICT: $bad claim(s) not findable. Those rows may not be written."
	exit 1
fi
echo "VERDICT: every claim is findable in the panel corpus under normalisation."
