#!/bin/sh
# what the committed registry holds today, for 186's phase two.
# counts are measurements, so each is produced by a command shown with it,
# and two controls that must come out zero run before the figures count.
set -eu
cd "$(dirname "$0")/../../../registry"

echo "### controls, both must be 0 or every figure below is void"
printf 'C1 nonexistent id                    %s\n' "$(cat ./*.toml | grep -c 'id = "this_id_does_not_exist_anywhere"' || true)"
printf 'C2 sentence_kind definition (today)  %s\n' "$(grep -c '^sentence_kind = "definition"' proposal-the-later-topics.toml || true)"

echo
echo "### the predicated surface committed today"
printf 'proposal rows                        %s\n' "$(grep -c '^\[\[proposal\]\]' proposal-the-later-topics.toml)"
printf 'proposal predicate fields            %s\n' "$(grep -c '^predicate' proposal-the-later-topics.toml)"
printf 'law rows                             %s\n' "$(grep -c '^\[\[law\]\]' law-the-later-topics.toml)"
printf 'law holds fields                     %s\n' "$(grep -c '^holds = ' law-the-later-topics.toml)"
printf 'law fails fields                     %s\n' "$(grep -c '^fails = ' law-the-later-topics.toml)"

echo
echo "### sentence_kind distribution"
grep '^sentence_kind' proposal-the-later-topics.toml | sort | uniq -c

echo
echo "### the committed enumeration row names no instrument (expect 0 evidence lines in its table)"
awk '/id = "where_fusion_changes_the_answer_it_is_not_a_lowering"/,/^\[\[/' proposal-the-later-topics.toml | grep -c '^evidence' || true

echo
echo "### the q4 withdrawal rows exist (expect 1 each)"
printf 'prior_calls_are_a_historical_log     %s\n' "$(grep -c 'id = "prior_calls_are_a_historical_log_not_calls"' ruling.toml)"
printf 'his_voice_is_demoted                 %s\n' "$(grep -c 'id = "his_voice_is_demoted_except_where_he_frames_it_absolute"' ruling.toml)"

echo
echo "### the q5 premise question exists with the call op's (expect 1 and 1)"
printf 'question the_container_premise       %s\n' "$(grep -c 'id = "the_container_premise"' question.toml)"
printf 'its decider = op                     %s\n' "$(awk '/id = "the_container_premise"/,/^\[\[/' question.toml | grep -c '^decider = "op"')"
