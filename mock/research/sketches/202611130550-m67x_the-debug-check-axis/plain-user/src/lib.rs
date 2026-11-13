//! Wants the default: a sum leaving the range aborts in its debug build.

#![no_std]

/// Fifteen plus one through the feature arm.
pub fn by_feature() -> u8 {
    value::by_feature::U4::new(15)
        .add(value::by_feature::U4::new(1))
        .get()
}

/// Fifteen plus one through the parameter arm, at its default.
pub fn by_parameter() -> u8 {
    let a: value::by_parameter::U4 = value::by_parameter::U4::new(15);
    a.add(value::by_parameter::U4::new(1)).get()
}
