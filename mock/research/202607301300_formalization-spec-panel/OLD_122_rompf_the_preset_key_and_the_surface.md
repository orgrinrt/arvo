# The preset key and the surface: the second reads, one spelling corrected, and a collision neither file saw

**Persona:** Tiark Rompf, staging and binding-time lens. Dispatched as the second independent read on the
preset key, as the deciding read on the `Precision` collision file 121 left open, and to land the material
two prior pairs have already converged on.
**Date:** 2026-08-05

**Reading order, stated because the dispatch made it a condition on one of the two questions.** I read
`110_consolidation_eleven.md` at its structural sections, then `118`, then `120`, then reasoned the preset
key out and **compiled my answer before opening `119`**. That order is recoverable from this file: every
probe in section 2 was built against `110`'s own declarations and against the two ratified tables, and the
one place I reach further than `119` (the reading of the rule) is the place where working without it forced
me to notice that the rule's primary clause does not say what it is being asked to say. Where `119` got
somewhere first I say so.

**Line numbers are against the working tree as of 2026-08-05**, after `121`'s edits and before mine. `119`
and `120` both pin to `ee027e1` and both record that the file moved under them; mine moved again, so a
citation from either of those files to `110` is roughly 44 lines low against what I read, and my own
citations are 437 lines low against what a reader will see after this pass. **That is now three files in a
row recording the same hazard**, and it is the failure mode the workspace's own reference rule names: a line
citation that still resolves and now points at different content. It is worth one sentence in the canon and
I have not written it, because it is a convention question and one is already on op's list.

---

## 0. The two gates, and one claim in the brief that does not survive

**The canon gate passes.** The governing material is op's own: `13c`'s standard, the four standing
directives (`110:119-210`), `16b`'s and `16c`'s posture corrections, `108b`'s two standing principles, and
the numbered register in the three frozen topic files. `108b:184-186` orders the remaining stretch to work
the open list down rather than open ground, which is what a second read plus a landing pass is. Nothing here
touches `mock/crates`, per `108b:188-193`.

**The test gate, run rather than cited.** `cargo test --workspace` from inside the tree, 2026-08-05, summed
per binary: **672 passed, 0 failed, 9 ignored**. That reproduces what files 102 through 108, 118 and 119 each
report independently. **The first attempt produced a green result that was a lie and I want it recorded**: I
wrapped the run in `timeout`, which does not exist on this host, and the pipeline's exit status came from
`tail` rather than from `cargo`, so it reported exit 0 having run nothing. The suite had to be re-run to a
log and counted. A green line whose provenance is a pipeline's last stage is not a measurement, and this one
took thirty seconds to catch only because the harness printed no test-result lines to count.

**I did not re-audit the test bodies**, and the reason is op's rather than convenience: `108b:174-181` rules
that the tautological tests this review found are collected and not acted on, becoming an
implementation-phase checklist which `110` section 4 carries. A sixth report of the same three findings is
what that ruling exists to stop. **What I will say is that the number is the weakest evidence in this file.**
It measures a tree the canon replaces, and the instrument that measured anything here was the compiler,
nineteen times.

**Breaking the brief.** Its factual claims hold where I could check them cheaply, with one exception and one
correction.

The exception: it describes `120` as having "drafted five canon sentences in its section 9". **Five is right
and `120`'s own count of them is wrong.** `120:455-456` says "Three are ratifying, one is a prohibition, one
is a perimeter statement, and the last is an open-list line", which describes six roles across five
blockquotes, and only two of the five are ratifying. That is a count disagreeing with its own list, which is
the defect `110` corrects four times in its own text (`118` section 5) and states as a thesis at three
separate places. It changes nothing about the sentences and I am landing all five; it is recorded because the
next file to cite `120`'s section 9 by count will cite it wrong.

The correction: the brief says `121` found the collision at the use site and that rustc's suggested repair is
`dyn`. Both true. **What it does not say, and what decides the question, is that the collision refuses one
step earlier**, at the declaration. Section 3 below.

---

## 1. What I formed before opening file 119

The defect is not in question and I add nothing to its statement. `Policy::Quantisation` and
`Lowering::StoredWidth` are nullary associated types on the marker (`110:3090-3102` as it stood), so
`Warm::StoredWidth` is one type, and the two ratified tables say `doubled` for fixed-point and `minimum` for
float. `118` found it, and `110:2746-2763` states it.

