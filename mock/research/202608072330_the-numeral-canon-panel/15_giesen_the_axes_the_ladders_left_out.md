# 15. The axes the ladders left out

**Date:** 2026-08-08, overnight. **Author:** Giesen, dispatched as a synthesiser rather than a
specialist. **Status:** breadth pass under `04`. Nothing here settles, nothing converges into a
closure, and where I agree with `12` or `13` that agreement is a result to present rather than a
conclusion to adopt.

This file is being written to disk before the work is done and extended in place, per
`RULES.md:168`. If it ends mid-sentence, the sections above the break are what landed.

## 0. Gates

### 0.1 The canon gate

There is no ratified canon for arvo. This panel is writing the first one, and `01` section 0 carries
op's correction that the `SETTLED.md` rows marked RATIFIED were classified under a superseded
reading. So there is nothing to defend and the governing material is the narrow set recording op in
the loop: `01`, `04`, and the quoted passages inside `SETTLED.md`.

Nothing this dispatch asks for is forbidden by that material. Two rows bear on it directly and both
push the same way. The acceptance criterion at `SETTLED.md:65-71` requires the typestate to derive
"the matching container **and numeral representations**", plural and two nouns. And `130b:39-43`, as
quoted at `11:871-873`, says the strategy guides container selection. Both are arguments **for**
doing the work this dispatch names rather than against it.

**Gate: passes.** Checked against `01`, `04`, `RULES.md`, and `SETTLED.md` lines 63-71, 80-98 and
100-111.

### 0.2 The test gate

There is no suite to run. This panel produces probes and prose, and the repository's `mock/crates`
is under a nuke. I read the four predecessor probe directories' outputs rather than their prose
where a claim mattered, which is the applicable form of the gate here, and section 12 says which.

### 0.3 Breaking my own brief before reasoning from it

`RULES.md` asks for this first, and it paid.

**The brief says `06` found "negative integer width has a caller, at 15 of 6561 operand pairs".**
That is true and it is not the whole sentence, and the missing half changes what the gap is. Section
1 has the check.

**The brief says "every ladder built this stretch maps width to container".** True. Verified in
section 3.1 by grepping the four probe directories rather than by reading the four files, and the
sharpest form of it is that `Cold` appears in **zero** probe files of the second stretch.

**The brief says the design has four strategies and one of them, `Cold`, is bitpacked.** True of the
shipped source and of `arvo-toolbox-not-policer.md`. Section 3.3 is where that turns out to matter
more than the brief expects: `Cold` is what forces the container map to have a second output, and the
second output is the acceptance criterion's own second noun, which nothing in this panel had used.

**The brief asks whether the wide rung and alignment are a third gap.** They are not, and section 5
says why: they fall out of the map once it has that second output, and they are invisible without it.

<!-- SECTIONS BELOW ARE APPENDED AS THE WORK LANDS -->

## 1. Gap two first, and the brief's summary of `06` is one clause short

The brief says `06` found negative integer width has a caller at 15 of 6561 pairs. It does, and the
sentence that matters is the next one. `06:400-403`:

> **The tight product form does not**: at 15 of 6561 pairs its answer has negative integer width
> (`p6.out`), and the design either admits that shape, or clamps upward and gives up tightness, or
> refuses.

So negative integer width is not a shape the design is *obliged* to carry. It is the price of the
tight product form, and clamping is a live option that costs one bit of width in a corner. That
reading changes the question from "can the arrangements carry a shape they must carry" to "what does
carrying it cost, against what clamping costs".

### 1.1 I reproduced the count with my own instrument, and the region is exact

`15_probes/q01_negative_width_recount.py`, three instruments so that agreement is arrival rather than
a rerun. **A** enumerates the exact rational product set and reads the least containing numeral off
the definition. **B** is A with the enumeration removed, so A against B checks the algebra rather
than the definition. **C** is `06`'s own piecewise rule quoted from `06:570`, so B against C is a
real second opinion on `06`'s number.

```
python3 q01_negative_width_recount.py
  A vs B over 625 pairs of box 4: disagreements = 0
  B vs C over 6561 pairs of box 8: disagreements = 0
  box 4: 7 of 625      box 6: 11 of 2401      box 8: 15 of 6561
```

Every number `06` reports, reproduced. The region is exactly the fifteen pairs `06` describes, one
operand `U<0,1>` and both purely fractional, and the deepest integer width reached is **-1**.

**A wrong turn worth recording, because it nearly shipped a false agreement.** My first A and B
shared a bug: `bits_for(units)` written as `(units + 1).bit_length()`, which is wrong at every
`units`. A and B agreed everywhere except the degenerate case, because the bug was common to both.
The cross-check caught it only because B short-circuits the degenerate case and A does not. Two
instruments that share a line of reasoning are one instrument, and the run that looked like a
disagreement was the only thing that revealed it.

### 1.2 The thing nobody asked, and it is the answer

Every arrangement encodes a **natural**. The checkpoint reads that as a collision with `06`. It is
one only if the numeral has to store the integer width, and the same probe answers whether it does:

