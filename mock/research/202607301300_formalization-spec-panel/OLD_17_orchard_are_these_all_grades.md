# 17: Are These All Grades

**Reviewer:** Dominic Orchard (graded modal types and coeffects lens: what a computation demands of its
context against what it provides to it, over a semiring or an ordered monoid, with one soundness
statement rather than one per feature).

**What I read.** `16b_op_design_the_shape_not_the_code.md`, `16c_op_the_downstream_contract.md`,
`16d_op_the_spirit_outranks_all.md` and `13c_op_the_standard_and_the_mode.md` first, as the brief
directs, then `11_current_shape_draft.md` in full. Then the dive: `13_mcsherry_where_the_laws_belong.md`,
`13b`, `14_dolan_which_algebra_is_this.md`, `15_willsey_what_a_law_is_for.md`,
`16_fallin_laws_as_backend_licences.md`, and their probe directories. Then, per the brief,
`07_spj_is_the_type_story_sound.md` and `12_lattner_fresh_read.md` in full, plus `12b` for the
checkpoint that holds the fidelity axis open, and the fallibility sections of
`05_leijen_fallibility_without_poisoning.md` by grep rather than in full. I listed the panel directory
before reading inside it, per the standing instruction, and that listing turned up prior work in my own
probe slot which section 0 deals with.

On source I read very little, deliberately, and only to check a claim before reasoning from it:
`arvo-strategy/src/cross_strategy.rs:3` for the wording of the `Resolve` projection,
`arvo-strategy/src/identity.rs:51-53` for what the ladder's bottom rung actually declares, and a grep
of `mock/crates/` for ladder names. All three confirmed what files 12 through 16 already reported and
none of it is an audit.

**What I compiled and ran**, as distinct from what I reasoned about, is three probes committed at
`17_probes/06`, `07` and `08`, all `rustc -O` against `nightly-2026-05-28`, plus a re-run of the three
files that were already in that directory. Results are in sections 2, 4, 5 and 8 with the counts
inline. Everything else here is argument, offered as directions rather than rulings, and where I hold
more than one reading I say so and leave the choice where it belongs.

## 0. Two premise checks, one of which changes what I could build on

**Three probes were already sitting in `17_probes/` when I started.** `03_supply_table.rs`,
`04_rounding_count_is_the_grade.rs` and `05_the_grade_is_not_a_count.rs`, timestamped between the
writing of file 16 and op's third posture correction, attributed to nobody, and describing themselves
as "a prior partial run of this dispatch". This is the same situation McSherry reported at
`13_mcsherry...md:65-68`, and I have handled it the same way: re-run everything, keep what reproduces,
say what does not.

`03_supply_table.rs` reproduces exactly and is good work. It is the sharpest single artifact in this
directory and section 5 uses it. `04` reproduces and its own successor already retracts half of it.

**`05_the_grade_is_not_a_count.rs` has four rows that are not measurements.** Its two-event evaluation
computes the intermediate shift as `2 * FRAC - (FRAC + g)` (`17_probes/05...rs:102`) in `u32`. With
`FRAC = 2`, every `g > 2` underflows: `g = 3` gives 4294967295 and `g = 4` gives 4294967294, which
`>>` then masks to 63 and 62. That is why its `g=3` and `g=4` rows all report an identical 3083
disagreements and a worst error of 32 regardless of which rounding rule is in the column, a tell the
file prints and does not read. Reproduced directly:

```
g=0  inner shift = 2  g=1  inner shift = 1  g=2  inner shift = 0
g=3  inner shift = 4294967295 (masked to 63)
g=4  inner shift = 4294967294 (masked to 62)
```

The underlying point is worse than a bug, and it is worth keeping. At `FRAC = 2` the exact product
carries `2 * FRAC = 4` fractional bits, so an intermediate cannot be held at more than `FRAC + 2`.
`g > FRAC` is not "more guard bits", it is a request for precision the exact product never had, and the
model computes garbage rather than refusing. The probe's actual finding, that `g = 2` to-odd reproduces
the one-rounding answer with zero disagreements, stands and is load-bearing for section 5. Its
conclusion is unaffected. Four of its nine rows should be deleted rather than read.

**And the brief's own framing needs a small correction before I use it.** It says the three candidates
"emerged separately, from members not looking for each other's results". Two of the three were
connected by the members who raised them. File 07 section 2 explicitly locates the delivery question at
"the grade-interpretation seam" and ties it to file 05's carrier join, and file 15 section 4 explicitly
says the fidelity axis and the multiplication law question are "the same question asked twice"
(`15_willsey...md:187-188`). So the convergence the brief treats as three independent arrows is two
arrows plus a member noticing. Mutual agreement between unratified artifacts is not corroboration; I
therefore treated the "these may be one thing" hypothesis as something to test rather than to explain,
and section 2 is that test.

