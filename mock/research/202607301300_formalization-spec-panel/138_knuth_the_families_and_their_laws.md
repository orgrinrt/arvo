# The families and their laws

**Date:** 2026-08-07
**Position:** after `137_aaltonen_erasure_without_a_condition.md`. Written against the standing base
`110_consolidation_eleven.md`, the law list `135_lamport_the_law_list.md`, the sub-table laws
`136_willsey_the_laws_under_the_laws.md`, and `130` section 10, which is the only place the design has more
than one numeral family interpreting one contract.

**Status: complete.** Written in pieces and saved as each section closed, per the dispatch.

Twenty-three laws in three families: seven on the exponent form, eight on the float family, eight on the
decimal family. Six of the twenty-three are stated nowhere in the design at all, and three of those six are
the sentences the conformance claim rests on.

The short version. **One contract does cover the families with the container derived**, and it covers more of
them than anyone has said, because the four families are four points in a product of four axes rather than
four kinds. The container is not a member of the contract at all, and `130`'s section 10 put it there, which
is the thing op refused hours later. **The exponent form's laws are seven and one of them removes a declared
axis**: the stored width is derivable from the numeral's own coordinates at every radix, by a single
expression, checked over the whole matrix of eleven shipped formats. And **the conformance claim at
`110:2379-2383` is right in substance and wrong in its reason**, resting on a theorem nobody stated and
crediting the wrong mechanism for the part that is strictly stronger.

## Contents

1. The premise check, including one claim of the brief's that does not survive
2. Does one contract cover the families, with the container derived
3. The exponent form's laws
4. The float family's laws
5. The decimal family's laws
6. The conformance claim at `110:2379-2383`, checked
7. What the count becomes
8. Attacking the premise, and what is op's

## 1. The premise check

Six of the brief's factual claims, checked before reasoning from them. Four hold, one holds with a shifted
citation, and one is an inherited miscount that changes the shape of the work.

**The toolchain claim holds.** `rustc +nightly-2026-05-28 --version` reports
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, and a bare `rustc` in the scratch tree reports
`rustc 1.94.0 (4a4ef493e 2026-03-02)`, which is stable. Both halves of the warning are real.

**`130` section 10 is one expert's first read and asks for a second.** `130:705-706`: "I am the first read on
this section and it is the part of the file I would most want a second one on, because it touches D53 and the
identity contract's shape rather than only the surface." Confirmed.

**`131` flagged it and did not redo it.** `131:866-868`: "`130:669-707` compiles float and decimal against
one contract with the container written. I did not redo that section against the projected container, and the
exponent form's own laws are still open there." Confirmed.

**The container is derived, in the design's own declaration.** `110:3251`: `type Container;` with the inline
comment "derived, never declared as an axis (1.22), and what `Number` holds (1.1)". And `130b:39-48` refuses
both arities `130` offered, "since both name the container, one directly and one folded inside the strategy
marker". Confirmed, and section 2 is what follows from it.

**The conformance claim is where the brief says, with the second half one line lower.** `110:2379-2381` is
the blockquote; "strictly stronger" is at `110:2383`. Immaterial, recorded because section 6 quotes both.

### The one that does not survive, and it halves the work

**The brief says "four numeral families interpreting one contract" and "the four numeral families beyond
fixed point". The second is false and it is inherited from `135`.**

`130:688-690` names the four: `UFixed<13, 3, u16, Warm>`, `IFixed<12, 3, u16, Warm>`,
`FastFloat<24, -126, 127, u32, Warm>`, `Decimal<16, -398, u64, Warm>`. **Two of those four are fixed point.**
So the families beyond fixed point number two, not four, and `135:540-541`'s "The four numeral families beyond
fixed point. Every row in section 3 is a fixed-point or truth-type row" counts the same four twice: once as
the set `130` compiled, and once as a set disjoint from what `135` had already covered.

`135:318-319` makes the same slip in the count's own qualification. That matters for the pricing at
`135:544-546`, which guesses the missing addition is "around fifty-four again rather than around five" on the
grounds that four whole families are missing. Two are missing, they share every operation with the two that
are not, and section 7 gives the number.

**There is also a fifth family the brief's framing has no slot for, and it is the one the conformance claim
is about.** `110:2380` says "a consumer for whom the quantum is part of the number uses a decimal `Implicit`
numeral". That is radix ten with a constant exponent: a decimal fixed-point numeral. It is not `UFixed`, not
`IFixed`, not `FastFloat`, and not `Decimal` as `130` spells it, since `130`'s `Decimal` carries `Ranged`. So
the design's own most consequential sentence about decimals is about a numeral that appears in none of the
four. Section 5 gives it laws under the name `DecFixed`.

## 2. Does one contract cover the families, with the container derived

**Yes, and the reason it is not close is that the families are not kinds.** They are four points in a product
of four axes, and the contract has exactly those four axes.

`110:911-916` declares `Numeral` with four members: `Radix`, `Precision`, `Exponent` (an `ExponentForm`), and
`Domain` (a `SignDomain`). Read the four named families against them:

| Family | `Radix` | `Precision` | `Exponent` | `Domain` |
|---|---|---|---|---|
| `UFixed<I, F>` | 2 | `I + F` | `Implicit<-F, 1, 0>` | `NonNegative` |
| `IFixed<I, F>` | 2 | `1 + I + F` | `Implicit<-F, 1, 0>` | `Symmetric` |
| `FastFloat<P, EMIN, EMAX>` | 2 | `P` | `Ranged<EMIN, EMAX, U, S>` | `Symmetric` |
| `Decimal<P, EMIN, EMAX>` | 10 | `P` | `Ranged<EMIN, EMAX, U, S>` | `Symmetric` |
| `DecFixed<P, E>` | 10 | `P` | `Implicit<E, 1, 0>` | `Symmetric` |

`UFixed` and `IFixed` differ on one axis and it is not the exponent. `FastFloat` differs from `UFixed` on one
axis and it is not the radix. `Decimal` differs from `FastFloat` on one axis and it is only the radix.
**Nothing in the design is a family. The word names a preset over a product**, in exactly the sense
`110:1710`'s own register uses it, and the product has more points than five: three sign domains times two
exponent forms times every radix from two upward is already infinite before precision is written.