The question I asked first was not which of the two offered spellings to take. It was **what actually
determines a row**, because that is the binding-time question underneath and the two offered spellings are
answers to it that nobody had checked against each other.

The row is a function of the preset and the number kind. The number kind, in this design, is not a separate
notion: `Numeral::Exponent` is bounded by `ExponentForm` (`110:880`), `ExponentForm` is sealed with exactly
two constructors, `Implicit` and `Ranged` (`110:3181`, sealed at `110:936-947`), and there are exactly two
tables. **So the exponent form is not a key that happens to work. It is the partition the two tables
induce**, and that is a structural fact about the design rather than a reading of any rule.

Then I compiled the rulings, because a rule applied by argument is a preference.

---

## 2. The preset key, compiled

### 2.1 The defect is a diagnostic, which no pass had shown

`118` argued the entailment and `110` states it in prose. Neither put it through a compiler. Against the
document's own declarations, stating both ratified rows is refused outright:

```
error[E0119]: conflicting implementations of trait `Lowering` for type `Warm`
  --> pA_current_defect.rs:34:1
   |
27 | impl const Lowering for Warm {
   | ---------------------------- first implementation here
...
34 | impl const Lowering for Warm {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Warm`
```

That is worth having rather than merely tidy. The entailment was the thing `118` marked as owed a second
read, and a coherence error is a stronger statement of it than any argument: the two ratified tables are not
inconsistent with each other, they are jointly unstatable against the declaration that is supposed to carry
them.

### 2.2 Both offered spellings are refuted

**Keying on the numeral over-keys, exit 0.** `impl const Lowering<U13F3> for Warm` with
`StoredWidth = DoubleLogical` and `impl const Lowering<U14F2> for Warm` with `StoredWidth = Minimum` compile
together. Two `Implicit` numerals, one number kind, disagreeing about what `Warm` means, with nothing in the
language relating them. Under this spelling "`Warm` stores doubled for fixed-point" stops being a statement
anyone can make: `Warm` is a per-numeral lookup that happens to be constant on each kind today. The
`Crosses<N: Numeral>` precedent `110:2757` reaches for does not carry: a crossing statement genuinely depends
on the individual numeral's value set, and a preset row does not.

**Per-kind markers admit a well-typed nonsense, exit 0.** `Number<Binary32, WarmFixed>`, a `Ranged` numeral
wearing the fixed-point preset's storage row, type-checks. One methodological note, because it bit me: **a
type alias alone proves nothing here**, since Rust does not check a type alias's bounds, so the probe has to
force the bound at a signature. `119:351-352` does force it and its result stands.

`119` reached both of these first and I reproduce them rather than claim them. **What I add is a second
ratified-rung objection to the per-kind spelling that neither file makes.** `119:364-365` notes that D72
gives `arvo-strategy` "`Hot`, `Cold`, `Warm`, `Precise`, and nothing else" and that eight markers is a
different crate. The sharper form: under that spelling **the four types op ratified by name cease to exist**.
`Warm` is not a marker any longer; `WarmFixed` and `WarmFloat` are, and `Number` takes one of those. D53's
aliases preserve the consumer's spelling (`110:3311`), so the consumer never notices, and D52 (`110:3365`)
makes compositions public and bindable by anyone, so the mispairing is reachable by exactly the door D52
opens. The spelling survives D53 and dies on D52 and D72 together.

### 2.3 The exponent form, and what actually forces it

Both contracts take the form, and each impl is quantified over that form's own parameters. Compiled at the
shipped `pub const trait` spelling, both tables transcribing cell for cell, exit 0. Three properties, each
compiled:

**The wrong pairing has no spelling**, because the kind is projected out of the numeral rather than supplied
beside it. Forcing a marker that carries only the fixed-point form against a float numeral:

```
error[E0277]: the trait bound `FixedOnly: Policy<base::Ranged<...>>` is not satisfied
help: the trait `Policy<base::Ranged<...>>` is not implemented for `FixedOnly`
      but trait `Policy<base::Implicit<_, _, _>>` is implemented for it
    = help: for that trait implementation, expected `base::Implicit<_, _, _>`, found `base::Ranged<...>`
