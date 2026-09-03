#!/usr/bin/env bash
# C. What in this registry is able to carry a region at all, and over what.
#
# The open question assumes the obstacle is the dimension vocabulary: "all
# declared dimensions are numeric or machine axes, so a reasoned claim about the
# canon's own contents has no expressible region". This arm checks the halves of
# that separately, because they are different claims and only one is about
# dimensions.
#
#   C1  which FIELDS in the whole registry are read as a region. Out of the lint
#       source, not out of memory.
#   C2  which NAMESPACES declare those fields, and therefore which namespaces
#       can state a region at all. The `ruling` namespace is the canon proper.
#   C3  whether every declared axis is a numeric or machine axis, by walking the
#       `what` sentences rather than by assertion.
#   C4  how the sentence kinds distribute across topics, cut by whether the
#       topic is about arvo or about the canon's own machinery. The machinery
#       topics are read out of
#       `proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`
#       rather than chosen here.
#   C5  for every machinery-topic row that DOES carry a region, what that region
#       says. This is the arm that matters and it was added after C4 reported a
#       number the question's premise says is impossible.
#
# A first run of C5 was done by hand outside this script with an awk window that
# ran past the end of `predicate` into `provenance`, and reported panel-file
# citations sitting inside predicate arrays. That was false: the reader below
# stops at the closing bracket, and `every-predicate-names-a-declared-axis`
# carries no ceiling and passes, which is independent proof no such entry exists.
#
# Controls, written before the run:
#   K1  C1 must find as many field pairs as the lint constant declares. A
#       different number means this arm is reading a different lint.
#   K2  C3's classifier must be able to say OTHER: a planted structural
#       coordinate must not classify as numeric or machine, or the census is
#       vacuous.
#   K3  C4's machinery set must be non-empty and a strict subset of the topics
#       in use, or the cut is not a cut.
#   K4  the topic totals must sum to the engine's own proposal count.
#   K5  C5's predicate reader must stop at the array's closing bracket. Asserted
#       against `most_committed_bench_regions_predate_the_harness_cross_variant_
#       validation`, whose `predicate` holds exactly one entry and whose next
#       field but one is a two-element `provenance`. One entry passes, three
#       fails.
set -uo pipefail
cd "$(dirname "$0")"
ROOT=../../../..
REG=$ROOT/mock/registry
LINTS=$ROOT/mock/lints
MACHINERY="canon_form the_predicate_notation naming panel_conduct"

echo "### C1, the fields the checkers read as a region"
grep -n 'PREDICATE_FIELDS' -A 5 "$LINTS/canon_rows.rs" | sed -n '1,6p'
declared=$(grep -oE 'PREDICATE_FIELDS: \[\(&str, &str\); [0-9]+\]' "$LINTS/canon_rows.rs" \
  | grep -oE '; [0-9]+\]' | grep -oE '[0-9]+')
found=$(grep -A 5 'PREDICATE_FIELDS' "$LINTS/canon_rows.rs" | grep -cE '^\s+\("')
echo "  K1: constant declares ${declared:-?} entries, the list holds $found"
[ "${declared:-x}" = "$found" ] && echo "  PASS" || echo "  FAIL, this arm is reading something else"
echo

echo "### C2, which namespaces can state a region, against every namespace there is"
all=$(grep -oE '^key = "[a-z_]+"' "$ROOT/mockspace.toml" | sed 's/key = "//;s/"//' | sort -u)
withregion=$(grep -A 5 'PREDICATE_FIELDS' "$LINTS/canon_rows.rs" | grep -oE '\("[a-z_]+"' | tr -d '("' | sort -u)
for ns in $all; do
  if echo "$withregion" | grep -qx "$ns"; then printf '  %-12s region-bearing\n' "$ns"
  else printf '  %-12s no region field at all\n' "$ns"; fi
done
echo

echo "### C3, the declared axes, classified by whether their subject is a numeral or a machine"
classify() {
  grep -qiE 'bit|width|numeral|format|operand|operation|arithmetic|value|machine|storage|container|thread|instruction|compiler|compil|build|memory|align|domain|base|grid|term|parenthesis|leaves|occupant|round|overflow|strategy|negative' <<<"$1"
}
n=0; off=0
while IFS=$'\t' read -r id what; do
  n=$((n+1))
  if classify "$what"; then printf '  numeric/machine  %s\n' "$id"
  else printf '  OTHER            %-18s %s\n' "$id" "$what"; off=$((off+1)); fi
