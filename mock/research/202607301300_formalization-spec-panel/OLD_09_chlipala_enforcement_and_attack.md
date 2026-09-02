# Panel 09: does enforcement exist, and the hole in the third repair

**Persona:** Adam Chlipala, proof-automation and correct-by-construction lens. Ninth member; read
`01_knuth_mathematical_rigour.md` through `08_fog_the_union_and_what_it_costs.md` in full, all three
op checkpoints (`04b`, `06b`, `08b`), and every probe under `02_probes/` through `08_probes/`, before
starting.
**Date:** 2026-07-30

**What I read in full:** the spec (`202607301200_topic.the-formalization-spec.md`), all eleven prior
panel files (eight numbered members plus three checkpoints), the panel brief, the governing panel rule
(`panels-argue-the-intent-not-the-wording.md`), and every probe committed under `02_probes/` through
`08_probes/` (forty files). Of those I compiled or ran, rather than only read: `02_probes/c_computed.rs`,
`05_probes/a_handler.rs`, `06_probes/c_nominal_and_modifier.rs`, `07_probes/a_witness_typestate.rs` and
`b_bounds_collapse.rs`, and `08_probes/a_union.rs` and `c_split_does_not_bind.rs`, since my two assigned
jobs both hang on what these specific artifacts actually do rather than what their panel files say they
do. **What I read in part:** the talk and the inherited-state file at the passages the prior eight cite;
D72's crate table (`202607301200_topic.the-formalization-spec.md:291-297`).

**Directory listing done.** `ls` across `mock/design_rounds/` (the three flat files at root are this
round, the newest closed round `202607300800/` predates it), `mock/research/` (nothing postdates the
panel directory), `mock/research/sketches/` (the two the spec cites plus
`202607291400_const-args-under-min-gca`, already pulled in by 02), and the panel directory itself,
including every `NN_probes/` subdirectory. Nothing supersedes the spec or the checkpoints.

**Gates.** I did not re-run the full suite; five prior members already did, independently, and nothing
in the working tree has moved since. I did not re-audit test bodies on the surface the first six members
already read in their own hands. My own compile/run work is the gate that matters for this dispatch: two
new probe families, described below and committed at `09_probes/`, each reproduced from a clean build.

**Brief-breaking.** The brief's own factual claims about the panel's history check out: `c_split_does_not_
bind.rs` really is written against `use crate::*;` (line 5), one crate, and never exercises a crate
boundary, module privacy, or the orphan rule. 07's witness mechanism really cannot express `Refuse` as
07 built it (`a_witness_typestate.rs:38`, `fn phi(x: i32, min: i32, max: i32) -> i32`, total). 08's
partial-map repair really does reproduce 01's whole table including both `Refuse` rows
(`08_probes/a_union.rs:200-214`). I found no false premise in the brief and proceed. I did, however, find
a false premise inside 08's own file, which is most of what follows: 08's stated result for its own
mechanism is not what its own compiled artifact does. That is not a brief defect; it is exactly the kind
of thing this panel's sequential-recompilation discipline exists to catch, and it survived three prior
readers of the same file (04b through 08b) because none of them ran the code end to end.

**Separation of evidence.** Sections marked *verified* were compiled and, where a runtime question is at
stake, run, under `nightly-2026-05-28`, from probes committed at `09_probes/`. Sections marked *reasoned*
are argument. I carry more than one reading wherever the evidence does not force one, and per the brief I
rule on neither job: job one's answer is an input to op's call on the split, not the call itself; job
two's answer is a finding to weigh before the repair is carried, not a verdict on whether it should be.

---

## Job one: does a mechanism exist that prevents a law from reading a `Lowering` member

### 0. What was never actually tested

