# 36. The normal form and its price

**Member:** Oleg Kiselyov. I wrote file 02, on type-level encoding, near the beginning; the design has
moved a long way since and I carry none of that file's conclusions forward unexamined. The habit of mind
this dispatch wants is the one I have: when an invariant is being maintained by an operation, ask whether
a different representation would make it hold by construction instead, because the operation that
maintains an invariant is the operation that can stop maintaining it, and nothing will say so.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0 failed, 9
ignored, matching the counts files 31 through 35 each report. I re-ran the negative greps rather than
trusting the citation chain, and one of them matters directly here: `grep -rln
"Adjustment\|adjustment\|Numeral\|FullRange" crates/ --include="*.rs"` returns **nothing**. The surface
this file is about has no shipped source and therefore no shipped tests, which is the honest statement of
what there was to audit: there is no perimeter open in the tree today, only one about to be specified.
`crates/arvo/tests/identity_laws.rs:1-21` is the full-matrix shape its own module doc describes and its
nine compile-fail pairs under `crates/arvo/tests/ui/` pin refusals a runtime assertion cannot state; I
found nothing tautological in it. Canon gate: `26_consolidation_two.md`, `30b_op_checkpoint_seven.md` and
`34b_op_checkpoint_eight.md` govern, all read in full. Nothing below overturns a D-numbered call or either
checkpoint. Where I contradict a panel file I say so in place, and the one place I do is a claim file 34
made in passing about work it did not do.

**What I read:** `26_consolidation_two.md` in full. `30b` and `34b` in full.
`34_giesen_the_three_halves_assembled.md` and `35_dolan_does_widening_collapse.md` in full.
`31_arntzen_settling_the_identity_contract.md` section 4 and section 1.5 (the biased-multiplication
formula my work extends). `34_probes/probe_5*.rs` and `34_probes/OUTCOMES.md`, the compiled failure that
produced my obligation. `25_probes/00` through `05`, the width machinery and the const-generic wall, read
as source and recompiled rather than cited. `typenum-1.20.1` source directly, `src/uint.rs:1467-1528` and
`src/private.rs:35-50,304-310`. `ls` of the directory once, 35 numbered files plus probe directories.

**What I compiled or measured, separated from what I reasoned.** Nine artifacts in `36_probes/`, each with
a row in `36_probes/OUTCOMES.md`, all against the workspace pin (`rustc 1.98.0-nightly (57d06900f
2026-05-27)`, confirmed with `rustc --version` from inside the repo). Four of the nine are committed
refusing, on purpose. A compile-cost sweep in `36_probes/price/`, with its generator, its runner and its
`results.csv`. Sections 1, 3 and 4 are compiled or measured except where marked. Sections 2, 5 and 6 are
reasoning built on those results and are marked as such.

One methodological note that cost me a false start and is worth one line: run from outside the repo, the
same `rustc` invocations resolve to stable 1.94, where `type const` does not even parse. Two of my early
"the feature is refused" readings were that, not the feature. Both were re-run inside the repo and the
numbers below are all from the pin.

## 0. The verdict, stated first

The obligation is a **perimeter** obligation, not a formula obligation, and it is open in two places
rather than the one file 34 named. File 34 raised it for the rational adjustment and recorded that the
integer half and the width chain already satisfy it (`34:328-331`). They do not. `UInt<UTerm, B0>`
inhabits `Width` with value zero, so does `UInt<UInt<UTerm, B0>, B0>`, and the width adder propagates the
spelling rather than normalising it: compiled, in probe 1, with the type-level consequence committed
refusing in probe 1b, which is file 34's own E0308 one layer down on the half it thought was safe. The
claim was true of the values the current operations happen to produce and false of the encoding, and the
obligation is about the encoding.

