// Arm B control. Same consumer, same dependency, same absent feature, naming
// only the item with no ADT const parameter in it.
#![no_std]

use door::{Width, plain};

pub const DECLARED: u32 = plain(Width::bits(13));
