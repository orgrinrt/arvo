# The precision bridge and the preset key: the second reads, and one spelling neither of us offered

**Persona:** Oleg Kiselyov, typed type-level programming and staged computation lens. Dispatched as the second
independent read on two of the three items `118:317-331` names as owed, its item 2 (a preset's row is a
function of a pair) and the bridge half of its section 7.
**Date:** 2026-08-05

**Reading order, stated because the dispatch made it a condition.** I read `110_consolidation_eleven.md` in
full at its structural sections and reasoned both questions out, then compiled my answers, and only then
opened `118_the_missing_declarations.md`. That order is recoverable from this file: every refutation in
sections 1 and 2 is a probe I built against `110`'s own declarations, and where `118` reached the same place
I say so rather than presenting it as mine.

**Line numbers are pinned to `ee027e1`**, the commit whose message is "docs: panel file 118, the missing
declarations", for the same reason `120:19-27` pins to it. The working tree is being edited concurrently by a
dispatch writing blocks marked "Correction, file 121", and `110` in the tree is longer than the 6,283 lines
at the tip. Two of the things I found are already recorded there, and I credit them below rather than
claiming them.

**Both gates pass, and the second one is worth one honest sentence rather than a paragraph.** The canon gate:
the governing material is op's own, `13c`'s standard, the four standing directives at `110:119-210`, and
`108b:184-186`'s order to work the open list down rather than open ground, which is what a second read on two
open-list items does. Nothing here touches `mock/crates`, per `108b:188-193`. The test gate: I ran
`cargo test --workspace` from the tree rather than citing anyone, 2026-08-05, and summed per binary:
**155 binaries, 672 passed, 0 failed, 9 ignored**, which reproduces what `118:47-49` reports. I did not
re-audit the test bodies, for the reason `118:51-56` gives and I agree with: op ruled at `108b:174-181` that
the collected tautologies are an implementation-phase checklist and not something to act on now, and a fifth
report of the same three findings is what that ruling exists to stop. A green suite over a tree the canon
replaces is the weakest signal in this room, and the instrument that actually measured something here was the
compiler, thirty-four times.

---

## 1. The precision bridge

### 1.1 What I reached before opening the first read

The gap is stated exactly at `110:3247-3252`. D53 makes `UFixed` a name for a composition, D48 and D31 keep
the public spelling with the widths as const parameters (`110:3239-3241`), and the numeral inside takes
`Precision` as a `Nat`, which `110:3104` declares as a named semantic alias over the sealed inductive carrier.
So a value known at the call site has to become a type, and `I + F` in type position is the const expression
`110:864-875` has already compiled shut in every permitted direction.

My first question was not how to bridge it. It was whether the bridge is necessary, which is a different
question and the one the design's own economics turn on. It is necessary if and only if `Precision` has to
participate in type-level arithmetic whose **result is a type**. It does: `110:391` makes the width chain the
founding instance of the spine rule, op's own at `44b`, and `110:864-866` states that every exponent position
is computed by `mulnum` and has to appear in the result numeral's type. A product's precision is not read, it
is inhabited. So the const column that `110:3831-3845` prices for capacity is unavailable here for the same
reason it is unavailable there, and `110:3847-3849` states that reason in one sentence: the tower needs
type-level arithmetic producing types, and a const parameter does not participate in it.

That settles that a bridge exists. It does not settle that a table is the bridge, and that is where I spent
the compiles.

### 1.2 The table is forced, not chosen, and this is the part the first read leaves open

`118:290-292` says "what is open is not whether the bridge exists but whether it is the design's answer". The
honest answer is stronger than either of us started with. **Under the permitted feature set there is no other
spelling.** A value-to-type escape over a const parameter needs a case split on that parameter, and an impl
is the only case-split mechanism Rust has over a const. Three routes exist on paper and all three are refused,
compiled under the pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024 --crate-type=lib`.

**Route one, structural recursion on the const.** Halve it, recurse, build the numeral bit by bit, one impl
instead of a table. Two independent refusals from one file, and the second is the deeper one:

```
error: complex const arguments must be placed inside of a `const` block
  --> e2a.rs:19:11
   |
19 |     Idx<{ N / 2 }>: ToNat,

