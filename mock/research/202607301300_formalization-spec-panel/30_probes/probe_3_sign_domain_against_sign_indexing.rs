//! Probe 3: the sign axis is two independent axes, and file 28's own two
//! sections disagree about where the second one lives.
//!
//! File 28 §1 adopts the two-coordinate structure and states the crossing
//! contract as two round-trip theorems: "decode after encode is the identity
//! on data, and encode after decode is the identity on bit patterns"
//! (`28:84-86`). File 28 §2 then gives three reasons the second of those is
//! false: signed zero, NaN payloads, and decimal cohorts all make the
//! interpretation non-injective (`28:119-138`). The two sections were written
//! in one file and do not compose.
//!
//! File 28 §3 then proposes `Sign` with three instances, `Unsigned`,
//! `TwosComplement`, `SignMagnitude`, on the identity side, justified by "at
//! the same width, a two's-complement signed numeral represents an asymmetric
//! range with one zero, and a sign-magnitude signed numeral represents a
//! symmetric range with two zeros" (`28:182-185`). That sentence bundles two
//! facts of different kinds: the RANGE difference is about values, and the
//! ZERO COUNT is about data. Under §1's own coordinate split the first is
//! identity and the second is encoding, so the three-instance axis puts an
//! encoding fact back on the identity side, which is exactly what file 27's
//! inversion removed.
//!
//! This probe checks that the two are genuinely independent (the test for
//! whether a bundling is wrong) and that separating them derives something
//! the design currently names by hand.
//!
//! Model: magnitude precision P = 3, four data bits, so sixteen data.
//! Every claim is a `const` assertion; compiling is the check.

#![no_std]

const P: u32 = 3;
const MAG_MAX: i32 = (1 << P) - 1; // 7
const DATA_COUNT: i32 = 1 << (P + 1); // 16

/// The value set, an identity fact: which rationals are representable.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Domain {
    NonNegative,   // 0 ..= 7
    Symmetric,     // -7 ..= 7
    AsymmetricLow, // -8 ..= 7
}

/// How a datum indexes a value, an encoding fact: no value is added or
/// removed by changing it, only which datum carries which value, and whether
/// two data carry one.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Indexing {
    Unsigned,
    TwosComplement,
    SignMagnitude,
    OnesComplement,
}

const NONE: i32 = i32::MIN; // "this datum carries no value in this domain"

/// decode: datum -> value, or NONE where the datum is outside the domain.
const fn decode(d: i32, dom: Domain, ix: Indexing) -> i32 {
    let v = match ix {
        Indexing::Unsigned => d,
        Indexing::TwosComplement => {
            if d >= DATA_COUNT / 2 {
                d - DATA_COUNT
            } else {
                d
            }
        }
        Indexing::SignMagnitude => {
            let mag = d & MAG_MAX;
            if d >= DATA_COUNT / 2 {
                -mag
            } else {
                mag
            }
        }
        Indexing::OnesComplement => {
            if d >= DATA_COUNT / 2 {
                d - (DATA_COUNT - 1)
            } else {
                d
            }
        }
    };
    if in_domain(v, dom) {
        v
    } else {
        NONE
    }
}

const fn in_domain(v: i32, dom: Domain) -> bool {
    match dom {
        Domain::NonNegative => v >= 0 && v <= MAG_MAX,
        Domain::Symmetric => v >= -MAG_MAX && v <= MAG_MAX,
        Domain::AsymmetricLow => v >= -MAG_MAX - 1 && v <= MAG_MAX,
    }
}

/// encode: value -> the CANONICAL datum carrying it. Where more than one
/// datum carries a value, this picks one, which is the whole content of the
/// word "canonical" and the reason the second round-trip is not an identity.
const fn encode(v: i32, dom: Domain, ix: Indexing) -> i32 {
    let mut d = 0;
    while d < DATA_COUNT {
        if decode(d, dom, ix) == v {
            return d;
        }
        d += 1;
    }
    -1
}

// ---- claim 1: the two axes are independent ---------------------------------
//
// Every (domain, indexing) pair that can carry the domain does, and the pairs
// are not in bijection: one domain is served by three indexings, and one
// indexing serves two domains. A bundled axis cannot express that.

const fn domain_is_served(dom: Domain, ix: Indexing) -> bool {
    // every value of the domain has a datum
    let mut v = -MAG_MAX - 1;
    while v <= MAG_MAX {
        if in_domain(v, dom) && encode(v, dom, ix) < 0 {
            return false;
        }
        v += 1;
    }
    true
}

// Symmetric is served by all three signed indexings.
const _: () = assert!(domain_is_served(
    Domain::Symmetric,
    Indexing::TwosComplement
));
const _: () = assert!(domain_is_served(Domain::Symmetric, Indexing::SignMagnitude));
const _: () = assert!(domain_is_served(
    Domain::Symmetric,
    Indexing::OnesComplement
));

// AsymmetricLow is served by two's complement and by nothing else here,
// which is the one real coupling and is a well-formedness row, not an axis.
const _: () = assert!(domain_is_served(
    Domain::AsymmetricLow,
    Indexing::TwosComplement
));
const _: () = assert!(!domain_is_served(
    Domain::AsymmetricLow,
    Indexing::SignMagnitude
));
const _: () = assert!(!domain_is_served(
    Domain::AsymmetricLow,
    Indexing::OnesComplement
));

// Two's complement serves two different domains, so naming the indexing does
// not name the value set and naming the value set does not name the indexing.
// That is the test for whether bundling them into one axis loses information.
const _: () = assert!(domain_is_served(
    Domain::Symmetric,
    Indexing::TwosComplement
));
const _: () = assert!(domain_is_served(
    Domain::AsymmetricLow,
    Indexing::TwosComplement
));

