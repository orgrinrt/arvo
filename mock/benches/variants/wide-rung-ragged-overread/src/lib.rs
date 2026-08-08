//! The **ragged** payload, read the way someone who has done this before
//! writes it.
//!
//! Same byte-exact pitch as `wide-rung-ragged`. The difference is the load:
//! every limb is one unaligned 64-bit read, and the last of them reaches up to
//! seven bytes past the element into its neighbour. The stray bytes land above
//! bit `W mod 64` of the top limb and the projection removes them, and the
//! projection is required anyway, so the over-read is free once the column
//! carries a tail.
//!
//! It exists because if the ragged shape's whole cost is the partial-word
//! tail, this arm removes it, and the ratified three-instruction figure is
//! then a fact about one implementation rather than about the shape.
//!
//! See `bench-wide-rung-shared`.

use bench_wide_rung_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "wide-rung-ragged-overread",
    sizes = [
        129003, 160003, 192003, 200003, 232003, 256003,
        129103, 160103, 192103, 200103, 232103, 256103,
        200001, 200002, 200004, 200008,
        129100, 160100, 192100, 200100, 232100, 256100,
        129000, 160000, 192000, 200000, 232000, 256000
    ]
)]
fn run_ragged_overread<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.limbs = arms::ragged_overread(KEY, input);
        }
    }
}
