//! Self-bench for arvo. Compares FNV1a and xxHash3 hash algorithms
//! across input sizes 64, 256, 1024, and 4096 bytes through the
//! mockspace bench harness pipeline.
//!
//! Uses the bench harness v3 helpers (`ByteRoutine` from
//! `mockspace-bench-core`, typed form of `#[bench_variant]` from
//! `mockspace-bench-macro`) so neither the orchestrator nor the
//! variant cdylibs declare a `Routine` impl.

use std::path::Path;
use std::process::ExitCode;

use mockspace_bench_core::{routine_bridge, ByteRoutine};
use mockspace_bench_harness::{self as harness, BenchManifest, RoutineSpec, Workload};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--worker") {
        return run_worker(&args);
    }

    let report_only = args.iter().any(|a| a == "--report-only");

    // Section filter. Without it every invocation re-runs every bench in the
    // manifest and rewrites every committed csv, meta and findings file, so a
    // dispatch measuring one thing destroys the artifact trail of all the
    // others. Six consecutive panel files declined to bench for exactly that
    // reason. `--bench <name>` may repeat; absent, the filter is inert and the
    // whole manifest runs as before.
    let only: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(_, a)| a.as_str() == "--bench")
        .filter_map(|(i, _)| args.get(i + 1).cloned())
        .collect();

    let manifest_path = Path::new("bench.toml");
    let manifest = match BenchManifest::load(manifest_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };

    let mock_benches_dir = std::env::current_dir()
        .expect("self-bench requires a readable current_dir for variant path resolution");

    let mut workload = Workload::new();
    workload.program("default", |b| {
        b.stage(vec![harness::algo_call(), harness::light_scalar()]);
    });

    if !only.is_empty() {
        for name in &only {
            if !manifest.bench.contains_key(name) {
                eprintln!("error: --bench `{name}` names no section in bench.toml");
                return ExitCode::FAILURE;
            }
        }
    }

    for (bench_name, section) in &manifest.bench {
        if !only.is_empty() && !only.iter().any(|n| n == bench_name) {
            continue;
        }
        for (size_idx, _size) in section.sizes.iter().enumerate() {
            let config = match manifest.for_size(bench_name, size_idx, &mock_benches_dir) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::FAILURE;
                }
            };
            // `BenchManifest::for_size` (mockspace-bench-harness `resolve_variant_path`)
            // already applies the platform dylib prefix/suffix to any extensionless
            // variant entry; re-shaping here doubled it (`liblibfoo.dylib.dylib`),
            // which is why every bench in this file failed to `dlopen` with
            // TIMEOUT/<load-fail> before this fix (found while landing the
            // quantiser-vs-fadd bench, file 57 of the formalization panel; not
            // specific to that bench, every existing bench in this manifest hit it).
            let routine = match routine_for_n(bench_name, config.n) {
                Some(r) => r,
                None => {
                    eprintln!(
                        "error: bench `{bench_name}` declares unsupported size n={}; \
                         add a match arm in routine_for_n",
                        config.n
                    );
                    return ExitCode::FAILURE;
                }
            };

            let csv_path = format!("{}_n{}.csv", bench_name, config.n);
            let report_path = format!("{}_n{}_findings.md", bench_name, config.n);

            if report_only {
                let samples = match harness::load_samples_csv(Path::new(&csv_path)) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!(
                            "error: report-only could not load `{csv_path}` for bench \
                             `{bench_name}` n={}: {e}",
                            config.n
                        );
                        eprintln!("hint: run the bench first to produce the csv");
                        return ExitCode::FAILURE;
                    }
                };
                if samples.is_empty() {
                    eprintln!(
                        "error: report-only: no samples in `{csv_path}` for bench \
                         `{bench_name}` n={}",
                        config.n
                    );
                    return ExitCode::FAILURE;
                }
                let result = mockspace_bench_harness::BenchResult {
                    title: section.title.clone(),
                    env: mockspace_bench_harness::EnvMeta::default(),
                    samples,
                    cache_path: csv_path.clone(),
                    report_path: report_path.clone(),
                };
                if let Err(e) =
                    harness::write_report_for_routine(&result, &routine, "warm", &report_path)
                {
                    eprintln!("error: writing report: {e}");
                    return ExitCode::FAILURE;
                }
                eprintln!("  regenerated {report_path}");
            } else {
                let result = match harness::run(&config, &routine, &workload) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("error: bench `{bench_name}` n={}: {e}", config.n);
                        return ExitCode::FAILURE;
                    }
                };
                if let Err(e) = harness::write_csv(&result, &csv_path) {
                    eprintln!("error: writing csv: {e}");
                    return ExitCode::FAILURE;
                }
                if let Err(e) =
                    harness::write_report_for_routine(&result, &routine, "warm", &report_path)
                {
                    eprintln!("error: writing report: {e}");
                    return ExitCode::FAILURE;
                }
                eprintln!("  wrote {csv_path} + {report_path}");
            }
        }
    }

    ExitCode::SUCCESS
}

