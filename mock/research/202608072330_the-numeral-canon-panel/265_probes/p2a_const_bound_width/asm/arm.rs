//! The arm from p2a, standalone so it can be lowered per target with bare rustc.
#![no_std]
#[must_use]
pub const fn arm() -> u32 {
    match usize::BITS {
        16 => 1,
        32 => 2,
        64 => 3,
        _ => 0,
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn observe_arm() -> u32 {
    arm()
}
