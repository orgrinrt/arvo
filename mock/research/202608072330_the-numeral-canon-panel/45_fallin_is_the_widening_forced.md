# 45. Is the widening forced?

**Date:** 2026-08-09. **Persona:** Chris Fallin. **Mode:** explore, do not settle (`00_brief.md`, `04`,
`28`). Nothing here settles anything; where I say a route is closed, the diagnostic that closed it is
attached.

**Status: COMPLETE.** Written to disk early per `RULES.md:222-223`; extended in place.

## 0. Gates

### 0.1 Canon gate

There is no ratified canon for this panel to defend or diverge from. The fixed material is `01`, `04`,
`28`, `INTENTS.md`, the workspace discipline, and the forbidden-feature list, plus the acceptance
criterion named in `00_brief.md`'s "What is fixed" section. The question I was dispatched on, whether
the container derivation's two-output finding is forced by arithmetic or only by the type system, is
squarely inside that fixed material: it is the deciding item `44` named for whether the acceptance
criterion's plural noun ("container and numeral representations") reflects an arithmetic necessity or an
implementation artifact of this type system. Nothing below proposes work the forbidden-feature list
excludes; every probe compiles gate-free on `nightly-2026-05-28`. **Gate: passes.**

### 0.2 Test gate

There is no suite. `mock/crates` is empty by construction. My evidence is probes and direct citations,
checked by opening them myself, per the same discipline `44` applied.

## 1. The question, stated precisely, before answering it

`44` named "does `Precise` widen compute past storage" as the single deciding item for whether the
container derivation's two-output finding "is forced by arithmetic or is an artifact of how the types
are arranged" (`44:9-26`). `16` posed the same fork (`16:759-767`, carried into `OPTIONS.md`'s
"derivation's outputs" section) as: absent widening, "the pair's irreducibility rests only on the
const-to-type argument" (weaker, type-system-shaped); with widening, "the pair is irreducible as a
matter of arithmetic, full stop" (stronger, forced).

I read `15` and `16` cold before opening either file's account of the other, per the reading order in
section 10 below, and derived my own formalization of what "forced" and "arithmetic" and "the type
system" should mean here, because none of the three files that use these words defines them, and the
distinction I was asked to draw only does useful work once it is precise.

**My formalization.** The derivation is a function `Derive : (Strategy, Width, Sign) -> (Carrier,
Stride)`, where `Carrier` is the type an arithmetic operation on the numeral lowers to (what a register
holds, what an add instruction operates on) and `Stride` is the bit distance between consecutive
elements of an aggregate of the numeral. `16` already established this pair, independently, by an
injectivity argument on `Cold` (`16:126-141`) and by measured cost (`16:293-335`). The question this
dispatch names is whether the pair `(Carrier, Stride)` can be collapsed to a single output, either
because one is always computable from the other (an arithmetic fact, independent of any language) or
because Rust specifically cannot express the computation (a fact about this type system, which a
different language or a future feature might not share).

**Three sub-questions fell out of formalizing it this way, and the panel's existing files answer only
one of them cleanly:**

1. Is the requirement for **two** outputs (rather than one) forced independent of `Precise`'s
   semantics? `16`/`44`/`OPTIONS.md` frame this as blocked on `Precise`. I find it is not: section 2.
2. If `Precise` widens compute past storage, is a genuinely **third** output forced, and is that third
   output mechanically expressible under the forbidden-feature list regardless of what `Precise` turns
   out to do? Nobody in the panel asked this precisely: section 4.
3. Is `Precise`'s widening itself forced by anything, or is it one mechanism among several that could
   satisfy op's stated intent (`I7`, "accurate... especially within chains and ops")? This is the
   sharpest form of "is the widening forced by the arithmetic," and it has a rigorous answer under one
   reading of that intent and a genuinely different, cheaper answer under another: section 3.

Section 5 states what I think "forced by arithmetic or only by the type system" should mean once these
three are separated, and argues the dichotomy as posed in `16`/`44`/`OPTIONS.md` collapses two
distinguishable phenomena into one label.

<!-- SECTIONS BELOW ARE APPENDED AS THE WORK LANDS -->

## 2. Two outputs are already forced, before `Precise` is settled at all

`OPTIONS.md`'s "derivation's outputs" section states: "Whether the two-output shape is forced by
arithmetic or only by the type system, blocked on the `Precise` strategy's undecided semantics"
(`OPTIONS.md:759`). `44` inherits the same framing (`44:358-372`). Both are wrong about what is
blocked, and the mistake is checkable.

