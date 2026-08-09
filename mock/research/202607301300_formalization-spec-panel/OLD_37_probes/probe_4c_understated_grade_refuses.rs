//! Probe 4c: COMMITTED REFUSING, on purpose.
//!
//! `Precise` below interior safety does not preserve definedness across
//! groupings. A combinator that regroups it must publish that in its own
//! result grade. This one declares `Folded<0>`, which claims the regrouping is
//! faithful in every generator class, and the const assertion refuses it.
//!
//! This is what makes the transfer rule a mechanism rather than a discipline:
//! the published grade is declared (it cannot be computed in return position,
//! which is the wall at `26:719-724`) and the declaration is checked against
//! the law, so understating it is a compile error rather than a convention
//! someone maintains.
//!
//! Verbatim diagnostic recorded in `OUTCOMES.md`.

#[path = "probe_4_view_as_a_return_type_and_the_transfer.rs"]
mod mechanism;

use mechanism::{regroup_fold, Folded};

pub const PRECISE_CLAIMING_FAITHFUL: Folded<0> = regroup_fold::<0, 0, 1, 4, 0, 0>([1, 2, 3, 4]);