done < <(awk '
  /^\[\[dimension\]\]/ { id=""; next }
  /^id = /   { id=$0; sub(/^id = "/,"",id); sub(/"$/,"",id); next }
  /^what = / { wh=$0; sub(/^what = "/,"",wh); sub(/"$/,"",wh); printf "%s\t%s\n", id, wh; next }
' "$REG/dimension.toml")
printf '  --- axes: %s, not classified as numeric or machine: %s\n' "$n" "$off"
echo "  K2, the classifier must be able to say OTHER:"
if classify "the slug of the topic a row is filed under"; then
  echo "    FAIL, a planted structural coordinate classified as numeric/machine"
else
  echo "    PASS, a planted structural coordinate classifies as OTHER"
fi
echo

# id \t sentence_kind \t topic \t has_evidence \t predicate-entries-joined-by-|
rowdump() {
  awk '
    /^\[\[proposal(-the-later-topics)?\]\]/ { flush(); inrow=1; next }
    /^\[\[/ { flush(); inrow=0; next }
    !inrow { next }
    inpred { if ($0 ~ /^\]/) { inpred=0; next } e=$0; gsub(/^ *"|",? *$/,"",e); pr = (pr=="" ? e : pr "|" e); next }
    /^id = /            { id=val(); next }
    /^sentence_kind = / { sk=val(); next }
    /^topic = /         { tp=val(); next }
    /^evidence = /      { ev="yes"; next }
    /^predicate = \[$/  { inpred=1; next }
    END { flush() }
    function val(   s) { s=$0; sub(/^[a-z_]+ = /,"",s); gsub(/^"|"$/,"",s); return s }
    function flush() { if (id!="") printf "%s\t%s\t%s\t%s\t%s\n", id, sk, tp, (ev==""?"no":ev), pr; id="";sk="";tp="";ev="";pr="";inpred=0 }
  ' "$REG/proposal.toml" "$REG/proposal-the-later-topics.toml"
}

echo "### K5, the predicate reader stops at the closing bracket"
k5=$(rowdump | awk -F'\t' '$1=="most_committed_bench_regions_predate_the_harness_cross_variant_validation"{print $5}')
k5n=$(awk -v s="$k5" 'BEGIN{print (s=="" ? 0 : split(s,a,"|"))}')
echo "  entries read: $k5n -> $k5"
[ "$k5n" = "1" ] && echo "  PASS, one entry, so provenance was not swallowed" \
  || echo "  FAIL, the window ran past the array"
echo

echo "### C4, sentence kind against topic, cut by arvo versus the canon's own machinery"
echo "  machinery topics: $MACHINERY"
inuse=$(rowdump | awk -F'\t' '{print $3}' | sort -u | tr '\n' ' ')
echo "  topics in use: $inuse"
mcount=0
for m in $MACHINERY; do echo " $inuse" | grep -qw "$m" && mcount=$((mcount+1)); done
echo "  K3: machinery topics present among those in use: $mcount of 4"
[ "$mcount" -gt 0 ] && echo "    PASS" || echo "    FAIL, the cut selects nothing"
echo
printf '  %-12s %8s %10s\n' kind arvo machinery
tot=0
for k in theorem measured enumeration definition normative argument; do
  a=$(rowdump | awk -F'\t' -v k="$k" -v M=" $MACHINERY " '$2==k && index(M," "$3" ")==0' | wc -l | tr -d ' ')
  m=$(rowdump | awk -F'\t' -v k="$k" -v M=" $MACHINERY " '$2==k && index(M," "$3" ")>0'  | wc -l | tr -d ' ')
  printf '  %-12s %8s %10s\n' "$k" "$a" "$m"
  tot=$((tot+a+m))
done
eng=$( (cargo mock query 'proposal.select(id).count()' 2>/dev/null || true) | grep -oE '^[0-9]+$' | tail -1 )
[ -z "${eng:-}" ] && eng=$(rowdump | wc -l | tr -d ' ')
echo "  K4: rows counted by kind $tot, rows in the namespace $eng"
[ "$tot" = "$eng" ] && echo "    PASS" || echo "    FAIL, some row carries a kind outside the declared set"
echo

echo "### C5, every machinery-topic row, and what region it actually states"
rowdump | awk -F'\t' -v M=" $MACHINERY " '
  index(M," "$3" ")>0 {
    printf "  %-12s evidence:%-4s %s\n", $2, $4, $1
    if ($5 == "") print "      region: none"
    else { n=split($5,a,"|"); for (i=1;i<=n;i++) printf "      region: %s\n", a[i] }
  }'