That reframing is what makes the rest of this file short. **A law is keyed on the axis it depends on and not
on a family name**, which is `110:1426-1430`'s own key requirement read literally, and it means the laws
below are seven plus eight plus eight rather than four times fifty-four.

### 2.1 The container is not a member, and `130` made it one

`130:681-685` declares:

```rust
pub trait Numeral {
    const RADIX: u32;  const PRECISION: u32;  const EMIN: i32;  const EMAX: i32;  const SIGNED: bool;
    type Exponent: ExponentForm;   // the kind is the type; the value is a read
    type Store: Container;
}
```

`type Store: Container` is a fifth member and it is the container. `110:911-916` has four members and no
container; `110:3246-3252` puts `StoredWidth` and `Container` on `Lowering`, with `Container` marked "derived,
never declared as an axis". So the design already had the split right, and `130`'s section 10 collapsed it,
which is why op's refusal at `130b:39-48` lands on this section as much as on the surface spelling.

`138_probes/a_one_contract.rs` redoes it with the split restored: `Numeral` carries the four members and no
container, `Lowering<N>` derives `StoredWidth` and `Container`, and the consumer writes neither. It compiles
**exit 0, gate-free**, no `#![feature]`, no `-Z` flag, `#![no_std]`, on `rustc 1.98.0-nightly (57d06900f)`,
reusing `137_probes/ladder.rs` unchanged so the container derivation is `137`'s and not a fresh one.

The coordinates read as themselves, which is `130`'s own test and D48's constraint:

```rust
const _: () = assert!(<<B32 as Numeral>::Radix as Nat>::V == 2);
const _: () = assert!(<<B32 as Numeral>::Precision as Nat>::V == 24);
const _: () = assert!(<<Decimal<N16, ENeg383, N384> as Numeral>::Radix as Nat>::V == 10);
const _: () = assert!(<<Money as Numeral>::Radix as Nat>::V == 10);
```

and the container is checked as a **type equality** rather than as an assertion about one, so the file does
not build if a rung is wrong:

```rust
pub fn c_b32(x: <Bin as Lowering<B32>>::Container) -> u32 { x }
pub fn c_b64(x: <Bin as Lowering<B64>>::Container) -> u64 { x }
pub fn c_e4m3(x: <Bin as Lowering<E4M3>>::Container) -> u8 { x }
pub fn c_s123(x: <Bin as Lowering<S12_3>>::Container) -> u32 { x }
```

### 2.2 The probe's own first failure was the finding, and I am reporting it as such

The first version of `a_one_contract.rs` derived the stored width as `sign + precision` for every `Implicit`
numeral and put it on `Numeral`. It compiled for every radix-two family and failed on exactly one line:

```
error[E0308]: mismatched types
   --> a_one_contract.rs:188:59
    |
188 | pub fn c_money(x: <Money as Numeral>::Container) -> u16 { x }
    |                                                     ---   ^ expected `u16`, found `u32`
```

`Money` is `DecFixed<16, -2>`, a decimal numeral of sixteen **digits**, and `sign + precision` counted the
digits as bits. The derivation was keyed on precision and domain and not on radix, so it was correct for the
family it was written against and silently wrong one axis over.

**That is `136`'s pattern at its section 3.3, arriving a third time**: `tag_one_representable(int_bits)`
reading one of two determining parameters, the `AddClosed` gate keying on `Bias = Zero` when the condition is
about `bias / adjustment`, and now a width derivation keyed on precision when the condition is about
`R^P`. `136:386-387` states the rule from two instances: "a predicate over numerals is keyed on every numeral
parameter or it is keyed on the ones that existed when it was written." Three instances is enough to stop
calling it a pattern and start calling it the default failure mode of this design, and section 3's X7 is what
a correctly keyed version looks like.

## 3. The exponent form's laws

Seven rows. Written to `135`'s seven-column standard, which is `110:1426-1430` read as a specification. The
exponent form is the only axis on which `Implicit` and `Ranged` differ, so every law keyed on it is a law
about the form and about nothing else, and that is what makes these seven the ones `130:706` left open.

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| X1 | An `Implicit` numeral's value set is an interval of a rank-one subgroup of the rationals, closed under addition wherever the sum stays in range | pairs of in-range operands | identity | radix, `P`, `E`, `A`, `B` | asserted, compiled: zero escapes over four models, 50,505,000 in-range sums at the largest | `110:1249-1250` |
| X2 | A `Ranged` numeral's value set is a union of intervals of subgroups whose generators form a geometric chain, and that union is not a subgroup | pairs of in-range operands | identity | radix, `P`, `EMIN`, `EMAX`, `Underflow` | asserted, compiled: escapes at every model, 2,716,288 of 3,143,936 in-range sums at radix two `p = 8` | `110:1250-1252` (this row is `135`'s Q7 restated on its real key) |
| X3 | The gap between adjacent values is one constant on `Implicit` and one constant per binade on `Ranged` | consecutive pairs of the value set | identity | the form, radix, `P`, the exponent bounds | derived from X1 and X2, compiled: 1 distinct gap against 3 | nowhere as a law |
| X4 | `Underflow` on an `Implicit` numeral is fixed at `Gradual` rather than absent, and the `Abrupt` reading is a well defined numeral containing no zero | the numerals | identity | the form, `Underflow` | asserted, compiled; **this restates `110:1038-1039`, whose own wording is "nothing for it to mean", and section 8 is why** | `110:1038-1039`, `49:125-128`, corrected in section 8 |
| X5 | The result numeral of `mulnum` carries the exponent sum on `Implicit` and the bound sums on `Ranged` | pairs of numerals | identity | the form, the operand numerals | asserted | `110:970-971` |
| X6 | The exponent form is a complete key for the preset table: two tables, two constructors, and no third | the numerals | identity | the form | asserted, and it is the only place the form is already used as a key | `127b:99-102`, `110:2836` |
| X7 | The stored width is `sign + ceil(log2(R^(P - h) * span))`, where `span = EMAX - EMIN + 1` and `h` is one exactly where a leading digit is hidden | the numerals | identity | radix, `P`, `Domain`, `EMIN`, `EMAX`, hidden-digit | **derived**, compiled over the whole matrix of eleven shipped formats, zero failures | nowhere; `110:3248` declares `StoredWidth` instead of deriving it |

