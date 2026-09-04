#!/bin/sh
# What it asks: `proposal.toml`'s own header states a test for what may be filed
# `normative`. Does it, and has any file in this panel used it on this question?
#
# It matters because that clause is governing text in the canon tier, and it
# decides the question's option two for one half of the class without anybody
# having to reason about it.
#
# The case that must fail: `control_absent_phrase`. A phrase invented for this
# control must return zero. If it returns a hit the search is matching
# everything and the zero below means nothing.
#
# The first run of this script FAILED that control, and the failure was real:
# the search ran over the panel directory, this script sits in the panel
# directory, and it names the invented phrase in order to search for it. So the
# instrument found itself. Both searches now exclude `262_probes/`, which is
# what a detector has to do about its own source, and the exclusion is stated
# rather than hidden because it is the one thing that could hide a true hit.
set -eu

ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT"
OUT=mock/research/202608072330_the-numeral-canon-panel/262_probes/p1_output.txt
PANEL=mock/research/202608072330_the-numeral-canon-panel
MINE='262_probes/'

{
    echo "base: $(git rev-parse HEAD)"
    echo

    echo "== the clause, quoted from the canon tier =="
    sed -n '17,21p' mock/registry/proposal.toml
    echo

    echo "== control: a phrase invented here, which must not be found =="
    N=$(grep -rl "however conjectural its grammar" mock/registry/ "$PANEL" 2>/dev/null | grep -cv "$MINE" || true)
    echo "hits outside this probe directory: $N"
    if [ "$N" -ne 0 ]; then
        echo "CONTROL FAILED: the search matches things that are not there"
        exit 1
    fi
    echo

    echo "== positive control: the real phrase is findable at all =="
    grep -rl "however definitional its grammar" mock/registry/ | sed 's/^/  /'
    echo

    echo "== who in the panel cites it =="
    H=$(grep -rl "however definitional its grammar" "$PANEL" 2>/dev/null | grep -cv "$MINE" || true)
    echo "panel files quoting the clause, excluding this probe directory: $H"
    grep -rl "however definitional its grammar" "$PANEL" 2>/dev/null | grep -v "$MINE" | sed 's/^/  /' || true
    echo
    echo "and the seat that answered this question before me:"
    grep -c "however definitional its grammar" \
        "$PANEL/261_jhala_a_structural_claim_has_a_region_and_the_checker_cannot_read_it.md" || true
    echo

    echo "== the row the clause bears on, and what it says about itself =="
    awk 'BEGIN{RS="\\[\\[proposal\\]\\]"} /id = "the_topics_form_a_stack_a_frame_and_the_canons_own_machinery"/{
        if(match($0,/\nsentence_kind = "[^"]*"/)) print "  sentence_kind:"substr($0,RSTART+17,RLENGTH-18);
        if(match($0,/Checkable rather than arguable[^\\]*/)) print "  its own note says: "substr($0,RSTART,RLENGTH);
    }' mock/registry/proposal.toml
} > "$OUT" 2>&1
cat "$OUT"
