#!/usr/bin/env bash
# 182 section 5.2 wrote seven claims off as unportable, recovered six from the
# committed instruments, and kept the seventh out because its instrument
# declares itself unable to support a how-much question.
#
# That reasoning is correct and it is about the COUNTS. The qualitative claim
# became a row. This asks whether that row can now carry a region, since the
# axis pass declared three axes the instrument's own header states.
#
# CASE THAT MUST FAIL: control 1 asks the same of the four axes 182 section 5.1
# named as blockers, which must all read NO, run beside two that must read yes
# so a NO is about the file rather than about the path. That control read NO six
# times on two earlier runs, both times because a `cd` had left the dimension
# path unresolvable and every grep errored to stderr. It is recorded rather than
# quietly fixed, because a zero that is a fact about the pipeline is the exact
# shape this corpus keeps catching.
set -uo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
PANEL="$ROOT/mock/research/202608072330_the-numeral-canon-panel"
D="$ROOT/mock/registry/dimension.toml"
PROP="$ROOT/mock/registry/proposal.toml"
ROW=a_law_layer_answers_whether_a_law_reaches_a_lowering_the_backend_cannot_prove

echo "=== the instrument's own header ==="
head -4 "$PANEL/80_probes/p4_asm_report.txt" | sed 's/^/  /'

echo
echo "=== the row it became ==="
awk -v id="$ROW" '$0=="id = \""id"\""{f=1} f{print} f&&/^keywords/{exit}' "$PROP" \
  | grep -E '^(id|sentence_kind|standing)' | sed 's/^/  /'
echo "  predicate field present: $(awk -v id="$ROW" '$0=="id = \""id"\""{f=1} f&&/^predicate/{print "YES";exit} f&&/^keywords/{print "NO";exit}' "$PROP")"

echo
echo "=== the axes the header states, against dimension.toml ==="
printf '%-30s %-22s %s\n' 'HEADER' 'AXIS' 'DECLARED'
m() { printf '%-30s %-22s %s\n' "$1" "$2" "$(grep -qE "^id = \"$2\"\$" "$D" && echo yes || echo NO)"; }
m 'aarch64-apple-darwin'      target_features
m 'rustc nightly-2026-05-28'  toolchain
m '-O'                        build_profile
echo "  and from the row's own because, the region the claim is about:"
m 'zero fraction width'       fraction_width
m 'a saturating reduction'    overflow_policy

echo
echo "=== when the axes were declared, against when the row landed ==="
echo "  toolchain declared: $(git -C "$ROOT" log -1 --format='%h %ci %s' -- mock/registry/dimension.toml)"
echo "  row landed:         $(git -C "$ROOT" log -1 --format='%h %ci %s' -S'a_law_layer_answers_whether_a_law_reaches' -- mock/registry/proposal.toml)"

echo
echo "=== CONTROL 1: the four axes 182 section 5.1 needs, which must all read NO,"
echo "    run beside two that must read yes so a NO is about the file ==="
for a in declared_operand_window representable_range_geometry encoding constant_embedding_convention toolchain build_profile; do
  printf '  %-34s %s\n' "$a" "$(grep -qE "^id = \"$a\"\$" "$D" && echo yes || echo NO)"
done
