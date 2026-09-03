#!/usr/bin/env bash
# Seat 258. Were 248 and 249 blind of each other, under the panel's own test?
#
# Both files answer 247's O1 and both say they were written blind. 249 says "There
# was no 248 file on disk when I read". 248 says it read 244 through 247 and does
# not mention 249. Neither can check the other's claim from inside itself.
#
# The panel's own certified blindness instrument is mutual non-ancestry of the two
# add-commits, which is what R3's `promotion` used and what 248's own p6 ran over
# the earlier cold pairs. This runs it on the pair 248 and 249 form, which nobody
# has run, because 248 predates 249's file and 249 could not run it on a file that
# was not there.
#
# It also prints what each blind commit added, because non-ancestry says the two
# branches did not see each other and says nothing about what was in either.
#
# THE CASES THAT MUST FAIL, run before the verdict is reported:
#   C1  A pair whose answer is known YES must come back YES: each blind commit is
#       an ancestor of HEAD.
#   C2  The same comparison reversed must come back NO: HEAD is not an ancestor of
#       either blind commit. Without it an always-true predicate passes C1.
#   C3  Each named commit must resolve and must be the commit whose message the
#       file claims, or the hashes are being taken on trust.
set -u
cd "$(dirname "$0")/../../../.." || exit 1 # the repository root
fail() {
	echo "CONTROL FAILED: $1"
	exit 2
}

A=c9afdeaa # 248's blind commit, per 248 section 12
B=cbcdce5b # 249's blind commit, per 249 section 11

echo "tree: $(git rev-parse HEAD)"
echo

# --- C3: the hashes resolve and carry the messages the files claim -------------
for c in $A $B; do
	git rev-parse --verify "$c^{commit}" >/dev/null 2>&1 || fail "C3: $c does not resolve"
done
msgA=$(git log -1 --format=%s $A)
msgB=$(git log -1 --format=%s $B)
case "$msgA" in *"seat 248"*) ;; *) fail "C3: $A is not seat 248's commit, subject is: $msgA" ;; esac
case "$msgB" in *"seat 249"*) ;; *) fail "C3: $B is not seat 249's commit, subject is: $msgB" ;; esac
echo "C3 $A : $msgA"
echo "C3 $B : $msgB"
echo

# --- C1 and C2 -----------------------------------------------------------------
git merge-base --is-ancestor $A HEAD || fail "C1: $A is not an ancestor of HEAD"
git merge-base --is-ancestor $B HEAD || fail "C1: $B is not an ancestor of HEAD"
echo "C1 both blind commits are ancestors of HEAD    : yes"
if git merge-base --is-ancestor HEAD $A || git merge-base --is-ancestor HEAD $B; then
	fail "C2: HEAD came back as an ancestor of a blind commit, so the predicate is always true"
fi
echo "C2 HEAD is an ancestor of neither              : yes"
echo

# --- the measurement -----------------------------------------------------------
ab=NO
ba=NO
git merge-base --is-ancestor $A $B && ab=YES
git merge-base --is-ancestor $B $A && ba=YES
echo "248's blind commit is an ancestor of 249's     : $ab"
echo "249's blind commit is an ancestor of 248's     : $ba"
if [ "$ab" = NO ] && [ "$ba" = NO ]; then
	echo "VERDICT: mutually non-ancestral, which is R3's own blindness test passing."
else
	echo "VERDICT: one saw the other's branch; the pair is not blind under R3's test."
fi
echo
echo "merge base of the two:"
git merge-base $A $B | head -1
echo
echo "what 248's blind commit added:"
git show --name-only --format= $A
echo "what 249's blind commit added:"
git show --name-only --format= $B
echo
echo "NOTE: non-ancestry establishes that neither branch could show the other's file."
echo "It establishes nothing about what either seat's context held, which is the same"
echo "bound 248's own p6 states about itself."
