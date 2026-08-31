# 210. The container premise is a theorem over a signature, and it has been asked as a premise

Seat 210, on `question::the_container_premise`. The answer is below and so is every route I closed
getting to it. Four probes, committed with their outputs and their controls at `210_probes/`, two of
them with a first run kept beside the second because the first run's failing controls are where the
sharper statement came from.

**The short form, for anyone who reads no further.** The question cannot be answered as put, because
it is not a question about arvo's types. Observability is not a property a representation carries; it
is induced by a signature, and "is footprint observable" therefore has no truth value until the
signature is fixed. The registry already says so at one row and says the opposite at another, and the
two rows point their dependency arrows at each other. Once the signature is named, the word
"footprint" turns out to be doing three jobs with three different answers, and every one of the three
is forced rather than chosen: two by op's own standards bound, one by the host language. The fork
does not need a ruling. It needs splitting.

---

## 0. The two gates

**Canon gate: passed.** Checked against `mock/registry/*.toml`, which `mockspace.toml` declares as
its `canon_paths`, read through `cargo mock query` and directly. Nothing below closes a question
reserved to op: every row I propose carries `answers` rather than a stamp, per the schema's own words on that
field, "It does not settle them while it is a proposal."

Two ratified rulings landed after my brief and both bear on this file. `ruling::an_ack_is_not_a_ratification`,
which is why nothing here reads op's bound as an answer. And
`ruling::the_strategy_intents_are_not_clear_cut`, in his words: *"it's
not clear cut. You should not write these as clear cut and settled. The intent is clear I think, but
nothing about them is absolute otherwise."* Section 4 leans on a strategy intent, and section 4.1 is
written specifically so that it leans on the **intent** and on nothing concrete, because
`ruling::what_is_ratifiable_is_the_intent_not_the_concretes` says the concretes were never his.

**Test gate: passed, before and after.** `cargo test -p arvo-checks --release`: 26 + 8 + the per-file
suites, every one green, no ignored, no filtered. I read the bodies of the files governing what I
write rather than their names: `every_predicate_names_a_declared_axis.rs` is a real suite with four
planted controls each asserting a distinct `kind`, and its `a_keywords_list_is_not_read_as_a_predicate`
arm is the one that stops the walker being noise. Nothing in it is tautological and nothing in it is
helped setup. `mock/checks` passed again after my rows land (section 9).

**One thing I did wrong, recorded because the rule says pushed mistakes are not tidied away.** I wrote
p3's first copy to `/Users/orgrinrt/Dev/clause-work/numeric-stack/mock/research/.../210_probes/`,
which is the workspace repository, not this worktree. It created one untracked file and one untracked
directory tree in a repo I have no business writing to. Caught on the next tool call by `git status`
in that repo, removed file-by-file rather than with a recursive delete, and the repo was clean again
before anything was staged. Nothing was committed and nothing was lost. It is the
`one-session-one-workspace` failure in its ordinary form: the two paths differ by one segment.

---

## 1. What I did first, which was try to break the question

**The brief hands over three options and says the answer is one of them or a concession. I do not
think it is either, so the first thing to establish is whether the question is well posed.** It is
not, and the registry contains the proof of that already.

`question::the_container_premise` asks whether a declared
numeral's footprint is observable, and its `unblocks` says it unblocks the statement of what a
primitive is. My brief adds that `question::which_operation_set_the_design_ships` "sits downstream of
it".

Open the other row. `question::which_operation_set_the_design_ships`, field
`unblocks`:

> The preset count and their names, which is op's own open intent. [...] **It also decides whether
> footprint is observable**, and with it one clause of the primitive candidate, the contested
> one-vocabulary-or-two fork, and whether the count of primitives is container-relative.

And its `bound`, four lines later:

> **It was recorded as blocked on him and it never was.** It is downstream of `the_container_premise`,
> which is itself now the panel's under a stated bound, so the chain that ends here is workable from
> end to end without him.

**One row says Q61 decides the container premise. The same row says Q61 is downstream of the container
premise.** Both are agent output, both are live, and they are in the same field group of the same row.
A reader working from either direction reaches the other and stops.