That reframing pays for itself, because the fix for the natural half is not a normalisation pass. It is a
different encoding, under which the illegal spelling has no type: `Pos ::= H | O<P: Pos> | I<P: Pos>`,
Coq's `positive`, where the terminator is the leading one and there is nowhere to put a zero in front of
it. Nothing normalises because nothing can be non-normal. The same construction covers the signed bias
(probe 6), and the same construction is what the rational half then rests on, because coprimality of a
pair says nothing if each component has many spellings.

For the rational half the normal form is a reduced pair and reduction genuinely needs computing. So the
answer to op's own framing of the question, enforced by construction against satisfied by a formula, is
**both, at different layers, and neither one alone is enough**: the naturals and the bias are enforced by
construction, the coprimality is enforced by a conditional impl (`N: Gcd<D, Out = H>`) so an unreduced
ratio is a well-formed type that is not an `Adjustment`, and a seal is needed on top of both or a
downstream crate reinstates the defect in one line (probes 5 and 5b).

The gcd is built (probe 3), the exact division reduction needs is built (probe 4), and both are priced.
The headline number: at 16-bit operands, 400 distinct compositions, `--emit=metadata`, min of three runs,
baseline subtracted, this gcd costs **5.08 ms per composition** against typenum's **15.55 ms**, and the
**full reduction, gcd and division together, costs 12.07 ms, less than the prior art's gcd alone.** An
ablation decomposes the 3.06x into 1.65x from the algorithm's formulation and 1.87x from the encoding and
the rest of typenum's implementation. And the number that decides how this reads in practice: over dyadic
adjustments, which is every composition arvo ships today, the full reduction costs **0.50 ms** per
composition, because the gcd terminates on its first impl. Zero symbols emitted, at any size.

Against that, one cost I did not find a way around and state as an honest debit: 1.3 KB of crate metadata
per gcd composition, 1.9 KB per reduction, which is type names and is real for downstream crates.

## 1. The obligation is a perimeter obligation (compiled)

### 1.1 Why it has to be a type at all, re-verified rather than cited

The consolidation's droplist records that width arithmetic cannot be computed in type position from
generic const parameters (`26:719-724`). I re-ran `25_probes/00`, `01` and `02` myself rather than
reasoning from the citation, and all three are refused exactly as recorded. Then I tried the fourth shape,
which is the one that would have made this whole file unnecessary, and which nobody had tried: have the
trait level compute the answer and project it **back down** into an ordinary const parameter, so a
numeral's identity is a plain `u64` and value-uniqueness comes free from structural equality on a value.

