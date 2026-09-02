// Which verbs force the format's own obligation, pinned from the refusing side.
//
// Three arms in `src/tests/obligations.rs` pin the routes that reach a value
// without meeting it. Nothing pinned the positive half, which is the claim that
// was wrong: the design named `apply` and the identity search, and `apply` forces
// the slot range's obligation rather than this one.
//
// `has_additive_identity` is one of the two functions that do force it, so a
// format whose phase names no position on the grid is refused here. The
// obligation is a const, so binding the call in a const item is what makes the
// refusal reachable at check time rather than only at codegen.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{has_additive_identity, Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;
use arvo_format::width::Bool;

struct PhaseNamesNoPosition;

impl Format for PhaseNamesNoPosition {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::of(1, 0);
}

const _REFUSED: Bool = has_additive_identity::<PhaseNamesNoPosition>();

fn main() {}