### 2.1 `Cold` alone already forces two outputs, and this does not depend on `Precise`

`16` section 2 (`16:126-141`) establishes this by injectivity: restrict the map from a declared numeral
to a container type to unsigned `Cold`, widths nine through sixteen. Every one of those eight widths
maps to the same sixteen-bit carrier, because that is the smallest native container that holds them.
Their strides (nine through sixteen bits) are eight different memory layouts. A carrier-only derivation
destroys the distinction the moment it returns.

**Nothing in this argument mentions `Precise`.** It uses `Cold` alone, whose intent ("aggressively
minimises and bitpacks," `I6`) is RATIFIED at the two-name level (`I2`, both prior-panel-ratified and
restated by op on 2026-08-08) and has never been in dispute in this panel. So the two-output
requirement was already settled, decisively, before `16`'s own file finished its first section, using a
strategy nobody is arguing about. Framing the two-versus-one question as blocked on `Precise` treats a
settled sub-case as though it were the whole question.

### 2.2 A second, independent forcing, at the wide rung, also has nothing to do with `Precise`

`16`'s own section 6 sweep (`16_probes/p5_recovery_direction.rs`) tests something narrower than "is a
one-output derivation possible": it tests whether the pair `(declared width, stride)`, taken TOGETHER as
a composite key, determines carrier. Read at the code (`16_probes/p5_recovery_direction.rs:28-65`,
opened directly): the sweep runs `for w in 1..=128u32`, and its `carrier_bits` function returns a bare
`u32` bit count, never a type.

Both choices are narrowings, and both matter. The sweep never reaches `W > 128`, which is precisely the
range where arvo's own documented architecture (quoted in the system material every member of this
panel receives) puts `Hot` on `AlignedWideBits16<BYTES>` (align 16) while `Warm`/`Cold`/`Precise` stay
on `WideBits<BYTES>` (align 1). And representing carrier as a bit count is exactly the representation
`16`'s OWN p7 (`16_probes/p7_alignment_is_not_a_third.rs`, same file, same author) uses to establish
that alignment "rides on the carrier" as a property of a type, not of a byte count. p5 never re-derives
carrier as a type, so it is structurally blind to precisely the fact p7, three subsections later in the
same file, diagnoses.

**I checked whether this blindness is exercised once the sweep is widened**, with two compiled probes.

`45_probes/p1_wide_rung_collision.rs` sweeps `W = 129..=768` using real carrier types matching arvo's
documented shape (`WideBits<BYTES>` at align 1, `AlignedWideBits16<BYTES>` at align 16), and finds:

```
W values in 129..=768 where Hot's byte count equals Warm's (no padding needed): 40 of 640

compiled, const-checked witness at W = 256:
  Warm carrier: WideBits<32>            size  32  align  1
  Hot  carrier: AlignedWideBits16<32>    size  32  align 16
  same declared width (256), same stride (256 bits), same byte count (32).
  different carrier TYPE (different alignment).
```

