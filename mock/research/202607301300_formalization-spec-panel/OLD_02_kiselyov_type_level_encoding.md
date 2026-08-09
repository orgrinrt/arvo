# Panel 02: the type-level encoding

**Persona:** Oleg Kiselyov, type-level encoding lens. Second member; read `01_knuth_mathematical_rigour.md`
in full before starting.
**Date:** 2026-07-30

**What I read in full:** the spec (`202607301200_topic.the-formalization-spec.md`), panel file 01, both
sketch FINDINGS under `mock/research/sketches/202607300500_*` and `202607300600_*` plus
`202607300600_.../01_derived_laws.rs`, the FINDINGS of
`mock/research/sketches/202607291400_const-args-under-min-gca/`, the panel brief and the governing panel
rule. **What I read in part:** the talk and the inherited-state file at the passages 01 and the spec
cite; source at `arvo-strategy/src/{lib,axes,identity,container}.rs`, `arvo/src/{lib,ufixed}.rs`, notko's
`consttry_const_path.rs` / `just_consttry_const.rs` / `outcome_consttry_const.rs`, and the whole of
`arvo/tests/ui/`. **Directory listing done** across `mock/design_rounds/`, `mock/research/` and
`mock/research/sketches/`; the three flat files at `design_rounds/` root are this round and nothing newer
supersedes the spec.

**Separation of evidence.** Sections marked *verified* were compiled or run under the pinned
`nightly-2026-05-28` from fourteen probe files I wrote for this panel, committed alongside it at
`mock/research/202607301300_formalization-spec-panel/02_probes/` and each named for the section that
cites it, or read directly out of source with a `file:line`. Sections marked *reasoned* are argument, and I say so.
I have tried to carry two readings wherever the evidence does not force one, and I rule on nothing.

---

## 0. Gates, and one thing 01 did not check

The suite runs green: 654 passed, 0 failed, 122 binaries, matching 01's report. I re-ran it rather than
inheriting the number.

01 says the compile-fail refusals are "pinned under `tests/ui/`" and moves on. Given this repo's own
precedent, a `.stderr` snapshot that captured a typo's error rather than the contract's, that sentence was
worth spending five minutes on, so I read all nine fixtures. They are real. Every
`no_multiplicative_identity_*.stderr` asserts `the trait OneRepresentable<1> is not implemented for
Picker` under the `#[diagnostic::on_unimplemented]` message, at eight distinct (signedness, strategy)
combinations, and `no_signed_identity_on_unsigned.stderr` asserts the different error it should. The
suite on this surface is honest and 01's summary of it stands.

**One premise I do want on the record before the lens work**, because it changes how any claim of the form
"arvo currently does X" should be read. `arvo/src/lib.rs:25` and `arvo-strategy/src/lib.rs:11` still carry
`#![feature(generic_const_exprs)]`, each under a comment calling it "WATCH-tier unstable feature,
soundness-vetted". That disposition was superseded by op on 2026-07-28;
`.claude/rules/unstable-features.md` now lists the feature as FORBIDDEN and names those two exact lines as
"DRIFT to remediate". So the mechanism inventory the spec draws on is partly drawn from code that has to
change before any of this ships, and section 8 below is where that bites the spec directly. This is not a
false premise in the brief, which states the rule correctly. It is a false premise in one of the spec's
own arguments.

## 1. Where I disagree with 01, and where I sharpen it

I agree with 01's substantive findings and will not restate them. Three places I read differently.

**01 treats the `SubstituteZero` counterexample as a mathematical slip. I think it is a structural
consequence of the encoding, and that matters because the repair differs.** A slip is fixed by fixing the
row. A structural consequence recurs. `impl<A: Resolution, B: Resolution> AddAssoc for ((A, B), Unsigned)`
(spec:212) is a blanket quantified over every constructor of `Resolution`, **including ones that do not
exist yet**. It is a default case in a fold, and the mathematics has no default case. Section 4 shows an
encoding in which the bug is unwriteable, and section 4 also shows that the same blanket currently hands
`AddAssoc` to a **downstream** marker, since `pub const trait Resolution {}` (spec:137) is shown unsealed
while arvo's shipped analogue at `arvo-strategy/src/identity.rs:89` is sealed for exactly this reason.

