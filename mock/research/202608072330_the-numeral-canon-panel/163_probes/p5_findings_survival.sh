#!/bin/bash
# 163 P5. Which of 157's thirteen findings and five register questions survive into 161,
# and in what form. A finding that survives ONLY inside an option or a retirement dies
# with that option, which is 162 section 7's shape applied to my own material.
#
# NEGATIVE CONTROL: a finding known to be carried prominently (F157-6, the certificate)
# must be found. If the search reports it missing, the search is wrong, not the candidate.
cd /Users/orgrinrt/Dev/clause-dev/arvo/mock/research/202608072330_the-numeral-canon-panel
C=161_leroy_the_canon_candidate_for_the_primitive.md
for f in F157-1 F157-2 F157-3 F157-4 F157-5 F157-6 F157-7 F157-8 F157-9 F157-10 F157-11 F157-12 F157-13 \
         Q157-A Q157-B Q157-C Q157-D Q157-E; do
  n=$(grep -c -- "$f" "$C")
  if [ "$n" = "0" ]; then printf '  %-8s NOT NAMED\n' "$f"
  else printf '  %-8s named %s time(s):  %s\n' "$f" "$n" "$(grep -o -- "$f[^.]*\." "$C" | head -1 | cut -c1-88)"; fi
done
echo
echo "CONTROL: F157-6 must be found ->" $(grep -c -- 'F157-6' "$C")
echo
echo "and the S-numbers, which is how the replacements were addressed:"
for s in S-1 S-5 S-6 S-8 S-10 S-11 S-12 S-13 S-14 S-15 S-16 S-17 S-18 S-19 S-20 S-21; do
  n=$(grep -cE -- "\`?$s\b" "$C")
  printf '  %-5s %s\n' "$s" "$n"
done
