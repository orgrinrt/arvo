# What the layers key on

Tiark Rompf, file 67. I wrote file 21, on what a fact is keyed on. Forty-five files have landed since
and I assumed nothing in it still holds. I did not rely on it here, did not re-read it before
compiling, and where this file needs something 21 said, it re-derives it.

**What I read, stated precisely.** `63_consolidation_six.md` in full, per the standing instruction
that it is the only required reading and is self-contained. The three deliverables since it:
`64_chlipala_the_owed_second_reads.md` at its section structure and sections 1, 5 and 7;
`65_pesce_pricing_the_l0_migration.md` at its section structure and section 1;
`66_lamport_the_transfer_argument.md` in full. I `ls`ed the panel directory once at the start.
Outside the review: `.claude/rules/unstable-features.md` at the "forbidden list is verification
infrastructure" section, read fresh; `45_leroy_what_each_claim_rests_on.md` at its ground table
(section 1.3, `45:160-171`), read fresh because file 66 builds on the `ffl` row and I would not take
that row's wording from a paraphrase; `66_probes/model.rs` and `66_probes/probe_4_abrupt_under_
unnormalised.rs` in full, because section 3 below reproduces a count against them; and the shipped
`arvo-strategy/src/container.rs` in full, which section 6 is about.

**On the independence instruction.** My brief says to form my own reading of the crossing-contract gap
before reading file 66's conclusions, and the review's standard is that a second read starting from
the first one's framing is corroboration in name only. So: I read `63` section 1.4, wrote out the
three statements with their domains and codomains, and reached the finding in section 1 below before
opening file 66. What I reached is not what file 66 reached. It is one step further in, and the
difference decides where the repair goes, so I state my derivation first and the reconciliation after
it rather than presenting the two as agreement.

**What I compiled against what I reasoned.** Ten artifacts in `67_probes/` with an `OUTCOMES.md`.
Five of them are expected to fail to compile and the compiler error is the result; five run to an
assertion. Sections 1, 2, 3, 6 and 8 are compile or run results and say which. Sections 4, 5 and 7
are reasoned from those results and are the design proposals; they are labelled reasoned where they
appear and I do not dress them up. There is no wall-clock number anywhere in this file, so nothing
here needed the bench harness and nothing here claims a timing.

**Gates.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth` both exit 1, empty, run fresh from the repo root for this file.
`git status --short` shows one untracked path, `mock/research/202607301300_formalization-spec-panel/
67_probes/`; no file under `mock/crates/` was edited, so no test-suite re-run is claimed and none is
needed. The pin resolves to `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, from `rust-toolchain.toml`, confirmed fresh before any probe ran. A bare
`rustc` outside the tree resolves to stable, as the dispatch warns; every probe was run from inside.

---

## 1. The gap is not a missing statement. It is a missing precondition, and two of the three existing statements do not typecheck without it

`63:186-191` gives the crossing contract as three statements:

> 1. `decode ∘ encode = id` on values, always.
> 2. `encode ∘ decode` is idempotent on data, always (canonicalisation).
> 3. `encode ∘ decode = id` on data iff the encoding is injective, a derived boolean.

Write out the domains, which is the whole exercise. Statement 1 says "on values", so `encode`'s
domain is the value set `V`. Statement 2 and statement 3 say "on data", so their quantifier ranges
over the datum set `D`. That fixes three maps:

- `encode : V -> D`
- `decode : D -> ?`
- and the design separately has a quantiser, `63:200-206`, mapping an exact rational onto the grid,
  which is a different map from `encode` and is partial by construction.

`decode`'s codomain is the question, and the design never states it. What `decode` actually is, at
every numeral this review has built, is the numeral's own arithmetic on the physical fields, `m * r^Q`
or its `Implicit` equivalent. That formula is total on the field tuple and it lands in the rationals.
Nothing in it consults `V`.

So the honest typing is `decode : D -> ℚ`, and now statement 2 is not a proposition. `encode ∘ decode`
requires `decode`'s output to be in `encode`'s **domain**, and `encode`'s domain is `V`, not `ℚ`.
Under a hole, statement 2 is not false. It is ill-typed. Statement 3, which is a claim about the same
composition, inherits the same defect.

That is a stronger statement than "a fourth statement is missing", and I compiled it rather than
arguing it. `probe_1_statement_two_is_illtyped.rs` types the three maps honestly, with `Value` a
newtype whose only constructor checks membership (the perimeter discipline
`what-you-can-observe-is-what-you-guaranteed.md` states: private field, one door), and writes
statement 2 out:

```rust
pub fn statement_two_canonicalise(f: &Fmt, d: Datum) -> Datum {
    encode(f, decode(f, d))
}
```

```
error[E0308]: mismatched types
84 |     encode(f, decode(f, d))
   |     ------    ^^^^^^^^^^^^ expected `Value`, found `Val`
