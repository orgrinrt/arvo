//! Hardware-`fadd` variant of the quantiser-vs-fadd bench. See
//! `bench-quantiser-fadd-shared` for the `AddSweep<PCT>` Routine and the reference model.
//!
//! Per file 50 (`50:553-565`) and file 51 (section 2.4), an unpinned hardware-float lowering
//! is not a legal `Lowering` of a `Ranged` numeral under the design's own ratified invariant
//! (`49:151-152`, `Lowering` changes no value) unless the environment is pinned. This variant
//! measures what that lowering costs where it IS legal, i.e. under the default entry FPCR
//! (`0x0000000000000000`, round-to-nearest-even, FZ off, measured by file 50 section 5.3 on
//! this same target), which the harness's worker subprocess inherits unless something upstream
//! writes the control register. It does not itself pin or verify the control state; that is
//! the build-layer receipt's job (file 50 section 5.3 clause 3), not the bench's.

use bench_quantiser_fadd_shared::{hardware_add, AddSweep, N};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(AddSweep, "quantiser-hardware", sizes = [0, 10, 25, 50, 75, 100])]
fn run_quantiser_hardware<const PCT: usize>(
    input: &<AddSweep<PCT> as Routine>::Input,
    output: &mut <AddSweep<PCT> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            for i in 0..N {
                output.s[i] = hardware_add(input.a[i], input.b[i]);
            }
        }
    }
}
