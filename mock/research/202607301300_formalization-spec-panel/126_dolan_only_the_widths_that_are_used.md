# 126. Only the widths that are used: what `Capacity` actually does, and what it does to the table

**Persona:** Stephen Dolan, type inference and representation lens. Third pass in this panel; file 118
declared the bridge, file 123 priced its ceiling, file 79 read `Capacity` at the theory layer.
**Date:** 2026-08-06
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, scratch tree outside the
repository. `mock/crates` untouched, `mock/design_rounds/` untouched.

Op rejected the generated table and named the precedent: `Capacity`, which expressed the same guarantee
the forbidden feature had expressed, in a form the solver accepts. I went and read what `Capacity`
actually does before reasoning about it, because op is citing it as a fact rather than as a memory, and
the fact turns out to be sharper than the panel's summary of it.

**`Capacity` does not convert anything.** It never derives a type from the const. It keeps the const
standalone and does its computing in value position. Written out as a rule, that rule is checkable, and
rustc states it in its own words when you break it: *const parameters may only be used as standalone
arguments here.*

Applied to the width, the same move works, and it works better than anyone in this panel has assumed.
**A numeral that carries its const is one impl, admits any width, chooses no cap, enumerates nothing, and
realises only the widths a program actually writes.** It compiles with no feature gate at all, it does
type-level arithmetic whose result is a type, and 64 distinct four-digit compositions cost 0.04 seconds
against the 4096-row table's 0.06 (`125:245-250`). That is op's stated property, delivered, and it is
below.

It is not free, and the price is not where the panel has been looking. It is not the escape. **The table
is paying for two things the escape was never the name of: canonicity, and the ability to choose a type
from a width.** Both are real, one of them is load-bearing, and the second is an open defect in the canon
that has nothing to do with the numeral and that the numeral has been made to pay for.

So my answer is neither of the two on offer. Files 119 and 123 concluded the table is forced; that
conclusion is sound about what it actually proves and wrong about why, and the difference is what op is
reaching for. File 125 concluded the two spellings should coexist; op called that convoluted and he is
right, because the second spelling is answering a question the first one should never have been asked.

---

## 0. The gates, and the brief's claims checked before reasoning from them

**Canon gate: passed.** The governing material is op's own: `13c`'s standard at `110:66-71`, the end-state
criterion at `110:403-406`, the pricing licence at `110:505`, and `16d:14-15`'s tiebreaker. This file
proposes nothing those forbid and reopens nothing op has not himself reopened. `panels-argue-the-intent-not-the-wording.md`
governs the reopening: op's calls are the only final ones and even those go stale, and D48 is his to
revisit. One note carried forward from `125:56-70` and not re-litigated here: **any recommendation that
drops the const spelling asks op to overturn D48**, and mine does not ask him to.

**Test gate: run, not cited.** `cargo test --offline --workspace` from `mock/`, 2026-08-06, summed per
binary by parsing every `test result:` line: **155 binaries, 672 passed, 0 failed, 9 ignored.** That
reproduces `118:47-49`, `119:25-27` and `125:76-78` exactly, on a tree nothing has moved. I did not
re-audit the bodies, for the reason `119:28-33` gives and op ruled at `108b:174-181`: the collected
tautologies are an implementation-phase checklist, and a seventh report of the same three findings is what
that ruling exists to stop. The suite covers a tree the canon replaces, so it is the weakest instrument
in the room. The instrument that measured something this pass was the compiler, twelve times.

**The brief's factual claims, checked before reasoning from them.**

*"The workspace's record quotes `arvo-comb`'s lib.rs."* **Holds**, `arvo-comb/src/lib.rs:16`, verbatim:
"No `generic_const_exprs` / `adt_const_params` gates: the capacity is a TYPE".

*"The crates that adopted `Capacity` dropped their feature gates."* **Holds for four, and the claim is
narrower than it reads.** `arvo-comb/src/lib.rs:16`, `arvo-graph/src/lib.rs:17`, `arvo-sparse/src/lib.rs:11`,
`arvo-spectral/src/lib.rs:11` each carry a comment saying the gate is gone. **Two crates still gate it:**
`arvo-strategy/src/lib.rs:11` and `arvo/src/lib.rs:25`, both `#![feature(generic_const_exprs)]`, both
listed as drift to remediate in `unstable-features.md`. That is not a quibble. Section 5 shows the
surviving two gate it for **exactly the operation the width bridge needs**, which means the precedent op
is citing has a known unfinished half, and the unfinished half is the half this question turns on.