/// Pick the right monomorphised Routine bridge for a given bench
/// name + input size. Hash benches go through `ByteRoutine`; graph
/// + spectral benches go through their per-routine bridges.
fn routine_for_n(name: &str, n: usize) -> Option<RoutineSpec> {
    use bench_bitpack_footprint_shared::FootprintColumn;
    use bench_bitpack_plan_shared::{MacColumn, PlanColumn};
    use bench_bitpack_shared::Column;
    use bench_quantiser_fadd_shared::AddSweep;
    use bench_quantiser_radix_shared::RadixAdd;
    use bench_spectral_bisection::Fiedler;
    use bench_structural_decomposition::Rcm;
    use bench_warm_clamp_shared::Case as ClampCase;
    use bench_warm_container_shared::Case;

    let bridge = match (name, n) {
        // hash benches: byte-array IO via ByteRoutine.
        ("fnv1a-vs-xxhash3", 64) => routine_bridge!(ByteRoutine<64, 8, true>),
        ("fnv1a-vs-xxhash3", 256) => routine_bridge!(ByteRoutine<256, 8, true>),
        ("fnv1a-vs-xxhash3", 1024) => routine_bridge!(ByteRoutine<1024, 8, true>),
        ("fnv1a-vs-xxhash3", 4096) => routine_bridge!(ByteRoutine<4096, 8, true>),

        // graph bench: RCM over BitMatrix-shaped input.
        ("structural-decomposition", 16) => routine_bridge!(Rcm<16>),
        ("structural-decomposition", 32) => routine_bridge!(Rcm<32>),
        ("structural-decomposition", 64) => routine_bridge!(Rcm<64>),

        // spectral bench: Fiedler vector + sign-cut partition over
        // dense Laplacian-weighted input.
        ("spectral-bisection", 16) => routine_bridge!(Fiedler<16>),
        ("spectral-bisection", 32) => routine_bridge!(Fiedler<32>),
        ("spectral-bisection", 64) => routine_bridge!(Fiedler<64>),

        // quantiser-vs-fadd bench: AddSweep<PCT> dispatched per swept
        // subnormal-fraction percentage PCT. Both variants (software
        // quantiser, hardware fadd) share this one Routine bridge per PCT;
        // which dylib runs is resolved from bench.toml's `variants` list,
        // not from this table, so one match arm per size covers both.
        ("quantiser-vs-fadd-subnormal-sweep", 0) => routine_bridge!(AddSweep<0>),
        ("quantiser-vs-fadd-subnormal-sweep", 10) => routine_bridge!(AddSweep<10>),
        ("quantiser-vs-fadd-subnormal-sweep", 25) => routine_bridge!(AddSweep<25>),
        ("quantiser-vs-fadd-subnormal-sweep", 50) => routine_bridge!(AddSweep<50>),
        ("quantiser-vs-fadd-subnormal-sweep", 75) => routine_bridge!(AddSweep<75>),
        ("quantiser-vs-fadd-subnormal-sweep", 100) => routine_bridge!(AddSweep<100>),

        // decimal-quantiser bench: RadixAdd<SPREAD> dispatched per swept
        // exponent-spread size. Both variants (radix two at binary32's
        // parameters, radix ten at decimal32's) share this bridge and the
        // identical operand stream; only the radix the kernel monomorphises
        // at differs.
        ("decimal-quantiser-radix-sweep", 0) => routine_bridge!(RadixAdd<0>),
        ("decimal-quantiser-radix-sweep", 2) => routine_bridge!(RadixAdd<2>),
        ("decimal-quantiser-radix-sweep", 8) => routine_bridge!(RadixAdd<8>),
        ("decimal-quantiser-radix-sweep", 20) => routine_bridge!(RadixAdd<20>),

        // bitpack access-pattern bench: the two `Layout::Bitpacked`
        // readings (byte-aligned slot, zero-inter-value-padding) dispatched
        // per swept column size. Sequential and random-access sections each
        // share the identical Column* Input/Output shape across their two
        // variant dylibs; only the extraction transform inside each dylib
        // differs.
        ("bitpack-sequential-sum", 256) => routine_bridge!(Column<256>),
        ("bitpack-sequential-sum", 4096) => routine_bridge!(Column<4096>),
        ("bitpack-sequential-sum", 16384) => routine_bridge!(Column<16384>),
        ("bitpack-random-sum", 256) => routine_bridge!(Column<256>),
        ("bitpack-random-sum", 4096) => routine_bridge!(Column<4096>),
        ("bitpack-random-sum", 16384) => routine_bridge!(Column<16384>),

        // bitpack decoder-shape bench: the identical zero-inter-value-padding
        // buffer read three ways (dense native carrier, index-driven decode,
        // plan-driven decode), swept across sizes that bracket this host's
        // L1 data cache rather than sitting entirely inside it.
        ("bitpack-decoder-shape", 16384) => routine_bridge!(PlanColumn<16384>),
        ("bitpack-decoder-shape", 65536) => routine_bridge!(PlanColumn<65536>),
        ("bitpack-decoder-shape", 98304) => routine_bridge!(PlanColumn<98304>),
        ("bitpack-decoder-shape", 262144) => routine_bridge!(PlanColumn<262144>),

        // the identical decoders feeding a heavier per-element kernel, to see
        // whether the decode-shape multiple is an upper bound over consumer
        // work or a constant factor on it.
        ("bitpack-kernel-amortisation", 16384) => routine_bridge!(MacColumn<16384>),
        ("bitpack-kernel-amortisation", 65536) => routine_bridge!(MacColumn<65536>),
        ("bitpack-kernel-amortisation", 98304) => routine_bridge!(MacColumn<98304>),
        ("bitpack-kernel-amortisation", 262144) => routine_bridge!(MacColumn<262144>),

        // container fork (panel file 141): the shipped Warm/Precise rule
        // against the deletion, against rung(W+1), against plain Rust
        // primitive arithmetic. One Case<KEY> bridge per row; which of
        // the four dylibs runs comes from bench.toml, not from here.
        ("warm-container-width-l1", 80003) => routine_bridge!(Case<80003>),
        ("warm-container-width-l1", 130003) => routine_bridge!(Case<130003>),
        ("warm-container-width-l1", 160003) => routine_bridge!(Case<160003>),
        ("warm-container-width-l1", 320003) => routine_bridge!(Case<320003>),
        ("warm-container-width-l1", 600003) => routine_bridge!(Case<600003>),
        ("warm-container-width-l1", 640003) => routine_bridge!(Case<640003>),
        ("warm-container-width-l2", 81003) => routine_bridge!(Case<81003>),
        ("warm-container-width-l2", 131003) => routine_bridge!(Case<131003>),
        ("warm-container-width-l2", 161003) => routine_bridge!(Case<161003>),
        ("warm-container-width-l2", 321003) => routine_bridge!(Case<321003>),
        ("warm-container-width-l2", 601003) => routine_bridge!(Case<601003>),
        ("warm-container-width-l2", 641003) => routine_bridge!(Case<641003>),
        ("warm-container-density-w13", 130001) => routine_bridge!(Case<130001>),
        ("warm-container-density-w13", 130002) => routine_bridge!(Case<130002>),
        ("warm-container-density-w13", 130004) => routine_bridge!(Case<130004>),
        ("warm-container-density-w13", 130008) => routine_bridge!(Case<130008>),
        ("warm-container-density-w13", 130016) => routine_bridge!(Case<130016>),
        ("warm-container-density-w64", 640001) => routine_bridge!(Case<640001>),
        ("warm-container-density-w64", 640002) => routine_bridge!(Case<640002>),
        ("warm-container-density-w64", 640004) => routine_bridge!(Case<640004>),
        ("warm-container-density-w64", 640008) => routine_bridge!(Case<640008>),
        ("warm-container-density-w64", 640016) => routine_bridge!(Case<640016>),
        ("precise-container-width-l1", 80103) => routine_bridge!(Case<80103>),
        ("precise-container-width-l1", 130103) => routine_bridge!(Case<130103>),
        ("precise-container-width-l1", 160103) => routine_bridge!(Case<160103>),
        ("precise-container-width-l1", 320103) => routine_bridge!(Case<320103>),
        ("precise-container-width-l1", 600103) => routine_bridge!(Case<600103>),
        ("precise-container-width-l1", 640103) => routine_bridge!(Case<640103>),
        ("warm-elementwise-width-l1", 80204) => routine_bridge!(Case<80204>),
        ("warm-elementwise-width-l1", 130204) => routine_bridge!(Case<130204>),
        ("warm-elementwise-width-l1", 160204) => routine_bridge!(Case<160204>),
        ("warm-elementwise-width-l1", 320204) => routine_bridge!(Case<320204>),
        ("warm-elementwise-width-l1", 600204) => routine_bridge!(Case<600204>),
        ("warm-elementwise-width-l1", 640204) => routine_bridge!(Case<640204>),
        ("precise-elementwise-width-l1", 80304) => routine_bridge!(Case<80304>),
        ("precise-elementwise-width-l1", 130304) => routine_bridge!(Case<130304>),
        ("precise-elementwise-width-l1", 160304) => routine_bridge!(Case<160304>),
        ("precise-elementwise-width-l1", 320304) => routine_bridge!(Case<320304>),
        ("precise-elementwise-width-l1", 600304) => routine_bridge!(Case<600304>),
        ("precise-elementwise-width-l1", 640304) => routine_bridge!(Case<640304>),
        ("warm-affine-collapse-l1", 80403) => routine_bridge!(Case<80403>),
        ("warm-affine-collapse-l1", 130403) => routine_bridge!(Case<130403>),
        ("warm-affine-collapse-l1", 160403) => routine_bridge!(Case<160403>),
        ("warm-affine-collapse-l1", 320403) => routine_bridge!(Case<320403>),
        ("warm-affine-collapse-l1", 600403) => routine_bridge!(Case<600403>),
        ("warm-affine-collapse-l1", 640403) => routine_bridge!(Case<640403>),
        ("precise-widening-theorem-l1", 80501) => routine_bridge!(Case<80501>),
        ("precise-widening-theorem-l1", 130501) => routine_bridge!(Case<130501>),
        ("precise-widening-theorem-l1", 160501) => routine_bridge!(Case<160501>),
        ("precise-widening-theorem-l1", 320501) => routine_bridge!(Case<320501>),
        ("precise-widening-theorem-l1", 600501) => routine_bridge!(Case<600501>),
        ("precise-widening-theorem-l1", 640501) => routine_bridge!(Case<640501>),

        // clamping-semantics bench (file 142). One ClampCase<KEY> bridge per row;
        // the six arms are resolved from bench.toml's `variants` list.
        ("warm-clamp-arity-w8", 80010) => routine_bridge!(ClampCase<80010>),
        ("warm-clamp-arity-w8", 80020) => routine_bridge!(ClampCase<80020>),
        ("warm-clamp-arity-w8", 80030) => routine_bridge!(ClampCase<80030>),
        ("warm-clamp-arity-w8", 80040) => routine_bridge!(ClampCase<80040>),
        ("warm-clamp-arity-w13", 130010) => routine_bridge!(ClampCase<130010>),
        ("warm-clamp-arity-w13", 130020) => routine_bridge!(ClampCase<130020>),
        ("warm-clamp-arity-w13", 130030) => routine_bridge!(ClampCase<130030>),
        ("warm-clamp-arity-w13", 130040) => routine_bridge!(ClampCase<130040>),
        ("warm-clamp-arity-w13", 130060) => routine_bridge!(ClampCase<130060>),
        ("warm-clamp-arity-w13", 130080) => routine_bridge!(ClampCase<130080>),
        ("warm-clamp-arity-w16", 160010) => routine_bridge!(ClampCase<160010>),
        ("warm-clamp-arity-w16", 160020) => routine_bridge!(ClampCase<160020>),
        ("warm-clamp-arity-w16", 160030) => routine_bridge!(ClampCase<160030>),
        ("warm-clamp-arity-w16", 160040) => routine_bridge!(ClampCase<160040>),
        ("warm-clamp-arity-w16", 160060) => routine_bridge!(ClampCase<160060>),
        ("warm-clamp-arity-w16", 160080) => routine_bridge!(ClampCase<160080>),
        ("warm-clamp-arity-w32", 320010) => routine_bridge!(ClampCase<320010>),
        ("warm-clamp-arity-w32", 320020) => routine_bridge!(ClampCase<320020>),
        ("warm-clamp-arity-w32", 320030) => routine_bridge!(ClampCase<320030>),
        ("warm-clamp-arity-w32", 320040) => routine_bridge!(ClampCase<320040>),
        ("warm-clamp-arity-w32", 320060) => routine_bridge!(ClampCase<320060>),
        ("warm-clamp-arity-w32", 320080) => routine_bridge!(ClampCase<320080>),
        ("warm-clamp-arity-w60", 600010) => routine_bridge!(ClampCase<600010>),
        ("warm-clamp-arity-w60", 600020) => routine_bridge!(ClampCase<600020>),
        ("warm-clamp-arity-w60", 600030) => routine_bridge!(ClampCase<600030>),
        ("warm-clamp-arity-w60", 600040) => routine_bridge!(ClampCase<600040>),
        ("warm-clamp-arity-w60", 600060) => routine_bridge!(ClampCase<600060>),
        ("warm-clamp-arity-w60", 600080) => routine_bridge!(ClampCase<600080>),
        ("warm-clamp-arity-w64", 640010) => routine_bridge!(ClampCase<640010>),
        ("warm-clamp-arity-w64", 640020) => routine_bridge!(ClampCase<640020>),
        ("warm-clamp-arity-w64", 640030) => routine_bridge!(ClampCase<640030>),
        ("warm-clamp-arity-w64", 640040) => routine_bridge!(ClampCase<640040>),
        ("warm-clamp-arity-w64", 640060) => routine_bridge!(ClampCase<640060>),
        ("warm-clamp-arity-w64", 640080) => routine_bridge!(ClampCase<640080>),
        ("warm-clamp-chain-l1", 80001) => routine_bridge!(ClampCase<80001>),
        ("warm-clamp-chain-l1", 130001) => routine_bridge!(ClampCase<130001>),
        ("warm-clamp-chain-l1", 160001) => routine_bridge!(ClampCase<160001>),
        ("warm-clamp-chain-l1", 320001) => routine_bridge!(ClampCase<320001>),
        ("warm-clamp-chain-l1", 600001) => routine_bridge!(ClampCase<600001>),
        ("warm-clamp-chain-l1", 640001) => routine_bridge!(ClampCase<640001>),
        ("warm-clamp-arity-l2", 81040) => routine_bridge!(ClampCase<81040>),
        ("warm-clamp-arity-l2", 131040) => routine_bridge!(ClampCase<131040>),
        ("warm-clamp-arity-l2", 161040) => routine_bridge!(ClampCase<161040>),
        ("warm-clamp-arity-l2", 321040) => routine_bridge!(ClampCase<321040>),
        ("warm-clamp-arity-l2", 601040) => routine_bridge!(ClampCase<601040>),
        ("warm-clamp-arity-l2", 641040) => routine_bridge!(ClampCase<641040>),
        ("warm-affine-density-w13", 130401) => routine_bridge!(Case<130401>),
        ("warm-affine-density-w13", 130402) => routine_bridge!(Case<130402>),
        ("warm-affine-density-w13", 130404) => routine_bridge!(Case<130404>),
        ("warm-affine-density-w13", 130408) => routine_bridge!(Case<130408>),
        ("warm-affine-density-w13", 130416) => routine_bridge!(Case<130416>),

        // footprint bench: prices Cold's own intent (a smaller column fits
        // where a larger one does not) rather than decode cost at
        // cache-resident sizes, which is all every bitpack bench above this
        // one measures. Sizes bracket this host's own 128 KB L1 and 12 MB L2
        // (read fresh via sysctl, `bitpack-footprint-shared`'s own doc
        // comment): 16384 and 65536 stay inside L1 for continuity with the
        // decoder-shape sweep; 1048576 and 4194304 sit inside L2; 7000000 is
        // the crossover where the packed region (10.85 MiB) still fits L2 and
        // the dense region (13.35 MiB) no longer does; 33554432 puts both
        // regions well past any cache this host has. Dense and packed share
        // one `FootprintColumn<N>` bridge (same seed, same logical value
        // stream, two encodings), so both bench sections below register the
        // identical routine per size.
        ("bitpack-footprint-dense", 16384) => routine_bridge!(FootprintColumn<16384>),
        ("bitpack-footprint-dense", 65536) => routine_bridge!(FootprintColumn<65536>),
        ("bitpack-footprint-dense", 1048576) => routine_bridge!(FootprintColumn<1048576>),
        ("bitpack-footprint-dense", 4194304) => routine_bridge!(FootprintColumn<4194304>),
        ("bitpack-footprint-dense", 7000000) => routine_bridge!(FootprintColumn<7000000>),
        ("bitpack-footprint-dense", 33554432) => routine_bridge!(FootprintColumn<33554432>),
        ("bitpack-footprint-packed", 16384) => routine_bridge!(FootprintColumn<16384>),
        ("bitpack-footprint-packed", 65536) => routine_bridge!(FootprintColumn<65536>),
        ("bitpack-footprint-packed", 1048576) => routine_bridge!(FootprintColumn<1048576>),
        ("bitpack-footprint-packed", 4194304) => routine_bridge!(FootprintColumn<4194304>),
        ("bitpack-footprint-packed", 7000000) => routine_bridge!(FootprintColumn<7000000>),
        ("bitpack-footprint-packed", 33554432) => routine_bridge!(FootprintColumn<33554432>),

        _ => return None,
    };
    Some(RoutineSpec {
        name: name.to_string(),
        bridge,
    })
}

