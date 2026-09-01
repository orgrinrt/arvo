//! A third arm: what the test would have to say to reach all ten.
//!
//! `Ternary` declares a format and borrows the crate's own `DecimalRationals`,
//! `Constant<-1>` and `Signed<3>` for the other three coordinates, so it writes
//! two of the ten associated constants and none of the other eight. A format
//! genuinely from outside supplies all four traits itself, which is what the
//! ratified clause means by supplying the concept's obligations.

/// Every trait an outside format implements, none of them borrowed.
pub const WHOLE_FORMAT: &str = r#"
struct TernaryDomain;
impl Ambient for TernaryDomain {
    const RADIX: u32 = 3;
    const SIGNED: bool = true;
}

struct ThreeToTheMinusOne;
impl Quantum for ThreeToTheMinusOne {
    const BASE: i32 = -1;
    const SLOPE: i32 = 0;
    const MAGNITUDES: u32 = 1;
}

struct ThreeTrits;
impl Slots for ThreeTrits {
    const MIN: i64 = -13;
    const MAX: i64 = 13;
    const WIDTH: Width = Width::bits(5);
}

struct Ternary;
impl Format for Ternary {
    type Ambient = TernaryDomain;
    type Quantum = ThreeToTheMinusOne;
    type Slots = ThreeTrits;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
}

struct TernaryAdd;
impl Operation for TernaryAdd {
    type Signature = Signature<Ternary, Adapt<HalfEven, Saturate>>;
    const ARITY: u32 = 2;
}
"#;
