# 84. The failure that is not a range event: quantise's is one after all, the kind that genuinely is not already has its generator, and the refusing tier can be made free

Daan Leijen, file 84. I wrote file 05, on fallibility without poisoning, when the panel was five files
old. Seventy-nine files have landed since and I treat none of my own earlier statements as standing;
where I lean on file 05 below it is on a probe that still compiles or on a gap I opened that has since
been closed by machinery I did not know was coming.

**What I read.** `78_consolidation_eight.md` in full, the standing base, and the deliverables since:
`79_dolan_what_capacity_is.md`, `80_leroy_the_verification_bundle.md`,
`81_fog_is_the_bitpack_cost_inherent.md`, `82_pesce_the_stretch_assembled.md`,
`83_lattner_how_many_widths.md`, each in full, plus op's `79b` and `82b`. Behind the consolidation,
with licence since each is a derivation this question sits on: file 50 sections 4.3 and 4.4 (the
grade's own generator set, which the consolidation compresses to "unchanged in mechanism" at
`78:256-260`), file 58 section 1.14 and file 49 section 1.14 (the grade's shape, which no
consolidation since restates), file 70 sections 3 and 4 (the preset rows' own arguments and the
`CanRefuse` fold), file 05 sections 2 and 3 (my own, re-read to check what survives), and
`62_probes/primary_sources.md` in full. One `ls` of the panel directory, current through `83_probes`.

The shipped tree I touched for three things and no more: the canon-gate greps, the test bodies in the
surface this file touches, and one factual check on how the tree answers a division by zero today,
which is licensed as evidence about why the redesign is happening. **No conclusion below rests on a
shipped-source citation, and every one survives deleting it.**