note: required by a bound in `Number`
```

**Over-keying is refused by coherence rather than by review**, attempted in the declaring crate so the orphan
rule does not mask it:

```
error[E0119]: conflicting implementations of trait `Lowering<base::Implicit<base::ENeg<base::I<base::H>>,
base::BZero, base::BZero>>` for type `Warm`
35 | impl<E: Exponent, A: Adjustment, B: Bias> const Lowering<Implicit<E, A, B>> for Warm {
   | ------------------------------------------------------------------------------------ first implementation here
92 | impl const Lowering<Implicit<ENeg<I<H>>, BZero, BZero>> for Warm {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Warm`
```

**Same-kind numerals share a row by construction**, because there is no per-numeral impl to disagree at, and
a projection through a second `Implicit` numeral resolves to the same type. This is the property the numeral
spelling cannot have.

### 2.4 Where I read the rule differently from file 119, and it matters for what the canon claims

`119:333-338` applies `110:478`'s primary clause directly: "a fact is keyed on the coarsest layer whose
identity its truth depends on", therefore the exponent form. `119:539-541` then says the rule "decides the
keying without needing a new decision".

**Read on its own text the primary clause does not reach this.** It is stated over layers of the identity
tower, and all four of its own instances are such layers (`110:480-486`): `TotalOrd`'s level fork, the
spectral NaN defect, the digest's datum-keyed against value-keyed pairing, and carrier identity. The exponent
form is not one of those. What fires here is the rule's **dual**, `110:4546-4550`, and the dual as stated
carries a determination requirement and no coarseness clause at all.

That distinction is not pedantry, because the two readings license different claims:

- **Determination alone** admits the numeral, the exponent form, and anything between. On that reading `119`
  has a preference and not a derivation, and `118` was right to decline.
- **The rule taken whole**, with both failure modes live at once, admits exactly one. Keying on the preset
  alone does not determine, which is the dual failure. Keying on the numeral is too fine, which is the
  primary failure and which `110:4546-4547` calls a false statement. The exponent form is the unique key
  avoiding both.

**So the answer is `119`'s and the derivation is not.** I would put the two-failure-mode form in the canon,
because it is what the document's own text supports, and because a future member checking `119`'s sentence
against `110:478` will find the clause does not say it and may conclude the keying was assumed.

And there is a route that needs no reading of the rule at all, which is the one I would actually lean on:
**`ExponentForm` is sealed with exactly two constructors and there are exactly two tables**, so the form is
the partition itself. Nothing coarser determines the row, since the only thing coarser is the trivial
partition and that is the present defect. Anything finer over-keys. That argument survives any reading of the
word "layer".

### 2.5 The spelling, where file 119 mints a name and does not need to

`119:405` writes the bound `Policy<FormOf<N>>`. **`FormOf` returns zero hits in `110`**, searched fresh
2026-08-05, so it is a name that would have to be declared, and `110:554-556`'s widened
definitional-completeness line requires that of the name being defined. `119:436-441` is explicit that it
mints nothing and considers and rejects a `Kind` projection on exactly that ground, so this is an oversight
in the spelling rather than a disagreement about the design.

`N::Exponent` is the associated type already declared at `110:3086` and needs nothing:

```rust
pub struct Number<N: Numeral, S: Policy<N::Exponent> + Lowering<N::Exponent>> {
    datum:    <S as Lowering<N::Exponent>>::Container,
    _numeral: PhantomData<N>,
}
```

Exit 0, and `Crosses` keeps its supertrait edge as `Crosses<N: Numeral>: Lowering<N::Exponent>`.

### 2.6 The cost to a consumer, which the dispatch asked for specifically

**At a concrete position, nothing.** `Number<U13F3, Warm>` is spelled exactly as before, and the four
semantic aliases expand onto it unchanged, so `UFixed<13, 3, Warm>` is untouched and D48 and D31 are
satisfied. That is the half `110:403-406` asks for, op's own: invisible for the most part to downstream
consumers while doing real work underneath.

**The cost lands one layer up, on a crate generic over both parameters.** An algorithm crate's signature
gains the projection twice:

```rust
pub fn generic_consumer<N: Numeral, S>(x: Number<N, S>) -> Number<N, S>
where S: Policy<N::Exponent> + Lowering<N::Exponent>
{ x }
```

Compiled, exit 0. That is the whole of the consumer-side price and it is paid by the author of an algorithm
crate rather than by anyone writing a number. **It is worth stating rather than waving at**, because the
crates that pay it are `arvo-graph`, `arvo-sparse`, `arvo-comb` and `arvo-spectral`, which `110:2524` already
requires to be generic over trait bounds rather than over concrete numeric types.

**Two members become restated per kind**, `Layout` and `Encoding`, which `119:474-480` states and does not
hide. I agree with its disposition: a restated cell is a redundancy and the current shape is a falsehood, and
those are not comparable costs. Splitting `Lowering` into a kind-keyed and a kind-free half would add a
contract to a design whose contract count is the question `120` is answering, and it fails `16d:14-15`'s
tiebreaker.

### 2.7 It does not disturb the contract split, re-run rather than cited

`120`'s load-bearing refusal is that a bound on `Policy` does not project a `Lowering` member. Under the
parameterisation, in the strongest form available, with `Lowering` fully in scope in the same crate:

```
error[E0220]: associated type `Layout` not found for `S`
  --> pF2.rs:95:10
   |
95 | where S::Layout: IsDense {}
   |          ^^^^^^ there is an associated type `Layout` in the trait `Lowering`
help: consider further restricting type parameter `S` with trait `Lowering`
```

Identical code, identical help line. Nothing here adds a supertrait, so `120`'s prohibition is untouched, and
`120:464-468`'s bound sentence needs the projection added and nothing else.

---

## 3. The `Precision` collision, decided

### 3.1 It refuses at the declaration, not only at the use

`121` found `E0782` at the use site and correctly noted that rustc's `dyn` repair is unavailable to arvo.
Declaring both under the one token, which is what a canon crate would do, refuses one step earlier:

```
error[E0428]: the name `Precision` is defined multiple times
  --> pH_precision_collision.rs:28:1
   |
24 |   pub trait Precision: Nat {}
   |   ------------------------ previous definition of the trait `Precision` here
...
28 | / pub type Precision<const I: u16, const F: u16> =
   | |________________________________________________^ `Precision` redefined here
   |
   = note: `Precision` must be defined only once in the type namespace of this module
```

`119:299-311` reached the same place independently. **That changes the item's status**: it is not a citation
defect at one paragraph that could be left standing with a note, it is a pair of declarations the type
namespace forbids, so something has to move.

### 3.2 The two branches are not symmetric, so this is not a free naming call

`121:186-187` and `110`'s open list both treat this as two one-line naming options and therefore as op's.
**One of the two collides with ratified text and the other does not.**

`Precision` is one of three named semantic aliases over the sealed carrier, declared as a family at
`110:3176-3178`, and that family exists because of **`74b`, where op adopted one sealed bottom carrier for
capacity with `Capacity` kept as a named alias over it, on his own stated condition that "the mechanism
unifies and the vocabulary does not"** (`110:4826-4828`). The three names are that vocabulary. Renaming
`Precision` rewrites the bound on `Numeral::Precision` (`110:3085`), the bound on `FieldLayout::Extent`
(`110:3191`), and the family's own shape, and buys nothing the other branch does not.

`16d:14-15` is the clause for exactly this position, and it is op's: where the current shape can be kept it
should be, and rewrite cost is the tiebreaker between designs otherwise equal against the intent. Here they
are not even equal, since one branch touches a ratified vocabulary.

**So: the marker trait keeps `Precision`. The bridge's result types take names of their own**, `NatOf` at one
argument and `PrecisionOf` at two. Compiled with the document's own load-bearing assertion, and with the
marker trait still applying to the result, which is the reason both names have to coexist at all:

```rust
pub type NatOf<const I: u16> = <Idx<{ I }> as AdmittedWidth>::Nat;
pub type PrecisionOf<const I: u16, const F: u16> =
    <NatOf<{ I }> as NatAdd<NatOf<{ F }>>>::Out;

const _: () = assert!(<PrecisionOf<13, 3> as Nat>::VAL == 16);
fn takes_a_precision<T: Precision>() {}
fn both_names_live() { takes_a_precision::<PrecisionOf<13, 3>>(); }
```

Exit 0.

**What remains op's is the word and not the fork.** Whether `PrecisionOf` is the spelling sits inside D56's
ratified naming rule (`110:3634-3639`) rather than beside it, and it can be answered with the `Exponent` and
`Dec` calls in one sitting, which is what `110`'s own list already proposes for those two.

### 3.3 The braces sentence: both prior readings are half right, and the cause is structural

`110:3327-3328` records that a const forwarded into the table's index position needs braces or rustc reports
`E0747`. `119:313-320` reports that this "does not reproduce in this spelling" and asks that the design not
carry a required-braces sentence.

**Both are right about their own probe and neither has the cause.** The braces are required exactly when the
const parameter's name also names a type in scope. In a crate holding the sealed carrier:

```
error[E0747]: type provided when a constant was expected
  --> pJ_collision.rs:25:37
   |
25 | pub type NatOf<const I: u16> = <Idx<I> as AdmittedWidth>::Nat;
   |                                     ^
   |
help: if this generic argument was intended as a const parameter, surround it with braces
```

Renaming the const parameters so they do not collide compiles unbraced, exit 0, which is why `119`'s probe
saw nothing.

**And the collision is not an accident of my transcription, it is this design's own vocabulary.** The public
fixed-point spelling calls its integer width `I` (`110:3311`, D48 and D31, ratified), and the sealed carrier
calls its odd constructor `I<P: Pos>` (`110:3148`). `119:164-167` concludes, correctly, that the bridge table
lives in the crate that declares that carrier. **So the two `I`s are in one scope by the design's own
placement decision**, and the braces are required there by default rather than by accident.

Two remedies, both compiled: brace the argument, or name the bridge's const parameters so they do not
collide. Which one is a naming matter under D56 and goes with the item above. **What the design should carry
is the cause**, because a reader who takes "braces are required" as a general rule will write them
everywhere, and a reader who meets `E0747` in a different spelling will not find the reason here.

---

## 4. What I landed in `110`, and on whose agreement

Seven marked insertions, 443 lines, no deletions, each in the form the file already uses and none silently
overwriting. Every superseded paragraph is left standing as the audit trail of what was asked when.

**The three-way contract split stays** (section 1.25). `117` and `120` reached it independently, `120`
forming and compiling its answer before opening `117` (`120:10-13`). All five of `120`'s canon sentences are
folded in as its author drafted them, with the perimeter's wording kept because a guarantee whose limits are
unstated is a claim rather than a guarantee. **The item moves off the loudest-for-op list as a live fork and
onto it as a one-line ratification of the incumbent shape**, since nothing that ships moves either way.

**`S: Policy + Lowering` is the bound** (section 1.1). `118` wrote it and named itself the first read;
`120:386-390` supplied the second and the reason the record lacked, that the conjunction is what makes the
weaker bound `S: Policy` enforceable. Still owed op's word, as confirmation rather than as a question with no
answer in the record.

**The supertrait prohibition, stated directionally** (section 1.25). `117` claimed it partially through
`S::Layout`, which still needs a lowering name and so reads as a partial leak; `120` compiled it total
through `Container`, which projects to a primitive and needs no import. I reproduced the total form
independently: two strategies with identical `Policy`, differing only in layout, disagreeing on whether
addition is associative, from a crate naming nothing from the lowering vocabulary. Both directions compiled:
`Lowering: Carrier` refuses `S::Layout` off a `Carrier` bound at `E0220`, `Carrier: Lowering` projects it at
exit 0.

**The precision bridge and its emission locus** (section 1.23). `118` and `119` built it independently and
reached the same mechanism. Landed with `119`'s forcedness argument, its orphan-rule locus, and the
`macro_rules!`-not-`build.rs` conclusion that `16b:50-53` and `110:2434-2437` decide between them. **One
strengthening is mine**: `119:222-224` says the downstream assertion also fires, so the table's correctness
does not rest on the per-row assertions alone. That holds for an uncompensated corruption and not in general.
A row corrupted **together with a compensating change in the addition impl** passes the downstream assertion
at exit 0 and is caught only at the row:

```
error[E0080]: evaluation panicked: assertion failed: <<Idx<13> as AdmittedWidth>::Nat as Nat>::VAL == 13
```

So the per-row assertions are not redundant, and that is a reason to write them into the canon's statement of
the table rather than leave them as an implementation nicety.

**The preset key** (sections 1.1 and 1.21), on `119`'s and my own agreement, with the derivation stated in
the two-failure-mode form and the bound spelled `N::Exponent`.

**The `Precision` spelling** (section 1.23), decided as above.

**The assembly prerequisites** (section 1.23). `121` found that the document's eight Rust blocks reach exit 0
only with a feature gate, an import, and a `notko::ConstTry` stand-in, and recorded them at section 9.
**Section 9 is the wrong place for a reader to meet them**, since a reader assembling blocks starts at
section 1.1 and would hit twenty-nine errors first, so they are now stated at the first block that needs
them. Reproduced independently on a superset of the declarations: dropping the gate gives 67 `E0658`,
dropping the import gives 9 `E0425`.

---

## 5. What goes to op, and what does not

**Two items narrow rather than stay as written**, and both are on `110`'s section 2 list in that form:

The **preset key** was "two spellings, section 1.21 states both and picks neither". Both are now refuted by
compiled diagnostics, two members independently reach the third, so what is owed is **confirmation of the
survivor** rather than a pick.

The **`Precision` collision** was "two spellings, both naming calls, both op's". One branch collides with
`74b` and with `16d`'s tiebreaker, so what is owed is **the word `PrecisionOf`**, inside D56.

**One item is genuinely new and it is one sentence, addressed to op.** `119:234-264` found it and I reproduce
the gap rather than the mechanism:

> **Which total widths may a consumer write, and is their sum one of them?** The bridge table bounds each
> width and not their sum, because the sum is produced by the tower's own addition and never touches the
> table, so `UFixed<40, 30, Warm>` compiles today at a total width of 70 with nothing anywhere admitting 70.
> The bounded quantity cannot be the precision: `mulnum` sums the operand precisions, so a product's
> precision legitimately exceeds any width a consumer wrote, and a ceiling on `Precision` would refuse a
> legitimate intermediate. The two are different facts and the bridge checks neither. One sentence naming the
> admitted range for a written total width decides it; the marker that enforces it is one extra line per row.

**Nothing else here is op's.** The split, the bound, the prohibition and the bridge each have two independent
agreements. The `Precision` branch resolved on ratified text rather than on taste. The braces cause is a fact
about the compiler and this design's own vocabulary. The assembly prerequisites are bookkeeping.

---

## 6. The honest limit

**Nineteen probes, every diagnostic verbatim**, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`--edition 2024 --crate-type=lib`, in a scratch directory outside the repository. **Run from outside the
tree a bare `rustc` resolves to stable `1.94.0`** on this host, which `110:6250-6253` records as a trap four
files have hit, so every invocation passed `+nightly-2026-05-28` explicitly.

**My tower is a transcription of `110`'s declarations and not the design's source**, since there is none. It
carries the sealed carrier, both `ExponentForm` constructors, three concrete numerals and both preset rows,
and it compiles; the tower's own addition is stubbed at the one instance the assertion needs, because
`Sum` is on the open list (`118:302-305`). So the shape transfers and the exact spelling of the arithmetic
does not.

**The preset probes model the tables' rows and not their content.** `Quantisation` is a five-member product
and the far point is a marker, which is enough to key the rows and says nothing about what the rows mean. And
**the whole of section 2 assumes the two tables at `110:2670-2729` are the tables.** They are ratified at
`70b`; if op moves a cell the keying argument is unaffected and the impl bodies are not.

**One thing I checked and one I did not.** I checked that the parameterisation preserves `120`'s refusal,
because a keying answer that breaks the split's guarantee is not an answer. I did not check whether the five
rows both tables state are the only members that vary by kind: `Encoding`'s own members are on the open list,
and if one of them is kind-dependent the impl count grows without the shape moving. `119:551-554` names the
same gap. The check is cheap and belongs with whoever writes `FieldLayout`'s members.

**And a compiler establishes that what is written is well formed and has no opinion on whether it is the
right thing to write.** Everything above says the exponent form is the only key that survives the document's
own rules and the two ratified tables. It does not say the two tables are right, and `70b` is where that
question lives.
