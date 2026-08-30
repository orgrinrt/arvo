# Probe outcomes, file 96

Every claim below is either a terminal transcript (compiled/measured) or a
`file:line` citation checked fresh this session. Nothing here is reasoned
without a primary source attached.

## P1. Host cache hierarchy, measured

```
$ sysctl -a | grep -iE "l1dcachesize|l1icachesize|l2cachesize|l3cachesize|perflevel|cachelinesize|memsize\b"
hw.memsize: 8589934592
hw.perflevel0.physicalcpu: 4
hw.perflevel0.l1icachesize: 196608
hw.perflevel0.l1dcachesize: 131072
hw.perflevel0.l2cachesize: 12582912
hw.perflevel0.cpusperl2: 4
hw.perflevel0.name: Performance
hw.perflevel1.l1dcachesize: 65536
hw.perflevel1.l2cachesize: 4194304
hw.perflevel1.name: Efficiency
hw.cachelinesize: 128
hw.nperflevels: 2
```

Performance-core L1d: 131072 bytes (128 KiB). Performance-cluster L2 (shared
across 4 P-cores): 12582912 bytes (12 MiB). No SLC/L3 is exposed via `sysctl`
on Apple Silicon; not measured, not claimed. `hw.memsize` (8 GiB) says this
host is a constrained environment (VM or container), not a full-size Mac; the
same L1/L2 figures match what file 81 read on its own host, so the same chip
class, different provisioning.

## P2. The by-reference input path already existed, before this dispatch touched anything

```
$ grep -n "build_input(" bench-core/src/lib.rs bench-core/src/byte_routine.rs \
    bench-harness/src/*.rs bench-macro/src/*.rs
bench-core/src/byte_routine.rs:64:    fn build_input(seed: u64) -> [u8; IN] {
bench-core/src/lib.rs:88:    fn build_input(seed: u64) -> Self::Input;
bench-core/src/lib.rs:171:            let input = Self::build_input(seed);   # inside the DEFAULT build_input_bytes body
bench-core/src/byte_routine.rs:110,111,117,118,124,178,179,180   # unit tests only
```

`build_input` (the by-value form) is called from exactly one non-test site in
the whole harness: the default body of `Routine::build_input_bytes`
(`bench-core/src/lib.rs:171`). `mockspace_bench_macro::routine_bridge!` takes
`build_input_bytes` as a function pointer
(`bench-core/src/lib.rs:340: input_builder: <$R as $crate::Routine>::build_input_bytes,`)
and never references `build_input` directly. `ByteRoutine<IN, OUT, MAY_DIFFER>`
(`bench-core/src/byte_routine.rs:82-93`) already overrides `build_input_bytes`
to fill a `Vec<u8>` directly, with this doc comment on the override, unchanged
since before this dispatch:

```
/// Heap-filling override of the bridge path: fill a Vec of
/// exactly IN bytes directly, never materialising `[u8; IN]` on
/// the stack. This removes the practical ceiling on IN (a
/// multi-megabyte input would otherwise be a stack overflow
/// hazard at build time) while staying fully const-generic...
```

`ByteRoutine` ships in the same crate every bitpack bench in this repo
already depends on (`mockspace-bench-core`, `features = ["std"]`). Nobody
before this file grepped for it.

## P3. mockspace's own trait doc for `build_input`/`build_input_bytes` was clarified during this session

```
$ cd ~/Dev/clause-dev/mockspace && git log -3 --oneline -- bench-core/src/lib.rs
5c65183 docs: say that overriding the serialiser lifts the input ceiling
2471643 feat: mockspace-bench-matrix semantic-matrix layer
cf7343c fix: address bench-harness review - utf8 render, matrix validate, ols scale, tests
```