I also checked the one shipped fact I intended to reason from rather than taking it from file 12:
`arvo-strategy/src/cross_strategy.rs:3` does say "`Resolve<S1, S2>::Out` projects the more conservative
of two", in those words. Section 4 is about that sentence.

## 1. The answer, stated in the order that is the argument

There are **two graded structures and one thing that is not a grade**, and the partition does not fall
where the brief draws it. Stating it in the order I would defend it:

**The structure.** Arithmetic in this design is a family of maps
`⊕_C : V × V → ⟦m(C)⟧(V)`, indexed by a composition `C`, where `⟦_⟧` interprets a grade as a functor on
values, and where a second index `ℓ(C)` decides not the type of the answer but which of several
equal-typed bodies computes it.

**The grade monoid**, for the first index. `M = P(Cause) × Multiset(Event)`, with
`(c₁, e₁) · (c₂, e₂) = (c₁ ∪ c₂, e₁ ⊎ e₂)`, unit `(∅, ∅)`, ordered componentwise by `⊆` and by
multiset inclusion. `Cause` enumerates the ways an operation can decline to answer (over range, under
range, divide by zero, whatever a future effect adds), and `Event` enumerates the quantisation events a
value passes through on its way to the answer, each described by the width it was held at and the rule
that landed it there. Both components are free commutative monoids, so the monoid is free, so
composition never loses information and never needs a choice.

**The modality.** `⟦_⟧` is a graded monad. `pure : V → ⟦(∅, ∅)⟧V`, a graded multiplication
`⟦m⟧⟦n⟧V → ⟦m · n⟧V`, and an upcast `⟦m⟧V → ⟦n⟧V` whenever `m ≤ n`, which is where file 07's
`LiftGrade` lives. The grade accumulates along the term, in the direction of the answer. That is an
effect.

**The second index is not on that monoid and is not an effect.** `ℓ(C)` is a licence: the set of
liberties an implementation may take. It does not accumulate along the term, it is inherited by
subterms, and its coercion runs the other way, because a value carrying a permissive licence can always
be viewed as carrying a restrictive one and never the reverse. The correct structure name, on the
evidence in section 4, is a **bounded meet-semilattice with a downward-closed coercion**, not a
semiring. Calling it a coeffect is right about the *direction* and would be wrong about the *algebra*,
and section 4.3 says why I am not claiming the semiring.

**The soundness statement, one sentence.** An arithmetic body typed at grade `m` under licence `ℓ`
computes a value in `⟦m⟧(V)`, and a rewrite of that body is licensed exactly when it is an equation of
the algebra at `(m, ℓ)` and does not raise `m`.

**The corollaries, which is the whole point of stating it that way.** Each of these is currently a
separate open item in the draft, and each falls out of the one statement rather than needing its own
mechanism:

Total meeting partial is the join in the first component, with the lift as its witness. That is file
07's section 2 result, unchanged, now an instance.

`Growth::Narrowed`'s second refusal site adds one term to the `Cause` component, and its quantisation
adds one `(width, rule)` term to the `Event` component. The draft's open item at 5.2 ("nothing has been
extended to a two-site case") is not an extension; it is what a free monoid does. And `Narrowed<W, A>`'s
own type parameters **are** the event description, which means the design already carries the grade in
the axis without having noticed.

`Growth::Exact` is the empty multiset on the second component. That is why every clean law derivation in
this whole round has been about `Exact` and why multiplication, which always quantises, has resisted.

A future divide-by-zero refusal adds a member to `Cause`. The enumeration does not change, which is file
07's own observation, now with a reason.

And the one that is not currently anywhere in the design: **associativity is "regrouping does not change
the grade and is an equation of the algebra at that grade", and those are two conditions, not one.** The
draft's machinery checks them fused, under Kleene equality. Section 5 separates them by measurement, and
they come apart on `Precise`, in a way that moves `Precise` from the refused column to a column the
design does not have.

## 2. The separating measurement, first, because it is what decides the brief's question

The brief asks whether the three are one structure. I did not want to answer that from vocabulary, so I
built an instrument that would show it if it were true: measure, for each arithmetic, over every one of
the fourteen distinct binary trees on a five-element fold, exhaustively over a signed three-bit model,
two independent quantities. How far apart two groupings' *answers* can be, and how often two groupings
*disagree about whether there is an answer at all*.

`17_probes/07_the_grade_is_the_diameter.rs`, `rustc -O`, 32768 inputs per row:

| arithmetic | numeric diameter | distinct answers at worst input | grouping-dependent refusals |
|---|---|---|---|
| `Exact` (unbounded) | 0 | 1 | 0 |
| `Wrap` (`Hot`) | 0 | 1 | 0 |
| `Saturate` (`Warm`/`Cold`), signed | **7** | **8** | 0 |
| `Refuse` (`Precise`) | 0 | 1 | **10992** |
| `f64`, on constructed inputs | **unbounded, then NaN** | up to 14 | 0 |

The two columns separate different sets and neither separates both. Numeric diameter distinguishes
saturating and float from wrapping and refusing. Definedness disagreement distinguishes refusing from
everything else, including from saturating, which never disagrees about definedness at all. Float
breaks the first column entirely: the relative gap reaches 100% at `[1e16, -1e16, 1, 1, 1]` (answers
spanning 0.0 to 4.0) and every one of the fourteen groupings returns NaN once an infinity and its
negation are both present, which is not a distance.

**That is the answer to the brief's question, and it is a measurement rather than a preference.** If the
three candidates were one graded structure, one index would separate all of these rows. Two are needed
and they are of different kinds: one is a set that accumulates, one is a distance that does not exist in
general.

Two further readings from the same probe, both of which I hold and neither of which I resolve.

**Signed saturating's diameter grows with fold length until it is the whole range.** Measured at
`n = 2, 3, 4, 5` over `[-4, 3]`: `0, 3, 6, 7`. By five elements the reachable answer set is the entire
representable set. So an attractive-looking idea, "bound a combinator on a diameter budget rather than on
a boolean", is dead for the arithmetic it was most wanted for. I want to be explicit that I went looking
for that idea and the probe refused it.

**Unsigned saturating's diameter is 0 at every fold length**, measured over `[0, 7]` at `n = 2` through
`5`. That reproduces Dolan's torsion-group and one-sided-clamp argument (`14_dolan...md:203-224`) on a
completely different instrument, which is worth one line because it is the kind of agreement that means
something: two constructions with no shared code reaching the same fact.

## 3. Fallibility is a graded monad, and the design's two-point version is losing something

File 07's shape is right and I would build it. Grade as a join over an operation's firing sites, carrier
as the interpretation of the grade, lift as one implementation per grade pair, computed and consumed in
one blanket so every arithmetic body carries a single bound. Everything in `07_probes/b_bounds_collapse.rs`
is the graded-monad presentation of exception effects, done correctly, and I have nothing to add to the
mechanism.

What I would change is the grade, and the change is small.

**The design's grade is a two-point lattice `No ≤ Yes`. The mathematics has `P(Cause)`.** The two-point
version is the image of `P(Cause)` under "is it empty", and collapsing to that image throws away exactly
the information that decides the error type. Under `P(Cause)`:

The carrier's error component derives from the grade rather than being a fixed payload. A composition
whose grade is `{OverRange}` returns through an error type with one inhabitant. A composition whose grade
is `{OverRange, DivideByZero}` returns through a two-inhabitant error type. Nothing anywhere has to
declare which error a given composition can produce, and no consumer ever matches an arm that cannot
fire.

The draft's own open question at 5.2, "a `Narrowed` intermediate can produce two refusal sites in one
operation, and no carrier-join machinery models it", stops being a question about carriers and becomes
one about whether the two sites contribute the same cause or different ones. If they are different (they
are: narrowing an intermediate and refusing a final result are different events), the grade distinguishes
them and the error tells a consumer which happened, for free.

`Precise`'s "call sites unwrap" (draft 3.5) becomes typed. Today the price of `Precise` is a refusal with
no information; under the graded reading the type says which refusals are reachable at all.

The cost, stated. `P(Cause)` needs a type-level set with a decidable union and a decidable inclusion, over
a closed constructor set. That is `notko-hlist` plus a fold, which this workspace already has and which
`11_current_shape_draft.md:56` lists as extracted and unreviewed. It is more machinery than a boolean and
it is the same machinery Dolan's atomic-fact fold already needs, so the marginal cost is small and the
two should be designed together rather than twice.

**The reading I am not taking, held honestly.** Two points may be all the design ever needs, if the only
refusal cause it ever ships is "the result left the representable set". The `P(Cause)` argument is worth
what future causes are worth, and the draft names exactly one prospective second cause (divide by zero,
4.2). One prospective member is thin evidence for a lattice. What tips me is that the collapse is
irreversible in the API sense: a consumer who has written `match` against a one-inhabitant error and a
consumer who has written it against a boolean are in different positions when the second cause arrives,
and only the first one's code still compiles for the right reason. But I would not fight for it, and it
is a decision that should be made on how many causes the design expects rather than on the elegance of
the free monoid.

## 4. Fidelity is the dual, the direction is load-bearing, and one shipped sentence is wrong to generalise

### 4.1 The measurement

`17_probes/06_two_lattices_opposite_variance.rs` encodes both grades with their coercions and asks the
compiler which coercions exist. Four builds, three of which I expected to fail.

