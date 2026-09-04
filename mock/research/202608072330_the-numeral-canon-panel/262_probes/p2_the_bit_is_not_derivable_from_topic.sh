#!/bin/sh
# What it asks: `261` section 12 leaves O2 open, whether a row can declare that
# its subject instantiates no declared axis, and says the cheap close is showing
# that bit is derivable from `topic` instead. This asks whether it is.
#
# Derivable means: every row on a machinery topic has a structural subject, and
# no row on a stack or frame topic does. One valid counterexample in the second
# direction refutes it.
#
# The case that must fail: `machinery_cell_is_nonempty`. If the classifier
# reports zero machinery rows it is not reading the registry and the
# counterexample hunt below proves nothing by returning nothing.
set -eu

ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT"
OUT=mock/research/202608072330_the-numeral-canon-panel/262_probes/p2_output.txt
FILES="mock/registry/proposal.toml mock/registry/proposal-the-later-topics.toml"

# The four machinery topics, read off `proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`.
MACHINERY='^(canon_form|the_predicate_notation|naming|panel_conduct)$'

rows() {
    for f in $FILES; do
        awk 'BEGIN{RS="\\[\\[proposal\\]\\]"} NR>1{
            id="";t="";k="";p="no";s="";
            if(match($0,/\nid = "[^"]*"/)) id=substr($0,RSTART+7,RLENGTH-8);
            if(match($0,/\ntopic = "[^"]*"/)) t=substr($0,RSTART+10,RLENGTH-11);
            if(match($0,/\nsentence_kind = "[^"]*"/)) k=substr($0,RSTART+18,RLENGTH-19);
            if($0 ~ /\npredicate = /) p="yes";
            if(match($0,/\nsays = "[^"]*"/)) s=substr($0,RSTART+9,RLENGTH-10);
            gsub(/\t/," ",s);
            print t"\t"k"\t"p"\t"id"\t"s
        }' "$f"
    done
}

{
    echo "base: $(git rev-parse HEAD)"
    echo "rows read: $(rows | wc -l)"
    echo

    echo "== control: the machinery cell is non-empty =="
    M=$(rows | awk -F'\t' -v m="$MACHINERY" '$1 ~ m' | wc -l)
    echo "machinery rows: $M"
    if [ "$M" -eq 0 ]; then
        echo "CONTROL FAILED: classifier sees no machinery rows; everything below is void"
        exit 1
    fi
    echo

    echo "== machinery rows, kind x carries-a-predicate =="
    rows | awk -F'\t' -v m="$MACHINERY" '$1 ~ m {print $2"\t"$3}' | sort | uniq -c
    echo
    echo "== which topic the region-bearing machinery rows sit on =="
    rows | awk -F'\t' -v m="$MACHINERY" '$1 ~ m && $3=="yes" {print "  "$1" | "$2" | "$4}'
    echo

    echo "== the refuting direction: structural subject on a non-machinery topic =="
    echo "pattern: a says that quantifies over the panels own disputes or over how the canon is organised"
    rows | awk -F'\t' -v m="$MACHINERY" '$1 !~ m && ($5 ~ /this unit disputed|amending the canon|The canon defines once/) {print "  "$1" | "$2" | "$4; print "     says: "substr($5,1,170)}'
    echo

    echo "== control for that hunt: the same pattern over machinery rows must also match something =="
    echo "  (a pattern matching only outside the machinery set would be selecting for the answer)"
    rows | awk -F'\t' -v m="$MACHINERY" '$1 ~ m && $5 ~ /canon|registry|row/ {c++} END{print "  machinery rows whose says names the canon, the registry or a row: "c+0}'
} > "$OUT" 2>&1
cat "$OUT"