```
product, over all 6561 pairs:
  pairs with I < 0: 15
  pairs with F < 0: 0
  pairs with W < 0: 0
addition, over the same 6561 pairs:
  pairs with I < 0: 0
  pairs with W < 0: 0
```

**The integer width is the only coordinate that goes negative.** Total width and fraction width never
do, at either site with a caller. So a numeral keyed on `(W, F)` rather than `(I, F)` carries the
whole negative-width region in naturals, with no signed ladder anywhere, and the integer width
becomes a derived view `I = W - F` that is computed for display and never stored.

And the corner `06` flagged as untested compounds, decisively, in one direction only. Repeated
squaring of `U<0,1>` (`q02` output):

```
  step 0: I=0   F=1    W=1 F=1
  step 1: I=-1  F=2    W=1 F=2
  step 2: I=-3  F=4    W=1 F=4
  step 3: I=-7  F=8    W=1 F=8
  step 5: I=-31 F=32   W=1 F=32
```

The integer width runs to `-(2^k - 1)`. The total width does not move. One bit holds every one of
those values, because two values need one bit however far below the point they sit. `06:613` names
repeated multiplication as one of three untested things that would make the corner larger; measured,
it makes the corner unbounded in the coordinates the panel has been using and constant in the ones it
has not.

### 1.3 Both shape rules survive the change of coordinates, and one gets shorter

`15_probes/q02_wf_coordinates.py`, over the same 6561 pairs.

| rule | in (W, F) | disagreements with ground truth |
|---|---|---|
| tight product | `W = W1 + W2 - [min(W1,W2) = 1]`, `F = F1 + F2` | 0 of 6561 |
| classic sum | `F = max`, align, `W = max(A1,A2) + 1` | 289 of 6561 |
| join | `F = max`, align, `W = max(A1,A2)` | 0 of 6561 |
| meet | `F = min`, align down, `W = min` | 0 of 6561 |

The product rule is the striking one. `06:572` noticed that its correction "never looks at the
integer-fraction split"; in `(W, F)` that is not an observation about the formula, it is the formula.
The alignment subtraction addition needs, `F_out - F_i`, is negative at **0 of 6561**, because
`F_out` is a maximum, so it is a natural subtraction with a provably smaller subtrahend and needs no
signed anything.