```
type const VALUE: u64 = const { Hi::VALUE * 2 + Lo::VAL };
error: generic parameters may not be used in const operations
   = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

Refused, and the feature named is `generic_const_args`, which is neither `min_generic_const_args`
(permitted) nor `generic_const_exprs` (forbidden) and is unvetted under `unstable-features.md`. So a
numeral's identity is a type, normalisation is a type-level computation, and everything below follows from
that. Recording the fourth refusal is the point of the paragraph: it is the cheap escape, it is closed,
and the next member who thinks of it can read the diagnostic instead of running it.

### 1.2 The width chain admits a second zero, and a second thirteen (probe 1, probe 1b)

`25_probes/03` and `05`'s encoding is `UTerm` and `UInt<Hi, Lo>` with `impl<Hi: Width, Lo: Bit> Width for
UInt<Hi, Lo>`. The observation surface is `Width`, because that is the bound every generic width position
carries. So the perimeter question, as `what-you-can-observe-is-what-you-guaranteed.md` puts it, is
whether any type other than `UTerm` inhabits `Width` with `VALUE == 0`. Compiled:

- `UInt<UTerm, B0>` does. So does `UInt<UInt<UTerm, B0>, B0>`. Zero has countably many spellings and so
  does every other width; probe 1 asserts three spellings of thirteen.
- The adder propagates the spelling. `<UTerm as AddWidth<UTerm>>::Output` is `UTerm` and `<UInt<UTerm, B0>
  as AddWidth<UTerm>>::Output` is `UInt<UTerm, B0>`, both value zero, two types.
- Probe 1b is the consequence, committed refusing: `expected PhantomData<UTerm>, found PhantomData<UInt<UTerm, B0>>`.

Two things make this more than a curiosity about a hand-written type.

**The literal bridge is a generator.** `25_probes/04` is a macro-generated table from `const N: u16` to a
width type, and the macro takes the spelling as an argument. Its committed rows are canonical, and nothing
checks that; a generator emitting fixed-length rows, which is the obvious way to write one at the "real
bound" the probe says is "a one-line change", pads every width. Probe 1 includes such a table and it
type-checks, values and all.

**Subtraction has not arrived yet.** The chain has only addition, which cannot produce a leading zero from
canonical inputs. That is the only reason it has not needed a repair pass. Any width difference introduces
one, and the design has three coming: a narrowing `quantize::<Src, Dst>`, an accumulator's guard-bit
headroom, and the `ceil(log2 n)` fold bound. The prior art shows exactly what that costs, because it hit
the same wall in the same encoding: typenum's `Sub` is `PrivateSub` followed by `Trim`
(`typenum-1.20.1/src/uint.rs:558-564`), and `Trim` is `Invert -> TrimTrailingZeros -> Invert`
(`src/private.rs:35-36, 304-310`), three further traversals of the digit chain whose only job is to delete
leading zeros the encoding permitted. `And` and `Xor` pay it too (`private.rs:79, 87`).

### 1.3 What this does to file 34's "already met" reading

File 34's sentence is: "The shipped width chain already satisfies it (typenum-style binary, no leading
zeros). Integer adjustments under file 31's biased formula satisfy it for free, because gcd's output is
canonical; the formula is self-normalising, a property nobody had named" (`34:328-331`).

The first clause is false as stated, per 1.2. The second is true at the value level and inherits the
first's hole at the type level: gcd's output being a canonical *number* says nothing about its being a
canonical *type* unless the encoding has one type per number, which is precisely what is in question. So
"self-normalising" is a real property and it is a property of the formula's arithmetic, not of the design.
Both halves of the obligation are open, not one, and the natural half is the one everything else rests on.

This is not a small correction to a passing sentence. File 34 named the rational adjustment as the one
place needing work and the width chain as the reassurance that the shape is known to be reachable. Reverse
that and the width chain is the larger of the two jobs, because it is under everything: `LogicalWidth`,
`Precision` (per D69, `30b:9-16`), `StoredWidth`, the accumulator numeral, the product numeral's own
widths, and the numerator and denominator of any rational adjustment.

## 2. The normal form, stated (reasoned on the compiled results; the section a consolidation takes)

### 2.1 The naturals, and they need no normal form

```rust
pub trait Pos: sealed::PosSealed { const VAL: u64; }   // sealed
pub struct H;                 // 1
pub struct O<P>(PhantomData<P>);   // 2p,   impl only for P: Pos, so >= 2
pub struct I<P>(PhantomData<P>);   // 2p+1, impl only for P: Pos, so >= 3