**Gates.** Canon gate, fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty. Test gate:
`cargo test --offline --workspace` from `mock/`, summed per binary by parsing every `test result:`
line, **666 passed, 0 failed, 9 ignored**, matching files 81, 82 and 83, and reproducible from HEAD
since commit `2e2b423` closed file 82's manifest gap (confirmed by file 83, not re-derived here). I
read the bodies of the tests in the surface I touch rather than their names:
`arvo/tests/strategy_wrapping_div_zero.rs` (six real assertions on a documented behaviour, one per
preset and sign, not tautological) and `arvo-bitmask/tests/out_of_range.rs` (five real assertions
including the specific aliasing a masked shift would produce, with the module doc stating why a
compile-time refusal is not available at that API). The one disqualifying test on record,
`arvo-tensor/tests/capacity.rs:14-18`, stands exactly as `78:874-876` and files 82 and 83 carry it:
three tautological lines, flagged for deletion rather than improvement, outside the panel's scope to
touch. Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml`, confirmed inside the tree this session, with every probe command run inside the
tree (the identical command outside resolves to stable `1.94.0`, which caught one wrong result here:
section 5.3).

**What is compiled and what is reasoned.** Sections 2, 5 and 6 are compiled or measured and trace to
`84_probes/` (six probes plus one expected-fail, outcomes and exact commands in
`84_probes/OUTCOMES.md`), built and run fresh this session. Section 1's premise check is an oracle run
against CPython's `decimal`. Sections 3, 4 and 7 are reasoned and say so per claim. The bench harness
was not run; nothing below is a timing claim. Everything is offered as suggestion; the calls are op's.

---

## 1. Two premises in the dispatch, checked before reasoning from them, and one is false

The brief hands over file 82's framing, which I am asked to derive against rather than adopt. Two of
its factual claims are cheap to check and one does not survive.

**"Rounding to `p` digits changes the value, which is precisely what `quantize` is defined not to do"**
(`82:426-427`). False. `quantize` is defined to produce the value of the operand *rounded to the
target quantum*; what it preserves is the quantum, not the value. When the target quantum is coarser
the standard rounds, delivers, and signals inexact. Run against CPython's `decimal`, an implementation
of the General Decimal Arithmetic specification that IEEE 754-2019's decimal formats align with, at
`prec = 3`:

```
quantize(1.234, 0.01) -> 1.23   flags: {Inexact, Rounded}
quantize(1234,  1)    -> InvalidOperation
```

Two different events. The first is soft and delivers a value; the second is hard and delivers nothing.
Collapsing them into one "exactness failure" is what makes the question look like it needs a new axis,
and separating them is most of the answer.

**"`quantize`'s failure is not a range event: the value is in range"** (`82:420-423`). True of the
operand and false of the result, and the distinction is the whole finding. Section 2.

The third claim in that section, that `Refuse` appears in the range row for `Precise` and no other
preset in either table (`78:412`, `78:438`), is correct; I re-read both tables.

*Grounded on: physical (the decimal oracle run, `84_probes/OUTCOMES.md`), ratified (`78:409-441`, both
tables re-read this session), settled shapes (`82:418-431`).*

---

## 2. Quantise's hard failure is a range event, on the numeral the operation targets

### 2.1 The identity, compiled cell for cell against file 80's own number

`quantize(x, q)` produces the value of `x` at quantum `q`. Its result therefore lives in a value set
that is not `N`'s: it is `{m · r^q : |m| < r^p}`, the numeral with `N`'s radix, precision and domain
and its exponent **fixed** at `q`. Call it `At<N, Q>`. That is not a new kind of object. The design
holds fixed-point and float as one formalisation differing in the exponent form, the ratified
`Numeral` contract is exactly `Radix`, `Precision`, `Exponent`, `Domain` (`78:618-623`), and a
constant exponent form is an exponent form.

Under that reading the hard failure is `OverRange` on `At<N, Q>`, and the claim is checkable rather
than suggestive, because the design already ratified what an out-of-range event *is*: the value at or
past the extended-grid rounding boundary half a top ulp beyond the maximum, with the tie resolved by
the ordinary even rule (`78:288-293`).

Compiled and run over file 80's own model, all 16,000,000 operand pairs, every count asserted, with a
negative control that must differ (`84_probes/probe_1_refusal_is_overrange.rs`):

| predicate | refusals | disagreement with A |
|---|---:|---:|
| A, "needs more than `p` digits" (file 80's) | 5,679,000 | |
| B, above `At<N,Q>`'s far point | 5,679,000 | 0 cells |
| C, the design's own extended-grid overflow | 5,679,000 | 0 cells |

File 80's number reproduces exactly through an independently written construction, **and the three
predicates agree cell by cell, not merely in total.** That agreement is a theorem rather than a
coincidence of the model, and the probe checks the theorem's own condition directly: the quotient
`value(x)/r^q` is never strictly between `r^p - 1` and `r^p` (it is an integer multiple of `r` when
`e_x > q`, an integer below `r^p` when `e_x = q`, and below `r^(p-1)` when `e_x < q`), so the half-ulp
window in which a rounding-boundary reading could differ from a digit-count reading is unreachable at
any operand pair. Zero cells land in it.

So the failure is not merely *like* a range event. It is the design's own range event, on the numeral
the operation targets, and the tables govern it as written.

### 2.2 The operation, built, one body, three preset rows exercised

`84_probes/probe_2_quantise_as_a_crossing.rs` compiles clean with zero feature gates and no new
mechanism. The arithmetic body is the design's own quantiser step (round first, classify second,
`78:206-210`) applied to the target numeral, and the resolution is a handler that the body calls
rather than a branch the body takes. That is the shape `05_probes/a_handler.rs` compiled at file 05
and file 70 folded into `Quantisation::Fallibility<T>` (`70:196-203`): the body never names a refusal
constructor, so it carries no `FromResidual`-shaped bound and instantiates at every tier.

Runner, whole matrix, counts asserted:

```
Precise sweep: cells=16000 refusals=5679 disagreements=0
Warm sweep:    cells=16000 clamped=5679  wrong=0
Hot sweep:     cells=16000 wrapped=5679  wrong=0
```

Three rows, the same 5,679 cells, each delivering its own ratified answer. Every plan quantity
(`ULP`, `MODULUS`, `MAX_MANTISSA`, `FAR_POINT`) is an associated const of `At<N, Q>` and is asserted
in const position, which is the fourth rule's own const-position requirement as file 82 stated it
(`82:481-498`) and file 83 refined into "every width-derived const names its level" (`83:290-316`).

The re-embedding into `N` is exact and needs no statement: every datum of `At<N, Q>` is a datum of `N`
whenever `q` lies in `N`'s exponent range, and *that* precondition is a type-level comparison at the
declaration site rather than a runtime check. The design gets the standard's preferred-exponent rule
for `quantize` (`Q(y)`, per `62_probes/primary_sources.md`) for free from the same framing: the result
is the target's own datum, whose exponent is `q` by construction, exact or not.

### 2.3 Which row governs is derivable, not a fork to hand over

One cell needs care and I do not want to leave it as a fork. `At<N, Q>` is a fixed-exponent numeral,
so if the fixed-point table governs, `Hot` reduces modulo; if `N`'s own table governs and `N` is a
decimal float, `Hot` goes to the far point. `Warm`, `Cold` and `Precise` are unaffected: the far point
of a bounded fixed-exponent set *is* the clamp, by the far-point rule's own third instance
(`78:279-281`), and `Precise` refuses either way.

Derived rather than picked, using file 70's own method. `Hot`'s two rows differ because `Hot` means
"as fast as possible" and the two number kinds give away different things for free: an arithmetic
right shift and two's-complement wraparound on an integer ALU, round-to-nearest-even on an FPU
(`78:415-424`, `78:443-446`). Apply the same test to `At<N, Q>` rather than looking up a table: the
row is whatever the target's own container gives away. A fixed-exponent decimal target lowered through
a software quantiser gives away nothing, so `ReduceModulo` earns nothing there and the far point is
the honest row; a fixed-exponent binary target in a native integer container does get wraparound free.
**So the row is a consequence of the target's lowering, not of a table lookup, and the two tables were
always two instances of one derivation.** That is one sentence of spec text and it removes the cell.

### 2.4 What this does to the three offered resolutions

None of them is needed, and one of them was already forbidden.

Offering `quantize` only where a refusal exists (`82:432-434`) removes a capability from three of four
presets because the substrate judged the consumer's choice wrong. `arvo-toolbox-not-policer.md:33`
forbids exactly that: "Refusals to expose a primitive because we think the consumer is misusing it."
It is not the cheapest option, it is the one option the ratified rules already close.

Making the exactness failure a grade generator (`82:435-441`) describes something the design already
has, applied to the wrong event; section 3.

Giving the presets a second resolution axis (`82:442-444`) buys nothing once the failure is a range
event, and would owe the pairwise-difference discipline the preset table already carries.

And file 80's own closing sentence, "a `NoSpecials` numeral offering `quantize` must route that branch
through the same `Refuse`/grade machinery as every other range event" (`80:133-135`), turns out to be
right for a reason it did not give. It is not that quantize should be *treated like* a range event. It
is one. The sentence stands and its justification changes.

*Grounded on: ratified (`78:275-293` the far point and the overflow boundary, `78:409-441` both
tables, `78:618-623` the `Numeral` contract, `arvo-toolbox-not-policer.md:33`), settled shapes
(`70:140-180` the rows' own arguments, `70:196-203` the `CanRefuse` fold, `78:206-210` the quantiser,
`80:118-135`), compiled (`84_probes/probe_1`, `probe_2`, `probe_2_run`, `probe_2b`), reasoned (the
target-numeral identification and the row derivation in 2.3, mine).*

---

## 3. Both halves of the answer were already ratified, in a section nobody read beside the tables

This is the part I did not expect and it is the strongest thing in the file.

**The soft event is already a grade generator.** Inexact is one of the design's own generators,
classed as a quantisation event, "raised by the quantiser, on a value it still delivers" (`50:304`).
That is precisely what `quantize` does when the target quantum is coarser. Nothing is owed.

**And the hard non-range kind already has its generators too.** The same table's second row:
`invalid` and `divideByZero`, classed as "causes with no quantiser origin, raised by the operation, on
operands, before any rounding" (`50:305`). That row is *the category the dispatch is asking about*, it
was derived against IEEE 754-2019 clause 7's own five exceptions, it was checked to be the free
commutative monoid the design already had, and it has been carried unchanged through four
consolidations (`58` section 1.14, `63`, `68`, `78:256-260`, each saying "unchanged in mechanism").

So the design does have a place for a failure that is not a range event. It is in section 1.14, not
section 1.21, and the two have never been read side by side. File 82's section 3.2 is right that the
*tables* have no row for one and wrong that the *design* has nowhere to put it.

That also closes a gap I opened at file 05 and could not resolve there. I wrote that "resolve and also
report" was a mode the vocabulary could not express at all, and that a design able to express neither
is missing something the field universally ships (`05:176-185`). It can express it now, and the answer
is better than the `ClampAndFlag` fifth resolution I was weighing: the resolution delivers the value
and the grade publishes the event, so the two halves key on different things and neither has to carry
the other's job. The machinery that closed it landed at files 47, 48 and 50, after I had left, and
nobody connected it back.

*Grounded on: settled shapes (`50:294-307` the generator table and its IEEE derivation, `49:464-546`
the grade's shape, `58` section 1.14, `78:256-260`), reasoned (the connection between the generator
table and the preset tables' scope, and the closure of `05:176-185`, mine).*

---

## 4. The general answer: three failure kinds, and what each one keys on

Stated for the spec, because the next operation whose failure is not a range event should be
recognised rather than argued about. Reasoned throughout, from the ratified pieces above.

**A failure is a point at which an operation cannot produce a datum of its result numeral. There are
exactly three kinds, and the design already sorts them.**

**Kind 1, the result value exists and lies outside the result numeral's value set.** A range event.
The `Resolution` axis governs it and the preset tables say what happens. Its two positions,
`OverRange` and `UnderRange`, are the two directions in which a computed value can leave an ordered
set. `quantize`'s hard failure is here, once the result numeral is named correctly (section 2), and so
is every arithmetic overflow. The `Resolution` axis is best read as a **totalisation** axis: the four
members are four ways to make an otherwise partial operation total, ordered by how much they lie.

**Kind 2, the result value does not exist mathematically.** Division by zero, `Recip` at zero, `Sqrt`
of a negative in a real domain. The operand is a perfectly good datum, the operation is partial as a
mathematical function, and no range is involved. This is the genuinely non-range kind, its grade
generators are `invalid` and `divideByZero` (`50:305`), and section 5 is about where the *value*
comes from, which the generator does not say.

**Kind 3, the operand is not a datum.** Closed by construction and not a runtime failure kind in this
design at all: statement 0 quantifies over every bit pattern of `Encoding::Fields`' width and
partiality is expressed by shrinking the fields rather than by a domain side-condition (`80:90-102`,
adopted at `82:296-303`), so an operand that is not a datum cannot be constructed through any tower
path, and the hand-laid path is the `unsafe impl Crosses` obligation.

**The two consequences worth spec text.** First, the preset tables' `OverRange`/`UnderRange` rows
govern kind 1 and the tables should say so in those words, which is file 82's own closing suggestion
(`82:445-448`) and the one part of its section 3.2 that survives intact. Second, and this is what the
scope sentence buys: a future operation is classified by asking which kind its failure is, and only
kind 2 needs anything the design has not already decided.

**A note on why kind 1 does not need a new position, if one ever seems to.** Should a kind-2 failure
ever want a preset row (a total division, say), the shape is a **third position on the existing axis**,
not a second axis. Positions name failure *sites*; resolutions name what happens at them. Every preset
already has a derivable answer for a new position from its own stated intent, by file 70's method:
`Hot` takes the cheapest defined value, `Warm` and `Cold` take the neutral or nearest defined value,
`Precise` refuses. A second axis would require every pair of presets to differ in two cells of a wider
table for no gain.

*Grounded on: settled shapes (`50:294-307`, `80:90-102`, `82:296-303`, `82:445-448`, `70:140-180`),
ratified (`78:409-441`), reasoned (the three-kind sort and the position-versus-axis statement, mine).*

---

## 5. Where a kind-2 failure's value comes from: four homes, priced

The generator says an event happened. It does not say what the operation returns, and for `1/0` there
is nothing to return. Four homes are available. I built all four on one operation and one shape and
measured, because the elegance ordering and the instruction ordering are not the same and I would
rather report the second.

The shipped tree's current answer is the honest motivation for asking: today a wrapping preset's
division by zero returns the numerator, described in the test's own module doc as "a cheap, defined
fallback" (`arvo/tests/strategy_wrapping_div_zero.rs:1-8`, cited as evidence about why the redesign is
happening, not as a design statement). That is an arbitrary value with no event published, and it is
exactly the state the generator table exists to replace.

### 5.1 The four homes and what each actually costs

`84_probes/probe_4_where_a_partial_operation_pays.rs`, a scaled reciprocal at `i64`, one call and a
64-element column, all four homes compiled at `-O` and disassembled.

| home | column of 64 | loop instructions | divisions | branches |
|---|---:|---:|---:|---:|
| (i) value, absorbing bottom in a spare pattern | 512 B | 18 | 1 | 3 |
| (iii) result type, `Outcome<T, DivideByZero>` | **1024 B** | 21 | 1 | 3 |
| (iv) declaration, operand carries the proof | 512 B | 19 | 1 | 2 |
| the relocated check, once at the boundary | | 10 | 0 | 2 |

**The first finding corrects the expectation, including mine.** At a dividing operation the three
homes cost nearly the same in instructions, because the division dominates and the branch is
predictable. The refusing carrier's real price is **layout**, a doubled column, which is the one axis
arvo exists for. Anyone arguing this on branch counts is arguing about the wrong number.

**The second finding is the one I did not expect, and it is against my own preferred home.** The
declaration home's loop still carries a `cbz` and a `panic_const_div_by_zero` landing pad, plus the
whole unwind apparatus. A `repr(transparent)` newtype over `i64` with a private field and a fallible
door carries no validity range, so the type system knows the divisor is nonzero and **the optimiser
does not**. The proof is free at the API and free in layout and is not free in the emitted code.

(ii), the grade alone, is not a fourth candidate and I compiled it beside (i) rather than arguing it:
a grade is a claim *about* a value, and a kind-2 failure has no value for the claim to attach to. The
grade is the witness; it is never the resolution. That is also the shape of one live finding in this
review, a published grade no consumer could act on, and the same test applies here.

### 5.2 What it takes for the proof to reach the optimiser

`84_probes/probe_5_the_proof_the_optimiser_can_see.rs`, three shapes, gate-free:

| loop, 64 elements | instructions | divisions | branches | panic references |
|---|---:|---:|---:|---:|
| newtype, proof in the type system only | 19 | 1 | 2 | 2 |
| `core::num::NonZeroI64` | 16 | 4 (unrolled 4x) | 1 | **0** |
| bare `i64`, check written out | 13 | 1 | 2 | 0 |

`NonZeroI64` unrolls four ways, keeps only the loop back-edge, and emits no check and no landing pad,
because `core` declares a validity range on it. And layout follows: `size_of::<Option<NonZeroI64>>()`
is 8 against 16 for both `Option<i64>` and `Option<Nz>`, asserted in const position.

So the mechanism the design wants is a declared validity range. Which raises the availability
question, and this is where running the check inside the tree mattered.

### 5.3 The availability check, and one stale memory corrected

On the pinned toolchain, `core`'s niche mechanism is **not** the `rustc_layout_scalar_valid_range_*`
attribute. That attribute is rejected outright, even under `#![feature(rustc_attrs)]`: "attributes
starting with `rustc` are reserved for use by the `rustc` compiler" and "cannot find attribute ... in
this scope". The live mechanism is the `pattern_types` language feature via `pattern_type!`, in
`core/src/num/niche_types.rs`, whose own module attribute reads `#![unstable(feature =
"temporary_niche_types", issue = "none", reason = "for core, alloc, and std internals until pattern
types are further along")]`.