**01's finding 3 proposes replacing `Faithful` with a `TranslationStable` marker, and I think that is
right mathematically and insufficient structurally.** Swapping which marker the blanket is bounded on
leaves the blanket. What buys the guarantee is not the marker's name, it is whether a new constructor can
enter the quantification without answering for itself. Verified in section 4: making the classification a
member of the trait turns a new constructor into `E0046: not all trait items implemented`, at the
constructor's own definition site, which is where the obligation belongs.

**01's finding 6 says `Fallibility` should be derived rather than declared. Verified, and the derivation
is only half the problem.** Section 7 shows that even a correctly derived `Fallibility` leaves arvo unable
to write one generic arithmetic operation, because nothing in scope can *construct* the refusal. The two
problems have one repair and it is worth stating as one.

## 2. What the sketch validated is not the shape the spec has. Verified.

`202607300600_.../01_derived_laws.rs:88-90` conditions on axes that are **direct type parameters** of the
composition:

```rust
impl<Fmt, Sign, Round, Over, Grow> Semigroup<Add> for Num<Fmt, Sign, Round, Over, Grow>
where (Over, Sign): AddAssoc {}
```

The spec's composition is `Number<N: Numeral, S>` (spec:61), where the same axes are **associated-type
projections** through `Numeral` and `Policy`. That is a different trait-solving problem. I rebuilt it
(`a_projected.rs`) and it resolves:

```rust
impl<N: Numeral, S: Policy + Lowering, Op> Semigroup<Op> for Number<N, S>
where ((<S::Quantisation as Quantisation>::OverRange,
        <S::Quantisation as Quantisation>::UnderRange),
       <N as Numeral>::Sign): AddAssoc {}
```

So obligation 3 (spec:333-334) will pass. **The thing the sketch reported as its most interesting finding
does not survive the move**, and the obligation as written would not notice. The sketch's FINDINGS calls
out the diagnostic quality as "itself a finding": the error named the composition's own pair and listed
the pairs that work. Under the spec's shape with `Faithful` interposed, the same refusal
(`a_refusal.rs`, signed clamping) reports:

```
error[E0277]: the trait bound `TowardNegative: Faithful` is not satisfied
   |     fold::<Number<I16, Warm>>();
help: the trait `Faithful` is implemented for `ReduceModulo`
```

emitted twice, once per operand position. The consumer wrote `Warm` and is told about `TowardNegative`, a
marker that appears nowhere in their source, and the actionable help is a list of which internal markers
are `Faithful`. Two readings, and they are not exclusive. Either obligation 3 should be restated
behaviourally, "produces a refusal that names the composition the consumer wrote", which is the property
worth having and is testable as a `.stderr` fixture in exactly the shape `arvo/tests/ui/` already uses.
Or the interposed classification is the wrong place to put the derivation, which is section 4's argument
and would restore the diagnostic for a different reason.

## 3. The coherence budget of "absence of an impl is the mathematical fact". Verified.

The spec's mechanism has a scaling limit that nothing in the round has met yet, and it has no escape under
this workspace's permitted features.

The two law impls (spec:212, 215) partition on `Signedness`, so they coexist. Now add one more true fact,
of a shape anyone would want to state once: wrapping addition folds whatever the domain is, because
`Z/2^n Z` is a group either way. `b_coherence.rs`:

```rust
impl<A: Resolution, B: Resolution> AddAssoc for ((A, B), Unsigned) {}
impl<A: Faithful,   B: Faithful>   AddAssoc for ((A, B), Signed) {}
impl<S: Signedness> AddAssoc for ((ReduceModulo, ReduceModulo), S) {}
```

```
error[E0119]: conflicting implementations of trait `AddAssoc`
              for type `((ReduceModulo, ReduceModulo), Unsigned)`
```

`b_minspec.rs` adds `#![feature(min_specialization)]` and the error is byte-identical, because neither
impl is more specific than the other: they are incomparable, and `min_specialization` orders only a chain.
Full `specialization` is forbidden by `unstable-features.md`, so there is no third option.

The general statement, reasoned from the verified case: **an encoding in which absence of an impl carries
the fact can only express index sets that partition without overlap.** As soon as the true index set is a
non-trivial boolean combination, "associative iff (one-sided domain and stable at the reachable end) or
(both ends faithful)", you must hand-flatten the disjunction into disjoint impls, and every future fact
about a new operation risks re-flattening the whole table. The law table today has one operation. D2 adds
multiplication and distributivity, which 01's finding 3 notes will need the translation-stability identity
re-instantiated per operation. That is the direction in which this wall lies.