Op's framing in `08b` is precise: "Sealing, module privacy, a crate boundary, coherence structure, a
witness on the law itself, a marker that only a `Policy` member can produce: none of these has been
probed, and the crate split D72 introduces may or may not help." I want to be equally precise about why.
`08_probes/c_split_does_not_bind.rs:5` opens with `use crate::*;`. Every name in that file, `Number`,
`Numeral`, `Policy`, `Lowering`, the illegal impl, lives in one compilation unit. There is no orphan rule
to test (orphan rules fire between crates), no module privacy to test (nothing is hidden from anything),
and no dependency-graph absence to test (nothing is absent; everything is `use`-able). The probe
correctly shows that **naming a lowering member in a law's where-clause is syntactically legal Rust**,
which was never in doubt. It does not show that no mechanism under arvo's constraints can prevent it,
because the mechanisms op named all operate at exactly the boundary the probe never built.

So this section builds that boundary for real: separate crates, linked with `--extern`, compiled and run.
Full reproduction steps and every raw compiler output are in `09_probes/README.md` and
`09_probes/crate-boundary/`; I cite the load-bearing ones inline.

### 1. The fact can be made structurally L-blind. Verified.

Four crates, matching D72's shape: `numeral` (a bare `Numeral` trait plus two constructors),
`policy` (a `Policy` trait with `type OverRange: Resolution`, plus `ReduceModulo` / `SubstituteZero` /
`Refuse`), `lowering` (a `Lowering` trait with `type Layout: StorageLayout`), and `algebra`, compiled
**without** `--extern lowering=...` at all (`09_probes/crate-boundary/algebra.rs`). `algebra` declares
the law trait `AddAssoc` and a carrier `Fact<N, P>` for the fact it proves, and the one blanket impl:

```rust
impl<N: Numeral, P: Policy> AddAssoc for Fact<N, P>
where
    P::OverRange: StableUnderTranslation,
    <P::OverRange as StableUnderTranslation>::Out: IsTrue,
{
}
```

This builds clean. `09_probes/crate-boundary/a_leak_attempt.rs` is the same crate with one line added,
`use lowering::Lowering;`, compiled the same way, with the same `--extern` set (no `lowering`):

```
error[E0432]: unresolved import `lowering`
  --> a_leak_attempt.rs:12:5
   |
12 | use lowering::Lowering;
   |     ^^^^^^^^ use of unresolved module or unlinked crate `lowering`
```

This is real, and it is stronger than a review-discipline convention. It is not that the `algebra`
crate's maintainer chose not to reference `Lowering`; it is that the symbol has no referent in that
compilation unit, checked by the compiler at the moment anyone tries. Under D72's literal crate table,
`arvo-algebra-contracts` has no `Cargo.toml` dependency edge to `arvo-lowering`, and this is what that
absence buys: the fact computation cannot mention a lowering member, full stop, and the error fires at
authoring time, in the crate that would be doing the wrong thing, naming the exact undeclared symbol.

### 2. The already-existing orphan rule, not this round's redesign, is what stops a foreign actor. Verified.

`09_probes/crate-boundary/numeric_honest.rs` is a fourth crate, `numeric`, depending on all three plus
`algebra`. It owns the physically real composition:

```rust
pub struct Number<N: Numeral, P: Policy, L: Lowering>(core::marker::PhantomData<(N, P, L)>);

impl<N: Numeral, P: Policy, L: Lowering> AddAssoc for Number<N, P, L>
where
    algebra::Fact<N, P>: AddAssoc
{
}
```

`Number` needs `L: Lowering` in scope for its own struct definition, because `Lowering` is what would
determine its real byte layout (`StoredWidth`, `Layout`) in a non-toy version; this is not an accident of
my probe, it is D54's own test applied to why the axis exists at all. `09_probes/crate-boundary/
downstream_hostile.rs` is a fifth crate, depending on all of the above plus `numeric`, attempting a
second impl of `AddAssoc` for `Number<N, P, L>` conditioned on `L::Layout: StorageLayout`:

```
error[E0117]: only traits defined in the current crate can be implemented for types
              defined outside of the crate