**The meet is where it would break, and the meet's caller is unsettled rather than absent.** Feed the
fifteen negative-width outputs back in and the meet reaches `W = -1`. Two looks failed to find a
caller, `03` section 1.1 ("I could not find, in everything I read, an operation the design needs that
the meet answers... **if that holds after a second read**") and `06:377-382`, and `06` states the
weight of that correctly: "two arrivals at a negative... raises the cost of the claim being wrong
without establishing it". `03:738` calls it "the highest-value open thing this file produces and I
could not settle it". So the exposure is bounded by a question nobody has closed, and the meet is the
one place a signed ladder would be forced. That is worth writing down rather than leaving as an
absence.

### 1.4 An unlicensed finding: addition's rule overshoots too, and nobody had measured it

`06:551` measured the product form's overshoot. Nobody measured addition's, and the classic
`I = max(I1,I2) + 1` rule is not tight either: **289 of 6561 pairs**, of which 161 involve the
zero-only numeral and **128 do not** (`q02` addendum). The plain witness is `U<0,1> + U<1,0>`, whose
values run to `3/2` and fit in `W = 2`, where the rule says 3.

`15_probes/q03_tight_addition.py` derives a tight form and it is exact at **0 of 6561** wrong. The
derivation is four lines and it is in the probe header. Align to `F = max(F1, F2)`, let
`d_i = F - F_i` and `A_i = W_i + d_i`, then the carry bit is needed exactly when

$$\min(A_1, A_2) > \max(d_1, d_2)$$

which is a comparison of naturals, not of exponentials. So addition costs two maxima, one minimum,
two safe subtractions, one comparison and a conditional successor, and is then exact.

The size of it, in the currency the erasure gate cares about, is that a wasted bit only matters when
it crosses a container rung: **36 of the 289 cross one** (`q03` output), against 112 of 476 for the
product. **Unpriced**, in the sense `RULES.md:119-122` reserves: no bench harness run bears on what a
container jump costs and I am not reaching for a number.

### 1.5 A count of mine disagreed with `06`, and it reconciles exactly

My instrument says the sum-of-widths product form overshoots at **476** pairs. `06:551` says 461.
This panel already carries one unreconciled discrepancy that `09` and `14` both call poison, so:

`15_probes/q03b_reconcile_461_vs_476.py`, whose whole job is to close this:

```
pairs where the sum-of-widths form exceeds the exact width: 476
  at least one operand is the zero-only numeral:            160
  narrower operand has total width 1, non-degenerate:       316
    of those, the exact answer has I < 0:                    15
    of those, the exact answer has I >= 0:                  301
```

`06`'s own decomposition is "160 pairs where one operand denotes only zero" plus "301 pairs where the
narrower operand's total width is 1" plus "16 pairs where the clamp hides the waste, which are the 15
negative-width pairs plus the doubly degenerate one". Every one of those terms appears above, exactly:
**160 + 301 + 15 = 476**, with the doubly degenerate pair inside the 160, which is why `06` writes 16
rather than 15. `06`'s 461 is its own first two terms; the fifteen it accounts for separately because
the clamp hides them there.

**Both numbers are right and they answer different questions.** Neither should be quoted without its
convention. There is nothing to escalate here and I am not proposing anything be changed in `06`.

## 2. Gap two, compiled

Prose about coordinates is worth nothing until it builds. `15_probes/q04_core.rs` is the nat
machinery: `Z`, `O<N>`, `E<N>`, addition, comparison, ceil-halving, the byte buffer and the rung
selector are lifted from `13_probes/p09_core.rs`, which is a spike and is cited for what it proved
rather than for how it is written. Three things it did not have are mine.

**Canonical natural subtraction.** `E<Z>` is a second spelling of zero and `13`'s `Cmp` reports it as
greater than `Z`, so every construction that builds an even node goes through a three-impl `MkE`
normaliser. Subtraction is seven impls plus six with borrow, total only where the subtrahend is
smaller, and **deliberately without an impl where it is not**, so an underflow is a compile refusal
rather than a wrapped value.

**Whole-matrix check, not a sample.** `q05_subtraction_matrix.rs` asserts `Dif<Na, Nb> = Na-b` for
**every** pair with `b <= a` in `0..=64`, which is 2145 assertions, plus 484 max and min assertions.

```
grep -c 'Sub<N' q05_subtraction_matrix.rs   ->  2145
rustc +nightly-2026-05-28 --edition 2024 --crate-type lib q05_subtraction_matrix.rs --out-dir build
  exit 0
```

**Then the two shape rules, over the whole box.** `q06_shape_matrix.rs` asserts the tight product at
every non-degenerate pair and the tight sum at every pair of the 81-shape box:

```
grep -c 'ProdS<Shape' q06_shape_matrix.rs   ->  6401   (6400 assertions plus the impl)
grep -c 'SumS<Shape'  q06_shape_matrix.rs   ->  6562   (6561 assertions plus the impl)
rustc ... q06_shape_matrix.rs --out-dir build   exit 0, 20.4s
```

Zero feature gates, default solver, edition 2024.

**And the negative control fires, so the matrix is not vacuous.** `q06_negctl.rs` falsifies one
expected value, the `U<0,1>` squared case, asserting `W = 2` where the rule gives 1:

```
error[E0277]: the trait bound `Shape<O<Z>, E<O<Z>>>: Same<Shape<E<O<Z>>, E<O<Z>>>>` is not satisfied
```

`Shape<O<Z>, E<O<Z>>>` is `Shape<1, 2>`. The machine has printed the negative-integer-width case
back, as `W = 1, F = 2`, and it did so in naturals.

**So gap two closes in the direction nobody expected.** The arrangements do not need extending, they
need re-coordinating, and the re-coordination is free at the mechanism level. What it is not free in
is the diagnostic, which is section 4.

## 3. Gap one. The three-input map builds, and it needs two outputs rather than one

### 3.1 First, the brief's claim, checked

`grep -c 'Strategy\|Hot\|Warm\|Cold\|Precise'` over the four probe directories is not the check,
because the markers are present as inert phantoms. The check is whether anything **consults** them.

```
grep -rn "Cold"    10_probes 11_probes 12_probes 13_probes | wc -l   ->    0
grep -rn "Precise" 10_probes 11_probes 12_probes 13_probes | wc -l   ->    0
grep -rno "Hot\|Warm" 10_probes 11_probes 12_probes 13_probes | wc -l ->  648
grep -rnE "impl[^(]*(for|<) *(Hot|Warm)\b" 10_probes 11_probes 12_probes 13_probes | wc -l -> 0
```

`Cold` and `Precise` appear **zero times** in the second stretch's 261 probe files. `Hot` and `Warm`
appear 648 times and **no impl anywhere matches on either of them**, which is the precise form of
"inert marker": present in every type, consulted by nothing. The brief is right, and the three files
that said so about themselves (`10:552-555`, `12:496-497`, `13:494`) were right.

### 3.2 The map, built

`15_probes/q07_three_input_map.rs`. The shape is `(strategy, width, sign)` into **two** outputs, and
the second one is where the interesting part is.

The width side is unchanged from `13`'s ladder and it is deliberately kept: a pure function of `W`
into one of six rung markers, which mentions no strategy and no sign. Keeping it is a result. It was
right, it is total, and making it strategy-aware would have been churn.

The sign side is a two-impl record: `Signedness` names five native primitives, `Unsigned` gives
`u8..u128` and `Signed` gives `i8..i128`. That is a closed record over a closed vocabulary and
**nothing in it grows when a consumer writes a new width**, which is the property that keeps it off
`SETTLED.md:110`'s enumeration refusal.

The strategy side states, per strategy, what it realises at each of the six rungs. Twenty-four impls,
counted by the command in `15_probes/`:

```
AtRung impls: 24        (6 for Hot written out, 6 x 3 in the align-1 macro)
Signedness impls: 2
StrideOf impls: 4
width-keyed impls anywhere in the map: 0
```

**Zero impls are keyed on a width.** The table of six is a table over rungs, which are a fixed
vocabulary of the hardware's shapes, not a table over the widths a consumer might write.

Whole domain, not a sample. `q08_map_matrix.rs` asserts container and stride for **four strategies by
two signs by every width 0 to 200**, 1608 triples:

```
rustc +nightly-2026-05-28 --edition 2024 --crate-type lib q08_map_matrix.rs --out-dir build
  exit 0, 2.55s
```

Zero feature gates in any file (`grep -c '#![feature' q0*.rs` returns 0 for all fourteen).

### 3.3 The second output, which is the acceptance criterion's second noun

`SETTLED.md:65-71` says the typestate derives "the matching container **and numeral
representations**". Two nouns. Every probe in this panel has derived one thing.

`Cold` is what forces the second. A bitpacked value has no standalone Rust type, because nothing has
a size of five bits, so either `Cold` is not a container choice at all or the map has a second
output. It is the latter, and the split is clean:

- **Container**, the standalone value's type. `Cold` agrees with `Warm` here. There is nothing to
  choose: a lone `UFixed<13,3>` is a `u16` whatever strategy you asked for.
- **Stride**, the bits one element occupies inside an aggregate. `Cold` is exactly `W`. The others
  are the container's width.

`q13_cold_packed.rs` builds a column that reads only the second output and knows nothing about which
strategy it is serving. One million elements:

```
UFixed<3,0,U,Cold>     stride=  3 bits    375000 bytes
UFixed<3,0,U,Warm>     stride=  8 bits   1000000 bytes
UFixed<5,0,U,Cold>     stride=  5 bits    625000 bytes
UFixed<5,0,U,Warm>     stride=  8 bits   1000000 bytes
UFixed<20,4,U,Cold>    stride= 24 bits   3000000 bytes
UFixed<20,4,U,Warm>    stride= 32 bits   4000000 bytes
UFixed<200,40,U,Hot>   stride=256 bits  32000000 bytes
UFixed<200,40,U,Warm>  stride=240 bits  30000000 bytes
```

That is the whole of `Cold`'s reason for existing, expressed as one associated type, and it is
**unpriced**: no bench harness run bears on what the packed access costs against the padded one, and
nothing here measures it.

### 3.4 My map was wrong twice, in the same way, and measuring is what found it

Both are worth reporting because both are the shape of defect a shapes-and-sizes probe cannot see.

**First**, I computed the non-`Cold` stride as `8 * ceil(W / 8)`. It is wrong at **every width that
rounds up a rung**. At `W = 24` the container is `u32`, four bytes, and `8 * ceil(24/8)` is three. A
column would have been sized `3N` bytes for an array of `u32`. The stride for a strategy that stores
whole containers is the **container's** width, not the value's, and the only place that is known is
the rung.

**Second**, having moved the stride onto the rung, it was still wrong at the wide rung, because
`Hot`'s wide arm pads to align 16. At `W = 200` the payload is 25 bytes and the container is 32. So
**the stride is not a function of the rung either**: it belongs to the `(strategy, rung)` pair,
which is where it now sits.

Both are visible at runtime and both are asserted statically now (`q13` output):

```
the derived stride against the container's real size, in bits:
  UFixed<20,4,U,Warm>      derived=  32   8*size_of(container)=  32
  UFixed<20,4,U,Cold>      derived=  24   8*size_of(container)=  32
  UFixed<200,40,U,Hot>     derived= 256   8*size_of(container)= 256
  UFixed<5,0,U,Cold>       derived=   5   8*size_of(container)=   8
```

Two negative controls pin the repairs. `q08_negctl.rs` asserts `Cold`'s stride at `W=5` as 8 bits and
is refused. `q08_negctl2.rs` asserts `Hot`'s wide stride at `W=200` as 200 bits, which is what the
first version produced, and is refused:

```
error[E0277]: the trait bound `E<E<E<E<E<E<E<E<O<Z>>>>>>>>>: Same<E<E<E<O<E<E<O<O<Z>>>>>>>>>`
              is not satisfied
```

**The generalisation is the finding, not the two bugs.** A derivation that produces only a container
can be checked against `size_of` and looks right. A derivation that produces a container **and** a
layout has an internal consistency condition, and the two can disagree silently. That is a class of
error the acceptance criterion's second noun exists to catch, and nothing in the panel had exercised
it.

### 3.5 The bound avalanche, and a way through it that is not naming

`13:214-231` reports the avalanche and repairs it by naming the whole derivation as one trait. That
works and it is in this map too, as `HasRung` and `Derived`.

The wide-rung stride needed `16 * ceil(bytes / 16)`, and writing the scaling as repeated **addition**
made the avalanche much worse rather than merely wide: each doubling is an `Add` bound whose subject
is the previous doubling's projection, so the bounds nest and rustc's own suggestion ran to a
2400-character single line.

The repair is not a better name. **On a little-endian binary nat, doubling is prepending an even
digit**, and the canonical constructor is total over every nat, so scaling by 16 costs four `MkE`
bounds that always hold and **zero** `Add` bounds. The arithmetic that caused the avalanche was
avoidable rather than nameable.

That is the microkernelling shape applied to the type level rather than to codegen: the expensive
thing was one operation the solver could not see through, and removing it was cheaper than working
around it.

### 3.6 Erasure survives all three inputs

`q12_erasure_asm.rs` puts a multiply-accumulate through the map at `Hot`, at `Cold` and at `Signed`,
against the bare primitives, `-O`, aarch64:

```
--- q12_arvo_hot:    ALIASED to _q12_arvo_cold
--- q12_arvo_cold (3 instructions)     madd w8, w1, w0, w2 / and w0, w8, #0xffff / ret
--- q12_arvo_signed (3 instructions)   madd w8, w1, w0, w2 / sxth w0, w8 / ret
--- q12_native_u16:  ALIASED to _q12_arvo_cold
--- q12_native_i16:  ALIASED to _q12_arvo_signed
```

The assembler equated the symbols, so the numeral through a three-input map is not merely equivalent
to the native primitive, it is the same code. And the sign axis reaches the right instruction:
`sxth` against `and #0xffff`. This is an **ad-hoc quick spike with no substance** as a measurement
and is named that; it establishes an existence claim about codegen and prices nothing.

### 3.7 What the strategy semantics are is NOT settled here, deliberately

The probe instantiates the map with one assignment, stated in its header as an assumption:

> Hot native rungs to 128 bits then a byte buffer at align 16; Warm and Precise the same at align 1;
> Cold as Warm for the standalone container and stride exactly `W` bits.

`SETTLED.md:85` says Warm's crossover at 65 bits is wrong with a precise reason, and `143b:10-12`
says everything varies granularly with the profile. Neither is discharged here and neither is
contradicted. What the probe establishes is that **the assignment is replaceable without touching a
line of the mechanism**, which is the property that makes it safe to leave open.

## 4. What the (W, F) keying costs, and an attack on it that works

The mechanism is free. The **diagnostic** is not, and the panel should not be handed the coordinate
change without this attached.

Under `(W, F)` keying the consumer's integer width is not a stored parameter at all. So `13`'s
arrangement B repair, making the width type be the literal, does not straightforwardly apply: the
stored width is `16` where the consumer wrote `13`. `13:279-300` already reports arrangement A's
message as the worst in its set, and this makes it worse by one number.

`15_probes/q10_diag_tag.rs` puts three arms side by side, all under `(W, F)` keying, and the file is
**expected to fail**: the failures are the result.

**Arm A, bare structural widths.** What `13` describes:

```
expected `Numeral<E<E<E<E<O<Z>>>>>, O<O<Z>>>`, found `Numeral<E<E<E<E<O<Z>>>>>, E<E<O<Z>>>>`
```

**Arm B, plus a const-generic pair carried for nothing but the message.** `Tag<const I: u32, const
F: u32>`, never read, never computed, propagated where a shape is preserved and dropped to `Anon`
where one is derived:

```
= note: expected struct `b::Numeral<_, O<O<Z>>, Tag<13, 3>>`
           found struct `b::Numeral<_, E<E<O<Z>>>, Tag<12, 4>>`
```

**`Tag<13, 3>` against `Tag<12, 4>`. The consumer's own four numbers, in a plain E0308, with no
annotation anywhere.**

`13:290-292` says the mismatch "is not repairable by annotation", and that is exactly right and
exactly why the repair is not an annotation. `#[diagnostic::on_unimplemented]` does not reach E0308.
What reaches E0308 is **the type**, so the numbers go into the type where the printer will find
them.

**Arm C** routes the equality through an annotated trait so the headline is controllable. It works
and the body is noisier than arm B's, so arm B is the better move: cheaper, no trait, cleaner
message.

### 4.1 The tag's one defect, found by attacking it

A type parameter is part of type identity however unused it is.
`15_probes/q11_tag_costs.rs` checks four things and the first fails, by design:

```
error[E0308]: mismatched types
65 | fn t1(p: Prod<Money, Money>) -> Squared {
   |                                 ------- expected `Numeral<..., Tag<26, 6>>` because of return type
66 |     p
   |     ^ expected `Tag<26, 6>`, found `Anon`
```

A computed product has the same `W` and `F` as the alias a consumer would write and **is not the same
type**. The other three checks pass: a tag-blind `SameShape` accepts the computed product, the tag
survives a shape-preserving operation, and the tag costs nothing at runtime (`size_of Val<Money> = 2`,
same as `u16`, and the two arithmetic functions produce identical output).

Note that even the failure is legible: `expected Tag<26, 6>, found Anon` tells the consumer what
happened in their own numbers.

### 4.2 Arrangement D dissolves it rather than tolerating it

`13`'s arrangement D has the consumer declare a product's output width, checked by a type-level
comparison, with no reverse table. If the consumer is already naming the output, then **the tag on
the output is the consumer's own numbers supplied at the same site**, and there is nothing to retag.
The defect and the arrangement cancel.

`15_probes/q14_declared_plus_tag.rs`, `(W,F)` keying plus the tag plus declared outputs:

```
=== adequate declaration            exit 0
=== too-narrow declaration
error[E0277]: the declared output numeral is narrower than the product needs
90 |     Money: MulInto<Money, TooNarrow>,
   |     ^^^^^^^^ widen the declared output, or state the rounding explicitly
   = note: a product occupies the sum of the total widths and the sum of the fraction widths;
           the `Tag<I, F>` in the note below carries the widths as the consumer wrote them
note: required for `Numeral<E<E<E<E<O<Z>>>>>, O<O<Z>>, Tag<13, 3>>` to implement `MulInto<...>`
```

Headline from the annotation, and `Tag<13, 3>` in the required-for line. The bridge for that whole
program is **six rows**, and they are exactly the six numbers written in consumer position:

```
lits!(0 => N0, 3 => N3, 6 => N6, 13 => N13, 26 => N26, 27 => N27);
```

Nothing computed appears in it, which is the bounded-domain property `11`, `12` and `13` reached
three different ways, surviving the coordinate change.

**So the composition that comes out of tonight is four things stacked**, and none of them was
available to any single file of the second stretch: `(W, F)` keying for the negative-width corner,
`13`'s arrangement D for the ceiling, the tag for the diagnostic, and a three-input map with two
outputs for the strategy and sign axes. I present that as a composition to look at, not as a
conclusion. `04:35-37` forbids treating even convergence as closure, and this is not even
convergence: it is one member's arrangement of four other members' results.

## 5. The wide rung and alignment: not a third gap, and it has a bench in it

The brief asks whether the wide rung and alignment are a third gap or fall out of the map. **They
fall out**, and they fall out only because the map has two outputs. With one output they are
invisible, which is section 3.4's second defect.

Concretely, from `q09` and `q13`, both at `-O` on this machine:

```
UFixed<200,40,U,Warm>   container size=30   align=1    stride=240 bits
UFixed<200,40,U,Hot>    container size=32   align=16   stride=256 bits
UFixed<200,40,U,Cold>   container size=30   align=1    stride=240 bits
```

`Hot`'s wide arm pads a 30-byte payload to 32 and a 25-byte payload to 32, so at `W = 200` it costs
**seven bytes per element**, which at the million-element scale `arvo-toolbox-not-policer.md`
describes is 7 MB. Whether an SSE2/NEON-aligned baseline is worth that is a **measurement**, and by
`01:96-98` it is not a question for op:

> At some point, somebody has to be confident enough about their take on it to write the benches, and
> once benches exist, it's hard to deny what they tell.

No bench harness run bears on it. It is **unpriced**, and the right next act is an arm rather than a
ruling.

One thing about it is a design question rather than a measurement, and it is small: `Hot`'s alignment
choice is currently a property of the strategy applied at the wide rung only. Whether alignment is a
fifth axis, or a consequence of the strategy as it is modelled here, is not decided by anything I
built. I am naming it and not resolving it.

## 6. Where I agree with the second stretch, where I do not, and where I read `14` differently

`RULES.md:99-101` makes keeping something a result, so this is not padding.

### 6.1 Kept, with my own reasoning behind it

**`13`'s width-to-rung ladder.** I built the three-input map on top of it unchanged and it did not
need to move. The rung is a fact about the hardware's shapes and it is right that the strategy and
the sign consume it rather than perturb it. Making it strategy-aware would have been churn.

**The bounded table domain.** `q14`'s bridge is six rows and they are the six numbers written in
consumer position; `q15`'s is six and the seventh is deliberately absent. Under `(W, F)` keying and
the tight product rule, the operation algebra still never consults it: `q15`'s `c1` multiplies four
times from a 16-bit numeral to a 256-bit one, with the table's last row at 40, and exits 0. That is a
fourth instance of a claim `11`, `12` and `13` reached three ways.

**`13`'s "cross once, at literals, in one direction".** Everything I built obeys it without my having
aimed at it, which is the sort of confirmation a rule wants. It survived a coordinate change it was
not written for.

**`12`'s section 7 defect.** Reproduced under a different arrangement (`q15` `c2`): the alias
*definition* with an undeclared width is accepted in silence and the error lands at the first use. So
is `10:290-300`'s: rustc dumps the bridge sorted lexicographically, and in a six-row table you can
already see it, `L<0> L<13> L<16> L<3> L<40> L<5>`. Two previously-reported defects, third
independent instance each.

### 6.2 Where I go further than `12` and `13`

`13` presents A, B and D as three arrangements differing only in what the machinery receives and what
the consumer reads. **That table has a fourth column nobody filled in**: what happens when the
operation's answer has negative integer width. Under `(I, F)` keying every one of the three has to
clamp or refuse. Under `(W, F)` all three carry it.

And `13:290-292`'s "not repairable by annotation" is right and is not the end of it. The repair is a
type parameter, not an annotation, because E0308 prints types.

### 6.3 Where I read `14` differently, and it matters

`14` section 1.2 calls the width-is-a-natural collision "load-bearing rather than a detail" and lists
three possibilities: it refutes the arrangements, it costs one more ladder, or it dissolves under a
sign parameter. `14` marks that as **guessing**, correctly.

**It is a fourth thing.** It does not refute them, it does not cost a ladder, and it does not need a
sign parameter. It dissolves under a change of which two of three coordinates are stored, which is
cheaper than any of the three and was not on the list. `14`'s instinct to open it first was right and
its enumeration of what opening it would find was wrong in the direction of expense.

`14` also says "a signed structural nat is a different ladder, and `13`'s ceiling-division,
comparison and container selection are all defined over the natural encoding". Both halves are true
and neither is needed, because nothing signed enters.

### 6.4 The route I did not take, and why, so the next expert does not repeat the reasoning

**The signed structural integer.** Not built. It is trivially buildable (a sign marker over the
existing magnitude) and the cost is not in building it, it is downstream: signed addition and
comparison, a negative branch at every site that consumes `I`, and a container map whose input
`W = I + F` is a signed sum that has to be proven natural before the rung ladder can see it. That is
a large amount of machinery for a corner that the coordinate change removes, so I stopped rather than
price it. **If the meet turns out to have a caller** (section 1.3) this route comes back, because the
meet is where `W` itself goes negative, and at that point somebody should build it rather than
reason about it.

## 7. What appears to hold, in the register the night allows

Nothing here settles. `04:35-37` binds and I am not offering a convergence either.

**This appears to hold.** The negative-integer-width corner is carried by the encodings the second
stretch built, at zero cost in the mechanism, when the numeral is keyed on total width and fraction
width rather than integer width and fraction width. Compiled: `q06`, whole matrix both rules, with a
negative control that refuses.

**This appears to hold.** The container map works with all three inputs live, over its whole domain
in this probe, with 24 rung impls, 2 sign impls, 4 stride impls and zero width-keyed impls. Compiled:
`q07`, `q08`, 1608 triples, two negative controls that refuse.

**This appears to hold, and it is the part I would most want a second read on.** The map has two
outputs rather than one, and the second is what `Cold` is. Compiled: `q13`, and the two defects in
section 3.4 are what convinced me, because both are invisible to a one-output map.

**This appears to hold.** The digit-tower diagnostic is repairable by a const-generic pair carried
for nothing else, at zero runtime cost, and its one defect is cancelled by `13`'s arrangement D.
Compiled: `q10`, `q11`, `q14`.

**This route is closed and here is the diagnostic.** Computing the total width in a const argument.
That is arithmetic in const-argument position and needs a forbidden feature. It is closed and it does
not matter, because the addition belongs in the nat algebra where it is ordinary trait resolution.
`a-refused-bound-wants-a-trait-not-a-feature.md` names exactly this move and it is the third time in
this panel it has been the answer.

**Unpriced**, and I use the word as `RULES.md:119-122` reserves it: what `Cold`'s packed access costs
against a padded one; what `Hot`'s align-16 wide arm buys for the seven bytes per element it costs at
`W = 200`; what a container-rung crossing costs, which is the currency both tight forms are measured
in; the compile time of any of this. No bench harness run bears on any of them. The assembly reads in
`q12` are an **ad-hoc quick spike with no substance** as measurements and establish an existence
claim about codegen only.

## 8. What I did not cover

Bounded honestly, per `RULES.md:103-104`.

I did not read `CANON_CANDIDATE.md`, `DROPLIST.md`, `02_carried`, `seed/`, `05`, `07`, `08`, or the
closed predecessor panel. I read `RULES.md`, `01`, `04`, `SETTLED.md` and `14` in full; `06` in the
sections its claim lives in plus the surrounding argument; `03` only at the passages a citation sent
me to; `10`, `11`, `12` and `13` at their section headings, their arrangement descriptions, and the
passages I cite, not in full.

**I did not run any arithmetic through the values.** `13:493-494` says this of itself and it is still
true of me: `q12` puts a multiply-accumulate through the container and reads the assembly, which is
one function on one width, not an arithmetic implementation. Nobody has built `Add` on numerals whose
shapes differ.

**I did not build `Precise` as anything but `Warm` with a different name.** The record does not say
what `Precise` does to a container and I did not invent it. If `Precise` widens intermediates rather
than containers then it is not a container-map input at all and belongs on a different axis, which is
a question I am raising rather than answering.

**I did not test the strategy axis against `notko`'s profile attribute.** `SETTLED.md:87` records that
arvo and notko concepts do not correspond, which suggests the interaction is a real question, and I
did not open it.

**I did not test the sign axis on the arithmetic.** The map reaches `i8..i128` and `q12` shows `sxth`,
which is the container being right. Whether the shape rules are right for signed numerals is
untouched: `06:613` names signed numerals as the first thing that would make the negative-width
corner larger, and my whole box is unsigned.

**I did not measure compile time as anything but a wall-clock number off one `rustc` invocation**, and
those numbers (20.4s for `q06`, 2.55s for `q08`) are in the file only so a reader knows the matrices
are cheap to rerun. They are not benchmarks and nothing rests on them.

**The `(W, F)` change's consumer-visible consequences are not worked out.** A consumer writing
`UFixed<13, 3>` gets a type whose stored parameters are 16 and 3. The tag hides that from the
diagnostic. What it does to a `Display` impl, to a `const` constructor, to a serialisation format, or
to anything that reflects on the type, is untouched.

## 9. What is op's, and it is one thing rather than a menu

`14` section 5.4 refuses `13`'s five-option menu going to op under "what is yours", and it is right,
so I am not adding to it.

The one question that is genuinely his, that nothing measurable decides and that the experts have not
converged on:

> **Does a consumer write the integer width, or the total width?**

Everything in section 1 says the machinery wants `(W, F)`. The consumer surface can keep `(I, F)`
through a door that adds them, which is what I built and it costs nothing at the mechanism level. But
it means the numbers the consumer types and the numbers the type stores are different numbers, and
every reflective surface (a diagnostic, a `Display`, a wire format, a macro that reads the type) has
to choose which pair it shows. The tag makes the diagnostic show the consumer's pair. Nothing decides
the rest.

That is a **taste and identity** question about what an arvo numeral IS, not a mechanism question,
which is why it is his. It is also narrower than the five-way pick, and it is prior to the family
question in `01` section 1, because the answer changes what "the shape space" is a space of.

I am **not** asking him to rule on the strategy semantics, the alignment choice, or `Cold`'s payoff.
Those are benches, and `01:96-98` is explicit that a measurement is answered by someone building the
arm rather than by him.

## 10. Every probe, and one command that runs them

`15_probes/verify.sh` reruns everything and reports each against its expected outcome. Output
committed as `15_probes/out_verify.txt`. Last line:

```
unexpected outcomes: 0
```

| probe | what it checks | outcome |
|---|---|---|
| `q01_negative_width_recount.py` | `06`'s negative-width count, three instruments | 15 of 6561, reproduced |
| `q02_wf_coordinates.py` | do the shape rules survive `(I,F)` to `(W,F)` | product 0 wrong, sum 289 wrong |
| `q03_tight_addition.py` | is there a tight addition rule in naturals | 0 of 6561 wrong |
| `q03b_reconcile_461_vs_476.py` | my 476 against `06`'s 461 | 160+301+15, both right |
| `q04_core.rs` | nat, canonical subtraction, max, min | compiles, 0 features |
| `q05_subtraction_matrix.rs` | subtraction over every pair `b <= a <= 64` | 2145 assertions, exit 0 |
| `q06_shape_matrix.rs` | both shape rules over the whole box | 6400 + 6561, exit 0 |
| `q06_negctl.rs` | is that matrix vacuous | refused |
| `q07_three_input_map.rs` | `(strategy, width, sign)` to container and stride | compiles, 0 features |
| `q08_map_matrix.rs` | the map over its whole domain here | 1608 triples, exit 0 |
| `q08_negctl.rs` | `Cold`'s stride asserted as padded | refused |
| `q08_negctl2.rs` | `Hot`'s wide stride asserted unpadded | refused |
| `q09_door_and_erasure.rs` | the door computes `W = I + F` at the type level | exit 0, sizes read |
| `q10_diag_tag.rs` | three diagnostic arms side by side | all three refuse, messages read |
| `q11_tag_costs.rs` | what the tag breaks | `t1` refuses, `t2`-`t4` hold |
| `q12_erasure_asm.rs` | does the three-input map erase | symbols aliased |
| `q13_cold_packed.rs` | is the second output enough to build the aggregate | exit 0, strides read |
| `q14_declared_plus_tag.rs` | arrangement D plus the tag | adequate ok, narrow refused |
| `q15_ceiling_and_alias_site.rs` | ceiling under the tight rule; the alias site | `c1` exit 0, `c2` refused |
| `gen.py`, `verify.sh` | the generators and the runner | committed |

Every `.rs` carries its build command in its header. Every `.out` is the committed output of that
command. `grep -c '#![feature' *.rs` is 0 for all twenty-one files and the check is inside
`verify.sh` so it cannot rot.

## 11. Two things for whoever runs the panel

**`MORNING.md` will need the strategy axis in it.** `14` section 1.1 grepped it and found zero hits
across 468 lines. That is still true of the document as it stands and the four files it summarises,
and it is no longer true of the panel.

**One number in this file will be quoted and should carry its convention.** My product-overshoot count
is 476 and `06`'s is 461, and section 1.5 shows they are the same measurement under two conventions.
If either travels without the other, the panel acquires a second 81-versus-zero, and it does not need
one.