The witness is const-checked, not printed and trusted: `size_of::<WideBits<32>>() ==
size_of::<AlignedWideBits16<32>>()` and `align_of::<WideBits<32>>() != align_of::<AlignedWideBits16<32>>()`
are both asserted at compile time (`45_probes/p1_wide_rung_collision.rs:86-90`) and the file compiles.
A negative control at `W = 240` (`15`'s own worked example, `I=200, F=40`) confirms the byte counts
genuinely differ there (30 versus 32), so the collision is not an artifact of the check always firing.

`45_probes/p2_p5_style_instrument_is_blind.rs` reproduces `16`'s own `native`/`carrier_bits`/
`storage_bits` functions bit-for-bit, swept over the identical `W = 129..=768` domain, and reports:

```
p5-style instrument (carrier as bit count): extents mapping to more than one carrier value: 0
```

Zero, on the same domain where the type-aware instrument found forty. **This is not a disagreement
about facts.** It is `16`'s own carrier representation being unable to express the fact that would
refute its claim, the same shape of blindness `16` itself named for the panel's erasure-and-codegen
check ("its instrument is a scalar... a carrier-only derivation passes it at full marks," `16:517-521`),
now found inside `16`'s own instrument for the pair's irreducibility claim, in the same file that
diagnoses the general pattern.

### 2.3 What this means for the register's framing

Two independent, already-settled facts (`Cold`'s ratified packing intent, and `Hot`'s wide-rung
alignment choice, itself a documented architectural decision rather than anything under dispute in this
panel) each independently force the two-output requirement. Neither depends on `Precise`'s semantics.
**"Blocked on the `Precise` strategy's undecided semantics" is true of a narrower, specific numeric
comparison (`16`'s own "0 of 251 versus 64 of 251" figure, computed over a sweep that excludes both the
wide rung and sign) and is not true of the two-output question itself.** The two-output question was
answered, twice over, by material already in the panel before `16`'s file was written.

## 3. Is `Precise`'s widening itself forced, under one reading of op's intent

Sub-question 3 from section 1: is widening the ONLY mechanism that satisfies "accurate... especially
within chains and ops" (`I7`), or one option among several genuinely different designs? This is the
sharpest reading of "is the widening forced by the arithmetic," and I attacked it directly rather than
arguing about it, because the question is exactly the shape a compiler engineer checks by construction:
does a bound on representable width, at every intermediate step of a computation, provably lose
information no downstream step can recover, regardless of the rounding policy chosen at each step?

### 3.1 The two live readings of "accurate... within chains," neither of them mine to choose

`18` section 3.4 (`18:471-507`) already names one reading precisely: "a strategy that refuses on
inexact is the strategy that demands its data keep a point denotation" (`18:480-481`), measured at
4.60% to 55.56% of in-range multiplications and divisions admitted, narrowing with fraction width. This
reading needs no extra compute width at all: refusing an inexact result can be checked at the storage
width plus one flag bit.

The other reading, which `16` and `35` both assume without stating it as a choice, is: compute the
BEST APPROXIMATION to the exact chain answer, for every input, rather than refusing some of them. Op's
own words favor this reading more than the refuse-on-inexact reading, though neither is stated in these
terms by him: "the most precise possible **answer**" (`I7`, my emphasis) names an answer rather than a
refusal, and "throwing out all cold or hot axis optimisations to be accurate and precise" reads as
computing something rather than declining to. I am not choosing between the two readings; `44` already
named settling this as cheap and op's, and I agree. What I attacked is narrower and does not require
choosing: **given the best-approximation reading, is widening the only way to satisfy it?**

### 3.2 The pigeonhole argument, and why round-nearest does not save a fixed-width intermediate

Consider any chain `x -> x*a -> (x*a)*b` where `x`, `a`, `b` are `F`-fractional-bit fixed-point values
and the algorithm rounds the intermediate result back to `F` bits after the first multiply, regardless
of which rounding rule it uses. I claim: for some inputs, this necessarily disagrees with the
once-truncated exact chain answer (`x*a*b` computed exactly and rounded once, at the very end), no
matter which rounding rule governs the intermediate step.

The argument is pigeonhole, not a property of one bad rounding choice. The set of `F`-bit representable
intermediate values has `2^F` elements. If two distinct true intermediate values `x1*a` and `x2*a` are
close enough that EVERY reasonable rounding rule maps them to the SAME `F`-bit value `m`, and if the
true once-truncated chain answers `x1*a*b` and `x2*a*b` differ, then the second step, computing from
`m*b`, gives one answer for both `x1` and `x2` (since `m` no longer distinguishes them), which is wrong
for at least one of the two. This holds for ANY rounding rule at the second step too, because the
information distinguishing `x1` from `x2` was already discarded at the first step; no downstream choice
can recover it.

**I searched for such a witness exhaustively, with two independently-coded instruments, rather than
constructing one by hand and hoping it generalizes.**

`45_probes/p3_search_pigeonhole_witness.py` searches integer-arithmetic fixed point at `F = 3, 4, 5, 6`,
under both round-half-up and round-half-to-even, for every representable `(x1, x2, a, b)` quadruple.
Witnesses exist at every `F` tested, under both rounding rules, growing with `F`:

```
F=3: round-half-up 61 witnesses, round-half-to-even 46 witnesses
F=4: round-half-up 732 witnesses, round-half-to-even 638 witnesses
F=5: round-half-up 7354 witnesses, round-half-to-even 6989 witnesses
F=6: round-half-up 73461 witnesses, round-half-to-even 71627 witnesses
```

First witness at `F = 4` (round-half-up): `x1 = 8/16`, `x2 = 9/16`, `a = 1/16`, `b = 15/16`. Both `x1*a`
and `x2*a` round to the same intermediate, `1/16`. The step-wise computation gives `1/16` for both. The
once-truncated exact reference gives `0/16` for `x1` and `1/16` for `x2`. Wrong for `x1`, by an entire
representable unit at this width.

`45_probes/p4_fraction_crosscheck_and_widening_recovers.py` is a second, independently-coded instrument:
`fractions.Fraction` exact rational arithmetic throughout, a different comparison method (exact
rational comparison against the midpoint rather than the integer-division trick p3 uses). It finds the
IDENTICAL counts to p3's round-half-up figures at every `F` (61, 732, 7354, 73461), an exact cross-check
between two genuinely different code paths, and additionally checks, on every single disagreement found
across all four widths (over 80,000 cases in total): does the WIDENED computation (never rounding the
intermediate at all, rounding only once at the very end) match the once-truncated exact reference? It
does, in every case checked, with zero exceptions.

### 3.3 What this establishes, and what it does not

**If op's intent for `Precise` is the best-approximation reading (match the exact once-truncated chain
answer for every representable input), then widening the intermediate, or an information-equivalent
mechanism (guard digits, which `42:196-197` names as the same idea under a different word, or an
arbitrary-precision accumulator narrowed once at the end), is forced by the information content of the
computation itself.** No rounding-rule cleverness at fixed width closes the gap; the pigeonhole argument
is independent of which rounding rule is chosen, because it is about what information a fixed number of
bits can carry, not about how that number of bits is used. This is not a property of arvo's specific
fixed-point encoding: the search is over plain rational arithmetic, with no arvo-specific representation
anywhere in either probe, the same discipline `42` section 5.2 used for its reachability finding.

