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

    for (bench_name, section) in &manifest.bench {
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
    use bench_bitpack_shared::Column;
    use bench_quantiser_fadd_shared::AddSweep;
    use bench_quantiser_radix_shared::RadixAdd;
    use bench_spectral_bisection::Fiedler;
    use bench_structural_decomposition::Rcm;

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