error[E0119]: conflicting implementations of trait `ToNat` for type `Idx<0>`
  --> e2a.rs:17:1
   |
16 |   impl ToNat for Idx<0> { type Out = Z; }
   |   --------------------- first implementation here
17 | / impl<const N: u16> ToNat for Idx<N>
```

The first is `min_generic_const_args` refusing the arithmetic. The second holds **even if the arithmetic were
free**: the base case and the recursive case overlap, because `Idx<0>` is an instance of `Idx<N>` and nothing
distinguishes them at the impl level.

**Route two, hide the arithmetic behind a `type const` path.** `110:3763-3765` establishes that a path is not
an expression and resolves under `min_generic_const_args` where an expression does not, which is the one real
opening in the wall. It does not open here:

```
error: complex const arguments must be placed inside of a `const` block
  --> e2b.rs:15:62
   |
15 | impl<const N: u16> Halve for Idx<N> { type const HALF: u16 = N / 2; }
```

A `type const` may be a path to a constant. **Its own body may not compute one from a generic parameter.** So
the opening `110:3763` correctly identifies carries a value *out* of a type and never carries a computed value
*in*, and the two directions are not symmetric. I record this precisely because a future member reading
`110:3763-3771` alone would reasonably guess otherwise.

**Route three, separate the base case with specialisation.**

```
error[E0658]: specialization is experimental
  --> e2c.rs:10:39
   |
10 | impl<const N: u16> ToNat for Idx<N> { default type Out = Z; }
   = help: add `#![feature(specialization)]` to the crate attributes to enable
```

A defaulted associated **type** needs full `specialization`, which `unstable-features.md` forbids outright and
which `110:3513-3515` names as one of the two bans the model-width transfer argument rests on. So the ban that
makes a check at eight bits mean anything at sixty-four is the same ban that forces the table. That is not a
coincidence worth admiring, it is one property with two consequences: an implementation that cannot observe
which instantiation it is in also cannot dispatch on a const without enumerating it.

**Conclusion.** The generated table is the design's answer, and I would put it in the canon as forced rather
than as chosen, with the three diagnostics above as its ground. `110:3756-3760` shows what happens when a
forcing claim is asserted without them: the array grammar's "forced by the language" was ratified and is false
twice over. This one is true, and it is true for a stated reason that a future member can re-run in ninety
seconds.

### 1.3 What the table costs, measured

Built as one crate under the pin, tower plus table plus the load-bearing assertion, walltime from
`/usr/bin/time -p`, three runs each after the first, spread under 10 ms:

| table rows | no per-row check | with per-row check |
|---|---|---|
| 256 | 0.04 s | 0.07 s |
| 1024 | 0.13 s | 0.25 s |
| 4096 | 0.93 s | 1.44 s |

Linear in the row count at roughly 0.35 ms per row with checks, and the checks cost about 55% on top. Against
the pricing pillar at `110:456-464` this is nothing, and against `110:3295-3298`'s own cost model it is two
orders of magnitude under the marginal cost of a single dyadic composition. **The table's size is not a free
parameter and should not be priced as one**: section 1.4 shows it is exactly the range of publicly writable
widths, so the row count is a design statement rather than a tuning knob.

The one number worth watching is not in the table. The tower's `Pos` is binary, so a row's numeral literal has
depth `log2(N)` and the recursion-depth ceiling `110:5666` mentions is nowhere near. Nothing here grows
super-linearly.

### 1.4 Where the table is emitted, which the orphan rule settles rather than taste

This is the part `118:291-292` left open and `110:3258` calls a binding-time question. It is narrower than
that, because one compile removes most of the freedom.

**A downstream crate cannot add a row.**

```
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
 --> locus/consumer.rs:5:1
  |
5 | impl NatIndex for Idx<13> { type Out = Z; }
  | ^^^^^^^^^^^^^^^^^^-------
  |                   |
  |                   `Idx` is not defined in the current crate
