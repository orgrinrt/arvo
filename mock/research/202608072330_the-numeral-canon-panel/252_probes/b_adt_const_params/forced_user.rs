// Arm B, the forcing control. `user.rs` compiled, and a compile alone does not
// show the ADT const parameter was ever evaluated in the consumer. This forces
// it twice: once with the right answer, which must build, and once with the
// wrong one, which must not.
//
// Build this file as-is for the positive half. The negative half is the same
// file with `WRONG` uncommented, which `run.sh` does by generating it.
#![no_std]

use door::{Signed, Width};

pub type Thirteen = Signed<{ Width::bits(13) }>;

const _: () = assert!(Thirteen::DECLARED == 13);