pub trait Nat: sealed::NatSealed { const VAL: u64; }   // sealed
pub struct Z;                 // 0
pub struct Pz<P>(PhantomData<P>);  // impl only for P: Pos, so >= 1
```

This is Coq's `positive` and `N` (Barras et al.), standard in constructive mathematics and not what
`typenum` or the current width chain uses. The leading digit is the terminator and it is a one, so there
is nowhere to put a zero in front of it.

**Uniqueness, by induction on the value.** 1 is `H` and nothing else, since `O<P>` needs `P: Pos` hence
denotes at least 2, and `I<P>` at least 3. An even `n >= 2` is `O<P>` with `val(P) = n/2 >= 1`, unique by
induction. An odd `n >= 3` is `I<P>`, likewise. Zero is `Z` and nothing else, since `Pz<P>` needs `P: Pos`.
So `Pos` is in bijection with the positive integers and `Nat` with the naturals, and **there is no
normalisation operator anywhere in the design, because there is nothing it could do.**

The induction has a hypothesis, and it is the part a formula cannot supply: that those are the only impls.
`Pos` and `Nat` therefore carry a private supertrait. Probe 5 is the sealed crate, probe 5b is a genuinely
separate crate doing `impl Pos for MySix { const VAL = 6; }` and being refused. Without the seal a
downstream crate reinstates 1.2's defect in one line, one crate away, invisible to arvo. A self-normalising
formula guarantees the numbers arvo *computes* are canonical; only a closed perimeter guarantees the
numbers arvo is *handed* are.

Compiled (probe 2): addition with carry is eighteen impls, no output is ever zero so no case needs a smart
constructor, and the sums the multiplicative half needs (13+7=20, 3+2=5, and the zero cases, `26:252-256`)
are both asserted for value and accepted by a type-equality demand. Associativity and commutativity of the
width adder hold **as type identities**, which is the form file 34 could state for the rational adjustment
and nobody had stated for the widths themselves.

### 2.2 The rational adjustment, which does need one

An adjustment is `Ratio<N, D>` with `N`, `D` inhabiting `Pos` and `gcd(N, D) = 1`. Both conditions are
load-bearing and they are enforced differently, which is the answer to op's own question about enforcement
against satisfaction.

The first is enforced by construction, per 2.1. The second cannot be: coprimality is a relation between two
independently chosen components and no constructor discipline makes 6 and 12 unspellable as a pair. So it
is enforced **where it is observed**:

```rust
impl<N: Pos + Gcd<D, Out = H>, D: Pos> Adjustment for Ratio<N, D> { ... }
```

`Ratio<Six, Twelve>` is a well-formed type and is not an `Adjustment`, so it cannot reach any position the
design bounds. Probe 4b is that refusal, and it is E0271 rather than E0277, which is the better
diagnostic: `type mismatch resolving <O<I<H>> as Gcd<O<O<I<H>>>>>::Out == H` names the gcd it computed and
what it wanted.

The consumer-facing spelling is a normalising alias:

```rust
pub type Reduced<N, D> = Ratio<<Ratio<N, D> as Reduce>::N, <Ratio<N, D> as Reduce>::D>;
```

so `Reduced<6, 12>` and `Reduced<1, 2>` are one type before anything asks whether they unify. Compiled
(probe 4): reduction is correct over 16 assertions including file 34's exact 6/12 witness, the UNORM-shaped
15/255 -> 1/17, and 12/8 -> 3/2; it is **idempotent as a type identity**, which is the normal-form property
itself; and file 34's refusing probe 5b now compiles, `Reduced<P6, P12>` and the directly written
`Ratio<P1, P2>` accepted as one type.

### 2.3 The bias, which is the third member and the same construction

`Implicit<const E: Exponent, A: Adjustment, B: Bias>` (`31:335`) carries a bias, and `bias = B1 * B2`
(`31:399-400`) is a signed multiplication. So `Bias` is a signed integer and needs the same treatment:
Coq's `Z ::= Z0 | Zpos p | Zneg p`, value-unique by the same induction, because `p: Pos` excludes zero and
therefore excludes negative zero.

Compiled (probe 6): positive multiplication by shift-and-add, with doubling structural, asserted at seven
values including the product-numeral case `13 * 7 = 91`; signed multiplication at all four sign
combinations and both zero sides; and the two spellings a sign-magnitude encoding would give for zero
collapse to one type.

Worth stating explicitly because it looks like a contradiction and is not: signed zero is real and wanted
in this design, on the **datum** side, inside `Encoding::Canonical` (`31:370-374`). A numeral parameter is
a value-level object and must not carry two zeros. That is the value/datum split of file 31 section 4.2 and
file 34 section 2.4 doing its job at two layers rather than a tension between them, and the `Bias` axis
falling on the value side is one more instance of D69's own direction of travel: mathematical coordinates
in the numeral, encoding coordinates under `Lowering`.

### 2.4 The reading I discarded, and why

There is a cheaper position available and I want it on the record as rejected rather than unnoticed, because
it is the one a cost-conscious reader will reach for.

**Referential uniqueness instead of value uniqueness.** Never let a consumer *name* a derived numeral by
literal; make every signature name it by projection (`<N1 as MulNum<N2>>::Out`). Then the spelling mismatch
never becomes a type error, because nobody ever writes the other spelling, and no reduction is needed at
all. It is strictly cheaper and it would have saved this file's entire section 3.

It is wrong for two reasons. It fails the moment a consumer stores a product in a numeral it declared,
which is the ordinary case (`let x: Q<1,2> = a * b`) and is exactly where file 34 says a generic consumer
of `mul_full`'s result stands (`34:322-324`). And it is an invariant living in a convention about how
signatures are written, which is the class of invariant this review has repeatedly found rots without
announcing itself. Op adopted value-uniqueness (`34b:25-33`) and it is the right call; this paragraph is
only the argument for why the cheaper reading does not survive contact.

## 3. The machinery, built (compiled)

### 3.1 The gcd is Stein's, and so is the prior art's

I read `typenum-1.20.1/src/uint.rs:1467-1528` rather than assuming Euclid, and the assumption would have
been wrong: typenum's `Gcd` is already Stein's binary algorithm, not the Euclid-via-`Rem` shape the name
"prior art" invites you to picture. So there is no algorithmic novelty available here and I claim none.
The difference is entirely what the encoding makes free.

On the value-unique encoding, three of Stein's five steps are pure impl selection: halving an even number
is `O<P> -> P`, doubling is `P -> O<P>`, and the parity test is a match on the outer constructor. And the
odd/odd step loses its halving outright, because for odd `x = 2a+1` and `y = 2b+1` the quantity Stein
needs, `(x-y)/2`, is exactly `a - b`, a subtraction of the two operands' own tails with no shift after it.
typenum's odd/odd impl instead names `Max` and `Min` four times between its where-clause and its output
type, each doing its own `Cmp`, subtracts at full width, and falls through to the even/odd case to do the
halving.

Compiled (probe 3): 28 binary instantiations plus 4 three- and four-argument folds, each asserted against a
hand-computed value, including the classical Euclid pair (1071, 462) -> 21, the coprime cases a reduction
actually asks about, and file 34's own biased-MAC numbers verbatim from its probe (`A = 4, B = 2` squared,
`gcd(16, 8, 8) = 8` and `gcd(16, 8, 8, 4) = 4`). Commutativity and associativity hold **as type
identities**, which is what makes the three- and four-argument folds in file 31's and file 34's own
formulas well formed as type-level statements rather than only as arithmetic.

### 3.2 Reduction needs a division, and it is the LSB-first one

This is the one genuinely new operation the obligation costs, and it is worth naming precisely because
"reduction needs division" sounds like it drags in the whole long-division tower.

It does not. Reduction needs **exact division by an odd divisor**, which is a strictly easier problem, and
the design reaches it in that form for free: strip the common power of two from the pair first, which is
structural on this encoding, and whatever gcd remains is odd. Exact division by an odd divisor has a
classical least-significant-digit-first algorithm (Jebelean's exact division, the 2-adic or Hensel form,
used in multiprecision libraries for exactly this reason), and this encoding reads least-significant-digit
first, because that is what the outer constructor is. Each step is one parity match, one subtraction, and
one structural halving. There is no comparison anywhere and no trial digit to retract, because exactness is
the precondition rather than a discovery.

The contrast with the prior art is the direction, not the algorithm: typenum's `Div` is MSB-first long
division with a comparison per digit and an `Invert` to walk the chain from the wrong end. Exact division
is the only division a reduction needs and it is the cheaper one, and this is the one place in the file
where the encoding's LSB-outermost shape is not merely neutral but actively the right one.

Compiled (probe 4): 8 division cases asserted, including `255/15 = 17` and `255/5 = 51`. Probe 4b pins the
precondition: with an even divisor the trait does not resolve at all rather than returning a wrong
quotient, so a refactor that dropped the strip step is a compile error rather than a silent wrong answer.

## 4. The price (measured)

Sweep in `36_probes/price/`: a seeded generator, a runner, `results.csv`. Build shape is
`rustc --edition 2021 --crate-type lib --emit=metadata`, which is trait solving with no codegen, and that
is the honest shape here because type-level arithmetic is entirely a trait-solving cost. Counts 0, 25, 50,
100, 200, 400 at 8-bit and 16-bit operand magnitudes, min of three runs, `count = 0` subtracted as fixed
cost. Every instantiation is forced by a const assertion against a Python-computed answer, so nothing is
elided and the run that times it also checks it. All 400 pairs are distinct, so no rustc trait-selection
cache hits; these are worst-case numbers, not typical ones.

Scaling is linear in the composition count for all three swept shapes across the whole range, so the
400-point figures are the slope rather than a knee.

| shape | 8-bit | 16-bit |
|---|---|---|
| this file's gcd, Stein on the value-unique encoding | 0.79 ms/comp | 5.08 ms/comp |
| same encoding, typenum's odd/odd formulation (ablation) | 1.26 | 8.25 |
| typenum's `Gcf`, the prior art | 2.69 | 15.55 |
| the full reduction, gcd and exact division | 2.19 | 12.07 |
| the full reduction over dyadic adjustments | not run | 0.50 |

**Which of the two existing measurements this matches.** The dispatch names two: about five milliseconds
per composition on the policy side, and flat at 557 symbols across four hundred compositions on the
identity side. This work matches the **first** on time and the **second** trivially on symbols. The gcd at
realistic width is 5.08 ms per composition, which is the policy side's figure to two significant figures,
and the full reduction is a little over twice that. Symbols are zero, measured with `nm -g` at
`-C opt-level=2` over 400 instantiations of either shape, which is the expected answer rather than a
surprising one since the whole tower is phantom types and associated types with no values.

**The ablation, because a 3x deserves decomposing rather than attributing.** I implemented typenum's
odd/odd formulation verbatim in shape on my own encoding, same four `Max`/`Min` bounds, same full-width
difference, same fall-through. At 16 bits the 3.06x total splits into **1.65x from the formulation** and
**1.87x from the encoding and everything else about typenum's implementation**. I do not claim the second
factor is purely the absence of `Trim`: it also contains typenum's `Unsigned` and `NonZero` bounds, its
`Gcf` alias, and the crate boundary. What the ablation does establish is that neither half accounts for
the win alone, and that the larger half is the one the perimeter question is about. Closing the perimeter
is not a correctness tax paid against speed; it is the larger half of the speedup.

**The number that decides how this reads in practice.** Every composition arvo ships today has a dyadic
adjustment, `2^-F`, so the numerator is one and the gcd terminates on its first impl (`Gcd<B> for H`).
Measured over power-of-two pairs: 0.50 ms per composition, twenty-four times cheaper than the general
16-bit case. So the 12.07 ms figure is the price of MATLAB's slope-and-bias and of UNORM, paid only by
compositions that use them, and the compositions that motivated `13c`'s IEEE-as-test requirement pay half
a millisecond. At 400 compositions that is 200 ms of compile time for the whole obligation, against a
budget the workspace's own rule already frames as the one to spend freely
(`arvo-compile-time-last.md`: compile time is the bucket we pour into).

**The debit.** Metadata grows: an empty crate's rlib is 53 KB, 400 gcd instantiations take it to 575 KB and
400 reductions to 827 KB, roughly 1.3 KB and 1.9 KB per composition of type names. That is real for
downstream crates and I did not measure its downstream effect, only its size. It is the one cost in this
file I did not find a way to remove, and a reader looking for the weak point should look there.

**Diagnostic length**, one measured pair, since the consolidation treats rendered diagnostic length as a
real cost (`26:33-35`): 715 bytes for the value-unique encoding against 759 for the current one on the same
deliberate width mismatch, and more usefully, the new one names both numerals in full where the old one
reports the first node at which the chains diverge with two levels elided as `_`. One pair is one pair and
I would not read the 6 percent as a trend; the qualitative difference is the part worth having.

## 5. The settled shape, stated for the next consolidation (reasoned)

File 35 gave the trait shapes after removing two axes. Nothing here disturbs them; what follows says what
the surviving members are made of, which is the question this file was sent to answer and which those
shapes leave open.

```rust
// Every numeral member that denotes a number is drawn from a value-unique,
// sealed, type-level encoding. There is no normalisation operator for any of
// them, because non-canonical spellings have no type.
//
//   Nat ::= Z | Pz<P>            P: Pos       widths, precision, exponents
//   Pos ::= H | O<P> | I<P>      P: Pos       magnitudes
//   Int ::= Z0 | Zpos<P> | Zneg<P>            biases
//
// Pos, Nat and Int are sealed. Without the seal a downstream crate restores
// the many-spellings defect in one line and nothing in arvo can see it.