help: try wrapping the expression in `Value`
84 |     encode(f, Value { inner: decode(f, d) })
```

**rustc's suggested fix is the bug.** `Value { inner: decode(f, d) }` is precisely the unchecked
coercion the design performs silently: it asserts membership without establishing it, and it is
available in the probe only because the suggestion ignores the private-field door. Under the real
perimeter it is not available at all, which is the point.

Two things follow, and both are about placement rather than content.

**Statement 0 goes in front because it is the side condition of two of the three, not because it reads
better there.** File 66 proposes it "in front of the existing three" (`66:475-476`) and gives no
reason beyond ordering. The reason is that statements 2 and 3 have no meaning until it holds. A spec
that lists four statements invites a reader to check them independently and find three of four
satisfied; a spec that states one condition and then three statements *over that condition* does not.

**And it is a condition on the pair, not on either side alone.** `decode` belongs to the encoding
(`63:179-181`: `Encoding`, nested inside `Lowering`, "may change which datum carries a value"). `V`
belongs to the numeral. Statement 0 relates them. That is the whole of section 5 below and it is the
reason this gap existed at all.

*grounded on: `pin`; `67_probes/probe_1_statement_two_is_illtyped.rs` (compile-fail, this file);
`tree` (`63:186-191`, `63:200-206`, `63:179-181`, `66:450-452`, `66:475-479`), read fresh.*

---

## 2. The escape is a family, not a cell, and file 66's matrix held the axis that produces the largest leak fixed

File 66 section 6 varies radix, precision, `Underflow` and normalisation, and concludes: "**Exactly
one cell of the matrix leaks**, and it is `Abrupt` with an unnormalised significand" (`66:470`).

That is true of the matrix it built. It is false of the design's configuration space, because the
matrix holds `Specials` fixed, and `Specials` is the design's other value-set-shrinking axis. It sits
on `Numeral` (`63:162`, inside `Ranged`, and `63:656`, a four-point product), while the field layout
that decides which data exist sits on `Lowering` (`63:648-651`, `Encoding::Fields`). Nothing couples
them, which is the same decoupling section 1 found, on a different pair of members.

`probe_2_the_escape_is_a_family.rs` models an IEEE-shaped field layout (biased exponent code, top
code reserved) at E4M3's own shape, four exponent bits and three significand bits, and counts data
whose denotation is not in `V(N)` across the whole `Specials` product:

| layout | `Specials` | data | escaping | percent |
|---|---|---:|---:|---:|
| ieee | `NoSpecials` | 128 | **8** | 6.2% |
| ieee | `NanOnly` | 128 | **1** | 0.8% |
| ieee | `InfOnly` | 128 | **7** | 5.5% |
| ieee | `IeeeSpecials` | 128 | 0 | 0.0% |
| ocp | `NoSpecials` | 128 | **1** | 0.8% |
| ocp | `NanOnly` | 128 | 0 | 0.0% |
| ocp | `InfOnly` | 128 | **1** | 0.8% |
| ocp | `IeeeSpecials` | 128 | 0 | 0.0% |

**Six of eight cells leak.** Under the IEEE layout, three of the four `Specials` members do; only the
member the layout was designed for does not. The largest leak is the entire top exponent code, `2^sw`
data, one part in `2^ew` of the datum set, and the probe asserts that identity rather than reading it
off the table.

The second row family is the finding worth keeping, and it is a convergence with `63:304-315` rather
than a criticism of it. OCP OFP8 E4M3 "does not represent infinities, uses two NaN bit patterns, and
raises `emax` from 7 to 8 to gain one binade of dynamic range". Read as a crossing-contract fact,
that sentence describes a format designer **performing the coupling by hand**: the value set lost
infinities, so the layout reassigned the freed patterns to finite values and moved `EMAX` to absorb
them, until `decode` was total again. The probe's `ocp/NanOnly` row is the check: zero escape.

So E4M3 is not a counterexample to the finding. It is the existence proof that the coupling is real,
deliberate work, that a published standard spends a paragraph on it, and that arvo currently has
nowhere to state it and nothing that notices when it is not done. And the `ocp/NoSpecials` and
`ocp/InfOnly` rows show the same hand-matched layout leaking the moment the value set moves again,
because arvo lets the two be chosen independently.

**What this does to the framing.** File 66's finding survives entirely and gets stronger: it is not a
decimal curiosity confined to one underflow policy, it is what happens whenever a `Numeral` axis
shrinks `V` and no `Lowering` axis correspondingly shrinks `D`. `Underflow = Abrupt` under an
unnormalised significand is one instance. `Specials` under a fixed field layout is another, and it is
the larger one. Anyone reading "exactly one cell leaks" will conclude the hazard is narrow and
configuration-specific. It is neither.

*grounded on: `pin`; `67_probes/probe_2_the_escape_is_a_family.rs` (compiled, whole product, this
file); `tree` (`63:162`, `63:304-315`, `63:648-651`, `63:656`, `66:458-473`), read fresh.*

---

## 3. The repair is derived, not chosen: there is no encode-side repair

Section 1 leaves a composition whose middle types do not meet. There are exactly two ways to fix
that, and file 66 proposes one without checking the other:

- **Shrink the source.** The encoding excludes the escaping data, so `decode`'s image lands in
  `encode`'s domain. This is statement 0, and it is an obligation on the encoding.
- **Widen the target.** Make `encode` accept everything `decode` produces. The design's only
  candidate is the quantiser: `encode ∘ quantise ∘ decode` typechecks if `quantise : ℚ -> V` is total.

The second is worth checking precisely because it would be cheaper. It needs no change to any
encoding and imposes no new obligation on anyone; it reuses a map the design already has.

`probe_3_no_encode_side_repair.rs` runs file 66's own quantiser, from file 66's own `model.rs` copied
verbatim so the answer is against its model rather than a re-implementation, on every escaping datum
of every leaking cell:

| r | p | data | escaping | `Value` | `Overflow` | `UnderflowRefused` |
|---:|---:|---:|---:|---:|---:|---:|
| 2 | 2 | 9 | 1 | 0 | 0 | **1** |
| 2 | 3 | 21 | 4 | 0 | 0 | **4** |
| 10 | 2 | 297 | 9 | 0 | 0 | **9** |
| 10 | 3 | 2,997 | 108 | 0 | 0 | **108** |

**The quantiser refuses on every escaping datum, without exception.** `encode ∘ quantise ∘ decode` is
exactly as partial as `encode ∘ decode` was, at exactly the same data. The widen-the-target repair
does not exist, and that is not a preference, it is arithmetic: a datum escapes precisely when its
value is in the hole, and the hole is precisely what `Abrupt` tells the quantiser to refuse.

The negative control matters here, so I state it rather than leave it in the probe: the quantiser is
the identity on all 2,701 values of `V(N)` at `r = 10, p = 3`, so "refuses on every escaping datum" is
a statement about the escaping data and not about a quantiser that refuses everything.

Two by-products. The 108-of-2,997 count reproduces independently, which is the second read file 66
asked for on its own probe 4 (`66:496-498`), and the smaller cells (1/9, 4/21, 9/297) are new and show
the leak is not a large-radix artifact. And the derivation upgrades statement 0's status: it is not
the repair someone picked, it is the only one available.

*grounded on: `pin`; `67_probes/probe_3_no_encode_side_repair.rs` (compiled, exhaustive over four
leaking cells plus a 2,701-element negative control, this file); `tree` (`63:200-206`, `66:458-473`,
`66:496-498`); `66_probes/model.rs`, used verbatim.*

---

## 4. Why the gap was there, and it is the same reason the transfer index set is short

Both halves of my dispatch turn out to be one defect, and naming it is the point of this file.

`63:179-181` states the design's own separation, and it is correct and load-bearing:

> `Lowering` changes no value; `Encoding`, nested inside it, may change which datum carries a value.
> **No law may read `Lowering`**; a law's key is a `const fn` parameter list and `Lowering` is not a
> parameter.

That rule buys a great deal. File 59's strategy door is safe because of it. `63:235-236` defines a law
as "a claim over a numeral's value set, quantified over its grouping class, keyed on every parameter
its proof used". Every mechanism the design has for stating a claim is keyed on `Numeral` and
structurally blind to `Lowering`.

Now look at the two defects side by side.

**The crossing contract omits a `Lowering` fact.** `decode`'s codomain is determined by the encoding.
The three statements quantify over `V` and `D` and never state the relation between them, which is
the one thing only `Lowering` knows.

**File 66's transfer index set omits `Lowering` coordinates.** Its table at `66:159-170` names six
coordinates of `Θ` and cites `63:154-163` for all six. Every one of the six is a `Numeral` member:
radix, precision, `EMIN`, `EMAX`, `Underflow`, `Specials`. The table is complete with respect to
`63:154-163`, which is the identity contract, and that is exactly the limitation: it enumerates the
mathematical coordinate system and stops.

**Same shape, twice, from the same cause.** The design's claim-stating machinery reads `Numeral` and
cannot read `Lowering`, so claims get written as though `Lowering` were not a coordinate. For a law
that is correct by construction, because a law is about values and `Lowering` changes no value. For
the crossing contract and for a transfer argument it is wrong, because both are statements *about the
relation between the two coordinate systems*, and a statement about a relation cannot be blind to one
of its arguments.

The distinction the design needs, and does not currently draw, is between two kinds of claim:

- **Value claims.** Keyed on `Numeral` alone. `Lowering` is not a parameter and reading it would be a
  defect. This is the law machinery and it is right.
- **Crossing claims.** Keyed on the pair `(Numeral, Lowering)`. Statement 0 is one. The transfer of a
  model claim to a real container is another. These are not laws, they are not on a law key, and the
  design has no machinery for them at all, which is why one of them was missing entirely and the
  other was stated over half its index set.

`63:179-181` is not wrong and I am not proposing to weaken it. What I am proposing is that "no law
may read `Lowering`" has been doing double duty as "nothing may read `Lowering`", and the second
sentence is what left both gaps.

*reasoned, this file, from `tree` (`63:179-181`, `63:235-236`, `63:154-163`, `66:159-170`), each read
fresh; the two defects it unifies are compiled in sections 1 through 3 and section 6.*

---

## 5. What a crossing claim should look like, and the shape is already in the design

Reasoned, and the reason it is cheap is that it copies a shape the notation vehicle already ships.

Look at how the face layer discharges the identical obligation. `63:657-660`:

```rust
pub trait NumeralFace {
    type Encoding: Bias;                  // unsealed, per-literal, bridges to the sealed tower
    const DISPLAY: &'static str;
}
```

`type Encoding: Bias` is a coarsening map from a fine layer to a coarse one, **with a bound the
compiler checks at the declaration site**. A face cannot be declared without exhibiting the element of
the coarse layer it maps to. The obligation is discharged where the face is minted, once, by the
person who knows the answer, and no consumer downstream ever re-checks it.

`decode` is the same kind of map, from data to values, and it has none of that. It is a formula the
design trusts. Statement 0 is exactly the bound that is missing.

So the proposal, in the design's own vocabulary:

```rust
/// The obligation a `Lowering` owes a `Numeral`. Not a law: it reads `Lowering`,
/// which `63:179-181` forbids a law from doing, and its key is the pair rather
/// than the numeral alone.
pub unsafe trait Crosses<N: Numeral>: Lowering {
    // Statement 0 is the safety condition:
    //   for every datum d of this encoding, decode(d) is in V(N).
    // Statements 1 through 3 are stated over it and are meaningful only where
    // it holds.
}
```

Five things about this shape, in the order they matter.

**It uses the discipline the design already ratified for claims it cannot derive.** `63:235-237`: a
law is "derived by blanket construction and safe, or asserted with `unsafe impl` (D16)". A crossing
claim gets the same two doors. Where the encoding is one the tower generates (a normalised binary
`Implicit`, say), the impl is blanket and safe, and section 2's `ieee/IeeeSpecials` and
`ocp/NanOnly` rows are what a blanket impl would cover. Where a consumer brings a hand-laid field
layout, it is an `unsafe impl` and the consumer is stating something they must know.

**It puts the obligation where the knowledge is.** Only the party choosing both the value axes and the
field layout can discharge it. That is the format declaration site, exactly as the face's `Encoding`
bound sits at the face declaration site. Not the law author, not the consumer of an operation, not
the quantiser.

**It does not weaken `63:179-181` anywhere.** A law stays keyed on `Numeral` and stays unable to read
`Lowering`. `Crosses` is a different trait with a different key, and no law may bound on it, which is
the sentence that keeps the split honest.

**It is the only place in the design a `(Numeral, Lowering)` key exists**, which is a feature. If a
second such claim ever appears, it goes here, and the pair-keyed surface stays enumerable rather than
diffusing into bounds all over the tower.

**Rewrite cost is zero against the shipped tree**, since no shipped source names any of this
(`63:663-665` and the gates at the top of this file), and near-zero against the design, since it adds
one trait beside `Lowering` and changes nothing else. That matters more than the mechanism's elegance:
per the panel's own tiebreaker, a proposal that reaches the same place by a smaller move is the better
one, and this one reuses `NumeralFace`'s shape rather than inventing a second.

**What I am not proposing**, and I want it on the record so a later reader does not have to guess: I
am not proposing that `decode` return a `Maybe`, or that the crossing contract be checked at runtime,
or that `Encoding` gain a well-formedness predicate every operation consults. All three move a
declaration-time fact into a use-time check, which is the binding-time error this design exists to
avoid. The fact is known when the format is declared. It should be checked there, once, and cost
nothing afterward.

*reasoned, this file, from `tree` (`63:179-181`, `63:235-237`, `63:657-660`, `63:663-665`); the
`NumeralFace` shape it copies is compiled in `67_probes/probe_4_what_the_layers_key_on.rs` and
`probe_5c_face_cannot_reach_numeral_position.rs`.*

---

## 6. Second read of file 66 section 1: the penultimate sentence's enumeration is also incomplete, and the missing coordinate is a `Lowering` member

File 66 quotes the ratified rule and says: "Every sentence but the last is correct and I would not
change a word of it" (`66:49-50`), then corrects the last. I agree entirely with its correction, and
with its four legs reading, and with its four-ground vocabulary, which I build on rather than
relitigate. My disagreement is with the concession.

The penultimate sentence is an enumeration:

> A design that verifies a claim exhaustively at, say, eight bits and relies on it at sixty-four is
> relying on there being no way for a type to observe which instantiation it is in and behave
> differently. **Full `specialization` is exactly such a way, and `TypeId` is another.**

There is a third way, it is permitted, and it is load-bearing in a shipped arvo crate today.
`arvo-strategy/src/container.rs:254-280` projects the container through
`Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, S>`. The width selects a tag, the tag
selects an impl, the impl selects an associated type, and the associated type is the arithmetic
container. That is a type observing which instantiation it is in and behaving differently. It is not
`specialization` and it is not `TypeId`. It is Pattern C const-tag dispatch, the design's own
mechanism, and file 65 spent a whole dispatch pricing its migration.

`probe_7_uniformity_fails_without_specialization.rs` reproduces the shape with no `#![feature(...)]`
gate at all and exhibits a property whose truth value moves across widths:

| width | container | doubling of 200 wraps |
|---:|---|---|
| 8 | `u8` | **true** |
| 9 | `u16` | false |
| 17 | `u32` | false |

One parametric body. No specialization. No `TypeId`. No forbidden feature. The property is TRUE at
the model width and FALSE one bit up, and what moved was the projection.

**The shipped ladder, counted exactly**, from `container.rs:60-96` and `:170-243`. `tag_hot_cold`
has six classes (`<=8, <=16, <=32, <=64, <=128, wide`); `tag_warm_precise` has five, skipping the
`u128` rung. `Hot`/`Cold` project the native ladder `u8, u16, u32, u64, u128` plus `WideBits<BYTES,
A16>` for `Hot` and `WideBits<BYTES, A1>` for `Cold`; `Warm`/`Precise` project `u16, u32, u64, u128`
plus `WideBits<BYTES, A1>`. Counting distinct container types the projection can select, across both
signs, the total is **twelve**. A model checked at eight bits, unsigned, `Hot` exercises exactly one
of them.

**This is not an argument against the bans and I want to be exact about that**, because it would be
easy to read as one. The bans are correct, they are necessary, and file 66's section 3.5 is right that
they extend to any parameter count without modification. What the compiled result shows is that the
rule's own enumeration of the ways an instantiation can be observed is a list of two where the design
has at least three, and the third is the one arvo actually ships.

