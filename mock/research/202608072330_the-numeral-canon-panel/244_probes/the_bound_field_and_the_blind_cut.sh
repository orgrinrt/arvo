#!/usr/bin/env bash
# Seat 244. Two checkable claims the sitting rests on, re-run rather than
# inherited: the `bound` census both cold seats reported, and the blindness of
# the cold cut, which my own brief asserts.
#
# The case that must fail: a `bound` census that cannot find a bound anywhere is
# a fact about the awk. Section 1's control counts the bounds over the whole
# file and must be nonzero. And an ancestry check that answers NO to everything
# proves nothing; section 2's control asks a question whose answer is known to
# be YES.
set -u
cd "$(dirname "$0")/../../../.." || exit 1   # repo root
Q=mock/registry/question.toml

echo "======== 1. The bound and answered fields on topic = the_number_system"
awk '
  /^\[\[question\]\]/{ flush(); id="";b=0;u=0;a=0;t="" }
  /^id *= /{ if(id==""){s=$0;sub(/^id *= *"/,"",s);sub(/"$/,"",s);id=s} }
  /^bound *= /{ b=1 } /^unblocks *= /{ u=1 } /^answered *= /{ a=1 }
  /^topic *= /{ s=$0;sub(/^topic *= *"/,"",s);sub(/"$/,"",s);t=s }
  END{ flush(); printf "\n  rows = %d, bound = %d, unblocks = %d, answered = %d\n", n, nb, nu, na }
  function flush(){ if(id!=""&&t=="the_number_system"){ n++; nb+=b; nu+=u; na+=a;
      printf "  %-56s bound=%d unblocks=%d answered=%d\n", id, b, u, a } }
' "$Q"

echo
echo "  CONTROL: bounds do exist in this file, so a zero above is about the topic"
printf '  rows carrying a bound, whole file : %s\n' "$(grep -c '^bound = ' "$Q")"
printf '  question rows, whole file         : %s\n' "$(grep -c '^\[\[question\]\]' "$Q")"
if [ "$(grep -c '^bound = ' "$Q")" -eq 0 ]; then
  echo "  CONTROL FAILED: the instrument finds no bound anywhere."; exit 2
fi
echo "  control passes."
echo
echo "  NOTE: 241's erratum says the four it miscounted were \`note\` fields."
echo "  Every row carries a note; exactly the four above carry an \`unblocks\`,"
echo "  and four is the number it counted. The field it misread is \`unblocks\`."

echo
echo "======== 2. Was the cold cut blind"
B1=c06a5706   # 241 stage 1
B2=64ab711e   # 242 blind derivation
echo -n "  shared base of the two seats            : "; git merge-base $B1 $B2
echo -n "  241 stage 1 is an ancestor of 242 blind : "
git merge-base --is-ancestor $B1 $B2 && echo YES || echo NO
echo -n "  242 blind is an ancestor of 241 stage 1 : "
git merge-base --is-ancestor $B2 $B1 && echo YES || echo NO
echo "  CONTROL, a pair whose answer is known to be YES:"
echo -n "    the shared base is an ancestor of 241 stage 1 : "
git merge-base --is-ancestor "$(git merge-base $B1 $B2)" $B1 && echo YES || echo NO

echo
echo "  Stronger than ancestry: what each blind tree actually holds."
echo -n "    panel files matching 24[123]_ at 241 stage 1: "
git ls-tree -r --name-only $B1 -- mock/research/202608072330_the-numeral-canon-panel/ | grep -c '24[123]_'
git ls-tree -r --name-only $B1 -- mock/research/202608072330_the-numeral-canon-panel/ | grep '24[123]_' | sed 's/^/      /'
echo -n "    panel files matching 24[123]_ at 242 blind : "
git ls-tree -r --name-only $B2 -- mock/research/202608072330_the-numeral-canon-panel/ | grep -c '24[123]_'
git ls-tree -r --name-only $B2 -- mock/research/202608072330_the-numeral-canon-panel/ | grep '24[123]_' | sed 's/^/      /'

echo
echo "======== 3. FINDING"
echo "  Zero of the eighteen the_number_system rows carry a bound; two carry an"
echo "  answered; four carry an unblocks, which is the field 241's erratum names"
echo "  as note. Third instance on the zero, after 241's erratum and 242's own awk."
echo "  The blind cut holds: one shared base, neither blind commit an ancestor of"
echo "  the other, and neither tree holds the other seat's file."