I first ran that check outside the tree and got `E0554`, "`#![feature]` may not be used on the stable
release channel", which is the stable-1.94 resolution files 73, 75 and 82 warned about. Re-run inside
the tree it compiles, with rustc's `internal_features` lint firing: "the feature `pattern_types` is
internal to the compiler or standard library ... using it is strongly discouraged". Measured as a
reference (`84_probes/probe_5b_pattern_type_reference.rs`): 16 instructions, zero panic references,
`size_of::<Option<Pos64>>() == size_of::<Result<Pos64, ()>>() == 8`, and one thing more than
`NonZero` gives, because the range `1..` tells LLVM the divisor is *positive* and it emits `udiv`
rather than `sdiv`. The design's `Domain: SignDomain` member is exactly that kind of statement.

**Vetted per `unstable-features.md`'s own procedure, and the answer is no.** The feature is
`internal_features`-flagged and `core`'s own wording calls its use a placeholder, so the std-internal
carve-out is the only route, and that carve-out's first step settles it: "First check whether a stable
or public wrapper suffices. If it does, use the wrapper instead." One does.

### 5.4 The construction: the refusing tier for free, stably, with no gates

`Encoding` is already allowed to change which datum carries a value (`78:193-195`). So a lowering may
store the datum **biased by one**, in a `core::num::NonZero`. The excluded pattern becomes zero, the
stable niche applies, and the whole fallibility ladder gets its layout back.

