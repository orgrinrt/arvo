//! Self-bench for arvo. Compares FNV1a and xxHash3 hash algorithms
//! across input sizes 64, 256, 1024, and 4096 bytes through the full
//! mockspace bench harness pipeline. First arvo consumer of
//! mockspace-bench-harness.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use arvo_benches::Fnv1aVsXxHash3;
use mockspace_bench_core::routine_bridge;
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
            let mut config = match manifest.for_size(bench_name, size_idx, &mock_benches_dir) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::FAILURE;
                }
            };
            config.variant_paths = config
                .variant_paths
                .into_iter()
                .map(shape_variant_path)
                .collect();
            let routine = match routine_for_n(&section.workload, config.n) {
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

/// Pick the right monomorphised Routine bridge for a given input size.
/// The bridge captures `Input = [u8; N]` at the type level, so each
/// declared size gets its own bridge. New sizes need a new arm.
fn routine_for_n(name: &str, n: usize) -> Option<RoutineSpec> {
    let bridge = match n {
        64 => routine_bridge!(Fnv1aVsXxHash3<64>),
        256 => routine_bridge!(Fnv1aVsXxHash3<256>),
        1024 => routine_bridge!(Fnv1aVsXxHash3<1024>),
        4096 => routine_bridge!(Fnv1aVsXxHash3<4096>),
        _ => return None,
    };
    Some(RoutineSpec {
        name: name.to_string(),
        bridge,
    })
}

/// Take a manifest variant path with bare cargo target stem and produce
/// the platform-shaped dylib path.
fn shape_variant_path(p: PathBuf) -> PathBuf {
    let parent = p.parent().map(Path::to_path_buf).unwrap_or_default();
    let stem = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
    parent.join(format!(
        "{}{}{}",
        std::env::consts::DLL_PREFIX,
        stem,
        std::env::consts::DLL_SUFFIX
    ))
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
    );
    ExitCode::SUCCESS
}
