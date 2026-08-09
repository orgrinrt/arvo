# The type-level float, and what radix ten costs the crossing contract

Oleg Kiselyov, file 54. I wrote file 02 (the type-level encoding) and file 36 (the normal form and its
price). Neither gets inheritance here. File 42 found that file 36's seal demonstration lived in a file
nothing composed with, and section 5 below overturns a second file-36 statement, in its own words, with
a compile: I wrote that the exponent is "never arithmetic in a way the wall blocks" (`36:446-447`) and
that is false.

**What I read.** `49_consolidation_four.md` in full, per the standing instruction that it is the
self-contained base, then the four deliverables since it (`50_fog_the_float_model.md`,
`51_fallin_the_last_tick_and_the_licence.md`, `52_ringer_the_tests_that_were_owed.md`,
`53_torvalds_does_it_still_earn_its_keep.md`) and the stand-in checkpoint
`53b_persona_checkpoint_twelve.md`, then `ls` of the panel directory. Behind those I opened only what my
own artifacts compose with or correct: `50_probes/vu_nat_sealed_adj.rs` and `vu_bias_sealed_adj.rs` (the
sealed tower, copied unmodified into `54_probes/` exactly as files 46, 50, 52 and 53 copied it),
`50_probes/probe_3_exponent_as_type.rs` (the exponent machinery I extend rather than reinvent),
`30_probes/probe_3_sign_domain_against_sign_indexing.rs` (the crossing contract's original witness),
`53_probes/price/gen.py` (the cost-sweep shape I reuse), and my own file 36, to quote rather than
remember it. I did not reason from `49:117`; file 50 named it a defect and section 5 finds the same
defect one line up, at `49:116`, which nobody has flagged.

**Gates.** Test gate, run fresh from `mock/`: `cargo test --workspace`, summed per binary rather than
read off a headline, 122 binaries, **654 passed, 0 failed, 9 ignored**, matching every file since 41.
Canon gate: the surface this file designs has no shipped source. The consolidation's own corrected
command, `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` from the repo root, exits
1 empty, as does the `FullRange\|UTerm\|AddWidth` variant. Nothing here critiques arvo's code, because
there is none to critique; the whole subject is design.

**Compiled or measured, against reasoned.** Eight probe files, three shared modules and three cost sweeps in `54_probes/`, every
one built on `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`. I verified the
pin resolves from the probe directory before running anything (`rustc --version` and
`rustc +nightly-2026-05-28 --version` agree from `54_probes/`), because file 52 paid for that lesson and
I decline to pay for it twice. Every compile-time figure is `rustc --edition 2021 --crate-type lib
--emit=metadata`, trait-solve-only, no codegen, min-of-3, the identical build shape files 36, 41, 42 and
53 used. **I ran no timing loop about runtime anywhere**; the one runtime claim this stretch owes stays
owed to `mock/benches/` exactly where file 50 left it. Sections 2 through 6 are compiled or measured and
say which at the point of use. Sections 4.4, 7 and 8 contain reasoning, marked as such.

## 1. What was built, in one paragraph

The float model and the numeral tower are the two halves of one object and nobody had joined them. The
join is now compiled: a `Specials`-carrying, radix-parameterised numeral whose every axis is a type, with
four new carriers each sealed and attacked at declaration time, and the crossing contract run against it
exhaustively at model widths with every parameter of the check read off the numeral type rather than
written by hand. Then radix ten, where the contract's third statement stops being a formality. The
headline results, ordered by how much they change:

- **The `Specials` axis is a product of two facts, not a chain of three**, and the corner file 50's chain
  cannot name is the one with a shipping witness (OCP OFP8's `E4M3`, NaN and no infinity). File 53's
  provenance demand is answered by moving the axis, not by grounding a rung `unknown`.
- **Radix ten is not what makes decimal non-injective.** The radix is the first link of a four-link
  chain and the other three are already named by the design's own axes. Section 4 states the chain.
- **The crossing contract's "derived boolean" is now derived**, as a closed form over the axes, checked
  against exhaustive enumeration in both directions and over the whole `Specials` product rather than a
  sample.
- **`Implicit`'s single exponent must be a type too**, which closes file 50's honest carve-out, corrects
  `49:116` alongside `49:117`, and overturns a statement of my own from file 36.
- **The `Radix` axis pays for itself in compile cost, and the margin is not a percentage.** Absorbing a
  decimal grid's quantum into the rational adjustment, which is the only alternative to a separate radix
  axis, stops compiling at exponent -39 and every real decimal format is past it. Measured, with the
  refusal.
- **A `Pos` has a depth ceiling near 128 constructors**, on every axis, and the design has never stated
  it. That is the general fact the decimal result is one instance of.

## 2. The four carriers, born sealed and born attacked

The carrier-at-birth rule (`49:74-87`) is the review's own measured comparison: four passes to close the
tower's seal against one pass for `Grade`, and one pass for file 50's exponent. This file mints four more
carriers and runs the two-obligation checklist at declaration time on each. The positive control is
`54_probes/probe_1_carriers_born_sealed.rs`; the negative controls are three separate crates against the
tower compiled as an rlib, one file per introduction route, because a compile-fail fixture sharing a
crate with a compiling one reports only its first error.

**Compiled.** All four carriers refuse all four routes:

| route | attack | result |
|---|---|---|
| direct impl on a local type | `impl Specials for ForgedSpecials`, and the same for `Underflow`, `SignDomain`, `Radix` | `E0277` on the private supertrait, all four |
| the supertrait itself | `impl vu54::numeral::specials_sealed::SpecialsSealed for A` | `E0603`, module is private |
| downstream blanket | `impl<T: MyMarker> Specials for T` | `E0210`, uncovered type parameter |
| re-impl on an inhabitant | not repeated; refused by the orphan rule before any seal is consulted, per file 46 | inherited |

The diagnostic is worth noting because it is better than the review has had: rustc's own message says
"`Specials` is a *sealed trait*, because to implement it you also need to implement
`vu54::numeral::specials_sealed::SpecialsSealed`, which is not accessible" and then lists the four
permitted types. The seal explains itself to a consumer at the point of refusal, which is the diagnostic
posture the toolbox rule asks for and which nobody had checked was available.

### 2.1 `Radix` is a carrier, and the ratified table's spelling admits two broken instances

`49:110` spells it `type Radix: Radix;` with the comment "2 and 10 instantiated; any r expressible". The
trait is open. An open `Radix` with a `const R: u64` admits `R = 1` and `R = 0`, and both compile clean
while falsifying the statement the entire float model rests on: file 50's own design sentence is that a
`Ranged` numeral denotes "the union, over `e` in `[EMIN, EMAX]`, of the grids with quantum
`radix^(e - p + 1)`" (`50:40-43`). At `R = 1` every grid in that union is the same grid, so the exponent
carries no information and the value set is one grid wearing a family's clothes. At `R = 0` the quantum
is zero.

So `Radix` carries a proof-relevant property (`R >= 2`) that its own declaration cannot state, which is
exactly the condition that makes something a carrier under `what-you-can-observe-is-what-you-guaranteed`.
But a finite constructor seal is the wrong instrument, because the design genuinely wants every radix
expressible.

**The shape that serves both, compiled**, and it is the shape `Bias` already uses:

```rust
pub trait AtLeastTwo: Pos + radix_sealed::AtLeastTwoSealed {}
impl<P: Pos> AtLeastTwo for O<P> {}   // O<P> = 2P >= 2
impl<P: Pos> AtLeastTwo for I<P> {}   // I<P> = 2P+1 >= 3
// H = 1 has no impl.

pub struct Rad<P>(PhantomData<P>);    // the ONLY Radix constructor
impl<P: AtLeastTwo> Radix for Rad<P> { type Digits = P; const R: u64 = P::VAL; }
```

One constructor family over the sealed `Pos`, so the inhabitant set is infinite but generated, and a
two-impl predicate on the constructor head carrying the well-formedness. Radix zero has no `Pos` spelling
at all; radix one is `Rad<H>`, a well-formed type with no `Radix` impl, refused with
`the trait bound H: AtLeastTwo is not satisfied` at the bound rather than at some later arithmetic that
produced a wrong answer (`probe_1b`). Radix three, sixteen and one hundred all instantiate in the
positive control by naming a `Pos`, with no new inhabitant of any sealed trait, which is the
"observation, not inhabitation" clause of the seal's own quantification (`49:394-399`) applied here.

The predicate is exhaustive by construction rather than by enumeration, which is worth stating because
file 52 was careful about exactly this distinction (`52:317-322`): `Pos ::= H | O<P> | I<P>` has three
constructors, `AtLeastTwo` covers two of them, and the third is the excluded value. There is no fourth
constructor for an attack to hide in, because `Pos` is sealed.

*grounded on: `pin`, `vu`, `seal-owed` discharged; the `R >= 2` requirement grounded on `50:40-43`.*

### 2.2 `Specials` is a product, and the corner with a witness is the one the chain omits

File 50 proposes three instances as a chain: none, infinities-only, IEEE (`50:506-510`), justifying the
middle with "a saturating format with an infinity but no NaN is a real shape". File 53 called that
plausible and uncited and demanded a witness or an `unknown` ground (`53:236-243`). Both are right about
the middle rung and both miss that the axis is the wrong shape.

Infinity presence and NaN presence are **independent** in shipping formats. The OCP 8-bit Floating Point
Specification's `E4M3` represents **no infinities** and reserves NaN only, spending the freed exponent
code to raise `emax` from 7 to 8 and gain a binade of dynamic range; its sibling `E5M2` follows IEEE and
carries both. That is a deployed format, in silicon, and the three-instance chain cannot name it.

So `Specials` is the four-point product `{INF} x {NAN}`:

| instance | INF | NAN | witness |
|---|---|---|---|
| `NoSpecials` | no | no | every fixed-point numeral; every integer type |
| `NanOnly` | no | yes | OCP OFP8 `E4M3`, and its `FNUZ` variant |
| `InfOnly` | yes | no | **none found. grounded `unknown`.** |
| `IeeeSpecials` | yes | yes | binary32, binary64, decimal64, OFP8 `E5M2` |

File 53's demand is honoured for the corner that still lacks a witness, and it is a different corner than
the one he was asking about. I searched for a format with infinity and no NaN and did not find one; the
row costs nothing to declare, `ExactWindow`'s gate wants the distinction, and it carries `unknown` rather
than a plausible sentence.

Two consequences fall out of the product shape that the chain shape hides, and section 3 compiles both.
Adding infinities adds two **values** and changes no statement of the crossing contract, because the two
infinities are distinct values with one datum each. Adding NaN adds one value and as many data as the
encoding reserves, so it is the NaN corner alone that can move the injectivity boolean. Under the chain
reading, where infinities-only sits between none and IEEE, those two look like the same kind of step.
They are not.

**Signalling NaN stays out of the axis**, following file 50: reading one is an operation and the design's
grade already carries what an operation raises. The quiet-versus-signalling split is `Encoding::Fields`
reserved-code content.

*grounded on: `pin` for the compile; the E4M3 and E5M2 facts on the OCP OFP8 specification as reported by
secondary sources this dispatch read rather than on the specification document itself, and a member with
the PDF should confirm the `emax` figure specifically.*

### 2.3 `Underflow` loses flush-to-zero, and gains a cross-axis constraint nobody has stated

File 50 section 5.2 moves flush-to-zero out of `Numeral` into a `Quantisation` resolution, on the ground
that it changes no representable set. Adopted here and compiled: `Underflow` is `Gradual | Abrupt`,
sealed, two instances.

What running the crossing contract adds is a dependency between the two coordinate systems that the
design has treated as independent. `Abrupt` says values below the smallest normal are not representable.
Under a **normalised** encoding that is free: the subnormal row's data simply become non-data, and
`decode` was already partial there. Under an **unnormalised** encoding (section 4, every radix above two)
there is no subnormal row to remove: the significand walks down to zero at the bottom exponent by
construction. Realising `Abrupt` there means declaring a contiguous region of otherwise-total datum space
to be reserved, which is expressible but is a real obligation on `Encoding::Fields` that the numeral
axis, on its own, does not carry.

Stated as the spec should carry it: **`Underflow = Abrupt` is a `Numeral` fact whose realisation
constrains `Encoding`, and it is the first axis found that does.** That is not a defect in the split;
it is the crossing contract doing its job, which is to be the one place both coordinate systems are in
scope at once.

*compiled for the normalised case (`probe_2` section E, `live_data` drops and both statements hold);
reasoned for the unnormalised case, which I did not model.*

## 3. The crossing contract, run against the numeral, radix two

`54_probes/crossing.rs` is the contract as a `const fn` model over the abstract datum space, and
`probe_2_crossing_binary.rs` drives it from numeral types: every value-side parameter is read off
`<N as Numeral>` and only the datum-side parameters (normalisation, the NaN datum count, whether the
negative-zero datum exists, which cohort member `encode` selects) are supplied at the crossing site. That
separation is the design's own coordinate split made operational, and it is what made section 4's result
findable.

The three statements are unchanged (`49:161-168`). What is new is that statement 3 now has a derivation.

### 3.1 The derived boolean, derived

`49:167` says injectivity is "a derived boolean, not an assumption". It has been asserted in that form
since file 31 and never written down. Here it is, and it is four clauses because there are exactly four
ways a value acquires a second datum in this design's axes:

> The encoding is injective iff no value has two data. A second datum arises from, and only from: a
> signed zero the encoding does not repurpose; more than one reserved NaN datum; a cohort, meaning an
> unnormalised significand over more than one exponent with room to shift; and a cohort of zeros, which
> an unnormalised encoding has even at precision one. The two infinity data are two distinct values and
> are never a source.

**Compiled, whole matrix rather than sampled.** The predicate is checked against exhaustive enumeration
of the datum space for every configuration in probes 2 and 3: both truth values, both radices, all four
`Specials` corners, both underflow modes, both cohort rules, signed and unsigned, with and without the
repurposed negative zero. It agrees everywhere. It is the kind of claim a suite can assert over the
subset where it holds, so it is asserted over the whole matrix and the agreement is the assertion, not
the truth value.

### 3.2 What the `Specials` product does to the contract, measured

Model numeral: radix 2, p = 3, e in [-2, 3], gradual underflow, symmetric domain. One numeral type, four
`Specials` corners, everything else fixed.

| corner | live data | distinct values | statement 1 | statement 2 | statement 3 |
|---|---|---|---|---|---|
| `NoSpecials` | 56 | 55 | holds | holds | false |
| `InfOnly` | 58 | 57 | holds | holds | false |
| `NanOnly`, 2 NaN data | 58 | 56 | holds | holds | false |
| `IeeeSpecials`, 2 NaN data | 60 | 58 | holds | holds | false |
| `IeeeSpecials`, 4 NaN data | 62 | 58 | holds | holds | false |

Statements 1 and 2 are invariant across the entire product. Statement 3 is false in every row here for a
reason that has nothing to do with `Specials`: the signed zero, which is one datum wide and which file 30
already found in 2026 with a sign-magnitude fixed-point witness (`30_probes/probe_3`, lines 244-254).

The infinity column is the check that makes the product shape pay: `distinct_values(InfOnly) ==
distinct_values(NoSpecials) + 2` and `distinct_values(NanOnly) == distinct_values(NoSpecials) + 1`, both
asserted. Infinities add values one-for-one with data; NaN adds one value for as many data as are
reserved. **`Specials::INF` cannot touch the crossing contract and `Specials::NAN` always can.** That
sentence is not derivable from a three-rung chain and is immediate from the product.

### 3.3 Statement 3 is not vacuously false either, and an encoding choice can restore it

Two configurations where injectivity **holds**, so the boolean is genuinely two-valued rather than a
polite way of saying no:

- Unsigned, no specials, normalised: every value has exactly one datum, `live_data == distinct_values`,
  statement 3 true and predicted true.
- **`E4M3FNUZ`**, the OFP8 variant that repurposes the negative-zero datum as its NaN. That removes the
  review's oldest non-injectivity witness by construction, and with a single NaN datum the encoding is
  injective while still carrying a special. Compiled at the real shape (p = 4, e in [-6, 8]): 512 data,
  256 live, 256 values, statement 3 true.

The `FNUZ` case is worth carrying into the spec because it is the design's own coordinate split
vindicated by a shipping format: `Numeral` is unchanged between `E4M3` and `E4M3FNUZ` (same radix, same
precision, same exponent range, same value set except for the removal of a datum that never denoted a
distinct value anyway), and every difference is `Encoding::Canonical`. A design that put signed zero on
the numeral would have to call these two different number types.

## 4. Radix ten, and where the standard and the design genuinely part company

This is the part the brief said to do carefully, and it repaid the care in a direction I did not expect:
the interesting result is not that decimal is non-injective, it is **why**, and the why makes the radix
almost incidental.

### 4.1 The chain, of which only the first link mentions the radix

> radix > 2 → there is no constant leading digit to hide → the significand is stored unnormalised → a
> value has one datum per representable exponent shift → the encoding is not injective.

Radix two can normalise for free: a normalised binary significand's leading digit is always 1, so it need
not be stored, and the hidden-bit trick both enforces normalisation and costs nothing. There is no
constant leading digit in radix ten, so enforcing normalisation means storing a leading digit and
refusing nine tenths of... more precisely, refusing the `[0, r^(p-1))` band of every normal row, which is
one tenth of the significand space per row. IEEE decimal declines to pay it, and the result is cohorts.

Only the first arrow is about the radix. The other three are about the significand's storage, and the
design's axes already name every link: `Numeral::Radix`, `Encoding::Fields` (the hidden digit is
explicitly listed there at `49:146`), `Encoding::Canonical` (the preferred cohort is explicitly listed
there at `49:147`). **Nothing new is needed, and that is the finding.** The design was built to express
this before anyone checked that it did.

### 4.2 The measurements

Decimal `Ranged` numeral, radix 10, p = 2, e in [0, 2], so quantum exponents q in [-1, 1]. Exhaustive
over the datum space, `probe_3_crossing_decimal.rs`:

| quantity | decimal, unnormalised | the same numeral IF normalised |
|---|---|---|
| data | 600 | 800 |
| live data | 600 | 560 |
| distinct values | **559** | **559** |
| statement 1 | holds | holds |
| statement 2 | holds | holds |
| statement 3 | false, predicted false | false, predicted false |

Three things in that table.

**The value sets are identical.** Normalising a decimal numeral changes no value. So cohorts are not
forced on decimal by its value set, which means they are a choice, which means the design has to know
whose choice it is (section 4.4).

**Every datum is live in the unnormalised encoding.** `decode` is total on the finite region: there is no
reserved significand band, which is the other side of the trade and is why decimal encodings waste
nothing despite the redundancy. The normalised counterfactual has 800 data of which 240 are dead.

**The collapse is proportional to the format and lands on ordinary finite values.** 41 out of 600 at
p = 2; 401 out of 6000 at p = 3 with the same exponent range. Compare the binary model with no specials:
one datum, the negative zero, which is a special case everyone already knew about. That is the sense in
which the injectivity statement has been exercised vacuously for twenty-four files: it had exactly one
witness, and the witness was zero.

The cohort census, measured at runtime over p = 3, q in [-2, 2] (`54_probes/report.rs`, an
`-O` build, no timing, counts only):

| cohort size | values with that cohort size |
|---|---|
| 1 | 8460 |
| 2 | 684 |
| 3 | 54 |
| **10** | **1** |

The one value with a ten-member cohort is **zero**: five exponent rows times two signs, every one of them
a spelling of zero. Zero's cohort is the largest in any decimal format and it is the whole exponent range.
That is worth stating in the spec because a reader who thinks of cohorts as a trailing-zeros phenomenon
will not expect the extremal case to be the value with no digits at all.

### 4.3 The section becomes a real choice, and radix ten is where it becomes visible

The crossing contract is a section-retraction pair (file 30's own framing, `30:154`): `decode` retracts,
`encode` sections. The **value identity** is the strip normal form and is not a choice. Which
representable datum `encode` picks is the section, and it is `Encoding::Canonical`'s content.

Under radix two with a hidden digit, the two obvious cohort rules (smallest significand with the largest
exponent, largest significand with the smallest exponent) are **the same function**, asserted in probe 2
section F. There is no choice to make, so `Canonical` looks like a formality.

Under radix ten they are different functions on the same value set, asserted in probe 3 section C with a
named witness rather than a count: the value 1 is spelled `1 x 10^0` and `10 x 10^-1`; the two rules pick
different data; both decode back to 1. **So `Encoding::Canonical` carries real content, the design owes a
choice there, and neither choice is derivable from the numeral.** That is a positive result about the
design: the slot exists, it is in the right place, and radix ten is what shows it is load-bearing.

### 4.4 Two encodings for one format, and the one thing the design cannot represent

*Reasoned, with one half compiled.*

IEEE's decimal formats ship two interchange encodings, BID and DPD. In the design's vocabulary that is two
`Encoding` instances under one `Numeral`, which is what the three-way cut predicts. Splitting it further:

**Repacking the significand digits is a bijection on the datum space**, and a bijection commutes with
`decode` and `encode`, so it cannot change any of the three statements. That is a theorem rather than a
measurement and it says the interesting difference between BID and DPD is not the packing.

**What is interesting is the non-canonical codes.** A binary significand field wide enough to hold
`10^p - 1` also holds codes above it, and the standard's rule is that they read as zero. Compiled at
p = 2 with a seven-bit significand field (`probe_3` section F): 768 data carrying the same 559 values,
against 600 data in the tight encoding. **209 of the 768 are redundant, against 41 in the tight
encoding.** The non-canonical codes are a third non-injectivity, five times larger than the cohorts, and
they live entirely on the `Encoding` side. Statements 1 and 2 hold for it, against the identical numeral,
and the value set it produces is the identical 559.

Now the part where the design and the standard part company, and I state it as sharply as I can because I
think it is the single most important sentence in this file.

IEEE 754 specifies, per operation, a **preferred exponent** for decimal results: which member of the
result's cohort the operation delivers, as a function of the operation and its operands' exponents rather
than of the result's value. A decimal consumer can observe it (`quantize`, `sameQuantum`, and the whole
ledger-arithmetic use case that decimal exists for). **The design's operations are value-valued**: an
operation produces a value, and `encode` then chooses a datum by a rule keyed on the value. There is no
place in that pipeline for the operation to choose the datum, and `Canonical` cannot express the rule,
because `Canonical` is a function of the value and the preferred exponent is not.

Three responses, and I recommend the third:

1. **Carry the cohort member in the value coordinates.** Then a decimal value is a pair, not a rational,
   which falsifies the design's founding sentence (`49:99-101`) and breaks the total order the whole
   algebra is stated over. Reject.
2. **Make operations datum-valued for decimal.** Then laws may not read operations, since a law's key is
   a `const fn` parameter list and `Lowering` is not a parameter (`49:151-154`). The entire algebra
   evaporates for one radix. Reject.
3. **Accept it, and say where the quantum belongs.** arvo's decimal `Ranged` numerals deliver IEEE's
   **values** and are not conformant to its preferred-exponent rules. A consumer for whom the quantum is
   part of the number uses a decimal **`Implicit`** numeral, where the exponent is a type-level constant,
   so the quantum is in the type, checked at compile time, and cannot drift through an arithmetic chain.

Response 3 is not a concession, and this is the part worth arguing rather than conceding. IEEE's
preferred-exponent machinery is a **runtime** mechanism for propagating a scale that the programmer knew
statically and had no way to write down. The design has a way to write it down. A ledger quantity is
`Fx<Ten, P, ENeg<P2>, ...>`, its quantum is a type, and `mulnum` computes the result quantum by adding
exponents at compile time (probe 4, section 5). That is strictly stronger than the standard's rule,
because it is checked rather than propagated, and it is unavailable to a language that has only runtime
decimals.

Compiled in support: `probe_3` section E shows a decimal numeral with a single exponent row has **no
cohort at all**, its only collapse is the signed zero, and dropping to a non-negative domain makes it
**injective**, statement 3 true for a radix-ten numeral. So the claim "decimal is never injective" is
false, and the design's own answer to the quantum question is exactly the configuration where it is
false.

What the spec owes, honestly, is the sentence naming the deviation: **the design represents every IEEE
decimal format's value set, both of its interchange encodings, and its arithmetic results as values; it
does not represent clause 5.2's preferred exponents, because those select a datum as a function of an
operation, and this design's operations produce values.** The standard-representation test the design
sets itself deserves that stated rather than discovered later.

*The clause-5.2 characterisation is read from secondary sources in this dispatch, not from the standard
document. File 39 checked clause 7 against the text directly; a member with the standard should do the
same here before this sentence hardens, and I flag it rather than let it inherit file 39's rigour.*

## 5. `Implicit`'s exponent, and a defect at `49:116` that sits beside the known one at `49:117`

File 50 closed the exponent-as-type fork for `Ranged` and was explicit about what it did not close: "it
is not met on whether the `Implicit` numeral's single exponent should move to a type at the same time,
which I did not test" (`50:602-604`). File 53 then took `49:117` as a settled defect and did not look one
line up.

`49:116` reads `pub struct Implicit<const E: Exponent, A: Adjustment, B: Bias>;`. It has the same defect,
for the same reason, and the reason is not analogy.

**Compiled, both halves.** `probe_4_implicit_exponent_as_type.rs` builds `mulnum` over two `Implicit`
numerals. Three quantities are computed and all three appear in the result numeral's type: the precision
sum, **the exponent sum**, and the adjustment product. `Fx<Two, P8, ENeg<P4>, 1/4, ...>` times
`Fx<Two, P4, ENeg<P2>, 1/8, ...>` gives precision 12, exponent -6, adjustment 1/32, every one asserted
through a const assertion and the whole thing forced through a function signature rather than left in an
inert alias. A three-step chain (-4, -2, -2 summing to -8) confirms the chain is not one deep.

`probe_4b_implicit_exponent_as_const_refused.rs` is the negative control, and it walks the same doors
file 50's `probe_3b` walked, for the smaller shape:

```
bare:                    error: generic parameters may not be used in const operations
                         help: add `#![feature(generic_const_exprs)]`          (forbidden)
min_generic_const_args:  error: complex const arguments must be placed inside of a `const` block
as a const block:        error: generic parameters may not be used in const operations
                         help: add `#![feature(generic_const_args)]`           (needs -Znext-solver=globally)
```

The shape being smaller matters, and it is why this needed its own compile rather than an inference from
file 50's: a reader could reasonably expect a single const to survive where a pair did not, since the
arithmetic is one addition rather than two. It does not. The wall is about a generic parameter appearing
in a const operation at all.

**This overturns a statement of mine.** File 36 flagged the exponent as the one member it had not traced
and wrote: "whether it should be a type at all, given that `E` is signed and small and never arithmetic
in a way the wall blocks, is a real fork I did not open" (`36:444-447`). Two of those three clauses are
right. `E` is signed and small. It is not "never arithmetic in a way the wall blocks": `E1 + E2` is
exactly the arithmetic the wall blocks, and it sits in the exact-widening family's own result type. I did
not open the fork and I was wrong about which way it would fall if anyone did.

**What the spine rule reaches, restated to include this.** The rule (`49:59-72`) says a quantity computed
and then appearing in a type is a type. Applied across the numeral, the complete list of members that are
therefore types rather than consts, with the map that computes each:

| member | computed by | appears in |
|---|---|---|
| `Precision` | `mulnum`, precision adds | result numeral's type |
| `Ranged`'s `EMIN`, `EMAX` | `mulnum`, bounds add | result numeral's type |
| `Implicit`'s `E` | `mulnum`, exponents add | result numeral's type |
| `Adjustment` | `mulnum`, gcd of products | result numeral's type |
| `Bias` | `mulnum`, `B1*B2` | result numeral's type |
| `Radix` | nothing computes it; it is declared and read | nowhere |

`Radix` is the only member of a numeral that a map never computes, which is why it is a tag with a
readable const rather than an arithmetic carrier, and why section 6's cost result is about the other
members and not about it.

## 6. The price, measured, and a ceiling the design has never stated

The brief said to price radix ten rather than assume it, because powers of ten are not dyadic and file 53
found a cliff at many distinct non-dyadic rationals (`53:162-171`). It is priced, and the answer has two
halves that point in opposite directions.

### 6.1 Decimal spans the cost curve, and file 53's two coefficients reproduce independently

`54_probes/price/bias_gen.py` and `bias_sweep.sh`, reusing file 53's generator shape and forcing
discipline (every composition asserted against a Python-computed reduced `Fraction`, so nothing is an
inert alias), min-of-3, `--emit=metadata`, sealed tower.

| profile | magnitudes | marginal cost per distinct composition |
|---|---|---|
| dyadic (file 53's control, reproduced) | `2^a / 2^b`, a and b to 15 | **2.3 ms** |
| decimal quanta | `(1/10^a) * (1/10^b)`, a and b to 6 | **6.8 ms** |
| decimal quanta, wider | a and b to 9 | **14.5 ms** |
| decimal slope and bias | two-digit numerators, denominators to `10^5` | **21.0 ms** |
| decimal, wide both sides | numerators to `10^6`, denominators to `10^6` | **78.9 ms** |
| 16-bit random rational pairs (file 53's control, reproduced) | all four operands in `[2^15, 2^16)` | **143.3 ms** |

Linear in every profile, checked at two counts each.

The two controls are the point of running them: file 53 measured 2.1 ms and 143 ms on this host
(`53:129-137`) and I measured 2.3 and 143.3 from a separately written generator. That is an independent
reproduction of the cost model's two anchor coefficients, which the review has not had.

**Decimal is not a single band; it spans most of the curve, and the numerator is the dominant term.**
Unit-numerator quanta (the currency and sensor-scale shape, the whole decimal fixed-point use case) sit
at three times dyadic, because a unit numerator makes the gcd terminate immediately. The denominator is
not free (10^6 to 10^9 doubles the cost, 6.8 to 14.5), but it is the weaker term: unit numerators with
denominators to 10^9 cost 14.5 ms while two-digit numerators with denominators only to 10^5 cost 21.0,
and wide magnitudes on both sides reach 79 ms, more than half the worst case file 53 named as the cliff.

So the cost paragraph `53b` adopted as spec text is right and wants one clause added. Its two
coefficients are stated per distinct composition and per repeated site; the per-composition coefficient
is itself a function of the operand magnitudes, dominated by the numerators. That is a sharper and more
useful statement for a code generator than "arbitrary rationals are expensive", because the numerator is
the one term an importer can often control. The nearest controlled pair this sweep has: unit numerators
over denominators to 10^6 cost 6.8 ms, and two-digit numerators over denominators to only 10^5 cost 21.0,
so introducing a two-digit numerator triples the cost while the denominator was shrinking. Nothing in the
design says so today.

### 6.2 The radix axis pays for itself, and the margin is a wall rather than a percentage

The question underneath section 5 that nobody has asked: `Implicit<E, A, B>` carries both an exponent and
a rational adjustment, and `A * radix^E` could be folded into a single rational `A'`. If the fold were
free, `E` would be a redundant axis and the whole question of whether it is a const or a type would not
arise. So: measure the fold.

Two spellings of the same decimal grid, `54_probes/price/gen.py`:

- **radix and exponent**: `Fx<Ten, P, ENeg<Pk>, 1, ...>`. The `Pos` nest to be written is the one for k,
  of depth `floor(log2 k) + 1`.
- **absorbed**: `Fx<Two, P, EZero, Ratio<H, Pos(10^k)>, ...>`. The nest is the one for `10^k`, of depth
  `floor(k * log2 10) + 1`.

| exponent k | nest depth, radix+exponent | nest depth, absorbed | absorbed compiles |
|---|---|---|---|
| 4 | 3 | 14 | yes |
| 19 | 5 | 64 | yes, but `Adjustment::DEN` is unreadable past this point |
| 20 | 5 | 67 | **`E0080`, `2_u64 * 12500000000000000000_u64` overflows** |
| 38 | 6 | 127 | yes as a type, readout still refused |
| **39** | 6 | **130** | **`E0275`, overflow evaluating the requirement `O<O<...>>: Pos`** |
| 398 (decimal64's own `emin`) | **9** | **1323** | no |

Two independent walls, both compiled, and neither is a slowdown:

**The readout wall at `10^20`.** `Pos::VAL` is a `u64`, so the absorbed adjustment's denominator cannot
be read past k = 19. The error names the offending type, and the diagnostic prints the full
sixty-four-constructor path, which is its own comment on the shape.

**The type wall at depth 130.** The `Pos` bound itself stops resolving: `E0275`. I attributed it rather
than assuming: the same depth on the **exponent** axis, with no `Gcd` and no `Adjustment` anywhere in the
chain, refuses identically at depth 130 and compiles at 127. So it is a property of `Pos`'s structural
recursion against the default `recursion_limit`, not of the reduction machinery. Raising the limit is not
a free out: file 41 already found that a naive `#![recursion_limit]` raise produced a SIGBUS inside
`rustc_trait_selection` (`49:310-312`).

Meanwhile the radix-and-exponent spelling of decimal64's own bottom grid compiles in **64 ms**, flat
across the whole exponent range, from a 519-byte source, against a 4486-byte source that does not
compile at all. Every compile-cost figure in this file is a difference between two points of one
harness; `54_probes/OUTCOMES.md` records a constant offset in the harness's absolute figures and why it
cancels.

So the fold is not merely expensive. **It does not exist at any real decimal format's exponent range**,
and `E` is not a redundant axis. The `Radix` and `Exponent` axes are what keep a decimal numeral off the
cliff, and that is an argument for their existence that nobody in this review has made.

### 6.3 The general fact, which the design should state once

*This generalises the measurement, and the generalisation is reasoned from it rather than separately
measured.*

> A `Pos` may not exceed roughly 2^127. Every axis that spells a magnitude as a `Pos` inherits the
> ceiling, and any quantity that would need a magnitude beyond it must be re-expressed as an exponent
> rather than as a magnitude. The value-unique tower is a **small-magnitude** encoding.

Checked against every magnitude the design actually spells: precision (113 for binary128, depth 7),
exponent bounds (16382 for binary128, depth 14), radix (16, depth 5), a MATLAB 16-bit slope or bias
(depth 16), the design's own division constants (48000, depth 16). All comfortable. The one magnitude
that breaches it is the one this section constructed, and the design already had the axis that avoids it.

The `u64` readout ceiling at 2^63 is the tighter of the two and is a property of the tower's own
`const VAL: u64`, not of the language. Whether that readout should be widened (to `u128`, or to a
multi-limb readout, or removed in favour of a comparison-only interface) is a real question this file
raises and does not answer; the design's consts are consumer-visible and 2^63 is a smaller number than a
reader of "arbitrary rational bias" would assume.

## 7. What I would put in the spec, verbatim

Seven sentences, in the form the next consolidation could take.

> `Radix` is one constructor, `Rad<P>`, over the sealed `Pos`, bounded on a sealed `AtLeastTwo`
> predicate. Every radix is expressible by naming a `Pos`; radix one and radix zero are unspellable,
> because a radix below two collapses the grid family the exponent function indexes.

> `Specials` is the product of two independent facts, infinity presence and NaN presence, not a chain.
> `NoSpecials`, `NanOnly` (OCP OFP8 `E4M3`), `InfOnly` (no witness, grounded `unknown`) and
> `IeeeSpecials`. Infinities add values one for one with data and can never affect the crossing
> contract; NaN adds one value for as many data as the encoding reserves and always can.

> The encoding is injective iff no value has two data, and a second datum arises from exactly four
> sources: an unrepurposed signed zero, more than one reserved NaN datum, a cohort, and a cohort of
> zeros. The predicate is derived from the axes and agrees with exhaustive enumeration at model widths
> over the whole configuration matrix.

> Radix two normalises for free through a hidden digit; no radix above two has a constant leading digit
> to hide, so its significand is stored unnormalised and its values have cohorts. The radix is the first
> link of that chain and the remaining links are `Encoding::Fields` and `Encoding::Canonical`, both of
> which the design already names.

> `Encoding::Canonical` selects the cohort member and is a genuine choice under any radix above two,
> where the two natural rules are different functions on the same value set. Under radix two with a
> hidden digit they are the same function, which is why the slot looked like a formality.

> This design represents every IEEE decimal format's value set, both of its interchange encodings, and
> its arithmetic results as values. It does not represent the standard's preferred exponents, because
> those select a datum as a function of an operation and this design's operations produce values. A
> consumer for whom the quantum is part of the number declares a decimal `Implicit` numeral, where the
> exponent is a type and the quantum is checked at compile time rather than propagated at runtime.

> Every numeral member a widening map computes is a type: precision, both `Ranged` exponent bounds,
> `Implicit`'s single exponent, the adjustment and the bias. `Radix` is the one member no map computes,
> and is a tag. A `Pos` may not exceed roughly 2^127 on any axis, so a quantity needing a larger
> magnitude is expressed as an exponent, never absorbed into a rational.

## 8. What I did not settle, and what I would measure

**The `InfOnly` corner still has no witness.** I searched and did not find a format with infinity and no
NaN. It is declared, it costs nothing, and it carries `unknown`. If a later member finds one, the product
shape already has the slot; if nobody does after another look, the honest move is to keep the row and the
ground rather than to delete the row, per file 50's own struck-versus-unknown amendment.

**The clause-5.2 characterisation of preferred exponents is from secondary sources.** It is load-bearing
for section 4.4's deviation sentence and it should be checked against the standard's text before that
sentence hardens, the way file 39 checked clause 7.

**`Abrupt` under an unnormalised encoding is reasoned, not modelled.** Section 2.3 states a cross-axis
constraint and I checked only the normalised half of it.

**The `u64` readout ceiling.** Whether `Pos::VAL` should stay a `u64` is a question this file raises from
a measurement and does not answer. It interacts with the consumer surface (a bias denominator above 2^63
is declarable and unreadable), and it is a one-line change with an unmeasured compile cost.

**Whether a decimal `Ranged` numeral's own quantiser needs anything new.** File 50 built the software
quantiser for radix two and showed the grid-selection step is the whole difference. I did not build the
radix-ten quantiser, and the one place I expect it to differ is the exact-division long-division kernel
file 50 had to repair (`50:110-119`), whose shift alignment is radix-two-shaped.

**A runtime bench, still owed and still not mine to run here.** File 50's software-quantiser-against-
`fadd` bench should sweep the radix as well as the subnormal fraction, because a radix-ten quantiser
cannot use shifts for its scaling step at all. That belongs in `mock/benches/` under the harness.

## 9. Where I disagree with something the review settled

Two, both small, both about wording that will mislead a later reader.

**The consolidation's table is wrong at `49:116` as well as `49:117`, and the review has been treating
the defect as a single line.** File 50 flagged 117, file 52 and file 53 both took 117 as settled and said
so in the same words, and none of the three looked one line up. `Implicit`'s exponent has the identical
defect and section 5 compiles it. The generalisation `53b` adopted (one member of every consolidation
pass diffs the tables against the sections they compress) is exactly right and this is the second exhibit
for it: a table-versus-section diff would have caught both lines in one pass, where three sequential
readers caught one line and inherited each other's scope.

**"The crossing contract has never been typed from the outside" is carried at `49:881-885` as one item
with the dithered entry point and the membership predicate.** After this file it should be split. The
crossing contract has now been typed from the outside and it produced two real findings (the derived
boolean, and the cohort-versus-quantum question), which is the same yield file 47's method produced for
the fold. The other two surfaces are untouched and the item should shrink to name them rather than stay
at its original width, because an open item that is two thirds open reads as fully open and gets
re-dispatched at full size.

## Provenance summary

Compiled or measured, this dispatch, fresh, on the pinned nightly, verified resolving from the probe
directory before any build: `probe_1_carriers_born_sealed.rs` (positive control),
`probe_1b`/`probe_1c`/`probe_1d` (three negative controls against the tower as an rlib, all four
introduction routes), `probe_2_crossing_binary.rs` (the crossing contract over the whole `Specials`
product, both underflow modes, the injective corner, the `FNUZ` corner),
`probe_3_crossing_decimal.rs` (radix ten, the section-difference witness, the normalised counterfactual,
the BID non-canonical model, decimal `mulnum`), `probe_4_implicit_exponent_as_type.rs` and `probe_4b`
(the `Implicit` exponent, positive and the three refusals), `crossing.rs` (the shared model),
`report.rs` (the runtime reporter that produced the tables in sections 3.2 and 4.2, counts only, no
timing), `price/gen.py` + `price/sweep.sh` + `price/results.csv` + `price/single.csv` (the radix-axis
depth and compile walls), `price/bias_gen.py` + `price/bias_sweep.sh` + `price/bias_results.csv` (the
decimal composition cost against file 53's two reproduced controls). Gates: `cargo test --workspace`
(122 binaries, 654 passed, 0 failed, 9 ignored) and both canon-gate greps, run for this dispatch.

Read and quoted rather than recalled: `49_consolidation_four.md`, `50_fog_the_float_model.md`,
`51_fallin_the_last_tick_and_the_licence.md`, `52_ringer_the_tests_that_were_owed.md`,
`53_torvalds_does_it_still_earn_its_keep.md`, `53b_persona_checkpoint_twelve.md`,
`30_probes/probe_3_sign_domain_against_sign_indexing.rs`, `50_probes/probe_3_exponent_as_type.rs`,
`53_probes/price/gen.py`, and my own file 36, re-read for this dispatch and overturned on one claim.

Reasoned, not compiled, and marked as such at the point of use: section 2.3's unnormalised-`Abrupt`
half, section 4.4's preferred-exponent characterisation and its three responses, section 6.3's
generalisation of the depth ceiling, and sections 7 through 9.

Read from secondary web sources rather than from a primary document, and flagged for a member with the
documents: the OCP OFP8 `E4M3` and `E5M2` specials and `emax` figures, the `E4M3FNUZ` variant's
repurposed negative zero, IEEE 754's treatment of non-canonical BID significands as zero, and clause
5.2's preferred exponents.

I suggest; op decides.
