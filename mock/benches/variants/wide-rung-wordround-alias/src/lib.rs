//! The **noise floor**.
//!
//! Identical stride to `wide-rung-wordround`, computed from a different
//! expression: the ragged size rounded up to a multiple of eight, rather than
//! the limb count times eight. The two are equal at every width, which
//! `wordround_alias_is_never_a_distinct_stride` asserts over one to five
//! hundred and twelve bits rather than over the swept set alone.
//!
//! So this cdylib holds byte-identical code to `wide-rung-wordround`, and the
//! spread between the two on a given row is that row's own run-to-run
//! variation. Any difference between two real arms smaller than that gap is
//! not signal. `bench-warm-container-shared`'s `plusone` arm is the same
//! device and is the reason this bench has one: a bench that carries its own
//! noise floor as an arm can say which of its differences are real without
//! appealing to the harness's own confidence intervals.
//!
//! See `bench-wide-rung-shared`.

use bench_wide_rung_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "wide-rung-wordround-alias",
    sizes = [
        129003, 160003, 192003, 200003, 232003, 256003,
        129103, 160103, 192103, 200103, 232103, 256103,
        200001, 200002, 200004, 200008,
        129100, 160100, 192100, 200100, 232100, 256100
    ]
)]
fn run_wordround_alias<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.limbs = arms::wordround_alias(KEY, input);
        }
    }
}
