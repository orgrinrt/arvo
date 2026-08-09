Sebastian Aaltonen, file 57. I wrote file 32 (does identity lower well), which measured whether the
identity contract's zero-cost claim held on this pin. Twenty-five files landed since; I do not carry
any conclusion of mine forward without re-checking it against what those files found, and one of
them (file 52) traced the one loose end I left in that file to an inspection-command flag rather
than to the design, so that specific finding of mine does not survive unexamined either. Nothing
else from file 32 is load-bearing to this dispatch.

**What I read.** `49_consolidation_four.md` in full, as instructed, then `50_fog_the_float_model.md`,
`51_fallin_the_last_tick_and_the_licence.md`, `52_ringer_the_tests_that_were_owed.md`,
`53_torvalds_does_it_still_earn_its_keep.md`, and `53b_persona_checkpoint_twelve.md`, which sets
this dispatch. `ls` of the panel directory (`00_context.md` through `56_jhala_...md` plus every
`NN_probes/` directory) confirmed nothing landed after `56`. Behind those I opened: `08_probes/`
(to attempt the flag-sweep reconstruction section 2 needed), `24_probes/` and `25_probes/` (same,
for the mul/MAC instruction claims), `51_probes/probe_4` and `probe_6` (the licence witnesses my
own pair extends), `52_probes/codegen_regression_harness.rs` and `52_probes/OUTCOMES.md` (the
shape section 3 follows), `34_giesen_the_three_halves_assembled.md:110-118` (the flag-per-question-
class statement section 2 applies), and `mock/benches/` (the harness itself, and the state it was
actually in before this dispatch ran it).

**Gates.** Canon gate: every surface this dispatch touches is design with no shipped source
(`grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth`, both from the repo root, both exit 1, reproduced fresh), except one
thing that is not design at all: `mock/benches/src/main.rs`, which is shipped bench-harness
infrastructure with a real, committed, working defect this dispatch found and fixed (section 1).
Fixing it is not a critique of arvo's design; it is bench tooling that failed to run any bench at
all, mine or the four that predate it. Test gate: `cargo test --offline --workspace` from `mock/`
reports **655 passed, 0 failed, 9 ignored** (the standing 654 plus the one correctness test this
dispatch adds, section 1). Every prior file's figure is otherwise unchanged; I did not touch
`mock/crates/`.

**What is compiled or measured, and what is reasoned.** This whole dispatch is measurement by its
own charter: a bench, a flag sweep, and a codegen regression pair. Every number below either came
out of the `arvo-benches` orchestrator's own CSV output (grounded on `pin`, `host`, `flags`, stated
per section) or out of a fresh `rustc` invocation on this dispatch's own probes, disassembled and
diffed by hand, never estimated. Where I state a hypothesis rather than a measured cause (the
per-op cost pattern in section 1.3), I say so and do not let it pass as a finding.

## 1. The runtime question: software quantiser against hardware `fadd`, subnormal fraction swept

### 1.1 The bench exists in `mock/benches/`, under the harness, not in a probe

Three new crates: `mock/benches/variants/quantiser-fadd-shared/` (the shared `Routine` impl and the
reference-model reimplementation of the design's round-first quantiser, `model.rs` copied
unmodified from `50_probes/model.rs`, per the review's own compose-rather-than-reinvent discipline),
`mock/benches/variants/quantiser-fadd-software/` and `.../quantiser-fadd-hardware/` (the two cdylib
variant bodies). Wired into `mock/Cargo.toml`'s workspace members, `mock/benches/Cargo.toml`'s
dependency list, and a new `[bench.quantiser-vs-fadd-subnormal-sweep]` section in
`mock/benches/bench.toml`, six sizes (`n = 0, 10, 25, 50, 75, 100`, read as the swept subnormal
percentage). This landed as a real commit on this branch, not as a probe: the repository's phase
gate treats `mock/benches/` as infrastructure, not per-crate source (`mock-workspace.md`'s own
statement, "the gate protects... per-crate documents under `crates/*/`, and Rust source" reads, in
the hook's actual behaviour, as source under `crates/`; `git commit` on the full bench addition
printed `infrastructure-only (no crate files staged)` and passed). So the dispatch brief's
contingency (land in a probe if the harness path is refused) did not fire; I state this because the
brief predicted a refusal that did not happen, and a reader should know which branch actually ran.

`AddSweep<const PCT: usize>` builds 256 operand pairs per call, `PCT` percent of them (by index,
both operands of a subnormal pair drawn subnormal together, so an add never mixes a subnormal
operand with a far-larger normal one that would just return the normal operand unrounded) drawn
from the subnormal range (exponent field zero, mantissa nonzero, the whole `[2^-149, 2^-126)` band)
and the rest from a normal band (`[2^-8, 2^8)`) wide enough to land additions on varied grid
positions without ever hitting infinity or the subnormal range by accident. `software_add`: decode
both operands, form the exact rational sum, run the design's own round-first quantiser
(`model::quantize`, `Dir::Nearest`), encode back. `hardware_add`: `a + b`.

**Correctness, checked, not assumed.** `bench-quantiser-fadd-shared`'s own `#[cfg(test)]` module
compares `software_add` against native `+` bit-for-bit over the exact distribution
`AddSweep<PCT>::build_input` generates, all six swept `PCT` values, 64 seeds each:
`6 * 64 * 256 = 98,304` operations, **0 mismatches**. `cargo test --offline -p
bench-quantiser-fadd-shared --release`: `1 passed; 0 failed`. This reuses `AddSweep::build_input`
directly rather than re-deriving the subnormal-fraction distribution a second time in the test
(one transform, one generation path). It is the same methodology file 50's own `probe_1` ran (round-
first quantiser against silicon, zero mismatches over 41M operations), applied to this bench's own
reference model rather than assumed to transfer from it.

