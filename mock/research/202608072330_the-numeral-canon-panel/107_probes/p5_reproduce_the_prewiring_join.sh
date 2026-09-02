#!/usr/bin/env bash
# p5: independent reproduction of 106 section 3.2's reproduction of 103's
# F-103-6. Written from the description rather than from 106_probes/p3, so the
# agreement is an instance rather than a rerun.
#
# The claim under test, three parts:
#   (a) 253 of 254 git_commit values carry -dirty, so a naive join returns 253
#       unresolvable and the finding evaporates
#   (b) harness::validate is present in mock/benches/src/main.rs at 9db33f8c
#       and absent at its parent
#   (c) 175 regions produced before that commit, 79 after, 0 unresolvable
set -u
cd /Users/orgrinrt/Dev/clause-dev/arvo || exit 1

echo "=== (a) the -dirty precondition ==="
tot=$(ls mock/benches/*.meta.json 2>/dev/null | wc -l | tr -d ' ')
dirty=$(grep -h '"git_commit"' mock/benches/*.meta.json | grep -c -- '-dirty')
clean=$((tot - dirty))
echo "meta files                          : $tot"
echo "git_commit values ending -dirty     : $dirty"
echo "git_commit values clean             : $clean"
echo
echo "-- what a naive join (no strip) would resolve --"
naive_ok=0; naive_bad=0
while read -r c; do
  if git cat-file -e "${c}^{commit}" 2>/dev/null; then naive_ok=$((naive_ok+1)); else naive_bad=$((naive_bad+1)); fi
done < <(grep -h '"git_commit"' mock/benches/*.meta.json | grep -oE "\"git_commit\":\"[^\"]+\"" | sed "s/.*:\"//; s/\"//")
echo "naive: resolvable=$naive_ok unresolvable=$naive_bad"

echo
echo "=== (b) the wiring commit ==="
echo -n "harness::validate present in main.rs at 9db33f8c : "
git show 9db33f8c:mock/benches/src/main.rs 2>/dev/null | grep -qE 'harness::validate|validation::validate' && echo True || echo False
echo -n "present at its parent 9db33f8c^                  : "
git show 9db33f8c^:mock/benches/src/main.rs 2>/dev/null | grep -qE 'harness::validate|validation::validate' && echo True || echo False
echo -n "subject: "; git log -1 --format=%s 9db33f8c

echo
echo "=== (c) the split, stripping -dirty ==="
WIRE_TS=$(git log -1 --format=%ct 9db33f8c)
before=0; after=0; unres=0
declare -a strs
while read -r c; do
  strs+=("$c")
  s=${c%-dirty}
  ts=$(git log -1 --format=%ct "$s" 2>/dev/null)
  if [ -z "$ts" ]; then unres=$((unres+1))
  elif [ "$ts" -lt "$WIRE_TS" ]; then before=$((before+1))
  else after=$((after+1)); fi
done < <(grep -h '"git_commit"' mock/benches/*.meta.json | grep -oE "\"git_commit\":\"[^\"]+\"" | sed "s/.*:\"//; s/\"//")
echo "produced BEFORE the wiring : $before"
echo "produced AFTER  the wiring : $after"
echo "unresolvable               : $unres"
echo
echo "-- 106 reconciles 103's 24 as a STRING count against 23 distinct commits --"
echo -n "distinct git_commit strings  : "; printf '%s\n' "${strs[@]}" | sort -u | wc -l
echo -n "distinct commits after strip : "; printf '%s\n' "${strs[@]}" | sed 's/-dirty$//' | sort -u | wc -l
echo "-- the string that appears both clean and dirty --"
printf '%s\n' "${strs[@]}" | sed 's/-dirty$//' | sort | uniq -c | while read -r n s; do
  cl=$(printf '%s\n' "${strs[@]}" | grep -cx "$s")
  dt=$(printf '%s\n' "${strs[@]}" | grep -cx "$s-dirty")
  [ "$cl" -gt 0 ] && [ "$dt" -gt 0 ] && echo "  $s : clean=$cl dirty=$dt"
done