X1 and X2 are the pair `110:1247-1255` calls "the algebraic difference from `Implicit`, stated once because
three separate results follow from it", and until now the sentence had one compiled witness, `1 + 2^-24`
against binary32. `138_probes/d_exponent_form.rs` checks both directions over the whole value set of seven
models rather than at one witness:

```
EF1  Implicit radix=2  p=6 E=-3      : 2080 in-range sums,        0 leaving the value set
EF1  Implicit radix=10 p=3 E=-2      : 500500 in-range sums,      0 leaving the value set
EF1  Implicit radix=2  p=8 E=0       : 32896 in-range sums,       0 leaving the value set
EF1  Implicit radix=10 p=4 E=5       : 50005000 in-range sums,    0 leaving the value set
EF1  Ranged   radix=2  p=4 E in [-3,3]: 3792 in-range sums,    2840 leaving the value set
EF1  Ranged   radix=10 p=2 E in [-1,2]: 130600 in-range sums, 109080 leaving the value set
EF1  Ranged   radix=2  p=8 E in [-6,6]: 3143936 in-range sums, 2716288 leaving the value set
```

Per `86b:6-8`, what the model separates: the models vary the radix (two and ten), the precision, the exponent
range and the form, and the closure verdict tracks the **form alone** across all of them. Two of the seven
models were built while getting this wrong, and both errors are worth one line because they are the shape
`86b` names. Sorting `(numerator, denominator)` pairs lexicographically made the range test compare the
largest numerator rather than the largest value, and a radix-two model at `p = 4` reported zero escapes
because the subgroup was coarse enough for every in-range sum to land back on it. Neither is a fact about the
design; both looked like one until the model was made to separate.

### 3.1 X7, which removes a declared axis

This is the one row here I would defend hardest, because it takes something the design declares and shows it
was always derivable.

`110:3248` puts `type StoredWidth: StoredWidth;` on `Lowering`, with the comment "the carrier level;
`W_F <= W_S`, declared (1.22)". Declared, not derived. The consequence is a written parameter one axis away
from the container `130b` refused, and it is written for the same reason the container was: nobody had the
expression.

The expression is one line and it is radix-general:

> **W_S = sign + ceil(log2( R^(P - h) * span ))**, where `span = EMAX - EMIN + 1` is the count of normal
> binades and `h` is one exactly where a leading digit is hidden, which at radix two is always and above
> radix two is never, since `110:2341-2343` establishes that no radix above two has a constant leading digit
> to hide.

`138_probes/b_width_law.rs` computes it exactly, in integer arithmetic with no floating point anywhere,
over every binary and decimal interchange format IEEE 754-2019 names explicitly, up to 256 bits, plus the
three non-IEEE binary formats the design's own chapters name:

```
format      radix    p     span  sig_exp   pred actual     ok
binary16        2   11       30       10     16     16   true
binary32        2   24      254       23     32     32   true
binary64        2   53     2046       52     64     64   true
binary128       2  113    32766      112    128    128   true
binary256       2  237   524286      236    256    256   true
bfloat16        2    8      254        7     16     16   true
E4M3(OCP)       2    4       15        3      8      8   true
E5M2(OCP)       2    3       30        2      8      8   true
decimal32      10    7      192        7     32     32   true
decimal64      10   16      768       16     64     64   true
decimal128     10   34    12288       34    128    128   true

failures: 0
```

Eleven of eleven, at three radix-ten formats whose encoding is a densely-packed or binary-integer significand
sharing a combination field with the exponent, and at three binary formats that are not IEEE's. Nothing was
chosen and nothing was left out of the range checked: the standard's `binaryK` family continues above 256
bits by a stated rule and the expression is parametric in `P`, `EMIN` and `EMAX`, so nothing about the
formula distinguishes those rows, but they were not run and I am not claiming them.

**The per-axis form fails and the joint form does not, which is the whole content of the row.** Deriving the
significand field and the exponent field separately and adding them is the obvious move, it is what my own
first probe did, and it overshoots:

```
per-field sum form  W = 1 + ceil(P log2 R) + bitlen(span):
  decimal32   sig=  24 exp=  8 sum=  33 actual=  32 OVERSHOOT
  decimal64   sig=  54 exp= 10 sum=  65 actual=  64 OVERSHOOT
  decimal128  sig= 113 exp= 14 sum= 128 actual= 128 ok
  ... every binary format ok
```

Two of eleven, and both are decimal. The overshoot is exactly the bit IEEE's combination field recovers by
encoding the leading significand digit and the top exponent bits jointly, and it disappears when the ceiling
is taken once over the product rather than twice over the factors. **So a derivation checked at the binary
formats alone reports total and is wrong at two of the three decimal ones**, and decimal128 passes under
both, which is why a three-format decimal check that happened to pick decimal128 would also have reported
total. That is the sampled-law failure `strict-by-design-quality-pressure.md` names, sitting inside a
derivation rather than inside a test.

### 3.2 A corollary of X7 that predicts a shipped format's `Specials`

Worth its own paragraph because it was not what I was looking for and it explains a cell of the design's own
float chapter.

The bit length of the exponent span leaves slack, and the slack is what the reserved codes live in:

```
  binary16    span=     30 expbits=  5 codes=      32 slack=2
  binary32    span=    254 expbits=  8 codes=     256 slack=2
  binary64    span=   2046 expbits= 11 codes=    2048 slack=2
  binary128   span=  32766 expbits= 15 codes=   32768 slack=2
  binary256   span= 524286 expbits= 19 codes=  524288 slack=2
  bfloat16    span=    254 expbits=  8 codes=     256 slack=2
  E4M3(OCP)   span=     15 expbits=  4 codes=      16 slack=1
  E5M2(OCP)   span=     30 expbits=  5 codes=      32 slack=2
```