```

Three consequences follow and I would write all three down.

**The table lives in the crate that declares the tower.** Both the index type and the indexing trait are the
tower's, so both are foreign everywhere else, and the impls have to sit beside them. Under D72's crate table
(`110:3355-3362`) that is `arvo-numeral`, which already holds `Bias` and therefore already holds the sealed
carrier; if the shared bottom carrier crate `110:3726` proposes is created, the table follows the carrier and
not the numeral. It cannot live in `arvo-numeric` with the aliases that consume it.

**It is emitted complete, once, at that crate's compile.** There is no lazy or per-consumer variant, because
the only mechanism for a per-consumer variant is a downstream impl and that is the diagnostic above.

**It is emitted by a macro in that crate, not by a build script**, and this is the one place the design's own
rule decides a question I would otherwise have argued. `110:3337` and `16b:50-53` state that arvo grows no
build harness of its own, and `110:2434-2437` states the positive form: "the macro is not a convenience
wrapped around the design; it is the design's own binding-time decision, made explicit and paid for exactly
once, at exactly the stage that has the information cheapest." A table of literal impls over a fixed integer
range is `macro_rules!` work at the crate root; `macro_metavar_expr_concat` is already on the allowed list and
already used by two shipped crates. Nothing here needs `build.rs`, `OUT_DIR`, or a proc macro, and reaching
for one would be arvo acquiring exactly the harness the design refuses.

**The downstream contract, since `16c:31-53` asks every member for one rather than an observation.** What a
build layer or a code generator reads out of this: the table is a total function from an admitted width to a
sealed numeral, and it is resolved entirely by the trait solver before monomorphisation, so **nothing
downstream sees it at all**. There is no per-width symbol, no runtime table, no relocation, and no artifact a
linker can observe. What the design needs back from the downstream target is likewise nothing, which is the
correct answer to state rather than a gap to apologise for. The only downstream-visible consequence is the
diagnostic in section 1.7, which is a compile-time artifact of the front end and not of any lowering.

### 1.5 Whether the bridge preserves the model-width transfer, which neither read checked

The dispatch was right to make this a question. A table maps each width to a **different type**, which is
precisely "a type observing which instantiation it is in", the third hole `110:620-644` names, that op flagged
at `12b:46-54` and the review rediscovered forty files later. So the bridge is in the same class as the
container-class dispatch, and the class matters. Two things separate them and both are checkable.

**The bridge changes which value a type-level numeral denotes. It does not change which code runs.** The
container-class hole is dangerous because the projected container has genuinely different arithmetic: a
property true at eight bits (`u8` wraps on doubling 200) is false at nine. The table's output feeds bodies
that are parametric over `N: Nat`, so the same body runs at every instantiation. The bridge composes with the
container-class hole downstream and adds no second one.

**The bridge's own claim is per-row and does not transfer by any of the four grounds**, so under
`110:589-591` it defaults to `unargued`, which is exactly the honest default and exactly the wrong resting
place for a table with a thousand rows. It does not have to rest there. Emit each row with its own agreement
assertion beside it, and the claim becomes machine-checked by construction, which is bin one of the four-bin
ledger at `110:3506-3509` rather than bin two:

```rust
impl AdmittedWidth for Width<13> { type Nat = Pz<I<O<I<H>>>>; }
const _: () = assert!(<<Width<13> as AdmittedWidth>::Nat as Nat>::VAL == 13);
```

The negative control fires. Corrupting one row of a 256-row table, `Pz<I<O<I<H>>>>` to `Pz<O<I<I<H>>>>`:

```
error[E0080]: evaluation panicked: assertion failed: <<Idx<13> as NatIndex>::Out as Nat>::VAL == 13
   --> bad.rs:135:15
error[E0080]: evaluation panicked: assertion failed: <PrecisionOf<13, 3> as Nat>::VAL == 16
   --> bad.rs:622:15