## 4. Computing the law's truth value instead of partitioning its impls. Verified, offered as one of two.

`c_computed.rs`, which compiles and runs. Each resolution states its own lemmas as trait members; a
type-level `And` folds them; one impl conditions the structure on the result.

```rust
pub trait Resolution {
    type StableOneSided: TruthMarker;   // enough for a one-sided domain
    type StableTwoSided: TruthMarker;   // translations in both directions
}
impl Resolution for SubstituteZero { type StableOneSided = False; type StableTwoSided = False; }

impl<N: Numeral, P: Policy, L: Lowering> Semigroup<Add> for Number<N, P, L>
where ((P::Quantisation::OverRange, P::Quantisation::UnderRange), N::Sign): AddAssocOf<Out = True> {}
```

Four properties, each checked:

| Property | How it was checked | Result |
|---|---|---|
| the whole table is one impl | `c_computed.rs` runs, three compositions hold | section 3's wall cannot be reached |
| 01's counterexample refuses | `c_refusal.rs`, `Number<U16, Sc0, Dense>` | `((SubstituteZero, SubstituteZero), Unsigned)` named in the error |
| a new constructor cannot slip in | `c2_totality.rs` adds `StochasticRound` | `E0046: missing StableOneSided, StableTwoSided` |
| the diagnostic is recoverable | `c4_diag.rs` routes through an `IsTrue` marker | the `on_unimplemented` message fires |

The third row is the one I care about. The failure lands at the constructor's own `impl Resolution` line,
not at some distant law table, and it is impossible to add a rounding mode to arvo without answering the
question that 01 found the current shape answering wrongly by default.

The fourth row carries a caveat worth writing into whichever encoding wins. Bounding on
`AddAssocOf<Out = True>` produces `E0271`, an associated-type mismatch, and
**`#[diagnostic::on_unimplemented]` does not fire on E0271**. Routing the same condition through a marker
(`<... as AddAssocOf>::Out: IsTrue`, `impl IsTrue for True`) makes it `E0277` and the attribute fires,
giving the quality that `arvo-strategy/src/identity.rs:81-84` already demonstrates for
`OneRepresentable`. Small mechanical detail, large difference in what a consumer sees.

**The honest cost, and the reason I offer this rather than recommend it.** A declared member is an
assertion. `impl Resolution for ReduceModulo { type StableTwoSided = True; }` is a human claiming a lemma,
exactly the thing D16 (spec:190-193) puts in the second category. What the encoding buys is not
derivation, it is **exhaustiveness**: the assertion becomes total over the constructor set and the
compiler enforces that it was made. So I think D16's binary is not a binary. Every derived property
bottoms out in some asserted primitive, and the design question is only where that assertion sits and
whether the type system forces it to be discharged for every constructor. Stating that openly seems better
than a rule that says "computed cannot lie", because the current encoding does have a place where the
assertion hides, and 01 found it by hand rather than by mechanism.

**The cheaper alternative, for the panel to weigh against this one.** Seal `Resolution`, keep the blanket,
and adopt a discipline that no law impl may be blanket over a marker trait. That costs no mechanism and
fixes the downstream-constructor hole immediately. It does not fix section 3's coherence wall, and it
relies on a person noticing at review time, which is the enforcement mechanism this workspace's rules
generally say not to rely on.

## 5. The two contracts the spec calls independent share one type parameter. Verified.

The spec says it plainly at :66-68: a `Lowering` member "changes only the emitted code and the bytes held,
and conditioning a law on one would be conditioning correctness on a storage choice." It then declares
`pub struct Number<N: Numeral, S>(..) where S: Policy + Lowering;` (spec:61).

`d_fusion.rs` conditions `Semigroup<Add>` on `<S as Lowering>::Layout` and compiles clean. The invariant is
prose. Nothing types against it, and a reviewer reading an impl header sees `S: Policy + Lowering` in both
the legitimate and the illegitimate case.

This is worth reading against the shipped source, because the fusion is not new.
`arvo-strategy/src/axes.rs:159-190` already bundles one policy member and two lowering members onto one
marker:

```rust
pub const trait HasAxes {
    type Overflow: OverflowPolicy;   // policy
    type Width: ContainerWidth;      // lowering
    type Layout: StorageLayout;      // lowering
}
```

So on this axis the spec renames the existing structure rather than changing it, while adding an argument
for why they should be separate. Three readings.

Split the parameter: `Number<N: Numeral, P: Policy, L: Lowering>`, with `Hot` and friends as alias sets
filling both. Every law impl then mentions `N` and `P` and cannot mention `L`, which makes the spec's own
sentence a typing fact rather than a review note. I used this shape in `c_computed.rs` and it costs one
parameter and nothing else. The public spelling is unaffected, because the presets are aliases anyway
(spec:315-318).

Keep the fusion and force laws through a projection newtype, so a law impl is written against
`PolicyOf<S>` and syntactically cannot reach a lowering member. Cheaper in parameters, and it is still
discipline: someone can write the unprojected form.

Accept that the three-contract split is documentation. Defensible, but then the round should say so,
because "Cold is the shipped proof they are independent" (spec:68) is being offered as evidence for a
structural claim the structure does not make.

## 6. `Direction: Resolution` asserts a containment that is false in both directions. Verified.

01's finding 5 says the hierarchy admits undefined points. I want to sharpen it into a claim about the
shape, because the repair follows from the shape rather than from the cases.

The spec's relation is a chain: `Direction: [const] Resolution` (spec:138), read as "everything usable
between neighbours is usable at the range ends". The actual relation is three partially overlapping sets.

| Rule | usable between neighbours | usable past the top | usable past the bottom |
|---|---|---|---|
| `TowardNegative` | yes | yes, clamps to MAX | no, names nothing |
| `TowardPositive` | yes | no, names nothing | yes, clamps to MIN |
| `TowardZero` | yes | yes | yes |
| `AwayFromZero` | yes | no | no |
| `ReduceModulo` | no, no modulus in a cell | yes | yes |

`ReduceModulo` sits in `Resolution` minus `Direction`, which the chain expresses. `AwayFromZero` sits in
`Direction` minus usable-at-either-end, which the chain **cannot** express, so it is silently admitted.
`e_direction.rs` builds a `Quantisation` whose `OverRange` and `UnderRange` are both `AwayFromZero` and it
compiles. That composition's range rules name no value at all.

`g_indexed.rs` replaces the chain with per-position capability, `UsableBetween` / `UsableAbove` /
`UsableBelow`, and both refusals arrive as errors:

```
error[E0277]: the trait bound `ReduceModulo: UsableBetween` is not satisfied
error[E0277]: the trait bound `AwayFromZero: UsableAbove` is not satisfied
```

The first is the refusal the spec already celebrates (spec:164-165). The second is the one it currently
admits. Three traits, no subtrait relation, and the vocabulary is still shared across positions, which was
the point of the chain in the first place. I do not think there is a cost here beyond one extra trait
name, but I state it as a suggestion rather than a conclusion because `ToEven` and `ToOdd` at a range end
are parity-dependent rather than plainly undefined, and someone should decide whether they are
`UsableAbove` before this is written down.

## 7. The fallibility projection. Verified, and obligation 5 tests the easier half.

Obligation 5 (spec:337-338) asks that `type Fallibility<T>: notko::ConstTry<Output = T>` type-check with
`Just<T>` and `Outcome<T, _>` both satisfying it.

**As literally written it does not compile.** notko's const path bounds both impls on `Copy`
(`notko/src/just_consttry_const.rs:9` is `impl<T: Copy> const ConstTry for Just<T>`, and
`outcome_consttry_const.rs:9` is `impl<T: Copy, E: Copy>`). `f_gat.rs` reproduces the two errors, and the
obligation passes only once the GAT reads `type Fallibility<T: Copy>: ConstTry<Output = T>` and the error
type is `Copy` too. That is a two-word fix and I mention it only because the obligation is written as a
pass/fail gate and would fail on its own text.

**The question the obligation does not ask is the one that decides the shape.** A generic operation must
sometimes return the refusal. `f2_refusal.rs` writes it with the only bound that constructs a failure
generically:

```rust
fn add_or_refuse<Q: Quantisation>(a: u32, b: u32) -> Q::Fallibility<u32>
where Q::Fallibility<u32>: ConstFromResidual<Outcome<Infallible, OutOfRange>>
```