**And it fits file 66's own vocabulary cleanly, which is why I am extending rather than objecting.**
`66:316-322` gives four grounds: `symmetry`, `saturation`, `induction`, `unargued`. Container class is
a coordinate, and it takes a `saturation` ground with an unusually clean threshold, because within a
class the container is **literally the same type**:

> **Container class.** Source: `Lowering::StoredWidth` through `Strategy`'s own projection. The
> claim's dependence on the width, through the container, stops changing inside a class by
> construction, since the projection resolves to one type there. Threshold: one width per class. The
> class boundaries are a `const fn` the shipped source already computes.

That is a `saturation` ground whose threshold is not measured, not argued and not believed. It is
read off `tag_hot_cold`. And its residue, in file 66's own sense (`66:345-350`), is nearly free: a
model declaration asserts at build time that its widths hit every class it will be cited for, which
is a `const` assertion over a `const fn` that already exists.

**Two consequences for how the review's own models should be read.** First, every `model` claim in
this review, including file 50's 41,380,159-operation binary32 check and file 64's exhaustive
eight-bit injectivity matrix, covers one container class of twelve, and none of them says so.
Second, the fix is cheap and specific: an additional model at nine bits costs `2^18` pairs where eight
bits cost `2^16`, well inside the budget file 66 section 5 correctly reframes as a step count rather
than a width. Classes above `u32` are not exhaustively reachable at any budget, and that is a real
limit that should be written down as `unargued` for those classes rather than left implicit in a
claim that names one width.