Compiled and run, zero feature gates, round trip asserted in const position over the whole 65,535-value
domain rather than a sample, with the one spent pattern asserted refused rather than aliased
(`84_probes/probe_6_the_refusing_carrier_for_free.rs`):

```
Biased = 2   Option<Biased> = 2   Result<Biased, ()> = 2
Plain  = 2   Option<Plain>  = 4
column of 64:  biased-refusing = 128 B     plain-refusing = 256 B
```

| sum over 64 elements | instructions | branches | vector ops |
|---|---:|---:|---:|
| `biased_sum` | 31 | 0 | 25 |
| `plain_sum` | 22 | 0 | 16 |

**The refusing tier is the same width as the infallible tier**, and the debias is nine extra
instructions across sixty-four elements, entirely inside the vector pipeline (`add.8h` against a
broadcast `-1`), with no branch and no loss of vectorisation.

Three things make this the right shape rather than a trick. It is a `Lowering`/`Encoding` choice and
touches no `Numeral` member, so identity does not move and no law reads it. It applies to every
numeral with at least one pattern outside its value set, which is every bounded fixed-point numeral
and every float numeral whose `Specials` is not the full IEEE product. And the bias is vocabulary the
design already has, used here for a layout purpose on the lowering side rather than a value purpose on
the identity side, which is exactly the split `78:193-195` draws.