The effect coercion, `Total → Fallible`, is a total function on values and compiles. The reverse does not
exist, and `--cfg bad_lift` shows that it does not exist *as a theorem rather than an omission*: the
compiler refuses the impl with `error[E0004]: non-exhaustive patterns: Or::Refused not covered`, which is
precisely the observation that a refusal has no image in the total carrier. The type system states the
missing coercion for me.

The coeffect coercion, `Relaxed → Strict`, exists: a value carrying a permissive licence may be viewed as
carrying a restrictive one, because declining a liberty is always sound. The reverse must not exist.

**The mixed-operand rule then has one statement and two instances.** "The result grade is the least grade
both operands can be coerced to." Nothing in that sentence says join or meet. The direction of the
coercion decides:

| operands | fallibility (coercion runs up) | fidelity (coercion runs down) |
|---|---|---|
| both permissive | `Total` | `Relaxed` |
| mixed | `Fallible` | `Strict` |
| both restrictive | `Fallible` | `Strict` |

Both columns land on the conservative answer and they get there by opposite lattice operations, because
the conservative end is the top of one lattice and the bottom of the other.

### 4.2 What that says about `Resolve`

`arvo-strategy/src/cross_strategy.rs:3` reads, verbatim, "`Resolve<S1, S2>::Out` projects the more
conservative of two". File 12 section 2 found that phrase hand-waved at four presets and undefined at
ten axes, and proposed defining the join where one exists and refusing where none does.

I would sharpen the finding and I think it changes the proposal. **"More conservative" is not a lattice
operation.** It is a human judgement that happens to name the join on some axes and the meet on others,
and a single projection computing one of the two is silently wrong on every axis that wants the other.
`--cfg bad_join` builds exactly that mistake, a uniform join over the fidelity axis, and the compiler
refuses it with `error[E0277]: the trait bound Strict: ViewC<Relaxed> is not satisfied`. The join has no
coercion witness from the strict operand, which is what "silently upgrading a licence nobody asked for"
looks like once the coercion is what defines the projection.

So the shape I would propose instead of file 12's two options is a third: **do not define a join per axis
and do not define a meet per axis. Define the coercion per axis, and derive the mixed grade as the least
common target.** Three things follow that neither of file 12's options gives.

Where a coercion exists in one direction, the mixed grade is forced and nobody chooses it. Widening a
stored value from minimum to doubled width is total and injective, so `StoredWidth` coerces upward and its
mixed grade is the maximum. Declining a liberty is free, so `Fidelity` coerces downward and its mixed
grade is the minimum. Neither needed a design decision.

Where no coercion exists in either direction, the mixed grade has no least common target and the operation
must refuse or requantise explicitly. `Layout` is the clear case: dense and bitpacked do not coerce into
each other for free, both need a repack, and the repack is a quantisation the consumer should see. That is
file 12's "refuse with diagnostic where no join exists", now derived from the coercion rather than
enumerated by case analysis.

And the diagnostic writes itself, because the missing thing is a named trait impl rather than an absent
match arm in a projection table.

### 4.3 The finding I did not go looking for, which is the sharpest thing in probe 6

I built `--cfg bad_grant` expecting a third compile error. It compiles clean.

`impl ViewC<Relaxed> for Strict {}` is one line, adds a licence nobody granted, and there is nothing for
the compiler to check, because the coeffect coercion carries no data. On the effect side the missing
coercion is missing because the function cannot be written. On the coeffect side it is missing only
because nobody typed it. With the line present, a strict operand flows into a relaxed context and the
probe prints:

```
a licence-gated body, on [1e16, -1e16, 1, 1]:
  under Strict   2.0
  under Relaxed  0.0
  a Strict operand in a Relaxed context: 0.0   <-- WRONG, and it COMPILED
```

A hundred percent of the answer, from a one-line impl, with no diagnostic anywhere.

**So the two grades need different enforcement, and the design currently plans neither for the second
one.** Fallibility's order is policed by the type system for free. Fidelity's order is a hand-typed leaf
fact of exactly the kind Thread C exists to kill, and it needs the same treatment Thread C gives a
resolution constructor: the liberty sets are data, inclusion between them is decidable, and a const check
can refuse an impl whose declared direction disagrees with the sets it claims to relate. That is a small
mechanism, it reuses the witness discipline verified at `07_probes/a7_door_checks_directly.rs`, and
nothing in this dive or the last one has named the need for it.

### 4.4 Why I am not claiming a semiring, and the reading that says the whole vocabulary buys nothing

I have called fidelity a coeffect and I want to be exact about how much of that vocabulary I am claiming,
because `13c`'s standard is that the structure named should be the structure the mathematics has and not
one adjacent to it.

