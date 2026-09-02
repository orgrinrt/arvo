//! accfit with the fold arity as a runtime value, to price that one static fact on its own.
//!
//! See `bench-warm-clamp-shared` for the arms, the key encoding, the
//! interior-safety predicate and the single pair of transforms every arm
//! calls.

use bench_warm_clamp_shared::{arms_dyn, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "warm-clamp-accfit-dyn",
    sizes = [
        80010, 130010, 160010, 320010, 600010, 640010,
        80020, 130020, 160020, 320020, 600020, 640020,
        80030, 130030, 160030, 320030, 600030, 640030,
        80040, 130040, 160040, 320040, 600040, 640040,
        130060, 160060, 320060, 600060, 640060,
        130080, 160080, 320080, 600080, 640080,
        80001, 130001, 160001, 320001, 600001, 640001,
        81040, 131040, 161040, 321040, 601040, 641040
    ]
)]
fn run_accfit_dyn<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms_dyn::accfit_dyn(KEY, input);
        }
    }
}
