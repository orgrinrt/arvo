// p2: the same shared parameter at two owners.
//
// Hypothesis: block floating point's shared exponent is not one thing. With the
// exponent owned by the TYPE (a const parameter), each instantiation is an ordinary
// format: Q is a constant of the type, membership and denotation are const fns of
// (type, bits), and every compile-time instrument applies. With the exponent owned by
// RUNTIME DATA (the block), the denotation question is forced to take the exponent as
// a runtime argument, visibly, in its signature: the (type, bits)-only spelling has no
// writable body because the information is not in scope.
//
// What this probe establishes is constructive and shape-level: ownership of a chain
// component can be moved between the type and the runtime, and the concept boundary
// (63:141-147, "a value set depending on other data has no Q") tracks the owner's
// resolution time, not the existence of sharing. What it does NOT establish: whether
// selecting among the per-exponent monomorphs by a runtime exponent is admissible.
// That selection is a runtime match over types, which is the dispatch-residue shape
// 68 section 5 names; this probe demonstrates the arity fact only.
//
// Bears on: 67 section 6 (shared parameters at two layers), 68 section 6 (the bottom
// tier as a pair of questions), OPTIONS.md Q26. Spike; shapes are scaffolding.

#![no_std]

// Arm A: exponent owned by the type. One type per block exponent.
pub struct Bf<const E: u8>;

impl<const E: u8> Bf<E> {
    // 4-bit mantissa; denotation: m * 2^E; Q = {0, 1, ..., 15} scaled by 2^E
    pub const Q_MAX_NUM: u32 = 15u32 << E;

    // membership and denotation: const fns of (type, bits) alone
    pub const fn valid(bits: u8) -> bool {
        bits <= 0x0F
    }
    pub const fn value_of(bits: u8) -> u32 {
        (bits as u32) << E
    }
}

// eight instantiations, eight distinct constant Qs; sampled assertions
const _: () = assert!(Bf::<0>::Q_MAX_NUM == 15);
const _: () = assert!(Bf::<3>::Q_MAX_NUM == 120);
const _: () = assert!(Bf::<7>::Q_MAX_NUM == 1920);
const _: () = assert!(Bf::<0>::Q_MAX_NUM != Bf::<1>::Q_MAX_NUM);
const _: () = assert!(Bf::<0>::valid(0x0F));
const _: () = assert!(!Bf::<0>::valid(0x10));
const _: () = assert!(Bf::<3>::value_of(0x05) == 40);

// Arm B: exponent owned by the aggregate, resolved at runtime.
pub struct BfRt;

impl BfRt {
    // the honest denotation signature carries the shared parameter at runtime:
    pub const fn value_of(bits: u8, block_exp: u8) -> u32 {
        (bits as u32) << block_exp
    }
    // A (type, bits)-only value_of for BfRt is not writable: the element's magnitude
    // is a function of data this type does not carry. The arity difference between
    // Arm A's value_of and this one is the finding, and it is visible in the
    // signatures rather than in a refusal transcript.
}