pub const trait Numeral {
    type Precision:  Precision;    // a Nat
    type Exponent:   ExponentForm; // nests Adjustment and Bias
    type Domain:     SignDomain;
}

// Inside ExponentForm's Implicit arm:
//   Adjustment = Ratio<N: Pos, D: Pos>, and `impl Adjustment for Ratio<N, D>`
//   carries `N: Gcd<D, Out = H>`, so an unreduced pair is a well-formed type
//   that cannot reach an adjustment position. The consumer-facing spelling is
//   the normalising alias `Reduced<N, D>`, so two spellings of one quantum are
//   one type before anything asks whether they unify.
//
//   Bias = Int. One zero, by construction. Signed zero is a datum-level fact
//   and lives in Encoding::Canonical, not here.

pub const trait Lowering {
    type Encoding:    Encoding;
    type StoredWidth: StoredWidth;   // a Nat, same encoding
    type Layout:      StorageLayout;
    // Widening removed, per file 35 section 1.
}

pub const trait Policy {
    type Quantisation: Quantisation;
    // Growth removed, per file 35 section 2.
}
```

Three consequences worth carrying, and one of them is a saving rather than a cost.

**The width chain is replaced, not extended.** `UTerm`/`UInt` goes; `Z`/`Pz`/`H`/`O`/`I` replaces it. That
is a rewrite of `25_probes/03`, `04` and `05`'s machinery, which is unshipped design-round material rather
than source, so the rewrite cost against the tree is zero and against the panel's own artifacts is one
encoding swap in three probe files. Op's tiebreaker (`26:566-571`, keep the current shape where it costs
nothing) does not fire, because the current shape is not shipped and the replacement is measurably cheaper
in the operations both encodings support.

**`Adjustment`, `LogicalWidth`, `Precision`, `StoredWidth` and `Bias` become one kind of object.** They are
all type-level numbers in one sealed encoding, differing only in which of `Nat`, `Pos` and `Int` they draw
from. That collapses what would otherwise be five separately-normalised parameter families into one, and
it is the same compression file 35 found twice: a fact that looked like it needed its own vocabulary
turning out to be expressible by machinery built for another reason.

**The downstream contract gains a concrete guarantee, per op's standing obligation (`26:557-565`).** File
20's symbol-table reader recovers a composition's axis instances from the mangled generic arguments
(`26:361-368`). Under the value-unique encoding two symbols denoting the same numeral carry the same
mangled name, so a reader deduplicating compositions by name counts them correctly. Under the current
encoding they need not, and a reader would over-count a composition set by however many spellings its
producers happened to use, silently, with no way to tell from the artifact. That is a property arvo owes
the build layer and can state, without arvo growing any build machinery: the guarantee is a consequence of
the encoding, and what the downstream target does with it is read the name.

## 6. What this file does not decide

The **firing site of reduction** is a design choice I have not made. `Reduced<N, D>` normalises at every
naming site, which is the shape probe 4 compiles and the one I would build first because it needs no
discipline anywhere; the alternative, normalising only where a derived numeral is produced and requiring
consumers to write canonical literals, is cheaper by however many redundant reductions the first shape
performs on already-reduced pairs, and I did not measure that difference. Both satisfy the obligation.

Whether the design wants a **`FullRange`-shaped constructor to survive as its own `Adjustment` instance**,
rather than being reduced into a bare ratio, is untouched. `1/(2^F - 1)` is a recognisable form that a
diagnostic could name and that a lowering could special-case, and reducing it into `Ratio<H, P255>` loses
that name. The consolidation's own closure gap (`26:326-331`) is where this bites; my work says the closure
is expressible, not that `FullRange` should stop existing.

**The exponent** is the one numeral member I have not traced. `Implicit<const E: Exponent, ...>` carries it
as a const parameter today (`31:335`) and the obligation reaches it the moment it is a type; whether it
should be a type at all, given that `E` is signed and small and never arithmetic in a way the wall blocks,
is a real fork I did not open.

I have not tested **whether the sealed encoding survives a downstream crate defining a foreign numeral**,
which is the untested orphan-rule direction the consolidation predicts weakly to fail (`26:496-503`). A
seal makes that prediction stronger, and whether that is a cost or the intended answer is a design call
rather than a measurement.

Everything files 34 and 35 left open and did not route through me stands exactly as they left it: the
relation-ladder fork for `Precise`, the `TotalOrd` level annotation, D39's honest content, the
dither-versus-`Refuse` choice, the `Growth`-leaves-`Policy` reading file 35 marks as reasoned rather than
compiled.

## 7. Open, net

Closed by this file, each with an artifact: file 34's value-unique-encoding obligation, stated as a normal
form (section 2), built (probes 2, 3, 4, 6) and priced (section 4), which was the one net item file 34
added. Opened by this file, one: the width chain and the integer half do not satisfy the obligation either
(probes 1, 1b), which is a correction to a claim rather than a new question, and its answer is in the same
section that raises it.

Standing from the predecessors, unchanged and untouched by me: event invariance still has no direct
measurement (`33:787-789`); the atom ladder's compile cost against a real consumer's composition set
(`26:668-674`), to which the numbers in section 4 are a neighbour rather than an answer, since they price a
different mechanism; richer canonicalisation branchlessness (`32:341-350`); division (`26:676-681`); and
`arvo-num-systems` and `notko-hlist` still unread by anyone (`26:661-666`), which I did not read either and
which bears on this file specifically, since a type-level set mechanism is the neighbour of a type-level
number one.

## 8. Standing

Nothing here overturns a D-numbered call, `30b` or `34b`. The obligation I was sent to make real was
adopted by op at `34b:25-33` and I have not renegotiated it; section 2.4 records the cheaper reading I
considered and rejected, with the argument, so that a later member reopening it knows what has to be
defeated. The one place I contradict a panel file is `34:328-331`, a passing reassurance rather than a
finding, and the contradiction is compiled in probe 1 with its type-level consequence committed refusing
in probe 1b. Four of my nine probes are committed refusing, and two of my own working assumptions were
killed by running them: that typenum's `Gcd` is Euclid, which reading its source disproved before I built
anything on it, and that a trait-level result could be projected back down into a const parameter, which
would have dissolved the whole obligation and which the compiler refused. Both are recorded rather than
quietly dropped, because a file whose every guess survived is a file that was not guessing at anything.
