// probe 3: the two mechanisms the deliverable proposes on top of the total
// far-point projection.
//
// first, the grade-side join: FarPointKind joins through a fold, with the
// silent kind (Finite) dominating, so a fold's published grade records
// "somewhere in this computation, out-of-range resolves silently" the
// moment any operand's numeral has a finite far point. the join's laws
// (commutative, associative, idempotent, identity) are checked over the
// WHOLE two-element carrier, all eight triples, in const context.
//
// second, the consumer-side opt-in bound: a consumer that needs the in-band
// overflow witness (the absorbing far point) states that need as a bound,
// and the bound refuses a finite-far-point numeral at the call site. the
// refusal is probe 3b, a separate expected-fail file; this file shows the
// positive half compiles.
//
// expected: compiles clean, no feature gates, all const assertions hold.

pub struct NoSpecials;
pub struct NanOnly;
pub struct InfOnly;
pub struct IeeeSpecials;

pub trait Specials {
    const HAS_INF: bool;
}
impl Specials for NoSpecials {
    const HAS_INF: bool = false;
}
impl Specials for NanOnly {
    const HAS_INF: bool = false;
}
impl Specials for InfOnly {
    const HAS_INF: bool = true;
}
impl Specials for IeeeSpecials {
    const HAS_INF: bool = true;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FarPointKind {
    Absorbing, // infinity: reaching the far point is witnessed in the datum
    Finite,    // saturation: reaching the far point is silent in the datum
}

// the join: Finite dominates. Absorbing is the identity.
pub const fn join(a: FarPointKind, b: FarPointKind) -> FarPointKind {
    match (a, b) {
        (FarPointKind::Absorbing, FarPointKind::Absorbing) => FarPointKind::Absorbing,
        _ => FarPointKind::Finite,
    }
}

const fn keq(a: FarPointKind, b: FarPointKind) -> bool {
    matches!(
        (a, b),
        (FarPointKind::Absorbing, FarPointKind::Absorbing)
            | (FarPointKind::Finite, FarPointKind::Finite)
    )
}

// the whole carrier, every law, every instance: 4 commutativity pairs,
// 8 associativity triples, 2 idempotence points, 2 identity points.
const CARRIER: [FarPointKind; 2] = [FarPointKind::Absorbing, FarPointKind::Finite];
const _: () = {
    let mut i = 0;
    while i < 2 {
        let a = CARRIER[i];
        assert!(keq(join(a, a), a)); // idempotent
        assert!(keq(join(FarPointKind::Absorbing, a), a)); // identity
        let mut j = 0;
        while j < 2 {
            let b = CARRIER[j];
            assert!(keq(join(a, b), join(b, a))); // commutative
            let mut k = 0;
            while k < 2 {
                let c = CARRIER[k];
                assert!(keq(join(join(a, b), c), join(a, join(b, c)))); // associative
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
};

// the opt-in witness bound. sealed-shape marker: implemented exactly for
// the members whose far point is absorbing. a consumer that needs the
// in-band witness bounds on it; every other consumer never names it.
// this is the constructive replacement for a preset-mandated
// well-formedness refusal: the CONSUMER states the need, the substrate
// does not police the pairing.
pub trait AbsorbingFarPoint: Specials {}
impl AbsorbingFarPoint for InfOnly {}
impl AbsorbingFarPoint for IeeeSpecials {}

pub fn fold_needing_inband_witness<S: AbsorbingFarPoint>() {
    // stand-in for a consumer fold whose downstream reads the datum for
    // the overflow fact (the ieee idiom: check the result for inf).
}

fn main() {
    // positive half: both absorbing members are accepted.
    fold_needing_inband_witness::<InfOnly>();
    fold_needing_inband_witness::<IeeeSpecials>();
    // the join, at runtime, same values as the const check.
    println!(
        "join(Absorbing, Finite) = {:?} (silence dominates)",
        join(FarPointKind::Absorbing, FarPointKind::Finite)
    );
    println!("laws checked const, whole carrier, all eight triples");
}
