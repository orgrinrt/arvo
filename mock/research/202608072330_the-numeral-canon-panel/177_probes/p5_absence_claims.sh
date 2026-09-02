#!/bin/sh
# 177 P5. 176's negative claims about evidence, each turned into a search.
#
# A negative claim about evidence names no place, so it passes every citation checker by
# construction. Each claim below is restated as a search over a named place, and the
# search is printed rather than its verdict alone.
#
# CASES THAT MUST FAIL
#   P-A  a pattern known to be PRESENT must be found, else the searcher is broken
#   P-B  a pattern known to be ABSENT must return zero, else a zero means nothing
cd "$(dirname "$0")/.." || exit 1
f174=174_mcsherry_signature_in_part.md
f175=175_rompf_signature_in_part.md
f171=171_rompf_reply.md
f173=173_leroy_the_canon_candidate_for_the_chain.md

echo "=== CLAIM 1: 'Neither signature stated this composition' (176 sec 1) ==="
echo "  the composition = B3's witnesses are chains where clause 6's licences refuse"
for f in $f174 $f175; do
  n=$(grep -ciE 'clause 6|deletion licence|algebra licence' $f)
  m=$(grep -ciE 'licence.*(refus|deny)|(refus|deny).*licence' $f)
  printf '  %-36s mentions-clause6/licence=%s  licence-refuses=%s\n' "$f" "$n" "$m"
done
echo "  lines in either signature joining the family to a licence:"
grep -nE 'boundary function' $f174 $f175 | grep -iE 'licen' | sed 's/^/    /' || echo "    (none)"
echo

echo "=== CLAIM 2: 'the signatures quoted the sentence and neither spelled its line' ==="
for f in $f174 $f175; do
  q=$(grep -c 'statability argument, not a benchmark' $f)
  l=$(grep -c '60:210' $f)
  printf '  %-36s quotes-disclaimer=%s  spells-60:210=%s\n' "$f" "$q" "$l"
done
echo "  any 60:NNN anchor in either signature:"
grep -noE '`60:[0-9-]*`' $f174 $f175 | sed 's/^/    /' || echo "    (none)"
echo

echo "=== CLAIM 3: 'partiality was in neither 171's tested six nor its named four' ==="
echo "  171 lines naming its tested and named channel sets:"
grep -nE 'six|four' $f171 | grep -iE 'channel|tested|named' | sed 's/^/    /' | head -8
echo "  occurrences of 'partial' in 171:"
grep -c 'partial' $f171 | sed 's/^/    total=/'
echo "  and in the channel-enumerating region only:"
sed -n '/^## 3\./,/^## 4\./p' $f171 | grep -c 'partial' | sed 's/^/    sec3=/'
echo

echo "=== CLAIM 4: 'O-171-1 ... recorded only inside L3 rung history' (176 R-o) ==="
printf '  occurrences of O-171-1 in 173: %s\n' "$(grep -c 'O-171-1' $f173)"
printf '  173 sections mentioning 172 section 10 (the closer): '
grep -n 'section 10' $f173 | sed 's/^/\n    /'
echo

echo "=== CONTROLS ==="
printf '  P-A present pattern ("boundary function" in 175): %s  (must be > 0)\n' "$(grep -c 'boundary function' $f175)"
printf '  P-B absent pattern ("zzz-not-a-string" in 175)  : %s  (must be 0)\n' "$(grep -c 'zzz-not-a-string' $f175)"