*grounded on: `pin`, `host`, `flags` (`--release`, no other codegen flags for this test).*

### 1.2 A pre-existing defect blocked every bench in this file, mine and the other four

Before this dispatch, `arvo-benches` (the orchestrator binary `mock/benches/src/main.rs`) could not
run **any** bench. Every worker reported `TIMEOUT\t<load-fail>` and the harness panicked on the
first bench's own empty sample set (`index out of bounds: the len is 0 but the index is 0`,
`bench-harness/src/analysis.rs:313`). The cause: `mockspace-bench-harness`'s own
`BenchManifest::for_size` (via `resolve_variant_path`,
`~/.cargo/git/checkouts/mockspace-.../bench-harness/src/config.rs:244-265`) already shapes an
extensionless variant entry into its platform dylib name (`DLL_PREFIX` + stem + `DLL_SUFFIX`).
`main.rs` re-shaped the already-shaped path a second time (`shape_variant_path`, applied via
`config.variant_paths = config.variant_paths.into_iter().map(shape_variant_path).collect()`),
producing `liblibbench_spectral_bisection.dylib.dylib`, a filename with no corresponding file on
disk. This is not specific to my bench; `fnv1a-vs-xxhash3`, `structural-decomposition`, and
`spectral-bisection` hit the identical defect, and none of the four existing benches had a committed
CSV in the repo before this dispatch (`find mock/benches -iname "*.csv"` from a clean checkout: no
hits), so nobody had run this harness successfully before, at least not on this pin, on this
machine, since whichever change introduced the double shaping.

