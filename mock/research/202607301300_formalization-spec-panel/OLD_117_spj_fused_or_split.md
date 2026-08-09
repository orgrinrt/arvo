# Panel 117: fused or split, re-derived from the intent

**Persona:** Simon Peyton Jones, type-system-and-implementation lens. Dispatched on the one question op
reserved to himself at the third checkpoint and has never been asked again.
**Date:** 2026-08-05

**What I read in full:** `110_consolidation_eleven.md` (the standing base), `08b_op_checkpoint_three.md`,
`09_chlipala_enforcement_and_attack.md`, `26_consolidation_two.md` section 1.1, and the sections of `08`
that bear on the split (`08:142-206`). **What I read in part:** `110` at its trait table (2886-2960), its
crate-table and enforcement section (3085-3190), its open lists (4466-4480, 4780-4800), and its coupling
note on diagnostics (4850-4875); `108b:9-22` for the re-derivation mandate.

**Directory listing done.** `ls` across the panel directory including every `NN_probes/`, and across
`mock/research/` and `mock/design_rounds/`. Nothing postdates `114_the_restoration_pass.md`; nothing in
either tree supersedes `110` or the checkpoints. `git log` confirms `114` is the tip.

**Gates.** I did not re-run the crate suite and I say so plainly. This dispatch is a question about a
design whose implementation is being replaced, the brief forbids auditing that source, and five prior
members ran the suite independently with nothing moved since (`09:25-28`). The gate that matters here is
my own: eleven probes across three crate topologies, each compiled from clean under
`nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`), every compiler output below
reproduced verbatim. The dispatch forbade me from adding files, so the probe sources are inline rather
than committed; each is short enough to retype and I give the exact `rustc` invocation shape for each
topology.

**Separation of evidence.** Sections marked *compiled* were built and the quoted diagnostics are real
output. Sections marked *reasoned* are argument. I rule on nothing. Op reserved this call in those words
and my job is to give him the thing he needs to rule, which is a set of compiled facts and one
recommendation he can reject cheaply.

---

## 0. Breaking the brief first, because two of its claims do not hold

The brief says the standing base ships a three-way split at `110:2467-2483`. **That citation points at
section 1.21, the strategy door and the two ratified preset tables.** There is no trait split in it. The
split is declared at `110:2898-2916`, seventeen lines of trait table in section 1.23, plus a second
`Numeral` declaration at `110:788-793`. This is a line-number slip rather than a substantive error, and
I record it only because a citation that resolves to the wrong section is the shape of drift this panel
has been catching all week.

The second one is substantive and it changes the question.

**The brief describes one split. There are two, they were decided separately, and only one of them is
open.**

The first is the **parameter count** on the numeric type. `02` proposed `Number<N, P, L>`, three
parameters. `08` measured it against the fused `Number<N, S>` and found it costs roughly 1.8x in
rendered diagnostic length while moving the truncation boundary down by one modifier (`08:142-172`), and
the standing base carries the fused two-parameter form as settled, with `Encoding` nested inside
`Lowering` **specifically so the three-parameter cost is not paid a second time** (`110:877-880`). So
that one is closed, on measurement, against the split.

The second is the **trait count**, `Numeral` / `Policy` / `Lowering` as three declarations rather than
one. This is what op reserved at `08b:47-51`, in the sentence that names "the three-contract split" and
the counterexample about "separating `Policy` from `Lowering`" (`08b:16-20`). This is open.

The two are independent, and the standing base does not connect them anywhere. What connects them is
the type's second parameter: `S` is one parameter that implements **both** `Policy` and `Lowering`. That
bound is stated at `26:28-35` ("`Number<N: Numeral, S>` where `S: Policy + Lowering`"), which is agent
output, and **`110` never states it at all.** I grepped four ways: `Policy + Lowering`, `S: Policy`,
`S: Strategy`, `Strategy>`. Zero hits in 5900 lines. The standing base declares three traits and a
two-parameter type and never says how the second parameter relates to two of the three.

That gap is worth op's attention independently of how he rules, because it is the exact sentence that
makes the whole shape legible, and section 5 below shows it is also the sentence that decides whether
the design's central enforcement property holds.

