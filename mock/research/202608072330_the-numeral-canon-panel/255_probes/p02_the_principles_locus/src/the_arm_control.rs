//! The positive control for both arms.
//!
//! Identical invocation, identical shape, the type changed to one `arvo-format`
//! exports and that is not zero sized. It compiles, so the refusals beside it
//! are facts about `IFixed` and about `UFixed`'s size and not about how rustc
//! was called.

use arvo_format::Width;
use core::mem::size_of;

pub struct ErrorCode(Width);

pub const THE_FIELD_HOLDS_A_CODE: () = {
    assert!(size_of::<ErrorCode>() == 4);
};