*"119 refuted three routes."* **Holds** (`119:56-121`), and I re-ran the load-bearing one plus a fourth
that nobody tried. Section 2.

---

## 1. What `Capacity` actually does: two doors, and rustc names the rule

`arvo-tensor/src/capacity.rs:44-59`, the whole mechanism:

```rust
pub struct Dim<const N: usize>;

impl<const N: usize> Capacity for Dim<N> {
    type Array<T> = [T; N];
    const CAP: Cap = cap(N);
}
```

**One impl.** Not a row per capacity. The trait is non-generic so a consumer binds `C: Capacity` with no
const parameter (`capacity.rs:38-43` says so in its own words), and `Dim<N>` is generic over the const, so
the only capacities that exist are the ones a program instantiates.

The move is not "make the capacity a type" as the shorthand at `arvo-comb/src/lib.rs:16` puts it. That
description is true and it is the consequence rather than the mechanism. **The mechanism is that the const
is never computed on in type position.** It is used in exactly two places, and those two places are the
only two the language permits without the forbidden feature:

**Door one, the standalone const argument.** `[T; N]` uses `N` bare. So does `Dim<N>`, `W<N>`, `Idx<N>`.
No arithmetic, no function call, no expression. Compiled, `p11_doors.rs`, exit 0.

**Door two, the const value body.** `const CAP: Cap = cap(N)` calls a const fn on `N` and gets a **value**.
Ordinary associated-const bodies may compute from generic parameters without restriction. Compiled, same
file, same exit.

**The refused third door is a computed const argument**, and rustc states the rule rather than leaving it
to be inferred (`p12_thirddoor.rs`):

```
error: generic parameters may not be used in const operations
6 | impl<const N: usize> Capacity for Dim<N> { type Array<T> = [T; N.div_ceil(8)]; }
  |                                                                ^ cannot perform const operation using `N`
  = help: const parameters may only be used as standalone arguments here, i.e. `N`
```

That help line is the whole of `Capacity`'s trick, stated by the compiler. **A const may be carried and it
may be read. It may not be transformed on the way into a type.** Every GCE-free construction in arvo obeys
it and every one that does not still carries the gate.

This is worth having as a canon sentence in its own right, independent of the width question, because it
is the test that would have caught the two surviving gates at the time they were written.

---

## 2. The fourth route, refuted, and the control that shows which half of the door is open

`119:56-121` shut three routes from a const to a type. I re-ran the load-bearing one and it reproduces.
Then I tried a fourth that nobody had, and it is the one that looked most likely to work, because
`110:3763-3765` establishes that a **path** resolves under `min_generic_const_args` where an expression
does not, and `119:85-99` only tested whether a `type const` body may compute directly. It never tested
whether the computing could be moved somewhere legal and the `type const` reduced to a path at it.

**Route four, split the computation from the naming.** An ordinary associated const does the arithmetic,
where arithmetic is legal, and a `type const` is a bare path to it, where paths are legal
(`p2_typeconst_via_path.rs`):

```rust
pub trait HalfInner { const H: u16; }
impl<const N: u16> HalfInner for W<N> { const H: u16 = N / 2; }

pub trait Halve { type const HALF: u16; }
impl<const N: u16> Halve for W<N> { type const HALF: u16 = <W<N> as HalfInner>::H; }
```

```
error: use of `const` in the type system not defined as `type const`
14 |     type const HALF: u16 = <W<N> as HalfInner>::H;
   |                            ^^^^^^^^^^^^^^^^^^^^^^
help: add `type` before `const` for `HalfInner::H`
```

**The wall recurses.** Every link in the chain must itself be a `type const`, and the terminal link is
where the arithmetic has to happen, and there it is refused for the reason 119 gave
(`p3_chain_terminal.rs`): `error: complex const arguments must be placed inside of a const block`. There is
no legal position anywhere in the chain for the division. The escape hatch is closed all the way down.

**The control matters more than the refutation, because it says precisely which half of the door is open**
(`p4_identity_forward.rs`, exit 0):

```rust
pub trait Id { type const V: u16; }
impl<const N: u16> Id for W<N> { type const V: u16 = N; }
pub type Round<const N: u16> = W<{ <W<N> as Id>::V }>;
```

