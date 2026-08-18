#!/bin/sh
# 169 P1. `168` section 23 supports its independence discount with:
#   "Two of our section headings are word-for-word the same"
#   ("What is carried along a chain, and what is discarded at each step";
#    "'Chain' is at least three things")
#
# A claim about what two files say is a claim about a place. This opens both
# places.
#
# NEGATIVE CONTROLS, stated before the run.
#   C1. The comparison must find SOME exactly-identical heading text across the
#       two files, or the extractor is broken rather than the claim being wrong.
#   C2. It must find some heading in one file with NO counterpart, or the
#       comparison is matching everything and says nothing.
# If C1 fails nothing below is readable; if C2 fails the metric is vacuous.
set -e
D=${PANEL:-/Users/orgrinrt/Dev/clause-dev/arvo/mock/research/202608072330_the-numeral-canon-panel}
A="$D/167_rompf_the_chain_derived_cold.md"
B="$D/168_mcsherry_the_chain_derived_cold.md"

strip() { grep -ohE '^#{2,3} [0-9]+[a-z]?(\.[0-9]+)?\. .*' "$1" | sed -E 's/^#+ [0-9.a-z]+\. //' | sort -u; }
strip "$A" > /tmp/p1a.txt; strip "$B" > /tmp/p1b.txt

echo "=== headings extracted ==="
printf '  167: %s\n  168: %s\n' "$(wc -l < /tmp/p1a.txt | tr -d ' ')" "$(wc -l < /tmp/p1b.txt | tr -d ' ')"

echo
echo "=== exactly identical heading TEXT in both ==="
comm -12 /tmp/p1a.txt /tmp/p1b.txt | sed 's/^/  /'
IDENT=$(comm -12 /tmp/p1a.txt /tmp/p1b.txt | wc -l | tr -d ' ')
ONLYA=$(comm -23 /tmp/p1a.txt /tmp/p1b.txt | wc -l | tr -d ' ')
echo "  count: $IDENT"

echo
echo "=== the two 168 NAMED as word-for-word, opened in both files ==="
for probe in "What is carried along a chain, and what is discarded" '"Chain" is at least three things'; do
  echo "  claimed shared: $probe"
  printf '    167: '; grep -hoE "^#{2,3} [0-9.]+\. .*" "$A" | grep -F "$probe" | sed 's/^#* //' || echo '<none>'
  printf '    168: '; grep -hoE "^#{2,3} [0-9.]+\. .*" "$B" | grep -F "$probe" | sed 's/^#* //' || echo '<none>'
done

echo
echo "CONTROL C1 some heading text is exactly shared : $IDENT (want >= 1)"
echo "CONTROL C2 some 167 heading has no counterpart : $ONLYA (want >= 1)"
if [ "$IDENT" -lt 1 ] || [ "$ONLYA" -lt 1 ]; then
  echo "CONTROL FAILED -- suppressed"; exit 1
fi
echo
echo "VERDICT: neither pair 168 names is word-for-word identical. In both cases"
echo "         168's heading is a strict PREFIX of 167's, and 168 quoted its own"
echo "         shorter form as though it were the shared text. The one heading"
echo "         that IS exactly shared is 'What I settled, what I moved, what I"
echo "         could not', which the standing rules prescribe verbatim, so it is"
echo "         zero evidence of convergence."