**The candidate agrees with the first half and not with the second.** `161` clause 4, as amended at
`164`:

> ...This holds over signatures whose operations are functions of the value set and the realisation
> map; an observation of the container **is not such a function**, it splits every class it touches,
> and **whether such an observation is in the design's operation set is op's open decision**, on which
> this clause's saturation is conditional.

So the candidate's own statement of the premise is *"is a container observation in the operation
set"*. That is a question about a signature. It is one coordinate of Q61 and it cannot be upstream of
the thing it is a coordinate of.

**Finding, and it is the reason nine units did not close this.** The container premise has been asked
as a premise, which is to say as a fact about arvo's types waiting to be discovered or ruled on. It is
not one. It is a **theorem schema parameterised by the operation set**, and a schema has no truth
value until its parameter is supplied. Every derivation that treated it as a bit to be flipped was
correct about its own model and could not have converged, because there was nothing there to converge
on.

**What that costs, stated so it is not read as a technicality.** `162` measured 32 primitives under
one branch and 64 under the other and the corpus has carried "a container observation is measured to
split every class, 32 to 64" ever since, as though it named a contested fact. It does not. Clause 2
already says identity is denotational sameness **"relative to the declared operation set"**, so 32 and
64 are the extensions of one parametric definition at two values of its parameter. Both numbers are
right. They were never in tension, and the row that reads them as a fork
(`question::the_container_premise`, its `note` field) has been quoted as evidence that something is
unresolved, when what it measured is that the definition is parametric, which clause 2 says in words.

---

## 2. Route one, closed: pick a branch and mark the clauses

The obvious move, and the one `164` took: keep the premise as a bit, mark every clause whose truth,
extension or admissibility it moves, and wait for a ruling. `164` section 2 does the sweep properly and
finds three clauses conditional in full, one by design, and two subordinate phrases.

**Closed, and by the sweep's own result rather than by an argument against it.** A conditionality that
reaches four clauses and two phrases of a thirteen-clause statement is not a marked clause, it is a
statement written twice. `163` says so directly and I agree with it: *"no wording is true on both
branches until this is ruled."* The disjunctive form `164` adopts for clause 6 is honest and it is not
a canon sentence, because the permanence test in `161`'s own closing section asks whether every
sentence survives a rewrite in another decade, and a sentence of the form "under branch A, X; under
branch B, not X" survives nothing: it is a note saying the canon has not been written yet.

**What closed it was reading the marks.** Four clauses and two phrases all conditional on one bit is
the shape a **missing distinction** makes, not the shape a missing decision makes. A missing decision
touches the clause that needs it. A missing distinction touches everything downstream of the word that
is doing two jobs.

---

## 3. Route two, closed: X3, and what op has already said about it

Option 2 in the row is that a strategy changes a computed value, under which the carrier pair is
separated arithmetically and clause 6 survives untouched. `163` names it reading 2 and declines to
choose; `164` narrows it toward false on the strength of a claim about the shipped bench suite.

**I went to check that claim, because it is the only one in the neighbourhood that is checkable in an
afternoon. It is false.** `164` section 1.4, catch two:

> The shipped corpus already asserts exactly that agreement: `warm-container-shared`'s
> `all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key`
> (`warm-container-shared/src/lib.rs:1356`) requires every arm over every carrier rule to produce the
> byte-identical result against an independent oracle on every declared key. So on the shipped
> evidence the pair `163` constructs has identical denotation by test, and reading 2 [...] is, for
> this crate's swept keys, **measured false rather than open**.

**The suite does not compare two markers. It compares four carrier rules under one fixed semantics,
and the semantics is a component of the key.** `warm-container-shared/src/lib.rs:115-122`, the source's
own words:

> Semantics: 0 is a wrapping reduction (`Warm`), 1 is a saturating reduction (`Precise`), ...

```rust
pub const fn key_op(key: usize) -> usize {
    (key / 100) % 10
}
```

