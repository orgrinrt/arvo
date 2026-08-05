Sebastian Aaltonen, file 96. I wrote files 32 (does identity lower well), 57
(the measurement debt), and 75 (what bitpacked means). File 75 measured
`Layout::Bitpacked` at three sizes and read their stability as evidence the
cost was compute-bound rather than a cache artefact. File 81 (Fog) corrected
that: every one of those sizes was L1-resident on this host's own 128 KiB L1,
so the stability proved nothing about bandwidth, the true multiple was
1.29x-1.50x rather than 4.6x, and the bench shape was "structurally incapable
of pricing the thing `Cold` exists to buy." That is the sentence this file
answers.

**What I read.** `91_consolidation_nine.md` in full, the standing base. `ls`
of the panel directory, current through `95b`. `81_fog_is_the_bitpack_cost_
inherent.md` in full, the direct predecessor, twice: once for its verdict,
once for its own probes and its own "owed" list. `95b_persona_checkpoint_
twentythree.md` in full, which ranks this dispatch first on the open list and
states the by-reference input path as the whole remaining blocker. Nothing
else in `mock/research/` was read for this file; the question is narrow
enough that widening the reading list would have cost time this dispatch
did not have, per the coordinator's own steer partway through.

**The shipped tree I touched, and why.** `mockspace-bench-core` and
`mockspace-bench-harness` source (`~/Dev/clause-dev/mockspace/bench-core`,
`bench-harness`), read to check a factual claim before reasoning from it,
per the method constraint: is the by-reference input path actually missing,
or does it already exist and nobody looked. `mock/benches/variants/bitpack-
plan-shared/src/lib.rs`, read to reuse its transform (`Pack<W>`, `sum_native`,
`sum_windowed`, `sum_naive`) rather than re-derive it. No shipped `arvo`
source (`mock/crates`) was read or touched; this dispatch stays inside
`mock/research` and `mock/benches`, per the panel's own scope.