A const parameter can travel **out** of a type through a `type const` and back **into** type position, a
full round trip, with no feature gate. What cannot survive the trip is one arithmetic operation. So
`119:96-99`'s reading, that the opening carries a value out and never carries a computed value in, is
right and is sharper than it was stated: the transport is bidirectional and it is **the computation, not
the direction**, that is refused.

Which is the same rule as section 1, arrived at from the other side. Two independent compiles, one rule.

---

## 3. The construction: only the widths that are used

Apply section 1's rule to the width instead of to the capacity. Do not derive a numeral from the const.
Carry the const, and read it in value position.

`p5_direct.rs`, in full, **no feature gates at all**, exit 0:

```rust
pub trait Nat { const VAL: u32; }

pub struct W<const N: u32>;
impl<const N: u32> Nat for W<N> { const VAL: u32 = N; }

pub struct Sum<A, B>(A, B);
impl<A: Nat, B: Nat> Nat for Sum<A, B> { const VAL: u32 = A::VAL + B::VAL; }

pub type UFixed<const I: u32, const F: u32, S> = Number<Sum<W<I>, W<F>>, S>;
```

Three impls where the incumbent has four thousand and ninety-six plus a tower of twenty-one. Door one
carries `I` into `W<I>` standalone. Door two reads it back as `VAL`. `Sum`'s body computes, which is legal
because it is a value.

**Every property op named, checked at compile time in the same file:**

```rust
const _: () = assert!(<Sum<W<13>,  W<3>>  as Nat>::VAL == 16);
const _: () = assert!(<Sum<W<40>,  W<30>> as Nat>::VAL == 70);        // 123's escaping case
const _: () = assert!(<Sum<W<100000>, W<3>> as Nat>::VAL == 100003);  // outside any proposed table
const _: () = assert!(<Prod<W<65535>, W<65535>> as Nat>::VAL == 4294836225);
```

No admitted range, because there is nothing to admit into. No ceiling, because nothing enumerates. No cap
chosen by the design, because the design chooses nothing: the ceiling is the const parameter's own type,
which is where `123:44-51` correctly located the outer bound and which is not a design decision.

**And the requirement 119 said forces the bridge is met without one.** `119:46-51` states it as the whole
case: the bridge is necessary if and only if `Precision` participates in type-level arithmetic whose
result is a type. Here is that arithmetic, with the result a type, no bridge and no gate:

```rust
pub fn mul<const I: u32, const F: u32, const J: u32, const K: u32>(
    _a: UFixed<I, F, Warm>, _b: UFixed<J, K, Warm>,
) -> Number<Sum<Sum<W<I>, W<F>>, Sum<W<J>, W<K>>>, Warm> { todo!() }
```

Exit 0. A product's precision is inhabited rather than read, which is `119:48`'s own criterion, and no
width was enumerated to inhabit it.

**Measured, because a construction that is slow is not a construction.** 64 distinct compositions at
four-digit widths, each with its own compile-time assertion, `/usr/bin/time -p`, three runs
(`p8_scale.rs`): **0.46 s cold, then 0.04 s, 0.04 s.** Set that against `125:245-250`'s own table for the
identical workload: 0.06 s through a 4096-row table, and 5.87 s through use-site realisation. **The
construction with no table is faster than the table**, at widths the table does not hold, in a crate that
links nothing. The negative control fires
(`error[E0080]: evaluation panicked: assertion failed: <Sum<W<1000>, W<1000>> as Nat>::VAL == 2001`).

---

## 4. What it costs, and it is exactly two things

I want to be as exact about the price as about the property, because the panel has twice now stated a
forcing claim without compiling it (`110:3756-3760`, and `110:3633` caught at `125:301-346`).

**Cost one, canonicity.** Equal precisions are no longer the same type (`p6_normalform.rs`):

```
error[E0308]: mismatched types
11 | pub fn assign(x: Number<Sum<W<13>, W<3>>>) -> Number<W<16>> { x }
   = note: expected struct `Number<W<16>>`
              found struct `Number<Sum<W<13>, W<3>>>`
```

This is the real content of the structural tower, and **nobody in this panel has named it.** A binary
numeral built by carry-chain addition is a canonical form: one type per value, so type identity tracks
value equality, so the product of two `UFixed<13, 3>` has a type the consumer can spell as
`UFixed<26, 6>`. Under section 3 it does not. The consumer's declared field type and the expression's type
are different types holding the same number.