Read fresh partway through this dispatch, `bench-core/src/lib.rs:70-88` and
`:163-180` now state the override contract in the trait's own doc comment
("A routine that overrides that serialiser fills the byte buffer directly,
and nothing on that path calls this. `ByteRoutine` already does exactly
that."). Commit `5c65183` landed on the shared `mockspace` `dev` branch
during this session; whether that is a response to this dispatch or a
coincidence is not something this file can establish, and it changes nothing
about P2, which was true before that commit and is true independent of it:
the override mechanism (`ByteRoutine`), not the trait doc comment describing
it, is what makes a footprint bench buildable.

## P4. The single-variant bench section never runs the harness's own validator

```
$ grep -n "validation needs at least 2 variants" bench-harness/src/validation.rs
73:                "validation needs at least 2 variants, got {}",

$ sed -n '376,435p' bench-harness/src/driver/mod.rs
378:        if config.variant_paths.len() >= 2 {
...
431:        } else {
432:            // A single-variant bench has nothing to cross-validate
433:            // against; `validate` requires two, so it is skipped and
434:            // the `required` flag has no validation to act on here.
435:        }
```

The first sweep this dispatch ran (`bitpack-footprint-dense` and
`bitpack-footprint-packed`, one variant each) never called
`validation::validate` at all, at any size. Correctness for that run rested
entirely on this crate's own unit tests (below), never on the harness's own
per-seed check. Found by reading the driver after the run had already
finished; not caught before running. Fixed by adding a genuine second
variant to each section (`bitpack-footprint-dense-alt`,
`bitpack-footprint-packed-naive`), which the second sweep confirms triggers
real cross-validation (no `VALIDATION:` line in the run log for any
completed size, meaning nothing was dropped, at every size checked).

## P5. A self-inflicted naming bug, found by running the fix and reading the output

The first attempt at the two extra variants (P4) reused the bench.toml
section's own name as the `#[bench_variant(..., "name", ...)]` argument for
BOTH dylibs in a section (`"bitpack-footprint-dense"` for both the main and
the `-alt` variant). The macro's `bench_name()` export is what the CSV writer
and the statistical-comparison tables key on, not the dylib path, so both
dylibs' 80 samples per size landed under one indistinguishable label and the
per-variant medians silently merged. The tell: packed's reported per-element
cost jumped from ~0.16 ns (the `windowed` decoder alone) to ~0.40-0.47 ns
once merged with `naive`'s much slower samples under the same name, an
internally consistent number that was measuring nothing real. Every
established variant crate in this bench directory (`bitpack-plan-native`,
`bitpack-plan-windowed`, etc.) already uses a crate-specific name distinct
from its bench.toml section, which this file's first draft did not follow.
Fixed: `bitpack-footprint-dense-alt`, `bitpack-footprint-packed-naive`.
Confirmed post-fix (`grep` of the corrected CSVs): each size's CSV carries
two distinct `variant` strings, 80 rows each.

## P6. Test gate, with this file's own additions registered

```
$ cargo test --offline --workspace 2>&1 | grep "^test result:" \
    | grep -oE '[0-9]+ passed; [0-9]+ failed; [0-9]+ ignored' \
    | awk '{p+=$1; f+=$3; i+=$5} END{print "binaries="NR, "passed="p, "failed="f, "ignored="i}'
binaries=155 passed=672 failed=0 ignored=9
```

Standing baseline per `91:44`: 666 passed, 149 (implicit) binaries, 0 failed,
9 ignored. Delta: +6 binaries beyond the 153 an earlier partial count in this
session showed (the 5 crates this dispatch adds:
`bench-bitpack-footprint-shared`, `-dense`, `-dense-alt`, `-packed`,
`-packed-naive`, none of the four `cdylib`-only crates carrying their own
`#[test]`s but each still producing an empty test binary; the shared crate's
6 `#[test]`s are the entire `+6 passed`). 0 failed, ignored unchanged. This
matches the figure `672 passed across 155 binaries` reported by later files
attributing the drift from the 666/149 standing baseline to this dispatch's
uncommitted manifest additions: correct.