```

This is real, and it is the standard orphan rule, unconditional on anything in this round: neither
`AddAssoc` (owned by `algebra`) nor `Number` (owned by `numeric`) is local to `downstream_hostile`, so
Rust refuses the impl regardless of whether D72 ships. This was already true before the spec existed. It
answers a threat model nobody had actually posed (a foreign, unrelated crate injecting a law impl), and
it is worth saying plainly that this part of "is enforcement possible" was never the hard part.

### 3. The hard part: the crate that legitimately owns `Number` can still condition the law on `L`, and the split does nothing to stop it. Verified, and this is where 08's finding survives, sharpened rather than refuted.

`09_probes/crate-boundary/numeric_dishonest.rs` is the same `numeric` crate, with the one law impl
altered:

```rust
impl<N: Numeral, P: Policy, L: Lowering> AddAssoc for Number<N, P, L>
where
    algebra::Fact<N, P>: AddAssoc,
    L::Layout: IsDense,
{
}
```

This builds clean, exactly as `c_split_does_not_bind.rs` found, but now at the one location a real D72
would actually put it: the crate that owns `Number`'s physical definition, four genuinely separate
crates away from `arvo-lowering`'s own declaration, with the fact itself proven in a crate that
structurally cannot see `Lowering` at all. `09_probes/crate-boundary/c_dishonest_refusal_check.rs` makes
the consequence concrete rather than vacuous: `Number<Fix13_3Signed, Warm, MinWidth>` (Dense) folds;
`Number<Fix13_3Signed, Warm, DoubleWidth>` (Bitpacked), same `N`, same `P`, same algebra-proved fact,
refuses:

```
error[E0277]: the trait bound `Bitpacked: IsDense` is not satisfied
```

Two numerals equal in every identity and policy respect, differing only in a lowering member, now
disagree on whether their addition is associative. That is precisely the thing D54 and spec:66-68 say
must not happen ("conditioning a law on one would be conditioning correctness on a storage choice"), and
the crate split, taken exactly as D72 states it, does not prevent it. The reason is structural, not a
matter of anyone being careless: `Number`'s own definition requires `Lowering` in scope, so any crate
capable of writing `impl AddAssoc for Number<N, P, L>` at all is, by construction, a crate where `L` has
methods a where-clause can name. The crate split closes the door on the fact's own computation (section
1) and on a foreign actor (section 2); it does not and cannot close the door on the one impl that
legitimately has to exist in a `Lowering`-aware crate, because that impl's own type target demands it.

### 4. A macro-export attempt, and why it fails for a reason worth recording. Verified.

Before concluding the first-party gap is unclosable, I tried the most obvious candidate: have `algebra`
export a `macro_rules!` that generates the entire forwarding impl, so `numeric` never hand-writes the
impl header and therefore has nowhere to splice an extra bound. `09_probes/crate-boundary/algebra_macro.
rs` / `numeric_macro.rs`:

```
error[E0277]: the trait bound `L: Lowering` is not satisfied
   |
19 | algebra_macro::derive_add_assoc!(Number<N, P, L>);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Lowering` is not implemented for `L`
   |
note: required by a bound in `Number`
```

Rust does not imply a struct's own where-clause into every impl targeting it; each impl restates the
bound. So the macro's expansion needs `L: Lowering` written somewhere, and the only place that bound can
be *named* is a scope where `Lowering` resolves, which is exactly the scope the macro's premise tries to
avoid. This is not a failure of my macro's design; it is a general fact about Rust's lack of implied
bounds, and it means **no macro exported from a Lowering-blind crate can generate a valid impl header
for the physically real `Number<N, P, L>` at all**, whether that impl is honest or not. The gap in
section 3 is not "nothing stops an extra clause being added"; it is "the forwarding site, wherever it
lives, is unavoidably a place where `Lowering` must be nameable," which is a stronger and more useful
statement of the limit, and closes off an entire family of attempted fixes before anyone spends more time
on it.

### 5. A shape that closes the gap completely, verified, at a real cost worth naming plainly.

The macro attempt's failure points at the actual lever: the reason the forwarding impl needs `Lowering`
in scope is that `Number`'s own type requires it. So make the type the law targets **not** require it.
`09_probes/crate-boundary/algebra_logical.rs`:

```rust
/// L is a FREE type parameter here: no bound, because nothing in this crate
/// can name a bound for it.
pub struct LogicalNumber<N, P, L>(core::marker::PhantomData<(N, P, L)>);

impl<N: Numeral, P: Policy, L> AddAssoc for LogicalNumber<N, P, L>
where
    P::OverRange: StableUnderTranslation,
    <P::OverRange as StableUnderTranslation>::Out: IsTrue,
{
}
```

Builds clean, entirely inside the Lowering-blind crate, with no forwarding step anywhere. `09_probes/
crate-boundary/numeric_via_logical.rs`, from a crate that genuinely has `Lowering` in scope, instantiates
`L` with both a Dense and a Bitpacked lowering, and both fold, by construction, since `L` carries no
bound the impl's where-clause could ever have conditioned on. `09_probes/crate-boundary/numeric_via_
logical_hostile.rs` attempts the same second-impl attack as section 3, now against `LogicalNumber`, from
a crate with `Lowering` fully in scope:

```
error[E0117]: only traits defined in the current crate can be implemented for types
              defined outside of the crate
```

This is airtight, and it is airtight for a reason independent of the crate split: **the mechanism that
actually does the work is that `L` is unconstrained in the type the law targets, not that `Policy` and
`Lowering` live in different crates.** The crate split (section 1) is real, valuable, and complementary:
it turns an accidental `Lowering` reference inside the fact's own derivation into a compile error at
authoring time rather than a thing a reviewer has to notice. But it is not what makes section 5's closure
hold; a single-crate version of `LogicalNumber` with unconstrained `L` would be exactly as airtight,
because `AddAssoc` and `LogicalNumber` being *both local to the deriving crate* is what orphan-rules the
world out, crate split or no.

**The honest cost, stated rather than priced.** This is an architectural change past D72's literal shape,
not a drop-in fix to it. The type a law is proven about (`LogicalNumber<N, P, L>`, phantom, no physical
storage) is now distinct from the type that genuinely holds bytes and depends on `Lowering` for its
layout. Consumer-facing `Number` would need to be, at minimum, the *same* type as `LogicalNumber` for the
law to attach with zero forwarding step (a type alias, not a wrapper: `impl Trait for a foreign type
alias` does not typecheck as a separate impl target, so aliasing is what keeps the orphan rule biting),
which is compatible with `06`'s already-verified finding that nominal, alias-free constructors render
cleanly in diagnostics, but is a different claim from "`Number` is a `repr(transparent)` newtype over its
composition" (04 section 7, 06 section 3). I did not wire this end to end against `07`'s graded `Arith`
trait, which correctly and legitimately *does* need `Lowering` in scope (that is what `Lowering` is for),
and connecting "the type the law is proven about" to "the type the arithmetic actually runs on" without
reopening a forwarding-impl gap somewhere in that connection is exactly the kind of question job two
below turns out to bear on directly. I flag this as the honest boundary of what I verified: the closure
mechanism is real and mechanically checked; wiring the physically real numeric primitive through it,
so `Number<13, 3, Warm>` both carries real bytes and is provably the same type the law was proven about,
is a genuine design exercise this dispatch did not complete.

### 6. Summary of what is mechanically true, for op's call

Sealing and coherence, taken alone, answer a question that was never actually open (section 2). A crate
boundary matching D72 literally, taken alone, makes the fact's derivation provably independent of
`Lowering` (section 1) but does not and structurally cannot stop the one forwarding impl that legitimately
needs `Lowering` in scope from also conditioning on it (section 3), and no macro can route around that,
for a reason that generalises past this one macro (section 4). A design where the law-bearing type itself
carries no `Lowering` bound closes the gap completely, independent of whether the crate split exists at
all, at the cost of separating the law's carrier from the physical storage type in a way this dispatch
verified in isolation but did not wire through the rest of the design (section 5). Whether that cost is
worth paying, whether D72's crate split is worth keeping given it delivers less than 02 claimed but more
than nothing, and whether the wiring in section 5 is where the round should spend its next budget: op's
calls, not mine.

---

## Job two: attacking the third shape of thread C before it is carried

### 0. What I inherited, and what I did that the chain has not done yet

03 proposed bounded const falsification, uncompiled. 07 built it, made the recovery map `phi` total, and
claimed the strongest possible form of the result: "the oracle is not a second place to be wrong, because
it is the same phi the runtime arithmetic calls" (07 file, section 1.1). 08 found `phi` as 07 built it
cannot express `Refuse` at all, since refusing is the absence of a value and a total map has no absent
case, and repaired it by making `phi` partial (`Rec::At(i32) | Rec::Refused`), which mechanically
reproduces 01's whole table, both `Refuse` rows included. Every member from 04b onward, including 08
itself, treats 07's central claim, that the checked `phi` **is** the code the runtime calls, as
established. Nobody compiled the union of the witness machinery and the actual arithmetic machinery into
one program and ran it until I did.

08 built exactly that union, `08_probes/a_union.rs`, for a different purpose (whether five standing
proposals compose). I ran the two halves of 08's own file against each other.

### 1. The witnessed `phi` and the runtime arithmetic are two independently authored specifications that never touch. Verified by compiling and running.

`a_union.rs:139-145` declares `ReduceModulo`'s witnessed recovery map:

```rust
const impl Resolve for ReduceModulo {
    fn phi(x: i32, min: i32, max: i32) -> Rec {
        let span = max - min + 1;
        Rec::At((x - min).rem_euclid(span) + min)
    }
}
```

Genuine wraparound. This is the function 07's claim says the runtime calls. Now the runtime.
`a_union.rs:692-698`:

```rust
pub fn add<C: Arith>(a: u16, b: u16, min: u16, max: u16) -> C::Answer<u16> {
    match a.checked_add(b) {
        Some(v) if v <= max && v >= min => C::ok(v),
        Some(_) => C::over(max),
        None => C::over(max),
    }
}
```

Every out-of-range branch calls `C::over(max)`, the literal constant `max`, unconditional on which
`Resolution` is actually configured. `C::over` (`a_union.rs:667-683`) runs a `const` block checking that
the *declared markers* agree with `phi`, then calls `Deliver::refuse(nearest)`. `Deliver<False>::refuse`
for `AsSum` (`a_union.rs:276-281`), the delivery every preset in the union uses, is:

```rust
impl Deliver<False> for AsSum {
    type C<T: Copy> = Total<T>;
    fn refuse<T: Copy>(nearest: T) -> Total<T> {
        Total(nearest)
    }
}
```

It wraps whatever it was handed. It never calls `Resolve::phi`. Nothing in the whole chain,
`add -> over -> Deliver::refuse`, ever calls `phi` for its *value*; the only call to `phi` anywhere in
the union's runtime path is inside the door's `const` block, checking a boolean classification against
it, not computing an answer from it.

`09_probes/d_delivery_disconnected_from_phi.rs` is `a_union.rs` verbatim, with a `main` appended that
calls both halves side by side, under `Hot` (declared `OverRange = ReduceModulo`, the wrap preset,
witnessed and passing every check the union performs):

```
ReduceModulo::phi(9, min=0, max=7) = 1  (wrap-around answer)
add() under Hot/ReduceModulo returned Total(7)
phi says wrap gives 1; the runtime delivery ignores that and returns the caller's
hardcoded `max` regardless of which resolution is configured.
```

`Hot`'s whole documented identity is that it wraps. Under the union, it clamps, silently, in code that
passed the witness, passed totality, passed coherence, and that 08's own file cites as demonstrating the
mechanism working. `TowardNegative`, `TowardPositive`, `ReduceModulo` and `SubstituteZero` all deliver
the identical value, `max`, on overflow, in this union, despite `phi` computing four different things for
them. The witness never looks, because nothing asked it to.

### 2. This is not new to 08; 08 is the first place it could have been seen, and it was not. Verified.

`07_probes/a_witness_typestate.rs` contains `phi` and the witness. `07_probes/b_bounds_collapse.rs`
contains the graded `Arith` aggregate and `add`. They are two separate files, never joined; 07's own
"what I did not get to" names this gap sideways ("whether the graded frame wants the join computed per
operation... to also carry 05's per-operation lemma indexing... I ran out of budget before testing it")
without ever stating that the two halves had not been checked against each other at all. 08 built the
first artifact where this was even checkable, `a_union.rs`, joining witness and arithmetic in one crate
for the first time in the panel's history. 08's own file spends two full sections (5 and 6) reading the
assembly this exact code emits, instruction by instruction, comparing byte sizes and branch counts across
delivery shapes, and never once compares the *values* the deliveries produce against what `phi` says they
should be. The measurement lens measured everything except correctness. That is not a criticism of
measurement as a discipline; it is the specific, avoidable gap this dispatch exists to name.

### 3. Answering the brief's specific questions

**Does this make an illegal state unrepresentable, or relocate the assertion?** Relocates it, and
relocates it somewhere strictly less visible than where it started. The witness makes exactly one
illegal state unrepresentable: a declared classification marker disagreeing with what `phi` computes for
that marker's shape. That is real (07's `a2`/`a4`/`a7`, re-verified by me at the union's real trait shape
in `09_probes/e_totality_still_holds.rs`, section 4 below). It says nothing about the state a consumer
actually cares about, which is whether the arithmetic that runs disagrees with the resolution they typed.
That state remains fully representable, is instantiated in the union's own reference build, and nothing
refuses it, because nothing was ever asked to.

**Can a lying implementor still be written?** Trivially, and not even maliciously: `AsSum`'s existing,
already-shipped `Deliver<False>::refuse` implementation, unmodified, is already the lying implementor, for
every resolution other than clamp-toward-max. No new code is required to reproduce this; I only had to
call the two halves of `a_union.rs` against each other and print the result.

**What happens when someone adds a resolution nobody anticipated?** This part is unaffected by the
finding above, and I re-verified it independently rather than trusting 07 and 08's own account of it.
`09_probes/e_totality_still_holds.rs` appends a `StochasticRound` constructor implementing `Resolve` but
missing all three `Resolution` associated types, at the union's real three-member trait shape (not the
smaller shape 07's own totality probe used):

```
error[E0046]: not all trait items implemented, missing:
              `StableOneSided`, `StableTwoSided`, `Refuses`