The fix: drop the redundant `.map(shape_variant_path)` call and the now-dead `shape_variant_path`
function (`mock/benches/src/main.rs`, committed on this branch). After the fix, all four existing
benches plus the new one run clean: `arvo-benches` (no arguments) completes all six bench sections,
writes a CSV plus meta plus findings artifact per size, and exits zero. This is exactly the class of
finding the dispatch was framed to surface: a headline that would have measured the harness's own
brokenness rather than anything about the design, caught by trying to actually run the thing rather
than by trusting that "the bench is owed, one afternoon" (file 50 section 7's own phrase) meant
merely writing it, not also confirming it runs.

*grounded on: `pin`, `host`, `tree` (the fix is a source change, recorded in the committed diff).*

### 1.3 The numbers

Six sizes, `runs_per_pass = 1000`, `batch_size = 100`, `passes = 4`, two modes (warm/cold) per pass,
matching the manifest's standing `[timing]` section (unchanged by this dispatch). Medians below are
the harness's own `algo_ns` column (function-under-test time, bridge overhead subtracted), warm mode
only, read directly from the committed CSVs.

| PCT subnormal | software median (ns) | software ns/op | hardware median (ns) | hardware ns/op | ratio |
|---|---|---|---|---|---|
| 0 | 5074.2 | 19.821 | 307.5 | 1.201 | 16.50x |
| 10 | 5246.7 | 20.495 | 306.4 | 1.197 | 17.12x |
| 25 | 5079.1 | 19.840 | 304.6 | 1.190 | 16.67x |
| 50 | 4453.4 | 17.396 | 360.2 | 1.407 | 12.36x |
| 75 | 4397.3 | 17.177 | 329.6 | 1.288 | 13.34x |
| 100 | 4056.9 | 15.847 | 308.3 | 1.204 | 13.16x |

**The conversation the strategy axis needs has a number now.** File 50 section 7 asked for exactly
this measurement, and the persona checkpoint's unasked item (`53b`, "What none of the questions
asked") named the missing thread: the design's own answer to who picks a speed-against-semantics
trade is the strategy marker, and nobody had threaded it through the lowering choice. This bench is
not that threading (which is a design decision, not a measurement); it is the number the decision
needs. The software quantiser costs **13x to 17x** a native `fadd` on this target, at every point of
the sweep, roughly 16 to 20 ns per operation against roughly 1.2 to 1.4 ns. That is not a rounding
error a `Hot`-tier float preset could absorb silently; it is the concrete magnitude of the trade a
`Hot` float preset opting into the hardware lowering (per file 50 section 5.3's receipt mechanism)
would be buying, and the concrete magnitude of the safety margin a semantics-first preset would be
paying for staying inside the type's own stated guarantee.

**Apple Silicon shows no subnormal cliff on the hardware side, on this target, confirming file 50's
own stated guess rather than leaving it unchecked.** File 50 section 7: "the historical penalty for
subnormals is enormous on some x86 cores and reportedly absent on Apple silicon, and if the second
is true then the usual argument for FTZ does not apply on this target... I am not going to assert
either number from memory. The bench is the answer." The hardware column above never exceeds 1.41
ns/op across the full 0-to-100 percent subnormal sweep; there is no order-of-magnitude jump the way
a legacy-x86 subnormal microcode trap would produce. This is a real, measured, citable answer to
file 50's own open question, on this target, at this pin.

**The software side's per-op cost falls, not rises, as the subnormal fraction increases, and I do
not have a compiled explanation for it.** 19.8 ns/op at 0% subnormal down to 15.8 ns/op at 100%.
Stated as a hypothesis, not a finding: the subnormal path in `model::quantize` takes a narrower,
more uniform branch pattern per call (the quantum exponent floors to the same fixed bottom-grid
value every time, per `Fmt::quantum_exp`'s own `if unfloored < floor { floor }` clause), which a
branch predictor could exploit across a run of 256 subnormal-heavy calls in a way it cannot across a
run mixing varied normal-band exponents. I did not instrument branch mispredictions or isolate the
effect; I report the measured pattern and flag the explanation as unverified rather than let it read
as compiled. The middle of the sweep (PCT 50 and 75) shows the hardware column's only real bump
(360.2 ns and 329.6 ns against a 305-to-330 ns floor elsewhere); whether that is signal or the
harness's own measurement noise (the `n0` findings report CV around 3.7% for hardware's own single-
size distribution) is a question this single run cannot settle and I do not resolve it here.

*grounded on: `pin`, `host`, `flags` (the harness's own standing build, `cargo build --offline
--release`, no manual codegen flags added by this bench beyond what the harness already applies).
The per-op hypothesis above is explicitly ungrounded; do not cite it as measured.*

## 2. The codegen-flag audit

### 2.1 What "the right flag set per question class" means, stated from where the review already
said it

File 34 (`34:110-118`): "axis legibility reads the check build, codegen quality reads a shipping-
shaped build." File 52 sharpened this into a concrete finding: `34:122-124`'s own vectorisable-
loop-idiom claim was never re-verified under its own corrected methodology, and the specific reason
it happened to still be true (`-C codegen-units=1`, inherited from an earlier LTO investigation, not
identified as load-bearing for that claim) was never named until file 52 named it. So the standing
question this dispatch inherits (`52` section 5, "I did not audit every OTHER codegen claim in the
review... a member picking up the review's own recommendation next should treat that audit as part
of the job") is not "what is the one correct flag set" but "which of the review's claims depend on
an unstated flag, and does the dependence hold."

### 2.2 Two of file 25's claims, swept, both survive

`25_xu_building_the_exact_product.md:395` ("the standard `umulh`/`madd`/`madd`/`mul` sequence") and
`25:273` ("four lanes of `adds`/`adc` pairs") both name instruction sequences with **no flags stated
at all**. Reproduced fresh from `25_probes/05_composed_exact_product.rs`'s `probe_mul_full_2_2` and
`25_probes/06_mac_fold_checked_accumulator.rs`'s `probe_mac_256`, both under `-C codegen-units=1`
and the rustc default (`codegen-units=16`), both byte-identical `diff` across the flag change (full
transcripts, `57_probes/OUTCOMES.md`). Both claims survive: they were never actually flag-fragile,
which is itself worth recording, because it means not every uncited codegen claim in this review is
a latent instance of file 52's own finding. Both files export exactly one `#[no_mangle]` symbol
each, which is the structural reason: `-C codegen-units` partitions a crate's *multiple* translation
units for parallel compilation, and a single-exported-symbol crate has no cross-function boundary
for that partitioning to land on differently. File 52's own flag-sensitive case (the assert-equal-
length idiom) compares **two** exported functions in one file; that is the shape that is at risk,
not every codegen claim in general. This is a sharper statement of when the flag matters than "sweep
everything and hope," and I offer it as the practical rule the next member should apply before
re-running the whole review's instruction-count inventory by hand: **a claim about a single exported
function, compiled alone, is very unlikely to be codegen-units-sensitive on this pin; a claim
comparing two or more functions in one compilation unit needs the sweep.**

*grounded on: `pin`, `host`, `flags` (both values checked and diffed).*

### 2.3 File 8's five-shape table is not reproducible from the committed audit trail

`08_fog_the_union_and_what_it_costs.md:225-233`'s instruction-count table (8 / 9 / 8+2-exits / 87 /
10 across five delivery shapes) states `-C opt-level=3` and nothing else, and its own probe,
`08_probes/e_codegen.rs`, opens with `use union::*;`. The only candidate for that module in the
committed tree, `08_probes/a_union.rs`, itself declares `pub mod spare;` and `pub mod fusion;` at
lines 720 and 721, and neither `spare.rs` nor `fusion.rs` exists anywhere in the panel directory.
Compiling `a_union.rs` as `--crate-name=union` fails with `E0583` on both missing modules. I did not
attempt to reconstruct `spare.rs`/`fusion.rs` from the surrounding prose; that would be writing new
source under my own name and presenting it as a reproduction of someone else's build, which is a
different act than auditing one. **This table cannot currently be reproduced by anyone from what is
committed.** That is not a claim that the numbers are wrong. It is the finding the dispatch's own
framing names directly: a claim that cannot be reproduced is a finding in itself, distinct from and
weaker than "the claim is false," and the distinction matters because the next reader should not
treat this table as re-verified by this audit (it is not) nor as refuted (it is not that either).

*grounded on: `pin`, `tree` (the missing-module fact is a fact about the committed tree, not about
the design).*

### 2.4 What this audit did not cover, stated as owed rather than silently dropped

I did not sweep files 24, 27, 43, 50, or 51's own remaining instruction-count claims beyond the
licence-leak pair in section 3. Files 27 and 43 make no instruction-level codegen claims at all
(`grep -c instruction` on both returns zero; nothing to audit there). File 24 makes an "instruction
cluster" claim (`24:557`, "a lowered body is observable in the artifact... each is a shift-and-round
instruction cluster") without a specific count to check for flag sensitivity; I judged this too
vague to sweep meaningfully rather than invent a specific assertion to test against it. Files 50 and
51's own instruction claims are self-audited within those files (both state their own flags
explicitly, `50` grounded on `pin, host, flags` throughout, `51` naming `-C opt-level=3, no other
codegen flags`) and this dispatch's section 3 extends file 51's specific pair rather than re-checking
its prior probes independently; a member with slack should still re-verify those under the
codegen-units axis specifically, since neither file states codegen-units and this audit found that
*most* single-symbol claims are insensitive to it but did not check every one of theirs.