`35`'s `p10` (`35:390-423`) already found an EMPIRICAL instance of exactly this phenomenon in a
downstream EMA formula: round-to-nearest reduces the failure rate from 87.5% to 12.5% but does not
reach zero. My pigeonhole argument is the general reason that residual can never be driven to zero by a
better rounding rule at fixed width: `35`'s 12.5% is not evidence of a suboptimal rounding choice, it is
the pigeonhole collision's signature, and no amount of tuning the rounding rule removes it, only
widening does.

**What this does not establish**, and I want to be precise about the boundary: it does not establish
that op's intent IS the best-approximation reading. If the refuse-on-inexact reading (`18`) is what he
means, none of this applies, because that strategy never computes an approximation to refuse cheaply;
it needs no widening at all, and its cost is a much smaller admitted domain (`18`'s 4.60%-55.56%
figures) rather than a wider register. The two readings are genuinely different designs with genuinely
different costs, and the pigeonhole argument only bears on one of them. This sharpens `44`'s open item
into a single, precisely testable question, stated at the end of section 8.

## 4. If `Precise` widens, is a third output mechanically blocked? No.

Sub-question 2 from section 1. If `Precise` diverges its compute carrier from its storage carrier, the
derivation's output arity grows from two to three: a storage carrier (used for `size_of`/array-layout
purposes, matching the other three strategies), a stride (element spacing in an aggregate), and a
compute carrier (what an operation actually lowers to, which for `Precise`-if-it-widens is wider than
the storage carrier). I checked whether expressing this third output hits the same kind of wall `16`'s
`p5b_const_to_type.rs` compiled-refused for the two-output case.

It does not. `45_probes/p5_third_output_is_mechanically_free.rs` extends `16`'s own trait-based
derivation (`16_probes/p6_trait_form_recovers_both.rs`, `type Carrier; const Stride`) with a third
associated const, `COMPUTE_CARRIER_BITS`, and compiles it under BOTH readings of `Precise`:

```
cfg(precise_widens) = false
Precise    rung=  16  carrier=  16  stride=  16  compute_carrier=  16

cfg(precise_widens) = true
Precise    rung=  16  carrier=  16  stride=  16  compute_carrier=  32  <- compute diverges from storage
```

Both compile clean, zero feature gates, and switching between the two readings costs exactly one impl
block; nothing in `Hot`, `Warm`, or `Cold`'s impls, and nothing in the trait's shape, moves either way.
This is the same move `a-refused-bound-wants-a-trait-not-a-feature.md` names in general: the refused
form was a const expression in a bound position (`16_probes/p5b_const_to_type.rs`); the trait form,
computing named associated items in an impl rather than an expression in a bound, was never blocked and
was already sitting in `16`'s own probe directory for two outputs before this dispatch extended it to
three.

