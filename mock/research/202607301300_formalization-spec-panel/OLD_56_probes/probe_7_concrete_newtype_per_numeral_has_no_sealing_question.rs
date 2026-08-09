//! Probe 7: probe 6 found the const-generic face (`NFace<S>`) cannot carry
//! a structural seal, only an evaluative one, because the reducedness
//! bound would need `S.is_reduced()` computed FROM a generic const
//! parameter and placed in a bound, which is `generic_const_exprs` again,
//! the same wall the whole tower exists to avoid (`49:552-563`). This
//! probe checks the alternative the macro proposal already implies but
//! nobody has stated as the resolution to the sealing question
//! specifically: a CONCRETE, non-generic newtype per numeral, minted only
//! by the macro's own emission, with no consumer-visible constructor. If
//! nothing can name the type without going through the macro, there is no
//! sealing question left to ask, because there is no attacker position: a
//! consumer cannot spell a second, malformed `Q0_15`.
//!
//! EXPECTED: unknown going in, specifically whether the diagnostic on a
//! mismatch names the concrete newtype (as probe 1's const-generic form
//! did) with none of probe 4/6's sealing hazard.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   probe_7_concrete_newtype_per_numeral_has_no_sealing_question.rs

#![allow(dead_code)]

use core::marker::PhantomData;

// Stand-in for the sealed tower's encoding; not imported here because this
// probe is only about the face layer's own shape, which is orthogonal to
// what it wraps.
pub struct Enc<const V: u64>;

pub trait Numeral {
    type Encoding;
    const DISPLAY_VALUE: u64;
}

// What the macro emits per invocation. No pub constructor exists for
// either type; only the trait impl, which associates the name with an
// encoding. A consumer who writes `q15!(37)` gets this; a consumer who
// tries to hand-write `struct Q37;` themselves produces a DIFFERENT,
// unrelated type with no `Numeral` impl at all, refused wherever a
// `Numeral`-bounded position needs `DISPLAY_VALUE` or `Encoding`, not
// because anything was sealed, but because nothing implemented the trait.
pub struct Q37;
impl Numeral for Q37 {
    type Encoding = Enc<37>;
    const DISPLAY_VALUE: u64 = 37;
}

pub struct Q53;
impl Numeral for Q53 {
    type Encoding = Enc<53>;
    const DISPLAY_VALUE: u64 = 53;
}

pub struct Container<N: Numeral>(PhantomData<N>);

pub fn needs_q37(_: Container<Q37>) {}
pub fn give_q53(x: Container<Q53>) {
    needs_q37(x);
}
