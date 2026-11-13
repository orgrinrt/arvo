//! Wants completion as the meaning: no abort.

#![no_std]

/// Fifteen plus one through the parameter arm, quiet by its type.
pub fn by_parameter() -> u8 {
    let a: value::by_parameter::U4<value::Quiet> = value::by_parameter::U4::new(15);
    a.add(value::by_parameter::U4::new(1)).get()
}