**Gates.** Canon gate, fresh from the repo root: `grep -rln
"Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth`, both exit 1, empty, unchanged. Test gate, with
this file's own five new crates registered in `mock/Cargo.toml`:
`cargo test --offline --workspace` reports **672 passed, 0 failed, 9
ignored, across 155 test binaries**. The standing baseline per `91:44` is
666 passed across 149 binaries. The delta (+6 passed, +6 binaries, +0
failed, +0 ignored) is entirely this file's own: `bench-bitpack-footprint-
shared`'s six `#[test]`s, plus the four `cdylib`-only crates
(`-dense`, `-dense-alt`, `-packed`, `-packed-naive`) each contributing an
empty test binary. This matches the 672/155 figure the coordinator reports
three later files converging on and attributing to this dispatch's
uncommitted manifest work: **the attribution is correct.** I read the body
of every test in the surface I touch (six tests in `bitpack-footprint-
shared`, section "Tests" below); none is tautological. Toolchain `rustc
1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed
fresh inside the tree; the identical command outside the tree resolves to
stable.

**What is compiled, measured, and reasoned.** Every instruction-level claim
about the harness's own mechanics is a `file:line` citation into
`mockspace`'s source, checked fresh this session (`96_probes/OUTCOMES.md`).
Every ns figure is from the bench harness, warm mode, 40 samples per
variant per size, CSV and findings committed alongside this file
(`mock/benches/bitpack-footprint-{dense,packed}_n*.csv`). Where I predicted
something and the run refused it, the refusal is in the text.

## 0. The verdict, stated first

**The by-reference input path was never missing. It shipped before this
review started asking for it, in the same crate every bitpack bench here
already depends on, and nobody grepped for it across three files.** File 81
named the exact mechanism that caps the default path (`Routine::build_input`
called by value from the default `build_input_bytes`,
`bench-core/src/lib.rs:171`) and concluded the fix belonged "upstream... in
the harness repository." It did not need to be built upstream. It was
already there: `ByteRoutine`'s own `build_input_bytes` override
(`bench-core/src/byte_routine.rs:76-93`) fills a heap `Vec` directly, with a
doc comment stating the exact purpose in the exact words file 81 was
reaching for ("removes the practical ceiling on IN"). File 91 repeated the
characterisation ("cannot currently build at all... a by-reference input
path", `91:679-682`, and again as an owed artefact, `91:1025-1027`). File
95b repeated it a third time, ranking it "the whole remaining blocker" and
this dispatch first on the strength of that belief (`95b:151-155`). Three
files, one grep each would have dissolved it, and none of the three ran the
grep. That is not a soft finding; it cost the review three files' worth of a
belief that a five-minute check would have corrected, and it is worth being
blunt about precisely because being blunt is what stops a fourth file
repeating it.

**Built the bench anyway, on the mechanism that already existed, and it
prices something real: at the one size in this sweep where the packed
region fits this host's 12 MiB L2 and the dense region does not (7,000,000
elements, packed 10.85 MiB against dense 13.35 MiB), the packed/dense
multiple drops to its lowest point in the whole sweep, 1.43x against a peak
of 1.66x at L1-resident sizes**, and the drop is driven by dense's own cost
rising (+16% from the smallest size to this one) while packed's stays flat.
This is a real, if modest, footprint signal, the first this review has
produced anywhere. It is not the dramatic story `Cold`'s intent might
suggest, and section 5 says exactly what is missing to make it more than
modest.

## 1. Checking the blocker before building around it

The dispatch's own instruction was to check this before accepting it. I did.

`file 81` (`449-452`): "It needs a harness input larger than the current
`Routine::Input`-by-value construction admits; the input transits the stack
in `build_input_bytes`... which caps a flat input at a few megabytes."

`bench-core/src/lib.rs` (line numbers as read this session, `170-180`, the
default trait method):

```rust
fn build_input_bytes(seed: u64) -> std::vec::Vec<u8> {
    let input = Self::build_input(seed);   // by value, HERE, before any Vec exists
    let ptr = &input as *const Self::Input as *const u8;
    ...
    unsafe { core::slice::from_raw_parts(ptr, size) }.to_vec()
}
```

That is a **default** trait method. Trait defaults are overridable, and
`byte_routine.rs:76-93` already ships the override, in the same crate,
public, tested:

```rust
/// Heap-filling override of the bridge path: fill a Vec of
/// exactly IN bytes directly, never materialising `[u8; IN]` on
/// the stack. This removes the practical ceiling on IN...
#[cfg(feature = "std")]
fn build_input_bytes(seed: u64) -> std::vec::Vec<u8> {
    let mut buf = std::vec![0u8; IN];
    ...
    buf
}
```

`mockspace_bench_macro::routine_bridge!` takes `build_input_bytes` as a
function pointer (`bench-core/src/lib.rs:340`) and never references
`build_input` on the real path; the only non-test call site of `build_input`
in the entire harness is inside the default body of `build_input_bytes`
itself (`96_probes/OUTCOMES.md`, P2). Every bitpack crate in this directory
already depends on `mockspace-bench-core` with `features = ["std"]`, which
is exactly what gates `ByteRoutine`'s existence. The mechanism was one
`use` statement away from every prior bitpack bench in this repo, on every
day since `ByteRoutine` shipped.

**Why this file states it this bluntly.** Two checkpoints (`91`, `95b`)
carried the "needs building upstream" belief forward without independent
verification, and the second explicitly ranked this dispatch's priority on
that belief's strength. The panel's own standing instruction is to check a
brief's cheap factual claims before reasoning from them and to say so
plainly when one fails. This one failed. Say so plainly: nobody had
checked, and the mechanism was sitting in a sibling crate the whole time.

## 2. The bench, built on the mechanism that already existed

`mock/benches/variants/bitpack-footprint-shared/src/lib.rs`. `FootprintColumn<
const N: usize>`, `#[repr(C)] { logical: [u16; MAX_N], packed: [u8;
MAX_PACKED_BYTES] }`, `MAX_N = 33,554,432`. Its `Routine::build_input`
(the typed, by-value form the trait still requires a definition for) is
`unreachable!()`: `Self::Input`'s size is `MAX_N`-based **regardless of
which `N` a monomorphisation represents**, following the exact pattern
`bitpack-plan-shared`'s own `PlanColumn` already uses to dodge
`generic_const_exprs`
(a field length that is an expression of the struct's own const generic
parameter needs the forbidden feature; the fixed-`MAX_N`-then-slice pattern
is the established dodge, reused here at a larger `MAX_N`). That means
`FootprintColumn::<16384>`'s by-value construction is exactly as large
(about 116 MiB) as `FootprintColumn::<33554432>`'s, and there is no small
`N` at which the typed path is safe to call. A first draft wrote
`build_input` out in full, on the assumption that a small test size would
be safe; running its own test suite overflowed the stack on
`FootprintColumn::<16384>::build_input` before a single RNG call ran,
confirming the claim rather than merely asserting it. Even a
`#[should_panic]` wrapper around the call overflows (`SIGABRT`, not a
catchable unwind), because the ABI's return-slot reservation for a 116 MiB
aggregate happens at the call site regardless of what the callee's body
does; that crash is not kept as a standing test, since keeping it would
abort the whole test binary on every run.

