# 41. The rational bias

**Member:** Adam Chlipala. I wrote file 09, in the review's first stretch, on enforcement and attack;
thirty-one files have landed since and I carry none of that file's conclusions forward unexamined. The
habit of mind this dispatch wants is the one CPDT and Fiat both spent a career arguing: an invariant
maintained by an operation is an invariant that operation can stop maintaining, and the only guarantee
that survives contact with a hostile or merely careless caller is one enforced by construction, at the
type, with the perimeter closed. `Bias` is exactly the place this review has been assuming that
argument rather than running it, and running it is what this file is for.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0 failed, 9
ignored, matching every count files 31 through 39 report, confirmed by summing the per-binary lines
rather than trusting a headline. Nothing regressed under this dispatch, because nothing in it touches
the shipped tree: `grep -rln "Adjustment\|adjustment\|Bias\|Numeral" crates/ --include="*.rs"` returns
nothing, the same empty result file 36 recorded, so the surface this file builds on has no shipped
source and no shipped tests to run. Canon gate: `26_consolidation_two.md`'s governing principles,
`30b`, `34b` and `39b`'s standing instructions, and `40_consolidation_three.md` in full, all read before
writing a line of code. Nothing below overturns a ratified call; section 3 below is a correction to a
dispatch instruction rather than to anything op decided, and I say exactly where and why.

**What I read:** `40_consolidation_three.md` in full, per the dispatch's own instruction that it is the
only required reading. I went further into the numbered files by exception, where the consolidation
compresses a derivation I needed to build against rather than cite: `36_kiselyov_the_normal_form_and_
its_price.md` in full (the value-unique `Pos`/`Nat`/`Ratio`/`Adjustment`/`Reduce` machinery this file
composes with, and the one passing sentence, `36:328-331`, whose correction pattern I am repeating one
member later), `39_knuth_does_it_still_represent_them.md` section 1.3 (the defect statement and its
proposed repair, `39:123-170`), and `31_arntzen_settling_the_identity_contract.md`'s closure formula
(`31:397-400`, cited by both, read at the source rather than through either paraphrase). `36_probes/
vu_nat.rs`, `probe_4_the_rational_normal_form.rs`, `probe_4b_the_unreduced_ratio_is_not_an_adjustment.
rs`, `probe_5_sealed_perimeter_lib.rs`, `probe_5b_downstream_cannot_widen_the_perimeter.rs`, and `probe_
6_signed_bias_is_the_same_construction.rs`, read as source and compiled again from a fresh copy rather
than trusted from their own outcome files, because the machinery I build on is exactly the machinery
whose composability I was sent to test. `ls` of the review directory once: 40 numbered deliverables
plus probe directories.

