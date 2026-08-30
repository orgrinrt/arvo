// Probe: is Growth derivable from the operation name, or can the two vary
// independently (which would mean the law's key genuinely needs both as
// separate slots)? Tested by making Growth an ASSOCIATED type of a marker
// per named operation, rather than a co-equal parameter alongside it. If
// that compiles and is total (every operation has exactly one Growth,
// with no way to name a mismatched pair), the two facts were never
// independent: the pairing was a relation with the shape of a function
// all along, and stating it as two co-equal key slots let the copies
// drift by construction (nothing stopped a caller writing an
// inconsistent pair when Growth was its own free-standing parameter).
#![allow(dead_code)]

// The two named operations this stretch of the review actually uses.
// Adding a third (say, "narrow-to-fewer-fractional-bits-with-rounding")
// is adding a marker + one impl line, not touching a cross product.
pub struct MulFull; // exact: widths add, quanta multiply, nothing dropped
pub struct MulThenQuantize; // composite: mul_full, then one named quantize call

// the OLD shape: Growth as a free-standing enum, paired with an operation
// tag by convention, with nothing stopping a mismatched pairing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GrowthOld {
    Exact,
    Narrowed,
}

// a law's OLD key, both slots present, independently suppliable:
pub const fn law_holds_old(op_is_mul_full: bool, growth: GrowthOld) -> bool {
    // interior safety only ever held for the exact primitive; the
    // "Narrowed" case fires a quantiser and is a different, weaker fact
    // (probe_2 of file 34: interior safety needs zero quantiser calls).
    op_is_mul_full && matches!(growth, GrowthOld::Exact)
}
// nothing in the type system stops this call. it type-checks, it runs,
// and it is a lie: MulFull paired with Narrowed does not correspond to
// anything the design can build (MulFull's own definition never narrows).
const IMPOSSIBLE_BUT_EXPRESSIBLE: bool = law_holds_old(true, GrowthOld::Narrowed);

// the NEW shape: Growth is not a parameter at all. it is an associated
// fact OF the operation marker, fixed at one value, with no separate
// slot for a caller to disagree with.
pub trait Op {
    const IS_EXACT: bool;
}
impl Op for MulFull {
    const IS_EXACT: bool = true;
}
impl Op for MulThenQuantize {
    const IS_EXACT: bool = false;
}

// the law's NEW key: one slot. Growth is not named because it cannot be
// named independently; asking for it would be asking a question the
// operation marker has already answered by construction.
pub const fn law_holds_new<O: Op>() -> bool {
    O::IS_EXACT
}

const _: () = assert!(law_holds_new::<MulFull>() == true);
const _: () = assert!(law_holds_new::<MulThenQuantize>() == false);

// the redundancy claim, stated as a theorem and checked at both points
// the old key could be called with a TRUTHFUL pairing (the only two the
// design's own operations produce): the old and new keys agree exactly
// where the pairing is one the design can actually build.
const _: () = assert!(law_holds_old(true, GrowthOld::Exact) == law_holds_new::<MulFull>());
const _: () =
    assert!(law_holds_old(false, GrowthOld::Narrowed) == law_holds_new::<MulThenQuantize>());

fn main() {
    println!(
        "law_holds_new::<MulFull>()         = {}",
        law_holds_new::<MulFull>()
    );
    println!(
        "law_holds_new::<MulThenQuantize>() = {}",
        law_holds_new::<MulThenQuantize>()
    );
    println!(
        "IMPOSSIBLE_BUT_EXPRESSIBLE (old key, mismatched pair) = {}",
        IMPOSSIBLE_BUT_EXPRESSIBLE
    );
    println!("the new key has no slot in which to spell that mismatch at all");
}