**So the entire remaining uncertainty about `Precise`'s container derivation is about op's intent, not
about what Rust can express.** Neither the two-output requirement (section 2) nor a possible
third-output requirement (this section) is gated by the forbidden-feature list or by any expressiveness
limit of the type system, once the derivation is stated as trait projection rather than as const
arithmetic in a bound. This directly narrows `44`'s open item (`44:430-436`): the question is not "can
the design express a third output if `Precise` needs one" (it can, gate-free, cheaply), it is "does
`Precise` need one" (section 3), which is a question about intent, answerable in one sentence, not a
question about mechanism.

## 5. What "forced by arithmetic or only by the type system" should mean, and why the dichotomy as
posed collapses two different things

`16`, `44`, and `OPTIONS.md` all use this dichotomy, and I want to state precisely what each half is
actually claiming, because on inspection neither is quite what its label suggests.

### 5.1 The "type system" half is weaker than its name, once you check what it actually refuses

`16`'s "second, independent reason" (`16:272-282`) is that recovering carrier from stride at a
downstream site is "recoverable by arithmetic" but "not the same as available at the type level,"
evidenced by `p5b_const_to_type.rs` refusing a const expression in a bound position with "generic
parameters may not be used in const operations," naming the forbidden `generic_const_exprs`.

That refusal is real and compiled. What it establishes is narrower than "the type system cannot express
the reduction": it establishes that ONE SPECIFIC syntactic form (arithmetic on a bare const, in bound
position) is refused. `16`'s own `p6_trait_form_recovers_both.rs`, in the same probe directory, by the
same author, shows a DIFFERENT form (a trait with associated items, computed per impl rather than by
expression) computes BOTH outputs together, gate-free, from the ORIGINAL inputs `(Strategy, Width,
Sign)`. So "the pair's irreducibility rests only on the const-to-type argument" (`OPTIONS.md:761-762`,
`44:361-362`) overstates what that argument shows: it shows one dead-end route is dead, not that the
design needs the pair to be irreducible in any sense that costs anything, because the trait mechanism
was never trying to recover carrier FROM stride alone in the first place. It computes both together,
which is a different and much cheaper thing.

### 5.2 What was actually being asked, and whether it is a question that arises in the design at all

I want to name the hypothetical the "recoverable from stride alone" framing is implicitly testing,
because once named, it is easy to see it is not a scenario anyone has proposed building. The scenario
is: a numeral type that, post-derivation, DROPS its `Strategy` type parameter and keeps only the
declared width and the derived stride, recovering carrier later, on demand, from that reduced pair. If
the design ever wanted to minimize `UFixed<I, F, S>`'s own type-parameter surface to something like
`UFixed<I, F>` with `S` erased once layout facts are known, THAT is the scenario where "is the pair
recoverable from one output alone" would matter, and the answer (per section 2's compiled collisions,
which hold with or without `Precise`) is: no, not safely.

**But nothing in this panel proposes erasing `S` early.** The acceptance criterion's own wording,
"erase on lowering" (`00_brief.md:145`, `seed/SETTLED_container.md:33-35`), places erasure at CODEGEN
time, not at the point the layout facts are derived. Every numeral type retains `Strategy`, `Width`, and
`Sign` as its own type parameters for the entirety of its Rust-level existence; that was never in doubt,
and it predates `15`/`16`'s work (`15:274-282` builds the map keyed on strategy from the start). Given
that `S` is ALWAYS available at any site holding the numeral's type, any downstream consumer that needs
`Carrier` or `Stride` can simply project `Derive<S, W, Sign>::Carrier` or `::Stride` independently, at
zero cost, via the trait mechanism section 4 confirms composes for three outputs as cleanly as it does
for two.

So the "is the pair reducible" investigation is answering a real, well-posed, but narrower question
than the framing in `OPTIONS.md` suggests: not "does the design need two separate facts to flow through
the type system as independent, persistently-carried quantities" (it does not; both are derived facts,
recomputed for free, whenever needed, from inputs the type never drops), but "could the type surface be
minimized to drop `S` after deriving layout facts" (it could not, safely, independent of `Precise`, per
section 2). The second question is real and worth having answered. It is a much smaller and more
hypothetical question than "is the two-output requirement forced," and conflating the two overstates
what `p5b`'s refusal costs the design.

### 5.3 The honest label for what forces the requirement