The body compiles. The **call site at a total composition** does not:

```
error[E0277]: the trait bound `Just<u32>: ConstFromResidual<Outcome<Infallible, OutOfRange>>`
              is not satisfied
help: but trait `ConstFromResidual<Infallible>` is implemented for it
```

which is correct and is the whole point: a total quantisation must not be constructible from an
out-of-range residual. The consequence is structural and the spec does not mention it. **arvo cannot have
one generic `add` over all compositions.** Either the operation splits into a total path and a fallible
path selected by the composition, which duplicates every arithmetic body, or the recovery constructor
moves onto the resolution itself so that only `Refuse` can produce a refusal and the branch is never
generic.

The second is the same repair 01's finding 6 proposes for a different reason, and I read that as
corroboration rather than coincidence: the classification of a resolution and its behaviour are one thing,
and separating `Fallibility` onto `Quantisation` (spec:156) split them. Put `CanRefuse` and the recovery
function on `Resolution`, fold `Fallibility` out of the five members, and both problems close together. A
second reading worth stating: keep `Fallibility` declared and accept the split arithmetic surface, on the
grounds that a total `add` and a refusing `add` genuinely want different bodies anyway. I do not believe
that, because `Growth` and the quantisation triple are identical between them, but it is arguable and
someone should argue it before the first line of `arvo-policy` is written.

## 8. The width and exponent encoding is mixed, and the mix is exactly where the forbidden feature was needed. Verified.

This is the finding I would most like the round to take, because it is cheap now and expensive later.

The spec encodes two things of the same kind two different ways. `type LogicalWidth: Width` (spec:45) is
an **associated type**. `Stored<const BITS: Width, U: Underflow>` (spec:95) and
`Implicit<const EXPONENT: Exponent>` (spec:90) are **const parameters**. And spec:118 derives across them:
"the significand derives by subtracting the exponent field and the sign bit".

`mock/research/sketches/202607291400_const-args-under-min-gca/FINDINGS.md` settled what that costs, and
its conclusion is stated in its own words: "there is no way to compute a const from a generic const
parameter, in type position, under any feature this workspace permits." It probed the bare shape, the
`min_generic_const_args` shape, and rustc's own suggested `const` block, and all three refuse. It found
two gate-free escapes: an impl table with the const as a standalone argument (Pattern C, which
`BitsContainerFor` and `Project` already use), or the parameter stops being a const and becomes a type.

The consequence for the spec, reasoned from that verified result: subtracting `BITS` from `LogicalWidth`
under the table escape needs **one impl row per (LogicalWidth, BITS) pair**, which is quadratic, whereas
under the type escape it is a projection. `h_widthtype.rs` builds the type version, derives the
significand by two chained projections, and compiles with **zero feature gates**:

```rust
impl<W: Nat, B: Nat, S: Signedness> Numeral for Binary<W, B, S>
where W: Sub<B>, <W as Sub<B>>::Out: Sub<S::Bits>
{ type Significand = <<W as Sub<B>>::Out as Sub<S::Bits>>::Out; }
```

This is the same move the repo already made for capacity, and the spec is half way through it: it chose
the type form for `LogicalWidth` and the const form for the two things `LogicalWidth` is subtracted from.

**And this is where the spec's D73 argument is wrong on its stated reason.** Spec:224-228 says integrality
must be a macro-expanded table "because 'fractional' means a negative exponent and an inequality in a
bound needs const-expression bounds, which are forbidden". arvo ships exactly such an inequality today, at
`arvo/src/ufixed.rs:100`:

```rust
Picker: OneRepresentable<{ tag_one_representable(I.raw()) }>,
```

with `tag_one_representable` at `arvo-strategy/src/identity.rs:69-75` compressing `I >= 1` into a
two-element tag space and `identity.rs:89-91` implementing the witness only at the true tag. So the
inequality is not the obstacle. The obstacle is the computed const argument, which is why that line needs
`generic_const_exprs` and why the gate at `arvo/src/lib.rs:25` is drift the workspace rule already names.
The spec's **conclusion** (use a table) is right; its **reason** is wrong; and getting the reason right
matters because at least four other derivations in the same document hit the same wall and the spec flags
none of them: the significand (spec:118), the representable range (spec:230-232), the dyadic membership
via `FullRange<F>`'s quantum (spec:107-109), and the container width, which the 0500 sketch already
identified and cured by making the total an associated type.