So the question I actually answer is: **should `Policy` and `Lowering` be two traits or one, given that
one type parameter implements both?**

And one shape is off the table before I start, on ratified ground. Fusing `Numeral` into the strategy
contract would contradict D54's sorting test (`talk:352-356`, carried at `110:2941-2947`) and op's own
statement of the principle behind it, "what the number *is* does not change through strategies"
(`talk:334-336`, at `110:2948-2950`). A fully fused single-trait format descriptor makes `Warm` and
`Hot` different numbers rather than the same number computed differently, which is the one thing that
statement forbids. I checked whether the grounds have moved, per `108b:11-20`, and they have not: nothing
in the hundred files since weakens it. So `Numeral` stays separate and the live question is two traits
against one.

---

## 1. What the enforcement result actually settles, which is not what it was supposed to settle

Op made this call conditional: "those are op's, and they are **downstream of whether enforcement is
possible**" (`08b:49-51`). File 09 answered the enforcement question, and the answer discharges the
condition in a way nobody has read back against the call.

File 09 established four separate things, and they pull in different directions:

The fact's own derivation **can** be made structurally blind to `Lowering`, by a crate with no dependency
edge to it. Verified, `E0432` at the point of use (`09:68-101`). This is real, it fires at authoring
time, in the crate doing the wrong thing, naming the undeclared symbol.

A foreign actor injecting a law impl was **never** open. The orphan rule already handles it,
unconditionally on this round (`09:103-133`). This part was not the hard part.

The crate that legitimately owns `Number` **can** still condition a law on `L`, and the split does not
stop it (`09:135-169`). `Number<Fix13_3Signed, Warm, MinWidth>` folds and
`Number<Fix13_3Signed, Warm, DoubleWidth>` refuses, same identity, same policy. No macro routes around
it, because Rust has no implied bounds and the forwarding site is unavoidably somewhere `Lowering` must
be nameable (`09:171-196`).

A shape that closes it completely exists, and here is the sentence that matters most for op's call
(`09:229-231`):

> **the mechanism that actually does the work is that `L` is unconstrained in the type the law targets,
> not that `Policy` and `Lowering` live in different crates.**

Read that against the gate op set, and the conclusion is this. **The enforcement answer does not decide
the fused-versus-split call, because the mechanism that enforces is not the split.** The condition op
attached has been discharged, and it discharged to "the condition is independent of the question". That
is a real result and it is the first thing op needs, because it means the call cannot be made by reading
09's verdict off the page. It has to be made on other grounds.

It also kills the ground the split was originally adopted on. `02` called the separation "a typing fact
rather than a review note" and `04` and `05` endorsed it on that basis (`08b:18-20`). `08` compiled the
refutation and `09` sharpened it. That ground is gone and does not come back.

So the split now has to earn its keep from scratch, or not. Which is what the next three sections do.

---

## 2. The fused shape, argued properly

One trait, `Strategy`, carrying the policy member and the four lowering members. `Number<N, S>` with
`S: Strategy`. Nothing else in the design moves.

**What it buys, and these are not nothing.**

It restores a name. The whole workspace says `Strategy`: arvo's own agent instructions state "every
numeric type carries `S: Strategy`", the lint rules repeat it per crate, the shipped facade spells it
that way, and `arvo-strategy` is a crate that exists. Under the split there is **no collective noun for
what `S` is**, and the bound has to be spelled `S: Policy + Lowering` at every generic position that
reads both halves. A design with no name for its own second parameter is a design that will acquire one
by accident, and section 6 shows what happens when it does.

It halves the impl burden per preset. Four presets today, one impl block each instead of two. Small, but
real, and it grows with every preset a consumer adds, which the toolbox rule explicitly invites.

It is the honest shape if the two halves genuinely always travel together. Three contracts is a claim
that three things vary independently. `Numeral` clearly varies independently of the other two: that is
D54 and it is ratified. But `Policy` and `Lowering` are, in the shipped design, **always chosen as a
pair**, by one preset marker, at one call site. Nobody writes `Number<Fix13_3, WarmPolicy, ColdLayout>`,
because the second parameter is one type. The independence the split claims is not exercised anywhere in
the design's own instantiations.

