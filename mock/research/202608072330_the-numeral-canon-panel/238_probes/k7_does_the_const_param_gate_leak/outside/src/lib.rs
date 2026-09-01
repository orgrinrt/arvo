#![no_std]
//! A consumer with no `#![feature(...)]` of its own, naming the door's types.
//!
//! If this builds, the gate is contained. If it does not, the second option
//! moves the cost onto exactly the crate
//! `obligation::the_unstable_machinery_does_not_reach_a_consumer` protects.

use door::{Signed, Slots, Width};

/// Naming the type with a const argument of the door's own type.
pub type Eight = Signed<{ Width::bits(8) }>;

/// Reading the associated constant back off it.
pub const EIGHT: Width = <Eight as Slots>::WIDTH;

/// And generically over the parameter, which is the harder half: a consumer
/// writing its own function over a declared width rather than naming one.
pub const fn width_of<S: Slots>() -> Width {
    S::WIDTH
}

/// The answer, forced at const time.
pub const READ_BACK: u32 = width_of::<Eight>().count();

#[cfg(test)]
mod tests {
    use super::*;

    /// The value crossed the boundary, rather than merely the type name.
    ///
    /// A build that succeeds says the consumer compiles. It does not say the
    /// const argument survived monomorphisation with its value, and a door
    /// type that arrived as something else would still compile.
    #[test]
    fn the_declared_width_reaches_the_consumer_with_its_value() {
        assert_eq!(READ_BACK, 8);
        assert_eq!(EIGHT.count(), 8);
    }

    /// The control: a different declaration reads back differently.
    ///
    /// Without it the assertion above passes for any implementation that
    /// returns eight, including one that ignores the parameter.
    #[test]
    fn the_control_a_second_declaration_reads_back_as_itself() {
        type Thirteen = door::Signed<{ Width::bits(13) }>;
        assert_eq!(width_of::<Thirteen>().count(), 13);
        assert_ne!(width_of::<Thirteen>().count(), READ_BACK);
    }
}