### 5.5 The answer, stated

**Kind-2 failures are refused at the declaration wherever the operand's admissible domain is
expressible as a predicate on the operand, and the design already ships that vocabulary twice over**
(notko's `NonZeroable`, and the `IsZero`/`IsNonZero`/`IsPositive`/`IsNonNegative` family in
`arvo-numeric-contracts`). The fallibility does not vanish; it relocates to the one place the fact
enters, and every downstream operation on the column is total. That is the fourth design rule applied
to fallibility: a check the type system could settle at declaration is not deferred to every use site.
Measured, the relocated check is one pass at 10 instructions per element with no division and no
carrier, against a per-operation branch and a carrier forever.

**Where the domain is not so expressible, the refusing carrier is the home, and the biased-niche
lowering makes it cost nothing in layout.** The value home stays available and stays documented, and
it keeps the obligation my own file 05 found and this file recompiled: an absorbing bottom must absorb
under selection too, and a plain total order does not give that. The bottom sorting below every value
is silently discarded by a running maximum, asserted in const position at
`probe_4`; it is the defect IEEE 754-2008 shipped in `minNum`/`maxNum` and 754-2019 replaced with
propagating `minimum`/`maximum` (`05:236-245`). A design offering the value home owes a propagating
selection contract, not a `TotalOrd` derivation.