Two readings, and I hold them both. Either every axis whose value is subtracted, compared or otherwise
computed from becomes a type carrying its derived facts as members, which is uniform, linear, gate-free,
and matches the capacity precedent. Or the const form is kept for ergonomics at the public spelling and a
type form sits underneath it, with the aliases doing the conversion, which is what
`202607291400_.../04_escape_assoc_table.rs` sketches and costs one impl row per width. The choice is real.
What I would not do is leave the two forms mixed with a subtraction crossing between them, because that is
the one configuration with no expression under the permitted features.

## 9. Compile-time cost is a shape question before it is a bench question. Reasoned.

Spec:343-344 parks compile cost as a bench per `bench-and-sketch-discipline.md`, and it is right that
measuring belongs in `mock/benches/`. But the two encodings in section 8 differ **asymptotically**, not by
a constant: Pattern C tables grow as (values x derivations), typestate projections grow as
(values + derivations). Benching whichever shape happens to get written first measures a choice rather
than informing it. If a bench is going to be written, the bench worth writing compares the two encodings
on one derivation across the real width range, and it can be written before any of `arvo-numeral` exists.

## 10. A tagless-final reading of the whole thing. Reasoned, offered.

The spec is an initial-algebra encoding. The axes are constructors, given as types; the derived properties
are folds, written as blanket implementations; a composition is a term and its properties are the fold's
result. That is a coherent and well understood design and I do not think it is wrong. I do think it has
one failure mode and that this round has already met it twice.