**Forced by semantics, not primarily by numeric injectivity and not primarily by the type system.**
Every strategy that diverges "what does an operation compute in" from "what does a value occupy at
rest" forces the derivation's output arity past one, because those are genuinely different questions
with genuinely different answers the moment ANY strategy chooses to make them differ. `Cold` diverges
them by design intent (RATIFIED, `I2`/`I6`, storage tighter than compute). `Hot`'s wide-rung alignment
choice diverges them by a documented architectural decision (SIMD-friendly padding, unrelated to
anything under dispute in this panel). `Precise`, if it widens, would diverge them a third way, by an
intent still undecided.

None of these three is forced by pure numeric injectivity in the sense `16`'s framing suggests (a
mathematical fact true in any encoding, in any language). All three are forced by a DESIGN CHOICE that
a strategy is entitled to make, and once made, no type system, however expressive, collapses the
resulting divergence into one output, because the divergence is a fact about what the two outputs MEAN,
not about how compactly a language can compute them. A hypothetical dependently-typed language with
perfect const-to-type recovery would still need two (or three) named facts here, for the same reason
Rust does: "aligned to 16" and "aligned to 1" are different ABI guarantees, not different spellings of
the same number, and no amount of type-system power makes two different guarantees into one.

So the dichotomy as posed, "arithmetic versus type system," is answering the wrong question along one
axis (the type-system half is a claim about one refused syntactic form, not about expressiveness in
general, and is answered, not merely weaker) and understating the real forcing mechanism along the
other (the arithmetic half, as `16` poses it, is one instance, `Precise`'s undetermined case, of a
forcing pattern that TWO OTHER, already-settled facts already exhibit). The requirement is forced. It
was forced before this panel started arguing about `Precise`.

## 6. Is the container derivation itself licensed, or is it an unlicensed mechanism

The dispatch asked me to challenge, in these words, whether the container derivation should exist at
all and whether it sits on the right side of the design's artifact boundaries. I looked for a reason it
does not, in the same spirit `35` section 7 used to find that a large share of the register's derived-
numeral machinery has zero located callers.

I did not find one, and I want to say so plainly rather than manufacture a doubt. The container
derivation's existence traces directly to a RATIFIED requirement: op's own words, quoted and checked
against the source myself (`seed/SETTLED_container.md:33-35`, matching character for character), state
the acceptance criterion as deriving "the matching container **and** numeral representations," plural,
two nouns. `44` traced this plural through three generations of drift and found it collapsed to singular
in the panel's own founding brief (`44:150-177`); I re-checked `44`'s citation myself:

```
grep -n "matching container" 00_brief.md
145:bytes, the typestate derives the matching container and representation, it validates, and it erases
```

Confirmed, singular, unchanged since `44` found it. This is a real, cheap, one-word fix that costs
nothing to make and I flag it again because it has now been found and reported and not fixed across two
consecutive dispatches.

**The derivation is not the unlicensed thing here.** What section 5.2 flags as narrower than it looks is
one specific sub-investigation (whether the pair is recoverable from one output with the strategy type
parameter dropped), not the derivation itself. The derivation, needing at least two outputs, is exactly
what the ratified acceptance criterion demands, independent of anything `Precise` does, per section 2.

## 7. Bearing on the live options

Per `OPTIONS.md`'s own instruction, each gets fits-well, fits-badly, or kills. I cite `OPTIONS.md` by
section and quoted phrase, not by line, per my brief.

**The derivation's outputs section.** *Corrects the framing, does not kill the finding.* The two-output
result is not blocked on `Precise`; it is already forced, twice over, by `Cold`'s ratified intent and by
`Hot`'s wide-rung alignment, independent of `Precise`. The "0 of 251 versus 64 of 251" comparison is
real but measures a narrower and more restricted domain (widths 1 to 128, carrier represented as a bit
count) than the section's prose implies. I would restate the section's fork as: whether a possible
THIRD output is forced (section 4's question), not whether the pair is forced (already settled).

**Q5, is the arithmetic column one axis or two.** *No new bearing on the axis count itself.* Section
5.3's observation, that the forcing mechanism is "does this strategy diverge compute-type from
storage-footprint," is a property of the STRATEGY axis's content (which strategies choose to diverge,
and why), not a claim about how many axes the arithmetic column has. I flag it as connective tissue to
`42`'s law-layer work (`42:34-38`, "laws come from the axis values, not from the strategy name") rather
than as a new finding about Q5.