That last point deserves to land properly, because it is the strongest thing the fused side has. **On
the design's actual axes, `Policy` and `Lowering` do not vary independently.** They are two halves of one
four-valued choice. A reader coming to this cold would ask why one choice needs two contracts, and
"because a sorting test puts the axes in different columns" is a reason about the taxonomy, not about the
type system.

**What it costs.** One thing, and it is decisive, and nobody has compiled it until now.

---

## 3. Under the fused trait, the crate boundary has nowhere to cut. Compiled.

The law-bearing crate has to read the policy half. Under the fused shape, reading the policy half means
bounding on `Strategy`, and bounding on `Strategy` hands over the lowering half in the same breath,
because a trait bound projects every member of the trait.

Topology C, three crates, `rustc --edition 2021 --crate-type=lib -L .` with explicit `--extern` per
edge:

```rust
// crate `fused`
pub trait Strategy {
    type OverRange: Resolution;   // policy half
    type Layout: StorageLayout;   // lowering half
    type Bytes: Copy;
}

// crate `numeric`
pub struct Number<N, S: Strategy>(pub S::Bytes, pub PhantomData<N>);

// crate `c1_law`, which owns the law trait
pub trait AddAssoc {}
impl<N, S: Strategy> AddAssoc for Number<N, S>
where S::OverRange: Stable, S::Layout: IsDense {}
```

`libc1_law.rlib` builds clean. There is no `--extern` set that prevents it, because the crate cannot do
its own job without `fused` linked, and `fused` carries both halves.

Compare topology A, the split, same shapes, `Policy` and `Lowering` in separate crates, the law crate
compiled with `--extern policy` and `--extern carrier` and **no `--extern lowering`**:

```rust
// crate `algebra`
impl<N: Numeral, S: Policy + Carrier> AddAssoc for Number<N, S>
where S::OverRange: StableUnderTranslation {}
```

`libalgebra.rlib` builds clean. Adding the single line `use lowering::IsDense;` to that same file, same
invocation:

```
error[E0432]: unresolved import `lowering`
 --> a2_leak.rs:6:5
  |
6 | use lowering::IsDense;            // <-- the only added line
  |     ^^^^^^^^ use of unresolved module or unlinked crate `lowering`
```

And a hostile crate that **does** have `lowering` fully linked, attempting the second, lowering-conditioned
impl:

```
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
  --> a3_hostile.rs:11:1
   |
11 |   impl<N: Numeral, S: Policy + Carrier + Lowering> AddAssoc for Number<N, S>
   |   ^                                                             ------------ `Number` is not defined in the current crate
```

So the split's real value, stated in the form the panel can check rather than the form `02` claimed:

> **Separate traits do not prevent a law from being conditioned on a cost axis. What they permit is a
> crate that reads the policy half without linking the lowering half, and a crate that cannot link a
> thing cannot name it. The enforcement is the crate edge. The trait split is what makes a crate edge
> available in a useful place.**

That is a typing consequence of the split, it is compiled, and it is not the one `08` refuted. `08` was
right that the split alone binds nothing, and right to say so. The correction is that the split was never
the mechanism; it is the **precondition** for the mechanism, and under fusion the precondition is
unavailable.

Note what this also does to `09`. `09:229-236` says the crate split is not what makes its closure hold,
and that a single-crate `LogicalNumber` would be exactly as airtight. True, for `09`'s own topology,
where `L` is a free phantom parameter with no bound at all. Under the design's actual fused
two-parameter form there is no free parameter to leave unconstrained, because the law needs `Policy` out
of `S` and `S` is one type. So `09`'s closure does not transpose to the shipped shape directly, and the
crate edge is doing more work here than `09`'s own summary credits it with. This is not a correction to
`09`, which was working a three-parameter topology throughout; it is what changes when its result is
read against the form that actually ships.

---

## 4. The split, argued properly, including what it still does not close. Compiled.

The split is not free of `09`'s gap. It relocates it, and the relocation is a narrowing rather than a
closure. Here is the honest account, compiled.

`Number`'s own struct definition needs the stored bytes, and the stored width is a lowering axis. Rust
has no implied bounds (`09:187-190`), so **whatever bound sits on the struct must be restated by every
impl targeting it, and therefore must be nameable by the law crate.** In topology A I put the smallest
possible thing there, a one-member `Carrier` trait in its own crate:

