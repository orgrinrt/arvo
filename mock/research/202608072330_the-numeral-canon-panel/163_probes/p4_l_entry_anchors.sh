#!/bin/bash
# 163 P4. For each L-entry that compresses 157, does the named anchor hold the claim?
# 161 section 8 defers this check to the round after it. This runs it for my own entries.
#
# NEGATIVE CONTROL: a deliberately wrong anchor is included and must show text that does
# not support the claim. If every anchor "supports" whatever it is asked about, the check
# is reading confirmation into whatever it finds.
cd /Users/orgrinrt/Dev/clause-dev/arvo/mock/research/202608072330_the-numeral-canon-panel
show () { echo "--- $1  ($2)"; f=$(ls ${1%%:*}_*.md 2>/dev/null | head -1); r=${1##*:}; a=${r%%-*}; b=${r##*-};
  if [ -z "$f" ]; then echo "    UNRESOLVED"; else sed -n "${a},${b}p" "$f" | sed 's/^/    /'; fi; echo; }
echo "=== anchors 161 names for entries compressing 157 ==="
show "157:358-362" "L23 cites this as S-8's synthesis"
show "157:695-701" "L19/R10 cite this as S-14's completeness clause"
show "111:531-535" "L16 cites this for the two halves"
show "111:552-553" "L16 cites this for 'nobody's yet'"
show "112:934-937" "L16 cites this as the classification rule"
show "111:555-556" "R16 retires this sentence"
show "109:649-651" "L20 cites this as the target-independence clause"
show "111:1175-1176" "L11 cites this for r1's figures"
show "82:770-774"   "L10 cites this, the one NOVEL anchor"
echo "=== CONTROL: an anchor that should NOT support the claim asked of it ==="
show "157:35-36" "CONTROL: asked whether it states the two-branch certificate (it does not)"
