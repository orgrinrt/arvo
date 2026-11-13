# The bench harness at head, run on bitpack-carrier-width

Job arvo-bench-harness-alive, closing fault A6. Host: x86_64 Linux, Intel Xeon @ 2.10GHz, 4 cores, 192 KiB L1d and 8 MiB L2 in total. Harness: mockspace dev 90b53199. Profile: release, one shared target at the workspace root.

## What was committed where

The earlier results at `mock/benches/bitpack-carrier-width_n*` are Apple M1 runs and are unchanged: the research under `mock/research/202608072330_the-numeral-canon-panel/` reads those paths. The harness writes its outputs to those same paths whatever the host, so the x86_64 results of this job were moved to `results-x86_64-xeon/` beside this note. The two sets are not comparable: the host, and with it the cache sizes, differ, and so does the size at which the working set leaves L2.

Five arms ran (d16, d16-control, d32, d64, packed). The sixth, packed-simd, is aarch64 only (O4) and is in no result here.

## Runs

Run 1, `run-1-sizes-16384-to-4194304.log`, one invocation over the sizes in order. Sizes 16384, 131072, 1048576 and 2097152 completed (totals 127.1s, 153.8s, 372.3s and 712.0s as printed). At n=4194304 pass 1 timed, then the shared-target alias `mock/target` was reaped by the disk trimmer and passes 2 to 4 print `TIMEOUT <load-fail>`; the csv written for it has 160 rows and is not a result. The invocation then failed at n=8388608: "failed to load variant cdylib .../libbench_bitpack_carrier_packed.so: ... cannot open shared object file: No such file or directory", after the warning "could not disassemble any of the 5 variant(s) (objdump/otool unavailable or failed); the duplicate check did not run." (log line 109).

Run 2, `run-2-size-8388608.log`, n=8388608 alone, 2580.5s, 400 rows.

Run 3, `run-3-size-4194304.log`, n=4194304 alone after the failed pass of run 1, 1311.6s, 400 rows.

The csv, meta.json and findings files in `results-x86_64-xeon/` are the outputs of runs 1 (four sizes), 3 and 2. The n=4194304 files of run 1 were overwritten by run 3. What the logs do not show: the duplicate-disassembly check prints a line only as that one warning, so they do not say whether it ran at the sizes that completed.

## Reproducing the runs

Each of runs 2 and 3 used a temporary `bench.toml` that was not committed: one size of the bench, the aarch64 arm dropped, and the arms pointed at `../../../target/release/` (O3). `run-2-n8388608-bench.toml.patch` and `run-3-n4194304-bench.toml.patch` are that change as a diff against the committed `mock/benches/bench.toml`, regenerated from the same one-size rewrite after the runs. Run 1 kept all six sizes and dropped only the packed-simd arm, with the paths unchanged and a repo-local `mock/target` link to the shared target. Build the arms first (O1).

## What was not as the brief said, and the smallest change for each

O1. `cargo mock bench run bitpack-carrier-width` built no arm. bench.toml names every arm as `../target/release/bench_<name>`, a path outside `variants/`, and the launcher only rebuilds arms under `variants/` (`variant_dirs_for` in mockspace `src/bench.rs`). The logs say "warning: 30 variant entries outside variants/ were not rebuilt for this filtered run" for run 1 and "5 variant entries" for runs 2 and 3, which had one size each. The run then timed what was on disk. Smallest change here: build the arms by hand with `cargo build --release --manifest-path mock/benches/variants/<dir>/Cargo.toml` before the run. Ticket 0006f375c423.

O2. The launcher forwards `--only <name>` to the consumer driver and arvo's driver read only `--bench`, so the filter was ignored and the first bench in the manifest ran instead. Fixed in `mock/benches/src/main.rs`: the driver now accepts both spellings.

O3. The cdylibs live in the workspace's one shared target. `resolve_variant_path` joins the manifest path to the bench directory, so `../target/release/...` is `mock/target/release/...`, and a repo-local `mock/target` alias is reaped by the disk trimmer when free space is under 10 GB. It was deleted twice during run 1, the second time at n=4194304, which is the failure described above. `CARGO_TARGET_DIR` and `--target-dir` are refused by a hook here. Tickets 6010c09b4695 (the harness) and f5fed2ff6414 (the trimmer).

O4. The arm `bench_bitpack_carrier_packed_simd` does not build on x86_64: E0432 unresolved import `bench_bitpack_carrier_shared::sum_simd_padal`, the item gated behind `#[cfg(target_arch = "aarch64")]` (`bench-bitpack-carrier-shared/src/lib.rs:458`). The bench lists the arm in all six sizes, so the committed bench cannot validate on another host. Ticket 2bb1132d2619.

O5. The harness writes its csv, meta.json and findings to `mock/benches/<bench>_n<size>.*` with no host in the name, so a rerun on another host overwrites the committed results of the first. This job overwrote the Apple M1 files and restored them after review. Ticket 3ceb078b0fc0.

## Can the harness commit a disassembly of a named function

Not as it is. The only public disassembly entry is `check_duplicates(&[String])` in `bench-harness/src/disasm.rs`. It hardcodes `bench_entry` and the whole text section, compares in memory and prints a warning. Its named-symbol helper `objdump_symbol` passes `--disassemble-symbols=<sym>`, which is llvm-objdump's flag. GNU objdump on this host answers:

```
objdump: unrecognized option '--disassemble-symbols=bench_entry'
```

so the cheap comparison returns nothing on a GNU host. Ticket 913abcb7fd46.

A named function is committed by hand, with either of:

```
llvm-objdump -d --disassemble-symbols=<sym> <lib.so> > <file>
objdump -d --disassemble=<sym> <lib.so> > <file>
```

Both outputs for `bench_entry` of the d16 arm are committed beside this note, each with a header naming its command, the arvo commit it was built at and the release profile: `bench_entry.d16.llvm-objdump.txt` (310 lines with the header) and `bench_entry.d16.gnu-objdump.txt` (340 lines with the header).

The ticket asks for the harness form, `cargo mock bench disasm <bench> <arm> <symbol>`, writing the file beside the results.
