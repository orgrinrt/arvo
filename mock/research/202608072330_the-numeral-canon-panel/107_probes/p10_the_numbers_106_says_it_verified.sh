#!/usr/bin/env bash
# p10: 106 section 16 names exactly which numbers it verified itself rather
# than taking from a file. Those are the ones a check should redo, because a
# file that says "I verified this" is making a claim like any other.
set -u
B=/Users/orgrinrt/Dev/clause-dev/arvo/mock/benches
cd "$B" || exit 1
V="--exclude-dir=target"

p() { printf "  %-34s claimed=%-6s measured=%-6s %s\n" "$1" "$2" "$3" "$([ "$2" = "$3" ] && echo OK || echo MISMATCH)"; }

echo "=== the corpus counts 106 claims to have taken itself ==="
p "variant crates"            94   "$(ls -d variants/*/ | wc -l | tr -d ' ')"
p "crates impl score_output"   0   "$(grep -rl 'fn score_output' variants/ $V | wc -l | tr -d ' ')"
p "crates impl score_dimensions" 0 "$(grep -rl 'fn score_dimensions' variants/ $V | wc -l | tr -d ' ')"
p "crates impl max_relative_error" 0 "$(grep -rl 'fn max_relative_error' variants/ $V | wc -l | tr -d ' ')"
p "crates defining validate_output" 15 "$(grep -rl 'fn validate_output' variants/ $V | cut -d/ -f2 | sort -u | wc -l | tr -d ' ')"
p "crates mentioning outputs_may_differ" 1 "$(grep -rl 'outputs_may_differ' variants/ $V | cut -d/ -f2 | sort -u | wc -l | tr -d ' ')"
p "committed CSVs"           254   "$(ls *.csv 2>/dev/null | wc -l | tr -d ' ')"
p "committed meta files"     254   "$(ls *.meta.json 2>/dev/null | wc -l | tr -d ' ')"

echo
echo "=== the satfold-const-gate numbers 106 cites for 'the selection erases' ==="
F=satfold-const-gate_n10000_findings.md
if [ -f "$F" ]; then
  grep -E 'gate-true|lanes16|gate-false|1438|1456|38391' "$F" | head -12
else
  echo "  MISSING: $F does not exist"
fi

echo
echo "=== 106 section 16 also claims 'the existence of twenty cited probe files"
echo "    across six probe directories'. Counted from 106's own text: ==="
cd /Users/orgrinrt/Dev/clause-dev/arvo/mock/research/202608072330_the-numeral-canon-panel/
T=106_giesen_consolidation_the_strategy_axis.md
A=$(grep -n '^## 15\. Anchor accounting' "$T" | cut -d: -f1)
Bn=$(grep -n '^## 16\.' "$T" | cut -d: -f1)
sed "${A},$((Bn-1))d" "$T" | grep -oE '[0-9]+_probes/[A-Za-z0-9_.]+' | sort -u > /tmp/107_cited_probes.txt
echo -n "  distinct probe files cited in 106's body : "; wc -l < /tmp/107_cited_probes.txt
echo -n "  distinct probe directories               : "; cut -d/ -f1 /tmp/107_cited_probes.txt | sort -u | wc -l
echo "  -- do they all exist? --"
miss=0
while read -r f; do [ -e "$f" ] || { echo "     MISSING $f"; miss=$((miss+1)); }; done < /tmp/107_cited_probes.txt
echo "  missing: $miss"

echo
echo "=== correcting this probe's own false positive, and the probe-file count ==="
T=106_giesen_consolidation_the_strategy_axis.md
echo "  The 'MISSING 106_probes/p2' above is MINE, not 106's: 106 cites its own"
echo "  probe by stem and the file is 106_probes/p2_corpus_counts.sh, which exists."
echo
echo -n "  distinct probe files 106 cites, excluding its own : "
grep -oE '[0-9]+_probes/[A-Za-z0-9_.]+' "$T" | grep -v '^106_' | sort -u | wc -l
echo -n "  of those, missing from the tree                   : "
m=0; for f in $(grep -oE '[0-9]+_probes/[A-Za-z0-9_.]+' "$T" | grep -v '^106_' | sort -u); do [ -e "$f" ] || m=$((m+1)); done; echo "$m"
echo -n "  distinct probe files including its own            : "
grep -oE '[0-9]+_probes/[A-Za-z0-9_.]+' "$T" | sort -u | wc -l
echo "  106 section 16 says 'twenty cited probe files across six probe"
echo "  directories'. Six directories reproduces. Twenty does not on this"
echo "  extractor, which finds 18. Every probe 106 does cite exists, so the"
echo "  gap is in how each counts rather than in a citation pointing nowhere."

echo
echo "=== the satfold medians, at their line in the committed findings file ==="
sed -n '93,95p' /Users/orgrinrt/Dev/clause-dev/arvo/mock/benches/satfold-const-gate_n10000_findings.md