```

Two things about that pair are worth keeping. The row's own assertion catches it at the row, naming the wrong
row. And the downstream claim fails too, which means the table's correctness is not resting on the assertions
alone.

The spelling is not a new invention. It is the free anonymous const item `110:3745-3754` already compiled for
the capacity and bitfield findings, in the position that section names as the one that works: "available
exactly where the design owns the declaration site, which a macro does and a consumer-instantiated generic
does not." The table is a macro-owned declaration site. **The design's own already-compiled result supplies
the bridge's verification spelling**, and I would write the assertion into the canon's statement of the table
rather than leaving it as an implementation nicety, because a table emitted without it is a thousand
`unargued` claims wearing a compiled-artifact costume.

### 1.6 What neither read checked, and it is a hole in the bridge as specified

The table bounds each width. **It does not bound their sum**, because the sum is produced by the tower's
addition and never touches the table. Compiled against my own bridge with a 64-row table:

```rust
pub fn bad(_x: UFixed<40, 30, Warm>) {}   // total width 70, exit 0
```

Both 40 and 30 are admitted, `SumOf<40, 30>` is the tower's numeral for 70, and nothing anywhere says 70 is
admissible. My first `on_unimplemented` note said "the sum I + F must also lie in that range" and the bridge
did not enforce the sentence I had written on it, which is the failure mode this review names most often.

The closure is one extra emitted line per row, a marker on the tower's numeral rather than on the index, and
it compiles:

```
error[E0277]: the trait bound `Pz<O<I<I<O<O<O<H>>>>>>>: AdmittedPrecision` is not satisfied
   --> ceiling.rs:511:25
    |
511 | pub fn over_ceiling(_x: UFixed<40, 30, Warm>) {}
note: required for `FixedNumeral<Pz<O<I<I<O<O<O<H>>>>>>>>` to implement `Numeral`
```

**I am not proposing the ceiling, because the design has not stated one and inventing it here would put a
decision nobody made into the canon's voice.** The question I am handing over is prior to the mechanism:
`Precision` legitimately exceeds any consumer-written width during a product, since `mulnum` sums the operand
precisions (`110:864-866`), so a hard ceiling on `Precision` would refuse a legitimate intermediate. So the
bounded quantity is the publicly writable total width and not the precision, the two are different facts, and
the bridge currently checks neither. **What is owed is one sentence saying which widths a consumer may write
and whether the sum is one of them**, and the mechanism follows from the sentence in one line either way.

### 1.7 The diagnostic, since the consumer never wrote any of this

Raw, at a width outside the table:

```
error[E0277]: the trait bound `Idx<200>: NatIndex` is not satisfied
help: the trait `NatIndex` is not implemented for `Idx<200>`
    = help: the following other types implement trait `NatIndex`:
              Idx<0>
              Idx<10>
              ... and 56 others
```

`#[diagnostic::on_unimplemented]` reaches it, which is worth stating because `110:2430-2433` records that the
same attribute does **not** reach `E0275` on the `Reduce` trait. Here it does, and the first line becomes
"width `Idx<200>` is outside the widths arvo admits" with two notes carrying the range and the remedy. Two
residues survive and cannot be suppressed: the `help:` line naming `NatIndex`, and the eight-example list.

The sharper cost is the ceiling case in section 1.6, where the unsatisfied bound is on a tower numeral and the
consumer reads `Pz<O<I<I<O<O<O<H>>>>>>>` after writing `UFixed<40, 30, Warm>`. The message line and the notes
are recoverable; **the encoding in the `help:` line is not, because the type genuinely is that type and no
attribute renders it as 70.** So the honest statement of the bridge's price is: invisible when it succeeds,
and at its most visible exactly when the consumer has made the one mistake it exists to catch. Against
`110:359-362`'s consumer-facing half of the bar ("invisible for the most part to downstream consumers") that
is a real cost and it belongs in the canon's statement of the table rather than in a footnote.

### 1.8 One defect in the first read's spelling, already found by a concurrent dispatch

`118:288` and `110:3252-3254` spell the bridge's result `Precision<13, 3>`. `110:3104` declares
`pub trait Precision: Nat` with a blanket impl. The concurrent file 121 records the use-site half at
`110:3333` in the working tree, `E0782`, "expected a type, found a trait", and correctly notes that rustc's
`dyn` suggestion is unavailable to arvo. Credit there rather than here.

The half worth adding is that the collision refuses one step earlier. Declaring both in one crate, which is
what the canon would do:

```
error[E0428]: the name `Precision` is defined multiple times
 96 |   pub trait Precision: Nat {}
    |   ------------------------ previous definition of the trait `Precision` here
105 | pub type Precision<const A: u16, const B: u16> =
    = note: `Precision` must be defined only once in the type namespace of this module
```

So it is not a bad citation at a use site, it is a declaration the trait table forbids. This is address one of
`110:3577-3582`'s three, one token with two ratified meanings, and it is the third instance in this document.

