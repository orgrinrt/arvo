#!/bin/sh
# What it asks: what the shipped gate does to seat 223's own structural claim
# under each filing it could take. Third independent instance of the split that
# `261_probes/d_can_a_structural_claim_state_its_region.sh` and
# `261_probes/e_lintdrive` reached; different author, different worktree,
# different base, written before that file was read.
#
# The case that must fail: arm `c_control_must_fire` plants a `normative` row
# carrying a predicate, which `a-region-agrees-with-the-sentence-kind` refuses.
# If that arm comes back silent the instrument is not reading the planted rows
# at all and every other verdict here is void.
#
# Run from the repo root. Appends to the real registry and truncates back,
# asserting the tree is clean at the end.
set -eu

ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT"
REG=mock/registry/proposal.toml
OUT=mock/research/202608072330_the-numeral-canon-panel/262_probes/p3_output.txt

if [ -n "$(git status --porcelain "$REG")" ]; then
    echo "refusing: $REG is dirty before the run" >&2
    exit 1
fi
LEN=$(wc -c < "$REG")

: > "$OUT"
{
    echo "base: $(git rev-parse HEAD)"
    echo "registry byte length before: $LEN"
    echo
} >> "$OUT"

# Every arm is seat 223's own claim, varying only the two fields at issue.
arm() {
    name=$1
    kind=$2
    extra=$3
    cp "$REG" "$REG.orig"
    {
        echo
        echo '[[proposal]]'
        echo "id = \"probe262_$name\""
        echo 'kind = "finding"'
        echo "sentence_kind = \"$kind\""
        echo 'standing = "one_expert"'
        echo 'topic = "canon_form"'
        echo 'says = "The twenty topics are not one order: eleven form a stack, four a frame, four the canons own machinery."'
        echo 'because = "Derived from the what sentence of each topic row by one edge test, which is a walk over the registry and could be measured false."'
        [ -n "$extra" ] && echo "$extra"
        echo 'provenance = ["panel::202608072330_the-numeral-canon-panel::223_checkpoint_the_topic_layering::#the-stack-bottom-first"]'
    } >> "$REG"

    echo "--- arm $name (sentence_kind=$kind; extra=${extra:-none})" >> "$OUT"
    cargo mock --lint-only > /tmp/p3run.txt 2>&1 || true
    # Report only what names this planted row, plus the pass/fail verdict line.
    grep -E "probe262_$name|all lints passed|BLOCKED" /tmp/p3run.txt >> "$OUT" || \
        echo "  (nothing matched: neither a finding nor a verdict line)" >> "$OUT"
    echo >> "$OUT"

    mv "$REG.orig" "$REG"
    touch "$REG"
}

# Control 0: the registry untouched must be green, or every verdict below is
# measuring a pre-existing failure rather than the planted row.
echo "--- arm a_control_baseline (nothing planted)" >> "$OUT"
cargo mock --lint-only > /tmp/p3run.txt 2>&1 || true
grep -E "all lints passed|BLOCKED" /tmp/p3run.txt >> "$OUT" || echo "  (no verdict line)" >> "$OUT"
echo >> "$OUT"

# The two filings the question is about.
arm b_argument_no_region argument ''
arm d_normative_no_region normative ''

# Control: must fire. A normative row carrying a region is refused.
arm c_control_must_fire normative 'predicate = ["fraction_width: F = 0"]'

# The axis the claim would want if it could name one.
arm e_argument_corpus_axis argument 'predicate = ["corpus_state: the registry at HEAD"]'

# Control: an argument over a declared axis is accepted, so the refusals above
# are about the structural claim rather than about `argument` as such.
arm f_control_argument_declared_axis argument 'predicate = ["fraction_width: F = 0"]'

if [ "$(wc -c < "$REG")" != "$LEN" ]; then
    echo "FAILED TO RESTORE: byte length differs" >> "$OUT"
    exit 1
fi
if [ -n "$(git status --porcelain "$REG")" ]; then
    echo "FAILED TO RESTORE: tree dirty" >> "$OUT"
    exit 1
fi
echo "restored: registry byte length $(wc -c < "$REG"), tree clean" >> "$OUT"
cat "$OUT"