A coeffect grade proper lives in a semiring: addition for contraction (a variable used in two places sums
its demands) and multiplication for nesting (a demand inside a demand composes). **Neither operation is
exercised here.** A value is not used "r times at a fidelity"; the licence sits on the operation and a
subterm needing a liberty its enclosing licence does not grant is a check, not a product. So writing
`(L, ⊓, ⊔, ⊤, ⊥)` and calling it a semiring would be naming a structure the mathematics does not have,
and I decline it. What is real is an ordered set with a downward-closed coercion, and what the coeffect
literature contributes is the *direction*, which sections 4.1 through 4.3 show is load-bearing and
measurable.

**The reading that says even that buys nothing.** Fidelity could be read as a plain selector: a type
parameter choosing between two implementations, with a "must match" rule and no modality at all. Under
that reading the meet follows from the coercion direction, which the selector reading also has, and the
word coeffect adds vocabulary without content. I think that reading is defensible and I have not found a
fact that refutes it. What the graded framing bought me in practice was the question that produced 4.3, in
the specific form "the two coercions should have symmetric proof obligations, do they", which the selector
framing does not prompt. That is an argument about what the vocabulary is good for rather than about what
it is true of, and it should be weighed as one.

## 5. Laws are neither grade, and separating them from the grade moves `Precise`

### 5.1 What a law is, in this frame

A law is not an index on anything. It is an equation of the algebra at a point in the grade, and it is
what licenses a rewrite. Willsey's section 1 has this exactly right (`15_willsey...md:44-57`), from the
opposite direction, and I want to state where the graded reading agrees and where it adds something.

It agrees that arvo wants the gate and not the licence-generating engine, for the reasons file 15 gives.
It adds that the gate has an index, and that the index is currently fused into the check.

### 5.2 Associativity is two conditions, and the design checks them as one

The draft derives its laws from translation stability under Kleene equality (3.4). Kleene equality says
"both refuse, or both return and agree", which fuses two questions: does the grade agree between the two
groupings, and does the value agree. Probe 7 separates them, and they come apart:

`Refuse` has numeric diameter **0** at every fold length from 2 through 5, and **10992 of 32768** inputs
at `n = 5` where one grouping returns and another refuses.

Read that carefully, because it inverts a stated consequence of the design. **Every grouping of a
`Precise` fold that returns at all returns the same number.** `Precise`'s regrouping sensitivity is
entirely a definedness phenomenon and contains no numeric disagreement whatsoever. The draft's
counterexample at 3.4, `(127 + 1) + (-1)` refusing while `127 + (1 - 1)` returns 127, is real and is a
grade disagreement, not a value disagreement, and the design currently records it as unfaithfulness in
the same column as signed clamping's genuine numeric divergence.

In the graded frame these are different facts and both are worth having:

**Grade-invariance under regrouping.** Does the multiset of refusal causes depend on how the fold was
grouped? For `Precise`, no: it does. For `Hot` and `Warm`, vacuously yes, since their grade is empty.

**Value-agreement at a fixed grade.** On the inputs where every grouping returns, do they agree? For
`Precise`, yes, always. For signed `Warm`, no, and by up to the full representable range.

Which gives `Precise` a law the design does not currently have a name for: **partially associative**,
meaning associative on its domain of definedness. That is not a hedge, it is a standard notion, and it is
exactly the right contract for a combinator that is willing to propagate a refusal. A four-way accumulator
split over `Precise` is sound in the sense that matters, provided the combinator agrees to refuse
whenever any grouping would have, which is what the join over firing sites already computes. So the
mechanism to state and enforce it is one the design has already built for a different purpose.

I hold the alternative and it is not weak. One could say that a fold whose *definedness* depends on
grouping is unusable regardless of what the values do, because a consumer who regroups for speed and then
gets a refusal they would not have got has been surprised in a way no numeric guarantee compensates for.
Under that reading Kleene equality is right to fuse the two and `Precise` belongs where the draft puts it.
The choice turns on whether a refusal is an answer or the absence of one, which is a question about what
`Precise` is *for*, and it is not mine to settle.

### 5.3 What the grade does and does not do to the "make the law graded" idea

The obvious move once you have a fidelity grade is `Associative<Op, Fidelity>` rather than
`Associative<Op>`, so that `FastFloat` derives associativity at `Relaxed` and `StrictFloat` does not. That
would dissolve two standing findings at once: McSherry's 1024-to-1 over-strictness
(`13_mcsherry...md:230-234`) and his observation that `arvo-spectral/src/power.rs:71` is arvo's one real
fold and an associativity gate refuses it at every strategy (`13_mcsherry...md:490-496`).

I want to develop that idea and then say plainly where the measurement stops it, because the second half
matters more than the first.

