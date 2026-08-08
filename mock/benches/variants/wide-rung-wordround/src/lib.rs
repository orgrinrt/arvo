//! The **word-rounded** payload: whole 64-bit limbs, naturally aligned.
//!
//! `seed/SETTLED_container.md:337-341` assigns this shape to `Hot` and `Warm`:
//! the payload is rounded up to whole 64-bit words, so a 200-bit numeral
//! occupies 32 bytes rather than 25. That is the seven bytes per value the
//! ratified rule trades away, in exchange for the three instructions per
//! operation it claims to save.
//!
//! See `bench-wide-rung-shared`.

use bench_wide_rung_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "wide-rung-wordround",
    sizes = [
        129003, 160003, 192003, 200003, 232003, 256003,
        129103, 160103, 192103, 200103, 232103, 256103,
        200001, 200002, 200004, 200008,
        129100, 160100, 192100, 200100, 232100, 256100
    ]
)]
fn run_wordround<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.limbs = arms::wordround(KEY, input);
        }
    }
}
