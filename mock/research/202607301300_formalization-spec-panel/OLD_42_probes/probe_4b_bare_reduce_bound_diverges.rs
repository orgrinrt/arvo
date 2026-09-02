//! Probe 4b: the bare trait bound, unmodified in substance from file
//! 41's own probe 2(b), re-run here to confirm it independently (not
//! trusted from file 41's own outcome file) and to isolate it against
//! probe 4's positive control (the identical facts, spelled as
//! individual bounds rather than through `Reduce`, compile clean).
//!
//! One additional negative control run during this dispatch and not
//! committed as a fourth file: dropping the `: Pos` bound on `Reduce`'s
//! own associated types (`type N: Pos` -> `type N;`) does NOT change the
//! outcome, ruling out "eager well-formedness checking of the declared
//! associated-type bound" as the mechanism. The trigger is naming
//! `Reduce` as a bound at all, independent of what its associated types
//! are declared to satisfy.
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_4b_bare_reduce_bound_diverges.rs --out-dir <dir>
//! Outcome: FAILS WITH E0275, verbatim in OUTCOMES.md, matching file
//! 41's own probe 2(b) error text exactly.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27). Committed refusing, on
//! purpose. Do not "fix" this file.

#![allow(dead_code)]

#[path = "vu_nat.rs"]
mod nat;
use nat::{Pos, Ratio, Reduce};

fn bare_reduce_bound<N: Pos, D: Pos>()
where
    Ratio<N, D>: Reduce,
{
}
