// Arm A3 discriminating control. Identical, feature on in the consumer.
#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use bound_lib::joined;

pub const TOTAL: usize = joined::<3, 4>();
