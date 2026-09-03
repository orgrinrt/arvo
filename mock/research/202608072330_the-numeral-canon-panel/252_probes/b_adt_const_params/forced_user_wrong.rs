// Arm B, the negative half. Identical but for the asserted value. This MUST
// fail, or the assertion in the positive half was never evaluated and arm B
// proves nothing about whether the consumer really resolves the ADT parameter.
#![no_std]

use door::{Signed, Width};

pub type Thirteen = Signed<{ Width::bits(13) }>;

const _: () = assert!(Thirteen::DECLARED == 12);
