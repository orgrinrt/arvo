#!/usr/bin/env bash
# Which candidate axis would unblock which committed clause, at row granularity.
#
# `unblock_value.out` ranks PHRASES by predicate spans. This reads the row-level
# inventory instead: every statement clause the port could not write, with the
# reason column naming the axes that blocked it. Two instruments over two
# populations; where they disagree that is worth knowing.
#
# It also re-scores the reasons against TODAY's registry, since `ambient_domain`
# and `radix` were declared after the inventory was taken and still appear in
# its reason column.
#
# Controls, written before the run:
#   B1  the verdict tally must reproduce the committed probe row's counts:
#       24 NO-AXIS, 16 NO-REGION, 23 PORTED, 1 REFUSED, 1 folded, 65 total.
#   B2  a family nobody wrote must score zero (`phase_of_the_moon`).
#   B3  re-scoring against today's registry must strictly reduce the count of
#       clauses blocked only by now-declared axes, or the rescore did nothing.
set -euo pipefail
cd "$(dirname "$0")"
INV=../183_probes/blocked_inventory.tsv

echo "### B1, verdict tally"
awk -F'\t' 'NF==5{v[$4]++; n++} END{for(k in v) printf "  %-10s %s\n", k, v[k]; printf "  %-10s %s\n", "TOTAL", n}' "$INV" | sort
echo

echo "### NO-AXIS clauses, with the reason column verbatim"
awk -F'\t' 'NF==5 && $4=="NO-AXIS"{printf "  %s:%s  %s\n      blocked by: %s\n", $1, $2, substr($3,1,70), $5}' "$INV"
echo

# The slugs declared today, so a reason naming only these is already unblocked.
DECLARED_SPELLINGS='ambient domain|domain|radix|accumulator width|edition|toolchain|operand window|declarations|restrictions|occupancy'
echo "### re-scored against today's registry"
echo "### (a reason phrase matching a declared axis or one of its keywords is struck)"
awk -F'\t' -v dec="$DECLARED_SPELLINGS" '
NF==5 && $4=="NO-AXIS" {
  n = split($5, parts, "; ")
  left = ""
  for (i=1;i<=n;i++) {
    p = parts[i]
    gsub(/^ +| +$/, "", p)
    if (p ~ ("^(" dec ")")) continue
    left = (left=="" ? p : left "; " p)
  }
  if (left=="") { cleared++ ; printf "  CLEARED  %s:%s  %s\n", $1,$2,substr($3,1,60) }
  else { still++; printf "  STILL    %s:%s  %s\n      remaining: %s\n", $1,$2,substr($3,1,55), left }
}
END { printf "\n  cleared by the four already declared: %d\n  still blocked: %d\n", cleared+0, still+0 }' "$INV"
echo
echo "### B2 control"
awk -F'\t' 'NF==5 && $5 ~ /phase_of_the_moon/{n++} END{ if(n+0==0) print "  PASS, no clause names it"; else print "  FAIL" }' "$INV"

echo
echo "### B3, the rescore must clear some clauses, and must clear NONE with an empty declared set"
c_real=$(awk -F'\t' -v dec="$DECLARED_SPELLINGS" '
NF==5 && $4=="NO-AXIS" { n=split($5,p,"; "); left=""
  for(i=1;i<=n;i++){ q=p[i]; gsub(/^ +| +$/,"",q); if(q ~ ("^(" dec ")")) continue; left=(left==""?q:left "; " q) }
  if(left=="") c++ } END{print c+0}' "$INV")
c_null=$(awk -F'\t' -v dec="__nothing_matches_this__" '
NF==5 && $4=="NO-AXIS" { n=split($5,p,"; "); left=""
  for(i=1;i<=n;i++){ q=p[i]; gsub(/^ +| +$/,"",q); if(q ~ ("^(" dec ")")) continue; left=(left==""?q:left "; " q) }
  if(left=="") c++ } END{print c+0}' "$INV")
printf '  with the real declared set: %s cleared\n  with an empty one:         %s cleared\n' "$c_real" "$c_null"
if [ "$c_real" -gt 0 ] && [ "$c_null" -eq 0 ]; then echo "  PASS"; else echo "  FAIL, the rescore does not depend on the declared set"; fi