**The grade publishes the event in every case**, and it is never the resolution.

*Grounded on: compiled and measured (`84_probes/probe_4`, `probe_5`, `probe_5b`, `probe_6`, all fresh
this session, disassembly and counts in `84_probes/OUTCOMES.md`), ratified (`78:193-195`,
`78:152-166` the pricing pillar, `unstable-features.md`'s carve-out procedure), settled shapes
(`50:305`, `05:236-245`), tree-fact (`arvo/tests/strategy_wrapping_div_zero.rs:1-8`, the current
behaviour only, as evidence of why the redesign exists), reasoned (the four-home ordering and 5.5's
statement, mine).*

---

## 6. The quantum belongs in type position, and that removes the standard's own carve-out

File 80 established that `quantize` is the standard's one exception to value-determinism because its
result value reads its operand's *datum*, so it is pair-keyed and can never be a law (`80:127-135`,
citing clause 5.2's own sentence, which `62_probes/primary_sources.md` carries verbatim). That is
correct about the two-datum signature, and the two-datum signature is not the only one available.

**The datum-dependence is entirely the second operand's.** `quantize(x, y)` uses nothing of `y` but
its exponent. Lift the exponent to type position, write `quantise::<Q>(x)`, and the result value is a
function of `x`'s value and the type-level `Q` alone. The carve-out does not apply, because the
exponent is no longer an operand. **`quantise::<Q>` is value-keyed and law-eligible**, which is the
opposite of what the two-datum form permits, and it is not a contradiction of file 80: it is a
different operation wearing the same name, and it is the one the fourth design rule reaches.

The cost of the two forms, measured (`84_probes/probe_3_quantum_binding_time.rs`, with a const-position
assertion that the two shapes compute the same function over the model's whole mantissa range):

| shape | instructions | hardware divisions | branches |
|---|---:|---:|---:|
| quantum is a type, standalone | 20 | 0 | 2 |
| quantum is a datum, standalone | 66 | 1 | 14 |
| quantum is a type, 64-element loop | 43 | 0 | 1 |
| quantum is a datum, 64-element loop | 237 | 2 | 30 |

The typed loop **vectorises to NEON 2-wide with no branch but the back-edge**: the division by the
quantum is strength-reduced against a magic constant, the ties-to-even rule becomes vector compares
and selects, and the clamp becomes `cmgt`/`bif`. The dynamic loop keeps an `sdiv` and a divide-by-zero
check inside the per-element body, because nothing in the program states the quantum. That is file
81's own bitpack finding in a second place, and it satisfies file 82's standing test verbatim: is any
quantity computed inside a per-element loop a function of the type's parameters alone (`82:513-521`).
Here it is three of them, and every one is a function of the target numeral.

**Both forms ship**, because removing the datum-taking form would be the policer posture and because
"match this other value's quantum" is a real decimal use. What the design states is what each costs
and what each is: `quantise::<Q>` is value-keyed, law-eligible, total per preset, plan-in-consts; and
`quantise_to(x, y)` is pair-keyed, never a law, and carries the runtime plan. That is a diagnostic,
not a directive (`arvo-toolbox-not-policer.md:82`).

**Conformance is a declaration-site bound, not a runtime check.** A consumer needing the standard's own
behaviour, invalid operation rather than a substituted value, states it and is refused at the call site
by a preset whose range row does not refuse. Compiled as an expected-fail
(`84_probes/probe_2b_conformance_refused.rs`), `E0277`, with rustc's own diagnostic naming `Precise` as
the remedy. This is the same shape the far-point work already ratified for `AbsorbingFarPoint`
(`78:311-316`), applied one section over, and it costs nothing at run time.

*Grounded on: settled shapes (`80:127-135`, `82:513-521`, `81:220-239`), physical (IEEE 754-2019
clause 5.2 via `62_probes/primary_sources.md`, position-cited, read this session), ratified
(`78:152-166`, `78:311-316`, `arvo-toolbox-not-policer.md:82`), compiled and measured
(`84_probes/probe_3`, `probe_2b`), reasoned (the carve-out removal, mine).*

---

## 7. What a consolidation could take, close to verbatim

*The preset tables' `OverRange` and `UnderRange` rows govern **range** events: a result value that
exists and lies outside the result numeral's value set. The design sorts every failure into exactly
three kinds and already has a home for each. Kind 1 is the range event, governed by the `Resolution`
axis, which is best read as a totalisation axis whose four members are four ways of making an
otherwise partial operation total, ordered by how much they lie. Kind 2 is the partial-function
failure, where the result value does not exist mathematically (division by zero, a reciprocal at zero,
a square root of a negative in a real domain); its grade generators are `invalid` and `divideByZero`,
ratified since file 50 as "causes with no quantiser origin, raised by the operation, on operands,
before any rounding", and what the generator does not say is where the value comes from. Kind 3, an
operand that is not a datum, is closed by construction, because statement 0 quantifies over every bit
pattern of the fields' width and partiality is expressed by shrinking the fields.*

*`quantize`'s hard failure is kind 1, not a new kind. The operation targets the numeral with the
operand numeral's radix, precision and domain and its exponent fixed at the requested quantum, and the
failure is `OverRange` on that target: checked over the whole 16,000,000-pair matrix at the decimal
model, the "needs more than `p` digits" predicate, the above-the-far-point predicate and the design's
own extended-grid overflow predicate agree cell for cell, at file 80's own count of 5,679,000. The
agreement is a theorem rather than a model coincidence, because the quotient of the operand's value by
the quantum is never strictly between `r^p - 1` and `r^p`, so quantise has no rounding ambiguity at its
overflow edge at all. The soft event, where a coarser target quantum rounds the value and delivers it,
is not a failure and is already the `inexact` quantisation generator. Under the presets the operation
is therefore total on `Hot`, `Warm` and `Cold` and refusing on `Precise`, one arithmetic body serving
all four with the resolution acting as a handler, and it re-embeds into the operand numeral exactly,
delivering the standard's preferred exponent by construction. Which row a mixed case takes is derived
rather than looked up, by the same test that produced the two tables: `Hot`'s row is whatever the
target's own container gives away for free, so a fixed-exponent binary target in a native integer
container reduces modulo and a software-quantised decimal target goes to the far point.*

*The quantum belongs in type position. `quantize`'s status as IEEE 754's one exception to
value-determinism is entirely an artifact of taking the quantum from a second datum; with the quantum
as a type parameter the result value is a function of the operand's value alone, the carve-out does
not apply, and the operation becomes value-keyed and law-eligible. The datum-taking form still ships,
stated as pair-keyed and never a law. Measured at `-O` on aarch64, the typed form is 20 instructions
with no hardware division against 66 with one, and in a 64-element loop 43 instructions with one
branch, vectorised two-wide, against 237 instructions with two hardware divisions, thirty branches and
a divide-by-zero check that cannot be eliminated. Conformance to the standard's own invalid-operation
behaviour is a declaration-site bound refused with `E0277`, not a runtime check, not a value a consumer
must inspect, and not a grade nobody reads.*

*A kind-2 failure is refused at the declaration wherever the operand's admissible domain is a
predicate on the operand, which the design already ships as notko's `NonZeroable` and the
`IsZero`/`IsNonZero`/`IsPositive`/`IsNonNegative` family; the fallibility relocates to the one place
the fact enters and every downstream operation is total. Where the domain is not so expressible the
refusing carrier is the home, and its price is layout rather than instructions: measured, the three
homes are within three instructions of each other in a dividing loop while the carrier doubles the
column. That price is removable. A proof carried only in a `repr(transparent)` newtype is invisible to
the optimiser, which still emits the check and a panic landing pad; a proof carried in a declared
validity range is not, and the stable way to declare one is to store the datum biased by one in a
`core::num::NonZero`, which is an `Encoding` choice the design already permits since an encoding may
change which datum carries a value. Compiled with zero feature gates and exact over its whole domain,
that makes `Option<T>` and `Result<T, E>` the same width as `T`, halves a refusing column, and costs
nine instructions across sixty-four elements inside the vector pipeline with no branch and no loss of
vectorisation. The value home, an absorbing bottom in a spare pattern, stays available and keeps its
obligation: an absorbing element must absorb under selection, which a total order does not give, and a
design offering it owes a propagating selection contract rather than a `TotalOrd` derivation. The
grade publishes the event in every case and is never the resolution, because a grade is a claim about
a value and a kind-2 failure has none.*

---

## 8. Out of scope, reported under the standing obligation

**The `pattern_types` availability finding belongs in `unstable-features.md` and nowhere in the panel's
output.** The rule's tables carry neither `pattern_types` nor `pattern_type_macro`, and its own
procedure says a feature must not sit unvetted. It is vetted above and the verdict is do-not-adopt,
with a stable wrapper named. The rule edit is op's, since the rule is ratified and the review already
carries three of its wording edits waiting on op (`78:863-865`).

**One stale memory is worth correcting wherever it lives.** `#[rustc_layout_scalar_valid_range_start]`
is not the niche mechanism on this toolchain and is rejected even under `#![feature(rustc_attrs)]`.
Nobody in this review has cited it, so this is prophylactic rather than a correction.

**The count discipline file 83 asked for, complied with.** Every count in this file names the artifact
that produced it: the 5,679,000 and its three predicates are `84_probes/probe_1`'s asserted output, the
sweep counts are `probe_2_run`'s, and every instruction count is read from an emitted `.s` file by the
command recorded in `84_probes/OUTCOMES.md`. Three files in a row published counts they had not
re-derived (`83:320-335`); this one publishes the commands instead.

---

## 9. What this leaves open

- **The second independent read of section 2.** The target-numeral identification is the load-bearing
  half and it is one pass. The place to attack it is the claim that `At<N, Q>` is an ordinary member
  of the ratified `Numeral` vocabulary rather than a new construct, and the claim in 2.3 that the row
  is derivable from the target's lowering rather than being a table fork.
- **Whether the `Resolution` axis's own name should change.** I read it as a totalisation axis in
  section 4 and the reading does real work, but renaming a ratified axis is op's and I do not propose
  it.
- **The propagating selection contract**, if the value home ships at all. It is one trait and it
  touches every selection site in the algorithm crates; I did not count the sites, and file 05 did not
  either.
- **`sqrt` of a negative under a signed domain**, which is kind 2 and whose predicate
  (`IsNonNegative`) already exists, so it should fall straight out of section 5.5. Unchecked.
- **Whether the biased lowering interacts with the container level file 83 just named.** The bias sits
  between the datum and the carrier, which is `embed`'s territory and statement P's; I did not check
  whether a biased carrier changes what statement C quantifies over, and it plausibly does.
- **My probe 2 models the exponent as four ZST instances rather than over the tower's sealed `Pos`
  grammar**, because the grammar is not this file's subject. A reader who wants the construction over
  the real grammar should expect the `At<N, Q>` consts to need the same structural-recursion care file
  82 established at `82:113-167`.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion, not a
ruling.

*Grounded on: ratified (`78:152-166` the pricing pillar, `78:193-195`, `78:275-293`, `78:311-316`,
`78:409-441`, `78:618-623`, `arvo-toolbox-not-policer.md:33` and `:82`, `unstable-features.md`'s
vetting procedure and carve-out), settled shapes (`50:294-307`, `49:464-546`, `58` section 1.14,
`70:140-203`, `80:90-135`, `81:220-239`, `82:296-303`, `82:410-454`, `82:481-521`, `83:290-335`,
`05:172-185`, `05:236-245`), compiled and measured (`84_probes/probe_1` through `probe_6` plus the
expected-fail `probe_2b`, all built and run fresh this session on the pinned toolchain inside the
tree; commands, outputs and disassembly counts in `84_probes/OUTCOMES.md`), physical (IEEE 754-2019
clause 5.2 via `62_probes/primary_sources.md`, read this session; the CPython `decimal` oracle run),
tree-fact (`arvo/tests/strategy_wrapping_div_zero.rs:1-8` and `arvo-bitmask/tests/out_of_range.rs`,
existence and current behaviour only, no conclusion resting on either), reasoned (sections 2.3, 3, 4,
5.5, 6's carve-out argument, and every suggestion in section 7, all mine).*