and the assertion body at `:1372-1375` calls `arms::headroom(key, cols)`, `arms::minimum(key, cols)`,
`arms::plusone(key, cols)` and `arms::native(key, cols)` on **one key**. `headroom`, `minimum`,
`plusone` and `native` are four container rules for whichever semantics that key selected. The test
never holds a carrier fixed and varies the marker; it holds the marker fixed and varies the carrier,
which is the opposite quantifier.

**`210_probes/p1`** makes that a measurement rather than a reading, by depending on the bench crate by
path so the oracle under test is the shipped oracle rather than a copy:

```
A3. the shipped claim: four carrier rules agree at a FIXED key
  57 keys, 0 disagreements   required=0   as required

A1. the axis the shipped test holds FIXED: op=0 (wrapping, Warm) against
    op=1 (saturating, Precise), on identical values, width and depth
  28 keys compared, 28 where the two semantics differ   required=>0   as required
  first witness: key 80003, wrapping gives 159, saturating gives 255
```

**Twenty-eight of twenty-eight.** The axis `164` cited the suite as having tested is the axis the suite
holds constant, and on every key where the two values can be compared at all, they disagree. A1 is the
control: had the two semantics agreed, my distinction would have been vacuous and the paragraph would
have been right by accident. A2 is the second control, the same call twice, which is what says A1's
disagreement is the operand rather than nondeterminism.

**So `164`'s narrowing is withdrawn and reading 2 is exactly as open as `163` left it.** A retirement
row carries this, because the paragraph is the kind that gets quoted: it reads as a reproduction catch,
it arrives with a `file:line`, and it moved a live question. Every other reproduction in `164` section
1 that I checked reproduces; this one is a misreading of a quantifier and it is the only one.

**Route two is closed as a route rather than as a claim.** X3 may well be true, and op's ratified
intent for Hot points that way: `strategy::hot`'s intent is *"Performance and efficiency, at the cost
of accuracy or soundness where the loss buys something"*, and an accuracy loss that changes no computed
value is not an accuracy loss. But it does not close the premise, for a reason that has nothing to do
with whether it is true: **the carrier pair does not need two markers with different realisation maps.
It needs two with the same one**, and nothing about X3 says that pair cannot exist. A design shipping
any two selections that agree on the realisation map and disagree on the carrier reconstructs `163`'s
pair immediately. X3 removes some instances of the pair and not the pair.

---

## 4. Where the answer actually comes from

Two forces decide it, and they are op's two, used as he stated them rather than as a preference.

### 4.1 Soundness, and one ratified intent used only for its intent

Op returned the question with a bound: *"it is bounded by soundness and also the rule that demands we
provide first-class matlab and ieee754 compatible apis as aliases over arvo primitives and such."*

The soundness half bites here in a way nobody has used yet, and it is an argument about **content**
rather than about correctness:

1. **Some ratified intent in the strategy catalogue is a footprint intent.** `strategy::cold`'s
   `intent` field: *"It aggressively minimises and bitpacks"*, `weighs`: *"Footprint above speed."*
   Under `ruling::the_strategy_intents_are_not_clear_cut` the concretes attached to Cold are not
   settled and I do not use one. Under `ruling::what_is_ratifiable_is_the_intent_not_the_concretes`
   the intent is the ratifiable part, and `strategy::cold`'s own note records that op said it *"survives
   the set being reshaped, resized or renamed"*, whether the strategies are *"four or seventeen or a
   billion"*. So this argument does not name Cold, does not need Cold to exist, and does not need the
   set to have four members. It needs there to be **an** intent about footprint, which there is.
2. **An intent whose subject is unobservable has no content.** If no signature the design ships can
   distinguish a smaller footprint from a larger one, then "minimises footprint" is a sentence no
   program can act on, no consumer can select on, and no test can check. It is not a weak intent, it
   is not an intent.
3. **Therefore the footprint-internal branch, read as "nothing observes the footprint", contradicts a
   ratified intent.** It is not a design arvo may choose; it is a design in which one of op's stated
   intents denotes nothing.

**That is the soundness half discharged, and it discharges toward observable.** It says nothing yet
about *what* observes it, which is where the second force comes in and where the answer gets its
shape.

### 4.2 The standards bound, which forces the other half and forces it the other way

`obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives` records this as an
**adequacy test**: a convention that cannot be written as an alias is a gap in the primitives. So the
two standards op named are a refutation instrument, and `210_probes/p4` is that instrument, run
against three candidate statements of where arvo's encoding is stated: over the declared width `W`,
over the minimum native container `rung(W)`, and over the shipped `Warm`/`Precise` rule
`rung(rung_bits(W)+1)`.

**MATLAB `fi` is the interesting one, because it ships both observations and gives them different
rules, which is the distinction this whole topic has been missing.** From the documentation, quoted
rather than remembered:

- **`bin(a)`** returns the unsigned binary representation of the stored integer, **exactly WordLength
  characters**. The documented example is a `WordLength: 8` object printing `'10000000 01111111'`.
- **`storedInteger(a)`** returns *"the data type of the output determined based on the signedness and
  word length (WL) of the stored integer"*: `WL <= 8` gives int8/uint8, `8 < WL <= 16` gives int16,
  `16 < WL <= 32` gives int32, `32 < WL <= 64` gives int64, and above 64 it is an error.

**That second mapping is `rung(W)`. It is a container observation, and its rule is a function of the
declared width, not of any implementation's carrier.** MATLAB, the named standard, has exactly the
distinction the panel has been arguing about, ships both halves under different names, and states both
over `W`.

IEEE 754-2019 is blunter. Clause 2: *"interchange format: A format that has a specific fixed-width
encoding defined in this standard."* Section 3.4: *"Representations of floating-point data in the
binary interchange formats are uniquely encoded in k bits"*. Table 3.5's last row closes it: *"k,
storage width in bits ... 1 + w + t"*. And section 3.2 makes it an obligation rather than a
description: *"A conforming implementation of a supported interchange format shall provide means to
read and write that format using a specific encoding defined in this clause."*

The run:

```
MATLAB `fi`, arvo's encoding stated over W
  bin        exactly WordLength characters, every swept width   PASS     fails at 0 of 12 widths
  storedInt  the documented rung(WL) type, every swept width    PASS     fails at 0 of 12 widths

MATLAB `fi`, arvo's encoding stated over Min
  bin        ...                                                REFUSED  fails at 8 of 12 widths
  storedInt  ...                                                PASS     fails at 0 of 12 widths

MATLAB `fi`, arvo's encoding stated over Head
  bin        ...                                                REFUSED  fails at 12 of 12 widths
  storedInt  ...                                                REFUSED  fails at 12 of 12 widths

IEEE 754-2019 binary interchange, stated over W     PASS     fails at []
IEEE 754-2019 binary interchange, stated over Min   PASS     fails at []
IEEE 754-2019 binary interchange, stated over Head  REFUSED  fails at [16, 32, 64]
```

Three things fall out, and the third is the one that matters most.

- **Only the `W` statement satisfies every observation of both standards.** Both aliases are writable
  over the declared width and over nothing else.
- **The shipped `Warm`/`Precise` container rule fails everything.** Stated over `Head`, `fi` is not
  writable at any of twelve swept widths and IEEE 754 is not writable at any interchange width. Under
  op's own adequacy test that is a gap in the primitives, and it is a gap that exists **only if the
  canon states the encoding over the container**. State it over `W` and the same shipped rule is fine,
  because the container stops being what the alias reads.
- **Switching arvo to the minimum container does not rescue it.** This is the escape I expected to
  have to argue against and the probe closes it: over `Min`, `fi`'s `bin` still fails at 8 of 12
  widths, because `bin` wants exactly `W` characters and `rung(13)` is 16. **So the answer does not
  depend on which container rule ships**, which detaches it from a fork that is genuinely open and
  genuinely not mine.

**The controls.** `WBroken` is `W + 1`, and it is refused by both standards at every width, which is
what stops a `PASS` meaning only "this arm mentioned `W`". Control 0 checks IEEE Table 3.5's own
closure `k = 1 + w + t` at all five widths before any arm is judged against it, so the parameter table
is not itself the thing being assumed.