## 3. The licence-leak regression pair

### 3.1 What was owed and what is built

`53b:57-59` (the persona checkpoint): "The `51` codegen regression pair (interior-safe `fold`
vectorises, `fold_compensated` on identical data stays scalar and unfused) joins the owed test list
so a toolchain change cannot leak the licence across the combinator boundary silently." Built as
`57_probes/codegen/licence_leak.rs` (the fixture) plus
`57_probes/codegen_regression_licence_leak.rs` (the harness, following file 52's exact shape:
`#![no_std]` free-standing fixtures, `emit_asm`/`body_of`/`count` helpers reused verbatim from
`52_probes/codegen_regression_harness.rs`'s own pattern, destined to merge into the same
`mock/crates/arvo/tests/codegen_regression.rs` file rather than ship as a second file once real
source exists).

`fold_interior_safe` reproduces `51_probes/probe_4`'s `sum_algebraic` (an 8-element `f32` reduction,
`.algebraic_add()` throughout, the form a build layer is licensed to emit once interior safety is
proven, per `51` section 2.4). `fold_compensated_step` reproduces `51_probes/probe_6`'s
`kahan_step_strict` (`(sum + y) - sum - y`, plain `+`/`-`, the form `fold_compensated` must always
compile from, since the licence must never reach it).

