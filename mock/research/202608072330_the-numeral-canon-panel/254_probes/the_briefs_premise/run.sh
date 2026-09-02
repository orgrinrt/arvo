#!/usr/bin/env bash
# Seat 253. My brief says: "`242` states explicitly that its reading of admission
# should not be merged with `241`'s". Checked before anything else, because a
# false premise in a brief produces confident work in its direction.
#
# The same claim was handed to seat 244 and seat 245 and both refuted it. This is
# a third independent run with its own controls, because two agreeing unratified
# reports are not corroboration.
set -u
D=../..
echo "======== the claim: 242 says its reading must not be merged with 241's"
for p in "merge" "merged" "merging" "combine" "combined" "synthes" "should not be read" "cannot be read as one" "must not be" "collaps"; do
  n241=$(grep -ci -- "$p" "$D/241_kiselyov_admission_is_a_resolution_not_a_verdict.md" 2>/dev/null || true)
  n242=$(grep -ci -- "$p" "$D/242_what-admits-a-number-system.md" 2>/dev/null || true)
  n243=$(grep -ci -- "$p" "$D/243_seat242_the_resolution_has_no_second_arm.md" 2>/dev/null)
  printf "  %-24s 241=%-4s 242=%-4s 243=%-4s\n" "$p" "$n241" "$n242" "$n243"
done
echo
echo "======== POSITIVE CONTROL: words that are certainly in those files"
for p in "resolution" "tier" "admission" "coordinate"; do
  n241=$(grep -ci -- "$p" "$D/241_kiselyov_admission_is_a_resolution_not_a_verdict.md")
  n242=$(grep -ci -- "$p" "$D/242_what-admits-a-number-system.md")
  n243=$(grep -ci -- "$p" "$D/243_seat242_the_resolution_has_no_second_arm.md")
  printf "  %-24s 241=%-4s 242=%-4s 243=%-4s\n" "$p" "$n241" "$n242" "$n243"
done
echo
echo "======== what 242 and 243 do say about 241's reading, in full"
echo "-------- 243 section 6, the withdrawal:"
sed -n '/^## 6\. Where 241 beat me/,/^## 7\./p' "$D/243_seat242_the_resolution_has_no_second_arm.md"

echo
echo "======== the absence claim in section 13: prior statements of the tuple non-injectivity"
echo "-------- files matching each phrase, excluding seat 253's own"
for p in "normal form" "same set" "denote the same" "non-injective" "not injective"; do
  hits=$(grep -ril -- "$p" "$D"/*.md 2>/dev/null | grep -v '253_' | tr '\n' ' ')
  printf "  %-18s : %s\n" "$p" "${hits:-<none>}"
done
echo "-------- POSITIVE CONTROL: a phrase certainly present across many files"
printf "  %-18s : %s files\n" "representable set" "$(grep -ril -- 'representable set' "$D"/*.md | wc -l | tr -d ' ')"
echo "-------- what the one recurring hit is about, checked rather than counted"
grep -n -- "not injective\|non-injective" "$D"/243_seat242_the_resolution_has_no_second_arm.md | head -5