**Q6, does `Warm` wrap or clamp.** *No bearing.* This is about the overflow policy axis, orthogonal to
the compute-versus-storage divergence question this file is about.

**Q11, what does the numeral guarantee to a fold, and what does a composition supply.** *A small
addition, not previously stated.* `35` established the container derivation's two outputs answer a
per-value question and a per-aggregate question (`35:344-349`, citing `16` section 12). My section 5.3
sharpens WHY: it is not that layout facts happen to come in a pair, it is that any strategy entitled to
diverge compute-type from storage-footprint forces the arity past one, and a fold's accumulator relation
(`35` section 3.2, capacity-derived, a THIRD aggregate-keyed output by `35`'s own count, `35:351-354`)
is a further instance of the same pattern: the accumulator's width is a fact about what a computation
NEEDS to hold, distinct from what any single value occupies either at rest or in a register. I would
not claim these unify into one mechanism; `35` already declined to claim that (`35:353-354`) and I have
no evidence past what she states.

**The Precise-on-inexact open item (`18` section 3.4, `01` section 4, `SETTLED.md:173`).** *Directly
connects, and section 3 sharpens rather than resolves it.* `18` measured the cost of the refuse-on-
inexact reading. This file measures the cost of NOT taking the widening mechanism under the
best-approximation reading (my pigeonhole argument, an information-theoretic lower bound rather than an
empirical measurement). Both readings remain live; I am not choosing between them, and doing so is
explicitly the point section 8's question is written to settle.

## 8. What I would add to the register

I am not editing `OPTIONS.md`, `INTENTS.md`, or `00_brief.md`, per my brief. These are for whoever does.

**A correction to the "derivation's outputs" section's framing.** The two-output requirement is not
blocked on `Precise`. It is already forced, independently, by `Cold`'s ratified intent (`16:126-141`,
using nothing beyond `I2`/`I6`) and by `Hot`'s wide-rung alignment choice (`45_probes/p1`, `p2`,
compiled, 40 of 640 collisions in a domain `16`'s own instrument cannot see). What genuinely remains
blocked on `Precise` is narrower: whether a THIRD output is needed, and section 4 shows that question is
not blocked by the type system either; it is answerable in one sentence by op.

**A sharpened, single question for op**, combining `18`'s finding, `35`'s p10 finding, and this file's
pigeonhole proof into one decidable choice, offered as the shape of the question rather than as an
opinion on the answer:

> For `Precise`, does "the most precise possible answer... especially within chains and ops" mean (a)
> matching the exact, once-rounded chain result for every representable input, which by `45`'s pigeonhole
> argument requires an intermediate computed wider than storage (or an information-equivalent mechanism,
> such as guard digits), or (b) refusing an operation whose result cannot be represented exactly, which
> by `18`'s measurement admits 4.60% to 55.56% of in-range multiplications and divisions depending on
> fraction width, and needs no extra compute width at all?

**A note that the "recoverable from stride alone" sub-investigation (`16` section 6, `p5`, `p5b`)
answers a narrower, more hypothetical question than the register's framing suggests**, per section 5.2:
whether the numeral's own `Strategy` type parameter could be dropped after deriving layout facts,
recovering carrier from the reduced pair later. Nothing in the design proposes doing this, and the
acceptance criterion's own "erase on lowering" places erasure at codegen, not at derivation time. I
would not remove the sub-investigation's finding (it is real and correctly established: the reduced pair
is not recoverable, with or without `Precise`), but I would rename what it is evidence FOR: not "the
pair is irreducible" as a cost the design pays, but "the type surface cannot be minimized past keeping
`S`," which is a smaller and cheaper claim that happens to have already been the design's assumption all
along.