// ---- claim 2: the crossing contract is a section-retraction pair -----------
//
// decode after encode IS the identity on values, always.
// encode after decode is NOT the identity on data whenever the indexing is
// non-injective, and IS idempotent, which is the correct weaker statement and
// is exactly what IEEE calls a canonical encoding and what decimal calls a
// preferred exponent.

const fn decode_after_encode_is_id_on_values(dom: Domain, ix: Indexing) -> bool {
    let mut v = -MAG_MAX - 1;
    while v <= MAG_MAX {
        if in_domain(v, dom) {
            let d = encode(v, dom, ix);
            if d < 0 || decode(d, dom, ix) != v {
                return false;
            }
        }
        v += 1;
    }
    true
}

const _: () = assert!(decode_after_encode_is_id_on_values(
    Domain::Symmetric,
    Indexing::SignMagnitude
));
const _: () = assert!(decode_after_encode_is_id_on_values(
    Domain::Symmetric,
    Indexing::TwosComplement
));
const _: () = assert!(decode_after_encode_is_id_on_values(
    Domain::AsymmetricLow,
    Indexing::TwosComplement
));
const _: () = assert!(decode_after_encode_is_id_on_values(
    Domain::NonNegative,
    Indexing::Unsigned
));

const fn encode_after_decode_is_id_on_data(dom: Domain, ix: Indexing) -> bool {
    let mut d = 0;
    while d < DATA_COUNT {
        let v = decode(d, dom, ix);
        if v != NONE && encode(v, dom, ix) != d {
            return false;
        }
        d += 1;
    }
    true
}

/// Injective indexings: the second round-trip holds, and file 28 §1 is right
/// about them.
const _: () = assert!(encode_after_decode_is_id_on_data(
    Domain::AsymmetricLow,
    Indexing::TwosComplement
));
const _: () = assert!(encode_after_decode_is_id_on_data(
    Domain::NonNegative,
    Indexing::Unsigned
));

/// Non-injective indexings: it fails, and these are the cases file 28 §2 says
/// the design must admit. Two data, one value, in the smallest possible
/// model: sign-magnitude and ones' complement each carry two zeros.
const _: () = assert!(!encode_after_decode_is_id_on_data(
    Domain::Symmetric,
    Indexing::SignMagnitude
));
const _: () = assert!(!encode_after_decode_is_id_on_data(
    Domain::Symmetric,
    Indexing::OnesComplement
));

/// The named witness, so the failure is a fact rather than a count.
const NEG_ZERO: i32 = DATA_COUNT / 2; // 0b1000
const _: () = assert!(decode(NEG_ZERO, Domain::Symmetric, Indexing::SignMagnitude) == 0);
const _: () = assert!(decode(0, Domain::Symmetric, Indexing::SignMagnitude) == 0);
const _: () = assert!(encode(0, Domain::Symmetric, Indexing::SignMagnitude) == 0);
const _: () = assert!(
    encode(
        decode(NEG_ZERO, Domain::Symmetric, Indexing::SignMagnitude),
        Domain::Symmetric,
        Indexing::SignMagnitude
    ) != NEG_ZERO
);

/// The weaker statement that DOES hold and should replace the second theorem:
/// encode after decode is idempotent, so it is a canonicalisation rather than
/// an identity, and applying it twice is applying it once.
const fn canonicalisation_is_idempotent(dom: Domain, ix: Indexing) -> bool {
    let mut d = 0;
    while d < DATA_COUNT {
        let v = decode(d, dom, ix);
        if v != NONE {
            let c = encode(v, dom, ix);
            let cv = decode(c, dom, ix);
            if cv == NONE || encode(cv, dom, ix) != c {
                return false;
            }
        }
        d += 1;
    }
    true
}

const _: () = assert!(canonicalisation_is_idempotent(
    Domain::Symmetric,
    Indexing::SignMagnitude
));
const _: () = assert!(canonicalisation_is_idempotent(
    Domain::Symmetric,
    Indexing::OnesComplement
));
const _: () = assert!(canonicalisation_is_idempotent(
    Domain::AsymmetricLow,
    Indexing::TwosComplement
));

// ---- claim 3: SystemC's SC_SAT_SYM is a numeral, not a resolution ----------
//
// Clamping toward negative delivers "the neighbour that exists below", which
// is the DOMAIN's minimum. Split the axes and the same `TowardNegative`
// marker delivers -8 over an asymmetric domain and -7 over a symmetric one.
// SystemC names the second a separate saturation mode; here it is the same
// mode over a different numeral, which is one fewer thing in `Policy`.

const fn domain_min(dom: Domain) -> i32 {
    match dom {
        Domain::NonNegative => 0,
        Domain::Symmetric => -MAG_MAX,
        Domain::AsymmetricLow => -MAG_MAX - 1,
    }
}

/// `TowardNegative` at the under-range position, as the spec already defines
/// clamping (`202607301200:159-163`).
const fn clamp_low(exact: i32, dom: Domain) -> i32 {
    if exact < domain_min(dom) {
        domain_min(dom)
    } else {
        exact
    }
}

const _: () = assert!(clamp_low(-20, Domain::AsymmetricLow) == -8); // SC_SAT
const _: () = assert!(clamp_low(-20, Domain::Symmetric) == -7); // SC_SAT_SYM
const _: () = assert!(clamp_low(-20, Domain::NonNegative) == 0);

// ---- what this does NOT show ----------------------------------------------
//
// It does not show where the design should put the indexing axis, only that
// the two facts are independent and that bundling them costs the ability to
// express a symmetric two's-complement numeral, which SystemC ships. It also
// does not address NaN payloads or decimal cohorts, which are the other two
// entrances to non-injectivity file 28 names; the sign case is used here
// because it is the one expressible in sixteen data.
