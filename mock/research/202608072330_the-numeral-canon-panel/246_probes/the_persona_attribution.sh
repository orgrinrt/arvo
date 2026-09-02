#!/usr/bin/env bash
# Seat 246. Two persona questions, one about 241's own accounting and one about
# mine, because both change what may be counted as an instance.
#
# 1. 241 declines to count 08 as a second instance for its Q22 compositional
#    route, on the stated ground that "It is an earlier Kiselyov seat, so this
#    is one persona agreeing with itself and I do not count it." 244 carries the
#    same sentence forward in its C3. If 08 is not a Kiselyov seat, then 241
#    declined an instance it was entitled to count, and 244 and 245 inherited a
#    false persona attribution. An UNDER-count is the direction nobody audits.
#
# 2. I am seat 246 and I am a Kiselyov seat. 241 is a Kiselyov seat. Whatever I
#    conclude in agreement with 241 is one persona agreeing with itself by the
#    same standard 241 applied to itself, so this instrument records the
#    constraint rather than leaving it to be discovered.
#
# THE CASES THAT MUST FAIL, planted and shown before the verdict:
#   C1 the persona extractor must return kiselyov for a file known to be one.
#   C2 it must return something OTHER than kiselyov for a file known not to be,
#      or an extractor that answers kiselyov to everything passes C1 and says
#      nothing.
#   C3 the quoted sentence must be found in 241, or the claim that 241 says it
#      is unsupported. A planted variant must also be found, so a hit is a fact
#      about the file rather than about a pattern that matches anything.

set -u
cd "$(dirname "$0")/.." || exit 1   # the panel directory
fail() { echo "CONTROL FAILED: $1"; exit 2; }

# The panel's convention is <number>_<persona>_<slug>.md. Files that do not
# carry a persona in that position are reported as such rather than guessed at.
# BSD sed has no portable `t` branch here, so this is awk. The first attempt at
# this line used `sed -E '...; t; ...'`, which BSD sed rejects with "undefined
# label"; C1 below caught it, which is the whole reason the control exists.
persona() { echo "$1" | awk '{ if (match($0, /^[0-9]+_[a-z_]+_/)) { s=substr($0,1,RSTART+RLENGTH-2); sub(/^[0-9]+_/,"",s); sub(/_[a-z_]*$/,"",s); print s } else print "(no persona in the filename)" }'; }

echo "=== controls on the extractor ==="
k=$(persona 241_kiselyov_admission_is_a_resolution_not_a_verdict.md)
[ "$k" = kiselyov ] || fail "C1, the extractor returned '$k' for a kiselyov file."
echo "  C1 passes: 241 -> $k"
n=$(persona 08_knuth_what_the_one_format_concept_covers.md)
[ "$n" != kiselyov ] || fail "C2, the extractor returned kiselyov for 08 as well, so it answers kiselyov to everything."
echo "  C2 passes: 08 -> $n, which is not kiselyov, so the extractor distinguishes"
echo

echo "=== 1. what 241 says about 08, quoted from 241 ==="
F=241_kiselyov_admission_is_a_resolution_not_a_verdict.md
grep -n 'earlier Kiselyov seat' "$F" | sed 's/^/  /' || fail "C3, the sentence is not in 241."
echo "  in context:"
grep -n -B3 -A2 'earlier Kiselyov seat' "$F" | sed 's/^/    /'
echo
planted=$(mktemp -t s246p); trap 'rm -f "$planted"' EXIT
{ cat "$F"; echo 'It is an earlier Lamport seat.'; } > "$planted"
grep -q 'earlier Lamport seat' "$planted" || fail "C3b, a planted variant was not found."
grep -q 'earlier Lamport seat' "$F" && fail "C3b, the planted variant is somehow already in 241."
echo "  C3 passes: the sentence is present in 241, a planted variant is found in the planted copy only."
echo
echo "  every kiselyov-named file in this panel:"
ls | grep -E '^[0-9]+_kiselyov_' | sed 's/^/    /'
echo "  every knuth-named file in this panel:"
ls | grep -E '^[0-9]+_knuth_' | sed 's/^/    /'
echo
echo "  the handle table's row for 08, which records the seat independently of the filename:"
grep -n '08_knuth' HANDLES.md | sed 's/^/    /'
echo
echo "  08's own header, first six lines:"
head -6 08_knuth_what_the_one_format_concept_covers.md | sed 's/^/    /'
echo
echo "  VERDICT: 08 is a knuth seat. 241's ground for declining it is false, so"
echo "  241 declined an independent prior instance for its Q22 compositional"
echo "  route. 244 C3 repeats the attribution and 245 did not check it."
echo "  The likely source of the confusion is visible in 08's own reading list,"
echo "  which names a kiselyov file:"
grep -n '06_kiselyov' 08_knuth_what_the_one_format_concept_covers.md | head -2 | sed 's/^/    /'
echo
echo "  the passage of 08 that 241 says is its route almost verbatim:"
awk '/^#+ 4\.5/{f=1} f{print} /^#+ 4\.6/{if(f) exit}' 08_knuth_what_the_one_format_concept_covers.md \
  | grep -n 'pair of numerals' | sed 's/^/    /'
echo

echo "=== 2. my own seat, and what it forbids me from counting ==="
echo "  this file's seat number : 246"
echo "  this seat's persona     : kiselyov"
echo "  241's persona           : kiselyov"
echo "  So by 241's own standard, quoted above, any conclusion of mine that"
echo "  AGREES with 241 is one persona agreeing with itself and is not a second"
echo "  instance. What is unaffected: measurements, source reads, date and hash"
echo "  comparisons, and refutations, none of which is an agreement."