```

Totality holds, genuinely, and is a real, working, unaffected part of this design. The finding above is
not "the witness mechanism is broken"; it is "the witness mechanism verifies a narrower claim than 07 and
08 both state it verifies," and totality is exactly the part of that narrower claim that is sound.

**The boundary between what the const evaluator refuses and what merely fails to resolve.** The
evaluator refuses precisely the disagreement it was told to check: declared markers against `phi`'s
classification, at the door and at the eager per-constructor const. It has no opinion at all about `phi`
against `Deliver::refuse`'s returned value, and "fails to resolve" does not even apply to that comparison,
because no trait bound stating it was ever written. There is no unsatisfied obligation sitting latent in
the union waiting to fire; there is an obligation that the design never states, so the compiler's silence
here is not evidence of soundness, it is evidence that nobody asked the question in the type system's
hearing. This is D16 sharpened past where 03 and 07 left it: 03 showed totality and coherence are not a
truth check on a leaf; this shows that even a leaf the mechanism genuinely does check for truth (`phi`'s
own classification) sits arbitrarily far, with no linkage at all, from the code a consumer's `a + b`
actually executes.

**Does the mechanism survive the width ceiling 08 measured?** The ceiling (0.53s at 3 bits, 28.45s at 8,
refused past 8, `08_probes/g_classification_table.rs` and section 11 of `08_fog...md`) applies to `stable`
and `ever_refuses`, exactly the two functions this finding shows are disconnected from delivery. Paying
that cost, at any width, at any representative sample, buys a more thoroughly checked *classification*.
It buys nothing toward the property that actually matters here, because delivery does not consult the
classification's underlying function regardless of how exhaustively that function was checked. The
ceiling question and this finding are orthogonal, and answering the ceiling question more precisely would
not have surfaced this: the two are dual failures (03/08 measured *how far* the check reaches; this
measures *what* the check is actually wired to), and neither subsumes the other.

### 4. What a real repair would need, offered and not built

The only shape that closes this is making `phi`, or its genuine successor, the literal function the
runtime calls, generically over the real payload representation rather than a fixed small `i32` model:
one definition, checked at a small width for the const-eval proof and invoked at the real width for the
real arithmetic. This is a substantial engineering lift, not a naming fix: arvo's real values are
`Bits<N, S>`-shaped, not `i32`, so the recovery map's signature would need to be generic over whatever
trait family the storage primitives already expose, and every one of `phi`'s five constructor bodies
would need re-authoring against that generic surface rather than against raw signed-integer arithmetic.
I did not attempt this and say so plainly; it is a fair-sized sketch on its own, before it is worth
touching `arvo-policy`.

It is also worth noting where this repair would land relative to job one. A `phi` genuinely shared
between the const-eval witness and the real, running arithmetic is a function whose real invocation site
is wherever `Number`'s actual arithmetic executes, which per job one section 3 is unavoidably a crate
where `Lowering` is in scope (the real storage width and layout are `Lowering` facts). So the repair this
section wants and the closure job one section 5 built are not independent questions: if the panel pursues
both, the shared `phi` would need to be provably `Lowering`-blind in exactly job one's sense, or the two
gaps recombine into one. I flag the connection rather than resolve it; nobody has built the shared `phi`
to check whether it can be kept `Lowering`-blind while also being generic enough to run for real.

### 5. What I verified and found solid, so the panel does not re-litigate it

The totality mechanism, at the union's real scale, genuinely refuses an incomplete `Resolution`
constructor (section 3 above, re-verified independently rather than trusted). The partial-`phi` repair
genuinely reproduces 01's whole table, both `Refuse` rows, mechanically (`a_union.rs`'s own compiled
classification table, unmodified by anything in this file). The door check genuinely cannot be disarmed
by an overriding implementor (07's `a6`/`a7`, which this file did not need to re-attack because the gap
found here is orthogonal to it: an honest, un-disarmed door still only checks the classification, not
the delivery). None of this argues 07 and 08's mechanism should be discarded; both halves of it, the
witness and the graded aggregate, are real, working pieces. The finding is that they were never actually
one mechanism, only presented as one, and the gap between them is exactly where a resolution's whole
documented behaviour can silently become a different resolution's behaviour without a single diagnostic
firing anywhere in the design.

---

## The trusted computing base, named for both jobs, because leaving it implicit is the failure this file exists to correct

Job one's closed mechanism trusts: that `algebra_logical`'s single blanket impl over `LogicalNumber<N, P,
L>` is the *only* impl of `AddAssoc` ever written for it (true by construction, since the type and trait
are both local to that one crate and no second impl can be added anywhere, verified in section 5), and
that the fact `StableUnderTranslation` computes for each `Resolution` constructor is itself correct
(unverified here; this is exactly 01's and 03's leaf-truth question, orthogonal to the enforcement
question this file answers).

Job two's finding narrows what was trusted, rather than removing trust: before this file, the union's
implicit trusted base was "`phi`, once witnessed, governs the composition's behaviour." After this file,
the honest trusted base is "`phi`, once witnessed, governs nothing at runtime; the actual trusted base is
every `Deliver::refuse` implementation and every `add`-shaped arithmetic body, none of which is checked
against anything." That is a much larger, much less examined trusted base than 07's and 08's own framing
implied, and it is the thing op's `08b` asked the next lens to find before it was carried further.

I rule on neither job. Section 6 above states what job one leaves for op's call; job two leaves the
choice of whether to carry 08's repair forward as-is (with this gap named and unfixed, which is a real
and legitimate option for a design that is still one day old), to fix it via section 4's sketch before
carrying it, or to treat the gap as evidence that thread C wants a fourth shape entirely.