**One smaller correction while I am here.** `110:3254-3257` records, as a spelling detail worth keeping, that
a const forwarded into the table's index position needs braces or rustc reports `E0747`. **It does not
reproduce in this spelling.** `pub type NatOf<const N: u16> = <Idx<N> as AdmittedWidth>::Nat;` and
`<NatOf<A> as NatAdd<NatOf<B>>>::Out` both compile unbraced, exit 0, because a single-segment path resolving
to a const parameter is accepted in const-argument position. The braced form also compiles. I am not claiming
the original observation was invented; I am recording that the design should not carry a required-braces
sentence, because a reader who takes it as required will write braces everywhere and a reader who hits `E0747`
in some other spelling will not find the real cause here.

---

## 2. The preset key

### 2.1 The reading I formed from the document's own rule

`110:2674-2682` states the fact and I agree with it entirely: one preset name denotes two rows, the two
ratified tables differ at four cells, and `Policy::Quantisation` and `Lowering::StoredWidth` are nullary
associated types on the marker, so `Warm::StoredWidth` is one type where the tables require two. `110:4424-4427`
names the class: not a false statement but a non-function presented as one.

The question I asked was the layer-keying rule's own question, `110:436`: **a fact is keyed on the coarsest
layer whose identity its truth depends on.** So: what determines the row? Not the numeral. Not the precision,
the radix, the domain, the bias. The tables split on fixed-point against float, which in this design is the
exponent form, `Implicit` against `Ranged`, and `110:3109` declares `ExponentForm` sealed with exactly those
two constructors. **The coarsest layer whose identity the row's truth depends on is the exponent form, and
nothing finer.**

That reading rules on both spellings `110:2684-2688` offers before either is priced, and it rules against both.
Then I compiled the rulings, because a rule applied by argument is a preference.

### 2.2 The per-kind marker spelling admits a well-typed nonsense. Compiled.

The second offered spelling keeps the four names nullary and maps each to a per-kind marker inside the
semantic aliases. Modelled minimally, with a float numeral wearing the fixed-point preset:

```rust
pub struct WarmFixed;  impl Lowering for WarmFixed { type StoredWidth = DoubleLogical; }
pub struct WarmFloat;  impl Lowering for WarmFloat { type StoredWidth = Minimum; }
pub type IllFormed = Number<Binary32, WarmFixed>;
pub fn takes_it(_x: IllFormed) {}
```

**Exit 0.** A float numeral carrying the fixed-point preset's storage row is a type this spelling admits.

That is fatal under a ratified decision rather than under my taste. D52 (`110:3261-3264`) is explicit:
"Compositions are public and bindable by anyone; semantic names and strategy presets are the default
documented path, not the only path", and `110:3265-3266` draws the consequence, that the two preset tables are
four documented points and not the surface. A consumer exercising the licence D52 grants has, under this
spelling, eight markers, no rule relating them to numerals, and a compiler that accepts every one of the
sixteen pairings. The aliases hide it only for the four the aliases cover.

It has a second cost that is smaller but is op's own text: D72 gives `arvo-strategy` "`Hot`, `Cold`, `Warm`,
`Precise`, and nothing else" (`110:3360`). Eight markers is a different crate.

### 2.3 Keying the contracts on the numeral over-keys, and the over-keying is not detectable. Compiled.

The first offered spelling takes the numeral, `Policy<N>` and `Lowering<N>`, on the precedent of
`Crosses<N: Numeral>`. It is sound and it costs nothing at the consumer's call site. What it permits is the
problem:

```rust
impl Lowering<Fix13_3> for Warm { type StoredWidth = DoubleLogical; }
impl Lowering<Fix7_9>  for Warm { type StoredWidth = Minimum; }
```

**Exit 0.** Two fixed-point numerals, same kind, disagreeing on what `Warm` means. Under this spelling
"`Warm` stores doubled for fixed-point" stops being a statement anyone can make, because `Warm` is no longer a
preset, it is a per-numeral lookup table that currently happens to be constant on each kind. The
`Crosses<N: Numeral>` precedent does not carry over: a crossing statement genuinely depends on the individual
numeral's value set, and a preset row does not.

This is the layer-keying rule failing in its original direction, keyed too finely, which `110:436-442` calls a
false statement. Trading one direction of the rule's failure for the other is not a repair.