**Proposed wording**, one clause, keeping the ban untouched exactly as file 66 keeps it. Replace
"Full `specialization` is exactly such a way, and `TypeId` is another" with: "Full `specialization` is
one such way and `TypeId` is another; a const-tag projection that selects a different associated type
per width is a third, it is permitted, and arvo's own container dispatch is an instance, so the bans
close the ways an instantiation can be given a different *body* and not the ways it can be given a
different *type*."

This is one expert's read of a sentence a second expert declined to change, so it is the first read
of a disagreement rather than a second read of an agreement, and it wants its own second before
anything is written to a ratified rule.

*grounded on: `pin`, `host`; `67_probes/probe_7_uniformity_fails_without_specialization.rs`
(compiled and run, this file); `tree` (`arvo-strategy/src/container.rs:60-96`, `:170-243`, `:254-280`,
read fresh; `63:120-121`; `66:41-47`, `66:49-50`, `66:316-322`, `66:345-350`); ratified rule
(`.claude/rules/unstable-features.md`, "The forbidden list is verification infrastructure", quoted
from file 66's own verbatim quotation and checked against the rule text).*

---

## 7. The keying rule

This is my subject and it is now answerable, because the two halves of the dispatch turn out to
constrain each other.

### 7.1 There are two identity notions, not three

`63:864-868` names the notation residuals and the dispatch frames the wider question as three layers:
face, encoding, value. That framing is one layer too many, and collapsing it is what makes the rule
short.

The encoding is **value-unique**, ratified at `44b` and restated at `63:174-175`: `Bias` and
`Adjustment` are "signed, gcd-normalised rationals, value-unique and sealed". Value-unique means one
type per value. So type equality on the encoding *is* value equality, and the encoding is not a third
identity notion, it is the value's representative in the type system.

That leaves two:

- **Face identity.** Established at expansion time, per invocation. `numeral_face!(Third = 1/3)` in one
  module and `numeral_face!(OneThird = 2/6)` in another mint two types. Syntactic, site-local.
- **Encoding identity, which is value identity.** Established at type-check time, structurally,
  globally. `BPos<H, I<H>>` is the same type wherever written.

`probe_4_what_the_layers_key_on.rs` compiles the collapse: two faces, one literal, and a law keyed on
one type parameter used twice accepts them both, which it could not do if they projected to different
types. Both give `(1, 3)`. Their `DISPLAY` strings differ, `"1/3"` and `"2/6"`, and that is the only
difference observable anywhere in the program.

### 7.2 The rule

**A fact is keyed on the coarsest layer whose identity its truth depends on.**

With two layers that is binary, and the question to ask at every fact is: does this depend on *where
it was written*, or only on *what it denotes*?

- **Only on what it denotes:** every law, every arithmetic result, every comparison, every `mulnum`
  computation, every membership question. **Keyed on the encoding.** A face must never appear in the
  key, because two faces for one literal denote one value and a fact that distinguished them would be
  stating something false about the numbers.
- **On where it was written:** diagnostics and display. Nothing else. **Keyed on the face.** The face's
  entire content is "what did the consumer write here", which is exactly what expansion time knows and
  what nothing later can recover.

"Coarsest" rather than "finest" is the load-bearing word, and it is where I depart from what I wrote
in file 21. A fact keyed too finely is not merely imprecise, it is **wrong**, because the extra
distinctions the finer layer draws are not value-carrying and a fact that respects them asserts
something false. `probe_5b_face_keyed_refuses_one_value.rs` is that failure, compiled:

```
error[E0308]: mismatched types
   |         Tagged::<OneThird>(core::marker::PhantomData),
   |         ^^^^ expected `Tagged<Third>`, found `Tagged<OneThird>`
```

`Third` and `OneThird` are both 1/3. A refusal here is a false statement about arithmetic, delivered
with a compiler's authority.

### 7.3 The rule is enforced, and nobody arranged it

The good news, and the reason this half of the design needs no new mechanism at all.

A face cannot reach a numeral position. `probe_5c_face_cannot_reach_numeral_position.rs`:

```
error[E0277]: the trait bound `Third: tower::Bias` is not satisfied
help: the trait `tower::Bias` is not implemented for `Third`
```

A numeral position takes the **sealed** carrier. A macro-minted face implements `NumeralFace` and
never `Bias`, because the seal's private supertrait is unreachable from the expansion crate, which is
exactly file 61's structural necessity (`63:377-390`). So the only route from a face into the tower is
`<F as NumeralFace>::Encoding`, and that projection is a function: nothing downstream recovers its
argument. `probe_4`'s `erases::<F>()` is the statement, and both faces give the same answer.

**The seal that forced the bridge is the same mechanism that guarantees face identity cannot leak
into a type-level fact.** File 61 found the seal forces a bridge trait and reported it as a cost. It
is also the enforcement of the keying rule, for free, and the design did not have to be arranged for
it.

### 7.4 What this says about the decoder ring, which is not a defect

`63:428-435` records two diagnostic findings and calls the first a decay: "an operation generic over
the raw encoding decays the face one hop in, exactly as file 56 predicted", and file 59's separate
finding that an `on_unimplemented` on an outer projected tag never renders.

Under the rule, neither is a defect in the notation. `probe_5a_encoding_keyed_refuses_two_values.rs`:

```
error[E0308]: mismatched types
   |     let _ = law_keyed_on_encoding(mk::<Third>(), mk::<Half>());
   |             ---------------------                ^^^^ expected `Implicit<BPos<H, I<H>>>`,
   |                                                       found `Implicit<BPos<H, O<H>>>`
```

The message names the expansion because the **operation is keyed on the encoding** and the encodings
are what differ. Naming the face there would name something that did not fail. The decay is the
keying rule made visible, and file 59's independently-reached fix is the general one: write the
message on the carrier whose bound actually fails, which is another way of saying write it on the
layer the operation is keyed on.

So the spec sentence to avoid is "the error names your numeral", unconditionally. The sentence that
is true and checkable is: **an error names the layer the failing operation is keyed on.** A
declaration-site mismatch is a fact about the face and names the face. A law's refusal is a fact about
the encoding and names the encoding. Both are correct and they are different, and a consumer told the
rule can predict which they will get.

### 7.5 The same rule, at the physical layers, is the crossing gap

The rule generalises down, and this is where the two halves of the file meet.

- `face -> encoding`: coarsening, total, **enforced by a bound** (`type Encoding: Bias`), erases the
  declaration site.
- `datum -> value`: coarsening, **total only if statement 0 holds**, enforced by nothing, erases the
  cohort member.

Both coarsen. Both discard a distinction that is not value-carrying. And the failure mode is
identical: key a fact on the finer layer and it asserts something false.

The datum-side instances are already in the registry and were not previously connected to each other
by a rule. `63:485-493`: the shipped `TotalOrd` compares bit patterns, so `-0.0` and `0.0` compare
`Less` rather than `Equal`, and a `const` assertion of "two data that denote the same value compare
`Equal`" refuses to compile against it. `63:820-825`: `arvo-spectral` classifies a Fiedler component
by comparing against zero under that order, so two NaN patterns differing only in a sign bit land in
opposite partition classes. Both are a value-fact keyed on the datum. `probe_5b` is the same defect
one abstraction layer up, keyed on the face.

**One rule, three layers, one failure mode.** And the fix has one shape too. The face side gets it
free because the coarsening is a checked bound at the declaration site and the only door. The datum
side has no such door, which is why `TotalOrd` had to be caught by a probe rather than refused by a
compiler. The `TotalOrd` split file 60 proposed and `63:495-502` adopted is the right fix and it is a
special case of the general one: **a value-keyed operation consumes its operand through a
canonicalising projection, and that projection is the only door.**

*grounded on: `pin`; `67_probes/probe_4_what_the_layers_key_on.rs` (compiled and run),
`probe_5a`, `probe_5b`, `probe_5c` (all compile-fail, this file); `tree` (`63:174-175`, `63:377-390`,
`63:428-435`, `63:485-493`, `63:495-502`, `63:657-660`, `63:820-825`, `63:864-868`), read fresh;
reasoned for 7.2 and 7.5's generalisation.*

---

## 8. The three notation residuals, decided

`63:864-868` lists three. The rule in section 7 decides two of them and reduces the third.

### 8.1 `Adjustment`'s entry point: it needs its own, and the reason is keying rather than machinery

The residual asks "whether `Adjustment` needs its own entry point or shares `Bias`'s emission
machinery under a different wrapping constructor". Posed that way it reads as a question about
duplication, and the duplication answer is clear: parse, digit extraction, decimal-point folding, gcd
reduction and bit decomposition are the same arithmetic on the same digits for both roles, so writing
them twice is a missing generator and one implementation serves both.

But that is the smaller half. Under the keying rule the question is whether the **role** is a fact
that belongs in the type, and it is, because the two roles enter the value map at different places.
File 66's own section 3.2 depends on exactly this: its exponent-shift symmetry is proved conditional
on "no `Numeral` member contributing a nonzero additive constant to the value", and it holds for
`Ranged` only because `Ranged` carries no `Bias` (`66:203-212`). An adjustment scales; a bias offsets;
they are not exchangeable.

`probe_6_adjustment_needs_its_own_door.rs` prices the exchange. With one shared face type and the role
carried by argument position only, `value::<E, X, Y>` and `value::<E, Y, X>` both compile, both run,
and denote 11 and 84.333 respectively. `probe_6b_role_swap_refuses.rs`, with two doors over one shared
generator:

```
error[E0277]: the trait bound `Bia<7, 1>: Adjustment` is not satisfied
error[E0277]: the trait bound `Adj<1, 3>: Bias` is not satisfied
```

**So: two entry points, one generator.** `raw_bias!(EXPR)` and `adjustment!(EXPR)` emit different
wrapping constructors over the identical host-side reduction, and the reason is not symmetry or
tidiness, it is that the role is value-affecting and therefore belongs in the type rather than in a
convention about argument order. That is the same conclusion the design already reached for `Bias` and
`Adjustment` as separate sealed carriers (`63:161`); the residual is asking whether the *notation*
should preserve a distinction the *tower* already draws, and the answer is that a notation which
collapses a distinction the tower draws is a notation that can express states the tower forbids.

### 8.2 Cross-call-site face identity: the question is about the wrong layer, and per-site is correct

The residual asks "whether every literal a consumer writes twice should resolve to the same face
type", and the dispatch adds that it "decides whether a consumer's error message says one thing or
two".

It does not need establishing, and it should not be established. Section 7 gives three reasons and
each is compiled.

**Nothing that affects compilation is keyed on the face.** `probe_5c` shows a face cannot reach a
numeral position at all; `probe_4` shows the projection erases the site; so two faces for one literal
are interchangeable everywhere the type checker looks. Whether they are "the same face" is not a
question the type system ever asks on a value fact.

**Where it is observable, per-site is the better answer.** The face's whole content is `DISPLAY`, and
what a consumer wants in an error at site A is what they wrote at site A. Unifying two faces would
make one site's diagnostic name the other site's declaration, which is strictly worse. So the error
message saying "one thing or two" is correct when it says two, because there were two declarations
and each names itself.

**And unifying them would create the failure in 7.2.** A mechanism that makes two literals resolve to
one face is a mechanism that keys an identity fact on the face layer, and `probe_5b` is what that
costs when the two spellings differ.

So the residual closes without a mechanism: **face identity is per declaration site, deliberately, and
nothing is keyed on it except display.** The one thing worth adding to the spec is the sentence that
tells a consumer this, because a consumer who assumes faces unify will eventually write a signature
keyed on one, and the diagnostic they get will be `probe_5b`'s, which is confusing precisely because
the two types denote the same number.

### 8.3 The pricing hazard: it is a keying question too, and it names its own control

The dispatch reports a flagged hazard: "an unused alias may cost nothing while a used one costs, which
makes a measurement of the notation's price depend on what the measurement itself instantiates". The
droplist already records the instance (`63:902-905`): pricing a declaration sweep against an unused
type alias produced a result in the wrong direction, the trusted face looking more expensive than the
open form, and the correction was to force the bound in both arms.

Under the keying rule this stops being a benchmarking gotcha and becomes a statement about what is
being measured. A declaration's cost is not a fact about the declaration, it is a fact about the
**obligations the declaration discharges**, and an unused alias discharges none: no bound is forced,
so `Reduce` never runs, so the type checker does the work that its instantiation demands and no more.
That is monomorphisation behaving correctly, not an artifact.

So the control is not a trick to remember, it is derivable: **a measurement of a declaration's price
states which bounds it forces, and two arms are comparable only when they force the same ones.** For
the notation macro specifically, the bound to force is the one that makes the emitted type's
`Encoding` projection normalise, since that is the obligation a real consumer's use creates. I did not
build this measurement, because it is a timing claim and timing claims belong in the bench harness
rather than in a probe with a timer; I am naming the control the harness run should carry, and file
61's own corrected arms (`63:419-425`) already satisfy it.

*grounded on: `pin`; `67_probes/probe_6_adjustment_needs_its_own_door.rs` (compiled and run),
`probe_6b_role_swap_refuses.rs` (compile-fail), `probe_4`, `probe_5b`, `probe_5c` (this file);
`tree` (`63:161`, `63:419-425`, `63:864-868`, `63:902-905`, `66:203-212`), read fresh; reasoned for
8.3's control, which is not measured here.*

---

## 9. Where I checked myself, and what I could not close

My subject is the shape of error I could most easily commit: a rule about keying, stated so
abstractly that it is true of everything and decides nothing. Four checks.

**Is the keying rule vacuous?** It would be if nothing were ever keyed on the face. `probe_4` shows
`DISPLAY` differs between two faces for one literal, and the rule assigns diagnostics to the face
deliberately rather than emptying the layer out. Both layers carry something.

**Does the rule decide anything it did not already know?** It decides three residuals in section 8, in
one case against the framing the residual was posed in (8.2), and it reclassifies two registry entries
(`TotalOrd`, the spectral NaN classification) as instances of one defect rather than two mechanisms.
Those are consequences, not restatements.

**Is section 2's `Specials` extension a set the design does not range over?** It would be if the field
layout were fixed by the tower. It is not: `Encoding::Fields` is a `Lowering` member (`63:648-651`)
and the design's own downstream contract (`63:682-689`) says a build layer "acts freely on
`Lowering`". So the configuration is reachable by exactly the mechanism the design licenses.

**Is probe 7's counterexample a disguised specialization?** It would be if the two widths ran different
bodies. They run one parametric body; the probe asserts that two widths in the same container class
give the same answer, and that the u16 arm does real arithmetic (40,000 doubled wraps there), so the
difference is the projection and not a second implementation.

And one I could not close, recorded rather than smoothed.

**Statement 0 against the two datum-dependent operations.** File 66 asked its second reader to check
statement 0 against `quantize` and `roundToIntegralExact`, which `63:338` carves out as datum-dependent
by definition (`66:496-498`). I did not do it. Section 5's `Crosses` shape makes me think the check is
about something narrower than it sounds, since a datum-dependent operation is one whose *result datum*
depends on an *operand datum*, which is a statement wholly inside `D` and does not obviously touch
`decode`'s codomain. But that is a guess about the answer and not the answer, and the check is still
owed to someone who does it. I flag it forward unchanged rather than claiming a reading I did not
perform, which is the same recusal file 62 made on the membership read.

---

## What a consolidation could take, close to verbatim

The crossing contract's gap is not a missing statement, it is a missing precondition: `encode`'s
domain is the value set, so `encode ∘ decode` requires `decode`'s output to be in the value set, and
statements 2 and 3 are therefore ill-typed rather than merely unchecked wherever `decode` escapes,
compiled here as an `E0308` whose suggested fix (`Value { inner: decode(d) }`) is exactly the
unchecked coercion the design performs silently. Statement 0 goes in front because it is the side
condition of two of the three, not because it reads better there. The escape is a family rather than
one cell: file 66's matrix held `Specials` fixed, and `Specials` is the design's other value-set-
shrinking axis, so across the whole product six of eight cells leak, three of four `Specials` members
under an IEEE-shaped layout including the whole top exponent code under `NoSpecials`, and even OCP
E4M3's own hand-matched layout leaks under two of four. E4M3 is not a counterexample but the existence
proof: its designers dropped infinities and raised `emax` from 7 to 8 until `decode` was total again,
which is the coupling performed by hand, and arvo has nowhere to state it and nothing that notices
when it is not done. The repair is derived rather than chosen, because the only alternative,
composing through the quantiser, is refused at every escaping datum without exception, 1 of 9, 4 of
21, 9 of 297 and 108 of 2,997, against a quantiser that is the identity on all 2,701 values of the
value set. Both this gap and the short transfer index set have one cause: `63:179-181`'s correct rule
that no law may read `Lowering` has been doing double duty as nothing may read `Lowering`, so the
crossing contract omits `decode`'s codomain and file 66's six-coordinate index set names six `Numeral`
members and no `Lowering` one. The design needs a second kind of claim beside the law, keyed on the
pair rather than on the numeral, and its shape is already in the tree: `NumeralFace`'s `type Encoding:
Bias` is a coarsening with a compiler-checked bound at the declaration site, and a `Crosses<N:
Numeral>: Lowering` obligation with D16's own safe-blanket-or-`unsafe impl` discipline gives `decode`
the same treatment at zero rewrite cost against the shipped tree. Separately, the ratified rule's
penultimate sentence is incomplete as well as its last: a const-tag projection that selects a
different associated type per width is a third way an instantiation is observed, it is permitted, and
`arvo-strategy/src/container.rs:254-280` is an instance, demonstrated by a property TRUE at eight bits
and FALSE at nine with one parametric body and no gate; container class is a transfer coordinate that
takes a `saturation` ground in file 66's own vocabulary with a threshold of one width per class, six
classes for `Hot`/`Cold` and five for `Warm`/`Precise`, twelve distinct container types in all, of
which every `model` claim in this review exercises one. On the notation side there are two identity
notions and not three, because value-uniqueness makes the encoding the value's representative, so a
fact is keyed on the coarsest layer whose truth depends on it: laws on the encoding, diagnostics and
display on the face, nothing else on the face at all. Keying too finely is not imprecision but a false
statement, since two faces for one literal denote one value and an operation refusing the pair is
wrong about the numbers. The rule is already enforced and nobody arranged it: a face cannot reach a
numeral position because the seal forbids it, so the `NumeralFace` projection is the only route and it
erases the site, which means file 61's bridge-trait cost is also the keying rule's enforcement. The
same rule one layer down is the crossing gap, and `TotalOrd`'s bit-order comparison and
`arvo-spectral`'s NaN classification are the same defect as a law keyed on a face. Three residuals
close: `Adjustment` gets its own entry point over one shared generator, because a single door lets the
roles exchange silently and move a value from 11 to 84.33 while two doors refuse it, and because file
66's own symmetry proof depends on the distinction; cross-call-site face identity should not be
established, since nothing that affects compilation is keyed on the face and per-site display is the
better diagnostic, so the residual is asking about the wrong layer; and the pricing hazard's control
is derivable rather than memorable, since a declaration's price is a fact about the obligations it
forces and two arms are comparable only when they force the same bounds.