**One arm failed on the first run and I kept it.** `p4_v1_min_arm_failed_at_k256.out`: I had required
the `Min` arm to pass IEEE at every interchange width and it failed at `k = 256`, because `rung` has
no value above 128. That is my model speaking rather than the design: above 128 bits there is no
native container for the `Min` and `Head` statements to refer to, so those arms have no referent
rather than a wrong one. The scored sweep is now `k in {16, 32, 64}` and `k in {128, 256}` is reported
unscored, which is the honest shape and incidentally a third independent way the container statement
fails: above the native rungs it cannot even be written.

---

## 5. The answer

**Footprint is observable, through exactly one observation, and it is the one arvo does not own.**

The word has been carrying three jobs. Split it and every part is forced:

| observation | who supplies it | what it reads | carrier observable |
|---|---|---|---|
| **layout**: `size_of`, `align_of`, array stride | the host language, for every `Sized` type | the carrier | **yes** |
| **encoding**: `bin`, `storedInteger`, interchange read and write, `to_le_bytes`, packing | arvo, and every standard it aliases | the declared width | no |
| **value**: arithmetic, comparison, conversion | arvo | the value set and the realisation map | no |

`210_probes/p2` measures the second and third over the pair the shipped rule creates, `W = 13` in a
`u16` against a `u32`, exhaustively over all 8192 values and all 8192 x 8192 ordered pairs:

```
SIGMA_SIZE  size_of    SEPARATES   2 vs 4
            align_of   SEPARATES   2 vs 4
SIGMA_BITS  encoding stated over the declared width W      agrees
            encoding stated over the container             SEPARATES   (control)
SIGMA_VAL   Add/Mul/Xor x Wrap/Sat, arity 2, projected     agrees      (six arms)
            chain d=1 and d=4, projected                   agrees
```

**And the consequences for the statement, which is what the topic actually wanted.**

**Clause 6 is true as written and needs one word, not a branch.** Its first sentence is *"the
realisation is not part of identity"*. Clause 2 already fixes which of three sameness relations
"identity" names: *"Of the three sameness relations, nominal, representational, denotational, each
licensing a different operation (assignment, memory reinterpretation, rewriting), only the denotational
one is a congruence under composition."* **The carrier is a coordinate of representational sameness.
It licenses memory reinterpretation, which is exactly the operation clause 2 assigns to that relation,
and it licenses nothing about substitution inside a composite.** So clause 6's sentence is true of
denotational identity, which is what clause 2 says identity means, and the "footprint-observable
branch" was never a rival reading of it: it is a statement about the neighbouring relation, which the
candidate had already named and separated two clauses earlier. Write **"the realisation is not part of
denotational identity"** and the sentence is true on both of what used to be branches, because there
are no branches.

**Clause 9's completeness obligation is defective, and independently of the premise.** It reads: *"every
pair of distinct shipped instantiations is either separated by one witness, discharged at any width
with nothing to transfer, or connected by a weakening in exactly one direction; a pair with neither is
a spurious split and is refused."* **A witness there is an input, and `size_of` takes no input.** The
obligation quantifies over value-indexed operations and cannot see a nullary one, so it refuses a pair
that a consumer can distinguish in one line.

`210_probes/p3` is that, in two parts. Part A is not a model: it declares
`#[repr(transparent)] struct Num<C, S>(C, PhantomData<S>)`, instantiates it at `u16` and `u32`, and
asks the compiler.

```
PART A
  size_of::<Cold13>()    = 2   (minimum native)
  size_of::<Precise13>() = 4   (rung(rung_bits(13)+1))
  the ambient observation separates the pair: true   required=true   as required
```

**That compiling is the finding.** The footprint-internal branch does not describe a design arvo could
ship. It describes a host language in which `size_of` is unavailable on a `Sized` type, and no ruling
of op's can produce one.

Part B runs clause 9's verdict function over five pairs under both witness sets:

```
PART B, witness set InputsOnly
  THE CARRIER PAIR   Cold<13> / Precise<13>      dirs=2 witness=none   Refused      as required
  G1 spurious ctl    AliasA<13> / AliasB<13>     dirs=2 witness=none   Refused      as required
  G2 refinement ctl  Cold<13> / Cold<13>[0,99]   dirs=1 witness=none   Refinement   as required
  G4 policy ctl      Cold<13> / Warm<13>wrap     dirs=0 add(1, 8191)   Separated    as required
  G4 width ctl       Cold<13> / Cold<14>         dirs=0 W=13 vs W=14   Separated    as required

PART B, witness set InputsAndNullary
  THE CARRIER PAIR   Cold<13> / Precise<13>      dirs=2 size_of 2 vs 4 Separated    as required
  G1 spurious ctl    AliasA<13> / AliasB<13>     dirs=2 witness=none   Refused      as required
  G2 refinement ctl  Cold<13> / Cold<13>[0,99]   dirs=1 witness=none   Refinement   as required
  ... policy and width unchanged
```

**G3 is the arm that matters for trusting the rest: under `InputsOnly` the carrier pair comes out
`dirs=2, witness=none, REFUSED`, which is `163` p1 reproduced to the character.** I am not
contradicting its measurement, I am disagreeing with the quantifier its model correctly inherited from
clause 9's wording.

**G1 is the control the whole repair rests on and it is the one I was most worried about.** Two names,
one carrier, one value set, one realisation map: the genuinely spurious pair. It stays **REFUSED**
under the wider witness set. So widening does not disarm clause 9, it disarms clause 9 on exactly one
pair, and that pair is the one a consumer can separate by calling a function.

**Clause 4's conditional discharges, in the direction it was leaning.** It says its saturation is
conditional on whether a container observation ships in the design's operation set. It does not: every
operation arvo declares and every operation either named standard declares is a function of the
declared width, per section 4.2. The layout observation is ambient rather than declared, so clause 4's
class of signatures is the class arvo actually has, and saturation holds unconditionally.

**Clause 2 needs nothing.** It is already parametric in the operation set and says so. 32 and 64 are
its extension at two parameter values, both correct.

---

## 6. The arm boundary, which the candidate already contains

The answer above is stated at a sole-occupant placement and it does not hold everywhere, and the
dimension that moves it is already in clause 5:

> The lens **degenerates to an ordinary value exactly where its focus is the sole logical occupant of
> its carrier allocation**; padding is permitted, sharing is not. At a degenerate point the language
> supplies a standalone type and the native end never mentions the lens; **everywhere else the
> primitive is reached through its carrier and no `Sized`-bounded contract ranges over it.**

**At a shared placement there is no per-element `size_of` to call.** Clause 5 says the element is not
reached as a standalone type and no `Sized`-bounded contract ranges over it, which is precisely the
condition under which the layout observation does not exist. So:

- **Sole occupancy.** The layout observation exists, the carrier is observable through it, clause 9
  separates the carrier pair on a nullary witness, and two selections differing only in the carrier are
  two primitives.
- **Shared occupancy.** No per-element layout observation exists. The carrier is unobservable, and two
  selections differing only in it over one packed column are **one** primitive with two placements,
  which is what clause 5's lens says they are. What is observable there is the allocation's stride,
  which is a property of the column rather than of the element.

**This is not a hedge and it is not a second branch of the premise.** It is one rule evaluated at two
placements, and the placement is a structural fact the design already carries. It also says something
useful about where a footprint intent is realised: at a shared placement it is realised through the
stride, not through any element's size, which is the packing story the corpus already has and had not
connected to this question.

**I have not added an `occupancy` dimension to `dimension.toml`.** The axis is real and the vocabulary
is append-only so adding one is permitted, but `alignment` already covers the packed side at a
different granularity and whether these are one axis or two is a call I do not think one seat should
make alone. It is named in the rows I write as a stated condition rather than as a declared dimension,
which is the narrower thing to do, and section 10 flags it.

---

## 7. Predicates

Stated per `every-finding-carries-its-predicate`, with omission meaning the finding does not hold where
that dimension exists.

**The layout observation separates a carrier pair.** `integer width: 13; fraction width: 0; signedness:
unsigned; container in {u16, u32}; strategy: any two whose carrier rules differ; threads: 1; target
features: any; toolchain: nightly-2026-05-28; build profile: release.` Compile-time structural, so
`threads: 1` is what the run establishes and I am not writing `any`.

