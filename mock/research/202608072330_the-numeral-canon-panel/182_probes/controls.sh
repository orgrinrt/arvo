#!/usr/bin/env bash
# Controls for the two validators this pass relies on, so a pass can be believed.
#
# Each arm mutates exactly one thing in a committed registry file, runs the
# validator, restores, and prints how many substitutions actually landed. The
# substitution count is printed because a mutation that silently changes nothing
# produces a clean run that reads exactly like the checker being correct.
#
# That is not hypothetical here. The first version of this script used `|` as
# the delimiter of a `0,/re/s//repl/` range address, which GNU sed tolerates and
# BSD sed refuses with "expected context address". Five of seven arms mutated
# nothing, every one of them reported a clean validator, and only the
# substitution counter said so. Every delimiter below is `/` for that reason,
# and no pattern below may contain one.
#
# Run from the repository root.
set -uo pipefail

LAW=mock/registry/law.toml
PROP=mock/registry/proposal.toml
BAK=$(mktemp -d)
trap 'rm -rf "$BAK"' EXIT

snapshot() { cp "$1" "$BAK/$(basename "$1")"; }
restore()  { cp "$BAK/$(basename "$1")" "$1"; }
count()    { grep -c -- "$1" "$2" 2>/dev/null || true; }

lint() { cargo mock --lint-only 2>&1 | sed -n '/--- registry ---/,$p' | grep -vE 'unknown-config-key|lint-only mode, skipping'; }
checks() { ( cd mock && cargo test -p arvo-checks 2>&1 | grep -E '^test result:' ); }
arm() { ( cd mock && cargo test -p arvo-checks --test "$1" 2>&1 | grep -E "$2|^test result:" ); }

echo "=============================================================="
echo "CONTROL 0. Baseline, unmutated."
echo "=============================================================="
lint
checks
echo "(one arvo-checks binary is expected FAILED at baseline: the twelve"
echo " measured proposals carry no evidence because probe.toml does not exist.)"

echo
echo "=============================================================="
echo "CONTROL 1. A predicate naming an axis no dimension row declares."
echo "=============================================================="
snapshot "$LAW"
sed -i '' 's/fraction_width: F any/no_such_axis_at_all: F any/g' "$LAW"
echo "substitutions applied: $(count no_such_axis_at_all "$LAW")"
echo "--- cargo mock --lint-only ---"
lint
echo "--- arvo-checks, axis arm ---"
arm every_predicate_names_a_declared_axis 'predicate-names-an-undeclared-dimension'
restore "$LAW"

echo
echo "=============================================================="
echo "CONTROL 2. A provenance citation naming a file that does not exist."
echo "=============================================================="
snapshot "$LAW"
sed -i '' 's/63_spj_consolidation_the_format_concept/63_NO_SUCH_FILE/g' "$LAW"
echo "substitutions applied: $(count 63_NO_SUCH_FILE "$LAW")"
lint
restore "$LAW"

echo
echo "=============================================================="
echo "CONTROL 3. A line citation past the end of a real panel file."
echo "=============================================================="
snapshot "$LAW"
sed -i '' 's/the_format_concept::444/the_format_concept::99999/g' "$LAW"
echo "substitutions applied: $(count 'the_format_concept::99999' "$LAW")"
lint
restore "$LAW"

echo
echo "=============================================================="
echo "CONTROL 4. A line citation into a living ledger, refused by name."
echo "=============================================================="
snapshot "$PROP"
sed -i '' 's/AGREEMENTS::#6-cross-topic-agreements/AGREEMENTS::470/g' "$PROP"
echo "substitutions applied: $(count 'AGREEMENTS::470' "$PROP")"
echo "--- cargo mock --lint-only ---"
lint
echo "--- arvo-checks, living-ledger arm ---"
arm no_line_citation_into_a_living_ledger 'living-ledger|line-citation'
restore "$PROP"

echo
echo "=============================================================="
echo "CONTROL 5. A heading anchor naming no heading."
echo "=============================================================="
snapshot "$PROP"
sed -i '' 's/#6-cross-topic-agreements/#6-cross-topic-agreements-BROKEN/g' "$PROP"
echo "substitutions applied: $(count 'cross-topic-agreements-BROKEN' "$PROP")"
lint
restore "$PROP"

echo
echo "=============================================================="
echo "CONTROL 6. An established claim carrying no region."
echo "=============================================================="
snapshot "$PROP"
before=$(count 'sentence_kind = "normative"' "$PROP")
sed -i '' 's/sentence_kind = "normative"/sentence_kind = "theorem"/g' "$PROP"
after=$(count 'sentence_kind = "normative"' "$PROP")
echo "normative rows before: $before   after: $after   substitutions applied: $((before - after))"
arm what_one_field_obliges_another_to_carry 'an-established-claim-carries-no-region'
restore "$PROP"

echo
echo "=============================================================="
echo "CONTROL 7. An imposed proposition carrying a region."
echo "=============================================================="
snapshot "$PROP"
before=$(count 'sentence_kind = "theorem"' "$PROP")
sed -i '' 's/sentence_kind = "theorem"/sentence_kind = "normative"/g' "$PROP"
after=$(count 'sentence_kind = "theorem"' "$PROP")
echo "theorem rows before: $before   after: $after   substitutions applied: $((before - after))"
arm what_one_field_obliges_another_to_carry 'an-imposed-proposition-carries-a-region'
restore "$PROP"

echo
echo "=============================================================="
echo "CONTROL 8. Restored. Back to the baseline exactly."
echo "=============================================================="
lint
checks