### 2.4 The third spelling: key on the exponent form, and coherence enforces the rule

Key both contracts on the exponent form, and quantify each impl over that form's own parameters:

```rust
pub const trait Policy<F: ExponentForm>   { type Quantisation: Quantisation; }
pub const trait Lowering<F: ExponentForm> { type StoredWidth: StoredWidth;
                                            type Layout: StorageLayout;
                                            type Door: LoweringDoor; }

impl<E: Exponent, A: Adjustment, B: Bias> Lowering<Implicit<E, A, B>> for Warm {
    type StoredWidth = DoubleLogical;  type Layout = Dense;  type Door = Inert;
}
impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials>
    Lowering<Ranged<EMIN, EMAX, U, S>> for Warm {
    type StoredWidth = Minimum;        type Layout = Dense;  type Door = HostFloat;
}

pub struct Number<N: Numeral, S: Policy<FormOf<N>> + Lowering<FormOf<N>>>(PhantomData<(N, S)>);
```

Both of `110`'s ratified tables transcribe into this cell for cell, four presets by two kinds, and the whole
of section 1.21 compiles as sixteen impls. **Exit 0.** Three properties, each compiled rather than argued.

**The row is selected by the numeral and cannot be chosen.** `FormOf<N>` is a projection, so there is no
position where a consumer supplies a kind, and the well-typed nonsense of section 2.2 has no spelling.

```
error[E0308]: mismatched types
70 |     let _: <Warm as Lowering<KindOf<Fix13_3>>>::StoredWidth = Minimum;
   |            ------------------------------------------------   ^^^^^^^ expected `DoubleLogical`, found `Minimum`
```

**Over-keying is refused by the coherence checker, not discouraged by a rule.** Attempting to give one
particular fixed-point numeral its own row:

```
error[E0119]: conflicting implementations of trait `Lowering<Implicit<EOne, _, _>>` for type `Warm`
43 | impl<E: Exponent, A: Adjustment, B: Bias> Lowering<Implicit<E, A, B>> for Warm {
   | ------------------------------------------------------------------------------ first implementation here
64 | impl<A: Adjustment, B: Bias> Lowering<Implicit<EOne, A, B>> for Warm {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Warm`
```

This is the property I would put in front of op, because it is the difference between a rule and a mechanism.
Under section 2.3's spelling the layer-keying rule is a convention a reviewer has to enforce on every new
impl. Under this one **the rule is a coherence obligation**, and the only way to violate it is with
specialisation, which is forbidden. The rule stops needing anybody to remember it.

**Zero new vocabulary.** The exponent form is already declared, already sealed, and already has exactly two
constructors. I considered adding a two-element `Kind` projected off the form, compiled it, and it also works;
I am not proposing it, because the quantified-impl spelling gets the same keying with no name minted and gets
the `E0119` property, which the `Kind` spelling does not (two impls for `Kind = Fixed` on one preset are as
overlapping, but the finer-than-kind case becomes unstatable rather than refused, which is weaker evidence for
a future reader).

### 2.5 It does not disturb the concurrent second read on the split. Compiled.

`120:127-196` establishes the load-bearing typing fact for the contract split: a bound on `Policy` does not
project a `Lowering` member, `E0220`, independent of the dependency graph, and `120:453-470` writes it into a
proposed canon sentence. My proposal adds a parameter to both contracts, so it has to survive that result or
it is dead. Re-run under the parameterised, `pub const trait` spelling:

```
error[E0220]: associated type `Layout` not found for `S`
  --> converge120.rs:31:8
   |
31 |     S::Layout: StorageLayout,
   |        ^^^^^^ there is an associated type `Layout` in the trait `Lowering`
