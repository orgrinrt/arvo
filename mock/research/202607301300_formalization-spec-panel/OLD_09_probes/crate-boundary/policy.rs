//! crate `policy`: minimal stand-in for arvo-policy (D72's crate table).
//! Carries `Resolution` and `Policy` with an `OverRange: Resolution` member,
//! exactly the axis 01/02's `AddAssoc` blanket conditions on.
#![crate_type = "rlib"]
#![crate_name = "policy"]

pub trait Resolution {}

pub struct ReduceModulo;
impl Resolution for ReduceModulo {}

pub struct SubstituteZero;
impl Resolution for SubstituteZero {}

pub struct Refuse;
impl Resolution for Refuse {}

pub trait Policy {
    type OverRange: Resolution;
}

pub struct Warm;
impl Policy for Warm {
    type OverRange = ReduceModulo;
}

pub struct Hot;
impl Policy for Hot {
    // deliberately the unstable-under-substitution rule (01 finding 1's
    // counterexample), so the fact really does have a false case to refuse.
    type OverRange = SubstituteZero;
}

pub struct Precise;
impl Policy for Precise {
    type OverRange = Refuse;
}
