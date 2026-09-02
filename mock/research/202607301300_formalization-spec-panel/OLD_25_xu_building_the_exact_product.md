# 25: Building the Exact Product

**Member:** Haoran Xu (VM and low-level codegen lens: I built an interpreter that beats a JIT on real
workloads by refusing to treat the cheap tier as a toy, then built a meta-compiler, Deegen, that
derives every execution tier mechanically from one semantic definition so the tiers can never drift
apart. The habits I bring here: read the generated code before believing a claim about it; a design
that needs two hand-authored copies of one truth is a design with a bug on a delay timer; and a
proposal is a claim about what compiles, so I compile it.)

**Position:** file 25, immediately after the multiplicative-half proposal. Not a synthesis; the dive
continues.

**What I read.** The two files this dispatch named as my subject and my immediate predecessor,
`11_current_shape_draft.md` in full and `24_smith_the_multiplicative_half.md` in full, plus the four
named op files (`13c`, `16b`, `16c`, `24b`). I did not open files 13 through 23; where file 24 cites
them I repeat the citation as file 24's, not as independently verified by me. I listed the panel
directory and `24_probes/` before starting. On source: `arvo-strategy/src/lib.rs:1-19` and
`arvo-strategy/src/width.rs:33` only, to check what `Width` actually is today (a plain `struct
Width(pub u16)`, not a trait, and the crate root is gated on the forbidden `#![feature(generic_const_exprs)]`
right now), and a grep across `mock/crates/**/*.rs` for `mul_full`, `AddWidth`, `fn quantize`, all
empty. Nothing here exists yet; there is nothing to audit and nothing to defer to.

**What I compiled**, as distinct from what I reasoned about: seven files under `25_probes/`, every one
`rustc +nightly-2026-05-28 -O`, run to completion where they have a `main`, disassembled where the
question was cost rather than correctness. Two of the seven are deliberately negative: one records a
compile failure that settles a question by failing, one records a compile-time panic that proves a
check is load-bearing rather than decorative. I did not run `cargo test --workspace`; nothing I built
touches shipped source, file 24 already established the suite says nothing about this surface
(`24_smith...md:34-37`), and re-establishing that here would spend a dispatch on a fact already on the
record, which `16b` and `24b` both ask members to stop doing.

**The gates.** No ratified canon governs this question. `13c`'s standard and `16b`/`16c`/`24b`'s
correction to it are the operative posture, unchanged by anything below: optimal and representative of
the mathematics, capable of expressing MATLAB, IEEE 754 and SystemC as a test, the existing code
irrelevant except as evidence for why the redesign is happening, and every boundary where arvo
deliberately stops short owed a designed downstream contract rather than an observation that the
boundary exists.

---

## 0. The question, answered before the argument

File 24's claim splits into two parts and they have different fates.

**The relocation is right, and stronger than argued.** Multiplication is exact; narrowing is where
every rounding law failure actually lives. This is not merely defensible, it is the only reading under
which `Growth::Exact`'s own stated sentence (`11_current_shape_draft.md:163`, "widths add, quanta
multiply, nothing is dropped") is true rather than aspirational. Nothing in this file weakens it.

**The mechanism file 24 gestured at and did not build does not work, and a different one does.** The
literal proposal, `<W1 as AddWidth<W2>>::Out` computed as a const generic under `min_generic_const_args`
(`24_smith...md:568-569`), fails to compile for a structural reason, not an incidental one:
`min_generic_const_args` explicitly refuses to let a generic const parameter enter arithmetic on the
right-hand side of the very item (`type const`) it introduces to legalize const-generic projections.
Section 1 shows the failure. The mechanism that does work needs no unstable feature at all: widths
become distinct *types*, not const-generic values, and addition is ordinary trait dispatch resolved at
monomorphisation, the technique the `typenum` crate has shipped on stable Rust for a decade. Section 2
builds it, section 3 wires it into a real product, section 4 answers the loop and fold questions the
brief asked for, section 5 answers the firing-site question, section 6 is the downstream contract `16c`
requires.

---

## 1. Why the const-generic route is dead, compiled

