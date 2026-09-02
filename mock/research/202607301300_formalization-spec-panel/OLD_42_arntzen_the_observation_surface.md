# 42. The observation surface

**Member:** Hans-Kristian Arntzen. I wrote file 31, eleven files back, on settling the identity
contract; parts of it have since been superseded (the encoding it assumed is gone, replaced by files
35 and 36's value-unique towers) and I carry none of its conclusions forward unexamined. The habit of
mind this dispatch wants is the one I have spent a career running against shader compilers and
synchronisation primitives: a guarantee is not what a type's constructor checks, it is what every
operation through which a value of that type can be reached checks, and the two are the same size only
by luck or by an argument someone actually made. Nobody had made that argument for this tower's
foundation. This file makes it, finds it does not close, and closes it.

**Gate:** run before this work, per the review's own standing discipline. `cargo test --workspace`
from `mock/`: 654 passed, 0 failed, 9 ignored, summed per-binary rather than trusted from a headline,
matching file 41's own reported count exactly. `grep -rln "Adjustment\|adjustment\|Bias\|Numeral"
crates/ --include="*.rs"` from the repo root returns nothing (`grep exit=1`), the same empty result
files 36 and 41 record: the surface this file builds on has no shipped source and nothing in the
shipped tree to regress. Canon gate: `40_consolidation_three.md` in full is the only required reading
per the dispatch's own instruction, and I read it before writing a line of code. Nothing below
overturns a ratified call. Section 3's correction (the value-unique encoding stays held pending the
bias repair, per op's own ninth checkpoint) is unaffected by anything here; I close the gaps file 41
opened while building that repair, I do not reopen the repair itself.

**What I read:** `40_consolidation_three.md` in full. `41_chlipala_the_rational_bias.md` in full, the
only deliverable since it, per the dispatch's own instruction that it is the sole prior file this
dispatch is answerable to beyond the consolidation. I went further by exception, where file 41 itself
pointed at a derivation I needed to check rather than cite: `36_kiselyov_the_normal_form_and_its_price.
md` section on the sealed perimeter (`36:` the `Pos`/`Nat` seal and its own probe 5/5b), read at the
source because file 41's own probe 4 cites it as settled and my job was to check whether it actually
reaches where file 41 built on it. `31:397-400` (my own file, the closure formula file 41's section 6
names as untouched), read at the source rather than through file 41's paraphrase, per this review's own
discipline against citing a citation. `ls` of the review directory once: 42 numbered deliverables
including this one, plus probe directories, `40` and `41` current since the last consolidation.

**What I compiled or measured, separated from what I reasoned.** Everything load-bearing in this file
is compiled, against the workspace pin (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, confirmed with
`rustc --version` from inside the repo), and every probe was rebuilt fresh while writing this file, not
trusted from a prior run. `42_probes/` holds eighteen files: four copies of the encoding at two states
(`vu_nat.rs`/`vu_bias.rs` unsealed, `vu_nat_sealed.rs`/`vu_bias_sealed.rs` sealed, the fix), ten
numbered probes (three attack/refutation pairs, a composition-wall isolation pair plus a compiler-crash
probe, a generic-trait construction, an adjustment-half construction, and a unified-trait construction
with its own negative control), and `price/` (a generator, a sweep script, and a committed CSV).
`42_probes/OUTCOMES.md` carries the full outcome table, every verbatim error text, the price
methodology, and the Python cross-checks run independently before a single type alias was spelled.
Everything else in this file (the perimeter argument's completeness, the reading of what the wider
observation surface implies beyond what compiles today, the API-shape recommendation) is reasoning on
those results and is marked as such in place.

## 0. The verdict, stated first

**The perimeter is open one layer below where file 41 looked, and the hole is worse than the one file
41 found.** `Bias`'s own seal (`bias_sealed::BiasSealed`) is real and file 41 verified it correctly: no
foreign type can implement `Bias` or `BiasSealed` directly. But `Bias`'s blanket impl (`impl<N: Pos +
Gcd<D, Out = H>, D: Pos> Bias for BPos<N, D>`, `41_probes/vu_bias.rs:202-205`) is an ordinary generic
impl over the LOCAL type `BPos`, satisfiable by any caller-supplied `N`, `D`, with no orphan-rule
obstacle regardless of where they come from. `Pos`, `Nat` and `Gcd` (the traits that bound `N`, `D`)
carry no seal at all in `vu_nat.rs`, the module `Adjustment`, `Bias`, and every price sweep this review
has run actually compose with. **Compiled**: a genuinely separate downstream crate fabricates a foreign
`Pos` type with a `Gcd` impl that claims unconditional coprimality (`Out = H` regardless of the actual
gcd, no Stein computation performed), and constructs `BPos<Fabricated, D4>` denoting the unreduced
value 4/4, the same value `BPos<H, H>` (1/1) does not, under a type nothing in the design ever verified
was reduced, was coprime, or was even a rational at all (`42_probes/probe_2b_widen_bias_via_
fabricated_pos.rs`). The identical attack defeats `Adjustment` directly, one crate, no `Bias` involved
(`42_probes/probe_1b_widen_adjustment_via_fabricated_pos.rs`), which means file 41's own recommended
fix (seal `Adjustment` the same way `Bias` is sealed, `41:section 2`) does not close this: the attack
never implements `Adjustment`, so sealing it changes nothing.

**The consolidation's own claim that "`Pos`, `Nat` and `Int` are sealed" (`40:446`) is false of the
composing design as it is actually built.** It is true only of a standalone, orphaned demonstration
file (`36_probes/probe_5_sealed_perimeter_lib.rs`) that nothing else in the review, including file 41's
own `Bias` construction, actually imports or composes with. The seal exists in one file nobody uses and
the tower everyone uses is an unsynced, unsealed copy: one definition, two hazards, exactly the
duplication class this workspace already names and forbids.

**The fix is minimal and I built and verified it.** Sealing `Pos`/`Nat` with the same private
supertrait `36_probes/probe_5` already demonstrated in isolation, applied to the module everything
actually composes with, closes both attacks (`42_probes/probe_3b_sealed_tower_refuses_both.rs`), with
zero change to `Adjustment`, `Bias`, `Gcd`, `ExactDivOdd`, `Strip2` or `Reduce`. Sealing `Gcd` itself is
unnecessary: once `Pos` is sealed, no foreign type can satisfy `N: Pos` at all, so it never reaches the
point of needing a `Gcd` bound to be checked, real or fabricated. **Measured**: the seal costs nothing
distinguishable from noise, 15.486 ms/composition against the unsealed tower's 15.407 (a 0.5 percent
difference well inside single-dispatch measurement noise), and roughly 0.2 percent more crate metadata
(2247.4 bytes/composition against 2242.3).

**File 41's own conclusion that "the design cannot have a generic `BiasMul` trait" is too strong, and I
built the trait file 41 said could not exist.** The actual trigger, isolated by controlled elaboration
rather than guessed at, is narrower than "`Reduce`'s machinery is unrepresentable generically": it is
specifically naming the `Reduce` TRAIT as a bound (`T: Reduce`), which forces the solver to select and
confirm `Reduce`'s one blanket impl (discharging its own where-clauses eagerly, as part of confirming
the impl applies, rather than deferring them as ordinary caller assumptions). **Compiled**: the exact
same constituent facts (`Strip2`, `Gcd`, `ExactDivOdd`, `AsPos`), spelled directly as a generic trait's
own where-clauses rather than through `Reduce`, compose cleanly and produce `BiasMulGeneric`, a real,
fully generic, `b1.mul(b2)`-shaped trait (`42_probes/probe_5_generic_biasmul.rs`), verified correct
against both of file 41's own witnesses. I extended it to all nine sign combinations, unifying with
file 41's own `BiasProduct` dispatch trait rather than sitting beside it
(`42_probes/probe_7_unified_biasproduct.rs`), which closes file 41's own section 6 open question about
whether the four-alias asymmetry is the shape the shipped crate should carry: it is not; one trait,
covering zero and both signs, both non-zero magnitudes, is. **Measured**: the generic trait costs the
same as file 41's bare-alias mechanism to compile, 15.738 ms/composition against 15.407, indistinguishable
from noise at this scope, and marginally SMALLER crate metadata (2227.6 bytes/composition against
2242.3). There is no compile-cost argument for the alias shape over the trait shape.

**One further, previously unrecorded rustc behaviour, independently corroborated twice**: raising
`#![recursion_limit]` on the failing bare-`Reduce`-bound file, per rustc's own suggested remediation,
does not produce a clean, deeper answer. It crashes the compiler, SIGBUS, inside
`rustc_trait_selection`'s `OpportunisticVarResolver`, reproduced identically on two independent
invocations (`42_probes/probe_4c_recursion_limit_crashes_rustc.rs`, `42_probes/OUTCOMES.md`). Stronger
evidence than a bare overflow diagnostic that the divergence is a genuine unbounded search, not a
shallow default-limit artifact.

**The adjustment half of the closure formula (`31:397-400`, `gcd(A1*A2, A1*B2, A2*B1)`), generalised
over rational `Adjustment` and `Bias`, which file 41 named as untouched, is built and verified.**
"Gcd of three rationals" reads as a generator of the additive subgroup the three terms jointly
generate, per file 41's own section 6 framing: reduce each product, place all three over their common
denominator (the lcm of the three reduced denominators), take the ordinary integer gcd of the placed
numerators, reduce once more. No new arithmetic primitive: `Lcm<A, B> = A * (B / gcd(A, B))`, and `B /
gcd(A, B)` is exactly the reduction chain's own denominator output, applied to the pair. **Compiled**
against two independent witnesses, both cross-checked against Python's `fractions.Fraction` before a
single type alias was spelled: A1=3/4, A2=1/2, B1=1/2, B2=1/3 gives 1/8; the harder cross-denominator
case A1=2/3, A2=3/5, B1=1/4, B2=5/6 gives 1/180 (`42_probes/probe_6_adjustment_half.rs`). One small,
clean finding beyond the mechanism itself: `Adjustment`'s own type is unsigned, so this half of the
formula never needs the sign-combination dispatch `Bias`'s own multiplication requires; only
magnitudes ever enter it, matching ordinary gcd's sign-indifference exactly.

## 1. The observation surface, enumerated

`what-you-can-observe-is-what-you-guaranteed.md`: a guarantee about a type holds only over the
operations through which a value of it can be reached, so the question is never "does the property
hold when the type is built" alone, it is "is there any way to reach a value of this type for which it
does not hold." File 41 asked this question of `Bias`'s own trait and answered it correctly for the
routes it checked. I asked it of the whole encoding this tower rests on, and enumerate it here because
a property-carrying design sealed in the places someone happened to look is not sealed.

| Member | What it exposes | Closed by | Status |
|---|---|---|---|
| `Pos` (`H \| O<P> \| I<P>`) | Any type implementing it, unconditionally reachable by any downstream crate | Nothing, in `vu_nat.rs` as shipped by file 36 and used by everything since | **OPEN.** Compiled attack: `42_probes/probe_1b`, `probe_2b`. This is the root of both other holes. |
| `Nat` (`Z \| Pz<P>`) | Same shape as `Pos`, same absence | Nothing | **OPEN**, same root cause, not independently attacked in this file because `Pos` alone already carries every load-bearing use in the probes I built; the argument in section 1.1 below covers it without a separate compile. |
| `Gcd<Rhs>` | `type Out: Pos`, any impl for any `Self: Pos` | Nothing directly, but see 1.1: closed transitively once `Pos` is sealed, because a fabricated `Self` can no longer exist to carry a fabricated `Gcd` impl anywhere it matters | **CLOSED transitively**, argued in 1.1, not independently compiled (the compile that would isolate it is a coherence question about implementing `Gcd` for an EXISTING sealed `Pos` inhabitant with a foreign `Rhs`, which cannot reach a `Pos`-bounded position either way; see 1.1). |
| `ExactDivOdd`, `Strip2`, `Reduce` | Associated types, one blanket impl each, no seal | Nothing directly; closed transitively the same way `Gcd` is, since every position they bound is itself `Pos`/`Nat`-typed | **CLOSED transitively**, same argument. |
| `Adjustment` (`Ratio<N, D>` blanket impl) | `const NUM: u64; const DEN: u64;`, any `Ratio<N, D>` with `N, D: Pos + Gcd<..., Out=H>` | Nothing (file 41's own finding, `41:section 2`) | **OPEN before the fix, CLOSED after** (`42_probes/probe_1b` before, `probe_3b`(a) after). File 41's proposed fix (seal `Adjustment` itself) would not have closed this: the attack never implements `Adjustment`. |
| `Bias` (`BZero \| BPos<N,D> \| BNeg<N,D>`) | `const NUM: i64; const DEN: u64;` (read-only, derived from `N::VAL`/`D::VAL`, no independent constructor), plus the `BPos`/`BNeg` blanket impl bounded on the same `Pos + Gcd<..., Out=H>` condition | `bias_sealed::BiasSealed` on `Bias` itself (file 41, verified correct against the two routes it checked); NOT closed against the third route | **OPEN before this file, CLOSED after** (`42_probes/probe_2b` before, `probe_3b`(b) after). This is the headline finding: `Bias`'s own seal was necessary and correctly built, and insufficient, because the hole was never in `Bias`, it was one layer below it. |
| `ReducedBiasPos<N,D>`/`ReducedBiasNeg<N,D>` (file 41's normalising aliases) | Bare type aliases, no independent surface beyond `BPos`/`BNeg`'s own | Inherits `BPos`/`BNeg`'s closure | **CLOSED after the fix**, no independent argument needed. |
| `BiasMagN`/`BiasMagD`/`BiasMulPP`/`PN`/`NN` (file 41's product aliases) | Bare type aliases over raw `Pos` quads, no coprimality requirement on the INPUT side at all (by design, per file 41's own section 3: they compute the reduced product of whatever they are given) | N/A, by design; the aliases never claim their inputs are already valid `Bias` magnitudes | **NOT A HOLE**, a different contract. Worth stating precisely because it is easy to conflate with the `Bias`-typed-input shape probe 5/7 build: `BiasMulPP<N1,D1,N2,D2>` takes four raw `Pos` types and produces a reduced result; it never asserts `N1: Gcd<D1, Out=H>` on the way in, so it is not itself attackable the way `BPos<N,D>`'s own blanket impl is, but it is also not the "operate on two genuine `Bias` values" contract a `b1.mul(b2)` surface wants (section 2 below). |
| `Numeral::Precision` (a `Nat`, per the identity contract, `40:63-68`, `40:492`) | Same `Nat` surface | Same fix, once built | **REASONED, not compiled.** No probe in this review builds `Numeral` itself yet. The identical argument applies the moment it does: any axis typed as `Pos`/`Nat` inherits this closure or this hole depending on which encoding it composes with. |
| `Numeral::Radix`, `Numeral::Domain`, `Exponent`'s own `Underflow`/`Specials` axes | Small closed-enum-shaped axes (`SignDomain = NonNegative \| Symmetric \| AsymmetricLow`, etc., `40:494-496`) | Unbuilt | **REASONED, not compiled.** These are not `Pos`/`Nat`-shaped, so the specific attack in this file does not transfer verbatim, but the general question ("can a downstream crate widen this axis's value set by fabricating an impl the design never checked") is the same question, unanswered for each until each is built and its own perimeter is checked the way this file checks `Pos`/`Nat`/`Adjustment`/`Bias`. |

### 1.1 Why sealing `Pos` alone is sufficient, and `Gcd` needs no seal of its own

Reasoned, not independently compiled beyond what `42_probes/probe_3b` already demonstrates. Every
position that could carry a fabricated value in this tower is typed `N: Pos` or `D: Pos` (`Adjustment`'s
`Ratio<N,D>`, `Bias`'s `BPos<N,D>`/`BNeg<N,D>`, and every intermediate `Strip2`/`Gcd`/`ExactDivOdd`
step, per `vu_nat.rs`). Sealing `Pos` closes the set of types that can ever satisfy that bound to
exactly `{H, O<P>, I<P>}`, recursively, by the same induction file 36 already ran for value-uniqueness
one property over.

The residual question is whether a downstream crate could still corrupt `Gcd`'s behaviour on one of
those now-closed inhabitants, by adding a conflicting or malicious `impl Gcd<SomeLocalType> for H`
where `SomeLocalType` is downstream-local (a shape Rust's coherence rules do permit in some cases, when
a type parameter position, not only `Self`, is local). I did not compile this residual case, because it
does not matter: even if such an impl is coherence-legal, it is useless to an attacker, because the
OTHER operand position in every consuming site (`Ratio<N, D>`, `BPos<N, D>`) ALSO requires `D: Pos`,
and `D` being downstream-local would itself require `impl Pos for SomeLocalType`, which the seal already
refuses. Both positions in every pair this tower ever forms are `Pos`-bounded; sealing the one axis
both positions share closes the whole surface. This is the completeness argument the enumeration above
rests on, and it is why the fix in `42_probes/vu_nat_sealed.rs` touches only `Pos` and `Nat`, nothing
downstream of them.

### 1.2 The recommendation this finding earns, stated plainly

Land the seal in `36_probes/vu_nat.rs` (or wherever it ships as source), not merely in the standalone
`probe_5` demonstration. **The standalone file's existence is itself worth flagging as a discipline
failure independent of the attack it enabled**: it is a second, unsynced copy of `Pos`/`Nat`'s own
declaration, and the workspace's own rule on duplication as a hazard (`duplication is a synchronisation
hazard between source files`) predicted exactly this outcome, that the two copies would drift, before
anyone had to demonstrate it with a fabricated type. When the tower ships as real source, there is one
`Pos`, one `Nat`, sealed, and every consumer (the demonstration included) builds against that one
definition.

## 2. The composition wall, corrected

File 41's section 3 states, and this dispatch's brief quotes verbatim, that "the design cannot have a
generic `BiasMul` trait" and that `Reduce` "composes safely only as a bare, top-level type alias,
evaluated at a concrete numeral pair." I do not think this holds, and I built the thing it says cannot
exist.

### 2.1 What actually diverges, isolated rather than guessed at

File 41's own explanation (`41:section 3`, "the wrapper position lets the solver invent a fresh P and
recurse without a base case") is a real mechanism and I do not think it is the whole story. I ran four
controls to find the actual boundary:

1. **The exact where-clause chain `Reduce`'s own impl requires, copied onto an unrelated function's
   signature, unused, no call site: compiles clean** (`42_probes/probe_4_reduce_chain_as_bare_bounds.
   rs`). The constituent facts, as ordinary deferred assumptions, do not diverge.
2. **The bare bound `Ratio<N, D>: Reduce`, unmodified from file 41's own probe 2(b): still diverges**,
   independently re-derived (`42_probes/probe_4b_bare_reduce_bound_diverges.rs`), confirming the
   trigger is naming `Reduce` itself, not the chain.
3. **Dropping the `: Pos` bound on `Reduce`'s own declared associated types does not change the
   outcome**, ruling out eager well-formedness checking of the declared bound as the mechanism.
4. **Two synthetic isolations of "wrapper position alone"** (an abstract `Wrap<P>`-style recursive
   trait, over a bare rigid parameter and over an unresolved associated-type projection) both compile
   clean, falsifying "any wrapper position diverges" as a sufficient explanation on its own.

The reading that survives all four: naming a trait with exactly one matching blanket impl AS A BOUND
forces the solver to select and confirm that one impl, discharging its own where-clauses eagerly as
part of confirming it applies, rather than deferring them as ordinary caller assumptions the way an
unused where-clause on your own function is deferred. `Reduce` has exactly one impl
(`impl<N: Pos, D: Pos> Reduce for Ratio<N, D>`), matching unconditionally, so naming `Ratio<N, D>:
Reduce` for abstract `N, D` forces this confirmation, and the confirmation recurses into
`ExactDivOdd`'s own `Pz<P>`-wrapped pattern with no reachable base case for an abstract input.

I want to be honest about where this reading is incomplete, because I found the boundary is narrower
than even my own first restatement of it. Building probe 7 (section 2.4), I tried to factor the
magnitude computation into a fresh, shared helper trait, `Magnitude<T>`, one blanket impl, and expected
naming it as a bound to diverge the identical way. **A bare, unused `Mag: Magnitude<(N1, D1, N2, D2)>`,
with nothing projected from it, does not diverge** (`42_probes/probe_7b_shared_helper_trait_also_
diverges.rs`'s own header records the corrected reading). It diverges only once the trait's associated
type is additionally projected and re-bounded (`MagN<..>: Pos`), which is exactly what any real caller
needs to name the computed output type at all. `Reduce` itself diverges as a bare, unused,
unprojected bound; a freshly declared trait of the identical shape does not necessarily, until it is
actually used. I did not chase why `Reduce`'s own particular shape (`Self = Ratio<N, D>`, parameterised
identically to the bound) differs from a fixed unit struct carrying the abstract parameters in the
trait's own generic position; that is a question about rustc's internal candidate-assembly heuristics,
out of the scope a black-box compile-time experiment can settle, and I record the discrepancy rather
than paper over it. The practical conclusion is unaffected either way: any real consumer of either
trait, needing the computed type, hits the wall, so the discipline (spell the chain directly, never
hide it behind a trait, even a fresh one nobody has used before) holds regardless of which exact route
gets you there.

### 2.2 `BiasMulGeneric`: the trait file 41 said could not exist

Compiled, `42_probes/probe_5_generic_biasmul.rs`. The exact constituent facts from `Reduce`'s own
declaration (`Strip2`, `Gcd`, `ExactDivOdd`, `AsPos`), spelled directly as a generic impl's own
where-clauses rather than through `Reduce`, mirroring file 41's own failing draft
(`41:section 3`) one construction later:

```rust
pub trait BiasMulGeneric<Rhs> {
    type Out: Bias;
}

impl<N1, D1, N2, D2, RawN, RawD, StripN, StripD, Divisor, QuoN, QuoD, FinalN, FinalD>
    BiasMulGeneric<BPos<N2, D2>> for BPos<N1, D1>
where
    N1: Pos + Gcd<D1, Out = H> + PMul<N2, Out = RawN>,
    D1: Pos + PMul<D2, Out = RawD>,
    N2: Pos + Gcd<D2, Out = H>,
    D2: Pos,
    RawN: Pos,
    RawD: Pos,
    Ratio<RawN, RawD>: Strip2<N = StripN, D = StripD>,
    StripN: Pos + Gcd<StripD, Out = Divisor>,
    StripD: Pos,
    Divisor: Pos,
    Pz<StripN>: ExactDivOdd<Divisor, Out = QuoN>,
    Pz<StripD>: ExactDivOdd<Divisor, Out = QuoD>,
    QuoN: AsPos<Out = FinalN>,
    QuoD: AsPos<Out = FinalD>,
    FinalN: Pos + Gcd<FinalD, Out = H>,
    FinalD: Pos,
{
    type Out = BPos<FinalN, FinalD>;
}
```

One repair beyond mechanical unbundling, and it is worth stating precisely because it is a small hole
in the design's own account of `Reduce`, not only a fact about my construction. `Reduce`'s own
declaration (`pub trait Reduce { type N: Pos; type D: Pos; }`) never states that the reduced pair is
COPRIME as a type-level fact; the algorithm's correctness is informal, established by the mathematics,
not machine-checked anywhere in the type. `BPos<N, D>`'s own bound to `Bias` (`N: Pos + Gcd<D, Out =
H>`) DOES demand coprimality, so a generic caller producing a `BPos` output has to supply it as an
explicit axiom (`FinalN: Pos + Gcd<FinalD, Out = H>` in the impl above), the same axiom every other
consumer of a `Reduce`d pair in this design already relies on without stating it.

Correctness, checked against file 41's own two witnesses: 1/2 * 5/2 = 5/4, and the unreduced-magnitude
case, 2/3 * 3/4's raw componentwise product 6/12 correctly renormalising to 1/2. Both are const
assertions against the computed type, not runtime checks: a wrong reduction path fails to compile here,
it does not silently produce a wrong value.

### 2.3 Price: the trait costs nothing over the alias

Measured, `42_probes/price/` (methodology and full table in `42_probes/OUTCOMES.md`). `BiasMulPP`
(file 41's alias) and `BiasMulGeneric` (mine), same 8-bit random operand distribution, same
methodology, same session: 15.407 ms/composition against 15.738, a 2 percent difference well inside
single-dispatch measurement noise, and the generic trait's metadata is marginally SMALLER
(2227.6 bytes/composition against 2242.3). There is no measured cost argument for keeping the alias
shape over the trait shape. (My absolute numbers differ from file 41's own recorded 19.10 ms/composition
for the identical `BiasMulPP` mechanism at the same width by roughly 20 percent; read as machine-load
and min-of-1 noise between independent dispatch sessions, not as a correction to file 41's figure,
and exactly why the comparison this file's recommendation rests on is the RELATIVE one, all three
kinds measured in one session, not the absolute one.)

### 2.4 What the surface should be: one unified trait, not four aliases

File 41's own section 6 leaves open "whether the sign-and-magnitude split this file adopts ... is the
shape the eventual shipped crate should carry," and states plainly it did not have the budget to answer
it. I think the evidence above answers it. `BiasProduct` (file 41's own zero-absorbing dispatch trait:
`BZero * anything`, `BPos * BZero`, `BNeg * BZero`) extends cleanly to all nine sign combinations by
adding the four non-zero-times-non-zero impls, each inlining the chain directly (a shared helper trait
does not survive, per 2.1's own correction), pairwise disjoint by construction and confirmed by rustc's
own coherence checker against file 41's three existing impls with no overlap error
(`42_probes/probe_7_unified_biasproduct.rs`). The consumer writes `<b1 as BiasProduct<b2>>::Out`, one
trait, uniform across every sign combination including zero, exactly the surface file 41's own header
wished for and its own section 3 concluded was unavailable. Checked against file 41's own witness, a
sign-mixing case neither file 41 nor my own probe 5 individually needed (`BPos * BNeg = BNeg`), and a
zero-absorption case confirming the new and old impls coexist.

I priced `BiasMulGeneric` standalone, not the unified `BiasProduct`; the four new impls are the
identical chain probe 5 already prices, so I read the cost as the same measured figure, not as an
independent claim, and say so rather than implying a fourth sweep ran.

## 3. The adjustment half, built

File 41 built the bias half of the biased-product closure formula and named the adjustment half
untouched (`41:section 6`): `adjustment = gcd(A1*A2, A1*B2, A2*B1)` (`31:397-400`), now over three
rational-valued products (Adjustment was already rational before file 41; Bias is rational as of file
41's own repair), where the formula's own gcd needs a meaning it never had over integers alone.

### 3.1 What "gcd of three rationals" means, and why this reading

File 41's own section 6 offers the reading I use: a generator of the additive subgroup the three terms
jointly generate, "the natural reading given the design's own overflow-band-membership use of the
concept elsewhere." Concretely: reduce each of the three products to its own lowest terms, place all
three over their common denominator (the lcm of the three reduced denominators), take the ordinary
integer gcd of the three placed numerators, reduce the resulting (gcd, common-denominator) pair once
more (it is not guaranteed already lowest terms). No new arithmetic primitive is needed:

```
Lcm(A, B) = A * (B / gcd(A, B))
```

and `B / gcd(A, B)` is exactly the reduction chain's own denominator output, `GenReduce<A, B>::D`,
applied to the pair rather than to a single `Bias` magnitude. Three-way lcm and three-way gcd are both
associative, so each composes from the two-argument primitive with no new mechanism
(`42_probes/probe_6_adjustment_half.rs`).

### 3.2 Correctness, checked against two independent witnesses

Both computed independently in Python's `fractions.Fraction` before a single type alias was written,
per this review's own discipline against letting the construction and the check share an author's
blind spot:

```
A1=3/4, A2=1/2, B1=1/2, B2=1/3  ->  1/8
A1=2/3, A2=3/5, B1=1/4, B2=5/6  ->  1/180   (lcm(5, 9, 20) = 180, the harder cross-denominator case)
```

Every intermediate value in the type-level construction is asserted, not only the final answer, so a
wrong step fails to compile at that step rather than silently downstream. Both witnesses pass
(`42_probes/probe_6_adjustment_half.rs`, `42_probes/OUTCOMES.md`).

### 3.3 A small, clean finding beyond the mechanism

`Adjustment`'s own type carries no sign (`const NUM: u64`, `vu_nat.rs`), so the closure formula's
OUTPUT never needs the sign-combination dispatch `Bias`'s own multiplication requires (file 41's
`BiasMulPP`/`PN`/`NN`, or the unified `BiasProduct` in section 2.4). Only the MAGNITUDES of `B1`, `B2`
ever enter this half's computation; sign is absent from the type, not merely unused by convention,
matching ordinary gcd's own sign-indifference. I did not need to build a sign dispatch for this half
because there is nothing for one to dispatch on.

## 4. What this file does not decide

**The residual `Gcd`-for-a-local-`Rhs`-on-a-sealed-`Self` coherence question (section 1.1) is argued,
not compiled.** I believe it is moot for exactly the reason stated there, but a member with reason to
distrust the argument should compile the two-crate case directly; I did not, because every route I
could find to make it matter routes back through a `D: Pos` position the seal already closes.

**`Numeral::Precision`, `Radix`, `Domain`, and the `Underflow`/`Specials` axes are not checked against
this file's attack, because none of them is built in any probe this review has produced.** Section 1's
enumeration states the reasoning that should apply once they are; it is not a substitute for running
the attack against the real construction when it exists.

**The unified `BiasProduct` in section 2.4 is not priced independently of `BiasMulGeneric`.** I read
its cost as the same measured figure because the mechanism is identical, four times over, and say so
rather than implying a fourth sweep ran.

**Whether `DatumDeterministic`, `TotalOrd`'s level annotation, the dither/`Refuse` interaction, or any
of the consolidation's other open items (`40:section 3`) interact with the sealing fix is untouched.**
Nothing in this file changes any axis those items depend on; I did not check for an interaction I have
no specific reason to suspect exists.

**The discrepancy in section 2.1 (why `Reduce` diverges as a bare, unprojected bound and a freshly
declared trait of the same shape does not) is recorded, not resolved.** It would need reading rustc's
own candidate-assembly source to settle properly, which is out of scope for a black-box compile-time
probe; the practical conclusion (spell the chain, do not hide it behind any trait) holds regardless of
which exact route triggers the wall for a given formulation.

## 5. Standing

The seal is a small, mechanical, two-file change (`Pos`/`Nat` in `vu_nat.rs`, unchanged everywhere
else) with a compiled, cross-validated fix and a measured zero compile-time cost; I recommend op ratify
it on this evidence, and separately note that the standalone `probe_5` demonstration should either be
retired or explicitly wired into the tower it was meant to protect, so a future reader does not repeat
the mistake of trusting it as evidence about a module it never touches. The generic `BiasMulGeneric`
trait, and its unification with `BiasProduct` into a single nine-combination surface, directly
contradicts file 41's own stated conclusion and I have tried to make the contradiction as checkable as
possible: every claim in section 2 has a compiled artifact behind it, the negative controls that
falsified my own first guess are committed alongside the positive result rather than quietly discarded,
and the price sweep answers the API-shape question file 41 left open with a number rather than a
preference. The adjustment half closes the one piece of the closure formula file 41 named as
untouched, using nothing beyond machinery this review has already built and priced. All four are
two-expert-agreement-shaped calls, not this file's to ratify alone, per the review's own standing
discipline; I have tried to leave each small enough, and each evidenced enough, that agreeing on it
costs little and disagreeing on it costs a specific, named compile to run.
