//! Probe 4d: COMMITTED REFUSING, on purpose.
//!
//! The consumer side of the transfer rule, and the reason no consumer declares
//! a view. A caller whose own contract is a fold whose definedness matches the
//! sequential one takes a `Folded<0>`. Handed the `Precise` regrouping below
//! interior safety, whose type is `Folded<1>` because its law does not
//! preserve definedness, it is refused by ordinary type checking with no
//! bespoke machinery at all.
//!
//! This is the coeffect discharging into an effect: "I will put up with
//! grouping-dependent refusals" is not a permission anyone grants, it is a
//! property of a value's type, and a caller that cannot put up with it simply
//! does not typecheck. The permission-shaped-coercions-carry-no-data
//! asymmetry (`26:213-215`) is avoided rather than mitigated.
//!
//! Verbatim diagnostic recorded in `OUTCOMES.md`.

#[path = "probe_4_view_as_a_return_type_and_the_transfer.rs"]
mod mechanism;

use mechanism::{needs_faithful_definedness, PRECISE_BELOW};

pub const REFUSED: i32 = needs_faithful_definedness(PRECISE_BELOW);
