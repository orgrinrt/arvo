//! The **ragged** payload, read the obvious safe way.
//!
//! `seed/SETTLED_container.md:337-341` assigns this shape to `Cold` and
//! `Precise`: the payload is sized to the exact bit count, so a `W`-bit
//! numeral occupies `ceil(W/8)` bytes and consecutive elements sit at that
//! pitch. That is the better footprint and the ratified rule says it costs
//! three instructions per operation.
//!
//! This arm reads whole unaligned 64-bit limbs while whole limbs remain, then
//! assembles the trailing one to seven bytes from the largest power-of-two
//! loads that fit. It is what the shape costs written without tricks.
//!
//! See `bench-wide-rung-shared` for the arms, the key encoding and the single
//! transform all five arms call.

use bench_wide_rung_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "wide-rung-ragged",
    sizes = [
        129003, 160003, 192003, 200003, 232003, 256003,
        129103, 160103, 192103, 200103, 232103, 256103,
        200001, 200002, 200004, 200008
    ]
)]
fn run_ragged<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.limbs = arms::ragged(KEY, input);
        }
    }
}