**A fold written as `impl<A: Resolution>` is a default case.** Defaults are where the `SubstituteZero`
error lives (01's finding 1), where a downstream marker acquiring `AddAssoc` for free lives (section 4),
and where `Adjustment = Unit` as a proxy for "the quantum is a power of the radix" lives (spec:200 against
01's `FullRange<1>` case). All three have the same shape: a classification stated as a pattern over the
constructors that exist, applied to a set that is open.

The dual encoding makes the property the signature and the constructors the interpreters. Every property
is a member every constructor must supply, so a new constructor without an interpretation is `E0046`
(verified, `c2_totality.rs`) and a new property without a visit to every constructor is likewise a
compile error. One statement of the structure, many interpretations, and the compiler checking totality in
both directions. Its cost is that the constructor sets become closed to downstream extension, which for
this design is not a cost at all: the conventions ship as **alias sets** over the abstraction and nothing
else (spec:276-278), so a `conv-*` feature never wants to add a constructor. `arvo-strategy` already seals
`Strategy` (`lib.rs:94-95`, "Sealed: consumers cannot add new strategies") on the same reasoning.

The second reading, which the panel should weigh rather than take from me: the initial encoding's
diagnostics are better in the common case, because a missing impl reports the missing thing directly while
a computed truth value reports `False`. Section 4 shows that gap is closable with one marker and an
attribute, but "closable" is not "closed", and a design that needs a workaround to produce a good error is
worse than one that produces it naturally. Someone should write both `.stderr` fixtures before choosing.

## 11. `Deterministic` and `ConstantTime` are not the same kind of thing as `AddAssoc`. Reasoned.

The spec files four items under "what is derived rather than declared": number-system membership,
algebraic laws, the type-family markers, the range, `Deterministic` (spec:234-236) and `ConstantTime`
(spec:238-241). The last two do not belong with the others, and the spec very nearly says so itself: "an
internals change adding a data-dependent early exit would withdraw the property silently. That is the
accepted trade."

Read that against D16. A property that an internals change can withdraw is not computed from the thing, it
is **asserted about the current implementation**, which is precisely D16's second category and precisely
what `arvo-always-optimal-internals.md` guarantees will keep moving. `AddAssoc` is a theorem about a
numeral and a policy and no internals change can withdraw it. `ConstantTime` is a claim about a lowering's
emitted code. Putting them in one family invites a consumer to trust them equally.

Two readings. Either `ConstantTime` is honest as an `unsafe impl`-shaped promise per lowering, carrying a
contract that an internals change must re-discharge, which is the mechanism D16 describes for exactly this
case. Or it leaves the derived family and ships as a documented per-preset note, which is weaker and at
least does not misrepresent itself. `Deterministic` needs the same question asked separately, because
determinism across build targets depends on the container projection, which is a `Lowering` member, which
by section 5's argument a derived property is not supposed to touch.

## 12. Smaller verified items

- **`Resolution`, `Direction` and `Quantisation` are shown unsealed** (spec:137-138, 149). Under the
  blanket at spec:212 an unsealed `Resolution` means a downstream marker obtains `AddAssoc` on the
  unsigned domain without earning it. `arvo-strategy/src/identity.rs:89` seals `OneRepresentable` with the
  reason written out: "Sealed via `crate::sealed::Sealed`, so downstream cannot supply the impl this
  deliberately withholds." The same sentence applies here and should be in the spec.
- **The struct-level where-clause.** `pub struct Number<N: Numeral, S>(..) where S: Policy + Lowering;`
  (spec:61) puts the bound on the struct rather than on the parameter, which propagates it to every impl
  block and every mention. arvo's shipped types bound the parameter instead
  (`arvo/src/ufixed.rs:96`, `const impl<const I: IBits, const F: FBits, S: Strategy>`). Minor on its own,
  and it multiplies across a surface this size.
- **`#[diagnostic::on_unimplemented]` does not fire on `E0271`.** Verified by the pair
  `c_refusal.rs` (associated-type equality, no attribute output) against `c4_diag.rs` (marker bound,
  attribute fires). Whichever encoding wins, the law markers want the attribute, and the bound has to be
  shaped so it can fire. The model to copy is `arvo-strategy/src/identity.rs:81-84`, whose note names the
  cause, the consequence and the remedy in three sentences.
- **The `Adjustment = Unit` equality bound is a name check, not a classification** (spec:200-201). 01's
  `FullRange<1>` case is the proof that it is already wrong in one direction. The structural point is that
  it is also one new constructor away from being wrong in the other, and the repair is section 4's:
  `trait Adjustment { type IsRadixPower: TruthMarker; }` makes the classification total over the
  constructor set.
- **Sketch obligations 1 through 6 test that machinery compiles and refuses.** 01 already noted that a
  sketch compiling a false blanket impl passes. I would add that obligations 3 and 5 as written are
  satisfied by the shapes in sections 2 and 7 while the properties they exist to protect are absent, so
  both want restating in terms of the observable outcome: the refusal names the consumer's own type, and
  the generic operation can both return and refuse.

## 13. What I did not get to

Three things I would look at with more budget, listed so a later member can pick them up rather than
rediscover them.

Whether `Growth` belongs on `Policy` at all. `Exact` against `Narrowed<W, A>` reads as an identity axis in
disguise, since the intermediate's numeral is a numeral, and 01's finding 11 (when quantisation fires)
suggests the same seam from the other side.

Whether the ten axes want a single `ConstParamTy` record per contract rather than one associated type per
axis, given `adt_const_params` is allowed and the sketch's escape 04 shows standalone const arguments are
gate-free. That is a whole category the round has not touched, and it trades impl-table rows for
monomorphisation instances in a way nobody has priced.

The trait-solver cost of the projected bounds in section 2, which is a bench and which nothing has
measured. The sketch says so about its own shape; the spec's shape has strictly more normalization work
per obligation and inherits the disclaimer without inheriting the measurement.

---

**Summary for the next member.** The three-contract decomposition is claimed and not encoded: one type
parameter carries both `Policy` and `Lowering` and a law conditioned on storage layout compiles today
(section 5, verified). The law derivation's blanket-over-a-marker shape is what admitted 01's
`SubstituteZero` counterexample, hands the same law to unsealed downstream markers, and hits a coherence
wall with no permitted escape the moment a second true fact crosses the partition (sections 3, 4,
verified). `Direction: Resolution` asserts a containment false in both directions and a three-way
capability split refuses both bad cases by typing (section 6, verified). The fallibility GAT does not
compile as the obligation writes it, and once fixed it still leaves arvo unable to write one generic
arithmetic operation (section 7, verified). And the spec mixes const parameters with associated types
across a subtraction, which is the one configuration with no expression under this workspace's permitted
features, while giving the wrong reason for the one place it noticed the wall (section 8, verified against
`ufixed.rs:100` and the min-GCA sketch). None of this argues the spec's shape is wrong. It argues that the
sentence the spec is proudest of, that the mathematics is carried by the types, is at present carried by
the prose in five places, and that each of the five has a construction available that carries it in the
types instead.