**What I compiled or measured, separated from what I reasoned.** Everything load-bearing in this file
is compiled. `41_probes/` holds thirteen files: `vu_nat.rs` (a trimmed copy of file 36's own module,
the `maxmin` ablation dropped, unneeded here), `vu_bias.rs` (the new module, `Bias`, its multiplication,
and the magnitude multiplication `PMul`/`PAdd`/`Succ` ported from probe 6 and retargeted at `vu_nat.rs`'s
sealed encoding), and nine numbered probes plus their two-crate halves, each independently rebuilt
against the workspace pin (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, confirmed with `rustc
--version` from inside the repo) while writing this file, not once at the end. `41_probes/OUTCOMES.md`
carries the outcome table. `41_probes/price/` carries the compile-cost sweep: a seeded generator, a
runner, two committed CSVs (`results.csv` at 16-bit operands, `results_8bit.csv` at 8-bit), run against
the pin, `--emit=metadata`, min-of-1 (stated honestly against file 36's min-of-3; section 5 below says
why the scope is smaller) with a least-squares slope over the swept counts. One additional check, not
swept, run once: `nm -g` on a `-C opt-level=2 --emit=link` build at 400 compositions, confirming zero
symbols, the same check file 36 ran. Everything else in this file (the perimeter argument's completeness,
the reading of what op's checkpoint asks for, the pricing recommendation) is reasoning on those results
and is marked as such in place.

## 0. The verdict, stated first

**`Bias` is built, value-unique by construction, and its perimeter is closed against the attack
`Adjustment`'s own perimeter turns out not to survive.** `BZero | BPos<N, D> | BNeg<N, D>` with `N: Pos
+ Gcd<D, Out = H>, D: Pos`, sealed with its own private supertrait on top of the concrete reduction
bound. Compiled: the MATLAB witness (`39:135-136`, slope 1, bias 1/2) is representable directly; an
unreduced pair (`BPos<P6, P12>`, six-twelfths unspelled) cannot reach a `Bias`-bounded position
(`E0271`); and a genuinely separate downstream crate cannot widen it on either of the two routes
available (`E0603` against the seal, `E0271` against the bound, both in the same committed file).

**Along the way, a real, previously unflagged hole: `Adjustment` itself, as file 36 shipped it, has no
seal.** `Pos` and `Nat` carry `sealed::PosSealed`/`NatSealed`; `Adjustment` does not, and a genuinely
separate downstream crate implements it directly on a foreign type with a fabricated, unreduced pair
(`NUM = 6, DEN = 12`, denoting the value `Reduced<P6, P12>` already names under a second type), and it
compiles clean (`41_probes/probe_4b`). File 36's own section 6 predicted this weakly, for the *sealed
encoding*; this is not that. `Adjustment` was never inside any seal to begin with, and `Bias` does not
inherit the hole only because it is bounded directly on the reduction condition (`N: Pos + Gcd<D, Out =
H>`) rather than on the abstract `Adjustment` trait, a choice I made for exactly this reason before
knowing it was load-bearing and confirmed by running the attack against both.

**The repair the consolidation calls costless is not costless in the shape it names, and the honest
shape costs more mechanism, not less.** Section 1.11's own sentence, "composed rather than invented,"
undersells what happened when I tried to compose it: a fully generic `BiasMul<N1, D1, N2, D2>` trait,
built the same way `Adjustment` composes `Gcd`, does not compile, `E0275: overflow evaluating the
requirement Pz<O<_>>: ExactDivOdd<_>`, and the minimal case that reproduces it needs no `Bias`, no
multiplication, and no sign, only `Ratio<N, D>: Reduce` as an unused where-clause bound on two fully
abstract `Pos` parameters (`41_probes/probe_2`). `Reduce` composes safely only as a bare, top-level type
alias, evaluated at a concrete numeral pair; embedded inside another trait's own associated-type
machinery, for an unconstrained-shape parameter, it refuses to type-check at all, independent of
whether anything ever calls it. `BiasMul`'s magnitude (`BiasMagN`/`BiasMagD`) and its four sign
combinations (`BiasMulPP`/`PN`/`NN`) are bare aliases for this reason, not a stylistic choice, and the
consequence reaches past `Bias`: any future generic composition of `Reduce` (a hypothetical generic
`Adjustment` multiplication, not only this one) hits the identical wall.

**Priced**, `--emit=metadata`, least-squares slope over 25 to 400 compositions: at 8-bit random
operands (a `Reduce` call over up to 16-bit products, the fair comparison point against file 36's own
16-bit `Reduce` headline of 12.07 ms/composition), the magnitude alone costs **13.61 ms/composition**,
essentially the same order as the primitive it is built from; the full sign-plus-magnitude composition
costs **19.10 ms/composition**. At 16-bit random operands (up to 32-bit products, a harder case file 36
never swept), the magnitude costs **102.60 ms/composition**, the full composition **159.42
ms/composition**. Over dyadic magnitudes, the shape every fixed-point numeral this stack ships uses in
practice, the cost drops to roughly **1.55 ms/composition** (single run, 400 compositions), the same
qualitative cheap-common-case result file 36 found for `Adjustment`'s own reduction. Zero symbols
emitted at 400 compositions. The metadata debit is real and larger than file 36's own reduction debit
(roughly 4.2 to 4.4 KB per composition against 1.9 KB), which is the honest consequence of reducing a
wider intermediate product rather than a new inefficiency in the mechanism itself.

## 1. The construction (compiled, `41_probes/probe_1`, `probe_1b`)

```rust
mod bias_sealed {
    pub trait BiasSealed {}
}

pub trait Bias: bias_sealed::BiasSealed {
    const NUM: i64;
    const DEN: u64;
}

pub struct BZero;
pub struct BPos<N, D>(PhantomData<(N, D)>);
pub struct BNeg<N, D>(PhantomData<(N, D)>);

impl bias_sealed::BiasSealed for BZero {}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> bias_sealed::BiasSealed for BPos<N, D> {}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> bias_sealed::BiasSealed for BNeg<N, D> {}

impl Bias for BZero { const NUM: i64 = 0; const DEN: u64 = 1; }
impl<N: Pos + Gcd<D, Out = H>, D: Pos> Bias for BPos<N, D> {
    const NUM: i64 = N::VAL as i64;
    const DEN: u64 = D::VAL;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> Bias for BNeg<N, D> {
    const NUM: i64 = -(N::VAL as i64);
    const DEN: u64 = D::VAL;
}

pub type ReducedBiasPos<N, D> = BPos<<Ratio<N, D> as Reduce>::N, <Ratio<N, D> as Reduce>::D>;
pub type ReducedBiasNeg<N, D> = BNeg<<Ratio<N, D> as Reduce>::N, <Ratio<N, D> as Reduce>::D>;
```

Three variants, one per sign, the third denoting zero unconditionally. Uniqueness is the same induction
file 36 already ran for `Pos`, `Nat` and `Adjustment`, one layer up: `BZero` denotes zero and nothing
else, because `BPos<N,D>` and `BNeg<N,D>` both require `N: Pos`, which excludes zero by construction (no
`Pos` constructor produces it), so neither non-zero variant can ever be assigned a magnitude that
collapses to zero. There is, structurally, no fourth constructor spelling a signed zero the way a
sign-magnitude encoding classically would, for the identical reason `O<Z>` is not a `Pos`: no impl
produces it, so the type it would inhabit does not exist. Compiled (probe 1, CLAIM D): every `BPos`/
`BNeg` instance in the file carries `N: Pos`, hence `N::VAL >= 1`, hence `NUM != 0`; there is no `N` with
value zero to name, which is the argument itself rather than merely a check of it.

The MATLAB witness (`39:135-136`), representable directly: `BPos<H, O<H>>` is `Bias::NUM = 1, DEN = 2`,
the axis's own reason for existing, one half. File 39's second witness, biases 1/2 and 5/2, both
represent (`BPos<H, O<H>>` and `BPos<I<O<H>>, O<H>>`). Normalisation at the naming site: `ReducedBiasPos
<P6, P12>` and `BPos<H, O<H>>` are the same type before anything asks whether they denote the same
value, the identical discipline `Reduced<N, D>` already gives `Adjustment`.

## 2. The perimeter, and the hole in `Adjustment` it exposed (compiled, `41_probes/probe_4`, `probe_4b`, `probe_5`, `probe_5b`)

The dispatch asked me to name the observation surface and say what each member permits, because a
guarantee holds only over the operations through which a value of the type can be observed
(`what-you-can-observe-is-what-you-guaranteed.md`). For `Bias` the surface is small and closed by
construction: three constructors, a sealed marker trait, and the associated consts `NUM`/`DEN` computed
from `N::VAL`/`D::VAL`, never exposed as a public field a caller could set independently of the
constructor that produced it. The two live questions were whether that surface stays closed across a
crate boundary and whether the magnitude bound it rests on is itself closed.

**It rests on the reduction condition, not on `Adjustment`, and this was the correct choice before I
knew why.** My first draft of the impl headers bounded `BPos`/`BNeg` on `N: Pos + Gcd<D, Out = H>, D:
Pos` directly, matching what a rational magnitude actually requires rather than reaching for the
already-named `Adjustment` trait as a shorthand. Running probe 4/4b against `Adjustment` as shipped
shows why that mattered: `Adjustment` is `pub trait Adjustment { const NUM: u64; const DEN: u64; }`,
implemented conditionally on `Ratio<N, D>` with the coprimality bound, but the trait itself carries no
seal. A genuinely separate crate,

```rust
pub struct Six;
impl vu_adjustment_unsealed::nat::Adjustment for Six {
    const NUM: u64 = 6;
    const DEN: u64 = 12;
}
```

compiles clean, both build steps, no warning about the shape. `Six` and `Reduced<P6, P12>` both satisfy
`Adjustment` and denote the same value under two types, which is precisely the class of defect file 36
built the whole `Nat`/`Pos` encoding to eliminate, one layer up, in the one place file 36's own
machinery does not reach. Had I bounded `Bias` on `Adjustment` (the more obvious, more DRY-looking
choice, and the one the consolidation's "composed rather than invented" phrasing invites), `Bias` would
have inherited this hole for free: a downstream crate could implement `Adjustment` for a fabricated
type, wrap it, and satisfy any `Bias`-bounded position with a value nothing in the design ever verified
was reduced or even a rational at all, since `Adjustment`'s bound is on the *pair*, not on `Bias`'s own
magnitude field.

**`Bias`, bounded on the concrete condition and carrying its own seal, resists the identical attack on
both routes.** Probe 5b, committed in one file so a partial fix to either route cannot be mistaken for
closing the perimeter:

```rust
// (a): cannot name the seal.
impl vu_bias_sealed::bias::bias_sealed::BiasSealed for MyBias {}
// error[E0603]: module `bias_sealed` is private

// (b): can name Bias, cannot satisfy the bound with an unreduced pair.
takes_a_bias::<BPos<P6, P12>>()
// error[E0271]: type mismatch resolving <O<I<H>> as Gcd<O<O<I<H>>>>>::Out == H
```

Route (a) is the seal doing its job: `bias_sealed` is a private module, so a downstream crate cannot
even name the supertrait it would need to implement, independent of whether it could satisfy any bound.
Route (b) is the bound surviving on its own, independent of the seal: even reaching for the exported
`BPos` constructor directly, an unreduced pair fails the same `Gcd<D, Out = H>` check probe 1b already
pins from inside the defining crate, now checked from across a crate boundary. Two independent reasons
the perimeter holds, which is stronger than either alone, and which is exactly the redundancy a
type-safety argument wants: the seal stops a foreign *type*, the bound stops a foreign *value*, and a
future refactor that accidentally drops one still has the other.

**The recommendation this finding earns**, beyond `Bias`'s own construction: `Adjustment` should carry
the same private supertrait `Pos`/`Nat` already have, closing the hole probe 4b demonstrates rather than
leaving it open for the next downstream crate to find by accident. This is a small, mechanical fix (one
private module, one blanket-conditional impl, the same shape `Bias` already uses) and I have not built
it as a change to `vu_nat.rs`'s own shipped-shape probes, because `36_probes/` is another member's
committed work and this file's job was to build `Bias`, not to patch file 36 in place. I flag it as the
concrete next step rather than leaving it as prose (`catalogue-edge-cases-as-tests.md`): probe 4b is
already the regression test that would need to start refusing once the fix lands, and its header says
so.

## 3. The composition wall, and why the design cannot have a generic `BiasMul` trait (compiled, `41_probes/probe_2`, `probe_2b`)

Section 1.11's repair sentence, quoted in full in this dispatch's own brief: "the multiplication the
closure formula needs is file 36's probe 6 signed multiplication composed with its probe 4 reduction. No
new mechanism, no new feature... unbuilt at the trait level." I built it at the trait level, the
straightforward way, before doing anything else, mirroring `Adjustment`'s own shape:

```rust
impl<N1, D1, N2, D2, RN, RD> BiasMul<BPos<N2, D2>> for BPos<N1, D1>
where
    N1: Pos + Gcd<D1, Out = H> + PMul<N2>,
    D1: Pos + PMul<D2>,
    N2: Pos + Gcd<D2, Out = H>,
    D2: Pos,
    Ratio<RawProdN<N1, N2>, RawProdD<D1, D2>>: Reduce<N = RN, D = RD>,
    RN: Pos + Gcd<RD, Out = H>,
    RD: Pos,
{
    type Out = BPos<RN, RD>;
}
```

It does not compile. `error[E0275]: overflow evaluating the requirement Pz<O<_>>: ExactDivOdd<_>`, and
the diagnostic's own trace shows a `Pz<O<O<O<O<...>>>>>` growing without bound. My first instinct, that
this was about my specific formulation (a raw associated-type projection, an unbounded output, a
free-variable equality rather than a fixed one), was wrong on every guess, and I did not stop at the
first failed guess (`run-the-experiment-not-the-argument`): probe 2 isolates the minimal case, a bare,
unused, never-called function,

```rust
fn reduce_bound_only<N: Pos, D: Pos>()
where
    Ratio<N, D>: Reduce,
{
}
```

which fails identically, with nothing about `Bias`, multiplication, or sign anywhere in the file. The
control in the same file, the identical shape with `Gcd<D, Out = H>` (the bound `Adjustment` already
ships) in place of `Reduce`, compiles clean. So the fork is not about generality, boundedness, or
whether an output is consumed; it is specifically about `Reduce`.

**The mechanism, read from the diagnostic rather than guessed at, and I say this is reasoning built on
the compiled trace rather than a further compiled result of its own.** `Reduce` has exactly one impl, a
blanket `impl<N: Pos, D: Pos> Reduce for Ratio<N, D>`, matching any `Ratio<_, _>` unconditionally at the
impl header. For a fully abstract `N, D`, the solver has exactly one candidate and commits to it, then
must discharge that impl's own where-clauses (`Strip2`, transitively `Gcd`, `ExactDivOdd`). Those are
defined over `Nat`'s `Pz<P>` wrapper by explicit, exhaustive pattern (`Pz<H>`, `Pz<O<P>>`, `Pz<I<P>>`),
and for an abstract `Pz<X>` the solver can unify `X` against the `O<P>` pattern by inventing a fresh `P`
for it, then repeat the same unification on that fresh `P`, with no base case anywhere to stop it,
which is exactly the ever-deepening trace the diagnostic prints. `Gcd` never reaches this: its own
impls pattern-match directly on `Pos`'s three constructors, with no wrapper position (no `Pz<...>`-like
indirection) for a fresh variable to unify into, so an abstract `N: Pos` genuinely has no candidate
impl of `Gcd` that matches it unconditionally, and the solver defers the whole bound to the caller
instead of trying to discharge it now. This is the same class of finding file 36 itself made about
`generic_const_args` (`36:90-100`, "the fourth refusal ... it is the cheap escape, it is closed"): a
compiler behaviour, not a design choice, worth recording once so nobody re-derives it by trial and
error a second time.

**The escape, and it is not a workaround, it is the shape the review's own machinery already uses
everywhere else.** A bare, top-level type alias referencing `Reduce` for fully generic `N, D` compiles
and stays lazy (probe 2b): nothing forces normalisation until something instantiates it with a concrete
type, the same deferral an ordinary generic function body gets. `Reduced<N, D>` (file 36's own alias for
`Adjustment`) is already this shape, unremarked on because nobody had tried to embed it inside a further
generic trait until this dispatch did. `BiasMagN`/`BiasMagD` (the reduced product, magnitude only) and
`BiasMulPP`/`PN`/`NN` (magnitude plus sign, one alias per combination) are the same shape:

```rust
pub type BiasMagN<N1, D1, N2, D2> =
    <Ratio<<N1 as PMul<N2>>::Out, <D1 as PMul<D2>>::Out> as Reduce>::N;
pub type BiasMulPP<N1, D1, N2, D2> = BPos<BiasMagN<N1, D1, N2, D2>, BiasMagD<N1, D1, N2, D2>>;
```

The one piece that IS a genuine trait, `BiasProduct`, covers only the cases that never touch `Reduce`
at all: absorbing `BZero` on either side. A consumer computing a concrete product names the alias
directly, exactly the way `Reduced<N, D>` is already named directly at every WorkUnit-declaration site
in the design rather than composed through a generic multiplication trait, which is also probe 4's own
CLAIM D/E in file 36 ("a type-level multiply ... is the one piece this probe assumes rather than
builds"). Nobody had tried to build that multiply generically before this dispatch; now that it has been
tried, the finding is that the design was already using the shape the toolchain requires, for a reason
nobody had needed to name until now.

## 4. Correctness (compiled, `41_probes/probe_3`)

Checked against file 39 probe 1's own value-level witness, biases 1/2 and 5/2, now at the type level:

```
BiasMulPP<H, O<H>, I<O<H>>, O<H>>::NUM == 5, DEN == 4     // 1/2 * 5/2 = 5/4
BiasMulPN<H, O<H>, I<O<H>>, O<H>>::NUM == -5, DEN == 4    // 1/2 * -5/2 = -5/4
BiasMulNN<H, O<H>, I<O<H>>, O<H>>::NUM == 5, DEN == 4     // -1/2 * -5/2 = 5/4
```

An unreduced-magnitude witness the same shape file 34's own probe 5b needed one layer down: 2/3 times
3/4 multiplies componentwise to 6/12, and `BiasMagN`/`BiasMagD` renormalise it to 1/2 before the sign
is ever applied. An identity-case witness, 3/4 times 4/3, both signs, chosen because it is the case
where a bug in the reduction path (an accidental factor of the gcd left in, a swapped numerator and
denominator) is easiest to miss precisely because the answer looks clean either way; it checks correctly
in both signs.

This is the bias half of the closure formula `31:397-400` states (`bias = B1 * B2`), now over the
domain the formula's own algebra always required (section 1.11's own finding: the formula mixes
adjustments and biases inside one gcd, "only defined if both live in the rationals"). The adjustment
half of the same formula (`adjustment = gcd(A1*A2, A1*B2, A2*B1)`) is untouched by this file: it was
already rational-valued before this dispatch (file 31's `Adjustment` was never the plain-integer
member), and generalising its own gcd to run over the now-rational `B1`, `B2` inputs is a further piece
of work this dispatch was not sent to do and does not claim to have done. I name it in section 6 as the
next open item rather than folding an unbuilt claim into this section's verdict.

## 5. Price (measured, `41_probes/price/`)

Full table and methodology note in `41_probes/OUTCOMES.md`; the headline numbers only here, plus the
one honest scoping decision the dispatch's own instructions asked me to state.

`BiasMagN`/`BiasMagD` reduces the RAW componentwise product of the two operand numerators (and
denominators), not the operands directly, so an `n`-bit operand produces up to a `2n`-bit `Reduce` call.
File 36's own headline reduction number, 12.07 ms/composition, is `Reduce` over 16-bit operands
*directly*. The comparable point for this file's mechanism is therefore **8-bit random operands** (up
to 16-bit products, the same width `Reduce` itself is doing the work at), not 16-bit operands (up to
32-bit products, a harder case file 36 never measured and this file is the first to). Both are swept and
both are reported, so the reader is never handed one number silently standing in for the other:

| shape | operand width | product width `Reduce` sees | ms/composition |
|---|---|---|---|
| magnitude only (`BiasMagN`/`BiasMagD`) | 8-bit | up to 16-bit | 13.61 |
| magnitude plus sign (`BiasMulPP`) | 8-bit | up to 16-bit | 19.10 |
| magnitude only | 16-bit | up to 32-bit | 102.60 |
| magnitude plus sign | 16-bit | up to 32-bit | 159.42 |
| dyadic magnitudes only | mixed | up to 32-bit | ~1.55 |

At the width comparable to file 36's own number, `Bias`'s magnitude costs essentially what `Reduce`
alone already costs (13.61 ms against 12.07 ms, a 13 percent difference that is well inside what two
independent random operand distributions and a 2x `PMul` on top of the reduction would explain), which
is the honest reading of "composed rather than invented" at the price level: the mechanism is not
adding a new order of cost, it is paying the cost the design already committed to when it adopted
value-uniqueness at `34b`. The 16-bit sweep is the harder, wider case a real MATLAB-slope-and-bias
composition (both operands independently at a realistic width) would actually hit, and it costs roughly
an order of magnitude more, consistent with file 36's own observation that this mechanism's cost scales
steeply with operand width (its own 8-to-16-bit jump for the bare gcd is a 6.4x factor; a further
doubling of the effective width here landing at roughly 8.5x from the 8-bit point is the same shape of
scaling, not a new one). The dyadic case, the shape every fixed-point numeral this stack ships today
actually uses, costs next to nothing (roughly 1.55 ms/composition), the identical qualitative result
file 36 found for `Adjustment`'s own reduction and for the same reason: the gcd terminates on its first
impl when the numerator is one.

Zero symbols emitted at 400 compositions, `nm -g` on a `--emit=link -C opt-level=2` build, the expected
answer and the same check file 36 ran, for the same reason: every type here is `PhantomData`-only, no
value ever exists at runtime. The metadata debit (roughly 4.2 to 4.4 KB per composition against file
36's own 1.9 KB) is real, larger than the prior figure, and is the honest cost of naming a wider
intermediate product's type in the compositions this operand width forces; I did not find a way to
reduce it and record it as a debit rather than argue it away, per file 36's own discipline with its own
debit.

**Scope stated honestly.** This sweep runs min-of-1 rather than file 36's min-of-3, and two swept
widths (8-bit for comparability, 16-bit for the harder realistic case) rather than the full 8/16 cross
of every shape file 36's own sweep covers. Both are single-dispatch wall-clock decisions, not claims
about what the mechanism costs beyond what is reported; a member with more budget re-running this sweep
at min-of-3 would tighten the numbers, not change their order.

## 6. What this file does not decide

**The adjustment half of the closure formula, generalised over rational biases, is not built.**
`31:397-400`'s `adjustment = gcd(A1*A2, A1*B2, A2*B1)` mixes what are now three rational-valued
quantities inside one gcd; what "gcd of three rationals" means precisely (a generator of the additive
subgroup they jointly generate, the natural reading given the design's own overflow-band-membership use
of the concept elsewhere) and whether it composes under this review's `Gcd`/`Reduce` machinery or needs
a genuinely new mechanism is untouched here. This is the one piece of the closure formula this dispatch
was not sent to build and does not claim to have priced.

**The `Adjustment`-unsealed fix is named, not shipped.** Section 2's recommendation (a private
supertrait on `Adjustment`, the same shape `Bias` already has) is a small, mechanical, and in my
reading uncontroversial change; I have not made it, because `36_probes/vu_nat.rs` is another member's
committed artifact and this dispatch's job was `Bias`, not a patch to file 36. Probe 4b is the
regression test that should start refusing once it lands.

**Whether `Bias`'s bound should also cover a would-be `FullRange`-shaped constructor** (section 3 of the
consolidation's open list, `40:685-687`, the reduction-firing-site and `FullRange`-survival questions)
is untouched; nothing in this file forecloses either answer, since both would still bottom out at the
same `N: Pos + Gcd<D, Out = H>` condition this file already builds against.

**Whether the sign-and-magnitude split this file adopts (a trait for `BZero`-absorption, bare aliases
for the four non-zero combinations) is the shape the eventual shipped `arvo-algebra-contracts` crate
should carry**, or whether a macro, a build-script code generator, or a different encoding of the
sign axis dissolves the asymmetry, is a design question this file answers only for the probe-level
mechanism, not for the crate that would eventually carry it. I built the smallest thing that both
compiles and demonstrates the composition; a member designing the shipped API surface should read
section 3's mechanism note before assuming a nicer, fully generic trait is available, because it is not,
for a reason internal to how `Reduce` is defined rather than to how this file happened to write it.

## 7. Standing

`Bias` moves from held to buildable: the encoding compiles, is value-unique by the same induction as
every other member of this tower, and its perimeter is checked, not assumed, against the identical
attack that succeeds against `Adjustment` as file 36 shipped it. That last clause is the finding I did
not expect going in and would not have found without running the attack rather than arguing that the
existing `Pos`/`Nat` seal was "probably enough": it is not, `Adjustment` was never inside it, and `Bias`
avoids the hole only because of a structural choice (bound on the condition, not on the trait) I made
before knowing it mattered. The composition wall in section 3 is a genuine, previously unrecorded
compiler-behaviour finding, not a workaround local to this file, and the design's own existing practice
(name concrete numerals at every composition site, per `26:668-674`'s own atom-ladder framing and probe
4's CLAIM D/E) turns out to already be the only shape the toolchain accepts for this specific
mechanism, which I read as the design having gotten this right for a reason nobody had needed to state
until this dispatch tried to violate it. The price is comparable to `Adjustment`'s own reduction cost at
a comparable width, an order of magnitude higher at the wider, harder-but-realistic width, and next to
free over the dyadic case that is every numeral this stack ships today. I recommend op ratify the
encoding on this evidence and separately direct the `Adjustment`-seal fix (section 2) as a small,
mechanical follow-up; both are two-expert-agreement-shaped calls, not this file's to make alone, per the
review's own standing discipline, and I have tried to leave the second one small enough that agreeing on
it costs little.
