# Panel 07: is the type story sound, and where the proof actually lives

**Persona:** Simon Peyton Jones, type-systems and functional-programming lens. Seventh member; read
`01_knuth_mathematical_rigour.md`, `02_kiselyov_type_level_encoding.md`,
`03_jhala_what_is_provable.md`, `04_torvalds_does_it_earn_its_keep.md`,
`04b_op_checkpoint_and_directions.md`, `05_leijen_fallibility_without_poisoning.md`,
`06_muratori_the_consumer_surface.md` and `06b_op_checkpoint_two.md` in full, plus every probe under
`02_probes/`, `05_probes/` and `06_probes/`, before starting.
**Date:** 2026-07-30

**Canon posture.** The brief states, and I confirmed against the panel rule, that there is no
ratified canon for this material; what governs is intent and the settled workspace discipline. I
checked my own proposals against that discipline: the probes below use `const_trait_impl` only
(WATCH-allowed per `unstable-features.md`), no `generic_const_exprs`, no `specialization`, and
nothing here polices a consumer choice.

**What I read in full:** the spec (`202607301200_topic.the-formalization-spec.md`), all eight panel
files, all twenty-seven prior probes, the panel brief, the governing panel rule
(`panels-argue-the-intent-not-the-wording.md`), `arvo-strategy/src/identity.rs`, and
`05_probes/README.md` / `06_probes/README.md` for the build contracts. **What I read in part:** the
talk and the inherited-state file at the passages the spec and prior members cite (the D16 passages
located by grep and read in place), `arvo/src/ufixed.rs` at the `OneRepresentable` bound,
`arvo/src/lib.rs` and `arvo-strategy/src/lib.rs` at their feature gates,
`notko`'s const `ConstTry` impls as cited by 02 and 05.

**Directory listing done** across `mock/design_rounds/` (the three flat files at root are this
round, nothing newer), `mock/research/` (nothing postdates the panel directory), and
`mock/research/sketches/` (nineteen entries, the newest being the two this spec cites). Nothing
supersedes the brief.

**Gates.** I re-ran the whole suite rather than inheriting a count: 654 passed, 0 failed, 9
ignored, 122 binaries, matching every prior member who ran it. I did not re-read every test body the
first six members already audited in their own hands; I spot-checked `identity.rs`'s witness pattern
in source and confirmed the two `#![feature(generic_const_exprs)]` gates every member has flagged
are still present today (`arvo/src/lib.rs:25`, `arvo-strategy/src/lib.rs:11`, both still carrying
the superseded WATCH comment). **Brief-breaking:** the brief's two claims about panel history (05
disproved 02's one-generic-add conclusion; 06 corrected 04's consumer count and diagnostic reading)
check out against the files and probes themselves. On 06's own meta-lesson that a sequential panel
should recompile load-bearing prior conclusions rather than read them, the two conclusions my file
builds on, 05's const-falsification mechanism and 02's computed-truth derivation, are both
re-implemented inside my own probes rather than trusted, and both reproduced. I found no false
premise and I proceed.

**Separation of evidence.** Sections marked *verified* were compiled, run, or deliberately failed
under `nightly-2026-05-28`, from seven probes committed at `07_probes/`, or read at a `file:line`.
Sections marked *reasoned* are argument. Impressions are labelled. I carry more than one reading
wherever the evidence does not force one, and I rule on nothing.

---

## 0. My lens, and the one sentence the panel keeps circling

01 asked whether the mathematics is true. 02 asked whether the encoding enforces what it claims. 03
asked what checks a leaf fact. 04 asked what earns its keep. 05 asked what a refusal costs. 06 asked
what a person types and reads. My question closes the loop: **where, in this design, does the proof
actually live, and does the whole type story compose once every member's repair is applied at
once?** The panel has produced repairs one at a time; nobody has yet checked that they stack, and
nobody has answered op's Thread C question in the form op asked it: a shape where the check **is**
the typestate rather than a mechanism bolted alongside.

Up front, one framing observation that runs through everything below. The panel's findings are not
six problems; they are one problem seen from six sides. A property was declared where it should
have been computed (01 finding 6), computed where nothing forced the leaves to be true (03), true
where nothing tied truth to the code that runs (05 section 4's oracle is a duplicate of the
semantics, not the semantics), and correct where the consumer cannot read the answer (06 section 6).
Each layer trusted the one below. The design that fixes this is the one where **a single definition
of each semantic object exists, everything else is projected from it, and the projections are
refused by the compiler when they disagree with it.** Sections 1 and 2 build that and measure what
it costs.