**The declared-width encoding does not separate a carrier pair.** `integer width: 13; fraction width:
0; signedness: unsigned; container in {u16, u32}; operation: encode; threads: 1; target features: any;
toolchain: nightly-2026-05-28; build profile: release.`

**Projected arithmetic does not separate a carrier pair.** `integer width: 13; fraction width: 0;
signedness: unsigned; container in {u16, u32}; operation in {add, mul, xor}; overflow policy in {wrap,
sat}; arity: 2; chain length in {1, 4}; threads: 1; target features: any; toolchain:
nightly-2026-05-28; build profile: release.`

**Unprojected arithmetic separates a carrier pair exactly where the intermediate can exceed the
narrower carrier.** `integer width: 13; fraction width: 0; signedness: unsigned; container in {u16,
u32}; operation in {add, mul}; overflow policy in {wrap, sat}; arity: 2; chain length in {2, 4, 8,
16}; threads: 1; target features: any; toolchain: nightly-2026-05-28; build profile: release.` This one
is section 8's, and it is the finding I did not set out to make.

**Neither named standard's surface is writable over a container statement.** `integer width in {3, 8,
13, 14, 16, 23, 27, 31, 32, 47, 60, 64}; radix: 2; signedness: unsigned and signed; threads: 1; target
features: any; toolchain: nightly-2026-05-28; build profile: release.` The IEEE half additionally at
`k in {16, 32, 64}` scored, `{128, 256}` reported unscored.

**Everything above is `fraction width: 0`, and I want that read as the limit it is.** Nothing here
tests a fractional format, and the encoding argument is the one I would expect to survive `F > 0`
untouched, since `bin` and an interchange encoding read a stored integer either way. I have not
measured it and I am not claiming it.

---

## 8. The failing control that improved the answer

`p2`'s first run required every arm with the projection removed to separate the carriers, and three of
four reported agreement. `p2_v1_controls_that_did_not_reach_the_path.out` is that run, kept.

The cause is arithmetic rather than conceptual: over a 13-bit domain at arity 2, an unprojected sum is
at most 16382, which fits a `u16`, so the two carriers **cannot** disagree and removing the projection
does not make them. My control never entered the path it was built to test.

**What it bought is a better statement than the one I designed for.** The projection is not the only
thing hiding the carrier from arithmetic. **Headroom sufficiency hides it too**, and the two regions
have a boundary. The repaired probe splits the arms by whether the unprojected intermediate can exceed
the narrower carrier's range and declares the verdict each region requires:

```
C2a: unprojected AND the intermediate can exceed the narrow carrier   (must separate)
  Mul/Wrap, arity 2       SEPARATES  a=9,b=7282
  Mul/Sat,  arity 2       SEPARATES  a=9,b=7282
  chain d=8,  Add/Wrap    SEPARATES  x=8,k=8191
  chain d=16, Add/Wrap    SEPARATES  x=0,k=4096
  chain d=8,  Add/Sat     SEPARATES  x=8,k=8191

C2b: unprojected AND it cannot exceed it                              (must agree)
  Add/Wrap, arity 2       agrees
  Add/Sat,  arity 2       agrees
  chain d=4, Add/Wrap     agrees
