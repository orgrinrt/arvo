// Option Z: keep the slot index in a signed 64-bit integer and restate the range
// so the count or the upper end is not a slot index. Two spellings the canon names
// (`ruling::a_platform_width_type_is_a_target_bound_member_of_the_format_family`,
// "restates the range as a count or an exclusive bound"), each with the count in
// the widest unsigned host integer of the same size.
//
// Every arm is selected by one `--cfg`, so each refusal and each control differs
// from the others in exactly one declaration.

#![no_std]
#![allow(dead_code)]

pub struct Slot(i64);
pub struct Count(u64);

/// The range as its lowest index and how many slots follow it.
pub trait Counted {
    const MIN: Slot;
    const COUNT: Count;
}

/// The range as its lowest index and one past its highest.
pub trait Exclusive {
    const MIN: Slot;
    const END: Slot;
}

pub struct Unsigned64;
pub struct Signed64;
pub struct Unsigned63;
pub struct Signed63;

// Refused: the count of a signed 64-bit range is 2^64, one past `u64::MAX`.
#[cfg(signed_64_counted)]
impl Counted for Signed64 {
    const COUNT: Count = Count(18446744073709551616);
    const MIN: Slot = Slot(i64::MIN);
}

// Refused: the count of an unsigned 64-bit range is 2^64 too.
#[cfg(unsigned_64_counted)]
impl Counted for Unsigned64 {
    const COUNT: Count = Count(18446744073709551616);
    const MIN: Slot = Slot(0);
}

// Refused: the exclusive end of an unsigned 64-bit range is 2^64, and no signed
// 64-bit index holds it; nor would the inclusive end, 2^64 - 1.
#[cfg(unsigned_64_exclusive)]
impl Exclusive for Unsigned64 {
    const END: Slot = Slot(18446744073709551616);
    const MIN: Slot = Slot(0);
}

// Refused: the exclusive end of a signed 64-bit range is 2^63.
#[cfg(signed_64_exclusive)]
impl Exclusive for Signed64 {
    const END: Slot = Slot(9223372036854775808);
    const MIN: Slot = Slot(i64::MIN);
}

// The controls, one width down. Both restatements reach 63 where the inclusive
// pair with a signed 64-bit count stopped at 62, which is the whole of what they
// buy.
#[cfg(controls)]
impl Counted for Unsigned63 {
    const COUNT: Count = Count(9223372036854775808);
    const MIN: Slot = Slot(0);
}

#[cfg(controls)]
impl Counted for Signed63 {
    const COUNT: Count = Count(9223372036854775808);
    const MIN: Slot = Slot(-4611686018427387904);
}

#[cfg(controls)]
impl Exclusive for Signed63 {
    const END: Slot = Slot(4611686018427387904);
    const MIN: Slot = Slot(-4611686018427387904);
}