fn run_worker(args: &[String]) -> ExitCode {
    let get = |flag: &str| -> Option<String> {
        let pos = args.iter().position(|a| a == flag)?;
        args.get(pos + 1).cloned()
    };

    let dylib_path = match get("--worker") {
        Some(p) => p,
        None => {
            eprintln!("worker: missing --worker <path>");
            return ExitCode::FAILURE;
        }
    };
    let bench_name = get("--bench-name").unwrap_or_default();
    let seed: u64 = get("--seed").and_then(|s| s.parse().ok()).unwrap_or(0);
    let cooldown_ms: u64 = get("--cooldown").and_then(|s| s.parse().ok()).unwrap_or(0);
    let mode = get("--mode").unwrap_or_else(|| "warm".into());
    let runs: usize = get("--runs").and_then(|s| s.parse().ok()).unwrap_or(0);
    let batch: usize = get("--batch").and_then(|s| s.parse().ok()).unwrap_or(1);
    let n: usize = get("--n").and_then(|s| s.parse().ok()).unwrap_or(1);
    let batch_k: usize = get("--batch-k").and_then(|s| s.parse().ok()).unwrap_or(1);
    let max_call_us: Option<u64> = get("--max-call-us")
        .and_then(|s| s.parse().ok())
        .filter(|&v| v > 0);
    // The coordinator passes this through from the manifest's `threaded`.
    // A threaded bench's spawned workers never inherit a core pin, so the
    // harness skips pinning rather than pin only the coordinating thread
    // and skew the workload.
    let threaded = args.iter().any(|a| a == "--threaded");

    let routine = match routine_for_n(&bench_name, n) {
        Some(r) => r,
        None => {
            eprintln!("worker: unsupported n={n} for bench `{bench_name}`");
            return ExitCode::FAILURE;
        }
    };

    let mut workload = Workload::new();
    workload.program("default", |b| {
        b.stage(vec![harness::algo_call(), harness::light_scalar()]);
    });

    harness::run_worker(
        &routine,
        &workload,
        &dylib_path,
        seed,
        cooldown_ms,
        &mode,
        runs,
        batch,
        n,
        batch_k,
        max_call_us,
        threaded,
    );
    ExitCode::SUCCESS
}
