#!/usr/bin/env nutshell
# p4. How often a question row and a member file land in one commit.
#
# The schema in `mockspace.toml` puts `two_experts` at "each deriving before
# reading the other". A row written in the same act as the file that argues for
# it cannot exhibit that ordering for anything dispatched on the row afterwards:
# the row was in the tree first by construction, carrying whatever the file said.
#
# This counts the commits where that shape occurs, so the case the panel is
# arguing about can be seen as a class rather than as one incident.
#
# THE CASES THAT MUST FAIL, STATED BEFORE THE RUN.
#
# C1. The count of commits touching `question.toml` must be non-zero and must be
#     smaller than the count over the whole panel directory. A filter matching
#     everything reports a large number and means nothing.
# C2. A commit that touches only registry files must classify as NOT co-landing.
#     The very first commit that created question.toml is the natural candidate;
#     if the classifier says yes for a commit with no member file in it, the
#     `grep -qE` for a numbered file is matching the registry path.
# C3. The numbered-member-file pattern must not match a probe artifact or a
#     directory. Asserted directly against two known strings.
use log

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT" || exit 1
P="mock/research/202608072330_the-numeral-canon-panel"
MEMBER="^$P/[0-9]+_[a-z][a-z0-9_]*\.md$"

echo "== C3 control: the member-file pattern, on strings whose answer is known =="
c3=0
for good in "$P/65_knuth_number_systems_derived_cold.md" "$P/09_persona_checkpoint.md"; do
  if echo "$good" | grep -qE "$MEMBER"; then echo "  PASS matches a member file: $(basename "$good")";
  else echo "  FAIL should have matched: $good"; c3=1; fi
done
for bad in "$P/240_probes/q1_output.txt" "mock/registry/question.toml" "$P/OPTIONS.md"; do
  if echo "$bad" | grep -qE "$MEMBER"; then echo "  FAIL should not have matched: $bad"; c3=1;
  else echo "  PASS rejects: $(basename "$bad")"; fi
done
echo

tot=0; co=0; nomember=0
for c in $(git log --format=%H -- mock/registry/question.toml); do
  tot=$((tot+1))
  files=$(git show --name-only --format= "$c")
  if echo "$files" | grep -qE "$MEMBER"; then co=$((co+1)); else nomember=$((nomember+1)); fi
done
allpanel=$(git log --format=%H -- "$P" | wc -l | tr -d ' ')

echo "== the counts =="
echo "  commits touching mock/registry/question.toml:            $tot"
echo "  of those, also landing a numbered member file:           $co"
echo "  of those, landing no member file:                        $nomember"
echo "  commits touching the panel directory at all:             $allpanel"
echo

echo "== C1 control: the filter discriminates =="
if [ "$tot" -gt 0 ] && [ "$tot" -lt "$allpanel" ]; then
  echo "  PASS $tot is non-zero and well under $allpanel"; c1=0
else
  echo "  FAIL the question.toml filter is matching everything or nothing"; c1=1
fi
echo
echo "== C2 control: a registry-only commit does not classify as co-landing =="
if [ "$nomember" -gt 0 ]; then
  echo "  PASS $nomember commits touched question.toml with no member file, so the"
  echo "       classifier is not simply matching the registry path"; c2=0
else
  echo "  FAIL every commit classified as co-landing; the pattern is too loose"; c2=1
fi
echo
echo "== what this establishes =="
echo "  A question row landing in the same commit as a member file is a shape the"
echo "  corpus contains $co times, not once. Wherever a later seat is dispatched on"
echo "  such a row, the ordering the schema asks for ('each deriving before reading"
echo "  the other') cannot hold for that seat: the row, carrying the earlier file's"
echo "  conclusion, was in the tree before the seat opened anything."
exit $(( c1 + c2 + c3 ))
