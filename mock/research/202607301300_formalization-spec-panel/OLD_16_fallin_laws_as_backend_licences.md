# 16: Laws as backend licences, checked against the toolchain rather than argued from it

**Reviewer:** Chris Fallin (instruction selection, register allocation, and mid-end optimisation lens:
what a compiler backend is actually licensed to do with a fact, what survives the trip from source to
machine code, and whether a checked definition and an executed definition are provably the same text).

**What I read.** `11_current_shape_draft.md` in full. `13c_op_the_standard_and_the_mode.md`,
`13_mcsherry_where_the_laws_belong.md`, `13b_op_checkpoint_five.md`, `14_dolan_which_algebra_is_this.md`
and its `14_probes/`, `15_willsey_what_a_law_is_for.md` and its `15_probes/`, all per the brief.
`12_lattner_fresh_read.md` in full for the arithmetic-fidelity finding (section 1), which is the finding
my dispatch descends from. `03_jhala_what_is_provable.md` section 3, which the brief did not name but
which turns out to be the single most load-bearing prior file for the last third of this dispatch; I
read it because file 15 flagged "verified lowering" as a question worth someone's time and I wanted to
know whether the earlier dive had already touched the solver question before I reasoned about it from
scratch. `ls` on the panel directory, on `mock/research/`, and on `mock/design_rounds/` before reading
inside any of them, per the standing instruction; nothing outside what the brief and files 12 through 15
already named bore on this question.

On source: `arvo-strategy/src/arith_macros.rs` in full, `arvo-strategy/src/arith.rs` (the trait
declarations, for the doc comments describing per-strategy semantics), `arvo-strategy/src/identity.rs`
(to confirm what is and is not shipped, distinct from what the draft proposes), `arvo/src/float.rs` in
full, `arvo/src/fixed_scale.rs`, `arvo-spectral/src/power.rs`, `arvo/build.rs`. Then, because the
question is centrally about what a fact does once it leaves arvo, I read the mechanism the draft's own
doc comments point at: `hilavitkutin-build/src/{bootstrap,config,profile,pragma,lib}.rs`, to trace where
the `arvo_fast_math` cfg and the "LLVM `unsafe-fp-math` flag" `lib.rs:40` claims actually get emitted.
This crosses into hilavitkutin, which `13c` ruled is not this review's concern for the engine's own
defects; I read only the build-pragma-to-codegen-flag path, which is the one piece of hilavitkutin that
bears directly on my assigned question (what a derived fact would have to reach to become a backend
licence), and I report only what that trace shows, not an opinion on hilavitkutin's engine quality.

Also read for the toolchain layer specifically: `unstable-features.md`'s forbidden table (the
`core_intrinsics` row) and `arvo-always-optimal-internals.md` in full, both already loaded ambiently in
this session, cited by path below rather than quoted at length since prior files (12, 13, 15) already
quote the load-bearing passages.

**What I compiled and ran, as distinct from what I reasoned about.** One probe,
`16_probes/01_no_stable_per_op_fast_math.rs`, `rustc +nightly-2026-05-28`, two builds against the pinned
toolchain: ungated (expect failure) and `--cfg gated` (expect success). Result in section 2. I also ran
`rustc +nightly-2026-05-28 -C help` and `-Z help` against the pinned toolchain directly, grepped for any
fast-math-shaped codegen or debug flag, and found none; that negative result is reported as a measurement,
not an inference, in section 2 as well. Everything else here is argument, offered as directions rather
than rulings, and where I hold more than one reading I say so and do not resolve it for the next member.

## 0. A premise check, and where this sits relative to the rest of the dive

