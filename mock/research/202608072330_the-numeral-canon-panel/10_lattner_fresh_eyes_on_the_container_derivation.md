# 10. Fresh eyes on the container derivation

**Persona:** Chris Lattner, compiler and infrastructure lens.
**Date:** 2026-08-08.
**Pin:** `rustc +nightly-2026-05-28`, reporting `1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`,
aarch64-apple-darwin. Every compile below names its command.
**Probes:** `10_probes/`, twenty sources plus `10_probes/reproduced/` (five sources copied from
`137_probes/` and recompiled here) and `10_probes/out/` (emitted metadata, assembly and diagnostics).
**Status:** nothing here settles anything, per `04`. A confirmation is a confirmation.

## What this dispatch is

Op's standing instruction at `01_op_answers.md:72-77` is not a question waiting on him. It is an order
four dispatches have walked past:

> We didn't land on this. If I remember correctly, my call on the latest, I tihnk very good attempt at
> it, was that we need fresh eyes to confirm it or improve on it. But it was a very good overall place
> already for this. And pretty sure that did not contain any enumeration, and had the contracts and
> typestate work fully (within its framework, some things might've changed later or change now, but
> just goes to say it's doable) without forbidden features. It was pretty nice, no reason to try to
> force it or force anything else. I said fresh eyes on it, that holds.

Against that, `SETTLED.md:149-153`:

> **The container-derivation mechanism was never ratified.** The `SETTLED_surface.md` sweep names this
> as the reason: every gate-free candidate the panel built either enumerated widths or degraded the
> diagnostic to something unreadable, and op declined the trade each time rather than picking a side.
> The erasure gate is ratified and its mechanism is not, which is the largest structural gap in the
> panel.

Both cannot be true of one artifact. Neither op's recall nor a survivor sweep's summary settles it.
Opening the artifact does, and that is what follows.

## The short answer, before the working

The attempt is `202607301300_formalization-spec-panel/137_aaltonen_erasure_without_a_condition.md` and
its `137_probes/`. It still stands on the pin: four cited probes compile clean, gate-free, and the
erasure result reproduces.

Three of op's four properties hold as he states them. The fourth, no enumeration, holds of the
**ladder** and fails of the **bridge**, which is the one line op himself refused at `137b:28-41`. His
recall is accurate about the part he was looking at and forgets the part he personally rejected. The
contradiction is not between two artifacts; it is between two distances from one artifact.

`SETTLED.md`'s sentence is the mirror error. Its "either enumerated widths or degraded the diagnostic"
is true of `137` taken whole and false of every component taken separately, and by joining them with
"either" it reads as a trade between two mechanisms when `137` has one of each in two different places.
Neither is a trade anyone was offered.

I then improved it, in the same mechanism, four ways, all gate-free and all codegen-neutral. And I
closed four routes toward dissolving the bridge, each with its diagnostic. The bridge itself I did not
crack, and section 8 is the concession with the dead routes enumerated.

---

## 1. Locating the attempt, and a correction to the brief

`seed/SETTLED_container.md:86-89` is the index and it is accurate. It points at `133:271-391` for the
structural construction and `137:99-133` for the wide rung, and names `137` as the file that closed the
gate. `137b_op_checkpoint_thirty.md:10-14` confirms it in op's own voice: "File `137` closed it."

**One correction to my brief, checked before reasoning from it.** The brief says op's call was "fresh
eyes to confirm it or improve on it" on the container attempt. Grepping the closed panel for that
phrase finds it at `140b_op_checkpoint_thirtythree.md:59-71`, and there it is attached to **the headroom
rule**, not to the container derivation:

> Note the structural constraint, hold calls on it until there are actual benches, and until someone is
> confident enough on their take to do the benches, it's not an answer anyone asked for. Which is
> entirely valid as we've now established, but requires fresh eyes on the case before it should ever
> come to me.

Command: `grep -rn "fresh eyes" 202607301300_formalization-spec-panel/`, ten hits, none of them a
container-derivation call.

So op's own hedge is load-bearing. The recorded "fresh eyes" call is about headroom. There may be an
unrecorded verbal one about the container; his sentence in `01` reads like a memory of `137b` rather
than of `140b`, since the four properties he lists are `137`'s properties and not headroom's. **I
proceeded on `137` because the four properties identify it uniquely, and I flag the identification as
mine rather than as recorded.** If op meant something else, this file is aimed at the wrong artifact
and the aiming is checkable from the properties.

---

## 2. Does it still stand on the pin

The first question a confirmation owes is whether the thing still builds. Sources copied unmodified to
`10_probes/reproduced/`.

```
rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib --emit=metadata \
      -o out/<name>.meta <name>.rs
```

| Probe | Result |
|---|---|
| `p4_structural_wide.rs` | OK |
| `p5_total_ladder.rs` | OK |
| `p6_surface_end_to_end.rs` | OK |
| `p7_law_site.rs` | OK |
| `ladder.rs` | fails, `E0425` on `PhantomData` |
| `p5_merged.rs` | fails, `E0119` conflicting implementation |

The two failures are not failures of the attempt. `ladder.rs` is an `include!` fragment (`p6` line 8 is
`include!("ladder.rs")`), so it has no imports of its own and was never meant to compile standalone.
`p5_merged.rs` is a superseded intermediate that `137` never cites. Every probe the file actually cites
compiles.

**Erasure reproduces.** Emitting assembly from `p7_law_site.rs` gives six bodies and 95 instructions,
with `_native16 = _arvo16`, `_native64 = _arvo64` and `_native_vec = _arvo_vec` as aliases rather than
bodies. That is `137:479-483` and `135b:42-46`, reproduced on this pin by a third party.

---

## 3. Op's four properties, one at a time

### 3.1 "without forbidden features": **holds**, and it is stronger than he says

The naive grep is a trap and I fell into it once. `grep -c '#!\[feature'` over `137_probes/*.rs`
returns 3, which looks like three gates. All three are the doc comment asserting there are none:

```
p5_merged.rs:4://! feeding P4's word cons. No `#![feature]`, no `-Z` flag, no width listed.
p5_total_ladder.rs:4://! ... same
ladder.rs:4:// ... same
```

The real query is `grep -n '#!\[feature(' *.rs`, which returns **nothing**. Combined with section 2's
compiles, which pass no `-Z` flag: zero unstable features, not merely zero forbidden ones. The
construction does not reach for `min_generic_const_args`, `adt_const_params` or the const-traits family
either, all of which it is permitted.

That is a stronger property than op remembers and it is worth stating in the canon in those terms: the
derivation is expressible in stable Rust's type system, and arvo's nightly dependence is elsewhere.

### 3.2 "had the contracts and typestate work fully": **holds**

Four things are asserted in the construction rather than argued, and each has a control that fires.

Layout is asserted at five widths (`p6:458-462` as quoted in `137`), including `UFixed<13,3,Hot>` at
`size_of::<u16>()`, `UFixed<3,0,Hot>` at one byte, and `UFixed<100,100,Hot>` at 32 bytes for 200 bits.
`137:465` records that changing the first to `u32` gives `E0080: evaluation panicked`, so the assertion
is capable of failing.

The width-generic law stays on the const coordinates. `p7_law_site.rs` states multiplication's output
widths as const parameters pinned to the structural sum by associated-type equality, and rustc infers
`OI = 26`, `OF = 6` from the relation with no const arithmetic anywhere. I recompiled it and the
inference holds.

Consumer extension works on the full construction, not a reduced one: a second marker with widths 777
and 41 gives an 818-bit numeral at 104 bytes.

A width outside the bridge refuses at the type, before any body runs. That is a real contract, and
section 5 is about the fact that it refuses with the wrong message.

### 3.3 "did not contain any enumeration": **holds of the ladder, fails of the bridge**

This is the whole of the contradiction and it resolves by counting.

```
grep -cE 'impl .*<[0-9]+>' p5_total_ladder.rs      ->  0
grep -oE '[0-9]+ =>' p6_surface_end_to_end.rs | wc  -> 11
grep -oE '[0-9]+ =>' p7_law_site.rs | wc            -> 13
grep -cE '^pub trait Shr[0-9]' p5_total_ladder.rs   ->  6
```

**The ladder enumerates nothing.** Zero width literals appear in any impl in it. Its case-split is
structural: digit count by unary tally for the native rungs, structural ceil-division for the word
count above them, with a `#[repr(C)]` word cons carrying the wide payload so no array length is ever
needed. The six `Shr` peels are not a width enumeration; they are the six bits of `log2(64)`, a fixed
constant of the machine word, and they do not grow with anything.

**The bridge enumerates.** `p6` line 21 is a `macro_rules!` whose body is
`impl ToNat<Arvo> for Idx<$n> { type N = $t; }`, invoked over eleven literals in `p6` and thirteen in
`p7`. That is one impl per written width.

Op refused exactly this, at `137b:28-41`, in the checkpoint immediately after the file:

> Hmm. This really looks like just another instance of the spelling out being the problem, all the
> heuristics should be there. If I understand correctly, that is. It should come implicitly from the
> heavy typestate. No enumerations, if we can help it; and I think we have much to explore to actually
> be able to help it.

So his recall in `01` is off by one item, and the item is one he personally rejected three months of
panel-time ago. That is an ordinary thing for a memory to do and it is why he hedged.

### 3.4 "within its framework, some things might've changed later": **one thing did**

`137:620-625` reports that `Warm` crosses into the multi-limb rung at 65 bits rather than 129, because
of the headroom rule. Op ruled that wrong at `137b:55-85`, the headroom rule was reopened, `140`
proposed deleting it, `141` benched it, and `SETTLED_container.md:410-420` records the whole thread as
**unsettled at panel's end**.

That does not touch the derivation. The ladder does not know what a strategy is; it maps a width to a
container. Where the strategy puts the crossover is an input to it. So `137`'s mechanism survives the
headroom question being open, and a canon sentence about the derivation can be written without waiting
for it.

---

## 4. The contradiction, resolved

They describe the same artifact at two distances, and each drops what the other kept.

**Op is describing the ladder** and the four properties he lists are the ladder's. Read as a claim
about `p5_total_ladder.rs`, every one of them is true, including no enumeration.

**`SETTLED.md` is describing the panel's whole sequence of candidates** and compressing it into one
sentence. Two things go wrong in the compression, and the second is worse than the first.

The first: "every gate-free candidate the panel built either enumerated widths or degraded the
diagnostic" makes those sound like two branches of a trade someone was offered. `137` does both, in two
unrelated places, and neither is traded against the other. Removing the bridge would not improve the
diagnostic and fixing the diagnostic does not touch the bridge. I know this because I fixed the
diagnostic (section 5) and the bridge is untouched.

The second, and this is the one that would have poisoned a consolidation: **it makes the container
derivation sound unsolved.** It is not. The derivation from a width to a container is total, gate-free,
uncapped and codegen-optimal, and op ratified the gate it satisfies at `137b:10-26`. What is unsolved
is one step upstream of it, the conversion of a written const literal into anything the type system can
compute with. Calling that "the container-derivation mechanism was never ratified" is accurate about
ratification and misleading about where the gap is, and a reader who acts on it will re-derive a ladder
that already exists.

**The honest sentence, offered to replace it:** the container derivation is built, total, gate-free and
erasing, and it is unratified because it rests on a per-width bridge from const literal to type that op
has refused, for which no alternative has been found. `SETTLED.md`'s open list at `SETTLED_container.md:458-462`
already says this correctly. The `SETTLED.md` summary of it does not.

---

## 5. Improving it, four ways, without moving the mechanism

Op's constraint is explicit: "It was pretty nice, no reason to try to force it or force anything else."
So none of what follows replaces anything. Same bridge, same ladder, same surface, same operations.
Every improvement is an attribute or a name.

`p12_improved_full.rs` is `137`'s `p7` with all of them applied. `p13_diag_showcase.rs` is the same file
with three deliberate errors, one per site class.

### 5.1 The law-relation diagnostic, which was the readable half of `SETTLED.md`'s complaint

**Control.** `p09a_lawsite_control.rs` is `p7` plus one wrong output width. Choosing 30, which is in the
table, so the bridge does not mask the law:

```
error[E0271]: type mismatch resolving `<Idx<30> as ToNat<Arvo>>::N == D0<D1<D0<D1<D1<Term>>>>>`
note: expected this to be `D0<D1<D0<D1<D1<Term>>>>>`
    = note: expected struct `D0<D1<D0<D1<D1<Term>>>>>`
               found struct `D0<D1<D1<D1<D1<Term>>>>>`
```

That is `137:563-567` reproduced. Three digit towers, no decimal number anywhere except the one the
consumer wrote.

**First idea, and it does not carry.** `p08a` and `p08b` establish that the tower prints because
`T13` is a **type alias** (`ladder.rs:446`, `pub type T13 = D1<D0<D1<D1<Term>>>>;`), which rustc
expands. Making the nat an opaque struct instead turns
`<Idx<16> as ToNat<Arvo>>::N == D1<D0<D1<D1<Term>>>>` into
`<Idx<16> as ToNat<Arvo>>::N == N13`, and deletes both tower notes.

I did not take it. At the law site one side of the equality is a **computed** sum, not a bridge output,
so it is a tower whatever the bridge's outputs are named. Making `Add` produce opaque nats would need a
total reverse map from towers to names, over an enumerated set, which is not total. The idea improves
one site class and cannot reach the one that matters. Recorded because the next person will have it too.

**What works.** Move the associated-type equality behind a named relation carrying its own message, and
put `#[diagnostic::do_not_recommend]` on the single blanket impl so rustc reports the relation instead
of drilling into its where-clause. `p09b_lawsite_named_relation.rs`, gate-free:

```
error[E0277]: width 30 is not the sum of widths 13 and 13
   |    ^^^ this output width does not follow from the input widths
   = note: the result of this operation is 13 + 13 bits wide; write that width, or let it be inferred
```

Not one digit appears in binary. Every number is a coordinate the consumer wrote.

`p09c_lawsite_named_clean.rs` is the same file with the mistake removed and it compiles, so **inference
through the relation is preserved**: rustc still solves `OI = 26` and `OF = 6` from the bound.

This makes `134`'s entire base-ten repair unnecessary for this site class. `137:544-571` had already
narrowed `133`'s "cost three" and found `134` addressed less than it thought; this narrows it to
nothing here.

### 5.2 The unshipped-width diagnostic, and a leak nobody has named

The control message for a width outside the table is worse than the record says, and the reason is not
the message.

```
error[E0277]: the trait bound `Idx<27>: ToNat<Arvo>` is not satisfied
    = help: the following other types implement trait `ToNat<M>`:
              Idx<0>  Idx<100>  Idx<13>  Idx<16>  Idx<200>  Idx<24>  Idx<26>  Idx<30>
            and 5 others
```

**rustc dumps arvo's shipped width table into the consumer's error output, sorted lexicographically.**
Zero, one hundred, thirteen, sixteen, two hundred. This is not a cosmetic complaint: it is the
enumeration becoming permanently visible at every consumer's build, in an order that reads as
corruption, and it appears in the record nowhere.

`#[diagnostic::on_unimplemented]` on the bridge trait fixes the message
(`p10_bridge_diag.rs`). It does not touch the dump.

`#[diagnostic::do_not_recommend]` on each bridge row does (`p11_suppress_table.rs`). Together:

```
error[E0277]: arvo does not ship this width: Idx<7>
   |    ^^^^^^^^^^^^^^^^ this numeral names a width arvo does not ship
   = note: widths are opt-in per program. Add `impl ToNat<MyWidths> for Idx<7>` and spell the
           numeral against `MyWidths`
note: required by a bound in `Fixed`
```

The width is named, the remedy is spelled with the width substituted in, and the table is gone. This is
the mechanism op adopted at `130b:69-80`, applied to the site `137:608-610` handed him as a choice
rather than built.

**Two consequences worth writing down.** The suppression is per-impl, so a consumer's own bridge rows
leak their own table unless they carry the attribute too, which `p13` shows happening for `Idx<777>`
and `Idx<41>`. That is arguably fine, since a consumer's own widths are information they have. And
`Fixed<7, 1, Hot>` produces **two** errors, one per coordinate, because both 7 and 1 are unshipped.
Minor noise, named so nobody rediscovers it.

### 5.3 The where-clause soup, which the record does not price at all

`137`'s `Fixed` carries four bounds on the struct itself:

```
Idx<I>: ToNat<M>, Idx<F>: ToNat<M>,
<Idx<I> as ToNat<M>>::N: Add<<Idx<F> as ToNat<M>>::N>,
Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>: Container,
```

Rust has no implied bounds on a struct's users, so **every generic site repeats all four**, and every
`impl` block in `p6` and `p7` does. `p7`'s `mul` carries twelve. That is a live cost against
`142c:329-333`, where op puts the ergonomics of writing these types above the plumbing, and it is not
mentioned in `137`, in `SETTLED_container.md`, or in `SETTLED.md`.

It collapses. Put the container behind one named trait on the marker, blanket-implemented where the
four hold, with the container as its associated type. `p16_single_bound.rs`, gate-free:

```
pub struct Fixed<const I: u32, const F: u32, S, M = Arvo> where M: Rep<I, F>
```

One bound on the struct. One bound at a generic site. The four survive in one place, the blanket impl,
where nobody reads them. `Rep` carries its own `on_unimplemented`, so a shape with no numeral says so
by name.

Layout assertions pass at 16, 64 and 200 bits, and the erasure survives: the object file has
`_native16 = _arvo16` and `_native64 = _arvo64` at shared addresses, two bodies and two aliases.

### 5.4 Codegen is untouched, which is the point

The improvements would be worthless if they cost an instruction. Emitting assembly from the control
(`reproduced/p7_law_site.rs`) and from the improved construction (`p12_improved_full.rs`) with the same
flags and diffing:

```
diff <(grep -v '^\s*\.file\|p7_law_site\|p12_improved' out/p07ctl.s) \
     <(grep -v ... out/p12.s)      ->  32 lines, all of them one anon symbol hash
```

Six bodies and 95 instructions on both sides. The only differences are the content-addressed hash of
the `todo!()` panic string, which changes because the filename changed. **The assembly is identical.**

---

## 6. Attacking the bridge, and every route that closed

The bridge is the one thing op refused, so it is the thing worth breaking. It asks a single question:
can a written const literal become something the type system computes with, without one impl per value?

Every route below is a compiled refusal on the pin. Six probes, `p01` through `p07`.

**Route 1, an associated const as an array length, no features.** `p01`. If an impl could compute a word
count from its own const parameters and use it as a length, no structural nat is needed at all.

```
error: generic `Self` types are currently not permitted in anonymous constants
   type C = [u64; Self::WORDS];
```

**Route 2, the same under `min_generic_const_args`.** `p02`. The feature is allowed by
`unstable-features.md` and is the deliberate successor to the forbidden `generic_const_exprs`. The error
changes, which is progress:

```
error: use of `const` in the type system not defined as `type const`
help: add `type` before `const` for `Pick::WORDS`
```

**Route 3, following rustc's own suggestion.** `p03`, declaring `type const WORDS: usize`:

```
error: complex const arguments must be placed inside of a `const` block
```

**Route 4, following the second suggestion.** `p04`, wrapping in `const { .. }`:

```
error: generic parameters may not be used in const operations
   type const WORDS: usize = const { ((I + F) as usize + 63) / 64 };
                                       ^
   = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

**`generic_const_args` is forbidden by the brief.** The compiler names the exact feature the design has
ruled out, which is the cleanest possible closure of a route.

**Route 5, a const argument computed from a const parameter in a where-clause.** `p05`, the recursive
bridge that would dissolve the enumeration by halving:

```
Idx<const { N / 2 }>: ToNat,
error: generic parameters may not be used in const operations
   = help: add `#![feature(generic_const_args)]`
```

Same wall, different position. Three independent instruments now agree: **a const generic parameter's
value cannot be operated on anywhere in the type system under the allowed feature set.**

**Route 6, the direction that is open, and why it does not help.** `p06` establishes that a `type const`
CAN be read through a generic type parameter in const-argument position:
`pub struct Holder<T: Sized2>(pub [u64; <T as Sized2>::N]);` compiles. That is type to const, and the
bridge needs const to type.

**Route 7, and this one matters for `137` rather than against it.** `133:410-417` reported that the
structural encoding cannot write `[u8; <B as Nat>::V]`, and `137` worked around it with a `#[repr(C)]`
word cons. `p07` retests that refusal under `min_generic_const_args` with `type const`, which `133` did
not have:

```
type const V: usize = const { 2 * <T as Nat>::V };
error: generic parameters may not be used in const operations
```

The RHS refuses a generic **type** parameter too, not only a const one. So `p06` works only because each
impl's RHS is a literal, which is an enumeration again.

**`133`'s refusal stands and `137`'s word cons is not a workaround, it is the only available
construction.** That strengthens the attempt, and it retires a route a future dispatch would otherwise
spend itself on.

---

## 7. Two findings about the bridge that change what op is being asked

Neither removes the enumeration. Both change its character, and neither is in the record.

### 7.1 The table is already lazy, which is what op asked for at `127b`

Op's objection to the width enumeration at `127b:36-50` was specific: "This is almost certainly doable
so that only used widths realise on const time."

`p14_bridge_laziness.rs` tests it directly. It adds a bridge row whose nat has **no** `Nat`, `Len`,
`Add` or `Container` impl at all, so the ladder cannot possibly run on it:

```rust
pub struct Unrealisable;
impl ToNat<Never> for Idx<9999> { type N = Unrealisable; }
```

The crate compiles. An unused row is registered and never realised, so the property op asked for is
already a property of `137`'s construction. What the table costs is parsing and registration, not
instantiation, and the ladder runs once per width a program actually writes.

That answers half of the `127b` objection. The half it does not answer is the half op restated at
`137b`, which is about a consumer having to write the line at all.

### 7.2 The table has no practical ceiling, and this is a residue, not a proposal

If arvo shipped a dense table the consumer-writes-a-line case would become rare rather than routine.
`p15_dense_bridge.rs` tests whether that is even expressible.

**Named accurately: this is an ad-hoc quick spike with no substance for magnitude.** No harness ran, so
the cost is **unpriced**. What it supports is an existence claim, which an ad-hoc spike may support.

| Rows | Result | wall (ad-hoc, unpriced) |
|---|---|---|
| 513 | compiles, size asserts pass at 16, 500, 512 bits | 0.10 s |
| 2049 | compiles | 0.61 s |
| 8193 | compiles | 3.11 s |

**Op has refused enumerated tables four separate times** (`SETTLED.md:110`, four instances cited). A
dense table is that refused shape at larger scale and **I am not proposing it.** It is recorded as a
residue because it bounds the problem: nothing about the mechanism breaks at eight thousand rows, so if
the refusal ever moves, the population question has no technical ceiling. If the refusal does not move,
this changes nothing and should not be quoted as though it did.

---

## 8. The concession: the bridge, and the routes I could not open

**I did not dissolve the enumeration, and I do not believe it is dissolvable under the allowed feature
set.** That belief rests on three compiled refusals in three different syntactic positions, all naming
the same forbidden feature as the escape.

Routes attacked and closed, in the order I tried them:

1. Associated const as an array length, ungated. Refused, anonymous constant.
2. The same under `min_generic_const_args`. Refused, wants `type const`.
3. `type const` with an expression RHS. Refused, wants a const block.
4. `type const` with a const block RHS. Refused, names `generic_const_args`.
5. Recursive bridge by halving the const in a where-clause. Refused, names `generic_const_args`.
6. `type const` RHS referencing a generic type parameter. Refused, same.
7. Opaque nats to shorten the law diagnostic. Compiles, but cannot reach the computed side of the
   equality, since a total reverse map from towers to names does not exist over an enumerated set.
8. A proc macro at the surface. Not attempted: refused at `139b:27-35`.
9. `generic_const_args` plus `-Znext-solver=globally`. Not attempted: forbidden, and refused
   specifically at `137b:43-45` as trading an enumeration for a feature and a flag.
10. A fixed-width carrier sized independently of the width. Not attempted: dead on structural grounds
    at `SETTLED_container.md:157-169`, since a fixed width is a ceiling and the ceiling was removed.
11. A container type parameter, defaulted or otherwise. Not attempted: refused at `130b:41-44`.
12. Type-level magnitude spelled at the alias site. Not attempted: refused at `142c:374-380` on the
    ergonomics bar.
13. A larger table. Expressible (section 7.2), and the refused shape. Not proposed.

**Where the wall is, stated as precisely as I can.** Rust admits a generic parameter into type position
only as a standalone argument of the item that parameterises it. A const's *value* is therefore
unreachable to the type system: it cannot be halved, compared, added, or matched. `min_generic_const_args`
lifts this for paths whose RHS is fully concrete, and for nothing else. So a mapping from an unbounded
set of literals to types is a function the language will not compute, and a table is the only object
that can stand in for it.

**What kind of help would move it.** Not another expert on this construction; three positions all
returned the same compiler sentence. It moves if the feature policy moves, if the surface's first
parameter stops being a const, or if `min_generic_const_args` gains generic RHS upstream. All three are
op's, and the first two are design calls rather than technical ones.

**A reframing offered as a read, not a proposal.** The record treats the bridge as a spelling problem
because op named it one, and he named it one from the ladder, where he was right. Measured, it is not a
spelling problem: it is the boundary between the value world and the type world, and the enumeration is
the only bridge Rust currently ships across it. That does not make op wrong to want it gone. It means
the thing to explore is whether the const has to be on the surface at all, and that is a question about
D48, which is his.

---

## 9. What I did not cover

- **The harness.** Nothing here is benched. Every timing in section 7.2 is an ad-hoc spike and is
  named as one. Compile-time cost of the bridge, of the ladder, and of the four improvements is
  **unpriced**.
- **x86-64.** Every assembly claim is aarch64, inheriting `137:645-647`'s own caveat.
- **`137`'s sections 1 through 4 on their own terms.** I recompiled its probes and reproduced its
  erasure result; I did not re-derive its instruction counts at 192, 256, 512 and 1024 bits, and I did
  not attack its chosen bar, which `137:627-635` explicitly asks a second read to attack. That
  obligation is still open and this file does not discharge it.
- **The strategy axis.** The ragged against word-rounded assignment adopted at `137b:47-53` is untouched
  here, and so is the reopened headroom question.
- **Multiply, divide and shift at the wide rung**, still unpriced since `137:642-644`.
- **Whether the improvements survive `Cold`'s bitpacked path**, which `137:651-653` also left as a
  source reading rather than a compiled result.

## 10. What is op's

Three things, and none of them is a technical question.

**Whether the bridge is acceptable.** He refused it at `137b`. Sections 6 and 8 say it cannot be removed
under the current feature policy and surface. So the live question is which of those two moves, and
both are his. Section 7.1 removes one of his stated objections, that unused widths cost something.

**Whether the four improvements are wanted.** They are attributes and one name. They cost nothing at
runtime, proved by identical assembly, and nothing in feature budget. They are in `p12_improved_full.rs`
and `p16_single_bound.rs`.

**Whether `SETTLED.md:149-153` gets corrected.** Its sentence makes a built mechanism read as an unbuilt
one, and section 4 offers a replacement sentence. That is a summary defect rather than a design
question, but a consolidation quoting it will carry the error forward.