`min_generic_const_args` (tracking #132980, the sound successor named in the workspace's own
`unstable-features.md`) introduces a new associated-item kind, `type const NAME: T;`, specifically so a
trait can expose a computed constant that is legal to *forward* into a const-generic argument position
without the arithmetic itself appearing inside the braces. The natural first attempt is to define
`AddWidth` with a `type const OUT: u16` and compute it in the impl body:

```rust
pub trait AddWidth<Rhs> { type const OUT: u16; }
impl<const A: u16, const B: u16> AddWidth<W<B>> for W<A> {
    type const OUT: u16 = A + B;
}
```

This fails at the definition site, not the call site (`25_probes/02_min_gca_type_const_block.rs`):

```
error: generic parameters may not be used in const operations
  --> 02_min_gca_type_const_block.rs:11:35
11 |     type const OUT: u16 = const { A + B };
   |                                   ^
   = help: add #![feature(generic_const_args)] to allow generic expressions as the RHS of const items
```

The compiler's own help text names exactly the boundary: the *sound, min* subset stops at forwarding
already-known consts, and any arithmetic over a still-generic const parameter needs the full,
unforbidden-only-in-name-not-in-fact `generic_const_args`, which is not on the allowed list and is not
the sound successor `unstable-features.md` blesses (that row names `min_generic_const_args`
specifically, `.claude/rules/unstable-features.md`, the Allowed table). This is not a workaround-away
error. `A + B` is exactly the shape `generic_const_exprs` exists to permit and exactly the shape its
sound successor was carved down to exclude, for the same reason arvo's own capacity migration hit this
wall first: "the capacity is a TYPE... so no `cap_size` expression sits in type position"
(`unstable-features.md`, the `generic_const_exprs` forbidden-table row, quoting `arvo-comb/src/lib.rs:16`).
Width addition is arithmetic on a const generic exactly as capacity indexing was; the fix has the same
shape, and it is not a smaller version of the const-generic mechanism. It is a different mechanism
entirely, built next.

One further, sharper confirmation before moving on: the very first naive attempt, projecting a `type
const OUT` result directly into a return type's braces with no intervening block, does not even reach
the arithmetic error. It fails on the associated-item declaration itself
(`25_probes/00_min_gca_associated_const_as_generic_arg.rs`):

```
error: use of `const` in the type system not defined as `type const`
```

which is the compiler telling you plainly that even a bare `const OUT: u16` (the ordinary, currently
stable-shaped associated const) is not accepted for this purpose under `min_generic_const_args`; the
feature demands its own new item form before it will even discuss whether the RHS is legal. Two
independent refusals, at two different points, both structural. File 24's flagged risk
(`24_smith...md:568-569`, "should be cheap... this dive's record is that unbuilt shapes have holes")
was exactly right to flag, and the hole is exactly where flagged.

---

## 2. The mechanism that works, compiled, exhaustively checked at a small model

Widths become types. `Width` is a trait with one associated const (`VALUE: u16`), and every width is a
distinct zero-sized type built from a binary digit chain, the same representation the `typenum` crate
has used on stable Rust since before const generics existed at all:

```rust
pub trait Bit { const VAL: u16; }
pub struct B0; pub struct B1;

pub trait Width { const VALUE: u16; }
pub struct UTerm;
pub struct UInt<Hi, Lo>(PhantomData<(Hi, Lo)>);
impl Width for UTerm { const VALUE: u16 = 0; }
impl<Hi: Width, Lo: Bit> Width for UInt<Hi, Lo> { const VALUE: u16 = Hi::VALUE * 2 + Lo::VAL; }
```

Addition is a ripple-carry adder built entirely from ordinary trait impls: a `FullAdd<Rhs, Cin>` truth
table on bits (eight impls, one per input combination), an `IncBy<C>` increment that ripples a carry
through a chain, and `AddC<Rhs, Cin>` recursing structurally on the two operand types the way a hardware
adder recurses on bit position. None of this needs a single unstable feature. It is dispatch on *types*,
resolved by ordinary trait matching at monomorphisation, exactly the mechanism this whole workspace
already trusts for everything else it dispatches on. The full construction, 132 lines, is
`25_probes/03_typelevel_binary_addwidth.rs`; I built it from scratch here rather than depending on
`typenum` itself so the mechanism is verified in this sketch and not merely trusted by citation.

