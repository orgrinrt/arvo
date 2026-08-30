#!/usr/bin/env bash
# Controls for the two validators this pass relies on, so a pass can be believed.
#
# Each arm mutates exactly one thing in a committed registry file, runs the
# validator, restores, and prints how many substitutions actually landed. The
# substitution count is printed because BSD sed and GNU sed disagree about
# several in-place forms, and a mutation that silently changes nothing produces
# a clean run that reads exactly like the checker being correct.
#
# Run from the repository root.
set -uo pipefail

LAW=mock/registry/law.toml
PROP=mock/registry/proposal.toml
BAK=$(mktemp -d)

restore() { cp "$BAK/$(basename "$1")" "$1"; }
snapshot() { cp "$1" "$BAK/$(basename "$1")"; }
subs() { grep -c "$1" "$2" 2>/dev/null || echo 0; }

echo "=============================================================="
echo "CONTROL 0. Baseline, unmutated. Both validators must be clean."
echo "=============================================================="
cargo mock --lint-only 2>&1 | sed -n '/--- registry ---/,$p'
( cd mock && cargo test -p arvo-checks 2>&1 | grep -E '^test result:' )

echo
echo "=============================================================="
echo "CONTROL 1. A predicate naming an axis no dimension row declares."
echo "=============================================================="
snapshot "$LAW"
sed -i '' '0,/^  "fraction_width: F any",$/s//  "no_such_axis_at_all: F any",/' "$LAW"
echo "substitutions applied: $(subs 'no_such_axis_at_all' "$LAW")"
echo "--- cargo mock --lint-only ---"
cargo mock --lint-only 2>&1 | sed -n '/--- registry ---/,$p'
echo "--- cargo test -p arvo-checks (axis arm) ---"
( cd mock && cargo test -p arvo-checks --test every_predicate_names_a_declared_axis 2>&1 \
    | grep -E 'predicate-names-an-undeclared-dimension|^test result:' )
restore "$LAW"

echo
echo "=============================================================="
echo "CONTROL 2. A provenance citation naming a file that does not exist."
echo "=============================================================="
snapshot "$LAW"
sed -i '' '0,|63_spj_consolidation_the_format_concept|s||63_NO_SUCH_FILE|' "$LAW"
echo "substitutions applied: $(subs '63_NO_SUCH_FILE' "$LAW")"
cargo mock --lint-only 2>&1 | sed -n '/--- registry ---/,$p'
restore "$LAW"

echo
echo "=============================================================="
echo "CONTROL 3. A line citation past the end of a real panel file."
echo "=============================================================="
snapshot "$LAW"
sed -i '' '0,|the_format_concept::444|s||the_format_concept::99999|' "$LAW"
echo "substitutions applied: $(subs 'the_format_concept::99999' "$LAW")"
cargo mock --lint-only 2>&1 | sed -n '/--- registry ---/,$p'
restore "$LAW"

echo
echo "=============================================================="
echo "CONTROL 4. A line citation into a living ledger, which is refused by name."
echo "=============================================================="
snapshot "$PROP"
sed -i '' '0,|AGREEMENTS::#6-cross-topic-agreements|s||AGREEMENTS::470|' "$PROP"
echo "substitutions applied: $(subs 'AGREEMENTS::470' "$PROP")"
echo "--- cargo mock --lint-only ---"
cargo mock --lint-only 2>&1 | sed -n '/--- registry ---/,$p'
echo "--- cargo test -p arvo-checks (living-ledger arm) ---"
( cd mock && cargo test -p arvo-checks --test no_line_citation_into_a_living_ledger 2>&1 \
    | grep -E 'living|^test result:' )
restore "$PROP"

echo
echo "=============================================================="
echo "CONTROL 5. A heading anchor naming no heading."
echo "=============================================================="
snapshot "$PROP"
sed -i '' '0,|#6-cross-topic-agreements|s||#6-cross-topic-agreements-BROKEN|' "$PROP"
echo "substitutions applied: $(subs 'cross-topic-agreements-BROKEN' "$PROP")"
cargo mock --lint-only 2>&1 | sed -n '/--- registry ---/,$p'
restore "$PROP"

echo
echo "=============================================================="
echo "CONTROL 6. An established claim with no region, and an imposed one with one."
echo "=============================================================="
snapshot "$PROP"
sed -i '' '0,|^sentence_kind = "normative"$|s||sentence_kind = "theorem"|' "$PROP"
echo "substitutions applied: $(grep -c '^sentence_kind = \"theorem\"' "$PROP")  (count of theorem rows after mutation)"
( cd mock && cargo test -p arvo-checks --test what_one_field_obliges_another_to_carry 2>&1 \
    | grep -E 'an-established-claim-carries-no-region|^test result:' | head -5 )
restore "$PROP"

echo
echo "=============================================================="
echo "CONTROL 7. Restored. Both validators must be clean again."
echo "=============================================================="
cargo mock --lint-only 2>&1 | sed -n '/--- registry ---/,$p'
( cd mock && cargo test -p arvo-checks 2>&1 | grep -E '^test result:' )
rm -rf "$BAK"
