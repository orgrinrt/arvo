//! P8. When is an encoding observable, and why do two panel artifacts appear
//! to disagree about it?
//!
//! PHASE TWO probe. Unlike P0 to P7 this one was built after reading the
//! panel, and it exists to test a reconciliation rather than to derive
//! anything blind.
//!
//! `63` section 3.5 says the encoding "is not part of identity and it is
//! observable", citing raw-order agreement and raw-adder correctness as
//! pattern-level properties an encoding buys or forfeits. `110`'s F2 says
//! every pure code assignment is presentation, and its criterion quotients
//! four encodings out of identity automatically, including a structureless
//! bijection.
//!
//! Read quickly those look like a conflict. They are not: both say the
//! encoding is not part of identity. What differs is the LEVEL at which each
//! looks for observability, and the hypothesis this probe tests names the
//! discriminator exactly:
//!
//!   H. An encoding is observable through an operation exactly when that
//!      operation is defined on the REPRESENTATION rather than on the
//!      DENOTATION. Under a signature of denotation-defined operations only,
//!      every bijective encoding of one value set gives the same algebra.
//!      Add one representation-defined operation and they separate.
//!
//! If H holds, `110`'s quotient is a consequence of its signature containing
//! only denotation-defined operations, `63`'s observability is a consequence
//! of naming two representation-defined ones, and neither is wrong.
//!
//! The probe also re-runs `63`'s reported figures for two's complement and
//! offset binary at W = 4, independently, since a re-run either raises their
//! rung or produces a finding.
//!
//! No feature gates. `std` used only by the test harness.
//!
//! Build: rustc --edition 2021 --test -O p8_when_an_encoding_is_observable.rs

#![allow(dead_code)]

/// The value set: signed, four bits, -8..=7. One value set throughout; only
/// the encoding moves.
const W: u32 = 4;
const CARD: i32 = 1 << W; // 16
const LO: i32 = -(1 << (W - 1)); // -8
const HI: i32 = (1 << (W - 1)) - 1; // 7
const K: i32 = 1 << (W - 1); // 8, the excess-K bias

/// An encoding is a bijection between the value set and the pattern set
/// `0..CARD`. Nothing about it is required to preserve order or structure.
trait Encoding {
    const NAME: &'static str;
    fn encode(v: i32) -> u32;
    fn decode(p: u32) -> i32;
}

/// Two's complement: the pattern is the value's low W bits.
struct TwosComplement;
impl Encoding for TwosComplement {
    const NAME: &'static str = "two's complement";
    fn encode(v: i32) -> u32 {
        (v & (CARD - 1)) as u32
    }
    fn decode(p: u32) -> i32 {
        let p = p as i32;
        if p >= K { p - CARD } else { p }
    }
}

/// Offset binary, excess-K: the pattern is the value plus the bias.
struct OffsetBinary;
impl Encoding for OffsetBinary {
    const NAME: &'static str = "offset binary";
    fn encode(v: i32) -> u32 {
        (v + K) as u32
    }
    fn decode(p: u32) -> i32 {
        p as i32 - K
    }
}

/// Gray code over the two's-complement pattern. Order-destroying and
/// structure-destroying, and included because it is the case a reader expects
/// to break something.
struct Gray;
impl Encoding for Gray {
    const NAME: &'static str = "gray over two's complement";
    fn encode(v: i32) -> u32 {
        let p = TwosComplement::encode(v);
        p ^ (p >> 1)
    }
    fn decode(mut p: u32) -> i32 {
        let mut shift = 1;
        while shift < W {
            p ^= p >> shift;
            shift <<= 1;
        }
        TwosComplement::decode(p & (CARD as u32 - 1))
    }
}

/// A structureless bijection, `i -> (7i + 3) mod 16`, with 7 coprime to 16.
/// Included because if the criterion depends on the encoding having any
/// structure at all, this is what exposes it.
struct Arbitrary;
impl Encoding for Arbitrary {
    const NAME: &'static str = "arbitrary bijection 7i+3";
    fn encode(v: i32) -> u32 {
        let p = TwosComplement::encode(v) as i32;
        ((7 * p + 3).rem_euclid(CARD)) as u32
    }
    fn decode(p: u32) -> i32 {
        // 7 * 7 = 49 = 3*16 + 1, so 7 is its own inverse mod 16.
        let q = (7 * (p as i32 - 3)).rem_euclid(CARD);
        TwosComplement::decode(q as u32)
    }
}

// ---------------------------------------------------------------------------
// Two kinds of operation. This is the whole experiment.
// ---------------------------------------------------------------------------

/// DENOTATION-DEFINED. Decode, compute on the values, encode. The encoding
/// appears only as a round trip.
fn add_denot<E: Encoding>(x: u32, y: u32) -> u32 {
    let v = E::decode(x) + E::decode(y);
    let v = if v > HI {
        HI
    } else if v < LO {
        LO
    } else {
        v
    };
    E::encode(v)
}

