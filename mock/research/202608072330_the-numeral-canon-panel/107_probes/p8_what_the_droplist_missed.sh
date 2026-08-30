#!/usr/bin/env bash
# p8: the four droplist omissions, each verified by hand rather than by the
# grep in p7, because a grep can miss a paraphrase and a false severe finding
# is worse than none. For each: quote the member text, then show 106's own
# text at the place it would have to appear.
set -u
cd "$(dirname "$0")/.." || exit 1
T=106_giesen_consolidation_the_strategy_axis.md

hr() { printf '\n----------------------------------------------------------------\n'; }

hr; echo "OMISSION 1. 97's F-H: a declared non-negative operand window RECOVERS"
echo "three laws that two-sided signed saturation loses. Licence-shaped, one file."
grep -n -A6 '^\*\*F-H\.' 97_dolan_the_strategy_space_attacked.md
echo
echo "-- 106, every occurrence of 'window' --"
grep -cin 'window' "$T"; echo "   (zero means absent)"

hr; echo "OMISSION 2. 97's F-B: the gap is polynomial against exponential in the"
echo "region count, which is what makes 72-of-15625 a fact about weightings"
echo "rather than about one table. 98 calls it 'a better argument than mine'."
grep -n -A5 '^\*\*F-B\.' 97_dolan_the_strategy_space_attacked.md
echo
echo "-- and 98's own bound on the counts without it --"
grep -n -A3 '^\*\*F-98-5\.' 98_spj_what_the_strategy_axis_settles.md
echo
echo "-- 106: 'polynomial', 'hyperplane', '47x', 'fact about one table' --"
for w in polynomial hyperplane '47x' 'fact about one table'; do printf "   %-22s %s\n" "$w" "$(grep -ci "$w" "$T")"; done

hr; echo "OMISSION 3. The exchange-rate-not-priority reading of op's FOUR intents."
echo "Three independent instances: 40 section 5.3, then 98 section 4.1 with a"
echo "probe, then 102 from the intents' text. It constrains what 106 section 1"
echo "means by 'a weighting', and it carries a consequence 98 said to state."
sed -n '499,536p' 98_spj_what_the_strategy_axis_settles.md
echo
grep -n -A4 '^\*\*F-98-7\.' 98_spj_what_the_strategy_axis_settles.md
echo
echo "-- 106: 'lexicographic', 'priority', 'threshold', 'hard bound' --"
for w in lexicographic priority threshold 'hard bound' 'exchange rate'; do printf "   %-22s %s\n" "$w" "$(grep -ci "$w" "$T")"; done

hr; echo "OMISSION 4. 97's law criterion is cited by SCORE and never stated."
echo "97 calls it 'the one I would most like carried'."
grep -n -B1 -A8 '^> a law holds in the representable set' 97_dolan_the_strategy_space_attacked.md
echo
echo "-- 106: 'realisation map', 'identity of exact', 'ordered nesting', 'congruence', 'quotient' --"
for w in 'realisation map' 'identity of exact' 'ordered nesting' congruence quotient; do printf "   %-22s %s\n" "$w" "$(grep -ci "$w" "$T")"; done
echo "-- what 106 DOES say about it --"
grep -n "criterion predicts every verdict" "$T"

hr; echo "AND THE CONSEQUENCE 106'S OWN AMENDMENT INTRODUCES AND DOES NOT TRACE"
echo "106 section 9.2 widens 'observable' to cover whether a value is produced."
echo "98 section 3.2, citing 40 section 5.3, says a build arm may move an"
echo "unobservable coordinate and is FORBIDDEN to move an observable one."
echo "I18's panic is moved by the build profile."
sed -n '479,484p' 98_spj_what_the_strategy_axis_settles.md
echo
echo "-- and the seam this lands on, which 102 already named as unnoticed --"
grep -n -A3 'A build condition is a .cfg.' 102_torvalds_does_the_mechanism_serve_the_intents.md
