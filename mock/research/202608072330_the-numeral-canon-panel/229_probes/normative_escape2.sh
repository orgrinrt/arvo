#!/usr/bin/env bash
# Arm 2, after arm 1's control fired.
#
# `normative_escape.sh` asked for rows filed under a region-free kind whose
# `because` names instruments, and its N2 control required the one row known to
# be in the target class. **N2 FAILED**, and the failure is kept in
# `normative_escape_first_attempt.out` rather than repaired away, because the
# arm was not wrong about its own question: that row's `because` genuinely names
# no instrument, it names a coordinator's derivation from the topic rows. So arm
# 1 measures a real but DIFFERENT set, relabelled below, and this arm measures
# the class.
#
# The class: a row whose own prose says its region could not be written, so it
# was filed under a kind that owes none. That is a self-report and is therefore
# a floor rather than a census: a row that made the same move without saying so
# is invisible to this and to everything else.
#
# Controls, outcomes written before the run:
#   M1  `the_topics_form_a_stack_a_frame_and_the_canons_own_machinery` must be
#       found, since its note says it in terms.
#   M2  a phrase nobody wrote must find nothing.
#   M3  the arm must find fewer rows than arm 1's 21, or it is not narrower and
#       the two are one instrument printed twice.
set -euo pipefail
cd "$(dirname "$0")"
REG=../../../registry

pat='no predicate, because|none in this registry can express|no way to say where it holds|no dimension|cannot express a region|no axis|filed .normative. after being written|owes no region because'

echo "### rows whose own prose says the region could not be written"
grep -n -E "$pat" "$REG"/*.toml | sed -E 's/^([^:]+):([0-9]+):.*/\1:\2/' | sort -u > hits.txt
while IFS=: read -r f l; do
  id=$(awk -v ln="$l" 'NR<=ln && /^id = /{last=$0} END{sub(/^id = "/,"",last); sub(/"$/,"",last); print last}' "$f")
  printf '  %-28s :%s  %s\n' "$(basename "$f")" "$l" "$id"
done < hits.txt
n=$(wc -l < hits.txt | tr -d ' ')
printf '  --- count: %s\n\n' "$n"

echo "### M1"
grep -q 'the_topics_form_a_stack' <(while IFS=: read -r f l; do awk -v ln="$l" 'NR<=ln && /^id = /{last=$0} END{print last}' "$f"; done < hits.txt) \
  && echo "  PASS" || echo "  FAIL"
echo "### M2"
grep -qE 'phase_of_the_moon' "$REG"/*.toml && echo "  FAIL" || echo "  PASS, nobody wrote it"
echo "### M3, narrower than arm 1"
[ "$n" -lt 21 ] && echo "  PASS, $n < 21" || echo "  FAIL, $n >= 21, the two arms are one"