There is no ratified canon governing this question, same standing as every file in this dive: `13c`
states the fixed test (optimal, ideal, representative of the mathematics, capable of representing
MATLAB/IEEE-754/SystemC) and declines to rule on mechanism ahead of time, and
`panels-argue-the-intent-not-the-wording.md` is the operative posture. I am not defending a locked design
against drift; I am testing whether a specific proposed direction (file 15's closing move, "the right
move is... to make sure arvo emits the annotations... that let whichever backend the build target uses
do that search safely, on arvo's behalf") survives contact with the actual toolchain, the way file 13
tested file 12's mechanism claim and found the claim right about where the trouble is and wrong about
what the trouble is made of.

I want to be exact about what I am and am not doing to file 15's argument. Its section 6 is, on its own
terms, careful: it says plainly that it did not verify which backend arvo's consumers build against and
did not go looking, and it offers the LLVM-annotation direction as a direction, not a finding. What
follows is the looking. The short version, stated now so the sections below read as evidence for a
conclusion rather than a mystery: **the correspondence the brief asks about is real for exactly one
narrow, already-stable, already-licensed case, and for everything else the "backend" side of the mapping
either does not exist in this toolchain or does not exist at all for arvo's primary type family. The
place the correspondence is real and already has a mechanism is not a backend. It is arvo's own dispatch,
and it is the thing Thread C already builds.**

## 1. What "a backend license" means in C and what it means in Rust are not the same thing, and the gap is the whole answer

The mental model the brief's framing and file 15's closing section both reach for, understandably, is the
C and C++ one: `-ffast-math` (or its LLVM-level constituents, `reassoc`, `contract`, `nnan`, `ninf`,
`nsz`, `arcp`, `afn`) is a compiler flag, and the field's entire distrust of it (file 15 section 6, citing
the same LLVM `InstCombine`/`Reassociate` pipeline this design's `FastFloat` rides on) is distrust of a
*flag-shaped* license: something that turns a whole compilation unit's floating-point codegen loose at
once, indiscriminately, with no confluence guarantee and no way to say "only this expression, because I
proved it safe."

Rust does not have that flag, in the sense of a first-class `-C fast-math` option a build script can set
per crate, and this is a measured fact about the toolchain, not an inference. I ran `rustc
+nightly-2026-05-28 -C help` and `-Z help` against the pinned toolchain and grepped the full output for
anything matching `fast`, `math`, `fp`, `reassoc`, `contract`, or `nan`. The only near-hits were
`-Z saturating-float-casts` (an unrelated cast-UB fix, on by default) and `-Z contract-checks` (contract
programming, unrelated to floating-point contraction). There is no `-C` or `-Z` option shaped like
`-ffast-math` on this toolchain at all. The nearest thing rustc offers is `-C llvm-args=val`, an escape
hatch that forwards raw arguments to LLVM's own command-line parser, which is exactly as coarse as it
sounds: whatever it does, it does to the whole compilation unit LLVM sees, with no Rust-level scoping to
one type, one function, or one expression.

## 2. The one lever that IS per-operation is forbidden, and the probe shows exactly where the wall is

Rust does have a per-operation mechanism, and it is real, and it is the thing file 15's "emit the
annotations" direction would actually need to reach for: `core::intrinsics::fadd_fast` (and its siblings
`fsub_fast`, `fmul_fast`, `fdiv_fast`, `frem_fast`), which lower directly to LLVM's per-instruction `fast`
flag on the corresponding floating-point instruction. This is genuinely finer-grained than any compiler
flag: it is attached to one SSA value, at the point rustc's own codegen decides to emit it, which is
exactly the kind of thing a per-composition arvo fact could in principle gate.

It is also, per `unstable-features.md`'s forbidden table, exactly the feature this workspace has already
and separately ruled out: `core_intrinsics` sits in the forbidden list as "compiler-internal surface, not
intended for general use... intrinsics are unlikely to ever be stabilized, use stable wrappers instead."
I did not take that on the rule file's word; `16_probes/01_no_stable_per_op_fast_math.rs` compiles the
claim directly. Ungated, `rustc +nightly-2026-05-28` refuses the `fadd_fast` call with `E0658`, and the
compiler's own diagnostic states the reason in almost exactly the rule file's words: "intrinsics are
unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of
the standard library." Gated (`--cfg gated`, adding `#![feature(core_intrinsics)]`), the same file compiles
clean (one `internal_features` warning) and runs, printing `fadd_fast(1.0, 2.0) = 3`, confirming the
intrinsic is real, sound in isolation, and simply off-limits.

