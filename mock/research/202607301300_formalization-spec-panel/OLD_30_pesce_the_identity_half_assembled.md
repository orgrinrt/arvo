# 30. The identity half, assembled, and what it costs a team to live with

**Member:** Angelo Pesce. Rendering-direction lens: the algorithm is never the whole decision. Who
authors the content, what the tools cost to build and maintain, how a junior will misuse it, and
whether anyone can still change it in three years are part of the technical evaluation, not soft
afterthoughts. My subject here is the whole picture across the three identity files, and specifically
whether a set of individually good decisions adds up to something a team can build, ship and live with.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: green everywhere, zero
failures, nine ignores, of which one is the catalogued divide gap
(`crates/arvo/tests/fixed_point_div.rs:111`, tracked #5) and eight are `ignore`-fenced doctests. File
27's "1 ignored" counted unit tests only and file 28 already reconciled the two counts; both are
consistent with what I ran. I read test bodies rather than names in the surface this file touches.
`crates/arvo/tests/identity_laws.rs:1-21` is the honest full-matrix shape and its own module doc records
that sampling was the prior failure. The refusal that cannot be written as a runtime assertion is pinned
as nine compile-fail cases with committed `.stderr` files under `crates/arvo/tests/ui/`, and
`no_multiplicative_identity.rs:1-11` states in its own doc comment why no correct expected value exists
to write down. That is the strongest shape a suite in this workspace can have and I found nothing
tautological in it. Canon gate: the governing calls are the D-numbered ones in
`202607301200_topic.the-formalization-spec.md`, whose own charter is "the point of sending it out is to
have it torn apart" (`202607301200:18-19`). Where this file argues against a ratified call it says so in
place and leaves the call where it belongs.

**What I read:** `26_consolidation_two.md` in full; `27_carmack_what_a_number_is.md`,
`28_leroy_what_identity_must_express.md` and `29_wronski_the_quantisation_contract.md` in full, with all
seven of their probes and all three `OUTCOMES.md`; `202607301200_topic.the-formalization-spec.md` in
full. Directory listed once. No other panel file fetched.

**What I compiled:** the workspace suite (above) and five probes in `30_probes/`, all `#![no_std]`, all
`const`-only so that compiling is the check, none using any unstable feature, each with a negative
control that was run and fails with `E0080`. Full record in `30_probes/OUTCOMES.md`. **Everything else
in this file is reasoned**, and where I reason from prior art I know first-hand (the OCP microscaling
formats, converter input stages) I say so and do not present it as checked against arvo's tree.

## 0. Cheap checks, run before reasoning from anything

Three of the brief's premises and three of the files' factual claims, checked.

`arvo-num-systems` does not exist. `ls mock/crates/` returns sixteen crates and none of them is it;
`grep -rn "Natural" mock/crates/ --include="*.rs"` returns nothing. File 27's brief-breaking check at
`27:31-40` is correct, and the consolidation's framing of the crate as "the single cheapest, most
repeatedly-flagged open item" (`26:661-666`) is right for the reason file 27 gives: there is no cost to
discover because nothing is built.

File 28's refutation of the spec's unsigned faithfulness blanket stands. I re-ran
`28_probes/probe_1_unsigned_blanket_refuted.rs` and it compiles; the witness
`(25 + 10) + 5 = 5` against `25 + (10 + 5) = 0` under `SubstituteZero` is real, and `SubstituteZero` is
in the vocabulary at `202607301200:146` precisely so SystemC's `SC_SAT_ZERO` is expressible. The blanket
at `202607301200:212` is false in that cell.

File 29's citation of the analogue-to-digital-converter sentence as `202607301200:118 to 120` is off by
ten lines; the sentence is at `202607301200:128`, and lines 118 to 121 are D69's logical-width
paragraph. The claim is true and the citation is wrong. I raise it only because the consolidation names
"prose claims about the design's own state are checked by nothing" as a standing failure mode
(`26:111-119`), and citation drift is that failure mode's cheapest instance. Three of the four panel
files in this stretch, including this one until I checked, cite by line number into files that are still
being edited.

And one substantive correction to file 27, which I compiled rather than argued:
`27:189-192` says that with infinity representable, overflow-to-infinity "stops being an out-of-range
resolution at all, because the range has a top element and past the top is unreachable". That is false,
and the detail matters for section 3. Section 4 states what is true instead.

## 1. What the identity half is, stated as a design

Here is the assembled thing. I state it first and defend it afterwards, because a design that has to be
reconstructed from its own defence is a design nobody will read.

### 1.1 Identity answers four questions and no others

**Which numbers exist.** That is all identity is for. Concretely: at what radix, to what precision, with
what exponent behaviour, over what sign domain, plus whether any data exist that carry no number at all.

```rust
pub const trait Numeral {
    type Radix:     Radix;        // 2 and 10 instantiated; any r expressible
    type Precision: Precision;    // significand digit count, primitive
    type Exponent:  ExponentForm; // where the exponent lives; nests the rest
    type Domain:    SignDomain;   // NonNegative | Symmetric | AsymmetricLow
}

/// One exponent for the type, so the spacing is constant and an affine map
/// over the whole set exists. The affine parameters live here because this
/// is where the affine map exists.
pub struct Implicit<const E: Exponent, A: Adjustment, B: Bias>;

/// An exponent that varies per value, so the spacing is per binade, there is
/// a bottom to fall off, and non-numeric data become possible.
pub struct Ranged<const EMIN: Exponent, const EMAX: Exponent, U: Underflow, S: Specials>;
```

Four top-level members instead of the spec's five, with `Adjustment`, `Bias`, `Underflow` and `Specials`
nested where they mean something. This is file 27's proposal 2 (`27:141-155`) with file 27's proposal 1
folded in (precision primitive, width derived, `27:82-123`) and file 27's proposal 3 partially
relocated: `Radix` is a top-level member, `Specials` is nested on `Ranged`, and signed zero is **not**
here at all, for the reason in section 2.2.

The nesting rule is the spec's own, applied consistently instead of once. `Underflow` nests "because a
constant exponent has no bottom to fall off and would have to carry a value meaning the axis does not
apply" (`202607301200:98-99`), and by that argument so do the other three: a stored exponent has no
global affine map for a `Bias` to shift, and an implicit exponent generates no infinities.

I add one forward provision that costs nothing today and is expensive to retrofit. **`ExponentForm`
should be understood as an open branch set, not a two-case enum**, because there is a third case in
shipping silicon that neither branch expresses: block floating point, where one exponent is shared
across a fixed-size block of values. It is the OCP microscaling family (a shared 8-bit exponent per
block of 32 elements) and it is in current machine-learning hardware. I know this as prior art and have
not checked it against arvo's tree. The point is not to build it. The point is that under the nested
shape adding a `Shared<BLOCK, ...>` branch later is additive, while under a flat axis list with a
`WellFormed` compatibility predicate it means revisiting every compatibility row. That is a concrete
argument for nesting that file 27 did not have, and it is the argument I find most persuasive, because
it is about what the shape costs the fourth person to touch it rather than about elegance.

### 1.2 The encoding is a separate contract, and it lives inside `Lowering`

```rust
pub const trait Lowering {
    type Encoding:    Encoding;          // new
    type StoredWidth: StoredWidth;
    type Widening:    Widening;
    type Layout:      StorageLayout;
}

pub const trait Encoding {
    /// How the sign is indexed. Independent of the sign DOMAIN above.
    type SignIndexing: SignIndexing;   // TwosComplement | SignMagnitude | OnesComplement | Offset
    /// Field widths, hidden bit, encoding bias, reserved codes. Every
    /// interchange-format parameter the identity used to carry.
    type Fields:       FieldLayout;
    /// Which datum is delivered where several carry one value. IEEE's
    /// canonical encoding; decimal's preferred exponent; NaN canonicalisation.
    type Canonical:    Canonicalisation;
}
```

Nested rather than a third type parameter, so `Number<N, S>` keeps its two-parameter fused form and the
1.8x diagnostic-length cost the three-parameter split was measured to carry (`26:32-35`) is not paid
again. Rewrite cost against the shipped tree is one associated type on a trait that is being rewritten
anyway.

**`Lowering`'s charter has to be restated, and this is the load-bearing sentence of the whole
assembly.** The spec says a `Lowering` member "changes only the emitted code and the bytes held"
(`202607301200:65-67`). With `Encoding` inside it that is false: a sign-magnitude encoding delivers a
negative zero where a two's complement one delivers a positive zero, and IEEE gives that datum
observable consequences. The restatement is one word:

> **`Lowering` changes no value.** `Encoding`, nested inside it, may change which datum carries a value.
> Every operation whose result depends on that is declared a datum-level operation, and no law may read
> one.

That restatement is what lets file 28's datum/value distinction and file 27's coordinate inversion
coexist. Without it they fight, which section 2.2 shows they currently do.

### 1.3 The crossing contract is a section-retraction pair, not an isomorphism

File 28 proposes two round-trip theorems as "the entire contract between the sides" (`28:84-86`). One of
them is false whenever file 28's own section 2 applies. The correct contract is three statements, and
the third is a derived boolean rather than an assumption:

1. `decode ∘ encode = id` on **values**. Always. This is what makes every proof on the identity side
   transfer to a stored artifact.
2. `encode ∘ decode` is **idempotent** on data. Always. It is canonicalisation, and it is what IEEE
   means by a canonical encoding, what decimal means by a preferred exponent, and where NaN
   canonicalisation lives.
3. `encode ∘ decode = id` on data **iff the encoding is injective**, which is derivable from the
   encoding's own members. This is the special case, not the contract.

All three are per-value statements over a finite datum set, so the model-width exhaustive mechanism
(`26:72-75`) applies unchanged, and the same-text-monomorphised-twice discipline of the recovery-map
witness applies to the encode/decode pair verbatim, exactly as file 28 argued for its two-theorem
version. Probe 3 checks all three at a sixteen-datum model across four indexings and three domains.

### 1.4 What the quantiser sees, and what it may deliver

Two amendments, both from file 28, both of which I second and one of which I extend.

**Round first, classify second.** The map is "round on the unbounded-exponent extension of the grid by
the direction triple, then classify the rounded result against the range and resolve by the range
rules" (`28:235-238`). File 28 priced this as one sentence replacing one sentence and gave two payoffs.
It has at least four, and section 4.1 shows the third is that it reproduces IEEE's overflow threshold
exactly, while file 29 found the fourth (it is the only place a caller could inject a pre-rounding
perturbation, `29:86-137`).

**"The neighbour that exists" is derived against the numeral's value set, including specials.** This is
already the spec's rule for clamping (`202607301200:159-163`); it just has to be read against a set that
now has an element above the largest finite when `Specials` says so. Probe 2 shows that with those two
in place, IEEE 754's overflow behaviour and SystemC's and MATLAB's saturation are the same pipeline over
different numerals, with no new axis, no new `Resolution`, and no rule of its own at the boundary.

### 1.5 What membership can say

File 27 and file 28 independently reached the same correction to D39 and it is right: **membership
licenses the exact operation family and nothing else** (`27:242-252`, `28:296-300`). Quantised
operations get their laws from the ladder, keyed as the consolidation already keys them.

Two amendments to that correction that neither made, both consequences of specials existing:

**The finest-system projection quantifies over the rational image, not over the data.** The spec's own
derivation sentence already reaches for this: "a floating numeral adds the specials that inhabit
nothing" (`202607301200:199-201`). But that sentence is ambiguous between two readings, and they differ.
If `Inhabits<S>` is a per-value statement over every datum, a float numeral inhabits nothing at all and
the derivation table's float rows silently produce no membership. If it is a statement over the
numeral's rational image, the float rows work and the specials are simply outside the quantifier. Both
readings fit the sentence; only the second is usable. The fix is one clause in the derivation's
statement, and it is the datum/value distinction earning its place a second time, on a case neither
file 27 nor file 28 connected it to.

**`ExactWindow` is not total where specials exist.** File 28's window-closure mechanism (`28:302-336`)
rests on the exact widening family being where "the inclusion is a homomorphism and the ambient laws
transfer" (`28:337-343`). With infinity representable, `∞ * 0` lands in no window, so the family is
partial and nothing transfers unconditionally. `ExactWindow<Op, Rhs>` therefore either requires
`Specials = None` or carries the fallibility, and the first is the honest first ship because the
fixed-point half is where the exact family actually earns anything.

## 2. Where the three files conflict, and how it resolves

Three real conflicts. One is inside a single file.

### 2.1 File 28 contradicts itself between sections 1 and 2

Section 1 states the crossing as two round-trip theorems including "encode after decode is the identity
on bit patterns" (`28:84-86`). Section 2 then names three separate entrances to non-injective
interpretation: signed zero, NaN payloads, and decimal cohorts (`28:120-138`), and decimal is not
optional because D58 leans on it as the proof case for the whole three-contract split
(`202607301200:70-75`).

Each of those makes the second theorem false. Probe 3 checks it: at a sixteen-datum model,
`encode(decode(0b1000))` under sign-magnitude is `0b0000`, and the same failure appears under ones'
complement. What survives is idempotence, which is section 1.3 above.

This is not a nitpick about phrasing. The two-theorem version says "the only thing either side may
assume about the other is the round-trip pair" (`28:96-98`). If a design builds to that sentence, every
downstream artifact is entitled to assume a bijection that does not hold, and the failure surfaces as a
lost negative zero or a wrong cohort member somewhere far from the assumption. The three-statement
version costs one extra derived boolean and closes it.

Worth saying plainly, because it bears on how the rest of this review should be read: file 28 is the
most careful file in this stretch, its author wrote both sections in one sitting, and the two do not
compose. This review has now found that pattern four times. Proposals that each compile alone are not
thereby compatible, and nobody has been checking.

### 2.2 File 28's `Sign` axis undoes file 27's inversion

File 28 proposes `Sign` with three instances, `Unsigned`, `TwosComplement`, `SignMagnitude`
(`28:189-192`), justified by: "at the same width, a two's-complement signed numeral represents an
asymmetric range with one zero, and a sign-magnitude signed numeral represents a symmetric range with
two zeros. Different representable sets, same axis instance" (`28:182-185`).

That sentence bundles two facts of different kinds. The **range** difference is about values. The
**zero count** is about data. Under file 28's own coordinate split the first is identity and the second
is encoding, so the three-instance axis puts a datum-level fact back on the value-level side that file
27's inversion had just cleared of encoding parameters. It is also stated in width coordinates, which
the inversion abolishes.

The resolution is to split what the sentence bundled:

- **Sign domain**, on identity: `NonNegative`, `Symmetric`, `AsymmetricLow`. This is a value fact.
- **Sign indexing**, on encoding: `TwosComplement`, `SignMagnitude`, `OnesComplement`, `Offset`. This is
  a datum fact.

Probe 3 checks that these are genuinely independent, which is the test for whether a bundling loses
information: the symmetric domain is served by all three signed indexings, and two's complement serves
two different domains. Naming one does not name the other.

Three things fall out, and the third is the one I would put in the spec:

File 28's conclusion survives. The three-instance axis was right that `Signed` alone under-determines
the set; it was one axis short, not one axis wrong.

Signed zero lands where file 28 wanted it (derived, not a `Specials` entry, against file 27's
`27:195-201`) but on the other side of the split: it is the datum/value collapse of a non-injective
**indexing** at magnitude zero, so it is an encoding fact with declared observable consequences, which
is exactly what section 1.2's restated charter is for.

**SystemC's `SC_SAT_SYM` stops being a saturation mode.** Probe 3: with the axes split, the same
`TowardNegative` clamp delivers -8 over an asymmetric domain and -7 over a symmetric one. Symmetric
saturation is a numeral choice, not a `Policy` instance, and the spec's claim that saturation is just
`(TowardNegative, TowardPositive)` at the range members (`202607301200:159-162`) stays true for both.
That is one fewer thing in `Policy` and one fewer special case in the `conv-systemc` alias set, which is
the kind of subtraction that makes a design cheaper to live with rather than more clever.

### 2.3 File 29's shaper conflicts with the fold algebra, and the conflict is a naming problem

File 29 says this is the point it would push hardest on if it were deciding alone (`29:385-392`): a
shaped fold forfeits the regrouping machinery of section 1.4 of the consolidation, the design has no
mechanism to say so at the type level, and the failure mode is "a scheduler that regrouped a feedback
loop because nothing told it the loop was there". The analysis is correct and the alarm is warranted.

I think the mechanism it asks for is not owed, because the conflict is an artefact of putting the shaper
on the wrong combinator.

**Error feedback is a scan, not a fold.** It is a stateful map over a sequence producing a sequence, and
that is what every instance of it in the field actually is: Floyd-Steinberg diffuses error across
pixels, a delta-sigma converter shapes across samples, and in both cases the output is the sequence.
File 29's probe 2 threads its shaper through a reduction, which is why the regrouping conflict appears
at all. A scan has no grouping freedom to forfeit, because a scan is sequential by definition.

The one genuinely shaped **fold** is compensated summation, which is a real and useful thing. But the
answer there is the same and it needs no new mechanism: `fold_compensated` is a distinct named
combinator, sequential by construction, sitting beside `fold`, which is the one that requires the
regrouping fact. Two function names with different bounds is a structural refusal, and it is file 29's
own preferred crossing pattern (the consolidation's "shallower entry point closer to where the
composition is still concrete", `26:489-495`) applied to file 29's own proposal.

The counter-reading, carried because it is not weak: if a future scheduler consumes combinators
generically and picks an execution strategy from a fact rather than from a name, then the name is not a
refusal and a type-level marker is owed after all. Whether that scheduler exists is a hilavitkutin
question the consolidation already ruled is not arvo's to answer (`26:143-146`). So the shape of the
answer depends on something outside this design, and both readings should be recorded rather than one
chosen here.

I would add one thing file 29 left as a free choice that should not be. Its `ErrorShaper::State` is an
open associated type (`29:228-233`). The carried residual is the difference between an exact value and
its quantisation, so it is bounded by one quantum in magnitude, always. **`State` should be derived from
the numeral, not chosen by the implementor**, both because a wrong choice is silently unsound and
because deriving it is what makes the cost comparison in section 4.4 statable.

## 3. The composition matrix

The brief asks which proposals genuinely compose and which merely coexist in separate files. I walked
every pair that could plausibly interact. Here is what I found, worst first.

**Broken as stated, fixed above.** File 28 §1's round-trip pair against file 28 §2's non-injectivity
(section 2.1). File 28 §3's `Sign` against file 27's inversion (section 2.2).

**Broken as stated, and nobody had noticed.** File 29's `quantize_dithered` against the `Refuse`
resolution. Probe 4: the top of a `UFixed` range is exactly representable and the undithered path
returns it unchanged, and one quantum of positive dither on that same input refuses under `Precise`.
File 29 says only that at the ends "the ordinary `OverRange` resolution takes over exactly as it would
for any other value that landed there" (`29:96-99`), which for the one resolution that is not total
means a caller's choice to decorrelate silently makes a computation fallible, on inputs the numeral
represents exactly. Every value within one dither amplitude of either end is affected. The fix is in
probe 4 and it is what a real converter's input stage does: confine the perturbed value to the numeral's
range before quantising. Checked over the whole range and every admissible draw at one and two quanta of
amplitude, it refuses exactly where the undithered path refuses, which is nowhere, and leaves the
interior mechanism intact. It costs the dither its uniformity within one amplitude of each end, which is
a real and known cost in file 29's own field, not a free repair.

**Broken as stated, and the fix is one word.** The finest-system derivation against `Specials`, and
`ExactWindow` against `Specials` (section 1.5). Both are the same omission: two files reasoned about
membership and window closure before specials were on the table, and neither went back.

**Compose, and together produce more than either claimed.** File 28's round-first amendment against file
27's `Specials`: probe 2, section 4.1. File 28's rational-pair adjustment against the closure gap:
section 4.3 gives the bias half a closed form, and the two halves together close `26:326-331`
completely. File 29's dither against round-first: file 29 is right that the amendment is its
precondition, and I would go further than file 29 did and say this is the strongest single argument for
the amendment, because a bug fix can be argued with and an enabling mechanism cannot.

**A stated mechanism that is off by a factor of two, on the two directions that matter most.** File 29
says "two positions with the same value modulo the quantum receive the identical error, always, from
every `Direction` the contract can express" (`29:70-73`). Not for `ToEven` and `ToOdd`, where the tie is
broken on the quotient's parity, so the error is periodic with period `2Q` rather than `Q`. Probe 4
pins it. File 29's own probe used nearest-ties-away, where the period is `Q`, so its measurement did not
reach the case. The conclusion survives untouched, because a period of `2Q` is still a pure function of
the input and a pure function cannot decorrelate. But `ToEven` is what `Warm`, `Cold` and `Precise` all
use (`202607301200:250-257`), so the design's default rounding rule already has twice the period file 29
credits it with, and a reader sizing a dither amplitude off that sentence would size it wrong.

**Compose silently and change something the design does not track.** Adding `Radix` changes the meaning
of two existing `Direction` markers. `ToEven` and `ToOdd` are predicates on the last significand digit,
and in radix 10 that is a decimal digit, which is what IEEE's decimal roundTiesToEven is defined on. The
markers survive; their implementations must read the radix. One sentence, and it is the sort of thing
that is free to write now and expensive to discover from a failing decimal alias later.

**Do not interact, checked.** File 27's precision-first inversion against the multiplicative half's
verified width adder: "precisions add, exponents add" is arithmetically the same adder over the same
numbers, since `P = I + F` and `(I1+F1) + (I2+F2)` is the product's precision. Nothing in section 1.5 of
the consolidation moves. Round-first against `Growth::Narrowed`'s double-rounding finding
(`26:300-306`): the first step still rounds, on a different grid, and the 12 to 19 percent disagreement
is unchanged.

## 4. Four things built past the three files

### 4.1 The overflow vocabulary is complete, and here is the proof

The consolidation claims the quantisation vocabulary "reproduces every named rounding and overflow mode
in IEEE 754, SystemC and MATLAB's Fixed-Point Designer with no gaps needing their own name"
(`26:48-50`). File 27 correctly found that claim false as it stood, because IEEE's default overflow
produces infinity and nothing in the vocabulary can (`27:185-189`). Its proposed repair is the part that
is wrong: overflow-to-infinity does **not** stop being an out-of-range resolution, and "past the top" is
not unreachable (`27:189-192`).

The reason it is wrong is worth stating because it is the crux of the whole top cell. With infinity in
the set, "nearest" between the largest finite and infinity is undefined, because the distance is
infinite and no midpoint exists. IEEE does not use neighbour comparison there. It defines overflow by
the unbounded-grid device: the result is rounded as though the exponent range were unbounded, and the
round-to-nearest boundary is `2^emax * (2 - 2^-p)`, the midpoint between the largest finite and the
first value of the binade that does not exist. That midpoint is on the unbounded grid, which is exactly
what file 28's amendment supplies.

Probe 2 puts the two together at a model float (radix 2, precision 3, emax 2) and checks agreement
exhaustively against three oracles written from the standard rather than from the pipeline:

- roundTiesToEven, where the tie at the boundary resolves upward because the largest finite is an odd
  multiple and the first value past emax is an even one, so infinity is delivered at exactly the
  magnitude IEEE says and the boundary case needs no rule of its own.
- roundTowardZero, where the largest finite is delivered and infinity never is.
- roundTowardPositive, where anything above the largest finite gives infinity and there is no boundary
  at all.

All three agree, with the same five positions, the same direction vocabulary and no new `Resolution`.
Removing `Specials` from the identity leaves the identical pipeline delivering the largest finite, which
is SystemC and MATLAB: one identity member decides it and no `Policy` member changes. And the probe
shows IEEE's mode coupling carries real information rather than restating a default, because pairing
ties-to-even in range with the largest-finite over-range resolution disagrees with roundTiesToEven at
the boundary. So D66's own test (a convention's modes must be writable as aliases,
`202607301200:280-283`) passes for IEEE overflow, and the four `conv-ieee754` rows earn their existence.

What this does not show, stated because a green probe over one corner is the weakest signal in the room:
underflow and subnormals, the sign of a zero result, NaN propagation, and the negative half of the range
are all outside it. The vocabulary is complete for overflow. Whether it is complete for float is not
established by this.

### 4.2 The consumer's divergence path, which nobody in this review has priced

Every file in this stretch has priced axes. Not one has priced what a consumer writes when a preset does
not fit. As the spec stands, `S` implements both `Policy` and `Lowering`, five associated types between
them (`202607301200:49-59`). A consumer who wants `Warm` with a different tie direction declares a
marker and restates all five, four of which they did not want to touch. That is a copy of a preset, and
a copy drifts the moment the preset moves.

This is, in my judgment, the single largest usability defect of the design as it stands, and it is
invisible from inside the axis discussion because the axis discussion never leaves the substrate. It is
also the failure I have watched sink otherwise good rendering pipelines: the abstraction is correct, the
common path is fine, and the first person who needs a variant forks it, and three years later there are
five almost-identical policy markers with one owner each and nobody can change any of them.

Probe 1 checks a fix and it works, on stable-shaped Rust with no unstable feature at all: **a preset can
be a partially applied generic whose untouched members project out of the preset it derives from.** A
generic parameter default of the form `<Warm as Policy>::Quantisation` resolves. So:

```rust
type MyPolicy = Like<Warm, ToOdd>;   // one override, four inherited, nothing restated
type Unchanged = Like<Warm>;         // reproduces Warm's fingerprint exactly
```

The probe checks the five-member fingerprint of the override equals the hand-written copy's, and that
zero overrides equals the parent's. The limit is real and recorded in the probe: generic arguments are
positional, so the first divergence is free and a later-positioned one costs spelling the earlier
positions. Two readings of what to do about that, and I hold both:

Order the members by how often they are overridden and accept the positional cost for the rare ones.
Cheapest, zero machinery, and the ordering is a judgement someone will get wrong once and then live
with.

Or add a declarative macro over the same mechanism, so overrides are named rather than positional. Costs
a macro to build and maintain, buys an error message that names the member, and is the shape that stays
readable when the axis count grows, which section 6 says it will.

Either way the mechanism underneath is the probe's, and the important thing is that it exists and is
free. I would build the positional form first and the macro when the second consumer asks, which is the
one place in this file where I am arguing for less rather than more.

### 4.3 Biased multiplication is closed, and the formula generalises the one that ships

The consolidation lists the closure gap as open with two candidate fixes, "a general rational-adjustment
constructor or a `MulClosed` condition with explicit renormalisation" (`26:326-331`). File 28 proposes
the rational-pair adjustment (`28:319-328`), which addresses the adjustment half and is silent on the
bias half.

The bias half has a closed form and it is not hard. For `v1 = A1*k1 + B1` and `v2 = A2*k2 + B2`:

```
v1*v2 = (A1*A2)*k1*k2 + (A1*B2)*k1 + (A2*B1)*k2 + B1*B2
```

The cross terms the consolidation names are real, and every one of them lies in the lattice generated by
`A1*A2`, `A1*B2` and `A2*B1`. So the product numeral is

```
adjustment   L = gcd(A1*A2, A1*B2, A2*B1)
bias         B1*B2
```

Probe 5 checks this exhaustively over the full window product for six operand pairs, including a
MATLAB-shaped slope-and-bias pair at scale 1000. With both biases zero the formula returns `A1*A2` and
bias zero, so **the multiplicative half's verified width adder is the special case rather than a second
rule that has to agree with this one**, which is the property that decides whether a generalisation is
worth having. The negative control replaces the formula with the naive `A1*A2` and fails on the biased
case, so the cross terms are load-bearing rather than an artefact of the numbers chosen.

Three honest limits. `L` is a lattice containing the product set, not necessarily the finest one, which
is what closure needs and is the safe direction. The product's width is a separate computation, ordinary
but unwritten. And the `FullRange` case needs file 28's rational pair before `A1*A2` and the gcd are
expressible at the type level at all, so the two proposals are complements: the rational pair makes the
quantities expressible, the gcd formula makes the product closed. Together they close `26:326-331`
rather than half of it, and the `MulClosed` gate stops being a permanent restriction and becomes what it
should have been, a first-ship simplification.

One verification I could not do and will not assert: whether MATLAB's `fi` actually offers multiplication
between two biased objects. D66's test says that if it does, the abstraction must express it, and this
formula is how. Somebody should check the MATLAB documentation before the alias set is written, because
the answer changes whether this is required or merely available.

### 4.4 Shaping against widening, priced

File 29 presents error feedback as the complement to interior safety and says so correctly
(`29:242-250`), but neither file gives the number that decides between them, and it is a one-line
derivation both were positioned to make.

Interior safety needs `ceil(log2(n-1))` extra accumulator bits for an additive fold of length `n`, and
`ceil(log2 n)` above the product width for a multiply-accumulate (`26:155-158` and `26:268-271`), and
buys grouping independence, which is what the measured 2x single-thread win from splitting one
accumulator into four (`26:140-142`) actually depends on.

Shaping needs one carried residual, bounded by one quantum in magnitude, so `O(1)` extra bits, plus one
subtract per step, and forfeits that 2x.

At `n = 1024` that is ten bits against two, and a 2x throughput difference on the fold. So the tradeoff
is not close in either direction and it does not need a bench to state: **widen when bits are available,
shape when the accumulator width is a fixed external constraint.** File 29 says this in words
(`29:246-250`); saying it in bits is what lets a consumer decide without measuring.

And it is what makes section 2.3's derivation of `State` matter: if the residual's type is derived from
the numeral, the `O(1)` is a fact the design can state. If the implementor picks it, it is a hope.

## 5. What a consumer writes

The common path does not change, which is the ratified call (D48, `202607301200:315-318`) and is
correct. `UFixed<13, 3, Warm>` reads as itself and expands to a composition whose identity parameters
are precision coordinates the alias arguments already are.

The float path gains a spelling nobody will write by hand and everybody will read in an error message:

```rust
type Binary64 = Number<
    Float<Radix2, Precision<53>, Ranged<Emin<-1022>, Emax<1023>, Gradual, WithSpecials>, Signed>,
    Ieee754<Interchange<64>, /* ... */>,
>;
```

That is behind `conv-ieee754` and a consumer writes `f64`-shaped aliases. It is still a real cost and
section 6 counts it.

A consumer who diverges on one policy member writes `Like<Warm, ToOdd>` (section 4.2). A consumer who
diverges on domain writes one bound and one impl per system they handle, in file 27's probe A shape,
which is compile-verified in both directions. A consumer who dithers writes:

```rust
let out = Q::quantize_dithered(exact, self.noise.next());  // arvo never generates the noise
```

and a consumer who shapes writes a different combinator name, not a different policy:

```rust
scan_shaped::<FirstOrderFeedback, _>(input, &mut out);   // sequential by construction
```

A consumer who does none of these writes nothing new and pays nothing, which is the bar file 27 set at
`27:333-347` and every proposal here meets.

## 6. What it costs

I would rather state this badly than not at all, because a recommendation without a price is marketing.

**Axis count.** The spec ships ten. The assembly above reaches roughly sixteen reachable members
(`Radix`, `Precision`, `ExponentForm` with four nested members on one branch and three on the other,
`SignDomain`, the two `Policy` members with the quantisation triple and pair inside one of them, and
four `Lowering` members with three nested inside `Encoding`). Nesting keeps the *reachable* product far
below the flat product and deletes the meaningless points, which is real, but the diagnostic a consumer
sees when a bound fails is longer, and the alias table is bigger. Nobody has measured either. Both
belong in `mock/benches/` alongside the four other unpriced compile-time items the consolidation already
lists (`26:668-674`), and they should go in the same measurement rather than as a sixth separate one.

**New machinery to build.** A `Radix` marker set with one instantiated arithmetic. A `Specials` member
and the "neighbour that exists" derivation against it. An `Encoding` contract with three members. Three
crossing laws with model-width witnesses, one per (numeral, encoding) pair. The sign-domain and
sign-indexing split, with a well-formedness row saying which indexings serve which domains. The
per-end, per-resolution faithfulness table replacing the refuted blanket. The `Like` override struct,
which is thirty lines. The gcd closure formula, which is one const fn. The dither entry point, which is
one function per composition. The `ErrorShaper` trait and a closed set of instances. The scan and
compensated-fold combinators.

**What it deletes**, which is the part these lists usually omit. The `Stored`-versus-IEEE unresolved item
(`26:649-652`) goes away rather than being fixed. `SC_SAT_SYM` stops being a `Policy` instance. The
overflow-to-infinity gap closes without a new resolution. The `MulClosed` permanent restriction becomes
a temporary one. The `Adjustment`-times-`Bias` meaningless axis-product points stop existing. That is
five things off the open list against roughly eleven added to the build list, which is an honest trade
rather than a good one, and it is the trade the standard at `26:534-541` asks for.

**Rewrite cost against the shipped tree.** Near zero on the fixed-point side, for file 27's reason: the
shipped `IFixed` already computes width from `I` and `F` at its declaration site
(`crates/arvo/src/ifixed.rs:37-40`), so the parameters do not move and only the axis they expand into
does. The float side is a rewrite either way, because `FastFloat`/`StrictFloat` are IEEE-width-tagged
today and the whole `Numeral` contract is new. `Encoding` nested inside `Lowering` rather than as a
third type parameter is what keeps this from also being a signature churn across every consumer.

**Maintenance cost, which is the one I actually worry about.** The design now has three places where a
single wrong row is silently wrong everywhere: the finest-system derivation table (file 28 already flags
this, `28:346-354`), the well-formedness rows pairing sign domains with indexings, and the
`conv-*` alias sets, which are the design's own falsifiability test and are therefore the thing whose
being wrong nobody would notice. All three are per-value or per-row finite claims, so all three take the
same model-width exhaustive witness, and the discipline that keeps them honest is the one the workspace
already has. The cost is that somebody has to write the witness for each, and the pattern this review
has established four times over is that the first two attempts at any new witness each have a hole the
next reader finds by compiling it.

## 7. Proposals, standing, and what I did not do

1. **Split the sign axis into a value-side domain and an encoding-side indexing.** Probe-verified
   independent. Fixes file 28 §3's conflict with file 27's inversion, derives signed zero where it comes
   from, and turns `SC_SAT_SYM` from a saturation mode into a numeral. File 28's conclusion survives;
   it was one axis short rather than one axis wrong.
2. **Replace the two round-trip theorems with the section-retraction triple.** `decode ∘ encode = id` on
   values, `encode ∘ decode` idempotent, bijection derived rather than assumed. Probe-verified false as
   stated and true as amended. This is where canonical encodings, preferred exponents and NaN
   canonicalisation live.
3. **Nest `Encoding` inside `Lowering` and restate its charter as "changes no value".** Keeps the
   two-parameter fused form, keeps laws unable to read the encoding, and is what makes proposals 1 and
   2 coexist with file 27's inversion instead of fighting it.
4. **Adopt round-first and `Specials`, and record that together they reproduce IEEE overflow exactly.**
   Probe-verified against three attributes and the boundary case. Correct file 27's "past the top is
   unreachable" in place, because a later reader will otherwise build the top cell on it.
5. **Quantify the finest-system derivation over the rational image, and gate `ExactWindow` on
   `Specials = None`.** Two one-word fixes to proposals that were written before specials existed.
6. **Ship the `Like` preset-override mechanism.** Probe-verified, no unstable feature, thirty lines,
   and it is the difference between a consumer diverging by one token and a consumer forking a preset.
   Positional first, macro when a second consumer asks.
7. **Adopt the gcd closure formula for biased multiplication.** Probe-verified, generalises the shipped
   exact-product rule rather than replacing it, and together with file 28's rational pair closes the
   whole closure gap. Verify MATLAB's actual behaviour before writing the alias row.
8. **Confine the dithered value to the numeral's range before quantising**, or offer the dithered entry
   point only where the out-of-range resolution is total. Probe-verified that the first restores
   totality; I have not decided between them and the choice is about what a `Precise` consumer expects,
   which is the same open question the consolidation already has about what `Precise` is for
   (`26:608-617`).
9. **Name shaping a scan rather than a fold, and derive its `State` from the numeral.** Dissolves the
   type-level refusal file 29 says is owed, into two combinator names with different bounds. Carried
   with its counter-reading, which depends on whether a future scheduler picks strategies from facts
   rather than from names.
10. **Record that `Radix` changes what `ToEven` and `ToOdd` mean.** One sentence now, one confusing
    decimal alias later.

**Standing.** Nothing here overturns a D-numbered call that files 27 and 28 had not already put in front
of op. On D65/D69 (the inversion) and D39 (membership) this file is a third reading agreeing with two
independent ones, which adds no standing under the workspace's own rule and is recorded so nobody counts
it as if it did. Proposals 1 and 2 are corrections to an unratified panel proposal, not to a call.
Proposal 4 corrects an unratified claim in file 27. All of it remains op's.

**What I did not do.** No compile-time or monomorphisation measurement of anything proposed here; it
belongs in `mock/benches/` and I did not put it there. No probe of the `Encoding`-nested-in-`Lowering`
shape actually resolving through arvo's real trait vocabulary, which is the one structural claim in
section 1.2 I am reasoning about rather than compiling, and it is the first thing I would check next. No
decimal cohort machinery, no subnormal or underflow behaviour, no NaN propagation, and nothing about the
negative half of the float range. And I did not read `notko-hlist`, which six members have now flagged
and which is still unread, though after file 27's check on `arvo-num-systems` I would expect the same
answer: it does not exist yet, so there is no cost to discover, only a design to write.
