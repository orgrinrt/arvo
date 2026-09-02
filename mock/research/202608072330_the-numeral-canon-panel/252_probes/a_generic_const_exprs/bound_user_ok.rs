// A3 control, positive half: 6 + 7 = 13, which is implemented. Must build.
#![no_std]
use bound_lib2::joined;
pub const TOTAL: usize = joined::<6, 7>();