**A cheap, unclaimed check, extending `45_probes/p1`'s methodology.** The 40-of-640 collision count is
specific to Hot's align-16 choice at exactly the widths where Warm's natural byte count already lands on
a multiple of 16. Whether a similar collision exists for the `Cold` strategy at the wide rung (does
`Cold`'s own standalone carrier, at `W > 128`, ever collide with `Warm`'s in size-but-not-alignment the
way Hot's does) was not checked; the wide-rung `Cold` carrier's exact rule was not stated anywhere I
read, and I would rather flag the gap than guess at it.

## 9. What I could not determine

**Which reading of `Precise`'s intent op means.** Section 3.1 states both readings precisely and section
8 sharpens the question. Neither `36` through `39` settles it, and I agree with `44` that this is cheap
and worth asking directly rather than guessing.

**Whether `Cold`'s wide-rung carrier collides with anything the way `Hot`'s does.** Named as a gap in
section 8; I did not attempt it because I do not know `Cold`'s wide-rung rule and did not want to invent
one.

**Whether the pigeonhole argument's magnitude (the fraction of chains affected) has been measured for
arvo's actual widths, rather than the small `F = 3` through `6` swept here.** `35`'s `p10` gives one real
downstream instance at a specific width; a general sweep tying chain length and `F` to the fraction of
affected inputs, matching `35`'s `p3` methodology for the reassociation question, would be a natural
next probe and I did not build it, because my question was existence (is a witness findable at all,
under every rounding rule) rather than magnitude (how often does it bite in practice), and the dispatch
was about the first.

**Whether there is a cheaper widening than "double the native rung."** My mechanism probe (`45_probes/p5`)
models Precise-widens as doubling the compute carrier's bit width, matching `16`'s own model
(`16_probes/p5_recovery_direction.rs:56-62`). Whether a narrower widening (say, `F` extra guard bits
rather than a full doubling) suffices for the pigeonhole argument to close is a real, cheap, unaddressed
question: my witnesses used one multiply step of headroom, and a chain of `K` multiplies plausibly needs
headroom growing with `K`, the same shape `35` section 3.2 derives for the fold-accumulator's
`ceil(log2 C)` growth. I did not derive the exact growth rate for a multiplicative chain and flag it as
the natural extension of this file's method.

## 10. Coverage, bounded honestly

**Read end to end, directly, before touching any summary of them:** `INTENTS.md`, `00_brief.md`,
`RULES.md`, `44`, `15`, `16`, `42`, `35`, `18` (section 3.4 and surrounding, opened at the passages
cited), `34`, `36`, `37`, `38`.

**Read at the specific passages I cite, by opening the lines:** `OPTIONS.md` (the derivation's outputs
section, lines 703-777, plus Q5, Q6, Q11), `DROPLIST.md` (lines 125-154, the retired "partial
associativity" entry, checked to confirm it is a different question, associativity-under-regrouping
rather than compute-width, from the one this file addresses), `seed/SETTLED_container.md` (lines 25-40).
Every `file:line` in this document was opened and its content checked against my claim, not merely
resolved.

**Not read:** `02` through `14` (except `15`, `18`), `17`, `19` through `33`, `39` through `41`, `43`,
`CANON_CANDIDATE.md`, `MORNING.md`, `PERSONA_CALLS.md`, the closed predecessor panel, `archive/`,
`seed/` beyond the one file cited. Where I refer to `18`'s or `35`'s findings past what I quote
directly, I read the cited passage myself rather than relying on `44`'s or `OPTIONS.md`'s account of
it, per `RULES.md`'s standing rule that the next dispatch on a shared, unread source reads the source.

**Not verified:** whether `43`'s grid-invariance apparatus (cited by `44` section 6, not read by me
directly) would extend to the compute carrier the way it extends to the storage carrier and the fold
accumulator. I did not open `43` and am relying on `44`'s account of it for that one connection only.

**Probes:** `45_probes/`, committed with sources and raw compiler/interpreter output, all on
`nightly-2026-05-28` / Python 3.14.6, zero feature gates in every `.rs` file (`grep -c '^#!\[feature'
45_probes/*.rs` returns 0 on all three). `p1` and `p2` (compiled Rust, the wide-rung collision and the
demonstration that `16`'s own representation cannot see it). `p3` and `p4` (two independently-coded
Python instruments, integer-arithmetic and exact-Fraction, cross-checked to identical counts). `p5`
(compiled Rust, both under both readings of `Precise`, showing the mechanism side is not what is
blocked).

**One instance of evidence is never enough, and I want to state where this file sits on that bar.** The
wide-rung collision (section 2.2) has two independent compiled instruments (`p1`, `p2`), both mine, so
by `RULES.md:116-118` that is one instance of evidence wearing two hats, not two independent instances;
what would make it a second, independent instance is a different author re-deriving the same collision
from arvo's actual wide-rung rule rather than from the documented shape I reconstructed. The pigeonhole
argument (section 3.2) has two genuinely independently-coded instruments (`p3`, `p4`, different
algorithms, cross-checked to identical counts), which is closer to the bar but is still one author's
work; a third instance, ideally built by someone attacking the argument rather than confirming it, would
be worth having before treating either finding as more than a strong, compiled, single-author result.
