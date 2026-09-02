#!/usr/bin/env bash
# Review of PRs 89/90/91/92 against origin/dev. Reproduces every measurement the
# report rests on. Run from the repo root. No writes.
set -uo pipefail
DEV=origin/dev
B90=origin/fix/obligation-coverage-reads-the-rung-and-follows-ratifies
B91=origin/fix/the-release-surface
B92=origin/fix/the-write-hook-excepts-what-the-canon-excepts
B89=origin/feat/the-standards-alias-and-what-it-refutes

echo "## 1. dev tip"; git rev-parse $DEV

echo; echo "## 2. is dev already contained in each branch, and what is left"
for b in $B90 $B91 $B92 $B89; do
  printf '%s\n' "$b"
  printf '   merge-base: %s\n' "$(git merge-base $DEV $b)"
  if git merge-base --is-ancestor $DEV $b; then echo "   dev is an ancestor: YES"; else echo "   dev is an ancestor: NO"; fi
  printf '   net files differing from dev: %s\n' "$(git diff --name-only $DEV $b | wc -l | tr -d ' ')"
done

echo; echo "## 3. the whole remaining delta of the three fix branches"
for b in $B90 $B91 $B92; do echo "--- $b"; git diff --stat $DEV $b; done

echo; echo "## 4. arvo-format's real version, which the probe lockfiles disagree with"
echo -n "  crate manifest: "; git show $DEV:mock/crates/arvo-format/Cargo.toml | grep '^version'
echo -n "  workspace:      "; git show $DEV:mock/Cargo.toml | sed -n '/\[workspace.package\]/,/^$/p' | grep '^version'
echo -n "  dev lockfile:   "; git show $DEV:mock/research/202608072330_the-numeral-canon-panel/242_probes/admission/Cargo.lock | grep -A1 'name = "arvo-format"' | grep version
echo -n "  b90 lockfile:   "; git show $B90:mock/research/202608072330_the-numeral-canon-panel/242_probes/admission/Cargo.lock | grep -A1 'name = "arvo-format"' | grep version

echo; echo "## 5. PR89: what the branch has that dev lacks, by status"
git diff --name-status $DEV $B89 | awk '{print $1}' | sort | uniq -c
echo "  files only on the branch:"; git diff --name-status $DEV $B89 | awk '$1=="A"{print "    "$2}'

echo; echo "## 6. PR89: the one test fn on the branch and not on dev"
git show $B89:mock/crates/arvo-format/src/tests/identity.rs | grep -oE 'fn [a-z_0-9]+' | sed 's/fn //' | sort -u > /tmp/r7_br
( git show $DEV:mock/crates/arvo-format/src/tests/the_identity.rs
  git show $DEV:mock/crates/arvo-format/src/tests/the_identity/the_cancelling_slot.rs
  git show $DEV:mock/crates/arvo-format/src/tests/the_identity/the_magnitude_range.rs
) | grep -oE 'fn [a-z_0-9]+' | sed 's/fn //' | sort -u > /tmp/r7_dv
echo -n "  branch fns: "; wc -l < /tmp/r7_br
echo -n "  dev fns:    "; wc -l < /tmp/r7_dv
echo "  on branch, absent from dev:"; comm -23 /tmp/r7_br /tmp/r7_dv | sed 's/^/    /'

echo; echo "## 7. PR89: that test's three assertions, located on dev"
for pat in 'Shrinking<2>, Signed<8>, 1, 2' 'cutting the magnitude range to one' 'Shrinking<40>, Signed<62>, 1, 3'; do
  echo "  [$pat]"; grep -rn "$pat" mock/crates/arvo-format/src/ | sed 's/^/    /'
done

echo; echo "## 8. PR89: the merge does not compute cleanly"
git merge-tree --write-tree --name-only $DEV $B89 >/tmp/r7_mt 2>&1; echo "  exit=$?"
grep -c '^CONFLICT' /tmp/r7_mt | sed 's/^/  conflicts: /'
grep '^CONFLICT' /tmp/r7_mt | sed 's/^/    /'

echo; echo "## 9. PR89 would put host primitives back on arvo-format's public trait consts"
git diff $DEV $B89 -- mock/crates/arvo-format/src/standards.rs | grep -E '^[-+] *const ' | sed 's/^/    /'
echo "  dev carries the lint that refuses exactly this:"
ls mock/lints/a_contract_coordinate_is_not_a_host_primitive.rs | sed 's/^/    /'

echo; echo "## 10. bylines, adverts and em-dashes, with the positive control"
for b in $B90 $B91 $B92 $B89; do
  printf '  %s: coauth=%s genwith=%s emdash=%s robot=%s\n' "${b#origin/}" \
    "$(git log --format='%B' $DEV..$b | grep -ci 'co-authored-by')" \
    "$(git log --format='%B' $DEV..$b | grep -ci 'generated with')" \
    "$(git log --format='%B' $DEV..$b | grep -c '—')" \
    "$(git log --format='%B' $DEV..$b | grep -c '🤖')"
done
echo "  POSITIVE CONTROL, same greps over all of dev's history (must be non-zero):"
printf '    coauth=%s emdash=%s\n' \
  "$(git log --format='%B' $DEV | grep -ci 'co-authored-by')" \
  "$(git log --format='%B' $DEV | grep -c '—')"

echo; echo "## 11. commit subjects past 72 characters"
for b in $B90 $B91 $B92 $B89; do
  git log --format='%s' $DEV..$b --no-merges | while read -r s; do
    [ ${#s} -gt 72 ] && echo "    ${b#origin/} len=${#s}: $s"
  done
done