### 3.2 Measured

```
_fold_interior_safe:
    ldp   q1, q0, [x0]
    fadd.4s  v0, v1, v0
    faddp.4s v0, v0, v0
    faddp.2s s0, v0
    ret
_fold_compensated_step:
    fadd  s2, s0, s1
    fsub  s0, s2, s0
    fsub  s0, s0, s1
    ret
```

`fold_interior_safe` vectorises (NEON `.4s`/`.2s`), reproducing the shape file 51 section 2.2
measured on a different fixture. `fold_compensated_step` stays scalar, unfused, exactly one `fadd`
and two `fsub`, reproducing the shape file 51 section 2.3 measured for `kahan_step_strict`. **This
pair is byte-identical across `-C codegen-units=1` and the rustc default** (full asm diffed, zero
lines differ; `57_probes/OUTCOMES.md`), unlike file 52's own flag-sensitive test 4. So the pair is
grounded on `pin + host + flags` with both flag values checked from the day it lands, closing the
gap file 52's own test 4 left open by discovery rather than by design.

The harness itself runs and passes: compiled standalone with `rustc --test`, `1 passed; 0 failed`,
confirmed both with `RUSTC` resolving the pin's absolute binary path explicitly and with bare
`rustc` invoked from inside the repo tree (both resolve to `1.98.0-nightly (57d06900f 2026-05-27)`,
because `rust-toolchain.toml` sits at the repo root and rustup walks up from cwd to find it). A
different toolchain-resolution trap than file 52's own surfaced and is recorded in
`57_probes/OUTCOMES.md`: `RUSTC="rustc +nightly-2026-05-28"` fails outright, because `Command::new`
treats the whole string as one literal binary name with a space embedded in it; `RUSTC` has to name
a bare executable path, not an invocation string. Same class of mistake file 52's own note warns
about (an implicit assumption about how a shelled-out toolchain resolves), a different instance.

### 3.3 What this test does and does not close

Per file 52's own measurement-versus-contract distinction (`52` section 1), the positive half
(`fold_interior_safe` vectorises) is a measurement: a red run after a toolchain bump is news about
the optimiser, read and re-recorded, never patched to whatever the new compiler emits. The negative
half (`fold_compensated_step` stays scalar and unfused) is closer to contract in spirit, since the
design's own meaning for this combinator depends on the plain, unreassociated sequence
(`49:184-186`), but the test itself only pins what LLVM does with the plain source it is given; it
cannot and does not prove no future build layer will ever hand `fold_compensated` algebraic
operations by mistake. That coverage is the receipt file 51 section 2.4 proposes (the fourth
clause, "the combinator is `fold`, never `fold_compensated`") and nobody has built. This test is the
narrower fallback: it catches the case where the optimiser itself starts reassociating plain float
arithmetic without being asked, which would be a much larger and more visible event than one
build-layer bug, but costs nothing extra to also pin here.

## 4. What this closes on the open list, and what it leaves open

**Closed.** The software-quantiser-against-`fadd` bench (`49:814-821` and `53b` item 4, "the
software-quantiser-against-`fadd` bench in `mock/benches/` under the harness with the subnormal
fraction swept"). Built, run, and its numbers state a real magnitude (13x to 17x) for the trade the
strategy-axis threading question needs. The licence-leak regression pair (`53b:57-59`), built,
measured, and confirmed flag-insensitive. Two of file 25's uncited instruction claims, swept and
confirmed to survive.

**Not closed, stated as owed rather than silently dropped.** The full codegen-flag audit across the
review's remaining instruction-count claims (section 2.4). File 8's five-shape table, which stays
neither confirmed nor refuted (section 2.3). The strategy-axis-through-lowering-choice design
question itself, which this dispatch supplies a number for but does not decide; that decision
belongs to whoever next takes up file 50's own unasked thread, with this bench's numbers in hand.
The build-layer receipt's fourth clause (file 51 section 2.4), which this dispatch's regression
pair narrows the risk of but does not build. Whether the software-side per-op cost pattern across
the subnormal sweep (section 1.3) has a real cause or is measurement noise: I flagged a hypothesis
and did not check it.

**A pre-existing bug fixed as a side effect of doing the measurement rather than assuming it would
work.** Section 1.2. Not part of the three items this dispatch was asked for, and not something I
went looking for; it is what actually running the harness, instead of continuing to leave it "owed,
one afternoon," turned up. Every prior file that deferred a bench to `mock/benches/` "once it is
convenient" was deferring into a harness that could not run any bench at all until this landed.