help: consider further restricting type parameter `S` with trait `Lowering`
29 | impl<N: Numeral, S: Policy<FormOf<N>> + Lowering</* F */>> AddAssoc for Fact<N, S>
```

Identical refusal, identical help line, and the help line even leaves the parameter hole visible. `120`'s
prohibition (no trait carrying both in its supertrait closure) is untouched, since nothing here adds a
supertrait. And the crate-edge story survives with one detail worth stating: the law crate's bound now mentions
`ExponentForm`, which lives on the identity side in `arvo-numeral`, not in `arvo-lowering`, so a crate
bounding `S: Policy<FormOf<N>>` still needs no lowering edge.

**So `120`'s sentence needs one edit and not a rewrite**: `Number<N: Numeral, S: Policy<FormOf<N>> +
Lowering<FormOf<N>>>`, with everything else in `120:453-470` standing as written.

### 2.6 What it costs, stated rather than skipped

**Impl count doubles, from four to eight per contract.** That is the two tables, so it is the design's own
content rather than an overhead; the current shape has four impls and cannot express either table.

**Two members become over-keyed.** `Layout` is `dense` for every preset except `Cold` in both tables, and
`Encoding` does not appear in either. Under the kind-keyed contract they are restated per kind. This is the
layer-keying rule biting in the other direction and I will not pretend otherwise. The distinction I would draw
is that **a restated cell is a redundancy and the current shape is a falsehood**, and the two are not
comparable costs. The alternative, splitting `Lowering` into a kind-keyed part and a kind-free part, adds a
contract to a design whose contract count is the open question `120` is answering, and it fails
`16d:14-15`'s tiebreaker.

**One line of ratified text changes**, the bound at `110:759`, which is the same one line both offered
spellings cost.

**What does not change.** The consumer's spelling, the four preset names, `arvo-strategy`'s contents under
D72, `RANK`'s ordering, the cross-strategy resolution rule, and every crate edge.

---

## 3. The two answers compose, in one file

Both questions are about the same declaration, so I built the answer as one artifact rather than two:
the sealed carrier, `Pos` addition as a carry chain, `Nat`, the bridge table with per-row assertions,
`Implicit` and `Ranged`, both preset tables at sixteen kind-keyed impls, and `Number` with the amended bound.
443 lines, `#![no_std]`, **no feature gate at all**, no `alloc`, no `dyn`, no `TypeId`, every size const.
Exit 0 under the pin, 0.21 s cold and 0.04 s warm.

What it asserts, in its own const positions:

```rust
const _: () = assert!(<<FixedNumeral<SumOf<13, 3>> as Numeral>::Precision as Nat>::VAL == 16);

let _: <Warm as Lowering<FormOf<FixedNumeral<SumOf<13, 3>>>>>::StoredWidth = DoubleLogical;
let _: <Warm as Lowering<FormOf<FloatNumeral<NatOf<24>>>>>::StoredWidth    = Minimum;
let _: <Hot  as Lowering<FormOf<FixedNumeral<SumOf<13, 3>>>>>::Door        = Inert;
let _: <Hot  as Lowering<FormOf<FloatNumeral<NatOf<24>>>>>::Door           = HostFloat;

let _: Option<UFixed<13, 3, Warm>>  = None;
let _: Option<StrictFloat<24, Warm>> = None;
```

The last two lines are the point. `UFixed<13, 3, Warm>` is the spelling D48 and D31 require, unchanged, and it
resolves end to end: the widths reach a type-level numeral of 16 through the table, and one preset name
reaches the fixed-point row while the same name on a float numeral reaches the other. The probes are in a
scratch directory outside the tree per the dispatch, and the generator is forty lines of Python, so the whole
artifact is reproducible from this file plus one command.

---

## 4. Where I agree with the first read and where I do not

**Agreed, and this is the second read `118:325-328` asks for.** A preset's row is a function of the pair, the
current nullary members cannot carry it, and this is the layer-keying rule's dual failure instantiated in
ratified text. I formed that from `110:436` and the two tables at `110:2598-2657` before opening `118`, and I
reached the same class name from `110:4424-4427` independently. **Two independent reads now agree on the
entailment**, each grounded in quoted text, which is what the workspace rule asks before a call is made. The
call itself is op's.

**Agreed.** The bridge exists, is a generated table plus the tower's addition, and compiles gate-free. My
tower is written independently of `118`'s and reaches the same place, which is corroboration of the mechanism
rather than of the file.

**Agreed and worth repeating rather than improving.** `118:19-21`'s discipline, that a plausible invention in
the canon's voice is indistinguishable from a decision. I have applied it to the width ceiling in section 1.6,
where the mechanism is compiled and the number is not mine to pick.