fn le_denot<E: Encoding>(x: u32, y: u32) -> bool {
    E::decode(x) <= E::decode(y)
}

/// REPRESENTATION-DEFINED. Operate on the bit patterns, without decoding.
/// This is what `63` calls raw-order agreement and raw-adder correctness, and
/// it is what a bitpacked column or a sort key actually does.
fn le_raw(x: u32, y: u32) -> bool {
    x <= y
}

fn add_raw(x: u32, y: u32) -> u32 {
    (x + y) & (CARD as u32 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patterns() -> impl Iterator<Item = u32> {
        0..(CARD as u32)
    }

    /// Every encoding here is actually a bijection. Without this the rest of
    /// the probe could be measuring a broken encode/decode pair rather than
    /// an encoding property.
    #[test]
    fn every_encoding_is_a_bijection() {
        fn check<E: Encoding>() {
            let mut seen = [false; 16];
            for v in LO..=HI {
                let p = E::encode(v);
                assert!(p < CARD as u32, "{}: pattern out of range at {v}", E::NAME);
                assert!(!seen[p as usize], "{}: not injective at {v}", E::NAME);
                seen[p as usize] = true;
                assert_eq!(E::decode(p), v, "{}: round trip failed at {v}", E::NAME);
            }
            assert!(seen.iter().all(|&b| b), "{}: not surjective", E::NAME);
        }
        check::<TwosComplement>();
        check::<OffsetBinary>();
        check::<Gray>();
        check::<Arbitrary>();
    }

    /// H, first half. Under DENOTATION-defined operations, all four encodings
    /// give the same algebra: for every pair of values, the result value is
    /// the same, and the order relation is the same. Exhaustive, 256 pairs
    /// per encoding pair.
    #[test]
    fn denotation_defined_operations_cannot_see_the_encoding() {
        fn compare<A: Encoding, B: Encoding>() -> (u32, u32) {
            let mut checked = 0u32;
            let mut differ = 0u32;
            for a in LO..=HI {
                for b in LO..=HI {
                    let ra = A::decode(add_denot::<A>(A::encode(a), A::encode(b)));
                    let rb = B::decode(add_denot::<B>(B::encode(a), B::encode(b)));
                    let la = le_denot::<A>(A::encode(a), A::encode(b));
                    let lb = le_denot::<B>(B::encode(a), B::encode(b));
                    if ra != rb || la != lb {
                        differ += 1;
                    }
                    checked += 1;
                }
            }
            (differ, checked)
        }
        for (name, (differ, checked)) in [
            ("twos vs offset", compare::<TwosComplement, OffsetBinary>()),
            ("twos vs gray", compare::<TwosComplement, Gray>()),
            ("twos vs arbitrary", compare::<TwosComplement, Arbitrary>()),
            ("gray vs arbitrary", compare::<Gray, Arbitrary>()),
        ] {
            assert_eq!(checked, 256);
            assert_eq!(
                differ, 0,
                "{name}: a denotation-defined operation must not see the encoding"
            );
        }
    }

    /// H, second half. Under a REPRESENTATION-defined operation the same four
    /// encodings separate immediately. This is the arm that makes the first
    /// half a finding rather than a demonstration: without it, zero
    /// differences everywhere would be equally consistent with the probe
    /// being unable to detect a difference at all.
    #[test]
    fn representation_defined_operations_do_see_the_encoding() {
        fn raw_order_disagreements<E: Encoding>() -> (u32, u32) {
            let mut bad = 0u32;
            let mut checked = 0u32;
            for a in LO..=HI {
                for b in LO..=HI {
                    if le_raw(E::encode(a), E::encode(b)) != (a <= b) {
                        bad += 1;
                    }
                    checked += 1;
                }
            }
            (bad, checked)
        }
        fn raw_adder_correct<E: Encoding>() -> (u32, u32) {
            let mut good = 0u32;
            let mut checked = 0u32;
            for a in LO..=HI {
                for b in LO..=HI {
                    // Correct exactly when the raw pattern addition equals
                    // the encoding of the wrapped exact sum.
                    let exact = {
                        let s = a + b;
                        let m = CARD;
                        ((s - LO).rem_euclid(m)) + LO
                    };
                    if add_raw(E::encode(a), E::encode(b)) == E::encode(exact) {
                        good += 1;
                    }
                    checked += 1;
                }
            }
            (good, checked)
        }

        let tc_order = raw_order_disagreements::<TwosComplement>();
        let ob_order = raw_order_disagreements::<OffsetBinary>();
        let gr_order = raw_order_disagreements::<Gray>();
        let ar_order = raw_order_disagreements::<Arbitrary>();
        let tc_add = raw_adder_correct::<TwosComplement>();
        let ob_add = raw_adder_correct::<OffsetBinary>();
        let gr_add = raw_adder_correct::<Gray>();
        let ar_add = raw_adder_correct::<Arbitrary>();

        println!(
            "encoding                     raw-order disagreements   raw-adder correct"
        );
        for (n, o, a) in [
            (TwosComplement::NAME, tc_order, tc_add),
            (OffsetBinary::NAME, ob_order, ob_add),
            (Gray::NAME, gr_order, gr_add),
            (Arbitrary::NAME, ar_order, ar_add),
        ] {
            println!("{n:28}  {:>10}/{:<10}   {:>6}/{}", o.0, o.1, a.0, a.1);
        }

        // The separation is the finding: the four encodings do NOT agree here.
        let orders = [tc_order.0, ob_order.0, gr_order.0, ar_order.0];
        let adds = [tc_add.0, ob_add.0, gr_add.0, ar_add.0];
        assert!(
            orders.iter().any(|&x| x != orders[0]) || adds.iter().any(|&x| x != adds[0]),
            "if every encoding scored identically here, H would be refuted"
        );
    }

    /// The independent re-run of `63` section 3.5's reported figures. It says
    /// "two's complement 256 of 256 adder-correct and order-disagreeing,
    /// offset binary order-agreeing and 0 of 256 adder-correct with constant
    /// defect 8". Each clause is asserted separately so a partial match is
    /// visible as a partial match rather than as a pass or a fail.
    #[test]
    fn the_reported_figures_reproduce() {
        // Two's complement: adder-correct on all 256.
        let mut tc_add_ok = 0u32;
        let mut tc_order_ok = 0u32;
        // Offset binary: order-agreeing on all 256, adder-correct on none,
        // with a constant defect.
        let mut ob_add_ok = 0u32;
        let mut ob_order_ok = 0u32;
        let mut ob_defects = std::collections::BTreeSet::new();
        for a in LO..=HI {
            for b in LO..=HI {
                let exact = ((a + b - LO).rem_euclid(CARD)) + LO;

                if add_raw(TwosComplement::encode(a), TwosComplement::encode(b))
                    == TwosComplement::encode(exact)
                {
                    tc_add_ok += 1;
                }
                if le_raw(TwosComplement::encode(a), TwosComplement::encode(b)) == (a <= b) {
                    tc_order_ok += 1;
                }

                let ob_got = add_raw(OffsetBinary::encode(a), OffsetBinary::encode(b));
                let ob_want = OffsetBinary::encode(exact);
                if ob_got == ob_want {
                    ob_add_ok += 1;
                }
                ob_defects.insert(
                    (ob_got as i32 - ob_want as i32).rem_euclid(CARD),
                );
                if le_raw(OffsetBinary::encode(a), OffsetBinary::encode(b)) == (a <= b) {
                    ob_order_ok += 1;
                }
            }
        }
        println!(
            "re-run at W=4: twos adder-correct {tc_add_ok}/256, order-agreeing \
             {tc_order_ok}/256; offset adder-correct {ob_add_ok}/256, \
             order-agreeing {ob_order_ok}/256, defect set {:?}",
            ob_defects
        );
        assert_eq!(tc_add_ok, 256, "two's complement: adder-correct on all pairs");
        assert!(tc_order_ok < 256, "two's complement: must disagree on order somewhere");
        assert_eq!(ob_order_ok, 256, "offset binary: order-agreeing on all pairs");
        assert_eq!(ob_add_ok, 0, "offset binary: adder-correct on no pair");
        assert_eq!(
            ob_defects.len(),
            1,
            "offset binary: the defect must be CONSTANT, which is the claim"
        );
        assert_eq!(
            *ob_defects.iter().next().unwrap(),
            K,
            "offset binary: the constant defect must be the bias K = {K}"
        );
    }

    /// And the exclusivity `63` reports, stated as the property rather than
    /// as two separate scores: over these four encodings, none has both raw
    /// order agreement and raw adder correctness.
    #[test]
    fn no_encoding_here_has_both_pattern_level_properties() {
        fn both<E: Encoding>() -> bool {
            let mut order_ok = true;
            let mut add_ok = true;
            for a in LO..=HI {
                for b in LO..=HI {
                    let exact = ((a + b - LO).rem_euclid(CARD)) + LO;
                    if le_raw(E::encode(a), E::encode(b)) != (a <= b) {
                        order_ok = false;
                    }
                    if add_raw(E::encode(a), E::encode(b)) != E::encode(exact) {
                        add_ok = false;
                    }
                }
            }
            order_ok && add_ok
        }
        assert!(!both::<TwosComplement>());
        assert!(!both::<OffsetBinary>());
        assert!(!both::<Gray>());
        assert!(!both::<Arbitrary>());
        // This is four bijections, not a proof over all of them. `63` reports
        // the uniqueness argument as ONE EXPERT and argued for bijections
        // only; nothing here widens that.
    }
}
