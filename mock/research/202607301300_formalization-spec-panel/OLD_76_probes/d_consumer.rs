//! D (consumer half). The consumer declares its own numerals through the
//! library's macro, which emits the predicate impl at the declaration site.
#![no_std]
#![feature(adt_const_params)]
use d_lib::*;
d_lib::declare_numeral!(Q13, 13, 0, Hot, yes);
d_lib::declare_numeral!(Q0F8, 0, 8, Hot, no);
pub fn one_ok()
where
    Q13: HasOne,
{
}