```rust
pub trait Carrier { type Bytes: Copy; }
pub struct Number<N, S: Carrier>(pub S::Bytes, pub PhantomData<N>);
```

The law crate can now name `Carrier`, and a dishonest impl can discriminate through it without ever
naming `Lowering`:

```rust
// crate `a5_discriminate`, compiled with NO --extern lowering. Builds clean.
impl<N: Numeral, S: Policy + Carrier<Bytes = u64>> AddAssoc for Number<N, S>
where S::OverRange: StableUnderTranslation {}
```

And it bites, exactly as `09:152-159` found, with two presets carrying identical `Policy` and differing
only on the lowering side:

```
error[E0271]: type mismatch resolving `<WarmPacked as Carrier>::Bytes == u64`
 --> a6_bite.rs:9:41
  |
9 | pub fn packed_folds() { requires_fold::<Number<Fix13_3, WarmPacked>>(); }
  |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u64`, found `u32`
  |
  = note: required for `Number<Fix13_3, WarmPacked>` to implement `AddAssoc`
```

Two numerals equal in every identity and policy respect disagreeing on whether addition is associative,
with `Lowering` nowhere in scope. **This is the wiring `09` explicitly did not do** (`09:246-253`, "the
honest boundary of what I verified"), and it is where the gap comes back: `09`'s `LogicalNumber` is
airtight precisely because it is phantom, and the moment it holds real bytes it acquires a projection,
and the projection is a discrimination surface.

So the residual attack surface is not "the lowering members". It is **whatever the physical type's own
struct bound forces the law crate to be able to name**. That is the dial, and it has a floor above zero,
because the bytes have to come from somewhere.

**And the dial can be turned to closed.** The attack above works because `u64` is spellable in the law
crate: `core` is always linked. Topology B keeps the same `Carrier` shape and changes only the spelling
of what it projects, so that every inhabitant of the carrier is one generic form whose parameter only the
lowering crate names:

```rust
// crate `store`:      pub struct Bits<W>(pub PhantomData<W>);   // nameable everywhere
// crate `lowering2`:  pub struct Minimum; pub struct Doubled;   // nameable only here
// crate `carrier2`:   pub trait Carrier { type Store: Copy; }
// crate `numeric2`:   pub struct Number<N, S: Carrier>(pub S::Store, pub PhantomData<N>);
```

The honest law builds in a crate with no `lowering2` link. The same attack, retried:

```
error[E0433]: cannot find module or crate `lowering2` in this scope
  --> b2_attack.rs:11:42
   |
11 | impl<N, S: Policy + Carrier<Store = Bits<lowering2::Minimum>>> AddAssoc for Number<N, S>
   |                                          ^^^^^^^^^ use of unresolved module or unlinked crate `lowering2`
```

The rule this gives, stated so it can be checked rather than remembered:

> **The carrier's inhabitants must not be spellable using names the law crate can reach.** One generic
> form from a crate everyone sees, parameterised by markers only `arvo-lowering` declares, satisfies it.
> A bare `u64`, or a width spelled as a `Nat` from `arvo-numeral` (which the law crate must link, since
> laws read identity), does not.

I flag that second case rather than resolve it, because I cannot settle from the design text whether
arvo's real carrier ends up spelled `Bits<Pz<...>>` with a numeral-side width, which the law crate can
name, or with a `StoredWidth` marker (`Minimum` / `DoubleLogical`, `110:1540`), which it cannot. **That is
a concrete thing to check when the carrier is declared, and it is cheap: one attempted equality
constraint in the algebra crate, which either resolves or does not.** The shipped facade's shape is
encouraging on this point, since `UFixed` is already a newtype over `Bits<..>` rather than over a bare
primitive, so the "one generic form" half is satisfied by the current shape and only the parameter's
provenance is open.

---

## 5. The trap, and it is the single most likely thing to be built next. Compiled.

Section 2 named the split's real cost: there is no collective noun for `S`. The obvious fix, which any
reasonable engineer reaches for on day one and which the whole surrounding rule set pushes toward, is a
convenience trait:

```rust
pub trait Strategy: Policy + Lowering {}
impl<T: Policy + Lowering> Strategy for T {}
```

**Do not do this.** It silently undoes everything in section 3, and it does so while passing the test
everyone would use to check it.

Topology C, second half. A crate bounds on a trait whose supertrait it does not link. First, with the
supertrait's crate not even on the search path, the crate will not load at all (`E0463`). Put it on the
search path but do not `--extern` it, and the bound compiles. Then ask whether the supertrait is
nameable:

```
error[E0432]: unresolved import `lowering3`
 --> c2c.rs:3:5
  |