```

**Why this matters to the answer rather than to the probe.** It converts "arithmetic does not observe
the carrier" from a measurement into an **obligation with a stated trigger**. The projection back to
`W` is not decoration and it is not always needed: it is required exactly where an unprojected
intermediate can exceed the narrower carrier, and omitting it there makes the carrier arithmetically
observable and the encoding argument in section 4.2 collapses with it. A design that writes the
projection lazily is choosing, silently, to put the carrier into `SIGMA_VAL`.

---

## 9. What I checked that I did not need to, and what I could not

- **`cargo test -p arvo-checks --release` before and after.** Green both times, no ignored tests. The
  registry schema check runs in the pre-commit hook and passed on both of my commits: *"651 rows across
  10 namespaces, schema check passed, all repo lints passed."*
- **The bench crate's own suite.** 15 tests green, including the one this file disputes the reading of.
  I am not saying the test is wrong. It is a good test and it asserts exactly what it says it asserts.
- **The hook reformatted two probe sources after I staged them**, which is the
  `cl-claim-sketch-discipline` hazard in its ordinary form. I re-ran all four probes against the
  formatted source and committed the result as its own commit; every arm still reports as required.
- **What I could not close: whether `F > 0` moves any of this.** Section 7 says so. My instinct is that
  it does not touch the encoding half and I have no evidence, so it is stated as unmeasured and the
  predicates say `fraction width: 0`.
- **What I could not close: the non-uniform value sets.** `161`'s own "where it is weaker than offered"
  names this as untested in both sittings and it still is. Every probe here is a uniform grid, so the
  float side of arvo is outside all of it. That is the largest single gap in what I am proposing and I
  would rather say so than let the table in section 5 read as covering `FastFloat`.

---

## 10. What I am handing on, including what I did not take

**For whoever attacks this next, the alternatives I looked at and did not ship:**

- **Add an `occupancy` dimension and gate the whole answer on it.** Section 6's boundary would then be
  a first-class predicate rather than a stated condition. I did not, because `alignment` is adjacent
  and one seat inventing a second axis over the same territory is how a vocabulary grows two words for
  one thing. Worth doing; worth two readers agreeing on the granularity first.
- **Rewrite clause 9's completeness in terms of a contextual equivalence** rather than patching its
  witness set. Every pair of instantiations is separated iff some context distinguishes them, with the
  context set named; nullary observations fall out for free instead of being bolted on, and the
  "discharged at any width with nothing to transfer" branch probably falls out too. **This is the
  formulation I would reach for if I were writing clause 9 fresh, and I did not propose it because the
  clause is under signature and replacing it wholesale is a bigger act than fixing what is wrong with
  it.** Whoever revises the candidate should consider it.
- **Say the three observation classes are three operation sets and give each a name in the canon.**
  Tempting and probably right eventually, and I stopped short because naming is `question::` territory
  and `clause 11` makes a name an existence claim.
- **Argue the shipped `Warm`/`Precise` container rule should change on the strength of p4.** It should
  not follow from p4 and I want that said plainly, because p4 looks like an argument for it. The
  probe's third result is precisely that the container rule is **irrelevant** to the standards
  question once the encoding is stated over `W`, so citing p4 in the container-rule fork would be
  citing it for the opposite of what it shows.

**One unlicensed mechanism, outside my question, reported as the standing instruction requires.** The
circular dependency in section 1 is not merely an inconsistency, it is two rows that between them
license reasoning in a loop, and both are live and cited. The `unblocks` and `bound` fields of
`question::which_operation_set_the_design_ships` sit in one row. Whichever direction is kept, one
of the two sentences has to go, and until it does any later reader can derive whichever conclusion
they arrived wanting.

---

## 11. Coverage, bounded honestly

I read `156` item 7, `161` sections 4 and 9, `164` in full, `163` sections 3 and 4, the question,
topic, dimension, strategy and obligation rows named above, the two ratified rulings my brief flagged,
and the `warm-container-shared` source and its tests. I did **not** read `162` beyond what `164` and
`163` quote of it, `154`, `155`, `157`, `158`, `159`, `160` or `165`, and my account of `162`'s two
findings is therefore taken from two files that both had reason to describe them accurately and
neither of which is `162`. **My own prior files in this panel are among the ones I did not re-read**,
which is worth stating because a seat agreeing with its own earlier work is not two instances.

Nothing here rests on a claim I did not open. The two quotations doing the most work, MATLAB's two
observation rules and IEEE 754's interchange clauses, were fetched from the vendor documentation and
from the standard's own text rather than recalled, and section 4.2 names where.

**Standing of what I propose: one expert.** Every derivation, every probe and every control in this
file is mine and nothing here was reached independently by anybody else. The section 3 refutation is
the exception in one direction only: it disagrees with `164` on a checkable fact and the check is
committed, so a second reader has something to run rather than something to agree with.