Every IEEE binary interchange format has slack exactly two: one code for zero and the subnormals, one code
for the infinities and the NaNs. **E4M3 has slack one**, so it can afford the zero-and-subnormal code and not
the second one, which is why its infinities do not exist and its NaN lives in the top significand slot
instead. `110:2171-2172` states that as a fact about E4M3 ("its all-ones slot in the top binade is the NaN
encoding instead") and derives the parity consequence from it. The slack count derives the fact.

So X7 has a corollary the float chapter can use: **`Specials` is not free of the exponent bounds.** A numeral
whose span leaves one code cannot carry `IeeeSpecials`, and a numeral whose span leaves none cannot carry
`NanOnly` either. That is a well-formedness condition relating two members of the identity contract that the
contract does not currently relate, it is checkable in a const, and it is one row of section 4.

## 4. The float family's laws

Eight rows. The float family is radix two with a `Ranged` exponent, so X1 through X7 already apply to it and
these eight are what that point adds. Six are harvested from section 1.16, which carries them as prose and
never as laws; one is a split of another; one is new.

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| FL1 | The far point is the supremum of the numeral's ordered representable values, and the three cases the design names are three instances of that one rule | the value set | identity | `Specials`, `Underflow`, the form | asserted, ratified at `74b`, compiled as a total const projection over the whole four-member `Specials` product | `110:2156-2161` |
| FL2 | The far point needs no NaN exclusion clause, because the supremum is over the ordered values and NaN is not in the order | the value set | identity | `Specials` | derived from FL1, and it is why the two no-infinity `Specials` members agree | `110:2161-2163` |
| FL3 | An out-of-range event begins half a top-binade ulp past the maximum finite, with the tie resolved by the ordinary even rule on the extended grid | the exact-value domain | identity | `P`, `EMAX`, radix, resolutions, `Specials` | asserted, compiled at the E4M3 model | `110:2167-2170` |
| FL4 | Where the maximum finite's stored significand is odd, the tie at FL3 rounds off the finite set with no directional constant, which is every IEEE binary interchange format | the top binade | identity | `P`, the format's reserved codes | derived from FL3 and the encoding; it is what makes the standard's own overflow-at-the-tie behaviour fall out rather than be written in | `110:2168-2171` |
| FL5 | Where it is even the identical rule rounds down, and the band above the maximum is ordinary in-range rounding rather than an overflow event at all | the top binade | identity | as FL4 | asserted, compiled at E4M3: the tie at 464 rounds to 448 and `(448, 464]` is in range | `110:2171-2174` |
| FL6 | The far-point kind joins through a fold with silence dominating: the published grade records `Finite` the moment any operand's far point is finite | the two-element kind carrier | identity | `Specials` per operand | asserted, compiled over the whole four join laws in const context, not a sample | `110:2186-2193` |
| FL7 | A come-back sum at a finite far point saturates to zero against a true maximum, a silent full-scale in-range error | triples in the value set | identity | `Specials`, preset, `P`, `EMAX` | asserted, measured at E4M3: `(448 + 448) - 448` delivers 0 against a true 448 | `110:2176-2178` |
| FL8 | At radix two, a numeral's `Specials` is bounded by its exponent bounds: `IeeeSpecials` needs the span to leave two spare codes and `NanOnly` needs one | the numerals | identity | radix, `P`, `EMIN`, `EMAX` | **derived from X7, compiled at eight binary formats**; a well-formedness condition between two members of the identity contract that the contract does not relate | nowhere |

FL4 and FL5 are one relation split on a key parameter, per `136`'s section 5 rule that a row whose verdict
depends on a key value has not been fully instantiated. The parity is a hand computation and I give it so a
reader can check the claim without running anything. E4M3's maximum finite is `1.110` binary times `2^8`,
that is 448, whose stored significand is `110` binary, six, even. The next point on the grid extended upward
is `1.111` times `2^8`, that is 480, and the midpoint is 464. Ties-to-even chooses between a significand of
six and one of seven and takes the even one, so 464 rounds down to 448 and the whole interval `(448, 464]` is
ordinary in-range rounding. For binary32 the maximum finite's significand is twenty-three ones, odd, so the
same rule at the same position rounds the other way and off the finite set. **One rule, opposite verdicts,
and the key parameter is the parity of an all-ones field**, which is a property of how many codes the format
reserved rather than of the arithmetic.

FL8 is new and section 3.2 derives it. It is worth a row rather than a note because the identity contract as
declared lets a consumer write a `Ranged` numeral carrying `IeeeSpecials` whose exponent span leaves no room
for the reserved codes, and nothing refuses it. The condition is a const comparison and the refusal is an
unimplemented bound, so it costs one `#[diagnostic::on_unimplemented]` and no mechanism.

**It is scoped to radix two on purpose.** The slack argument counts the codes a separate exponent field of
`bitlen(span)` bits leaves over, and section 3.1 is the demonstration that a separate exponent field is
exactly what a decimal format does not have. So FL8 holds where the fields are separate and its decimal
analogue has to be derived from the combination field's own encoding, which is an `Encoding::Fields` fact
rather than a numeral one. I have not derived it and I am not implying the row extends.

**What is not here, and deliberately.** The fold's exact accumulator for a `Ranged` numeral is `135`'s F5 and
I am not restating it. The float preset table (`110:2755-2761`) is a ratified table rather than a set of
laws: its cells fix resolutions, and resolutions are a key parameter of the laws above rather than a claim
about a grouping class. And `70b:23-33`'s open cell, what `Warm` and `Cold` do out of range at a numeral
whose `Specials` carries no infinity, is answered by FL1 rather than needing a table row of its own, since
the supremum of a no-infinity numeral's ordered values is its largest finite magnitude. That is op's own
instinct at `70b:29`, derived rather than chosen, which is the strongest position a held cell can reach.

## 5. The decimal family's laws

Eight rows, every one keyed on the radix rather than on a family name, which is what lets three of them state
the radix-two case in the same sentence.

The model throughout is `138_probes/c_decimal.rs`: a datum is a sign, a significand in `[0, 10^p)` and a
quantum exponent `q`; a value is the exact rational `(-1)^sign * s * 10^q`, reduced, so cohort members
compare equal as values while staying distinct as data. The model is validated against the design's own
published measurement before anything is claimed from it.

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| DL1 | A value's cohort has one member per representable exponent shift, and the redundancy is the data count minus the distinct-value count | the datum set | identity | radix, `P`, the exponent bounds, `Encoding` | asserted, compiled: 600 data, 559 distinct values, 41 redundant, reproducing `110:2350-2355` exactly | `110:2343-2345` |
| DL2 | Normalising a decimal numeral changes no value, **provided `Underflow = Gradual`** | the value set | identity | radix, `P`, the exponent bounds, **`Underflow`** | **asserted with a condition the design omits**: true under `Gradual`, false under `Abrupt`, which loses 18 of the 559 values in the design's own model | `110:2354-2355`, and the omission is section 5.1 |
| DL3 | The two natural cohort-selection rules are the same function at radix two with a hidden digit and different functions at every radix above two | the datum set | identity | radix, the hidden digit | asserted, compiled: 28 disagreeing values at radix ten `p = 2`, zero at radix two `p = 4` | `110:2357-2361` |
| DL4 | Non-canonical codes are a third and larger source of non-injectivity than cohorts, and live entirely on `Encoding` | the datum set | identity | `Encoding::Fields`, `P` | asserted, compiled: 209 of 768 data redundant at a seven-bit significand field against 41 of 600 in the tight encoding | `110:2363-2368` |
| DL5 | No arithmetic operation's delivered **value** depends on which cohort member carries an operand | pairs of cohort-equal data | identity | radix, `P`, the exponent bounds, the operation | **derived and compiled**: 1,280,000 comparisons over the whole model, zero divergences. This is what the conformance claim rests on and it is written nowhere | nowhere |
| DL6 | `quantize` and `roundToIntegralExact` are the exceptions to DL5, reading a datum where every other operation reads a value | pairs of cohort-equal data | identity | as DL5 | asserted, and already carved out as datum-dependent by definition | `110:1194-1196`, `110:2219-2222` |
| DL7 | On an `Implicit` numeral the design's result-numeral rule and IEEE's preferred exponent are the same function: the minimum for addition, the sum for multiplication | pairs of numerals | identity | the form, radix, the operand numerals | **derived**, and it is the reason the conformance claim can be made at all | nowhere; `110:970-971` states the design's half and never compares it |
| DL8 | IEEE's preferred exponent is best-effort and degrades silently where the format cannot express it; the design's is achieved at every position, because the result numeral widens rather than the value being reshaped | pairs of in-range operands | identity | `P`, the operand quanta, the operation | **asserted, compiled**: at a decimal32-shaped format 4,468 of 6,400 addition positions and 2,816 of 6,400 multiplication positions clamp | nowhere; this is what `110:2383` should say and does not |

### 5.1 DL2, which the design states without its condition

`110:2354-2355` reads:

> The value sets of the normalised and unnormalised counterfactuals are **identical**: normalising a decimal
> numeral changes no value, which means **cohorts are a choice, not forced by the value set, and the design
> has to know whose choice it is**.

The consequence is right and the premise is conditional. Checked both ways over the design's own model, which
is radix ten at `p = 2` with `e` in `[0, 2]`, the model `110:2350` measures:

```
DC1  p=2, q in [0,2]: data=600 distinct=559 redundant=41  (110:2350-2355 says 600/559)  match=true
DC2  normalised, Underflow = Abrupt   : distinct=541  identical to unnormalised = false
DC2  normalised, Underflow = Gradual  : distinct=559  identical to unnormalised = true
     values the Abrupt counterfactual loses: 18 of them, smallest three [(-9, 1), (-8, 1), (-7, 1)]
```

The lost values are the small integers. Under normalisation the significand's leading digit is nonzero, so at
the bottom exponent row the values one through nine and their negatives have no representation, and they have
none at any higher row either. **The band that restores them is exactly the subnormal band**, which is
`110:1243`'s own "together with the bottom grid extended down to zero when `Underflow = Gradual`". So the
sentence is true of every decimal numeral the design would actually ship, since IEEE decimal has no
flush-to-zero, and it is false as a general statement about the two counterfactuals.

That matters more than an eighteen-value discrepancy suggests, because the sentence is load-bearing twice. It
is what establishes that cohorts are a choice rather than forced, which is what makes `Encoding::Canonical` a
genuine axis at radix ten rather than a formality; and `110:2348` calls the surrounding finding "the design
was built to express this before anyone checked that it did". A premise stated without its condition, inside
the paragraph that congratulates the design for expressing the thing, is the shape `136:76-79` names: a
design states the properties it concludes and assumes the properties it needs, and the second set is where
the unchecked things live.

**The repair is four words.** DL2's key column carries `Underflow` where `110:2354`'s sentence does not.

### 5.2 DL5, which nobody stated and everything rests on

The conformance claim says arvo's decimal `Ranged` numerals deliver IEEE's values while not being conformant
to its preferred-exponent rules. That is only coherent if the preferred exponent never changes a value, and
the design nowhere says so.

It does not, and the check is the whole model rather than a sample of it. For every ordered pair of data in a
radix-ten model at `p = 2` with `q` in `[-1, 2]`, the exact sum and the exact product are compared against
the same operation applied to each operand's canonical cohort member:

```
DC4  exact + and * against the cohort's canonical member: 1280000 comparisons, 0 value divergences
```

Zero, and it could not have been otherwise, which is the point rather than an objection to having checked.
The exact result of `+` and `*` is a function of the operand **values**, and a cohort is by definition a set
of data sharing one value. So DL5 is a one-line theorem, and one-line theorems nobody writes down are how a
claim to the world ends up resting on nothing. **`110:2379-2381` is a conformance statement made against a
published standard, in a document about to be promoted to canon, and its supporting theorem is absent from
the document.**

DL6 is the exception, and the design already found it. `quantize(x, y)` reads `y`'s quantum, so two
cohort-equal `y` values give results of different value:

```
DC4  quantize: y1=(1, 0) and y2=(10, -1) are cohort-equal (values (1, 1) vs (1, 1)),
     and quantize(x, y) delivers quantum 10^0 against 10^-1: datum-dependent
```

`110:1194-1196` carves `quantize` and `roundToIntegralExact` out as "datum-dependent by definition", and
`110:2219-2222` gives `quantize` a target numeral `At<N, Q>` whose exponent is fixed at the requested
quantum, which is the right shape and expresses IEEE's one **required** (rather than preferred) exponent rule
exactly. **So the design's carve-out list is not an admission of a gap. It is precisely the complement of
DL5**, and stating DL5 turns a list of two into a theorem with two named exceptions, which is the difference
between a design that noticed something and a design that knows why.

## 6. The conformance claim at `110:2379-2383`, checked

The design's two sentences, quoted whole because the verdict turns on their wording:

> **arvo's decimal `Ranged` numerals deliver IEEE's values and are not conformant to its preferred-exponent
> rules; a consumer for whom the quantum is part of the number uses a decimal `Implicit` numeral, where the
> exponent is a type, checked at compile time, and cannot drift through an arithmetic chain.**
> (`110:2379-2381`)

> That is **strictly stronger** than the standard's own rule, because it is checked rather than propagated at
> runtime, and it is unavailable to a language with only runtime decimals. (`110:2383`)

**Verdict: the first sentence is correct and now has the theorem it needs. The second sentence's conclusion is
correct and its stated reason is wrong, in a way that understates the design.** Three separate findings, and
the third is a claim I would delete.

### 6.1 "Deliver IEEE's values": correct, and DL5 is why

An operation's delivered value in IEEE decimal is fixed by the operand values, the format and the rounding
attribute. The preferred exponent selects which member of the result's cohort carries that value, and DL5
establishes that the selection is invisible to every later operation's value. So a value-valued pipeline
delivers IEEE's values by construction, and the pipeline being value-valued is not a limitation of arvo but
the reason the claim is available to it.

The non-conformance is real and it is **observable**, which the design does not say and should. Two cohort
members compare equal under `compareQuietEqual` and are distinguished by `quantum`, `sameQuantum`,
`totalOrder`, `isCanonical` and by conversion to a decimal character sequence, which the standard requires to
preserve the exponent. Reproduced on a real conformant decimal implementation, CPython's `decimal`:

```
  Decimal('1') == Decimal('1.0') : True
  str(): 1 vs 1.0   as_tuple: (sign=0, digits=(1,), exponent=0) vs (sign=0, digits=(1,0), exponent=-1)
  same_quantum: False
```

**So the honest scope is: a consumer who never formats a result and never asks for its quantum cannot observe
the non-conformance at all; one who formats can, on the first line of output.** That sentence is worth more
to a reader than "not conformant", because it says when it bites, and it is what turns a bare admission into
a statement a consumer can act on.

### 6.2 "Strictly stronger": the conclusion holds, on a reason the design does not give

The stated reason is "because it is checked rather than propagated at runtime". **That is a claim about
binding time, and binding time is not strength.** Computing the same function earlier is earlier, not
stronger. If the design's rule and IEEE's were the same function, moving the check to compile time would buy
diagnostics and buy nothing about the result.

They are not the same function, and DL7 plus DL8 say exactly how they differ.

**DL7: on the positions where IEEE achieves its preference, the two rules agree.** IEEE's preferred exponent
is the minimum of the operand quanta for addition and their sum for multiplication (IEEE 754-2019 clause 5.2,
secondary read, primary owed per the review's standing practice). The design's result-numeral rule for an
`Implicit` numeral is the finer grid for addition, which at one radix is the minimum, and the exponent sum
for multiplication, which `110:970-971` states as `mulnum`'s own arithmetic. Same function, arrived at from
opposite directions: IEEE chose it as a convention, the design computes it as the lattice of the exact
result. That agreement is the reason a comparison is meaningful at all, and it is written nowhere.

**DL8: IEEE's preference is best-effort and the design's is not.** The standard delivers the achievable
exponent closest to the preferred one, so where the format cannot express the preference it silently
delivers something else. The design achieves the preferred exponent at every position, and the two operations
achieve it for different reasons, which is a distinction I got wrong on the first pass and which changes the
verdict's shape.

**Multiplication widens.** `110:1476-1479` establishes that multiplication needs `mul_full`, so the result
numeral is the exact product's numeral and both the preferred exponent and the exact value are delivered.
Against IEEE, which is stuck in the operand's format, this is strictly stronger with nothing given up.

**Addition does not widen, and does not need to.** `110:1470-1475` gives additive lattice closure for a
numeral whose `bias / adjustment` is an integer, which every unbiased `Implicit` numeral is, so the exact sum
is already on the finer of the two grids and the preferred exponent is the grid it is on. What can fail is
the **range**, and a range event is the `Resolution` axis's own domain with a named constant per preset.
IEEE, at the same input, keeps the value in range by moving the exponent and dropping low digits, with no
range event raised.

**So on addition the two are different rather than ordered, and which is stronger is a preset question.**
The design preserves the quantum and reports the range event; IEEE preserves significance and says nothing
about the quantum. Under `Precise` the design refuses where IEEE silently degrades, which is strictly more
informative. Under `Hot` it reduces modulo, which is worse than degrading. That is a real qualification on
"strictly stronger" and the canon should carry it rather than let a reader infer the multiplication case
covers both.

Measured over an exhaustive grid of operand quanta and significands at two format shapes:

```
DC5  decimal32-shaped: p=7
     add: preferred exponent achievable in-format 1932 of 6400, clamped 4468
     mul: preferred exponent achievable in-format 3584 of 6400, clamped 2816
DC5  decimal64-shaped: p=16
     add: preferred exponent achievable in-format 5802 of 15625, clamped 9823
     mul: preferred exponent achievable in-format 8750 of 15625, clamped 6875
```

and the worked case, reproduced against CPython's `decimal` so it is not my arithmetic being checked against
itself:

```
runtime decimal, preferred exponent achieved when the context is wide enough:
  prec=7    1.000000 * 1.000000 = 1.000000         exponent=-6
  prec=14   1.000000 * 1.000000 = 1.000000000000   exponent=-12
```

At `p = 7` the preferred exponent is -12, expressing the value 1 at that exponent needs thirteen significand
digits, seven are available, and the delivered exponent is -6. The preference degraded and nothing said so.
The design's `DecFixed<7, -6>` times `DecFixed<7, -6>` is `DecFixed<14, -12>`, exact, and the exponent is in
the type.

**So "strictly stronger" is the right phrase for multiplication and an overstatement for addition**, in the
technical sense that the design's rule agrees with IEEE's wherever IEEE achieves its preference, achieves it
in strictly more positions, and on addition pays for that with a range event where IEEE pays with silent
digit loss. Both halves are checkable statements about two functions. "Checked rather than propagated at
runtime" is not a statement about either function, and replacing it costs two sentences.

### 6.3 "Unavailable to a language with only runtime decimals": false, and it should be deleted

The clause overstates by naming the wrong thing as unavailable. A runtime decimal library with a wide enough
context achieves the preferred exponent at exactly the positions the design does, which is what the `prec=14`
row above shows: CPython's `decimal` delivers exponent -12 for the case a fixed `p = 7` format cannot.

What is genuinely unavailable at runtime is the **check**: nothing in a runtime decimal refuses a program in
which the quantum drifts, and nothing tells the consumer at authoring time what quantum a chain will produce.
That is a real and defensible claim about the design, and it is not the claim the sentence makes.

**My suggestion, and it is a suggestion.** Replace `110:2383` with the two facts that are checkable:

> That is stronger than the standard's own rule rather than a concession to it. The two rules coincide
> wherever the standard achieves its preferred exponent. Where a fixed format cannot, the design achieves it
> anyway: a product's result numeral widens, and a sum is already on the grid the preference names, so what a
> fixed format resolves by moving the exponent and losing low digits the design resolves as a range event
> with a named constant. And the quantum is visible in the type before the program runs, which no runtime
> decimal offers, though a runtime decimal with a wide enough context reaches the same exponents.

### 6.4 One arithmetic defect in `130`'s own decimal instantiation

`130:690` writes `Decimal<16, -398, u64, Warm>`. Under `110:1241-1243` the exponent bound `EMIN` is a
**binade** exponent and the bottom grid's quantum is `radix^(EMIN - p + 1)`. decimal64's bottom quantum is
`10^-398`, so its `EMIN` under the design's own convention is `-398 + 16 - 1 = -383`. Reading `-398` as
`EMIN` gives a bottom quantum of `10^-413`, fifteen decades below decimal64's:

```
EF4  110:1241-1243 makes the bottom quantum radix^(EMIN - p + 1).
     decimal64's bottom quantum is 10^-398, so EMIN = -398 + 16 - 1 = -383.
     130:690 writes Decimal<16, -398, u64, Warm>. Read as EMIN that is a
     bottom quantum of 10^-413, which is 15 decades below decimal64's.
```

`110:2393` uses `-398` correctly, as a quantum exponent, in prose ("decimal64's own bottom grid (exponent
-398)"). `130` moved the same number into a type parameter whose convention is the other one. **A `Ranged`
exponent form also needs two bounds and `130`'s decimal instantiation supplies one**, so whichever convention
was meant, the instantiation is incomplete. Both are why section 2 redoes the section rather than checking it.

This is the class of error `136:622-625` predicts: the claims most likely to be wrong in a section that
establishes a mechanism are the ones translating the mechanism back into the vocabulary it replaced, because
they are written last and checked least, and are the only part a reader who knew the old vocabulary reads.

## 7. What the count becomes

`135` counted fifty-four and called it a floor, and `135:544-546` declined to guess what the families beyond
fixed point would add, beyond "the ceiling is around fifty-four again rather than around five". **The answer
is twenty-two**, and the reason it is not fifty-four is section 2's.

| Source | Rows | Of which stated nowhere |
|---|---|---|
| `135`, fixed point and truth types | 54 | 0 (all harvested) |
| `136`'s M7 split, per its section 5 | +2 | 0 |
| `136`, mechanism and sub-table laws | +19 by its tables, +15 by its own prose | most |
| This file, the exponent form (X1, X3 to X7) | +6 | 2 |
| This file, the float family (FL1 to FL8) | +8 | 1 |
| This file, the decimal family (DL1 to DL8) | +8 | 3 |

X2 is not counted as an addition: it is `135`'s Q7 restated on its real key, which now names the exponent form
rather than being filed under the quantiser. That re-key is the only change this file makes to an existing row.

**One row of that table is a discrepancy rather than a number.** `136`'s section 2.1 carries V1 to V5, its
3.1 carries I1 to I7 and its 4.1 carries O1 to O7, which is nineteen; `136:616` calls them "these fifteen
rows". I have not resolved it and I am not guessing which is meant, because the difference is four rows in a
count op is going to price.

**Why twenty-two and not two hundred.** Because the operations are shared and the operation laws are already
keyed on numerals rather than on families. Addition's A1 and A2, multiplication's M1 through M8, the
quantiser's Q1 through Q7, division's D1 through D3 and the elementary functions' E1 through E7 all quantify
over "a numeral" and hold at every point of the product, with the radix, the form and the exponent bounds
already sitting in their key columns. Nothing about them is fixed-point-specific. **What the two families
beyond fixed point add is the laws of the axes that distinguish them**, and there are two such axes, so the
addition is linear in axes rather than multiplicative in families.

`135:544-546`'s guess was high by a factor of about two and a half, and the reason is worth carrying: it
guessed from the family count, and the family count is not a dimension of this design.

**The count is still a floor.** `135`'s section 5 lists eight operations carrying no law and this file adds
none of them. The `Implicit` numeral's `Adjustment` and `Bias` members carry laws nobody has written, and
`136:397-401` records that neither name exists in the tree at all. And section 8's identification, if op
takes it, removes rows rather than adding them.

## 8. Attacking the premise, and what is op's

### The premise this brief takes for granted, and there are two

**"One contract, four families."** Section 2 attacks the second half and section 1 the count. Neither is the
sharp one. The sharp one is underneath both:

**Is the exponent form two constructors, or one with a degenerate case?**

`110:918-921` declares `Implicit<E, A, B>` and `Ranged<EMIN, EMAX, U, S>` as two constructors of a sealed
`ExponentForm`, and every law in section 3 is keyed on which. But `110:1241-1245`'s own definition of a
`Ranged` value set, read at `EMIN = EMAX`, produces the normal band `[radix^E, radix^(E+1))` at quantum
`radix^(E-p+1)` **plus the bottom grid extended down to zero under `Underflow = Gradual`**, and those two
bands together are every multiple of `radix^(E-p+1)` from zero up, with `p` digits. That is an `Implicit`
value set exactly.

Checked over a matrix of seven (radix, precision) pairs spanning radices two, three, seven and ten and
precisions two through eight, at every exponent from -4 to 4, sixty-three points in all:

```
EF5  is Implicit<K> the same value set as Ranged<E, E, Gradual> with K = E - p + 1?
     identical at every (radix, p, E) in the matrix: true
```

**So `Implicit<K, 1, 0>` and `Ranged<K + p - 1, K + p - 1, Gradual, S>` denote the same value set, always.**
The two constructors are not disjoint kinds; one is the single-binade case of the other.

I do not think that collapses the axis, and I want to say why rather than assert it, because the reasons are
what the design should record if it keeps two constructors.

The two carry **different nested members**. `Implicit` nests `Adjustment` and `Bias`; `Ranged` nests
`Underflow` and `Specials`. The identification above holds at `A = 1, B = 0`, and a biased or rescaled
`Implicit` numeral has no `Ranged` spelling at all. So the constructors are distinguishable, and what is
identified is a slice of each.

And the design's stated reason for the nesting is subtly wrong. `110:1038-1039` says the nested shape stands
on "`Underflow` has no bottom to fall off under a constant exponent, so there is nothing for it to mean at
every other numeral", restored as "the only support that shape has". Under the identification there is
something for it to mean: the single-binade `Abrupt` case is a perfectly well defined numeral, and it is the
normal band alone, with no zero in it:

```
     radix=2 p=4 Ranged<0,0,Abrupt>: |set|=8, contains zero: false
     radix=10 p=2 Ranged<0,0,Abrupt>: |set|=90, contains zero: false
```

**A numeral with no zero is not meaningless, it is useless**, and those are different arguments with
different consequences. "Nothing for it to mean" says the axis is undefined on `Implicit`; "one setting is
the only one anyone wants" says the axis is defined and fixed. The second is true and is the stronger
support, because it says what an implementation should do when someone writes the other one: refuse it, with
a diagnostic naming the missing zero, rather than have no opinion.

So X4's status cell should read that `Underflow` is **fixed at `Gradual`** on `Implicit` rather than absent,
and the nesting argument at `110:1038-1039` should be restated on the useless-not-meaningless ground. That is
a correction to the only support a ratified shape has, which is why it is in this section and not in
section 3.

**The second premise, and I inherited it before I checked it.** The brief says the exponent form's laws are
"what distinguishes the families". Section 2's table says otherwise: the form distinguishes two of the four
named families from the other two, and the radix distinguishes one pair from the other, and the sign domain
distinguishes `UFixed` from `IFixed`. Three axes do the distinguishing and the form is one of them. Writing
the form's laws and calling the job done would have left the radix's laws and the domain's laws unwritten,
and the radix's laws are section 5, which is where the conformance claim lives. **The brief pointed at one of
three axes and the most consequential sentence in the stretch is on a different one.**

### Where I am the first read

Everything in sections 3, 4, 5 and 6 is a first read and none of it should enter the canon on one expert's
word. The three I would most want attacked:

**X7**, because it removes a member from a declared contract on the strength of eleven data points, and
eleven is every format anyone has shipped but is not every numeral arvo will accept. A second read should
try to build a numeral the expression gets wrong, starting with a radix that is neither two nor ten and a
precision where `ceil(P log2 R)` sits close to an integer, which is where a joint ceiling and a per-field one
stop differing by exactly the bit that made the decimal rows work.

**DL5**, because it is a one-line theorem and one-line theorems are where a hidden quantifier hides. The
check covers `+` and `*` over an exhaustive model; it does not cover division, `sqrt`, `fma` or the
conversions, and I asserted rather than checked that the same argument covers them.

**The identification in this section**, because it is a statement about a ratified declaration's shape and I
reached it from the value sets alone. A second read should ask whether anything other than the value set
distinguishes the two constructors at `A = 1, B = 0`, since if nothing does, the axis is a naming convenience
and the design should say so on purpose.

### What is op's

Six, one line each, and I am touching none of them.

1. **Whether `StoredWidth` stops being a declared member of `Lowering` and becomes a derivation** (X7).
   `110:3248` declares it; section 3.1 derives it at every shipped format. It removes a written parameter from
   a ratified declaration, which is the same kind of move as `130b`'s container refusal and is therefore his.
2. **Whether `110:2383`'s reason is replaced** by DL7 and DL8 (section 6.2), and whether the
   runtime-decimals clause is deleted (section 6.3). It is a claim the canon makes to the world about a
   published standard, so the wording is not a panel call.
3. **Whether FL8 becomes a checked bound or a documented condition.** Refusing a numeral at declaration is
   what `110:2195-2197` declined for a neighbouring case on the warn-never-police line, and this one is
   cheaper to refuse than that one was. His either way.
4. **Whether `Underflow` on `Implicit` is recorded as fixed at `Gradual` rather than absent**, and whether
   `110:1038-1039`'s nesting argument is restated on the ground section 8 gives. It is the only support a
   ratified shape has, so replacing it is his.
5. **Whether the canon declares a decimal `Implicit` family at all.** `110:2380` names the numeral in the
   sentence that carries the conformance claim, and nothing anywhere declares it. Section 5's laws are
   written for it under a name I invented for this file.
6. **Which token the exponent form's constant constructor carries.** `110:918` writes `Implicit`; `130:701`
   writes `Constant`. One line, and it is his namespace.

Two things are corrections rather than calls and should land without a decision. `110:2354-2355` gains its
`Underflow = Gradual` condition (section 5.1), and `130:690`'s decimal instantiation is wrong in its exponent
convention and incomplete in its arity (section 6.4).

### Standing

This file is agent output on the suspect rung. Every row cites where it is stated or says plainly that it is
stated nowhere. The compiled claims are reproducible from `138_probes/`: `a_one_contract.rs` is `#![no_std]`,
gate-free, `--crate-type=lib --emit=metadata`, exit 0, and includes `137_probes/ladder.rs` unchanged so the
container derivation is `137`'s rather than a fresh one; `b_width_law.rs`, `c_decimal.rs` and
`d_exponent_form.rs` are ordinary binaries computing in exact integer arithmetic, and I say so rather than
implying the arithmetic checks are also under the design's constraints. All four build and run under
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`.

`130`'s section 10 cites five probe sources by name (`h1_four_interpretations.rs`, `e1_capstone.rs`,
`a3_surface.rs`, `f1_arity3.rs`, `e2_scale.rs`) and **none of them exists anywhere in the repository**, nor
does a `130_probes/` directory. So section 10's "compiled, gate-free" is a claim I could not check and had to
redo rather than verify. That is worth recording as a fact about the archive rather than about the expert:
a panel that cites its evidence by filename owes the file.

One thing should be carried forward without further reads. **The conformance claim's supporting theorem is
DL5, it is one line, and it is absent from a document about to be promoted to canon.** That does not need a
second opinion. It needs writing down.
