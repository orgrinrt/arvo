#!/usr/bin/env bash
# p3 negatives. Each is generated from the committed positive file by appending
# one item, so the diff between a positive and a negative is visible and small.
# Run from this directory. Writes p3_n*.rs and p3_n*.stderr beside itself.
set -u
cd "$(dirname "$0")"
mkdir -p out

BASE=p3_membership_contract.rs

# ---------------------------------------------------------------------------
# n1. The conjunction refuses the collapsed term, which the coherence-only
#     bound accepted in the positive file. This is H3.
# ---------------------------------------------------------------------------
cp "$BASE" p3_n1.rs
cat >> p3_n1.rs <<'EOF'

// NEGATIVE n1: the conjunction bound applied to the collapsed term. Its
// reduction is honestly Coherent; its ambient domain does not associate.
pub fn n1(xs: &[u8]) -> u8 {
    reassociating_fold::<IdentCollapsed>(xs)
}
EOF
rustc --edition 2024 --crate-type lib --out-dir out p3_n1.rs > p3_n1.stderr 2>&1
echo "n1 EXIT=$?" >> p3_n1.stderr

# ---------------------------------------------------------------------------
# n2. A reduction declared over one ambient cannot be read as a term over
#     another. The dependency is enforced, not asserted.
# ---------------------------------------------------------------------------
cp "$BASE" p3_n2.rs
cat >> p3_n2.rs <<'EOF'

// NEGATIVE n2: attach the GF(2) reach to the reduction declared over the
// integer window. The two coordinates are not independently choosable.
pub struct MismatchedReduce;
impl Reduce for MismatchedReduce {
    type Over = Gf2Full;
    fn adapt(e: i16) -> u8 {
        (e & 0x0F) as u8
    }
}
EOF
rustc --edition 2024 --crate-type lib --out-dir out p3_n2.rs > p3_n2.stderr 2>&1
echo "n2 EXIT=$?" >> p3_n2.stderr

# ---------------------------------------------------------------------------
# n3. The closure fact is computed, not declared: asserting that integer
#     addition is closed on the window fails at compile time.
# ---------------------------------------------------------------------------
cp "$BASE" p3_n3.rs
cat >> p3_n3.rs <<'EOF'

// NEGATIVE n3: claim the ambient operation is closed where it is not.
const _: () = assert!(add_closed_on_window());
EOF
rustc --edition 2024 --crate-type lib --out-dir out p3_n3.rs > p3_n3.stderr 2>&1
echo "n3 EXIT=$?" >> p3_n3.stderr

rm -rf out
echo "done. transcripts: p3_n1.stderr p3_n2.stderr p3_n3.stderr"
