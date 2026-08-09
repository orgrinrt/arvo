Chris Fallin, file 51. I wrote file 16, which argued laws are backend licences and found, at the time,
that fixed point has no such licence to give (LLVM has no reassociation concept for integers because
wrapping addition is already associative) and that float's one real per-instruction lever was `mul_add`,
already unconditionally licensed by `arvo-always-optimal-internals.md` with no fidelity check gating it.
Neither claim needed revisiting here. Both are now independently corroborated, one by measurement I did
not have access to at the time, and the second by a collision I predicted and did not build. Nothing else
of mine survives unexamined; thirty-five files landed since and I read this dispatch's question against
what they settled, not against what I remembered writing.

**What I read.** `49_consolidation_four.md` in full, as instructed, plus `50_fog_the_float_model.md`, the
only deliverable since it. I re-read the sections of my own file 16 the dispatch names (`16:155-260`) to
quote rather than recall them, and the two prior statements of the tick-3 check
(`35_dolan_does_widening_collapse.md:216-246`, `39_knuth_does_it_still_represent_them.md:307-324`) to
compile exactly the check their authors named rather than a check I invented that resembles it. I did not
open any other file; the consolidation's own instruction is that it is self-contained, and I checked that
claim by grepping it (see below) before trusting it.

**Gates.** Test gate: `cargo test --workspace` from `mock/` reports 654 passed, 0 failed, 9 ignored,
matching every file since 41. Canon gate: the surface both halves of this dispatch touch has no shipped
source. `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same command with
`FullRange\|UTerm\|AddWidth` in place of the first pattern both return nothing, reproducing the
consolidation's own corrected verification command. Nothing here is a critique of code; both parts are
design, checked against the toolchain rather than against a tree that does not yet exist.

**What is compiled or measured, and what is reasoned.** Six probes in `51_probes/`, every one built and
run on `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`. Probes 1 through 3
resolve the pin from the repo's `rust-toolchain.toml`; probes 4 through 6 needed `+nightly-2026-05-28`
stated explicitly when I ran them from outside the repo tree, and I say so rather than let the toolchain
resolution look automatic. I ran no timing loop anywhere; every claim below is either an instruction count
from `--emit asm`, an LLVM IR flag from `--emit llvm-ir`, a `const` assertion the compiler checked, or a
compiler diagnostic. Where a claim is reasoned rather than compiled I say so at the point I make it.

## 1. Tick 3: compiled, and closed harder than the check asked for

### 1.1 The check, restated exactly as its authors stated it

File 35 named it first: "whether any consumer-facing operation exists, or could exist, whose growth
behaviour genuinely cannot be read off from (which primitive, which target numeral)"
(`35_dolan_does_widening_collapse.md:243-246`), held reasoned rather than compiled because its author "did
not exhaustively search arvo's operation surface for one". File 39 corroborated it from three shipping
standards' own vocabulary (IEEE's `formatOf`, MATLAB's `ProductWordLength`, SystemC's per-expression
precision, all placing growth on the operation's signature, none on a per-numeral attribute,
`39_knuth_does_it_still_represent_them.md:307-324`) and, correctly, did not move it out of the reasoned
bin: corroboration from outside the design is not a compile. The consolidation's own restatement
(`49:283-289`, `49:808-812`) carries the same status three files later, and the persona checkpoint's
instruction is exact: "compile the exhaustive search of arvo's operation surface file 35 itself proposed,
or drop the argument; no fourth round of corroboration without an artifact" (`49:810-812`).

So the artifact owed is an exhaustive search over the operation surface for a counterexample: one
operation whose growth needs something other than (which primitive, which numeral type(s)) to determine.
Three probes in `51_probes/` compile it, and the third turns the search from an enumeration into a
theorem.

### 1.2 The positive half, compiled, and it covers ground file 35 and file 39 did not have

`51_probes/probe_1_growth_surface_enumeration.rs`. Eleven operations, every one drawn from the design's
current surface rather than invented for the probe: the four in-numeral non-exact operations (add, sub,
mul, div, no growth, same declared numeral in and out), `mul_full` over `Implicit` numerals (`49:269`,
widths add), `mulnum` over `Ranged` numerals (file 50's own addition to the surface, `50:216-289`),
`div_exact` by a type-level constant (`49:439-447`), the Euclidean pair `div_floor`/`rem` (`49:449-458`,
file 43's addition), `fold`, `fold_sequential`, and `fold_compensated` (`49:502-514`, the three-way fold
surface), and `quantize::<Src, Dst>`. Each gets its own growth trait, generic over the operand numeral
type(s) alone. None of the eleven trait declarations takes a `Policy` parameter; the probe's own header
comment says so and a `grep Policy` against the file, which I ran, confirms it (zero hits outside the
comments). Every `Out` computed reproduces a number a prior file already reported where one exists to
check against: `mul_full(p4, p3) -> p7` and `mulnum_ranged(bin32, bin32) -> p48` both match
`50_probes/probe_3_exponent_as_type.rs`'s own asserted values exactly.

This is the enumeration file 35 declined to run and file 39 corroborated from outside without running.
It is genuinely new coverage, not a restatement: file 35 had `mul_full` and one composite; it did not have
division (file 43 landed nine files later) or the `Ranged` family (file 50 landed the file before this
one). Both are now in the sweep and neither breaks the pattern.

*grounded on: `pin`, `host`; the operation list itself grounded on `49:269, 439-458, 502-514` and
`50:216-289`.*

### 1.3 Why an enumeration is not, by itself, the strongest available answer

An exhaustive check over a finite operation list only establishes what it names is true of that list. The
"or is designed" clause in section 1.10's own restatement of the question (`49:284-285`) asks about
operations that do not exist yet, and no enumeration answers that. I looked for a structural argument
instead, one that would hold for any operation the design might add, and found one that the design's own
standing constraints (monomorphisation as the only dispatch, `arvo`'s own coherence discipline, per
`.claude/rules/harness-the-type-system.md` and every sealed-carrier argument since file 42) already
supply.

### 1.4 The structural argument, compiled in both directions

`51_probes/probe_2_policy_threaded_is_inert.rs` and `51_probes/probe_3_policy_dependent_growth_refused.rs`
together exhaust the two ways a hypothetical `Policy::Growth` could actually change an operation's result
numeral, and both are refused or shown inert.

**Shape A: thread `Policy` into the growth trait's own parameter list.** This compiles; Rust does not
require a generic parameter to be read in an impl's body. Probe 2 builds `MulFullGrowth<N1, N2, P: Policy>`
with two policies, `PolicyA` (`Growth = GrowUnbounded`) and `PolicyB` (`Growth = GrowBounded`), and forces
the two projections, `<MulFull as MulFullGrowth<N8, N16, PolicyA>>::Out` and the same with `PolicyB`,
through a function signature that requires them to unify. It compiles, and the `const` assertion at the
bottom (`<...PolicyA>::Out as Numeral>::P == <...PolicyB>::Out as Numeral>::P`) holds. The parameter is
legal and computes nothing: adding `Policy` to the signature does not give the impl anything to change
`Out` with, because `mul_full`'s own closure formula (`49:269`) already fixes it in terms of `N1` and `N2`
alone.

**Shape B: make `Policy` actually change the answer, without adding it to the trait's parameter list.**
The only way to do this is to write two implementations of the same growth trait for the same
`(MulFull, N1, N2)` domain that disagree on `Out`, and let something outside the trait's own signature
(a policy, live in the calling context) decide which one applies. Probe 3 writes exactly this, one impl
returning `MulNum<N1, N2>` and a second, fully generic over the same `N1: Numeral, N2: Numeral`, returning
a different type, `HalfMulNum<N1, N2>`. rustc refuses before reaching any question about which one is
correct:

```
error[E0119]: conflicting implementations of trait `MulFullGrowth<_, _>` for type `MulFull`
```

This is not an artifact of my particular choice of `HalfMulNum`; it is coherence, the rule that lets the
design dispatch entirely through monomorphised trait impls with no `dyn`, no `TypeId`, and no
specialization beyond the `min_specialization` carve-out (which does not touch this: specializing on a
type-equality condition is not what would be needed here, since `N1` and `N2` range over the same
unconstrained set in both impls). Any two competing answers for the identical generic domain are refused
at exactly this error, regardless of which two numerals or which alternative formula is on the losing
side.

**What the two together prove.** Every operation the design has, or could design, computes its result
numeral inside one trait impl (or the equivalent, an `Add`/`Mul`/`Sub`/`Div` impl's own `Output`
associated type, a function's own return-type signature, a fold combinator's own projection). For that
impl's answer to vary "by policy" without becoming Shape A's inert parameter, two impls disagreeing on the
answer would have to coexist for the same generic domain, and that is refused by the language the design
already commits to, independent of which operation is in question. This is a stronger result than the
enumeration in section 1.2, because it is not "checked eleven operations, found none", it is "no operation
expressible in this type system's dispatch discipline can have policy-dependent growth, and the enumeration
is the illustration, not the argument."

*grounded on: `pin`, `host`; the argument itself grounded on the design's standing constraint that
monomorphisation is the only dispatch (no `dyn`, no `TypeId`) and on `min_specialization` being the only
specialization the toolchain permits, neither of which this file established but both of which are already
settled.*

### 1.5 What this settles, and the one thing it does not

`Growth` leaves `Policy` entirely. Not merely from the law's key (already ratified, `49:283-284`), the
associated type itself has nowhere left to live: no operation can read it in a way that changes behaviour
without becoming a coherence violation, and every operation the design ships already computes its own
growth directly. The ratified table at `49:686-689` should drop the `// Growth removed from the key:
RATIFIED. Removed from Policy entirely: OPEN (tick 3).` comment and the line it annotates; `Policy` carries
`Quantisation` alone, as file 35's own section 4 already proposed (`35:270-283`) pending exactly this
compile.

What this does not settle: whether some future axis, not yet conceived, might need a Policy fact that
varies growth in a way that does not route through a trait impl's own `Output` (for instance, a runtime
flag read at a call site rather than resolved at monomorphisation). The design's own standing constraint
(monomorphisation as the only dispatch) already forecloses that for the operations this design can
express, and I have not found, nor gone looking for, a way to route a growth decision through a mechanism
outside trait dispatch that this design would otherwise want. I record the boundary rather than claim to
have swept past it: the theorem is about what this type system's dispatch discipline can express, and the
discipline is a standing choice, not a law of nature.

## 2. The licence

### 2.1 What file 16 already established, and what it left open

File 16 split the "derive a fact, hand it to the backend" question into two halves that this design has
kept separate ever since. Fixed point: "there is no `reassoc` flag on an integer add in LLVM IR. There
never has been... because integer arithmetic's associativity is a property of the operation chosen..., not
a property the optimiser needs a hint to exploit; wrapping integer addition over a fixed width already IS
associative" (`16:168-180`). File 50 measured this independently and it agrees to the instruction:
"`i_sum: 8 vector adds LLVM reassociates an integer reduction freely`" (`50:445`). Nobody built a licence
for fixed point because none is needed; the two files reach the same conclusion by different routes,
which is the good kind of corroboration, not the copied kind.

Float, I held differently: "the place this design already gestures at a backend license (`FastFloat`
versus `StrictFloat`) is not a coarse license today. It is no license at all" (`16:157-158`), and the one
real mechanism I found was `mul_add`, already licensed unconditionally by
`arvo-always-optimal-internals.md`'s own worked list, with a warning I did not resolve: "nothing distinguishing
a composition where the substitution is a silent value change from one where it is not"
(`16:245-247`). File 50 is the direct answer to the first half: it measured that LLVM refuses to
reassociate a float reduction (`f_sum: 5 scalar fadd s, 0 vector fadds`, `50:445`) and named interior
safety "exactly the licence the vectoriser lacks" (`50:449-451`), reasoned but not shown to have a real
toolchain-side counterpart. That is what this section builds.

### 2.2 The mechanism exists, is safe, is per-call-site, and is on the stabilization path

`f32::algebraic_add`, `algebraic_sub`, `algebraic_mul`, `algebraic_div`, `algebraic_rem` (and the `f64`,
`f16`, `f128` siblings), gated by `#![feature(float_algebraic)]`, tracking issue
[rust-lang/rust#136469](https://github.com/rust-lang/rust/issues/136469). A stabilization PR,
[#157029](https://github.com/rust-lang/rust/pull/157029), is open on this pin's upstream. The tracking
issue's own stated motivation, which I read rather than assumed, is this design's own problem stated in
someone else's words: "A stable Rust implementation of a simple dot product is 8x slower than C++ on
modern x86-64 CPUs, with the root cause being an inability to let the compiler reorder floating point
operations for better vectorization." Two structural facts distinguish this from a compiler flag:

The method is `const fn` and requires no `unsafe`, unlike its sibling `fadd_fast` family
(`core::intrinsics::fadd_fast` etc., `unsafe fn`, "Requires that inputs and output of the operation are
finite, causing UB otherwise", and the doc comment states plainly "this intrinsic does not have a stable
counterpart"; source: `$(rustc --print sysroot)/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs`
on this pin, lines 1572 through 1654). Rust's own maintainers have already drawn the line this design
needs: the finite-assuming bundle is unsafe and will not be exposed on stable at all; the
reassociation-only bundle is safe and is the one heading toward stabilization.

It is a call, not a flag. `-ffast-math` (or its LLVM-IR equivalent, the `fast` keyword on an instruction)
applies to a whole compilation unit or a whole function; `algebraic_add` applies to the one expression the
caller writes it on. This is precisely the granularity a per-composition proof needs and a global switch
cannot give: the design's own laws are keyed on a specific term, not a whole crate.

**Measured** (`51_probes/probe_4_licence_reassoc_vectorizes.rs`, `--emit asm,llvm-ir`, this target): an
eight-element `f32` reduction written with plain `+` emits eight scalar `fadd` instructions in sequence,
reproducing file 50's own shape exactly (`50:445`'s five-scalar count is a shorter reduction; the pattern,
one scalar `fadd` per addend, is identical). The same reduction written with `.algebraic_add()` compiles
to `fadd.4s v0, v1, v0` (one vector add across all four lanes) followed by `faddp.4s` / `faddp.2s`
(pairwise horizontal reduction), the identical two-instruction shape the integer reduction already gets
for free (`add.4s` then `addv.4s`). The LLVM IR names exactly which flags license this:

```
%1 = tail call reassoc nsz arcp contract float
       @llvm.vector.reduce.fadd.v8f32(float 0.0, <8 x float> %0)
```

`nnan` and `ninf` are absent. The mechanism does not assume the operands are finite; it grants
reassociation and three companion permissions without granting "you may treat this as if NaN and infinity
cannot occur," which is exactly the assumption the design's own `Specials` axis and grade tracking cannot
tolerate silently. File 50's own measured `Ranged`-numeral behaviour (NaN payload propagation, section 4.4)
would be exactly the kind of guarantee an `nnan`-bearing licence would void; this mechanism does not carry
one.

*grounded on: `pin`, `host`, `flags` (`-C opt-level=3`, no other codegen flags); the tracking-issue and PR
text is quoted from the linked GitHub pages, read directly, not recalled.*

### 2.3 The bundle over-grants, and it over-grants in exactly the shape file 16 already flagged

`reassoc nsz arcp contract` is four permissions, not one. Interior safety, as the design states it
(`49:257-265`), establishes exactly one of them: that no quantiser fires in the interior, which is the
condition regrouping (`reassoc`) needs to be sound. It says nothing about the other three, and two of them
are load-bearing enough to name.

**`nsz`** (treat `+0.0` and `-0.0` as interchangeable) is not a fact interior safety proves. It is a fact
about the target numeral's own `Canonical` axis (`49:143-149`, "signed zero, preferred cohort, NaN
canonicalisation"), which the design already tracks per numeral and already treats as something a law may
not read past without going through the canonical quotient (`49:229-230`). A numeral whose `Canonical`
distinguishes signed zero as observable cannot honestly accept `nsz`'s grant, and nothing in "this fold has
interior safety" says whether that numeral's `Canonical` does.

**`contract`** licenses fusing an adjacent multiply and add into one hardware `fmadd`, single rounding
instead of two. This is precisely the substitution file 16 flagged and did not resolve: "the rule licenses
this substitution by name... with nothing distinguishing a composition where the substitution is a silent
value change from one where it is not" (`16:245-247`), and the design's own droplist already states the
underlying fact plainly: "treating `f64::mul_add` as a source-expressible fidelity liberty: it lowers to
`llvm.fma`, an exact operation, not a permission" (`49:921-923`). What I had not shown in file 16, and what
`51_probes/probe_5_licence_contract_overgrants.rs` now compiles, is that `algebraic_mul` chained into
`algebraic_add` reaches the identical substitution through a route that never spells `mul_add`. On a
witness pair where fused and separately-rounded disagree (`a = b = 1.0000001192`, `c = -1.0000002384`,
found by a 2,000-candidate sweep), `mac_algebraic` (`c.algebraic_add(a.algebraic_mul(b))`) and `mac_fma`
(`a.mul_add(b, c)`) both deliver `0x28800000`; `mac_plain` (`c + a * b`) delivers `0x00000000`.
`mac_algebraic` compiles to one `fmadd`; `mac_plain` compiles to `fmul` then `fadd`, two instructions, two
roundings. So a build layer that read "this fold has interior safety" as licence to emit `algebraic_add`
and `algebraic_mul` on a multiply-then-add step would silently deliver `mac_fma`'s value, not
`mac_plain`'s, on exactly the class of witness `mac_plain`'s own semantics promise: the licence the design
can actually issue (regrouping) has smuggled in a second, unrelated licence (fusion) that needs its own
justification.

**The scope boundary is sharper still, and it is a genuine hazard rather than a subtlety.**
`51_probes/probe_6_licence_destroys_compensation.rs` compiles the textbook case: `fold_compensated`'s own
mechanism (`49:184-186`, "the one genuinely shaped fold", error feedback via a Kahan-style step) computes
`(sum + y) - sum - y`, which is algebraically zero as a real-number identity and numerically the exact bits
lost when `y` was added to `sum`. `reassoc` licenses treating those two readings as interchangeable,
because algebraically they are. Written strictly, on a witness where `y` is far below `sum`'s rounding
granularity (`sum = 1.0`, `y = 2^-30`), the step correctly recovers `-9.313226e-10`, using three real
instructions (`fadd`, `fsub`, `fsub`). Written with `algebraic_add`/`algebraic_sub` throughout, LLVM
optimises the entire expression down to one instruction, `fsub s0, s1, s1`, always zero: the compensation
term `fold_compensated` exists to compute is reassociated away to nothing. **The licence this section
argues for must never reach `fold_compensated`.** It is sound exactly for `fold`, whose accumulator is
already wide enough that regrouping changes nothing observable (`49:257-265`'s own interior-safety
statement); it is a correctness bug the moment it touches the sibling combinator the design built for the
opposite reason, to preserve a specific unreassociated rounding sequence.

*grounded on: `pin`, `host`, `flags`, all three probes; the droplist citation is `49:921-923`, quoted
directly.*

### 2.4 What the design should ask for, stated as a contract rather than as a workaround

Three clauses, following the shape file 50's own section 5.3 already used for the hardware-float-lowering
contract, because this is the same kind of thing: a permission the type system establishes that a build
layer, not arvo itself, is positioned to spend.

**First, the licence is `reassoc` alone, and the mechanism available bundles it with three others that
need separate accounting.** A build layer emitting `algebraic_add`/`algebraic_sub` on the strength of a
proven interior-safety fact is over-licensing by `nsz`, `arcp`, and `contract` unless it separately checks
each. `nsz` is discharged by reading the target numeral's own `Canonical` fact (already in the design,
`49:143-149`); where `Canonical` treats signed zero as observable, the build layer does not emit the
algebraic form, or accepts that this is the one place the available mechanism is coarser than the design's
own guarantee and states that in the receipt. `arcp` (reciprocal approximation) has no consuming operation
in the interior of an add-only fold and is inert there by the same argument section 1.4 used for a dead
`Policy` parameter; it becomes live only if a fold's interior also divides, which the design's current
surface does not do (division's float path, per `49:456-458` and file 50 section 4.4, quantises once at
the root, not inside a fold's interior). `contract` is never discharged by interior safety and needs its
own decision.

**Second, `contract`'s own decision already has an answer, and it is the one the design's exact-widening
family already gives.** A fold whose interior multiplies and adds (a MAC, a dot product) should not route
the multiply through same-format `algebraic_mul` and hope `contract` does the right thing. It should route
the multiply through `mul_full` into `mulnum(N1, N2)` (`49:269`, already the design's own mechanism for
exactly this case, "For a multiply-accumulate, N is replaced by the product numeral `mulnum(N1, N2)`",
`49:260-262`), which is exact by construction and lives in a wider numeral, not back in the operand's own
float format. There is no same-format float rounding on the multiply side of that path for `contract` to
fuse with, because nothing rounds until the accumulator itself quantises at the root. This is not a new
mechanism the design has to build; it is the existing accumulator-numeral construction, and it happens to
also be the answer to a question this section raised. Where a consumer genuinely wants the fused, single-
rounding semantics `contract` grants (which is a legitimate, cheaper, arguably more accurate operation in
its own right, matching the interior-safety spirit even more closely than separate rounding does), the
design's own existing droplist entry already names the right shape for it: state it as its own operation,
not inherit it silently.

**Third, `fold_compensated` is out of scope for this licence entirely, and the build layer's dispatch on
which combinator produced a call site is what keeps it that way.** Section 1.14's grade projection already
distinguishes `fold` from `fold_sequential` from `fold_compensated` at the type level (`49:502-514`); the
receipt this section proposes gates on that distinction directly rather than on "is this a float
reduction", which the probe in section 2.3 shows is not sufficient.

**The receipt itself.** File 50's section 5.3 already specified a build-layer receipt for a hardware-float
lowering naming the control state it assumed. This section's mechanism is a sibling, not a rival: a
build-layer receipt for an algebraic-intrinsic lowering names (a) that the call site's monomorphised type
carries the closed, constructor-headed `FoldGrowth`-shaped projection showing no interior quantisation
(the type-level fact section 1.14 already made observable), (b) the target numeral's `Canonical` fact,
checked and matching the `nsz` grant, (c) that the interior contains no adjacent same-format multiply the
`contract` grant would silently fuse, and (d) that the combinator is `fold`, never `fold_compensated`. All
four are const facts a post-monomorphisation verifier can read off the type, per the mechanism section
1.19 already keeps.

**The honest gap.** Rust's stable-track surface, as I found it on this pin, does not expose a finer grain
than the four-flag bundle; there is no `algebraic_add_reassoc_only` and no stable route to attach `reassoc`
without its companions. The proposal above works around that by discharging the companions separately
(checking `Canonical` for `nsz`, routing multiplies through `mulnum` to starve `contract`, gating on the
combinator for the compensated case) rather than by finding a mechanism that grants only what is proven. If
a finer-grained mechanism lands upstream, the design should prefer it; until then, the workaround is sound
because each companion permission is independently discharged rather than assumed, not because the bundle
itself became narrower.

## 3. What this does to the open list

**Tick 3** (`49:283-289`, `49:808-812`) closes. Compiled in section 1, both as an enumeration covering
ground file 35 and file 39 did not have, and as a structural theorem that holds for any operation the
design's dispatch discipline can express. `Policy` carries `Quantisation` alone.

**The licence half of "laws as backend licences"** (my own file 16, and file 50's "measured rather than
argued" framing at `50:449-453`) moves from reasoned to a stated design position with one compiled hazard
and one honest gap. It does not close anything else in section 3 of the consolidation; it opens a new item,
the receipt's fourth clause proposed in section 2.4, which nobody has built.

**The owed test debt** (`49:814-821`) gains one more concrete entry worth naming alongside the five already
there: a codegen regression pair, `fold` over a proven-interior-safe add reduction lowered to the vector
shape section 2.2 measured, against `fold_compensated` over the identical data confirmed to stay scalar and
unfused, so a future toolchain change cannot silently let the licence leak across the combinator boundary
section 2.3 found.

## 4. What I did not settle, and would want next

**Whether `float_algebraic` should be vetted allowed under `unstable-features.md`.** I read the tracking
issue and the stabilization PR directly and found no soundness concern, an open stabilization PR, and a
motivating case (the 8x dot-product slowdown) that is this design's own problem. That is my own reading,
one member's, and the workspace's own two-expert discipline for vetting a feature applies here exactly as
it does to any other; I record the reading and do not treat it as a ruling.

**Whether a same-format multiply ever legitimately needs `algebraic_mul` on its own, decoupled from an
adjacent add.** Section 2.4's `mulnum`-routing answer sidesteps the `contract` question by never putting a
same-format multiply next to an add in the first place. I did not check whether every consumer-facing
multiply the design has actually reaches an add immediately, or whether some stand alone (a plain
`mul_full` call with no following accumulation), in which case `algebraic_mul` alone (no `contract`
companion in play, since there is no adjacent add to fuse with) may be uncomplicated. I believe it is but
did not compile a case.

**Whether the model-width transfer argument extends to a licence built on a type-level fact.** File 50's
own section 7 already flags that the model-width transfer argument for `Ranged` numerals is a different
argument from the precision one and unproved; the receipt this section proposes reads a type-level fact
(the closed `FoldGrowth` projection) that is exactly the kind of thing that argument is about. I did not
extend the check; I inherited the open item rather than closing it.

## 5. Provenance summary

Compiled, this dispatch: sections 1.2 through 1.4 (probes 1 through 3), section 2.2's asm and LLVM IR
measurement (probe 4), section 2.3's `contract` witness and Kahan witness (probes 5 and 6). Reasoned, this
dispatch: section 2.4's proposed receipt clauses and the `mulnum`-routing answer to the `contract`
question, both design proposals rather than compiled facts, though each rests on a compiled or already-
ratified fact named at the point it is used. Quoted rather than recalled: file 16's own text (re-read for
this dispatch), the droplist entry at `49:921-923`, the tracking-issue and stabilization-PR text (read
directly from GitHub on this dispatch, not from memory of either page).
