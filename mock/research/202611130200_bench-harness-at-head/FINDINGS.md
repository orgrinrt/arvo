# The bench harness at head, run on bitpack-carrier-width

Job arvo-bench-harness-alive, closing fault A6. Host: x86_64 Linux, Intel Xeon @ 2.10GHz, 4 cores, 192 KiB L1d and 8 MiB L2 in total. Harness: mockspace dev 90b53199. Profile: release, one shared target at the workspace root.

## Result

The harness runs at head and the bench produces artifacts. Validation, the determinism check, the duplicate-disassembly check and timing all ran for five arms (d16, d16-control, d32, d64, packed) at all six sizes. The csv, meta.json and findings files beside this note are the new ones. They replace the earlier aarch64 results, which had a 128 KB L1D and a 12 MB L2, so the numbers are not comparable across the two hosts and the size where the working set leaves L2 differs.

The sixth arm, packed-simd, is aarch64 only and is not in these results.

Whole runs: sizes 16384 to 4194304 in one invocation (log: run-1-sizes-16384-to-4194304.log, 127.1s, 153.8s, 372.3s, 712.0s and 419.8s per size as printed, the last size in that log failed, see O3), then n=8388608 alone (run-2-size-8388608.log, 2580.5s).

## What was not as the brief said, and the smallest change for each

O1. `cargo mock bench run bitpack-carrier-width` built no arm. bench.toml names every arm as `../target/release/bench_<name>`, a path outside `variants/`, and the launcher only rebuilds arms under `variants/` (`variant_dirs_for` in mockspace `src/bench.rs`). Verbatim: "warning: 36 variant entries outside variants/ were not rebuilt for this filtered run; their artifacts may be stale." The run then timed what was on disk. Smallest change here: build the arms by hand with `cargo build --release --manifest-path mock/benches/variants/<dir>/Cargo.toml` before the run.

O2. The launcher forwards `--only <name>` to the consumer driver and arvo's driver read only `--bench`, so the filter was ignored and the first bench in the manifest ran instead. Fixed in `mock/benches/src/main.rs`: the driver now accepts both spellings.

O3. The cdylibs live in the workspace's one shared target. `resolve_variant_path` joins the manifest path to the bench directory, so `../target/release/...` is `mock/target/release/...`, and a repo-local `mock/target` alias is reaped by the disk trimmer when free space is under 10 GB. It was deleted twice, the second time during the n=8388608 run of the first invocation: "failed to load variant cdylib /home/user/arvo/mock/benches/../target/release/libbench_bitpack_carrier_packed.so: ... cannot open shared object file: No such file or directory". `CARGO_TARGET_DIR` and `--target-dir` are refused by a hook here. The run completed with a temporary bench.toml entry `../../../target/release/bench_<name>`, which bakes this workspace's layout into the manifest, so it was not committed. The committed bench.toml is unchanged.

O4. The arm `bench_bitpack_carrier_packed_simd` does not build on x86_64: E0432 unresolved import `bench_bitpack_carrier_shared::sum_simd_padal`, the item gated behind `#[cfg(target_arch = "aarch64")]` (`bench-bitpack-carrier-shared/src/lib.rs:458`). The bench lists the arm in all six sizes, so the committed bench cannot validate on another host. It was dropped from the manifest for the run only.

Each of O1, O3 and O4 and the disassembly gap below is a ticket, ids in the report.

## Can the harness commit a disassembly of a named function

Not as it is. The only public disassembly entry is `check_duplicates(&[String])` in `bench-harness/src/disasm.rs`. It hardcodes `bench_entry` and the whole text section, compares in memory and prints a warning. Its named-symbol helper `objdump_symbol` passes `--disassemble-symbols=<sym>`, which is llvm-objdump's flag. GNU objdump on this host answers:

```
objdump: unrecognized option '--disassemble-symbols=bench_entry'
```

so the cheap comparison returns nothing on a GNU host.

A named function is committed by hand, with either of:

```
llvm-objdump -d --disassemble-symbols=<sym> <lib.so> > <file>
objdump -d --disassemble=<sym> <lib.so> > <file>
```

The sample beside this note, `bench_entry.d16.llvm-objdump.txt`, is the first command on the d16 arm (`libbench_bitpack_carrier_d16.so`, 309 lines). It is the evidence that the second command also works: the GNU form gives 338 lines on the same file.

The ticket asks for the harness form, `cargo mock bench disasm <bench> <arm> <symbol>`, writing the file beside the results.
