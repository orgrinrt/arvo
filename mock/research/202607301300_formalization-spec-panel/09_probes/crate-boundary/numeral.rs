//! crate `numeral`: minimal stand-in for arvo-numeral (D72's crate table).
#![crate_type = "rlib"]
#![crate_name = "numeral"]

pub trait Numeral {}

pub struct Fix13_3Signed;
impl Numeral for Fix13_3Signed {}

pub struct Fix0_8Unsigned;
impl Numeral for Fix0_8Unsigned {}