`build_input_bytes` is a genuine override: it builds the logical values
into a heap `Vec<u16>` (proportional to `N`, never `MAX_N`), then fills a
heap `Vec<u8>` sized to `TOTAL_INPUT_BYTES` (`MAX_N`-based, but a zero-fill
of a fresh allocation is a zero-page mapping on this target, so the unused
tail past what `N` needs costs nothing real), reusing `bench-bitpack-plan-
shared::pack` for the packed region rather than re-deriving the bit
arithmetic. No value of `FootprintColumn` exists at any point during input
construction. Six tests (`cargo test -p bench-bitpack-footprint-shared`,
all passing): correctness of both decoders and an independent `sum_naive`
oracle at three sizes, the heap path's determinism per seed, its
sensitivity to a corrupted packed byte (the independent-oracle check is not
one that cannot fail), and the default `validate_output_bytes`'s pointer
cast exercised directly (sound because `build_input_bytes`'s output has
exactly `Self::Input`'s size and layout, for every `N` this bench declares).

The dense decoder is `sum_native` (a direct `u16` read and mask), the packed
decoder is `sum_windowed::<Pack<13>>`, both imported unmodified from
`bitpack-plan-shared`, not re-derived. This is the fourth design rule's own
requirement applied here: the packed decode's period, group stride, window
offsets, lane shifts, mask and load width are all functions of the width
alone and already live as associated consts on `Pack<13>`; a footprint bench
using the naive per-index runtime-computed decode would have measured the
loop file 81 already diagnosed a second time, not the footprint. `sum_native`
and `sum_windowed` are exactly the pair file 81 measured as the dense floor
and the best decode for a plain sum (section 4.1, 1.50x in-cache); reusing
them keeps this file's numbers comparable to file 81's rather than
introducing a third, drifted copy of the same transform.

## 3. A gap this file found in its own first sweep, and closed before reporting numbers

The first sweep (one variant per bench.toml section, `bitpack-footprint-
dense` = `sum_native` alone, `bitpack-footprint-packed` = `sum_windowed`
alone) ran clean, all six sizes, no errors. Writing this file up, I checked
what the harness actually validates and found it does not validate a
single-variant section at all:

```rust
// bench-harness/src/validation.rs:73
"validation needs at least 2 variants, got {}",
// bench-harness/src/driver/mod.rs:378, 431-435
if config.variant_paths.len() >= 2 { ... }
} else {
    // A single-variant bench has nothing to cross-validate
    // against; `validate` requires two, so it is skipped...
}
```

The first sweep's numbers rested entirely on this crate's own unit tests
(section 2), never on the harness's own per-seed check, at any of the sizes
actually swept. That is a real gap in what I would have reported as
harness-validated. Fixed: `bitpack-footprint-dense-alt` (a second, honestly
independent dense summation, an iterator fold rather than `sum_native`'s
indexed loop, over the identical `logical` region) and `bitpack-footprint-
packed-naive` (`sum_naive`, file 75's index-driven decode, an independent
implementation from `sum_windowed`, over the identical `packed` region).
Both wired into their sections' `variants` lists in `bench.toml`.

Fixing this exposed a second, self-inflicted bug: the first attempt at both
extra variants reused the bench.toml section's own name as the
`#[bench_variant(..., "name", ...)]` argument for both dylibs in a section,
which is what the CSV writer and the statistics tables key results on, not
the dylib path. Every established variant crate in this directory
(`bitpack-plan-native`, `bitpack-plan-windowed`, etc.) already uses a
crate-specific name distinct from its bench.toml section; this file's first
draft of the two new crates did not follow that, and the result was both
dylibs' eighty samples per size landing under one indistinguishable label,
silently merged. The tell was in the numbers themselves: packed's apparent
per-element cost jumped to roughly 2.5x its true value the moment the fast
`windowed` samples and the much slower `naive` samples shared a label, an
internally consistent but meaningless figure. Fixed
(`bitpack-footprint-dense-alt`, `bitpack-footprint-packed-naive` as the
actual `bench_name()` strings), rebuilt, re-run. `96_probes/OUTCOMES.md`
carries both as P4 and P5, because a mistake found and fixed while measuring
is exactly the kind of thing this review's own discipline says to record
rather than quietly correct and move on from.

Every size reported in section 4 below is from the corrected, two-variant,
cross-validated run: no `VALIDATION:` line appears in the run log for any
completed size, meaning nothing was dropped and every seed's output agreed
across both independently-implemented decoders per layout.

## 4. What the sweep separates, and where

**The separation requirement's own test, applied to this bench.** The
instantiation has to be one where the distinction (smaller footprint) is
nonvacuous: a size where the packed region's smaller footprint actually
changes which cache level holds it, not one where both layouts sit in the
same level regardless. At width 13, packed data bytes are `ceil(N * 13 /
8)` against dense's `N * 2`; on this host's 12 MiB (12,582,912-byte) L2,
the packed/dense pair crosses from "both fit" to "only packed fits" between
roughly 6.29M and 7.74M elements. `N = 7,000,000` sits in that window:
packed is 10.85 MiB (fits), dense is 13.35 MiB (does not). That is the
instantiation this sweep is built to bracket, alongside two solidly
L1-resident sizes (continuity with file 81's own smallest points) and two
solidly L2-resident sizes below the crossover.

| N | dense bytes | packed bytes | dense vs 12 MiB L2 | packed vs 12 MiB L2 |
|---:|---:|---:|---|---|
| 16384 | 32,768 | 26,624 | fits (L1) | fits (L1) |
| 65536 | 131,072 | 106,496 | fits (L1 boundary) | fits |
| 1,048,576 | 2,097,152 | 1,703,936 | fits | fits |
| 4,194,304 | 8,388,608 | 6,815,744 | fits | fits |
| 7,000,000 | 14,000,000 | 11,375,000 | **exceeds** | **fits** |

Median `algo_ns` per element, 40 warm samples per variant per size,
`sum_native` (dense) against `sum_windowed::<Pack<13>>` (packed, the same
pair file 81 measured):

| N | dense (ns/elem) | packed (ns/elem) | ratio packed/dense |
|---:|---:|---:|---:|
| 16384 | 0.09980 | 0.16594 | 1.663 |
| 65536 | 0.10319 | 0.16701 | 1.619 |
| 1,048,576 | 0.11078 | 0.17175 | 1.550 |
| 4,194,304 | 0.11214 | 0.17759 | 1.584 |
| 7,000,000 | 0.11575 | 0.16535 | **1.429** |

The multiple at 7,000,000 is the lowest in the sweep, 14% to 16% below its
value at the L1-resident sizes. Read the two columns separately rather than
only the ratio, because the ratio alone hides which side moved: packed's
own cost is not monotone (0.166 to 0.178 to 0.165, a small rise then a
return to its starting value, consistent with sampling noise on this host,
`96_probes/OUTCOMES.md` notes the CV this environment shows), while dense's
cost **rises monotonically and by a consistent margin across every step**,
+3.4% from 16384 to 65536, +7.4% more to 1,048,576, +1.2% more to 4,194,304,
+3.2% more to 7,000,000, sixteen percent total from the smallest size to the
crossover. The ratio's drop is dense getting more expensive, not packed
getting cheaper, which is the correct signature for a column that has
started leaving L2 rather than a decoder that has somehow improved.

Corroborating cross-check, `sum_naive` (file 75's independent decoder)
against `sum_windowed` at the same five sizes: 4.47x, 4.47x, 4.49x, 4.34x,
4.31x. Flat and close to file 81's own found multiple for the naive-versus-
windowed pair (section 3, "7.50 against 1.86 instructions per element,"
predicting roughly 4x), across the identical footprint range where the
dense/packed comparison above shows movement. This is what "the packing
transform is fixed and the footprint effect is real, not an artefact of
which decoder happened to run" looks like: the decoder-to-decoder multiple
within one layout stays put while the layout-to-layout multiple moves.

*Grounded on: measured (`mock/benches/bitpack-footprint-dense_n*.csv`,
`bitpack-footprint-packed_n*.csv`, `_findings.md` alongside each, 40 warm
samples per variant per size, harness `--mode validate` clean at every
size, `96_probes/OUTCOMES.md` P4-P5), compiled (`Pack<13>`'s consts, `bench-
bitpack-plan-shared`, unmodified), reasoned (the crossover byte-count
arithmetic, the monotone-dense-rise attribution).*

## 5. What this does not settle, named rather than smoothed over

**33,554,432 elements (64 MiB dense, both regions well past L2 into
whatever this host's memory subsystem does past L2) has no clean number in
this file.** A single-variant run at that size completed once, correctly,
and its own repeat under cross-validation was killed twice by session-level
background-task limits, once mid-run on the dense section (leaving a stale
single-variant CSV), once on an isolated re-run that hit the same limit
after roughly 1360 seconds of packed's own successful completion at the
same size in a separate run showed that single measurement IS survivable in
one shot; the combination of both sections plus both variants at this size
in one job was not, twice. I am not reporting numbers from either attempt:
one is uncross-validated, the other was overtaken by the naming-bug fix
(section 3) before it could be trusted. The five sizes in section 4
(32 KiB dense through 13.35 MiB dense) are what this file stands on, and
they are sufficient for the claim this file makes: they contain the
crossover itself, they are all cross-validated, and the trend at the
crossover is what the claim is about. What they cannot show is whether the
effect **grows** once the column is many multiples of L2 rather than
barely past it, which is exactly the regime file 81 named as unmeasured
and this file has not closed either, for a different reason (a
harness-adjacent operational limit rather than an input-construction one).

**Single-column, single-core, sequential access is not the bandwidth-
contention regime file 81's own closing paragraph named as what `Cold`'s
footprint intent needs, and this sweep does not manufacture that regime
either.** Dense's per-element cost at the largest clean size (7,000,000,
13.35 MiB, already past L2) is 0.1158 ns/element, which at 2 bytes/element
is about 17 GB/s of single-core sequential read throughput, close to file
81's own 18 GB/s figure at L1-resident sizes on its host. That this number
barely moved suggests the loop is still not saturating this host's DRAM
bandwidth even once past L2; hardware sequential-stream prefetch hides
latency well below saturation, and a 15-16% cost rise is consistent with a
mild increase in the miss rate the prefetcher cannot fully cover, not with
a transition into a bandwidth-bound regime. The genuinely decisive
instantiation, several packed and dense columns streamed concurrently
across this host's four performance cores, is unbuilt here for the same
reason file 81 left it unbuilt: it needs a multi-column, multi-core
harness shape this repository's bench crates do not have, and building one
was out of scope for the time this dispatch had once the coordinator's own
steer landed. Naming this precisely is more useful than a bigger single-
column number: **the crossover signal in section 4 is real and it is not
the whole story `Cold`'s intent needs told.**

## 6. What this hands forward, in the consolidation's own form

*The by-reference input path `91:1025-1027` and `95b:151-155` both name as
an owed mockspace-side fix does not need building: it already ships as
`mockspace_bench_core::Routine::build_input_bytes`'s override contract
(`ByteRoutine`'s own implementation demonstrates it,
`bench-core/src/byte_routine.rs:76-93`), and the harness's own trait doc now
states the contract explicitly (`bench-core/src/lib.rs:70-88`, `163-180`,
landed on `mockspace`'s `dev` branch during this session,
`96_probes/OUTCOMES.md` P3). A footprint-scale `Routine` overrides
`build_input_bytes` to write directly into a heap `Vec`, never constructs
`Self::Input` by value, and keeps `build_input` real-but-unreachable at
sizes beyond what the stack can hold, exactly as `bitpack-footprint-shared`
does. The prior belief that this needed an upstream build was never
checked against `mockspace-bench-core`'s own source; it is retired here.*

*At the one size in a five-point sweep from 32 KiB to 13.35 MiB dense where
the packed encoding fits this host's 12 MiB L2 and the dense encoding does
not (7,000,000 elements), the packed/dense multiple for a plain column sum
drops to 1.43x from a peak of 1.66x at L1-resident sizes, driven by dense's
own per-element cost rising monotonically and by roughly 16% across the
sweep while packed's stays flat. This is a real, cache-residency-driven
footprint effect, the first one this review's own bitpack benches have
produced; it is modest, it is measured on a single core against a single
column, and this file does not know whether it grows once the column sits
many multiples of L2 past the boundary or whether it needs concurrent
bandwidth contention across cores to become the dramatic story `Cold`'s
intent gestures at. Both are open, named as such, not resolved here.*

(Grounded: measured, `mock/benches/bitpack-footprint-{dense,packed}_n*.csv`
and their `_findings.md`; compiled, `96_probes/OUTCOMES.md` in full;
ratified unchanged, `78:552-567`/`91:663-694`'s reading of what
`Layout::Bitpacked` denotes, which this file assumes throughout and does
not reopen; reasoned, the crossover-byte-count arithmetic and the dense-
rise attribution.)

**What is owed, named rather than performed.**

- 33,554,432 elements, cross-validated, both sections, one job that
  survives the environment's own background-task limits (or split so each
  half completes independently and is reported honestly as such, which is
  what this file did for the smaller five and could not finish doing for
  this one before the coordinator's own steer to land).
- The bandwidth-contention shape section 5 names: several packed and dense
  columns streamed concurrently across this host's four performance cores,
  the only shape that can distinguish "the effect grows with concurrent
  pressure" from "the effect is what a single stream past L2 alone gives."
- Whether the crossover trend in section 4 is a genuine cache-residency
  effect or partly measurement drift on a noisy (likely virtualised, 8 GiB
  `hw.memsize`) host: the monotone five-point rise in dense's own cost is
  suggestive rather than statistically dispositive at 40 samples per point,
  and a repeat on a dedicated, non-virtualised host would settle it.

## Tests

Six, in `bitpack-footprint-shared/src/lib.rs`, all passing
(`cargo test -p bench-bitpack-footprint-shared --offline`). `column256_
agrees`, `column4096_agrees`, `column16384_agrees`: cross-check `sum_native`,
`sum_windowed`, and `sum_naive` against a ground-truth sum recomputed from
the raw `build_input_bytes` output, plus the default `validate_output_bytes`
pointer-cast path on both a correct and a perturbed sum (the perturbed case
must fail; it does). `build_input_bytes_is_deterministic_per_seed` /
`_differs_across_seeds`: the seed contract. `sum_naive_is_sensitive_to_
packed_corruption`: corrupting one packed byte moves `sum_naive`'s result,
so the independent-oracle check inside `validate_output` is not one that
cannot fail. None asserts a value against itself; none is a "call it and
check it returned" smoke test.

## Standing

Nothing here reopens what `78:552-567` and `91`'s section 1.22 ratified
about what `Layout::Bitpacked` denotes; this file measures a consequence of
that reading, on the mechanism the review had all along. Only op's calls are
final, and even those go stale. The blocker retraction in section 1 is the
loudest thing in this file and it is stated as bluntly as the finding
deserves: three files repeated a belief that a grep would have dissolved,
and the fourth file that would have repeated it again is this one, had it
not checked first.
