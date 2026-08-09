# The tests that were owed, and what a test that pins a measurement owes its reader that a test that pins a contract does not

Talia Ringer, file 52. I wrote file 19 (the witness and its upkeep) and file 44 (what the overturn
left behind); the second diagnosed the defect class the review's grounding registry now exists to
catch, and this dispatch is that diagnosis applied to a different artifact than the one it was
written for. My own earlier files do not get a pass for that reason. File 44's own subject
(`44:1-40`) was a claim surviving a coordinate change without being re-derived; section 4 of this
file finds exactly that defect again, one level down, inside a claim the review has cited four times
as settled, and it is not my earlier finding recurring by prophecy, it is the same failure mode
being generic across artifact kinds, which is the whole reason to write it down twice.

**What I read.** `49_consolidation_four.md` in full, per the standing instruction that it is the
only required reading, then the two deliverables since it (`50_fog_the_float_model.md`,
`51_fallin_the_last_tick_and_the_licence.md`), then `ls` of the panel directory. I did not reason
from `49:117`; file 50 already named the exponent-bounds-as-`const` line a known defect against
section 1.15 (`50:224-229`) and I take that as settled rather than re-litigating it. Behind the three
required files I opened exactly the material my own artifacts needed to reproduce rather than
invent: `46_probes/` (the seal tower and its adversary), `47_probes/` and `48_probes/` (the
projection-chain positive and negative controls), `43_smith_division.md` (the `div_floor`/`rem`
fusion claim's origin), `34_giesen_the_three_halves_assembled.md` plus `32_probes/` and `34_probes/`
(the vectorisable-loop-idiom claim's origin, where section 4 below finds the actual defect), and
`35_dolan_does_widening_collapse.md` plus `26_consolidation_two.md:452-457` (the carry-chain and
fold-vs-direct-multiply claims' origins, and the exact sentence, "a dependency on an optimiser
heuristic holding, not a guarantee," that names what this whole dispatch is about).

**Gates.** Canon gate: the surface every artifact below is about (`Pos`/`Nat`/`Adjustment`/`Bias`,
the fold's `Grade`, `mul_full`, `div_floor`/`rem`) has no shipped source. `grep -rln
"Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth`, both from the repo root, both exit 1, empty, reproduced fresh for this
dispatch (`52_probes/OUTCOMES.md`'s own gate section). So nothing here critiques shipped arvo code;
every artifact is design, checked against the toolchain rather than against a tree that does not yet
exist, exactly as files 50 and 51 state for their own dispatches. Test gate: `cargo test --workspace`
was not re-run because this dispatch adds no source under `mock/crates/`; the standing figure (654
passed, 0 failed, 9 ignored) is unchanged since file 41 and this dispatch touches none of that
surface. I ran the read-the-test-bodies half of the gate on the object this dispatch actually
produces (the fifteen artifacts below), which section 3 does explicitly, because a dispatch whose
entire subject is "are these tests real" cannot exempt its own tests from the question.

**What is compiled and what is reasoned.** Every claim in sections 1 through 4 that names an error
code, an instruction count, or a "compiles clean" is a fresh build on `rustc 1.98.0-nightly
(57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, resolved via `rustc +nightly-2026-05-28` (the
bare `rustc` on `PATH` resolves to the machine's stable `1.94.0` outside the repo tree; section 4
records finding this the hard way on this dispatch's own first pass and redoing every build once the
mistake surfaced). `52_probes/OUTCOMES.md` carries every command and every reproduced figure.
Everything else, the design contribution in sections 1 through 3, is reasoning about what the review
owes its own future readers, and I mark it as such throughout rather than let it borrow the
compiled sections' authority.

## 0. What landed, as artifacts

`52_probes/`, fifteen files the review can run today plus two harness files stating exactly what
changes when the design they test ships as real source:

- `52_probes/seal/`: the sealed `Pos`/`Nat`/`Adjustment`/`Bias` tower (an unmodified copy of
  `46_probes/probe_2_vu_core_lib.rs` and its two included files), `harness.rs` (destined
  `mock/crates/arvo/tests/seal_adversary.rs`), and `ui/` with seven attack fixtures plus their
  captured `.stderr` and one positive control, all renamed to their shipping filenames and rebuilt
  against the tower shipped alongside them, not against the copy in `46_probes/`.
- `52_probes/projection/`: the ratified tower plus the fold's `Grade` mechanism (unmodified copies
  of `47_probes/tower.rs`/`vu_nat.rs`/`vu_bias.rs`), `harness.rs` (destined
  `mock/crates/arvo/tests/grade_projection_chain.rs`), and `ui/` with the projection's positive
  control and the `Reduce`-bound negative control, renamed and rebuilt fresh.
- `52_probes/codegen_regression_harness.rs` + `52_probes/codegen/`: five target-scoped codegen
  regression tests (six `#[test]` functions; the fold-vs-direct-multiply item splits into a native-
  width and a multi-limb-width test because they are separately falsifiable claims), destined
  `mock/crates/arvo/tests/codegen_regression.rs` + `tests/codegen/*.rs`.

All three package headers state, once each, exactly what moving them into `mock/crates/arvo/tests/`
changes: delete the local tower/support files, rewrite `use vu_core::...`/`use tower::...` to
whatever module path the real tower lands at, and nothing else. No assertion, no error code, no
instruction count changes on the move; the seal's error codes are language facts about coherence and
privacy, unaffected by which crate imports the sealed traits, and the codegen fixtures reference no
tower type at all (section 1 explains why).

## 1. A test that pins a measurement is a different object from a test that pins a contract

The dispatch asks which of the fifteen artifacts are which, because they fail differently and want
different responses when red. Here is the classification, and the reasoning behind each entry is the
actual design contribution, not the table.

**The five codegen regression tests are measurements, without exception.** Every one of them is
"the LLVM backend, at this optimisation level, with this exact flag set, on this target, currently
lowers this source to this instruction shape." None of them is "the language guarantees this
lowering." `26_consolidation_two.md:452-457` says this about the carry chain specifically and the
sentence generalises to all five: "This is a dependency on an optimiser heuristic holding, not a
guarantee, and it costs one codegen test to make falsifiable." A red run on an unchanged pin and
unchanged target is a real regression, in the harness or the fixture. A red run after a toolchain
bump is **news**: read what changed, decide whether the design's cost promise for the affected
preset still holds under the new number, and record the new figure. It is never patched by loosening
the assertion to match whatever the new compiler emits, because that would be recording defeat as
success; and it is never patched by pinning an old compiler forever either, because the whole point
of a target-scoped regression test is to notice when the ground moves, not to freeze it. Every
doc comment on every codegen test in `codegen_regression_harness.rs` states this reading inline
rather than assume the next reader infers it from this document.

**The seal adversary is a contract suite.** Coherence, the orphan rules, and trait-privacy are
language guarantees the Rust project treats as part of its soundness story, not optimisation
heuristics subject to revision for a faster build. If `seal_direct_impl_all_four_carriers.rs` ever
compiles clean, or if `seal_extension_positive_control.rs` ever stops compiling, that is not news
about a toolchain improving; it is either a soundness regression in the compiler (vanishingly
unlikely and worth reporting upstream immediately) or a fact about how the real, shipped tower's own
seal differs from this stand-in's, which is the case worth taking seriously first. There is no
"reasonable middle" reading for a seal test the way there is for a codegen test: red here is always a
defect to fix, never information to log.

**The projection-chain pair splits down the middle, and the split is worth stating precisely because
the two halves look alike and are not.** `grade_projected.rs` (the positive control) is a contract
test: the fold's own signature, as designed, must never trigger the composition wall, and if it ever
does the design's own claim (`49:306-324`, "every trait in a chain that reaches a consumer-facing
signature either pattern-matches on constructor heads or has finite, non-recursive obligations") has
been violated by a change to the mechanism, which is always a defect. `reduce_bound_wall.rs` (the
negative control) is closer to a measurement than its neighbour, and file 48's own words already say
so: "If this ever compiles, the solver changed and every wall finding in this review needs
re-grounding (grounded on: pin)" (quoted in this dispatch's own harness header,
`52_probes/projection/harness.rs`). The wall exists because `Reduce`'s recursive definition has no
base case for an abstract, non-constructor-headed operand, which is a real mathematical fact about
the trait chain the review is not proposing to change; but WHETHER the solver reports that as
`overflow evaluating the requirement` (today's behaviour) or resolves it some other way is
trait-solver implementation, and the next-generation trait solver the workspace's own
`unstable-features.md` already tracks (`with_negative_coherence`, `min_generic_const_args`'s own
solver dependency) is exactly the kind of change that could move it. So: if `reduce_bound_wall.rs`
starts compiling, that is not automatically a defect the way a seal breach is. It is a prompt to
check whether the underlying recursion is STILL genuinely unbounded (if yes, the solver got smarter
and the finding is retired with gratitude, and the review's forbidden-feature discipline around
`generic_const_exprs`, `min_specialization` and `TypeId`, section 1.12's own carrier-sealing
argument, needs a fresh look at whether any of its own compiled refusals rested on the same solver
behaviour) or whether the trait chain itself changed underneath the test without anyone noticing
(if yes, it is a defect, because nobody meant to change `Reduce`).

## 2. What each test actually establishes, against what its name suggests

File 44's diagnosis (`44:1-40`, and restated at `49:614-619` in its own words: "a consolidation
compresses a conjunctive claim's conclusion and discards which of its members were actually
checked") applies to test suites as much as to prose claims, and a suite is worse exposed to it
than a paragraph, because a paragraph gets re-read and a passing test does not. Read every artifact
below by what its assertions actually pin, not by its filename.

**`multi_limb_carry_chain_compiles_to_straight_line_adc_no_calls`** pins two things its name states
correctly: zero calls, and exactly three carry-propagating instructions for a four-limb chain. It
does NOT pin that `core::arch::aarch64` has no fallback intrinsic (`35:113-119`'s own second half);
that is a fact about the platform's intrinsic surface, checked once by reading, not by a test that
could catch its own regression (an intrinsic being added upstream would not make this test fail; it
would make the finding stale in a direction a passing suite cannot report). The doc comment on the
test says this explicitly so a reader does not credit the test with more than it checks.

**`fold_vs_direct_multiply_native_width_folds_to_one_instruction`** pins symbol-level equality
(`_hot_mul_via_full_then_quantize = _hot_mul_direct` in the emitted assembly), which is a stronger
and more specific claim than "both compile to one instruction each" would be: two functions each
compiling to one `mul` with different register allocations would satisfy an instruction-count
assertion and would NOT satisfy this one, and the symbol-equality form is what `35:117-120` actually
measured. An instruction-COUNT-only version of this test would have a name indistinguishable from
this one and would establish something weaker; I checked which the source in `35_probes/` actually
demonstrates before writing the assertion, rather than write the assertion I expected and let the
name carry the rest.

**`fold_vs_direct_multiply_multi_limb_width_matches_instruction_shape`** pins instruction-count and
mnemonic-set equality (`umulh` x1, `madd` x2, `mul` x1, in either function), explicitly NOT
symbol equality, because `35:129-137` itself records the two functions are "the same shape up to
commutative operand order," not byte-identical. Naming this test with the same rigor as its native-
width neighbour, when the underlying claim is genuinely weaker, is the specific discipline this
section asks for: a test's name should not borrow its sibling's strength.

**`saturating_reduction_stays_scalar_wrapping_control_vectorises`** pins the ABSENCE of NEON lane
instructions in the saturating path and the PRESENCE of at least one in the wrapping control. It does
NOT pin that a saturating reduction over more lanes, or over a different width, behaves the same way;
`4` is the arity `35_dolan_does_widening_collapse.md:103-110` measured and the arity this test
measures, and the doc comment does not claim the finding generalises past it.

**`assert_equal_length_idiom_defeats_vectoriser_bare_loop_does_not`** is the one whose name and
whose actual grounds diverge the most, and section 4 is the full account. Stated compactly here: the
name says what the test checks correctly, but the CLAIM the test is supposed to be regression-testing
(`34:122-124`) had never itself been checked under the methodology it claims to use, and the flag
that makes the difference reproducible (`-C codegen-units=1`) is not named anywhere in the three
files that state the claim. The test as shipped is honest about its own grounds (its doc comment
states the flag sensitivity); the claim it defends was not, until this dispatch checked it.

**`div_floor_and_rem_fuse_into_one_hardware_divide`** pins `sdiv` count equal to one inside
`div_floor_and_rem` specifically, the function that calls both operations on the same operands in one
body. It does NOT pin, and the doc comment says so, that the two independently-callable public
functions `div_floor` and `rem_euclid` share a division when called separately at different call
sites; they do not, and cannot without the caller inlining both, which is a fact about function
boundaries, not about the fusion claim `43:283-287` actually makes ("when both are used," which the
review's own wording already scopes to one call site).

**The seven seal attacks** each pin one error CODE, not one error TEXT. `52_probes/OUTCOMES.md`
records the full verbatim text captured from a direct `rustc` run; the shipped `.stderr` files carry
that text, and the harness's own doc comment (`52_probes/seal/harness.rs`) states plainly that the
exact rendered text is expected, not yet confirmed, to match trybuild's own `cargo`-driven rendering
byte for byte, and that confirming it is the first `TRYBUILD=overwrite` run against the real crate,
the same bootstrap every other `.stderr` in `mock/crates/arvo/tests/ui/` went through. A reader who
takes the `.stderr` files as already-trybuild-verified would be crediting them with a check this
dispatch could not perform (there is no real `arvo`-side tower to run trybuild against yet); the
header says so rather than let the file format imply a rigor the file has not earned yet.

**`seal_extension_positive_control.rs`** pins that three specific extension shapes (structural
recursion over public constructors, MATLAB-numeral-piece composition, a convention-trait bound on
the carriers) still compile. It does not pin that EVERY legitimate extension shape compiles; it is
the same three shapes `46_probes/probe_6_extension_positive_control.rs` checked, no more, and a
future extension idiom nobody has written yet is untested by construction, the same honest limit
file 46 itself states for that probe.

**`grade_projected.rs`** pins that the fold's grade-as-projection compiles with no unstable feature.
It does not pin that the projection is CORRECT (that the grade it computes actually matches the
transfer rule); that is a separate claim, checked by the whole-matrix join-algebra assertions inside
`48_probes/probe_2_grade_algebra_lib.rs`, which this dispatch did not re-package because it is
already a real, compiled, whole-matrix suite and packaging it a second time under a different name
would be exactly the "the label claims more than the body checks" pattern in reverse: a body that
checks MORE than a differently-scoped test's name would suggest, mislabeled to look like a subset of
this one. It stays where file 48 put it.

## 3. A compile-fail test is only as good as its error being the intended one

File 46's own first draft of `probe_3d_malformed_types_refused.rs` used bare type aliases and
"COMPILED CLEAN while testing nothing," because a type alias defers its bound checks
(`46_probes/OUTCOMES.md`'s own row for that probe). The committed form forces well-formedness
through function signatures instead. Every fn-forcing pattern in `52_probes/seal/ui/` is inherited
unmodified from that lesson, not reinvented; I checked, by reading each fixture's body rather than
trusting its filename, that every attack that needs to be fn-forced still is, in the copy shipped
alongside this dispatch's own tower rebuild, and every attack that does not need forcing (the direct
impls, which reach a bounded position through their own trait declaration rather than through a
consumer using them) is left as file 46 left it.

The same discipline applies to the codegen tests in the opposite direction: a codegen assertion that
is TOO LOOSE passes for the wrong reason exactly as a compile-fail test with the wrong error code
does. `assert_eq!(carry_instrs, 3)` rather than `assert!(carry_instrs > 0)` is deliberate: the loose
form would still pass if LLVM emitted the chain as four separate non-carry-propagating adds plus
manual overflow detection (a real alternative lowering that would NOT demonstrate the idiom
recognition the test exists to pin), so the exact count is the assertion that actually distinguishes
"the idiom fired" from "the arithmetic happened to come out right." I wrote the loose form first (it
is the natural first draft) and tightened every one of the six codegen assertions to an exact count
or an exact symbol-equality check after checking, for each one, what a plausible wrong lowering would
still satisfy under the loose form. `sat_reduce`'s test is the clearest instance: `count == 0` for
the saturating path and `count > 0` for the wrapping control are BOTH load-bearing, because a version
of this test asserting only the first half would pass identically whether the wrapping control
vectorised or the control itself had silently broken, which is exactly the "the control stopped
controlling and nobody noticed" failure a paired positive/negative assertion exists to prevent.

## 4. The finding this dispatch was not sent to make, and made anyway

`34:122-124` (Giesen, file 34): "one of file 32's sub-findings is real and sharpened by shape A: the
`assert!(equal lengths)` loop idiom defeats the vectoriser on this pin with or without the identity
contract (`probe_elementwise_add_fixed_equal_len_idiom` and its no-generic ablation both scalar
under shape A)." This is stated as a measured fact, cited in the consolidation's own owed-test-debt
list three times over (`40:662-665`, `49:814-816`), and it is the specific item this dispatch's
fourth codegen test exists to pin.

`34_probes/probe_0_revectorise.sh`, the script file 34 names as the source of its shape-A evidence,
measures exactly two symbols under shape A: `probe_vectorises_verbatim_control` and
`probe_elementwise_add_fixed_no_assert` (both quoted verbatim at the top of this file's section 0).
Neither `probe_elementwise_add_fixed_equal_len_idiom` nor `probe_elementwise_add_ablation_no_generic`
(the two functions that actually carry the assert idiom, and the two the prose sentence names)
appears anywhere in that script, nor in `34_probes/OUTCOMES.md`, nor in `32_probes/OUTCOMES.md`.
`grep -n "equal_len_idiom\|ablation_no_generic" 34_probes/OUTCOMES.md 32_probes/OUTCOMES.md` returns
nothing; I ran it before writing this paragraph. The claim survived from file 32's own original
measurement (taken under the `-C lto=fat` methodology file 34's own section 1 spent its first page
correcting) into file 34's corrected prose, without being re-run under the corrected methodology
file 34 built specifically to fix everything else in that section.

Re-running it against the actual committed crate, under file 34's own stated flags (`--edition 2021
-C opt-level=3 -C codegen-units=1 -C panic=abort`, shape A, no LTO), the claim IS TRUE:

| symbol | NEON `.2d` lines |
|---|---|
| `probe_elementwise_add_fixed_equal_len_idiom` | 0 |
| `probe_elementwise_add_ablation_no_generic` | 0 |
| `probe_elementwise_add_fixed_no_assert` | 4 |
| `probe_vectorises_verbatim_control` | 4 |

So the claim is correct, and this is the first time it has actually been checked against the real
crate rather than carried forward from file 32's differently-flagged measurement. It is not, however,
true unconditionally, which is the second half of the finding and the reason the flag matters enough
to name: a minimal two-function standalone reproduction of the identical source, built with every
flag file 34 states EXCEPT `-C codegen-units=1` (the rustc default, 16, applies instead), vectorises
BOTH the assert idiom and the bare idiom identically. Only adding `-C codegen-units=1` back in
reproduces the split. Neither `26_consolidation_two.md`, nor `32_probes/OUTCOMES.md`, nor
`34_giesen_the_three_halves_assembled.md`, nor `34_probes/probe_0_revectorise.sh`'s own `COMMON`
variable comment states that this specific finding (as opposed to the LTO fix, which file 34's own
prose is careful about) depends on codegen-units being pinned to one; the flag is present in the
build command by inheritance from the earlier LTO investigation, not because anyone identified it as
load-bearing for the idiom-sensitivity claim specifically.

Nothing about this finding overturns file 34's own conclusion. The posture it draws from the finding
("do not lean on autovectorisation as a guarantee," `32:325-334`, which file 34 explicitly keeps) is,
if anything, better supported by a claim that turns out to be flag-sensitive in an undocumented way
than by one that was not. What this finding corrects is narrower and load-bearing anyway: a claim
this review had repeated four times as settled had never actually been re-verified under the
methodology its own author used to fix everything else in the same paragraph, and the specific reason
it happened to still be true was never named. `codegen_regression_harness.rs`'s own doc comment on
this test states the flag dependency inline, so the next reader who runs this suite without `-C
codegen-units=1` (the rustc default) and gets a different answer reads that as the documented
sensitivity, not as a fresh regression to chase.

## 5. What this dispatch did not do, stated as owed rather than silently dropped

**The `.stderr` files are not yet trybuild-confirmed.** Section 2's own caveat, restated here because
it belongs in the open list: the exact rendered text needs a real `TRYBUILD=overwrite` run against
the shipped tower to confirm byte-for-byte, which cannot happen before the tower has a home in real
source. The error CODES are verified now (`52_probes/OUTCOMES.md`); the exact text is the one piece
of this dispatch's own claims graded `pin`-only rather than `pin + tree`.

**The codegen harness resolves its fixtures by a relative path (`FIXTURE_DIR`) that assumes the
harness and its `codegen/` sibling land together.** True at both the destination
(`mock/crates/arvo/tests/codegen_regression.rs` + `tests/codegen/*.rs`) and in this probe directory;
stated because a future mover who relocates one without the other gets a panic with a clear message
(`fixture {fixture} failed to compile`) rather than a silently wrong result, and I checked that the
panic message is the one a mover would actually see, not a generic `unwrap` failure.

**Whether `-C codegen-units=1` should be the review's STANDING flag for every codegen-quality
question, not only this one test's.** File 34's own lesson (`34:110-118`, "axis legibility reads the
check build, codegen quality reads a shipping-shaped build") already argues for a fixed flag set per
question class; section 4 adds one more concrete instance of why the flag set has to be stated, not
merely used. I did not audit every OTHER codegen claim in the review (files 08, 24, 25, 27, 43, 50,
51's own instruction counts) against this exact question, and a member picking up the review's own
"one codegen test per question class" recommendation next should treat that audit as part of the
job, not as separately owed.

**Whether the seal's four introduction routes are actually exhaustive**, as opposed to "exhaustive
by every route this review's members have tried." Section 1.12 of the consolidation states the
four-route enumeration as a fact about the language (`49:364-378`), and I did not independently
re-derive that enumeration from the reference or attempt a fifth route nobody has tried; I verified
that the seven attacks this review already found each land in one of the four named routes and that
all four are represented, which is weaker than proving the four are the whole space.

## 6. Provenance summary

Compiled, this dispatch, fresh, on the pinned nightly, redone after the toolchain-resolution mistake
section 4's own paragraph names: every artifact under `52_probes/`, all eight seal-adversary
outcomes, both projection-chain outcomes, all six codegen-test outcomes, the corrected `34_probes`
re-measurement in section 4, and the two-function flag-sensitivity isolation that found `-C
codegen-units=1` load-bearing. Read and quoted rather than recalled: `49_consolidation_four.md`,
`50_fog_the_float_model.md`, `51_fallin_the_last_tick_and_the_licence.md`,
`46_probes/OUTCOMES.md`, `47_probes/OUTCOMES.md`, `48_probes/OUTCOMES.md`,
`34_giesen_the_three_halves_assembled.md`, `34_probes/probe_0_revectorise.sh`,
`32_probes/identity_model/src/lib.rs`, `35_dolan_does_widening_collapse.md`,
`43_smith_division.md`. Reasoned, not compiled: sections 1 through 3 in full (the
measurement-versus-contract classification, the name-versus-body audit, the compile-fail-for-the-
wrong-reason discipline), each grounded in a compiled artifact named at the point it is used rather
than asserted free-standing.