3 | use lowering3::Lowering;
  |     ^^^^^^^^^ use of unresolved module or unlinked crate `lowering3`
```

So the `E0432` test still passes. Now ask whether its member is **projectable**:

```rust
// c3_proj.rs, same crate, `lowering3` still not linked
impl<S: PolicySup> Law for Carrier3<S> where S::Layout: IsDense {}
```

`libc3_proj.rlib` builds clean.

**Projection travels through a supertrait edge that nameability does not.** A crate that cannot write
`use lowering::Lowering` can still write `S::Layout: IsDense` if anything it bounds on has `Lowering`
above it. The `E0432` check reports closed and the design is open.

Two consequences, and both are cheap to state as standing rules:

No supertrait edge may run from a trait the law crate bounds on to a trait the law crate must not read.
In topology B the edge runs the safe way (`Lowering: Carrier`, and the law crate bounds on `Carrier`),
which is why B closes. `Carrier: Lowering` would not.

The convenience `Strategy` trait is exactly the forbidden edge. If op wants the name back, it has to be
a **type alias for a bound at consumer-facing positions only**, or a documentation convention, never a
supertrait, and the design should say so in the sentence where it says what `S` is, because someone will
propose it otherwise and it will look harmless.

---

## 6. The consumer price, which is where I expected the argument to be decided and is not

The design's stated aim is to be invisible downstream. On that measure the two shapes are the same, and I
mean that literally rather than approximately.

At the call site, a consumer writes `UFixed<13, 3, Warm>`, or `Number<Fix13_3, Warm>`. **One parameter
either way.** The trait count is not visible. It becomes visible only at a consumer's own generic
boundary, where the split forces `S: Policy + Lowering` instead of `S: Strategy`. That is the entire
consumer-facing cost, it is one extra identifier on functions that read both halves, and it carries
information: a function bounded `S: Policy` alone is declaring that it does not read the cost axis, which
is the same property arvo wants for its own laws, made available to consumers for free.

I expected diagnostics to decide this, since the diagnostic surface is where `08` killed the parameter
split at a measured 1.8x (`08:160-172`), and `110:4862-4866` records the coupling as unresolved. So I
measured it. Same failure, an unsatisfied law bound reaching a consumer, under both shapes:

| | split (`S: Policy + Carrier`) | fused (`S: Strategy`) |
|---|---|---|
| rendered lines | 21 | 21 |
| rendered characters | 851 | 777 |
| structure | `E0277`, help, note, required-by | identical |

The 74-character difference is entirely identifier length in my probe's own naming
(`StableUnderTranslation` against `Stable`), not structure. **The bound on `S` does not appear in either
message**, because the failure is on a projected member and rustc reports the projection, not the
parameter's own bound. So the trait-count question has **no diagnostic cost at all**, and the 1.8x figure
belongs to the parameter-count question, which is closed and is a different question. That coupling can
come off the open list.

On implementation cost: the split costs one extra impl block per preset and one extra crate. The fused
shape costs section 3. Those are not close.

---

## 7. Where two readings survive

One thing here does not resolve on the evidence and I will not pretend it does.

**Whether the residual gap of section 4 is worth closing with a mechanism at all.**

Under the first reading, it should be closed structurally: the carrier spelling rule costs one trait and
one dependency edge, a property the compiler enforces is one nobody can forget, and this design has
already spent far more than that on smaller guarantees.

Under the second, it should be left as a review matter with a compile-fail test. The threat is entirely
first-party, it lives in a small enumerable set of impls (one blanket impl per law trait, in the crate
that owns the law trait), and a compile-fail test asserting that a lowering-conditioned law does not
build is cheaper than a trait and reports the same thing.

**What distinguishes them is a number nobody has stated: how many law impls the design ends up with, and
whether they are one-per-law-trait or proliferate per numeral or per preset.** A dozen impl headers in
one file is a review surface a person can hold in their head; a generated family is not. If op or a later
member can say which, the reading follows immediately. I could not find the answer in `110` and I am not
going to guess it.

---

## 8. What I recommend, and what op has to decide

**Keep the split. Three traits, two parameters, unchanged.** Not on `02`'s ground, which is dead and
should stay dead, but on section 3's: fusing `Policy` and `Lowering` puts the cost axis in scope
wherever the policy axis is read, and no crate boundary can then cut between them. The split is the
precondition for the only enforcement mechanism the panel has verified. It is also the smaller move,
since it is what ships, and rewrite cost is the tiebreaker when the intent does not decide.

That recommendation carries three riders, and they matter more than the recommendation does, because the
recommendation preserves the current shape and the riders are where the current shape is incomplete.

**One.** State what bounds `S`. `110` never does, in 5900 lines, and it is the sentence that makes the
whole shape legible. `S: Policy + Lowering` is the form `26:28-35` carries; whether it is what op wants
is his word, but the design cannot leave it unstated.

**Two.** Name the carrier explicitly and put it in its own crate below `Lowering`. It is the one lowering
fact the physical type's definition forces the law crate to name, it is on the wrong side of the crate
cut relative to D54's axis cut, and that mismatch is exactly why `09`'s gap existed. This is one small
trait, and it converts "unclosable without an architectural change past D72" into "closed, subject to a
spelling rule". It does not need `LogicalNumber` and it does not separate the law's carrier from the
bytes.

**Three.** Forbid any supertrait edge into `Lowering`, and specifically forbid a
`Strategy: Policy + Lowering` convenience trait, in the design text rather than in anyone's memory.
Section 5 shows it defeats the mechanism while passing the check that would be used to verify it, and
the surrounding rule set is actively pushing someone toward writing it.

**What is genuinely op's, stated so a one-line answer discharges it:**

Ratify or refuse the three-trait split. If ratify, the marker belongs on the trait table at
`110:2898-2916`, where `Numeral` already carries "ratified: identity contract" and the split itself
carries nothing.

Choose between the three treatments of section 4's residual gap: the carrier crate (rider two), a
compile-fail test, or accepting it named and open. Section 7 says what would decide it if op would rather
not decide it blind.

Say whether the `Strategy` name comes back, and if so as what. It cannot come back as a supertrait.

**And one thing that does not need op.** If the honest answer to "does this difference matter enough to
change anything" had been no, I would have said so, and it would have been the cheapest result available.
It is not no, but it is close: **the recommendation changes nothing that ships.** The split stays, the
parameter count stays, the diagnostics are measured equal, the consumer's call site is identical. What
changes is that the split acquires a reason that survives `08` and `09`, one unstated sentence gets
stated, one small crate gets named, and one attractive shortcut gets forbidden before somebody builds it.
That is a day of design work, not a rewrite, and downstream is not blocked on any of it once the one-line
ratification lands.

---

## Trusted base, named because leaving it implicit is what this panel keeps catching

Section 3's mechanism trusts that the law-bearing blanket impl is written in the crate that **declares**
the law trait, and that this crate has no dependency edge to `arvo-lowering`. If a law impl is ever
written anywhere else, `E0117` stops it (compiled, topology A). If the algebra crate ever acquires a
`lowering` dependency for an unrelated reason, the whole mechanism goes silently, with no diagnostic
anywhere. **That dependency edge's absence is a load-bearing part of the design and should be written
down as such**, next to the crate table at `110:3110-3120`, rather than living in a `Cargo.toml`.

Section 4's closure additionally trusts the carrier spelling rule, which I verified in a model topology
and which arvo's real carrier may or may not satisfy. That check is one attempted equality constraint and
it should be run at the moment the carrier is declared.

Nothing here touches the leaf-truth question. Whether `StableUnderTranslation` computes the right fact
for each `Resolution` constructor is `01`'s and `03`'s question, orthogonal to all of the above, and a
completely closed enforcement story around a wrong fact is a well-defended wrong answer.
