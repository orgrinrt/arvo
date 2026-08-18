#!/bin/sh
# P3. Did every option of mine survive, get closed with a diagnostic, or get dropped?
#
# 167 opened Q-C1..Q-C7. 171 opened O-171-1..O-171-4. The candidate's section 7
# carries eight live options and section 3 carries what was closed or retired.
# A dropped option is the failure RULES.md says a consolidation makes
# structurally, so this searches for each by name across the whole candidate.
#
# CASES THAT MUST FAIL
#   C-A  at least one of mine must be FOUND, else the pattern is broken
#   C-B  at least one must be found in section 3 rather than section 7, else
#        "closed with a diagnostic" is not being distinguished from "carried"
#   C-C  a name that does not exist must return zero, so a hit means something
cd "$(dirname "$0")/../.." || exit 1
F=173_leroy_the_canon_candidate_for_the_chain.md

flat() { tr '\n' ' ' < "$1" | sed 's/  */ /g'; }
BODY=$(flat $F)

echo "--- each option of mine, counted across the whole candidate ---"
for o in Q-C1 Q-C2 Q-C3 Q-C4 Q-C5 Q-C6 Q-C7 O-171-1 O-171-2 O-171-3 O-171-4 O-169-2; do
  n=$(printf '%s' "$BODY" | grep -o "$o" | wc -l | tr -d ' ')
  printf '  %-9s %s\n' "$o" "$n"
done

echo
echo "--- C-C: a name that does not exist ---"
printf '  %-9s %s\n' "Q-C99" "$(printf '%s' "$BODY" | grep -o 'Q-C99' | wc -l | tr -d ' ')"

echo
echo "--- where each found one sits: section 3 (closed) or section 7 (live) ---"
S3=$(sed -n '/^## 3\./,/^## 4\./p' $F | tr '\n' ' ')
S7=$(sed -n '/^## 7\./,/^## 8\./p' $F | tr '\n' ' ')
S5=$(sed -n '/^## 5\./,/^## 6\./p' $F | tr '\n' ' ')
for o in Q-C1 Q-C2 Q-C3 Q-C4 Q-C5 Q-C6 Q-C7 O-171-1 O-171-2 O-171-3 O-171-4; do
  a=$(printf '%s' "$S3" | grep -c "$o"); b=$(printf '%s' "$S7" | grep -c "$o"); c=$(printf '%s' "$S5" | grep -c "$o")
  printf '  %-9s closed(s3)=%s live(s7)=%s notsettled(s5)=%s\n' "$o" "$a" "$b" "$c"
done

echo
echo "--- the two I most want to find: O-171-1 and Q-C2, in context ---"
printf '%s' "$BODY" | grep -o '.\{190\}O-171-1.\{190\}' | head -2
echo
printf '%s' "$BODY" | grep -o '.\{160\}Q-C2.\{200\}' | head -2
