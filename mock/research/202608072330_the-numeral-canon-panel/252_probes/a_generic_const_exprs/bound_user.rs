// Arm A3 consumer. No feature attribute. It calls the fn whose `where` clause
// carries the const expression, and forces it at check time.
#![no_std]

use bound_lib::joined;

pub const TOTAL: usize = joined::<3, 4>();