Exhaustive check at a small model, in the same spirit as Thread C's own methodology
(`11_current_shape_draft.md:606-631`, checking a rule at a small width rather than arguing it): every
sum of two 2-bit values, 0 through 3, sixteen pairs, checked as a `const` assertion so a wrong answer is
a compile failure, not a runtime one:

```rust
const _: () = assert_eq_u16(<U2 as AddWidth<U3>>::Out::VALUE, 5);
// ... all sixteen pairs, 0+0 through 3+3
```

All sixteen compile and the binary runs clean. A second check at a realistic scale, `13 + 7 = 20` and
`3 + 2 = 5` (the exact I and F sums a `UFixed<13,3>` times `UFixed<7,2>` product needs), also compiles
and checks correct. Verified: `25_probes/03_typelevel_binary_addwidth.rs`, run output `2+3 = 5`,
`13+7 = 20`.

**The ergonomic cost, honestly.** The consumer-facing sugar (`UFixed<13, 3, Warm>`) writes I and F as
plain `usize`/`u16` literals, and that surface should not change; nobody wants to spell a binary tree at
a call site. Bridging a literal `const N: u16` to its type-level numeral, generically over an arbitrary
N, hits the identical wall section 1 just described: converting the value to a type IS the operation
that needs arithmetic in type position (peeling `N`'s bits recursively means computing `N/2` in a type
position at every recursion step). There is no way to do this generically over an open, unbounded N
without `generic_const_exprs`. The honest fix is not a clever escape. It is a bounded, macro-generated
dispatch table, one trait impl per literal width up to whatever bound arvo commits to
(`25_probes/04_literal_to_typewidth_bridge.rs`, ten widths hand-written to prove the shape, `13` and
`20` checked). This is *exactly* the table shape section 3.9 already measured and already characterised
as costly relative to the alternative ("the table's coherence cost is quadratic in row count, the
projection's is flat, and at 512 rows the table costs roughly eleven times the projection in coherence
alone", `11_current_shape_draft.md:412-414`), and it is exactly the mechanism arvo already ships for
container selection (`BitsContainerFor`'s Pattern C const-tag dispatch, named in this repo's own
`.claude/CLAUDE.md`). I am not proposing a new kind of cost. I am reporting that the multiplicative
half's ergonomic surface inherits a cost the design already priced, at a bound the design will need to
choose regardless (arvo already caps native container width at 128 before falling back to `WideBits`).
Whether to build this table as a flat impl-per-row set or as a typestate projection matching the
already-benched cheaper shape is the same open choice section 3.9 already surfaces, unchanged by
anything here, and the projection form should win for the same reason it already won there.

---

## 3. The composed exact product, and how products compose

```rust
pub struct Number<I: Width, F: Width>(pub i128, PhantomData<(I, F)>);

pub fn mul_full<I1, F1, I2, F2>(
    a: Number<I1, F1>, b: Number<I2, F2>,
) -> Number<<I1 as AddWidth<I2>>::Out, <F1 as AddWidth<F2>>::Out>
where I1: AddWidth<I2>, F1: AddWidth<F2>, I2: Width, F2: Width {
    Number::from_raw(a.0 * b.0)
}
```

One function, generic over every width pair, no per-pair duplication, matching the spec's own stated
obligation (`11_current_shape_draft.md:781-782`, "whether one generic arithmetic function body can serve
every strategy at once"). Full working file: `25_probes/05_composed_exact_product.rs`.

**Products of products, verified.** `mul_full(mul_full(a, b), c)` type-checks and widens correctly a
second time. `Q(2,2) x Q(2,2)` gives `Q(4,4)` (`ab.0 = 78`, matching `13 * 6`), and `Q(4,4) x Q(2,2)`
gives `Q(6,6)` (`abc.0 = 312`, matching `78 * 4`). The width grows by ordinary type-level addition at
every multiply, with no bound and no special-casing for the second application. This is `AddWidth`
composing with itself exactly the way ordinary addition composes, because it *is* ordinary addition,
relocated into the type system. There is no ceiling here other than the one the consumer's own chain
length imposes, and it is a compile-time ceiling (recursion depth in the trait solver, not a runtime
one), which is the correct place for it to live.

**The narrowing site, named and singular.** `quantize<SrcI, SrcF, DstI, DstF>` is the one function that
touches `Quantisation`. Nothing else in the pipeline rounds, clamps, or refuses. Section 3.3's entire
apparatus (the triple, the range pair, the fallibility projection) attaches here and nowhere else; this
file does not need to touch it to show where it lives, only to show that it lives in exactly one place,
reachable from exactly one function signature.

---

## 4. Function boundary, loop, fold: the answer is not "grow the type further"

Across a function boundary the story is already shown: `mul_full` and `quantize` are ordinary generic
functions, the width types flow through parameters and return positions the way any other generic
parameter does, and nothing about crossing a function boundary is special. That much was the easy part
of the brief.

**Through a loop, the naive extension is wrong, and I want to say so before proposing what replaces
it.** The instinct, having just shown widths grow by type-level addition, is to grow the accumulator's
type on every iteration of a loop. That cannot work for a genuinely runtime-bounded loop (`for i in
0..n`), because the accumulator's *type* would have to differ per trip count, and a trip count is a
runtime value, not something the type system can see. This is not a limitation of my mechanism
specifically; it is the general fact that a type-level computation and a runtime loop live on different
sides of the phase boundary, and no construction of `AddWidth` changes that.

File 24 already has the right shape for this, and I want to make it concrete rather than merely cite
it. Section 2 of `24_smith...md` establishes that the shape that actually occurs is the MAC, not a
chained product, and that its accumulator needs headroom `acc >= product_width + ceil(log2 n)`, with `n`
known at compile time (an arity or an unroll factor). The per-element product's numeral does *not* grow
per iteration, because the two per-iteration operand numerals are fixed types across the whole loop even
though their *values* vary at runtime. Only the accumulator needs sizing, once, and `n` is available at
compile time exactly where it needs to be: as an array length or a const generic on the function that
performs the fold.

**The mechanism I would add, and it needs no width arithmetic in type position at all.** Sizing the
accumulator is a *checked bound*, not a *computed type*. This is the same move Thread C already made for
leaf mathematical facts (`11_current_shape_draft.md:619-631`, checking a fact rather than trusting a
hand-typed classification), applied to a numeric sufficiency claim instead of an algebraic one:

```rust
pub fn mac<const N: usize, PI: Width, PF: Width, AccI: Width, AccF: Width>(
    products: [Number<PI, PF>; N],
) -> Number<AccI, AccF> {
    const { assert_accumulator_sufficient::<N>(PI::VALUE, AccI::VALUE) };
    let mut acc: i128 = 0;
    let mut i = 0;
    while i < N { acc += products[i].0; i += 1; }
    Number::from_raw(acc)
}
```

`assert_accumulator_sufficient` is `acc_width >= product_width + ceil_log2(N)`, an ordinary `const fn`,
no unstable feature, checked inside a `const {}` block so a violation is a compile failure. Verified
both directions, and the adversarial case is the one that matters: at `N = 256` (file 24's own citation,
`24_smith...md:164-165`, the 56000's eight guard bits over its 48-bit product for exactly 256 MAC steps),
an accumulator with 3 bits of headroom over the product width genuinely refuses to compile
(`25_probes/06_mac_fold_checked_accumulator.rs`, the commented-out `U9` case, run and captured before
being left as documentation rather than a live line):

```
error[E0080]: evaluation panicked: accumulator numeral too narrow for this MAC's trip count
```

and an accumulator with the required 8 bits of headroom compiles and runs correctly (`acc.0 = 256`).
Disassembling the compiled `mac::<256, ...>` (`25_probes/06_mac_fold_checked_accumulator.rs`, the
`probe_mac_256` export) shows a plain 128-bit accumulation loop, four lanes of `adds`/`adc` pairs, with
zero trace of the width check anywhere in the generated code: the assertion is entirely a compile-time
event, exactly as a `const {}` block promises and exactly as this whole design's cost model requires.

This resolves the loop and the fold together, and it resolves them the same way: neither needs the
accumulator's type to be *derived*, both need it to be *checked*. A consumer choosing too narrow an
accumulator gets a compile error naming the exact deficiency, not a silent overflow discovered in
production. This is, I think, a genuinely novel answer to a question this dive had not yet closed
(section 5.2 of the draft flags growth's interaction with delivery as "unaddressed by any of the
carrier-join machinery built so far", `11_current_shape_draft.md:760-762`, and the same unaddressed-ness
applies to how an accumulator's *sufficiency* gets established at all); it costs one small `const fn`
and one line inside the fold, and per op's standing instruction in `16c` it is written down here rather
than only used.

---

## 5. What a consumer actually writes, and whether it is tolerable

```rust
let a: UFixedExact<2, 2> = Number::from_raw(raw_a);
let b: UFixedExact<2, 2> = Number::from_raw(raw_b);
let ab = mul_full(a, b);              // type inferred: UFixedExact<4, 4>
let stored: UFixedExact<9, 9> = quantize(ab);
```

`UFixedExact<const I, const F>` is `Number<WidthOf<I>, WidthOf<F>>` per section 2's literal bridge; the
consumer never writes a `UInt<...>` chain by hand. Type inference carries the wide result through
`mul_full` without an annotation at all, the same way it already does through any other Rust generic
function; a consumer only spells the wide type explicitly if they choose to bind it to a named variable
with an annotation, in which case they pay the alias's own literal form (`UFixedExact<4, 4>`, not the
raw binary tree). This is tolerable in the same way file 24's own strong reading is tolerable
(`24_smith...md:131-138`): the type grows, the consumer mostly does not see it grow, and the one place
they must think about it, the narrowing call, is exactly the one place the design wants them thinking
about it.

**Diagnostics, where I am less certain and say so.** Section 3.9 and section 4.1 of the draft already
found that rustc prints the type arguments a consumer applied but not the associated types those
arguments project to (`11_current_shape_draft.md:468-476`), and confirmed it separately (this sketch,
`25_probes/05_composed_exact_product.rs`, the FFI-safety warning's own diagnostic: rustc wrote the
`Number<UInt<UInt<...>>>` composition's full name to a separate file rather than the console, because it
was long enough to trip the "long type written to file" threshold that section 3.9 already names). Since
`AddWidth::Out` is reached through an associated-type projection rather than a direct alias application,
I expect it inherits the same failure the draft already diagnosed for the policy/lowering axes, and I
expect the same fix (`13c`'s "nominal constructors... combined with small per-axis modifier types",
`11_current_shape_draft.md:468-476`) extends to widths without needing a new idea. I have not built that
extension and am not confident enough in the rendering specifics to claim it works; this is exactly the
shape of finding `24b` asks to be marked by confidence level rather than either asserted or omitted.

**Monomorphisation and codegen cost, measured on this mechanism specifically, not assumed from the
draft's numbers.** Disassembling a concrete `mul_full::<Q2.2, Q2.2>` instantiation
(`25_probes/05_composed_exact_product.rs`, the `probe_mul_full_2_2` export, `-O`, AArch64) gives exactly
four instructions: a 128-bit widening multiply (`umulh`/`madd`/`madd`/`mul`, the standard sequence for an
`i128 * i128` product) and nothing else. No branch, no load or store for the phantom width markers, no
symbol referencing `UInt` or `Width` anywhere in the emitted text. The type-level arithmetic is not
merely cheap at runtime; it is entirely absent from the generated code, which is the same claim section
3.9 already measured for the rest of this design's machinery ("zero additional symbols in the shipped
binary across a 400-times sweep", `11_current_shape_draft.md:417-419`) and here it is the same claim
verified against the specific new mechanism rather than inherited by assumption. I did not time the
compile itself; that is a bench question under `bench-and-sketch-discipline.md`, not a sketch question,
and this file is a sketch.

---

## 6. Where the quantiser fires, what names it, and why one definition covers both standards

Section 7 of file 24 reads the spec's open firing-site question (`11_current_shape_draft.md:686-692`)
against SystemC and MATLAB and finds them on opposite sides: SystemC quantises at assignment, MATLAB per
operation, and concludes the design has to express both. I built the "does one definition serve both, or
does it become two that drift" question directly, because that is the concrete risk in relocating a
single map to two firing sites: nothing structural should stop the same map from being written twice,
each copy slightly different, each correct on its own terms, the two silently disagreeing on some case
neither author thought to check.

```rust
pub fn systemc_style<...>(a: Number<I1,F1>, b: Number<I2,F2>) -> Number<DI,DF> {
    quantize(mul_full(a, b))   // exact until this line
}

pub fn matlab_product_mode<...>(a: Number<I1,F1>, b: Number<I2,F2>) -> Number<DI,DF> {
    quantize(mul_full(a, b))   // identical body
}
```

Both compile (`25_probes/05_composed_exact_product.rs`, `systemc_style` and `matlab_product_mode`). The
bodies are, character for character, the same call to the same function. The distinction between the
two standards is not in `quantize`'s definition at all; it is entirely in *where a caller places the
call*, how many exact operations sit between one narrowing and the next, and how many `Event`s (in
Orchard's vocabulary, cited by file 24 at `24_smith...md:285-294`) accumulate before the map fires. A
convenience wrapper named after MATLAB's `ProductMode = KeepLSB(W)` is a one-line function that calls
`quantize` immediately after `mul_full`; a convenience wrapper matching SystemC's deferred discipline is
a longer expression built entirely from `mul_full` and `add_exact` with `quantize` appearing once at the
end. Neither wrapper needs its own copy of the rounding, clamping, or refusal logic. This is, I think,
the strongest evidence available that file 24's strong reading (`mul_full` and `quantize` as the two
primitives, in-type multiply as a derived convenience, `24_smith...md:132-138`) is not merely
cleaner but *structurally* what keeps the two standards from drifting apart: under the weak reading
(keep in-type multiply primitive, add `mul_full` beside it), each standard's convenience function would
need its own understanding of how in-type multiply's implicit rounding interacts with an explicit
follow-up quantisation, reopening exactly the double-rounding question section 4 of file 24 already
worked through for the `Narrowed` growth axis. Under the strong reading there is nothing implicit to
interact with; every convenience is composition of two named, total, well-understood maps.

---

## 7. The downstream contract, designed

Per `16c`, section 9 of file 24 already designs the MAC's own lowering contract in full, and I am not
duplicating it. What follows is specific to the mechanism this file adds: the typed exact product and
the checked-accumulator fold, as distinct from the MAC discipline file 24 already covered.

**What a downstream target reads out of the types.** Every call to `mul_full` carries, in its
monomorphised signature, the exact bit width of its own return value as a fact the compiler has already
proven (not merely documented): `I1::VALUE + I2::VALUE` and `F1::VALUE + F2::VALUE`, computed once at
compile time, available to any pass reading the function's type signature or the MIR after
monomorphisation. A build layer does not need to re-derive that a given multiply's result fits in
exactly `bits(I1)+bits(I2)+bits(F1)+bits(F2)` bits; the type already says so, and section 3's
disassembly confirms LLVM already lowers it to the minimal widening-multiply sequence the target
supports without being told anything beyond the operand and result widths ordinary codegen already
carries. For correctness, section 9 of file 24's own conclusion holds here without qualification: the
whole discipline is source-expressible, and a downstream target needs nothing back to make it correct.

**What arvo needs back, for performance specifically.** LLVM's own widening-multiply lowering is already
close to optimal for the general case (the disassembly in section 5 shows the standard
`umulh`/`madd`/`madd`/`mul` sequence, which is what a hand-written widening multiply would emit too).
Where a build layer can do better than the generic lowering is exactly where `arvo-always-optimal-internals.md`'s
Kind 1 (structural, hardware-shaped, cfg-gated by default) already says it can: a target with a native
widening-multiply instruction whose result lands directly in the accumulator width a fold's `AccI`/`AccF`
already name (a DSP MAC instruction, `smlal` on AArch64, `pmaddwd` on x86), rather than LLVM's generic
sequence of a separate multiply followed by a separate add. The type carries exactly the fact such an
instruction selector needs to decide whether it applies: the accumulator's width, the product's width,
and (via `assert_accumulator_sufficient`'s own bound, itself a compile-time-checked fact rather than a
comment) the guarantee that no intermediate step in the fold can overflow the accumulator, which is
precisely the safety condition a MAC-instruction selector has to establish before it can fuse a multiply
and an add into one instruction. Nothing about this needs arvo to emit the instruction itself; it needs
arvo to make the width and sufficiency facts visible in the type, which they already are by
construction, and a build layer's job is to read them, exactly the division of labour `16b` states
(`16b_op_design_the_shape_not_the_code.md:50-53`, "arvo's job is to express the intent, first-class, in
the typestate. It is not arvo's job to provide the lowering").

**One thing worth stating plainly so it is not rediscovered.** File 23's rule-based verifier (cited by
file 24 at `24_smith...md:556-561`) has a natural extension here: a composition whose declared discipline
is "exact interior, one quantisation at the boundary" is falsified by the presence of *any* rounding
instruction sequence inside the fold's own loop body, and the absence of one is directly what section 4's
disassembly shows for the checked-accumulator MAC. A build layer that ever needed to fuse a per-element
quantise into the loop (breaking the exact-interior guarantee for a speed win it believed was free) would
be changing the Event grade of the computation, which file 23's receipt requirement already governs and
which file 24's section 9 already states applies verbatim here: better-but-different is still not
license, and it is not license for the checked-accumulator fold any more than it was for the MAC.

---

## 8. What I would flag for the next member, unresolved

**The width-diagnostics extension is proposed, not built.** Section 5's honest uncertainty stands: I
expect the nominal-constructor fix from section 4.1 of the draft extends cleanly to `AddWidth::Out`
without a new idea, and I have not verified the rendering. Whoever builds the modifier-type diagnostics
work for the policy/lowering axes should try the identical technique on a width sum and report whether
it actually shortens the error, or whether an associated-type projection specifically defeats it in a
way a direct alias application does not.

**The literal-to-typewidth bridge needs its real bound chosen, and needs to be generated, not
hand-written.** Ten entries proved the shape; arvo's real bound (whatever the largest native container
width the design settles on, currently 128 before `WideBits` per `arvo-strategy/src/lib.rs:34-38`) needs
the macro run at that scale, and needs the table-versus-projection choice made in the projection's favour
per section 3.9's own already-measured coherence cost, not re-litigated.

**`AddC`'s recursion depth at real widths is untested.** The exhaustive check in section 2 covers 2-bit
values; the realistic case covers a handful of specific literals. I did not push the binary adder to
arvo's actual maximum widths (128, or the `WideBits` range beyond it) to confirm the trait solver's
recursion limit is never approached. Binary encoding keeps the depth logarithmic rather than linear,
which is why I built this over the simpler Peano (unary) alternative, but "logarithmic" is a claim about
asymptotic shape, not a measurement, and the real bound should be checked before this is trusted at
arvo's actual widths.

**The checked-accumulator pattern generalises past addition and I did not chase how far.** Sizing a
result type by a checked bound rather than a derived type is not specific to multiply-accumulate; it is
a general answer to "this fold's result numeral needs to satisfy a property the individual elements'
numerals do not determine on their own." Whether the same shape covers, for instance, the growth-versus-
delivery interaction section 5.2 of the draft leaves unaddressed (`11_current_shape_draft.md:760-762`,
two refusal opportunities in one operation under `Narrowed` growth) is a real question I have not
chased, and it smells like the same tool.

**I did not read `arvo-num-systems` or `notko-hlist`.** Six members have now flagged the same unread
pair (five per file 24's own count at `24_smith...md:594-596`, plus this one); `notko-hlist` specifically
does not exist as shipped source yet (per `notko/docs/202607281547_design.notko-gains-the-hlist.md` and
`hilavitkutin/mock/design_rounds/202607281547_topic.adopt-the-shared-hlist.md`, both design-stage), which
means the "existing `Cardinal` machinery" file 24 gestured toward using is itself unbuilt; nothing in
this file depended on it, since the binary-numeral encoding built here needs no `hlist` at all, but
whoever eventually designs `arvo-num-systems`'s relationship to this format concept should know the
hlist crate they might reach for is not there yet either.