**The objection to answer first.** A consumer-declared grade that makes a derived law true is a lie
generator: write `Relaxed` and everything is associative, which is exactly the hand-typed leaf fact Thread
C exists to kill. The answer, in principle, is that the grade is an *input to the derivation* rather than
an override of it. `Associative<Op, Relaxed>` would still have to be derived, from the axes plus a
definition of what equality means at `Relaxed`, and would still refuse arithmetics whose regrouping
disagreement is not the kind `Relaxed` forgives.

**And the measurement says that definition does not exist.** For `Relaxed` to be an equality-with-slack
grade there has to be a slack. Probe 7 says there is not: the relative gap between groupings of an
`f64` fold reaches 100% on ordinary catastrophic cancellation, and reaches NaN, which is not a distance,
the moment an infinity and its negation are both present. No epsilon exists, at any fold length, and no
quantitative reading survives.

So the honest characterisation of `Relaxed` is not "equality with slack". It is **the replacement of a
function by a relation**: the answer is one of the set reachable by the licensed rewrites, and the design
declines to say which. That is what `-ffast-math` actually is, it is why numerical people distrust it, and
saying it in the type is more honest than any epsilon would have been.

Three consequences, and the third is the one I would act on.

`Deterministic` cannot be derived for a `Relaxed` composition, and now it has a reason rather than a
stipulation. Lattner reached the same conclusion from the toolchain side (`12_lattner...md:99-107`); this
reaches it from the equational side, and the two together make it a fact about the design rather than an
observation about a toolchain.

A graded law is expressible for `Relaxed` and its content is weaker than "associative": it says the
regrouping stays inside the licensed answer set, which is true by construction and therefore states
nothing. That is not a defect of the idea, it is the idea correctly reporting that `Relaxed` has already
given away the thing a law would have protected.

And for the fixed-point strategies, where a metric does exist, the diameter grows to the whole
representable range by a five-element fold, so no budget survives there either. **The graded law is
therefore not the general answer, and I want that recorded as a negative result rather than left as an
attractive direction for the next member to spend a dispatch on.** What remains true is narrower and
worth keeping: the law needs the fidelity index in its *key*, so that `StrictFloat` and `FastFloat` can
answer differently, even though at `Relaxed` the answer degenerates.

### 5.4 The supply-set reading, which is the prior probe's and which I would keep

`17_probes/03_supply_table.rs` computes, per arithmetic, which of seven named source-level rewrite
generators is sound, rather than which named structures hold. It reproduces exactly, and I think its
framing is better than the one this dive has otherwise been using, for a reason worth stating: the
generators are what a combinator actually performs, and the structures are a vocabulary for talking about
them. `REGROUP` is what a four-way accumulator split does. `COMMUTE` is what a reversed tail does.
`NEUTRAL` is what skipping an unwritten slot does. Naming a structure and then asking which generators it
licenses is one indirection more than the design needs.

Its most useful row for my question is one nobody has cited: at Q2.2 with an *unbounded* range, where no
recovery rule fires at all, `MUL_REGROUP` and `DISTRIB` still fail. The failure is the unconditional
`>> FRAC`, not the range. That is the `Event` component of the grade in section 1, isolated, and it is
what makes multiplication a different problem from addition rather than a harder instance of the same one.

## 6. What this does to the proposal that named structures be derived from atomic facts

Dolan's reading two (`14_dolan...md:255-296`) proposes that `Associative`, `Commutative`, `HasIdentity`,
`Idempotent`, `DistributesOver` and `Monotone` each be an atomic, independently-derived fact, with named
structures as blanket impls over conjunctions. Willsey endorses it as the applicability-condition layer a
rewrite engine would need (`15_willsey...md:195-204`). Fallin endorses it as the fact set a
monomorphisation-gated dispatch consumes (`16_fallin...md:218-224`).

The graded reading agrees with all three and changes one thing: **the atoms are indexed.** Not
`Associative<Op>` but `Associative<Op, ℓ>`, and separately the two conditions of section 5.2 rather than
their Kleene fusion. So an atom becomes a fact about an operation *at a licence*, over a grade, and the
derived named structures inherit the index.

Three things I would flag about that.

It does not increase the proof burden the way it looks like it should. Dolan's argument that the
sketch-and-bench obligation is paid once per atom rather than once per named rung survives, because the
index ranges over a two-element set with a coercion, so the `Strict` instance is the one that gets
checked and the `Relaxed` instance is derived from it by the section 5.3 degeneration. One check, two
instances.

It sharpens Dolan's own open item at `14_dolan...md:419-428`, whether the design's two uses of the word
`Monotone` unify. In the graded frame they clearly do not: one is a property of a unary quantisation
function and one is order-preservation of a binary operation's partial application, and they differ in
arity, not only in scope. A single `Monotone<F, Ord>` general over both would be naming a structure by
its shared word rather than by its mathematics, which is the failure `13c` names. I would keep two traits
with two names.