## 1. Thread C: the check is the typestate when the marker cannot be minted apart from the semantics. Verified, seven probes.

The state I inherited, in one line each: 03 proposed bounded const falsification and did not
compile it; 05 compiled six checks and found the oracle must be macro-instantiated because a
`const fn` cannot call through a `fn` pointer; 06 found the computed-truth encoding's diagnostic
never fires without a `Proves<C>` repair; op asked for a shape that fits typestate rather than a
check bolted beside it. Every one of those left the same gap 03 named: the check and the
classification are two artifacts, and nothing ties them.

### 1.1 The oracle stops being a duplicate

05's mechanism note says the oracle "must be macro-instantiated" because function pointers are not
const-callable. That constraint is real for function pointers and dissolves for const traits, which
this workspace allows and arvo already uses pervasively. `07_probes/a_witness_typestate.rs`:

```rust
pub const trait Resolve {
    fn phi(x: i32, min: i32, max: i32) -> i32;
}

pub const fn stable<R: [const] Resolve>(min: i32, max: i32, two_sided: bool) -> bool { .. }
```

The check is one generic function calling the recovery map **through the trait bound**. No macro,
no per-resolution restatement, and, the part that answers 03's own strongest objection to its own
proposal: **the oracle is not a second place to be wrong, because it is the same `phi` the runtime
arithmetic calls.** 03's section 3 second reading worried that a hand-written oracle is
symmetrically as trustable as the leaf assertion it checks. When the checked function is the
shipping recovery map itself, that symmetry breaks: the check now verifies that the *declared
classification agrees with the code*, which is exactly the doc-drift class of bug, mechanically
caught. What remains trusted is the checker `stable` itself (one small function, the
translation-stability identity from 01 finding 3, written once) and the width-uniformity
generalisation (03's frontier, unmoved).

One honest boundary, so it is not oversold: per `arvo-always-optimal-internals.md`, the runtime may
route through cfg-gated intrinsics or asm that are not the const path. The witness checks the
**reference semantics**. An optimised path is covered only insofar as the existing house discipline
(tests and benches against the reference) keeps it equal to `phi`. That is the same boundary
`ConstantTime` lives outside of, but much narrower: here there is a reference the check does bind.

### 1.2 The witness, and the three places it can be enforced

The classification trait requires the truth markers the law derivation projects, plus a defaulted
witness whose body asserts declared equals computed:

```rust
pub trait Resolution: const Resolve + Sized {
    type StableOneSided: TruthMarker;
    type StableTwoSided: TruthMarker;
    const WITNESS: () = {
        assert!(stable::<Self>(0, 7, false) == <Self::StableOneSided as TruthMarker>::VALUE, ..);
        assert!(stable::<Self>(-8, 7, true) == <Self::StableTwoSided as TruthMarker>::VALUE, ..);
    };
}
```

(A mechanism note recorded in the probes README: the supertrait must be the always-const
`const Resolve`, not `[const] Resolve`; the conditionally-const form does not give the default body
the obligation it needs, and the error says so.)

Now the part that matters more than the shape, because it is where a paper design would quietly be
wrong: **associated consts are lazy, and every enforcement route has to run through something the
compiler actually evaluates.** I measured all three routes:

1. **Eager per-constructor consts in the declaring crate.**
   `const _: () = <SubstituteZero as Resolution>::WITNESS;`. A lying marker fails the declaring
   crate's own build with `E0080` naming the constructor
   (`a2_lie_fails_at_declaration.rs`, verified). Early, local, the right error in the right place.
   Forgettable: without the line, the lie compiles and runs (`a3_lie_unforced_compiles.rs`,
   verified).
2. **Forcing the witness member at the use site.** An inline `const { <R as Resolution>::WITNESS }`
   in the one generic function through which a resolution's semantics is reached. The lie now fails
   at monomorphisation of the first use (`a4_lie_fails_at_use.rs`, verified), so it cannot ship in
   any program that computes with it. But the member route has a disarm: an implementor writing
   `const WITNESS: () = ();` in their impl silences it entirely
   (`a6_override_disarms_witness.rs`, verified, compiles clean past the eager const too).
3. **The door checks directly.** The inline const in the generic entry point runs
   `stable::<R>` against the markers itself, mentioning no overridable member. The same lie with
   the same disarming override still fails (`a7_door_checks_directly.rs`, verified):

   ```
   error[E0080]: evaluation panicked: this resolution's declared stability disagrees
                 with its own recovery map
     evaluation of `resolve::<SubstituteZero>::{constant#0}` failed here
   ```

So the discipline I would offer op for Thread C, stated as a two-site rule: **the declaring crate's
constructor macro emits the eager witness const (early, good error placement), and the single
generic door through which arithmetic reaches a resolution carries the direct check (late, cannot
be forgotten or disarmed).** With the constructor set sealed, as 02's section 12 already argues on
other grounds and as `arvo-strategy/src/lib.rs:94-95` already practises, the eager consts fire at
arvo's own build and no downstream consumer ever sees the post-monomorphisation form. Sealing
becomes load-bearing for *error placement*, not only for soundness, which is a second reason to do
what the panel already wanted.

### 1.3 Why this is "the check is the typestate" rather than a check beside it

The markers `StableOneSided` and `StableTwoSided` remain exactly what 02's computed derivation
projects; my probe A wires them into the same `StableFor` / `And` / `IsTrue` fold and the
derivation still refuses `SubstituteZero` (verified, the `False: IsTrue` error, with 06's
`Proves<C>` repair applying unchanged). What changed is their **mintability**: a composition whose
arithmetic is ever emitted has, by construction, had its markers checked against its own recovery
map, because the check sits inside the only door to the semantics and monomorphisation is arvo's
only dispatch. The moment of proof is the moment of instantiation. That is, I think, the typestate
reading op asked for: the typestate is unchanged as a surface, and it has become impossible to
inhabit dishonestly for any type that is actually used.

It is also the natural successor to the one witness pattern arvo already ships. The
`OneRepresentable` mechanism (`arvo-strategy/src/identity.rs:69-91`) compresses a computed fact
into a tag and implements the witness only at the true tag, but its use site
(`arvo/src/ufixed.rs:100`) needs a computed const argument in type position, which is precisely why
those crates still carry the forbidden `generic_const_exprs` gate that 02, 03, 04 and 06 have all
flagged as drift. The witness-const shape needs no const in type position at all. So the drift
remediation and Thread C are one edit apart: **the pattern that dies with the forbidden gate has
its replacement here**, and 06's observation that the remediation is also a diagnostic repair gains
a third leg: it is a diagnostic repair, a rule-compliance fix, and the migration path for the tag
pattern, in one round.

### 1.4 Boundaries, and two readings on the check's strictness

Three limits, stated plainly. First, 03's layer 1 is untouched: the identity `stable` encodes is
chosen by a human, and no mechanism certifies it is the right predicate to want. Second, the width
question: my checks run at 3-bit unsigned and 4-bit signed like 05's; the generalisation argument
is still prose. One extension the door makes newly available: since the door knows the concrete
composition, it could run the check at the composition's **actual** width whenever the span is
small enough for the const-eval budget (the check is quadratic in span for stability, cubic for
associativity), falling back to representative widths above a threshold. Whether that is worth the
compile time is exactly the kind of question `arvo-compile-time-last.md` answers in favour of
doing, and exactly the kind the encoding bench of 02 section 9 / 04 section 6 should price; I did
not measure it and say so. Third, the check I wrote asserts **equality**: a constructor claiming
`False` for a property that computes `True` also fails. Two readings on that. Equality catches the
typo in both directions and keeps the marker meaning "the fact", which is what a derivation wants
to project. Implication-only (declared `True` must compute `True`; `False` always accepted) permits
deliberate under-claiming, a resolution whose stability arvo does not want consumers to rely on.
I lean to equality, because an under-claim is a policy statement and deserves its own named marker
rather than a lie in this one, but the choice is real and op's to make.

## 2. Fallibility is a graded structure, and computing the grade in one blanket dissolves three findings at once. Verified.

05's probe A established the handler shape; 05's section 1 measured its cost, a five-line `where`
clause per arithmetic function, and named the repair, a blanket extension trait, without compiling
it. 06's section 8 added the orphan-rule argument for an arvo-owned carrier. 03's section 5 flagged
the notko-absence dependency. `07_probes/b_bounds_collapse.rs` compiles the repair and, I want to
argue, shows these are all one structure.

Name the structure first, because naming it is what makes the next three design questions answer
themselves. The fallibility of a composition is a **grade**: an element of a two-point join
semilattice (`No <= Yes`), with a monotone interpretation into carriers (`No -> Total`,
`Yes -> Fallible`) and a subsumption lift along the order, stated **once per grade pair** rather
than once per rule and payload type, which is what keeps the bounds finite:

```rust
pub trait LiftGrade<G: CarrierOf>: CarrierOf {
    fn lift<T: Copy>(x: Self::C<T>) -> G::C<T>;
}
```

An operation's grade is the join over its firing sites. The aggregate computes it once, in one
blanket impl, and, the addition that makes the collapse actually work, **consumes it once**,
because the resolution operations become methods of the aggregate:

```rust
impl<Q: Quantisation> QuantExt for Q
where
    OverGrade<Q>: Or<UnderGrade<Q>> + LiftGrade<JoinOf<Q>>,
    UnderGrade<Q>: LiftGrade<JoinOf<Q>>,
    JoinOf<Q>: CarrierOf,
{
    type Answer<T: Copy> = <JoinOf<Q> as CarrierOf>::C<T>;
    fn ok<T: Copy>(v: T) -> Self::Answer<T> { .. }
    fn over<T: Copy>(max: T) -> Self::Answer<T> { .. }
    fn under<T: Copy>(min: T) -> Self::Answer<T> { .. }
}

pub fn add<Q: QuantExt>(a: u16, b: u16, max: u16) -> Q::Answer<u16> { .. }
```

Verified output: `B: sat=100 precise=Refused mixed_hi=Refused mixed_lo=Ok(0)`. The mixed
composition refuses above and clamps below into the fallible carrier, neither rule knowing about
the other, and **both arithmetic bodies carry exactly one bound.** 05's five-line clause exists
once, at the blanket, and no consumer or arithmetic author ever restates it.

What this dissolves, item by item:

- **01 finding 6** (`Fallibility` declared, can lie): the carrier is now a projection of the
  computed join. There is nothing left to declare. The spec's `type Fallibility<T>` member
  (spec:155-157) should not survive in declared form, and under this shape it does not need to.
- **02 section 7 / 05 section 0** (who constructs the refusal): the rules construct their own
  answers in their own grade's carrier and the aggregate lifts. No arvo body ever names a
  `ConstFromResidual` bound, so **03's section 5 cross-crate absence dependency disappears
  structurally**: the bound that had to not exist in notko is a bound nothing asks for. The pin 03
  and 04 asked for is still worth an afternoon if any other surface leans on that absence, but this
  surface stops leaning.
- **06 section 8** (orphan rule forces one unwrap per operation on a foreign carrier): the
  interpretation `CarrierOf` is arvo's own trait over arvo-ownable types, which is the arvo-owned
  carrier 05 and 06 reached from layout, codegen and coherence, arrived at here from the type
  structure: the carrier is *the interpretation of the grade*, so of course arvo owns it.
- **05 section 9** (`Growth` adds a second refusal site and nothing says how it composes): under
  the fold this is a non-event. A `Narrowed` intermediate adds a firing site; the operation's grade
  gains one term in its join; no new machinery. Likewise division's `DivideByZero` and any future
  effect: the grade lattice widens (05's row reading), the enumeration does not.

And one thing it clarifies rather than dissolves. 05's load-bearing claim, that delivery is a
`Lowering` member by D54's own test, drew from 06 the counter that a delivery changes what a
consumer's call site looks like. The graded frame names the seam precisely: the **grade** is
policy-derived and law-relevant; the **interpretation of the grade** (sum type, absorbing bottom,
sticky flag, per 05's probes D and E) is where delivery lives. D54's sorting test (spec:32-36) asks
two questions, did the values change and did the cost change, and delivery answers no to the first
and yes to the second, which sorts it as lowering; but a Rust surface has a third sort D54 never
asks about, **did the type of the interaction change**, and delivery changes that. Two readings,
neither forced. Either delivery is a `Lowering` member with the settle-door obligation 05 states
made explicit (the only exit from a bottom-carrying value is `settle()`), and the spec records that
D54 gains a third question it deliberately answers "lowering may change interface shape but never
values". Or delivery is a small fourth contract of its own, `Delivery`, orthogonal to the three,
which is more honest about the sorting and costs one more parameter that the presets fill. I note
that 06's modifier shape (`DeliveredAs<Precise, Absorbing>`) works identically under both, so the
consumer surface does not decide it.

**The cost, stated.** The blanket's `where` clause is real and is re-checked by the trait solver at
every obligation; the compile-cost bench the panel keeps deferring should include this aggregate
shape as a third arm alongside 02 section 9's two. And a subtle point for whoever writes
`arvo-policy`: the aggregate trait is the one place the join and lift bounds may appear. If a
second surface restates them, the collapse is lost and 04's three-in-the-morning debugging scenario
returns. That is a review-discipline invariant unless the restatement is made impossible, which it
can be by making `QuantExt` sealed-blanket-only (no other impls possible), the same sealing move as
everywhere else in this design.

## 3. Does the whole story compose, and the principle underneath it. Verified in parts, reasoned as a whole.

The panel now holds, as proposals: split parameters `Number<N, P, L>` (02), nominal constructors at
every consumer-selected position (06), single-axis modifiers (06), the computed law derivation with
`Proves<C>` (02, 03, 06), the witness discipline (section 1), and the graded aggregate (section 2).
Nobody has checked they stack. I walked the compositions rather than compiling the union, and I
flag it as reasoned: the modifier is just another `Policy` impl, so the aggregate's blanket covers
it (06 verified derivation-through-modifier for the law fold, and the aggregate is the same
projection mechanism); the witness keys on resolution constructors and does not see the modifier at
all; `Proves<C>` parameterises the verdict and is orthogonal to both; the nominal numeral affects
rendering only. I see no interaction that could fail to solve, but "I see no interaction" is
exactly the sentence a compiled union replaces, and the union probe is a half-day: one file, all
five mechanisms, ten compositions, the `.stderr` fixtures 04 asked for written against it. I would
make that the next sketch obligation, replacing the spec's current obligation 1, which tests a
weaker thing.

Underneath the stack of repairs there is one principle, and I would write it into the spec because
it decides future questions without another panel. Rust has no implied bounds; a projection bound
restated at a call site is a cost paid at every call site forever, and 05 section 1 measured what
that looks like. The cure, used by probe B and by 06's probe D (which verified diagnostics stay at
the outermost instantiation), is: **derive in one blanket, consume through one name.** Every
derived fact is computed in exactly one blanket impl and reached through a method or associated
item of that trait; no signature outside the deriving crate ever states a projection. Where the
panel's shapes follow this principle they compose; where the spec's current text does not (the
declared `Fallibility` GAT, the two-impl law partition), the panel found the failures.

The same walk sharpens D16 into the ladder it should have been, collecting 03's demolition,
02's "every derived property bottoms out in an asserted primitive", and section 1 into one
statement with three rungs rather than two:

1. **Computed and witnessed.** The fact is projected from a single semantic definition, and a
   const check refuses disagreement at instantiation. Algebraic laws, membership, integrality,
   range facts: everything whose leaf is a decidable bounded statement. The general rule, and it
   extends well past `Resolution`: **a typestate marker over a decidable bounded domain should not
   be declarable without a witness the const evaluator can refuse.** 02 section 12's
   `IsRadixPower` marker, the dyadic-membership condition that 01's `FullRange<1>` case showed
   silently incomplete, and 01 finding 2's missing numeral-side condition on the law key
   (`AdditionExact` on `ExponentForm`) are all this rung: markers plus witnesses, same mechanism,
   same door.
2. **Declared, total, coherent.** The fact is human-typed, the compiler forces every constructor to
   answer and forbids contradiction, and no witness is possible because the domain is not bounded
   or not decidable. This rung should be nearly empty once rung 1 is real.
3. **Promised.** `Deterministic` and `ConstantTime`, which are claims about emitted code with no
   type-level referent (02 section 11, 03 section 7, 05 section 6), `unsafe impl`-shaped, with the
   discharge being a bench artifact per 03. The witness mechanism does not extend here, and that
   boundary is now mechanical rather than rhetorical: rung 1 is what the const evaluator can see,
   rung 3 is what only the artifact can.

The spec's "cannot lie" sentence survives on rung 1 only, and there it is finally true.

## 4. Smaller items, engagement, and dissents kept short

**On 01 finding 14 and 04's scope dissent (Kulisch).** I side with both, the way 05 did, and add
the operational point: the checker `stable` in probe A **is** the projection-properties frame,
reduced to code. Adopt the frame exactly there, as 05 proposed, one identity per law, cited to 01
finding 3's table, and leave the public vocabulary alone. The frame has now been operationalised
twice (05's macros, my generic const fn); it costs nothing further.

**On the algebra ladder (04 section 5, 06 section 13).** Agreed on deferring the uncalled rungs,
with one type-story addition: under the graded frame the ladder's law markers are not free-floating
declarations, they are the codomain of the derivation fold, so the rungs the derivation consumes
this round (`Magma<Op>` plus `AddAssoc`-class markers) are called code, and the rest are not.
The BACKLOG split 04 proposes falls out exactly along that line, which is a nice check that two
different lenses cut in the same place.

**On the fallibility GAT's `Copy` bound (02 section 7's two-word fix).** Under section 2 the GAT
is gone, but the `T: Copy` question survives on `CarrierOf::C<T: Copy>`, where I put it
deliberately: arvo's payloads are `Copy` by identity, and saying so at the interpretation keeps
every lift a move-free bit copy. If a non-`Copy` payload ever appears, the bound is one place, not
scattered.

**One impression, labelled as such.** The spec's ten axes as associated types on three contracts,
with nominal constructors and the aggregate pattern, is a design I would defend as sound and
implementable, and it is noticeably close in spirit to how a language with type families and no
dependent types does this well: closed constructor sets, facts as projections, witnesses at
instantiation, interpretation functors for effects. The places the panel found rot were, without
exception, the places the design deviated from its own pattern (a declared member here, a blanket
default there). The pattern is right; the discipline of applying it uniformly is what the spec's
revision should encode, as the three-rung ladder and the derive-once-consume-once principle, so the
next deviation is visible as one.

## 5. What I did not get to

The union probe of section 3: all five mechanisms in one file, ten compositions, `.stderr` fixtures
against it. Half a day, and it should gate the source changelist in place of the spec's current
sketch obligation 1.

Witnesses beyond `Resolution`: I claimed the mechanism generalises to `IsRadixPower`, membership
and the numeral-side law condition, and did not compile those. The quantum arithmetic for
`FullRange<F>` involves a rational check, which is still bounded and decidable but is not the
integer loop I wrote; someone should confirm the const-eval shape before the claim is leaned on.

The actual-width witness at the door, and its compile-time price across the real width range. This
is the encoding bench's third arm and it can be written before `arvo-numeral` exists.

Whether the graded frame wants the join computed per operation (as I argued from `Growth`) to also
carry 05's per-operation lemma indexing from its section 10, which would make the law markers and
the grade two projections of one per-operation structure. I suspect yes and that it is elegant in
the way that usually means it is also smaller, and I ran out of budget before testing it.

---

**Summary for the next member.** Thread C has an answer in the form op asked for: the recovery map
becomes a `[const]` trait method, the one definition runtime arithmetic calls; a generic
`const fn` checks 01's stability identity through the bound, dissolving 05's macro constraint; the
classification markers the law derivation projects gain a witness; and the enforcement lives in
two measured places, eager per-constructor consts in the declaring crate (fails early, names the
constructor) and a direct inline-const check in the single generic door to the semantics (fails at
monomorphisation, cannot be forgotten or disarmed, verified against a deliberately disarming
implementor). Lazy associated consts are the trap: a witness nothing forces enforces nothing
(`a3`, verified), and an overridable default member is not a load-bearing site (`a6`, verified).
The shape is the GCE-free successor of the shipped `OneRepresentable` tag pattern, so the
forbidden-gate remediation and Thread C are one edit (section 1). Fallibility is a graded
structure: grade = join over firing sites, carrier = interpretation of the grade, lift = one impl
per grade pair; computing and consuming it in one blanket gives every arithmetic body a single
bound (`b_bounds_collapse.rs`, verified, discharging 05's uncompiled repair), deletes the declared
`Fallibility` GAT, removes the notko-absence dependency structurally, and locates 05's delivery
question precisely at the grade-interpretation seam, where two readings remain for op (section 2).
The panel's five standing proposals appear to compose but have not been compiled together; the
union probe should replace sketch obligation 1 (section 3). And D16 should become a three-rung
ladder, computed-and-witnessed / declared-total-coherent / promised, with the rule that a marker
over a decidable bounded domain is not declarable without a witness the const evaluator can refuse.
I rule on nothing; op decides.