There is an escape and it is worth pricing rather than dismissing (`p9b.rs`, exit 0):

```rust
pub fn retype<A: Nat, B: Nat>(x: Number<A>) -> Number<B> {
    const { assert!(A::VAL == B::VAL, "precision mismatch") }
    Number(x.0, PhantomData)
}
```

The wrong coercion is refused at compile time with the design's own message (`p9c.rs`):

```
error[E0080]: evaluation panicked: precision mismatch
   = note: the above error was encountered while instantiating
           `fn retype::<Sum<W<13>, W<3>>, W<17>>`
```

**It works and it is weaker than a bound, and the weakness should be stated plainly rather than sold.**
The check is post-monomorphisation. It cannot be discharged as a `where` clause, it does not appear in a
signature, and a generic function that retypes is unchecked until instantiated. Against `110:3506-3509`'s
four-bin ledger this is a real claim mechanically checked at every use, which is better than `unargued`,
and it is not the same thing as an unrepresentable illegal state.

**Cost two, and this is the load-bearing one: a type cannot be chosen from a width.** The container is such
a type. `p10_container.rs`, the shape the design needs:

```rust
impl<P: Nat> Lower for P { type Container = <Picker as Project<{ tag(P::VAL) }>>::T; }
```

```
error: generic parameters may not be used in const operations
   = note: type parameters may not be used in const expressions
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

Section 1's rule again, third instance. A numeral that carries a value can be read but its value cannot key
an impl, and a structural numeral can, because structural recursion over a closed grammar is trait
resolution rather than const arithmetic. That is what `123:161-175`'s five-impl carrier demonstrates and it
is the tower's second genuine job.

---

## 5. Where 119's reason is wrong, and why the correction is the whole finding

`119:117-121` concludes the table is forced and I agree with the conclusion. **Its stated reason is wrong,
and correcting it moves the question somewhere op can actually decide it.**

119 argues: the tower needs type-level arithmetic producing types, a const does not participate in that, so
the const must escape to a type, and the escape needs an enumeration. The middle step is false. Section 3
compiles type-level arithmetic producing types, over const-carried numerals, at any width, with no escape
performed and no enumeration written. **A const parameter participates in type-level arithmetic perfectly
well provided the arithmetic runs in value position and the composition runs in type position**, which is
`Capacity`'s trick and which 119 did not consider because it was looking for an escape rather than for a
way to avoid needing one.

What actually forces the table is section 4: canonicity, and choosing a type from a width. **Neither is
what "bridge" names**, and the difference is not academic, because the two costs have very different
standing:

- **Canonicity is a property the design wants and has never stated as a requirement.** Search `110` for a
  sentence requiring that equal precisions be the same type and there is none; the tower was adopted for
  the arithmetic, and canonicity arrived with it as a free consequence that nobody costed. It is now
  load-bearing for the consumer surface and nobody decided that it should be.
- **Choosing a type from a width is not the numeral's problem at all.** It is the container dispatch, it
  exists identically in shipped arvo, and shipped arvo solves it with `generic_const_exprs`. Section 0's
  check found the surviving two gates and this is what they are for: `arvo-strategy/src/container.rs:137`
  declares `Project<const TAG: usize, ...>` and `container.rs:114` declares `BitsContainerFor<const N: u16, ...>`,
  and joining them requires `{ tag_hot_cold(N) }` in const-argument position, which is why
  `arvo-strategy/src/lib.rs:11` still carries the gate the canon forbids.

**So the design has one problem, not two, and it has been paying for it twice.** The width bridge and the
container dispatch are the same question: how does a const-written width choose a type, without GCE. The
canon has an unremediated GCE gate for the second and a 4096-row table for the first. Whatever answers one
answers the other, and the numeral should not be made to carry an enumeration on behalf of a lowering
decision that has its own unresolved gate two crates away.

`container.rs:124` is worth reading closely because it is the shape of the answer if there is one:
"Distinct impls keyed on the const-generic `TAG` value avoid E0119 overlap." That is the case split 119
proved is the only one available, **performed over a codomain of six rather than a domain of 65536.** The
enumeration that survives in shipped arvo is over the container ladder, which the hardware chooses, and not
over the widths, which nobody should. It costs GCE only at the joining step. If that joining step can be
made GCE-free, the per-width enumeration disappears from both places at once. I did not find a way to make
it GCE-free and I am not claiming one exists. I am claiming it is the right place to look, and that it is
not where anyone has been looking.

---

## 6. The categories, enumerated, so the next reader does not repeat them

Going wide before deep, as instructed. A width reaches the type system by one of these and there is no
seventh that I found.

| | mechanism | enumerates | cap | status |
|---|---|---|---|---|
| 1 | impl per admitted width | the domain | forced | the incumbent, works, `119:117` |
| 2 | structural recursion on the const | nothing | none | shut, `119:64-83`, arithmetic and `E0119` |
| 3 | `type const` path, direct | nothing | none | shut, `119:85-99` |
| 4 | `type const` path via a computing assoc const | nothing | none | **shut, new, section 2** |
| 5 | specialise the base case | nothing | none | shut, forbidden feature, `119:101-115` |
| 6 | carry the const, read in value position | nothing | none | **works, section 3, loses section 4** |
| 7 | const parameter of an ADT type | nothing | none | reduces to 2 or 6; see below |
| 8 | use-site macro over digits | the alphabet | none | works, priced at `125:245-250` |
| 9 | classify into a small codomain | the codomain | none | needs GCE at the join, section 5 |

**Seven is worth its own sentence because the brief named it and it does not survive contact.**
`adt_const_params` permits a const parameter whose type is a user ADT, so a numeral could be a const
*value* rather than a type. Its arithmetic is then const fn arithmetic with no trait recursion, which is
the brief's own framing, and it is exactly category 6 with a richer payload: the sum of two such consts is
a value, and putting that value back into type position is a computed const argument, refused by section
1's rule. It buys nothing over 6 and costs a `ConstParamTy` derive. **If nothing broke, it would be the
whole answer; what breaks is the same thing that breaks in 6**, which is why they are one row and not two.

**Nine is where I would spend the next dispatch**, per section 5.

---

## 7. On-demand realisation is not intrinsically expensive, and the brief asks

`125:245-250` measures use-site realisation at 5.87 s for 64 four-digit compositions and correctly declines
to say whether that is intrinsic. It is not, and the reason is visible in `125:253-258`'s own diagnosis: the
cost is decimal, not the parameter kind, because each decimal digit multiplies the accumulator by ten
through type-level structural addition.

**So the cost is the cost of converting to a canonical binary form, and it is paid only by a design that
has one.** Section 3 has no canonical form and therefore performs no conversion, and section 3's own
measurement is the evidence: 0.04 s for the same 64 four-digit compositions, against 5.87 s for use-site
realisation and 0.06 s for the table. Op's phrasing, that only used widths should realise on const time and
resolve just the same, describes a system with no realisation step at all, and that system is cheaper than
either measured alternative.

The general form, which I would put in the canon because it will be reached for again: **a staged
conversion is only worth staging if there is something to convert. The table is a cache for a computation
that a value-carrying numeral does not perform.** `125:279-284`'s binding-time argument for keeping the
table is correct given a canonical target and has no force without one, and 125 could not have known that
because it took the tower's representation as given.

---

## 8. What I would suggest

Not a recommendation to adopt section 3 wholesale. It has a stated cost and one half of that cost is
serious. What I would suggest is that the round stop asking the question it has been asking, because that
question has been answered four times and the answer keeps being the same.

**One. Stop calling it a bridge, and split the numeral's two jobs in the canon's own text.** The numeral is
asked to be the value that arithmetic produces, and the key that selects a lowering. Job one needs no
enumeration and no cap (section 3, compiled). Job two needs one impl per width or a GCE-free classifier
(section 4, compiled). Written as one thing, the cheap job pays the expensive job's price, and a 4096-row
table sits in the canon looking like the cost of arithmetic when it is the cost of a container.

**Two. Take the container dispatch first, because it is the foundational one and it is already broken.**
`arvo-strategy/src/lib.rs:11` carries a forbidden gate for exactly this operation and
`unstable-features.md` lists it as drift to remediate. Whatever answers it answers the width question, and
until it is answered the width question cannot be answered honestly, because any width-to-type mechanism
the round adopts will be superseded by the container's. **That is the order I would take these in**, and it
is the one thing in this file I would argue for over its alternatives.

**Three. If the container dispatch is answered by a per-width enumeration, the range is not op's to pick
and it is not 4095.** It is the extent of the native container ladder, which
`arvo-strategy/src/container.rs:60-91` already fixes at 128 by way of `u8` through `u128`, above which
every strategy dispatches through `WideBits`. A range set there is not a policy about what a workload
should need. It is the hardware's own list, which is exactly the answer `arvo-toolbox-not-policer.md`'s
"No bit-width cap below the largest container the substrate is willing to dispatch through" asks for, and
it removes op's "arbitrary" at the source instead of documenting it. `123:246`'s 4095 was derived from a
compile-time budget, which is the wrong kind of reason for a number in a numeric type, and `123` says so in
its own voice at `123:246-247`.

**Four. Do not ship the two spellings side by side.** `125:432-449` proposes it and op called it
convoluted. He is right for a reason worth recording: a second surface that exists to escape the first
one's ceiling is an admission that the ceiling should not be there, and shipping both makes the admission
permanent. If the ceiling is the container ladder (three above), no escape hatch is needed, because nothing
below the ladder is refused and nothing above it was ever representable.

**What I am not suggesting**, and I say so because it is the easiest misreading of this file: I am not
suggesting the canonical form be abandoned. I found what it costs and what it buys; I did not find that it
is unnecessary, and the consumer-facing consequence in section 4 is severe enough that abandoning it needs
its own decision with its own evidence. **What I am suggesting is that it be named**, because a property
this load-bearing arrived as a side effect of a representation choice and has never been written down as a
requirement, and the enumeration op objects to is its price rather than the bridge's.

---

## 9. What is op's, what is a second read's, and what is a defect

**Op's, and newly well-posed:** whether equal-valued precisions must be the same type. That is the whole
of the table's justification once section 5 removes the arithmetic argument, and it has never been stated
as a requirement anywhere in `110`. One sentence either way, and the answer decides the mechanism rather
than the mechanism deciding the answer. I did not answer it, because the record contains nothing to answer
it from.

**A defect, not a choice:** `arvo-strategy/src/lib.rs:11` and `arvo/src/lib.rs:25` still gate
`generic_const_exprs`, which `unstable-features.md` forbids outright and lists as drift to remediate. The
canon's numeral design cannot be settled while the crate it will lower through depends on the feature the
numeral exists to avoid. This is outside the question I was dispatched on and the standing instruction says
to report it anyway.

**Second reads owed, because one expert's word is not a call.** I am the first read on three things:
that section 1's two-door rule is the correct general statement of the `Capacity` precedent; that
`119:46-51`'s stated reason for the bridge is false; and that the width bridge and the container dispatch
are one problem. The first two are compiled and a second read should re-run them rather than agree with
them. The third is reasoned, and it is the one I would most want checked.

**Not open:** whether a value can escape from a const parameter to a type without an enumeration. It
cannot. `119:56-121` is sound, I re-ran its load-bearing route and refuted a fourth it had not tried, and
nothing here weakens it. What this file establishes is that **the design does not need that escape for
arithmetic**, which is a different claim and leaves 119's undisturbed.

---

## Verification

Every figure and diagnostic above was produced this pass under the pin, from
`scratchpad/w/` outside the repository: `p1_assoc_const_path.rs`, `p2_typeconst_via_path.rs`,
`p3_chain_terminal.rs`, `p4_identity_forward.rs` (exit 0), `p5_direct.rs` (exit 0),
`p6_normalform.rs`, `p7_classify.rs`, `p8_scale.rs` (exit 0, timed three runs after a discarded first),
`p9b.rs` (exit 0) with `p9c.rs` as its negative control, `p10_container.rs`, `p11_doors.rs` (exit 0),
`p12_thirddoor.rs`. Timing is `/usr/bin/time -p`. One probe defect is recorded rather than hidden: the
first `retype` was written `const fn` and failed on `E0493` destructor evaluation, which is a fact about
my probe and not about the mechanism; `p9b.rs` is the corrected form. Source facts cite
`arvo-tensor/src/capacity.rs`, `arvo-strategy/src/container.rs`, `arvo-strategy/src/lib.rs`,
`arvo/src/lib.rs`, `arvo-comb/src/lib.rs`, read this session for existence and shape, not for meaning.
Nothing under `mock/crates/` or `mock/design_rounds/` was modified.