And the coherence argument Dolan makes, that marker-trait conjunctions do not collide the way the draft's
per-`Resolution` blanket impls did, survives the index unchanged. `Associative<Add, Strict>` and
`Associative<Add, Relaxed>` being simultaneously true is a conjunction and nothing downstream has to pick
a winner.

**On the brief's question of whether deriving a fact by the presence of an implementation is a degenerate
grading, and whether saying so buys anything.** It is one, and I do not think saying so buys much. "There
is an impl" is a grade over the one-element monoid, which is a true statement with no consequences: the
composition rule is trivial, the order is trivial, and nothing follows. The place the observation nearly
pays is that it makes the *next* generalisation visible, since a design that has already written its facts
as a degenerate grading has somewhere to put the index when section 5.3 or section 3 needs one. That is a
small argument for writing `Fact<Op, ℓ>` from the start with `ℓ` inhabited by one marker today, and it is
the only thing I would take from the observation.

## 7. The two direct questions

**Where a total operation meets a partial one.** Join in the effect component, lift the total side, result
partial. One implementation per grade pair, which is file 07's `LiftGrade`. The lift is a real function and
the type system checks it, per section 4.1.

**Where a strict operation meets one licensed to reorder.** Meet in the licence, result strict, and there
is no lift because there is nothing to convert. This is the asymmetry that makes the effect and coeffect
distinction load-bearing rather than decorative, and it is the case where the shipped `Resolve` sentence
would be right by accident and its generalisation to ten axes would be wrong by construction (section 4.2).

The pattern behind both, which I would put in the spec as one line: **a coercion that carries data is
checked by the type system and a coercion that carries permission is not, so every permission-shaped
coercion in this design needs a witness and every data-shaped one does not.**

## 8. The downstream contract, designed

Op's obligation at `16c` is that every boundary this design stops at gets a design rather than an
observation, naming what a downstream target reads, what it can determine, and what arvo needs back. The
boundary that falls to me is fidelity, since files 12, 15 and 16 all left it at arvo declaring and a build
layer lowering.

**The premise all three share, which I tested and believe is false.** Each reasons about a licence that
has to *cross* the type-erasure boundary: file 15 proposes arvo emit annotations so a backend can search
safely (`15_willsey...md:270-276`), file 16 establishes that no annotation mechanism exists on this
toolchain that is not forbidden and that there is no LLVM-side concept to receive one for fixed point
(`16_fallin...md:116-124, 164-205`). Both are correct about the toolchain and both take for granted that
the licence has to cross.

**In C it has to cross because the C compiler owns the operation. arvo owns its own operation bodies.**
That is the whole difference, and it dissolves the boundary rather than bridging it.

`17_probes/08_the_licence_never_leaves_the_crate.rs` checks each named fast-math liberty for source
expressibility, and checks that each one actually changes an answer, because a licence that changes
nothing is not a licence:

| liberty | source form under `Relaxed` | strict answer | relaxed answer | differs |
|---|---|---|---|---|
| `reassoc` | regroup the fold | `2` | `0` | yes |
| `contract` | `a.mul_add(b, c)` | `0e0` | `-4.93e-32` | yes |
| `arcp` | `a * b.recip()` | `3ffaaaaaaaaaaaab` | `3ffaaaaaaaaaaaaa` | yes |
| `nsz` | canonicalise `-0.0` away | `8000000000000000` | `0000000000000000` | yes |

Every row is an ordinary generic function branching on an associated const. No feature gate, no compiler
flag, no build-script coordination, and the branch is gone before codegen because monomorphisation is the
only dispatch. `mul_add` in particular is stable and lowers to `llvm.fmuladd`, which file 16's own probe
established directly (`16_fallin...md:108-114`).

**The residue, read out of emitted assembly rather than asserted.** aarch64-apple-darwin,
nightly-2026-05-28, `rustc -O --emit=asm`, counted between the symbol and its `cfi_endproc`:

| function | vector int add | vector fadd | scalar fadd | `ld4` |
|---|---|---|---|---|
| `residue_integer_chain` | 7 | 0 | 0 | 0 |
| `residue_scalar_chain` | 0 | 0 | 5 | 0 |
| `residue_source_regrouped` | 2 | 0 | 48 | 4 |
| `residue_closed_by_intrinsic` | 1 | **1** | 9 | 0 |

