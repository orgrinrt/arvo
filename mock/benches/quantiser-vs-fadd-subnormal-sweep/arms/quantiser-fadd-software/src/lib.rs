//! Software-quantiser variant of the quantiser-vs-fadd bench. See
//! `bench-quantiser-fadd-shared` for the `AddSweep<PCT>` Routine and the reference model.

use bench_quantiser_fadd_shared::{AddSweep, N, software_add};
use mockspace_bench_core::{FfiBenchCall, Routine, timed};
use mockspace_bench_macro::bench_variant;

#[bench_variant(AddSweep, "quantiser-software", sizes = [0, 10, 25, 50, 75, 100])]
fn run_quantiser_software<const PCT: usize>(
    input: &<AddSweep<PCT> as Routine>::Input,
    output: &mut <AddSweep<PCT> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            for i in 0..N {
                output.s[i] = software_add(input.a[i], input.b[i]);
            }
        }
    }
}