The same probe compiles, gate-free, on both builds: `a.mul_add(b, c)`, stable since long before this
workspace existed, prints `mul_add(2.0, 3.0, 1.0) = 7`. That is `llvm.fmuladd.f32`, a single per-call-site
fusion of one multiply and one add into one rounding step, exactly the operation Lattner's finding names
(`12_lattner_fresh_read.md:92-96`) and exactly the operation `arvo-always-optimal-internals.md` already
names by its LLVM intrinsic ("`llvm.fmuladd`, `llvm.umul.with.overflow`, `llvm.bswap`... when the named
instruction is what the hardware actually does"). It needs no feature gate, no cfg, no build-script
coordination with hilavitkutin-build, nothing except a plain method call at the call site the fusion is
wanted.

So the toolchain answer to "what survives the trip through rustc and LLVM" splits cleanly into two
buckets, and the split is not a design choice this workspace made; it is where Rust's stabilisation
history happens to have drawn the line. **A whole class of per-instruction fast-math behaviour is real,
sound, and completely inaccessible without a feature this workspace has already, correctly, forbidden.
One specific, nameable instance of it (fused multiply-add) is stable, explicit, and requires no compiler
cooperation at all beyond calling the function.** There is no third option on this toolchain today: no
stable per-expression fast-math attribute, no stable way to mark one `Number<N, S>` composition's
arithmetic as reassociable while leaving a neighbouring composition alone within the same compilation
unit.

## 3. FastFloat and StrictFloat, checked against the source rather than the doc comment, deliver identical codegen today

Before asking what a *future* fidelity axis should attach to, I checked what the *shipped* fidelity
distinction (the one the draft's headline unification claim already treats as real,
`12_lattner_fresh_read.md:65-80`) actually does. It does nothing.

`float.rs:29` states, as a doc comment, "LLVM fast-math flags apply to arithmetic through this type."
`float.rs:179-213` is the entire arithmetic surface for both wrappers, and it is one macro,
`float_binop_impl!`, invoked identically for `FastFloat` and `StrictFloat`
(`float.rs:203-207` against `float.rs:209-213`): both bodies are `Self(<F as core::ops::$op>::$method(self.0,
other.0))`, a plain delegation to the inner primitive's `core::ops` impl, with no intrinsic, no cfg
branch inside the operation, and no attribute distinguishing one wrapper's codegen from the other's.
There is no code path in this crate through which a `FastFloat<f32>` addition and a `StrictFloat<f32>`
addition could compile to different LLVM IR. They are, today, the same instruction with a different type
name wrapped around it.

The `arvo_fast_math` cfg (`float.rs:219-227`) only ever changes which wrapper the `Float` alias resolves
to; it does not touch either wrapper's own arithmetic. So the question of whether a whole-build flag
would apply indiscriminately to both wrappers, which is the worry I expected to find (a Rust `-C
llvm-args` flag, being module-wide, cannot distinguish `FastFloat` call sites from `StrictFloat` call
sites in the same crate even if one existed), turns out to be moot for a sharper reason: no such flag is
emitted at all. I traced the claim `hilavitkutin-build/src/lib.rs:40` makes ("`FastMath` | LLVM
`unsafe-fp-math` flag plus the `arvo_fast_math` cfg") through every file in that crate that could plausibly
emit a codegen flag (`bootstrap.rs`, `config.rs`, `profile.rs`, `guards.rs`) and found exactly one
`println!` that reaches rustc: `bootstrap.rs:29`, `println!("cargo::rustc-cfg=arvo_fast_math")`. No
`-C llvm-args`, no `rustc-link-arg`, no `RUSTFLAGS` construction anywhere in the crate. `Pragma::FastMath`
carries a bit position (`pragma.rs:39,61`) and nothing downstream of the pragma set turns that bit into an
actual LLVM instruction. The doc comment overclaims what the crate does by exactly the half that would
have made `FastFloat` and `StrictFloat` diverge.

I hold this as a finding about the current shipped state, not a claim about design intent; someone may
already know the LLVM-flag half is planned and unbuilt. But it changes how the rest of this dispatch
should be read. **The place this design already gestures at a backend license (`FastFloat` versus
`StrictFloat`) is not a coarse license today. It is no license at all.** Whatever correspondence the brief
asks me to evaluate has to be evaluated against a mechanism that does not yet exist on either side: not
the derived-fact side (Lattner's fidelity axis is proposed, not shipped) and not the backend side (the
flag `FastFloat`'s own doc comment promises is not emitted).

## 4. Fixed point has no backend to license at all, which is a stronger statement than "the license is coarse"

Section 6 of file 15 already states this correctly and I want to sharpen it rather than repeat it,
because the phrasing matters for what follows: "fixed-point arithmetic (Hot/Warm/Cold/Precise) gets no
reassociation license from LLVM at all, because LLVM has no concept of `wrapping-fast-math` or
`saturating-fast-math` for integers."

This is not merely true; it is categorical, and it means the entire "derive a fact, hand it to the
backend" framing does not apply to arvo's own load-bearing type family at all, not "applies coarsely."
LLVM's integer instructions (`add`, `mul`, `sub`) carry exactly two optional overflow-behaviour flags,
`nsw` (no signed wrap) and `nuw` (no unsigned wrap), which are *narrower* promises than anything this
design's laws discuss (they say "this specific instruction's result, if it overflowed, would be undefined
behaviour, so the optimiser may assume it did not," which is a UB-shaped promise incompatible with
`Hot`'s entire premise of *defined* wraparound) and they say nothing whatsoever about associativity,
distributivity, or reassociation permission. There is no `reassoc` flag on an integer add in LLVM IR.
There never has been, and no RFC or LLVM proposal I am aware of adds one, because integer arithmetic's
associativity is a property of the *operation chosen* (wrapping versus saturating versus checked), not a
property the optimiser needs a hint to exploit; wrapping integer addition over a fixed width already IS
associative (file 13's own probe 01, `13_mcsherry...md:210-222`, "stable implies associativity... with
zero counterexamples" for the one map that is both), and LLVM's `InstCombine` already reassociates
wrapping-integer expressions on that basis alone, with no annotation from arvo needed, because the
mathematics is already sound for that one composition without anyone asking permission.

So for the presets whose reordering McSherry's committed bench actually prices (`13_mcsherry...md:364-396`,
the 2x single-thread win from splitting one accumulator into four), there is no gap for a backend license
to fill, and there never will be, because the reassociation McSherry measured is not something LLVM's
`InstCombine` performs automatically at a source-level API boundary like arvo's: `fold_quad`'s four
independent accumulator variables are a *source-level* transformation, chosen by whoever wrote that loop
body, not a transformation LLVM discovers on its own inside a single scalar accumulator (LLVM's own
auto-vectoriser and SLP-vectoriser can sometimes find this, under specific loop shapes, with `-C
opt-level=3`, but this workspace's own `arvo-always-optimal-internals.md` explicitly does not trust
that: "we do not rely on rustc + MIR + LLVM stability for anything we want to behave a specific way...
for anything we want to lower a specific way, we cfg-gate it explicitly"). The regrouping McSherry priced
is a hand-written source transformation gated by whether the operation is associative; it is exactly a
"which of several algebraically-equal regroupings is fastest" question answered by *writing the fast
regrouping in source and choosing between source variants*, which is precisely the shape a derived
`Associative<Op>` fact needs to gate, and precisely the shape that has nothing to do with LLVM at all.

**The correspondence the brief asks about, restated honestly against what I have found: it is real for
float (barely, and not shipped), it is a non-question for fixed point (there is no LLVM-side concept to
license), and the actual place fixed-point regrouping facts need to attach is not a backend, it is
arvo's own choice of which source-level function body to compile.**

## 5. Where the correspondence is real, it already has a mechanism, and the mechanism is not new

This is the direct answer to the brief's "what is the interface, and what would it take for a fact
proven in arvo's type system to reach a place that could use it." For the case that matters most
(fixed-point regrouping, which is where the real, benched, real-money win lives per file 13 section 7),
the fact does not need to *reach* anywhere. It needs to gate which of several already-written function
bodies gets selected at monomorphisation time, inside arvo's own crate, and that selection mechanism is
exactly what draft section 4.3's Thread C already builds: a `[const]` generic function, checked
exhaustively at a small model width, executed unmodified at the real width, one text serving both roles.

I want to be precise that this is not yet shipped. `grep`ing `mock/crates` for `AddAssoc`, `Magma`,
`Semigroup`, or `Monoid` returns zero hits (confirming files 12 through 15's identical finding), and
`identity.rs:47-54` ships only `Identity<Op>`, the bottom rung. What Thread C's fifth pass demonstrates
(draft section 4.3, and per file 15 section 5, "provably connected to the code that actually runs") is
the *shape* a derived-fact-gated function selection would take, verified against a probe in this dive's
own predecessors, not against shipped source. Dolan's reading two (`14_dolan...md:255-296`) and
McSherry's bench (`13_mcsherry...md:364-396`) already converge, independently, on exactly this: `Associative<Op>`,
`Monotone<Op, Ord>`, and whatever `Distributes<Op1, Op2>` turns out to need (section 6 below) are the
fact set, and the consumer is a combinator that picks the four-way-split body over the naive body, gated
on the fact, checked and executed as one text.

None of this needs to leave the crate. The "backend" that has to be given a license to reorder arithmetic
is not LLVM. It is arvo's own dispatch, sitting behind `arvo-always-optimal-internals.md`'s own freedom,
already fully controlled by arvo's own type system, already the place `AddAssoc` and its future siblings
are designed to live. The mapping problem file 15 named as smaller and more honest than building an
e-graph is real, and it turns out to be smaller still than file 15's own framing suggested, because the
"whichever backend the build target uses" clause does not apply here at all; the backend is arvo itself.

## 6. The one real per-instruction lever, and the direct collision it has with a standing rule

Fused multiply-add is the exception to section 4's "no backend concept exists" claim, because it is not
a reassociation license at all; it is a distinct, named, stable operation (`f32::mul_add`,
`f64::mul_add`) that fuses one multiply and one add into a single rounding step, and it applies to float,
not fixed point (arvo ships no fixed-point `mul_add`; the fixed-point multiply-then-add sequence the
same collision would apply to is two ordinary calls, `u_mul_fixed` then `u_add`, with no fused stdlib
equivalent, though nothing stops arvo writing its own).

Lattner's finding (`12_lattner_fresh_read.md:92-97`) is that `arvo-always-optimal-internals.md` already
licenses this substitution by name ("LLVM intrinsics directly (`llvm.fmuladd`... etc.) when the named
instruction is what the hardware actually does") with nothing distinguishing a composition where the
substitution is a silent value change from one where it is not. My probe confirms the mechanism this
license would actually use, `mul_add`, is stable, requires no feature gate, and compiles to exactly the
single-rounding-step instruction the collision is about. So this is not a hypothetical risk to be
designed around later; it is a rule that today licenses a specific, real, textually-locatable
substitution (any internal hot path computing `a * b + c` on a float-backed composition may reach for
`.mul_add()`, per the rule's own worked list) with no requirement anywhere that the substitution first
check whether the composition's fidelity permits it. The fix, on the evidence gathered across this whole
dive, is not to weaken `arvo-always-optimal-internals.md`'s freedom (the rule is correct that internals
should reach for the fastest sound codegen) but to make the freedom conditional on a fact that does not
exist yet: whichever axis eventually distinguishes `Exact` from `Relaxed` fidelity (Lattner's proposal)
needs to be the gate an internal `mul_add` substitution checks, at the same call site, the same way a
future `Associative<Add>` gate would decide whether the four-way accumulator split is legal.

## 7. Multiplication changes what the fact set even needs to be, and it changes it before anyone gates anything

File 15's own probe (`15_probes/01_distributivity_over_add_shipped_mul.rs`) is required background for
this dispatch and I want to state plainly why it matters to mine specifically, not only to the
algebra-ladder question it was written for: it establishes that the shipped fixed-point multiply
(`arith_macros.rs:33-34,95-101,147-148,214-220,464-465,510-511`, confirmed by my own read of the same
file, six macro sites, every one an unconditional `wrapping_mul` then `>> FRAC`, no rounding, no
ties-to-even, for every strategy including the ones the draft's own preset table, `11_current_shape_draft.md:327`,
says round to nearest-even) is not merely untested against the laws, it computes a different function
from what any future `Quantisation`-derived fact would be describing if it read the preset table rather
than the code. A `Distributes<Mul, Add>` marker gated on the *intended* per-strategy rounding mode would
be a marker about a function that does not exist; a marker gated on the *shipped* truncation is the only
one that could currently be checked-and-executed as one text under Thread C's own discipline (draft
section 4.3's central requirement).

This bears on my dispatch because it means the backend-licence question for multiplication cannot even be
asked coherently yet. `arvo-always-optimal-internals.md`'s freedom to reach for `llvm.fmuladd` collides
with a fidelity axis that does not exist (section 6); a hypothetical `Distributes<Mul, Add>` fact that
would gate a fixed-point regrouping analogous to McSherry's accumulator split cannot be derived yet
because the function it would be a fact about is undefined at the design level (draft 3.5's stated
intent) and defined-but-untested at the shipped level (this file's macro trace, file 15's probe). I would
not build any multiplication-side gate before that gap closes, and I would read file 15's own
recommendation the same way it reads itself: the fidelity axis and the multiplication dive are "the same
question asked twice" (`15_willsey...md:187-188`) and should not proceed independently.

## 8. Granularity: the atomic facts are exactly the right size for arvo's own dispatch, and structurally cannot be the right size for LLVM

The brief asks whether the derived facts are even the right granularity for a backend. Given section 4's
finding (there mostly is no backend to be the right or wrong granularity for), the sharper version of the
question is: could a per-composition fact ever be the right granularity for an LLVM-level license, even
in principle, on some future toolchain that stabilised a per-expression fast-math attribute?

No, and the reason is structural rather than a toolchain limitation that might someday lift. By the time
Rust's MIR lowers to LLVM IR, a `Number<N, S>`'s type information does not survive; what LLVM sees is a
sequence of `i32`/`i64`/`f32` (or wider, via the limb machinery for `WideBits`) operations with no
residual tag saying which arvo composition produced them. A per-composition marker trait like
`Distributes<Mul, Add>` is exactly the right size to gate a *choice rustc itself makes while it still has
the type* (which monomorphised function body to emit, whether to call `.mul_add()` at this call site),
because that choice happens before type erasure. It cannot be the right size for a license an LLVM pass
consults *after* type erasure, because there is nothing left post-erasure for the license to be *about*;
an LLVM `InstCombine` pass reasoning about reassociating a chain of `fadd`s has no way to ask "was this
fadd produced by a `Number<N, S>` whose `Distributes` fact was true," because the question does not
type-check at that layer, literally. The one place per-instruction fast-math flags in LLVM *do* survive
type erasure (`fadd fast`, `fmul fast`) get there because rustc itself sets the flag at codegen time,
which is exactly the `fadd_fast` intrinsic path section 2 shows is forbidden. So the granularity question
resolves cleanly: **the atomic facts are the right shape for the only place a fact can actually be
consulted, which is arvo's own source, before erasure. There is no post-erasure destination they could
ever be resized to fit, on any toolchain that keeps type erasure at the LLVM boundary, which every
mainstream compiler does.**

## 9. What verified lowering has to say, and where I think this dive's earlier finding (Jhala, file 03) already answered half of it

The brief asks whether verified lowering has anything to say about the design's central problem, the one
Thread C's fourth pass found (`11_current_shape_draft.md:606-617`): a checked classification and an
executed arithmetic pipeline can both individually pass every test while silently disagreeing with each
other. I want to credit `03_jhala_what_is_provable.md` before answering, because it already found the
relevant fact from the earlier dive and I do not think this design has acted on it.

Jhala's section 3 (`03_jhala...md:124-136`) establishes that every leaf fact this design's derivation
machinery checks, once a width is fixed, is a statement in a decidable fragment (bounded integer
arithmetic with min, max, and modular reduction, over a finite representable set), Presburger-shaped, and
"any SMT solver discharges it for a fixed width in milliseconds." Jhala is explicit that the reason no
solver runs is "workspace policy stated in the brief, not a fact about the mathematics," and the honest
substitute proposed there (bounded exhaustive const-eval, which Thread C's fifth pass then built and
verified) is offered as the cheap alternative given that policy, not as the mathematically strongest
option available.

Here is what my own field adds to that, and it is the direct answer to the brief's question. My work on
ISLE (Cranelift's term-rewriting DSL for instruction selection) exists because "checked definition and
executed definition can silently diverge" is not a novel problem this design happened to discover; it is
the load-bearing failure mode every production lowering pipeline has to solve, and the field's answer is
not bounded exhaustive checking at one small model width plus a prose transfer argument. It is machine
verification against an SMT solver, run offline, over the *actual* rule text, checked to hold for *every*
input the type permits, not sampled at a representative width. `08_fog_the_union_and_what_it_costs.md`'s
own finding (cited by Dolan, `14_dolan...md:180-182`) is that bounded exhaustive const-eval "quadruples
per bit, costs 28.45 seconds at 8 bits, and is refused by `#[deny(long_running_const_eval)]` at 9," which
is exactly the wall SMT solving does not hit, because a solver reasons over the bit-vector symbolically
rather than enumerating; a query like "does this recovery rule's fold satisfy translation stability for
every 64-bit signed input" is the same decidable fragment Jhala names, and a modern SMT solver (Z3, CVC5,
Bitwuzla) answers it directly, at the real width, not a proxy width, typically in well under a second.

The reason nobody has proposed this in either dive, as far as I can find, is the same reason Jhala names:
the workspace's standing preference against a compile-time solver dependency, stated in `03_jhala...md`'s
own brief. But that preference, read carefully, is about a **compile-time** dependency, something
`const fn` would need to invoke on every build. An offline, CI-time verifier that checks arvo's own
`[const] fn` recovery-rule bodies against an SMT encoding, the way Cranelift's ISLE verifier checks
lowering rules against CVC5 (Chris Fallin, Fraser Brown, and others, "Cranelift's ISLE verifier," part of
the broader Cranelift verification effort) is not a runtime dependency, not a `no_std` violation, not a
const-eval cost, and not a thing any consumer of arvo ever needs to install: it is a tool that runs
against source text at design time and either confirms or refutes a claimed fact, the same way a
formatter or a linter does, decoupled entirely from the compiled artifact. It would close exactly the gap
the draft's own ledger names as permanently unmechanized (`11_current_shape_draft.md:840-842`, "the
width-uniformity transfer argument itself... stays prose forever, is never mechanical") without touching
any of the constraints this design actually has to respect (`#![no_std]`, no `alloc`, no `TypeId`, no
specialization); those constraints bind the *compiled artifact*, and an offline verifier is not part of
the compiled artifact.

I want to state this as a direction, not a recommendation to build it now. It is new tooling, it is a
real design and engineering cost, and D47's sketch-and-bench obligation does not currently include SMT
verification as part of what a rung earns its place by. What I am confident of is narrower: the "no
solver" policy, read as forbidding SMT verification of leaf facts entirely, forecloses the one mechanism
that would let the width-uniformity transfer argument stop being prose, and I think that specific
consequence of the policy was not weighed when the policy was set, because Jhala's file already named the
tractability and nobody has revisited it since.

## 10. Direct answers

**Is there a genuine correspondence between a derived law fact and a backend license?** Real for exactly
one case (fused multiply-add, section 6), non-existent for arvo's primary type family because LLVM has no
integer-reassociation concept to license at all (section 4), and unbuilt for the case where the design
already gestures at one (`FastFloat`/`StrictFloat`, section 3, where the codegen is identical today
regardless of what the doc comment claims). "Coarse per-function fast-math flag set" as a starting
premise for this question was itself the wrong premise: Rust does not have a per-function fast-math flag
at all, coarse or otherwise; it has a whole-compilation-unit flag that is not even wired up on the arvo
side, and a per-instruction intrinsic that is forbidden.

**What is the interface?** For the case that matters (fixed-point regrouping, the real 2x McSherry
priced), there is no interface to design, because the fact never needs to leave arvo. Thread C's own
mechanism, already designed if not shipped, is the interface: a derived fact gates which monomorphised
function body compiles. For fused multiply-add, the interface is a plain method call gated by a fidelity
fact that does not exist yet and needs to (section 6, section 7).

**What survives the trip through rustc and LLVM?** Type information does not survive past MIR lowering,
structurally, on any mainstream compiler (section 8). Nothing an arvo type derives can attach to an LLVM
pass after that point without an intrinsic this workspace has already, correctly, forbidden. What
survives is source-level choice, made before erasure, which is exactly what monomorphisation-gated
dispatch already is.

**Is this a mapping problem or a search-engine-building problem, as file 15 framed the choice?** Neither,
for the case that actually pays. It is neither building an e-graph (file 15's rejected option) nor
mapping a fact to a backend annotation (file 15's proposed option); it is finishing Thread C and widening
its fact set, which this dive's members (Dolan, McSherry, Willsey) already converge on independently of
my question. The backend-facing half of file 15's proposal, section 6's "make sure arvo emits the
annotations that let whichever backend the build target uses do that search safely," is the part I would
not build, because for fixed point there is nothing on the other end to receive the annotation, and for
float the annotation mechanism itself does not exist on stable Rust without the forbidden intrinsic.

## 11. What I would flag for the next member, unresolved

**`arvo-always-optimal-internals.md` licenses the `llvm.fmuladd` substitution today with no fidelity
check gating it, and this is a live, textually-locatable collision, not a hypothetical one** (section 6).
I did not audit whether any shipped internal body currently reaches for `mul_add`; I would want that
grepped before anyone treats this as purely prospective.

**The offline SMT-verification direction (section 9) is genuinely new to this dive and to the earlier
one on the evidence I found, and I did not sketch it.** It would need its own feasibility check (can
arvo's actual recovery-rule function bodies, as `[const] fn`, be translated to an SMT encoding without
hand-transcription; existing Rust-to-SMT tooling such as Kani, built on CBMC, is the obvious starting
point to check against, and I have not verified it handles const-generic-width-parameterised functions
the way arvo's would need). I would not build this before someone runs that check.

**I did not check whether arvo's own shipped internals (as opposed to the macro bodies I read) already
perform any regrouping (a hand-unrolled accumulator, a manually fused multiply-add) anywhere today.**
McSherry's finding (`13_mcsherry...md:387-395`) is that this licence exists and has been used at least
once in a bench variant; whether it has shipped inside a real arvo crate body, gated by nothing, is a
grep someone should run before the fidelity axis lands, so the audit has a concrete list rather than a
hypothetical one.

**Whether the fixed-point family should get its own explicit fused-op surface** (an `mul_add_fixed`
analogous to the float stdlib's `mul_add`, rather than leaving the only fusion lever to float) is a real
design question this dispatch surfaced and did not answer. If McSherry's bench-priced regroupings and
Lattner's FMA collision are, per section 6 and section 4, the same *kind* of gap (a fact-gated
source-level substitution, never a backend annotation) for both type families, symmetry argues for one.
I hold this as a direction, not a finding; nobody has benched a fixed-point fused multiply-add against
the two-op sequence, and per `bench-and-sketch-discipline.md` that number belongs in `mock/benches/`
before anyone builds the surface.