A wrapping integer reduction vectorises with no annotation at all, because integer wrapping addition is
already associative and LLVM needs no licence. That is file 16 section 4, measured. A float reduction does
not vectorise: LLVM unrolls and refuses to break the dependency chain. A *source-level* regrouping
recovers the four independent chains, so the instruction-level parallelism McSherry's bench priced at 2x
(`13_mcsherry...md:370-378`) is source-reachable, and it is still scalar `fadd d`. **The vector lanes are
the residue, and they are the only residue.**

And the residue closes. `residue_closed_by_intrinsic` emits `fadd.2d v0, v0, v1` and `faddp.2d` through
stable `core::arch::aarch64`, which is not on `unstable-features.md`'s forbidden list (that list forbids
`core_intrinsics`, a different surface: file 16's probe is about `core::intrinsics::fadd_fast`, not
`core::arch`). Writing that intrinsic, cfg-gated per target feature, is exactly what
`arvo-always-optimal-internals.md` prescribes by default for structural lowering, as its Kind 1, without
waiting for a bench.

**So the designed contract is this, and it is short.**

*What a downstream target reads out of the types.* Nothing. It cannot, per file 16 section 8's erasure
argument, and it does not need to.

*What arvo needs back from a build layer.* Nothing, for fidelity. The fidelity grade gates which
monomorphised body compiles, every liberty class is reachable from a body arvo writes, and the one class
plain portable source cannot reach is reachable from a body arvo writes with an intrinsic. The loop closes
inside the crate.

*What the build layer's role becomes.* Convenience only. `hilavitkutin-build`'s `Pragma::FastMath` bit,
which file 16 traced to a `cargo::rustc-cfg=arvo_fast_math` and nothing else (`16_fallin...md:147-153`),
is honest as a *default selector*: it picks which alias a consumer gets when they do not name a licence.
It should not grow into a codegen-flag emitter, because the flag it would emit applies to a whole
compilation unit and cannot distinguish a `Strict` call site from a `Relaxed` one in the same crate, so it
would grant liberties to compositions that declined them. That is the section 4.3 failure with a build
system holding the pen.

*And one thing the build layer must be forbidden from doing, which nobody has stated.* Any mechanism that
selects among arvo's implementation variants *after* arvo's own type checking has run reopens Thread C's
fourth-pass gap at the build layer: the checked text and the executed text stop being the same text, and
every guarantee the witness machinery establishes is void, with no test anywhere positioned to notice. If
a build-layer knob over arvo's internals is ever wanted, the knob has to be an input to the type-level
selection rather than a post-hoc substitution. I would put that sentence in the spec, because it is the
kind of thing that gets added later for a good local reason by someone who has not read Thread C.

## 9. What I would flag for the next member, unresolved

**The `P(Cause)` versus two-point question in section 3 is a judgement about how many refusal causes the
design expects**, and I have argued it from the free monoid rather than from the roadmap. Somebody who
knows whether divide-by-zero, inexact and invalid are all coming should make that call, not somebody who
finds the construction tidy.

**Section 5.2's partial-associativity reading moves `Precise` and I did not build the combinator.** The
claim is that a regrouping combinator can be sound over `Precise` provided it refuses whenever any
grouping would have, and that the join over firing sites already computes exactly that condition. I
believe it and I did not compile it, and given this dive's record of each pass being broken by whoever
compiled the previous one, it should be read with that suspicion.

**The negative result in 5.3 is a negative result and I want it to survive as one.** The graded law is not
the general answer: there is no epsilon for float and no surviving budget for signed saturating. What
remains is only the index in the key. I have watched attractive ideas get retained in this review after
the measurement that killed them, and this is one worth killing cleanly.

**Multiplication is still untouched by the graded reading**, and section 5.4 says why it is a different
problem: its `Event` grade is nonempty even at `Growth::Exact`, which is the one case every clean
derivation in this round has relied on. Whoever runs the multiplication dive should read
`17_probes/03_supply_table.rs`'s unbounded-range rows first, because they isolate that fact from every
range phenomenon, and should read file 12 section 1 and file 15 section 4 alongside, per both of those
files' own closing requests.

**The witness for the fidelity coercion order, section 4.3, is a small designed mechanism I described and
did not build.** It is the same shape as `07_probes/a7_door_checks_directly.rs` and should be cheap, and
"should be cheap" is what four consecutive Thread C passes each said.

**I did not check whether any shipped arvo body already performs a regrouping or reaches for `mul_add`
today.** File 16 flagged the same gap at its own close (`16_fallin...md:411-415`) and it is still a grep
nobody has run. If any does, it is doing so under a licence that does not exist yet, and the audit wants a
concrete list rather than a hypothetical one before the fidelity axis lands.

**And I have not read `arvo-num-systems` or `notko-hlist`**, either of which could change section 3's cost
argument, since `P(Cause)` wants exactly the type-level set machinery the second one extracted and the
draft lists as reviewed by nobody (`11_current_shape_draft.md:56`).