**Not agreed: that the two offered spellings are the field.** `118:276-280` and `110:2684-2688` state both and
pick neither, correctly, on the ground that the record contains neither. My position is that the record
contains something better than either: the layer-keying rule at `110:436` decides the keying without needing a
new decision, and both offered spellings violate it in opposite directions, each refuted by one compile
above. Declining to pick between two options is right when both are admissible. Here neither is.

**Not agreed: that where the table is emitted is open.** `118:290-292`. The orphan rule closes most of it and
`110:2434-2437` closes the rest. What remains genuinely open is the width range, which is section 1.6 and is a
different question from the locus.

**Neither of us checked**, and I have now: the transfer argument (section 1.5), the sum ceiling (1.6), the
diagnostic (1.7), the compile cost (1.3), and whether any bridge other than a table exists (1.2). The last is
the one I would not have expected to matter and it turned out to carry the whole verdict.

**And one thing I did not check.** Whether the four names in the tables are the only members that vary by
kind. I transcribed the five rows both tables state and found four differing cells, but `Encoding`'s members
are on the open list per `118:127-133`, and if any of them turns out to be kind-dependent the impl count grows
without the shape changing. The check is cheap and belongs with whoever writes `FieldLayout`'s members.

---

## 5. Offered for the canon, as suggestions

> **The bridge.** A public width is a const parameter and a numeral's precision is a type, and the escape
> between them is a table: one impl per admitted width, in the crate that declares the sealed carrier,
> emitted by a macro with each row's agreement stated as a free const item beside it. **The table is forced
> rather than chosen.** Recursion on a const parameter needs a const expression in type position, which is
> forbidden; a `type const` body may not compute one either; and separating a base case from a recursive case
> needs full specialisation, which is forbidden and is one of the two bans the model-width transfer argument
> rests on. An impl is the only case split Rust has over a const, so an enumeration is the only bridge.

> **What the table costs and what it does not.** Linear in the admitted range, measured at 0.25 s for a
> thousand rows including every row's assertion, which under the pricing pillar is nothing. It resolves in the
> trait solver before monomorphisation, so **no build layer and no downstream target sees it**: there is no
> per-width symbol, no runtime table, and nothing arvo needs back from any lowering. What it does cost is one
> diagnostic: a width outside the range refuses at `E0277` with `#[diagnostic::on_unimplemented]` supplying
> the message, and a residue naming the indexing trait that no attribute suppresses.

> **The preset key.** A preset name denotes one row per number kind, so a preset's row is a function of the
> preset and the kind, and the kind is the exponent form. `Policy` and `Lowering` are therefore keyed on the
> exponent form, with each impl quantified over that form's own parameters, and `Number` reads
> `Number<N: Numeral, S: Policy<FormOf<N>> + Lowering<FormOf<N>>>`. **The consumer supplies no kind**: it is
> projected from the numeral, so a preset cannot be paired with the wrong kind, and a row keyed more finely
> than the form is refused by coherence rather than by review.

> **Open, and the mechanism is not the question.** Which total widths a consumer may write, and whether the
> sum of the integer and fraction widths is bounded by that range. The table bounds each width and not their
> sum; a marker emitted with each row bounds the sum in one line; and the precision of a product legitimately
> exceeds any width a consumer wrote, so the bounded quantity is the written width and not the precision.
> **One sentence decides it and the mechanism follows.**

---

## Trusted base and limits

Every diagnostic quoted above is verbatim output from `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`--edition 2024 --crate-type=lib`, in a scratch directory outside the repository, thirty-four probes across eleven topologies. Timings
are `/usr/bin/time -p` walltime, three runs after a warm first, on this host only, and they are a shape rather
than a portable number.

Three limits I would not want read past. **The tower I built is mine, not `110`'s**: it implements the same
BNF at `110:3004-3009` and its `Pos` addition is the standard carry chain, but the design's own `Sum` has not
been declared (it is on the open list per `118:302-305`), so my addition is a stand-in and the bridge's shape
rather than its exact spelling is what transfers. **The preset probes model the tables' rows and not their
content**: `Quantisation` is modelled as a five-member product and the far point as a marker, which is enough
to key the rows and not enough to say anything about what the rows mean. And **the whole of section 2 assumes
the two tables at `110:2598-2657` are the tables**; they are ratified at `70b`, and if op moves a cell the
keying argument is unaffected but the impl bodies are not.
