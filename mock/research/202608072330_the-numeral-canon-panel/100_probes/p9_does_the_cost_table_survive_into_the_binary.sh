#!/usr/bin/env bash
# p9. Does the cost table cost any BYTES in the shipped artifact.
#
# `p3` establishes that the cost-table encoding and the winner-table encoding
# emit byte-identical bodies at the point of use, to the point of being merged
# into one symbol. That answers the instruction-count question and leaves an
# adjacent one nobody asked: the cost table is |R| * |A| * |D| numbers, and the
# winner table is |R| numbers. If both survive into the object, the cost-table
# encoding carries a per-crate size cost that scales with the arm count and the
# coordinate count.
#
# `arvo-compile-time-last.md` licenses paying compile time for a runtime or
# correctness win. It says nothing about paying binary size, and a substrate
# whose consumers bitpack columns to save bytes would notice.
#
# This is a STATIC fact about an emitted object, not a timing, so an ad-hoc
# spike may settle it. It is not a bench and no measurement is taken.
#
# THE CONTROL matters more than the measurement. An absent section proves
# nothing on its own: a table nothing references is dead however it is written.
# So a control is built in which the same table IS read at runtime, and it must
# show the section that the real file must not.
#
# Run:  ./p9_does_the_cost_table_survive_into_the_binary.sh

set -u
cd "$(dirname "$0")"

RUSTC="rustc --edition 2024 -O -C panic=abort"
echo "toolchain: $(rustc --version)"
echo "target:    $(rustc -vV | sed -n 's/^host: //p')"
echo

echo "=== the real file: cost table read only at const time ==="
$RUSTC --emit obj -o /tmp/p9_real.o p3_three_encodings.rs 2>&1 | head -3
echo "sections:"
size -m /tmp/p9_real.o | sed 's/^/  /'
echo "symbols mentioning COST or WINNER:"
nm /tmp/p9_real.o | grep -iE 'cost|winner' | sed 's/^/  /' || echo "  none"
echo "object bytes: $(wc -c < /tmp/p9_real.o)"
echo

echo "=== the control: same table, one runtime read, so it MUST survive ==="
# One added function indexes COST with a runtime argument. Nothing else changes.
cat p3_three_encodings.rs > /tmp/p9_control.rs
cat >> /tmp/p9_control.rs <<'EOF'

/// Control. Reads the cost table with a runtime index, so the table cannot be
/// consumed at const time and must appear in the emitted object. If this
/// control shows no constant section either, the comparison above is vacuous
/// and proves nothing about const consumption.
#[unsafe(no_mangle)]
pub fn control_runtime_read(r: usize, a: usize, k: usize) -> u32 {
    COST[r % R][a % A][k % D]
}
EOF
$RUSTC --emit obj -o /tmp/p9_control.o /tmp/p9_control.rs 2>&1 | head -3
echo "sections:"
size -m /tmp/p9_control.o | sed 's/^/  /'
echo "object bytes: $(wc -c < /tmp/p9_control.o)"
echo

echo "=== verdict ==="
REAL_CONST=$(size -m /tmp/p9_real.o | grep -cE '__const|__data|__rodata' || true)
CTRL_CONST=$(size -m /tmp/p9_control.o | grep -cE '__const|__data|__rodata' || true)
echo "  constant-data sections in the real object   : $REAL_CONST"
echo "  constant-data sections in the control object: $CTRL_CONST"
if [ "$REAL_CONST" -eq 0 ] && [ "$CTRL_CONST" -gt 0 ]; then
    echo "  The cost table is consumed at const time and carries ZERO bytes."
    echo "  The control shows the section it would occupy if it were not."
elif [ "$CTRL_CONST" -eq 0 ]; then
    echo "  THE CONTROL FAILED. It shows no constant section either, so the"
    echo "  real object's absence proves nothing and this probe is vacuous."
else
    echo "  The cost table survives into the emitted object; the encoding has a"
    echo "  per-crate size cost that the winner table does not."
fi
