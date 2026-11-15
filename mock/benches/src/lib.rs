//! The hooks library of arvo's bench tree.
//!
//! The driver `mock bench run` generates from `bench.toml` compiles this file
//! in by path and calls `hooks()`. It contributes one thing: the routine each
//! bench measures, since no bench here takes plain bytes. The harness keys a row
//! by the bench and the point, so a bench with sweeps has one row over the points
//! of all of them.
//!
//! The routine types come from the support crates under `support/`, which the
//! generated driver takes as path dependencies. Which arms a row measures is
//! resolved from each bench's `bench.toml`, never from this table, so one
//! routine per (bench, point) serves every arm the row lists.

use bench_bitpack_carrier_shared::CarrierColumn;
use bench_bitpack_contend_shared::Contend;
use bench_bitpack_footprint_shared::FootprintColumn;
use bench_bitpack_plan_shared::{MacColumn, PlanColumn};
use bench_bitpack_shared::Column;
use bench_bitpack_wide_shared::Wide;
use bench_bitpack_write_contend_shared::WriteContend;
use bench_quantiser_fadd_shared::AddSweep;
use bench_quantiser_radix_shared::RadixAdd;
use bench_satfold_shared::Case as SatFoldCase;
use bench_warm_clamp_shared::Case as ClampCase;
use bench_warm_container_shared::Case;
use bench_wide_rung_shared::Case as WideCase;
use mockspace_bench_harness::driver::Hooks;
use mockspace_bench_harness::routine_table;

/// The hooks the generated driver runs.
pub fn hooks() -> Hooks {
    Hooks {
        routine_for: Some(routine_table!(
            // A subnormal-fraction percentage per point; both arms share it.
            "quantiser-vs-fadd-subnormal-sweep" => AddSweep [0, 10, 25, 50, 75, 100],
            // An exponent spread per point, radix two against radix ten.
            "decimal-quantiser-radix-sweep" => RadixAdd [0, 2, 8, 20],
            // The two `Layout::Bitpacked` readings per column size.
            "bitpack-sequential-sum" => Column [256, 4096, 16384],
            "bitpack-random-sum" => Column [256, 4096, 16384],
            // One buffer read three ways, at sizes bracketing the L1 data cache.
            "bitpack-decoder-shape" => PlanColumn [16384, 65536, 98304, 262144],
            // The same decoders feeding a heavier per-element kernel.
            "bitpack-kernel-amortisation" => MacColumn [16384, 65536, 98304, 262144],
            // Container fork (panel file 141), one `Case<KEY>` per row.
            "warm-container" => Case [
                80003, 80103, 80204, 80304, 80403, 80501, 81003, 130001, 130002, 130003, 130004,
                130008, 130016, 130103, 130204, 130304, 130401, 130402, 130403, 130404, 130408,
                130416, 130501, 131003, 160003, 160103, 160204, 160304, 160403, 160501, 161003,
                320003, 320103, 320204, 320304, 320403, 320501, 321003, 600003, 600103, 600204,
                600304, 600403, 600501, 601003, 640001, 640002, 640003, 640004, 640008, 640016,
                640103, 640204, 640304, 640403, 640501, 641003
            ],
            // Clamping semantics (panel file 142), one `ClampCase<KEY>` per row.
            "warm-clamp" => ClampCase [
                80001, 80010, 80020, 80030, 80040, 81040, 130001, 130010, 130020, 130030,
                130040, 130060, 130080, 131040, 160001, 160010, 160020, 160030, 160040, 160060,
                160080, 161040, 320001, 320010, 320020, 320030, 320040, 320060, 320080, 321040,
                600001, 600010, 600020, 600030, 600040, 600060, 600080, 601040, 640001, 640010,
                640020, 640030, 640040, 640060, 640080, 641040
            ],
            // Saturating-fold reassociation (panel file 92). KEY is
            // (LI+1)*1000 + NC*100 + AL*10 + OP; `bench-satfold-shared` has it.
            "satfold" => SatFoldCase [
                1000, 1001, 2000, 3000, 3001, 3010, 3100, 3101, 4000, 5000, 6000, 7000, 7001,
                7010, 7100, 7101, 8000, 9000, 10000, 10001, 10010, 11000, 12000, 12001, 12010,
                12100, 12101
            ],
            // Footprint: dense and packed share one `FootprintColumn<N>`.
            "bitpack-footprint" => FootprintColumn [
                16384, 65536, 1048576, 4194304, 7000000, 33554432
            ],
            // The wide rung. KEY is W*1000 + NC*100 + D; `bench-wide-rung-shared`.
            "wide-rung" => WideCase [
                129000, 129003, 129100, 129103, 160000, 160003, 160100, 160103, 192000, 192003,
                192100, 192103, 200000, 200001, 200002, 200003, 200004, 200008, 200100, 200103,
                232000, 232003, 232100, 232103, 256000, 256003, 256100, 256103
            ],
            // Carrier width (panel file 26), five arms per row.
            "bitpack-carrier-width" => CarrierColumn [
                16384, 131072, 1048576, 2097152, 4194304, 8388608
            ],
            // Contention (panel file 27). KEY is N*10 + T, the column size and
            // the thread count travelling together as one const parameter.
            "bitpack-contend" => Contend [
                163841, 163842, 163844, 163848, 10485761, 10485762, 10485764, 10485768,
                41943041, 41943042, 41943044, 41943048, 83886081, 83886082, 83886084, 83886088
            ],
            // Write contention (panel file 78). "safe" sizes land every
            // internal boundary on the packed period and "race" sizes do not,
            // which `bench-bitpack-write-contend-shared`'s own tests pin.
            "bitpack-write-contend" => WriteContend [
                655341, 655342, 655344, 655361, 655362, 655364, 20971501, 20971502, 20971504,
                20971521, 20971522, 20971524
            ],
            // The u16 question at a column several times past L2 (panel file 27).
            "bitpack-wide" => Wide [
                83886081, 83886084, 167772161, 167772164, 335544321, 335544324
            ],
        )),
        ..Hooks::default()
    }
}
