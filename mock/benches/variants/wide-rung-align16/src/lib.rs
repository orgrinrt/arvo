//! The **vector-aligned** payload: the ragged size padded to a multiple of
//! sixteen.
//!
//! `15` section 5 records this as live and unpriced. `Hot`'s wide arm pads a
//! 30-byte payload to 32 and a 25-byte payload to 32, so at `W = 200` it costs
//! seven bytes per element on top of ragged, which at the million-element
//! scale `arvo-toolbox-not-policer.md` describes is measured in megabytes.
//! Whether an SSE2 and NEON aligned baseline is worth that is a measurement,
//! and `15` says so and leaves it as an arm to be written rather than a ruling
//! to be asked for.
//!
//! At `W` in {200, 232, 256} this coincides with the word-rounded stride, so
//! at those widths it is a second independent noise-floor reading; at
//! {129, 160, 192} it is a genuinely third container.
//!
//! See `bench-wide-rung-shared`.

use bench_wide_rung_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "wide-rung-align16",
    sizes = [
        129003, 160003, 192003, 200003, 232003, 256003,
        129103, 160103, 192103, 200103, 232103, 256103,
        200001, 200002, 200004, 200008
    ]
)]
fn run_align16<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.limbs = arms::align16(KEY, input);
        }
    }
}
