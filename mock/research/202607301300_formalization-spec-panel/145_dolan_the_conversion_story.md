# 145. The conversion story, and the narrowing that was deferred

**Date:** 2026-08-07
**Position:** after `144b_op_checkpoint_thirtyseven.md`. The second read `134b:240-256` owed on `131` section 5,
plus the lossy narrowing `131:625-628` left undesigned and `135:552-554` filed as blocked on this dispatch.
**Probes:** `145_probes/`, thirteen files with `run.sh` and the captured `output.txt`.

## Verdict, stated first

There is one relation, and it is inclusion of value sets. Every conversion between two numerals is the unique
map that inclusion determines, and conversions do not need a list of five names because the classification is
the two independent ways inclusion can fail: the target's grid may be coarser, and the target's range may be
narrower. Both failure modes are already named in the design, as the two halves of `Quantisation`
(`110:3451-3459`), so a narrowing conversion is quantisation with the operation set to the identity. It costs
no new vocabulary and it takes no new decision, because the four strategies' resolutions of it are the cells
of the ratified fixed-point preset table (`110:2704-2711`) read straight off.

`131`'s five do not survive as five. Two of them are not conversions, one is the reflexive case of another,
and the two that remain are subcases of a single order. What is missing is larger than what is wrong. The
biggest omission is that arvo already has a conversion family in the tree, keyed on the strategy rather than
on the numeral, which no panel file has mentioned and which the tests describe with the same word `131` uses
for something else.

Two compiled results decide the implicit-versus-written question and neither is what anyone expected.

**The exact embedding cannot be a `From` impl.** `131:598` recommends shipping the lossless conversion as
both a named `widen` and a `From`. A generic `From` between two numerals of one family conflicts with core's
`impl<T> From<T> for T`, and conditioning it on the computed order witness does not rescue it, because
coherence cannot evaluate the projection at free const parameters (`e1_from_overlap.rs`, `e3_embed_gca.rs`,
both `E0119`). So the one conversion that deserves to read as free cannot.

**The conversions that do read as free are the ones that should not.** The tree ships six `From` impls and four
`TryFrom` impls between strategies at a fixed numeral (`arvo/src/ufixed.rs:361-430`,
`arvo/src/ifixed.rs:381-450`). A strategy retag changes no value and changes the resolution of every
subsequent operation, and it fails to commute with same-format addition on 2,667 of 5,460 operand pairs per
ordered strategy pair at the checked bound (`q2_retag.rs`). It is expressible as `From` only because the
strategy axis has four concrete points while the numeral axis is unbounded, which is a fact about coherence
rather than a fact about the design.

Put together: **nothing in either axis qualifies for `From`, and the one that has it did not earn it.** The
mechanism that honours op's instinct is not a conversion at all. It is that the operations take both operand
numerals and both strategies and name the result, so the site where a consumer would have written a cast is a
site where they write an annotation. That compiles with no feature gate and no `-Znext-solver=globally`
(`h2_heterogeneous_gatefree.rs`, exit 0), which makes it the cheapest thing in this file and the only part
that needs to be right first.

## Contents

1. The gates
2. The premise check, and what the tree already ships
3. The order, derived rather than asserted
4. What relations exist, and the three that are not relations
5. The narrowing, designed
6. What is implicit and what is written
7. The diagnostic
8. The laws these relations need
9. What is op's, separately from what I decided
10. What I did not check

---

## 1. The gates

**The canon gate passes.** This is canon work under the panel's standing shape: intent that survives an
implementation rewrite, checked against `.claude/rules/the-canon-is-intent-not-implementation.md`. Nothing
here proposes source. `mock/crates` was read at length and not written. The probes carry the viability
evidence and are not the design; every one of them is a spike whose names, arities and field orders are
scaffolding chosen to reach a check.

I checked the erasure gate at `135b` binds on what I propose: the consumer writes `UFixed<13, 3, Warm>` and
`add(a, b)` with an annotation, the typestate derives the container and the resolution, it validates at the
signature or at the named-item law, and it erases. Nothing in section 4 through 7 asks a consumer to name a
container, a tag, or a witness.

**The test gate.** The whole suite, not filtered: `cargo test --workspace` in `arvo/mock` gives **694 passed,
0 failed, 9 ignored across 171 targets**, on the pinned toolchain. 417 `#[test]` items across 81 integration
files plus lib units. Green, and green is the weakest signal in the room.

I read the bodies rather than the names in the surface this dispatch touches, which is
`arvo/tests/cross_width.rs`, the only test file in the tree about conversion. Fourteen tests. Five assert the
`Resolve` table, at five of its sixteen cells. Nine assert the strategy conversions. **Three of the nine are
built on a datum that is not a value**, and they are the three that carry the narrowing claim:

```rust
// arvo/tests/cross_width.rs:81-89
fn ufixed_try_from_warm_to_hot_out_of_range_fails() {
    type UHot = UFixed<{ ibits(8) }, { FBits::ZERO }, Hot>;
    type UWarm = UFixed<{ ibits(8) }, { FBits::ZERO }, Warm>;
    // 300 doesn't fit in 8 logical bits (max 255).
    let a = UWarm::from_raw(300);
    let narrowed: Result<UHot, ()> = a.try_into();
    assert!(narrowed.is_err());
}
```

`UFixed<8, 0, Warm>`'s value set is `{0..255}`. 300 is not in it. The test reaches the failing branch by
constructing a datum outside the value set through `from_raw` (`arvo/src/ufixed.rs:188-192`, public, no
check), then asserts that converting it fails. That is `110:1122-1130`'s precondition failure exactly: under a
hole, the statement is not false, it is ill-typed, and rustc's own suggested repair is the unchecked coercion
the design performs silently. Three tests, `ufixed_try_from_warm_to_hot_out_of_range_fails`,
`ufixed_try_from_precise_to_hot_out_of_range_fails` and `ifixed_try_from_warm_to_hot_out_of_range_fails`, plus
the two `*_try_narrow_outcome_err` tests on the bridge directly, all establish behaviour on inputs the numeral
cannot hold.

This is not a tautological test and I am not asking for it to be deleted. It is **setup that reaches a path
the design does not have**, which is the mirror of setup that helps: the input is chosen so the failing branch
is entered, and the failing branch exists only because the perimeter is open. On values the map is total, so
the whole `TryFrom` family is fallible for a reason that is not in the design.

**And there is no test anywhere for a conversion that changes the numeral.** The file's own header says so:
"Cross-width arithmetic is deferred (see ufixed.rs / ifixed.rs TODOs)". The subject of this dispatch has zero
coverage, which is honest, and it means nothing in the suite constrains what section 5 proposes.

I did not refuse the assigned work over this. The suite is not fabricated and the count is not inflated; what
it covers, it covers. The finding is that its conversion half describes an axis nobody in this panel knew was
there, and asserts its sharpest cases outside the value set.

---

## 2. The premise check, and what the tree already ships

The brief's cheap claims first, since three panel files have now been sent out on a false one.

**"Op withdrew the requirement at `130b:11-30`."** Holds. `130b:13-14`: "At `127b:22-31` op ratified that two
numerals of equal precision must be the same type. That requirement is withdrawn." The quotation the brief
reproduces matches `130b:16-23` word for word.

**"`131` section 5 is the one read that exists and has never been second-read."** Holds. `conversion` returns
one hit in `133`, four in `134`, three in `134b`, three in `135` and two in `138`, and every one of them is a
reference to `131`'s five rather than an examination of them. `134b:240-256` schedules this dispatch;
`135:414-418` and `135:552-554` both record the conversion laws as blocked on it. Nobody has looked.

**"`131:625-628` leaves narrowing undesigned."** Holds, verbatim.

**One claim in `131` does not survive, and it is about `131`'s own probe.** `131:598` says `widen` is "total
and value-preserving in both directions of reasoning". Its probe declares

```rust
// 131_probes/cv1_conversion.rs
impl<const I: u32, const F: u32, const J: u32, G, S> Widen<Fixed<J, F, G, S>> for Fixed<I, F, G, S>
```

with no relation between `I` and `J` at all. `x1_cv1_gap.rs` reproduces that impl shape and asks for
`UFixed<13, 3> as Widen<UFixed<8, 3>>`, thirteen integer digits into eight. Exit 0. The impl admits the
narrowing as a widening, so the losslessness the file asserts is asserted and not compiled. Cited for what it
proved, which is the `E0308` text at `131:617-623`, and not for how it was written.

### What the tree already ships, which no panel file has mentioned

`arvo` has a conversion family today. It is keyed on the **strategy**, at a fixed numeral, and it is complete
in one direction and absent in another:

| From | To | Shape | Site |
|---|---|---|---|
| `UFixed<I, F, Hot>` | `UFixed<I, F, Warm>` | `From` | `arvo/src/ufixed.rs:361` |
| `UFixed<I, F, Hot>` | `UFixed<I, F, Precise>` | `From` | `arvo/src/ufixed.rs:374` |
| `UFixed<I, F, Warm>` | `UFixed<I, F, Precise>` | `From` | `arvo/src/ufixed.rs:385` |
| `UFixed<I, F, Warm>` | `UFixed<I, F, Hot>` | `TryFrom` | `arvo/src/ufixed.rs:396` |
| `UFixed<I, F, Precise>` | `UFixed<I, F, Hot>` | `TryFrom` | `arvo/src/ufixed.rs:414` |

and the same five for `IFixed` at `ifixed.rs:381-450`. Underneath sit four bridge traits, `UWidenFrom`,
`IWidenFrom`, `UNarrowFrom`, `INarrowFrom` (`arvo-strategy/src/widen.rs:27-65`).

Three things about that family matter to this dispatch.

**It uses "widen" and "narrow" for the container, not for the numeral.** `widen.rs:1-13` says so: "produce
`Self::T` (the Dst container) from a `Src::T`". `Warm => Precise` widen is spelled `u16 => u16`
(`widen.rs:196`), the identity, because the two strategies share a ladder rung. So arvo's shipped `widen` and
`131`'s proposed `widen` are different operations on different axes wearing one word, and a consumer meeting
both would have no way to tell.

**Its narrow checks the value against the numeral's own range**, `(1 << N) - 1` for unsigned
(`widen.rs:115-116`) and `[-(1 << (N-1)), (1 << (N-1)) - 1]` for signed (`widen.rs:137-139`). Since source and
target share `N`, that check can only fire on a datum outside the shared value set, which is what section 1
found the tests feeding it.

**`Cold` is in none of it.** `widen.rs:318-328` defers it in a comment. `Cold` returns zero hits in
`arvo/src/ifixed.rs` and appears in `ufixed.rs` not at all. Meanwhile `Resolve` is total over the four
strategies and has six impls naming `Cold` (`arvo-strategy/src/lib.rs:230-244`). **The resolution table
promises a join the coercion family cannot perform on half its edges.** That is a hole in shipped code rather
than in the design, and it is worth recording because section 6 concludes the coercion family should not exist
in that shape at all, which resolves the hole by removing the thing that has it.

**And the widths are enumerated.** `impl_u_widen!(Hot => Warm, u8 => u16, 1, 2, 3, 4, 5, 6, 7, 8)` and
fourteen more lines like it (`widen.rs:158-300`). That is the same per-width enumeration op refused at
`127b:36-50` and the panel spent files removing from the container projection, still standing in the
conversion family because nobody looked at the conversion family.

---

## 3. The order, derived rather than asserted

Before naming any conversion I want the structure the conversions live in, because if the structure is right
the names fall out of it and if it is wrong no list of names will save it.

A numeral, in the design's own terms (`110:1472-1479` for the closure conditions, `110:3427-3433` for the
quantiser), determines two things a conversion has to respect: a **grid**, the lattice of values it can
represent, and a **range**, the interval of that lattice it admits. For an unbiased dyadic fixed-point numeral
these are

$$V(I, F) \;=\; \{\, k \cdot 2^{-F} \;:\; 0 \le k < 2^{I+F} \,\}$$

unsigned, and with $-2^{I+F} \le k < 2^{I+F}$ signed, so the grid is the multiples of the quantum $2^{-F}$ and
the range is $[0, 2^I - 2^{-F}]$.

Conversion is then a question about set inclusion, and inclusion factors into exactly two independent
conditions: is the target's grid at least as fine, and is the target's range at least as wide. For dyadic
fixed point those are $F_1 \le F_2$ and $I_1 \le I_2$.

**Claim (the order).** $V(I_1, F_1) \subseteq V(I_2, F_2)$ if and only if $I_1 \le I_2$ and $F_1 \le F_2$.

That is the componentwise order on the pair of coordinates, and it is the whole of the subtyping question for
numerals. Checked exhaustively rather than argued, over every ordered pair of shapes with $I + F \le 8$, both
signs, by enumerating value sets as integers over a common denominator (`o1_order.rs`):

```
pairs checked          2025
O failures (unsigned)  0
S failures (signed)    0
M failures (meet)      0
J failures (join)      0
families checked       9
A failures (antichain) 0
incomparable pairs     1080
  of which join strictly exceeds the union: 1080
```

Three further facts come out of the same sweep and each earns its place.

**The numerals form a lattice, and the meet is exact.** The meet is the componentwise minimum and the join the
componentwise maximum. $V$ preserves meets exactly, $V(A \wedge B) = V(A) \cap V(B)$, checked at every pair
with zero failures. It does not preserve joins, and cannot: the union of two value sets is not a value set,
so $V(A \vee B)$ is the least numeral set containing both and is strictly larger than the union at all 1,080
incomparable pairs. Leastness was checked against every other shape in the matrix rather than assumed.

That asymmetry is not a curiosity. The join with a carry digit is the sum numeral, which is why heterogeneous
addition has a clean numeral-level map, and section 4 uses it.

**Every equal-precision family is an antichain.** For a fixed precision $P$, the set $\{(I, F) : I + F = P\}$
has $P+1$ members and no two of them are comparable: if $I_1 < I_2$ then $F_1 > F_2$, so neither includes the
other. Zero violations across nine families.

**This is the proof of op's instinct, and it is stronger than the two arguments already in the record.**
`130:170-193` argues from an ambiguous decode, which is a consumer's mistake. `131` section 7 argues from a
false multiplicative law, which is the library's mistake and is sharper. The order argument is sharper again,
because it does not need a mistake at all: Q13.3 and Q8.8 are **maximally unrelated** among numerals of
precision sixteen. They are the two ends of an antichain. There is no conversion arrow between them in either
direction, and the withdrawn requirement proposed to identify precisely the pairs that have none.

Op's words at `130b:21-23` were "I can't think of a use case where the flipped fraction to i place would be
meaningfully considered type-equal and have that mean something". The structure agrees, and gives the reason:
equal precision and related-by-conversion are disjoint except at identity. Keying the numeral on precision
collapsed an antichain of size $P+1$ to a point.

**The honest restatement.** `131:570-573` proposes: two numerals of equal precision have the same `Precision`,
`Precision` is a type, and they are not the same numeral, because they are not the same number. I agree with
it and would add one clause, because the sentence as written says what they are not and a consumer will ask
what they are:

> Two numerals of equal precision have the same `Precision`, and `Precision` is a type. They are not the same
> numeral, and neither converts into the other exactly: the equal-precision family is an antichain in the
> order, and its members are the numerals furthest from each other at that precision rather than the nearest.

The added clause is the part that tells a consumer what to do next, which is that a map between two such
numerals exists, is a quantisation, and is written.

### What the order is not

It is stated for the unbiased dyadic family, which is what the fixed-point aliases are. The general numeral
carries a bias and an adjustment (`110:1472-1479`, `110:1649-1651`), and inclusion there is the condition that
the source's lattice is a subgroup of the target's coset and the source's range fits. I have not compiled the
general form and section 10 records it. What the general form cannot do is change the shape of the answer:
inclusion still factors into a grid condition and a range condition, because a value set is still a subset of
a lattice intersected with an interval. The two-halves classification in section 4 is therefore not specific
to dyadic even though its arithmetic here is.

`136:383-387` names the pattern this is at risk of: "a predicate over numerals is keyed on every numeral
parameter or it is keyed on the ones that existed when it was written". The order predicate as I compiled it
is keyed on `I` and `F` and is therefore keyed on the ones that exist today. Stating the general condition and
specialising it, rather than stating the special case, is the whole of the repair, and it is the third
instance of that pattern after `AddClosed` and `tag_one_representable`.

---

## 4. What relations exist, and the three that are not relations

`131` names five. My answer is that there are **two maps and one order**, that three of `131`'s five are not
conversions, and that two things it does not name are needed.

### The two maps

**`embed`, the exact map.** Defined exactly where the order holds. Total on the source's value set, injective,
value-preserving, and it raises no event. For dyadic fixed point it is a left shift of the raw datum by
$F_2 - F_1$, which is free when $F_2 = F_1$ and one instruction otherwise.

**`quantise`, the total map.** Defined for every ordered pair of numerals. It takes the exact value, lands it
on the target's grid using the target strategy's in-range direction, then classifies it against the target's
range using that strategy's out-of-range resolution. Section 5 is its design.

That is the whole surface, and the reason it is two rather than one is a law rather than a taste. `quantise`
subsumes `embed` pointwise: restricted to the region where the order holds they agree, checked over every such
pair and every value at all four strategies, 8,464 checks and zero failures (`q1_quantise.rs`, C2). So why
keep two names?

Because **an exact step before a lossy one changes nothing, and a lossy step before a lossy one changes the
answer.** Both compiled:

```
C3 checked 236992 failures 0          embed then quantise  ==  quantise
C4 checked 2411584 disagreements 800157   quantise then quantise != quantise
```

C3 is coherence: no consumer can get a different answer by routing through a wider intermediate, so an
implementation is free to insert or elide an embedding anywhere. C4 is double rounding, which is `110:1318-1323`
arriving in the conversion chapter, and it disagrees on 33.2 percent of the triples checked, distributed
216,042 for `Hot`, 217,310 each for `Warm` and `Cold`, and 149,495 for `Precise` (`Precise` fewer because a
refusal absorbs).

So the two names encode the one thing a consumer needs to know about composing conversions: **`embed` is free
to compose and `quantise` is not.** Collapsing them into one operation would make the difference invisible,
which is the same defect as the one op withdrew the canonicity over.

### The order itself

Not a map, and worth naming separately because it is what the type system checks. `A <= B` is the proposition
that `embed` exists. It is the subtyping relation, it is decidable from four const parameters, and it is what
the diagnostic in section 7 reports on.

### `131`'s five, judged

**One, identity.** Not a relation of its own. It is the reflexive case of the order, and `embed` at `A = A` is
the identity map. Counting it separately costs a row and buys nothing, and it actively misleads on the one
place it matters: core already supplies `impl<T> From<T> for T`, so the reflexive case is the one case a
design must **exclude** from any impl it writes rather than include (section 6).

**Two, inferred at the operation.** Not a conversion. `131:592-594` says so in its own text ("there is nothing
to convert, the operation produced that type") and then numbers it as relation two anyway. A relation is
between two types and there is no second type here; there is one operation whose output coordinates are
parameters. That is a property of the law, already recorded in `135`'s key column as "the result numeral for a
widening operation" (`135:81-83`).

Deleting it from the conversion list is not deleting the observation, which is the most important one in
`131` section 5. It is relocating it, and section 6 puts it where it belongs, which is first rather than
second, because it is the answer to op's question and the other four are the residue after it has done its
work.

**Three, `widen`.** Survives, as one arm of `embed`. `131` defines it as "same exponent, more integer digits",
which is the $F_1 = F_2$, $I_1 < I_2$ arm. Growing $F$ is equally exact and equally total: the grid refines
and every source value is a target value. Both arms are one relation and there is no reason to name only one
of them; the design would otherwise have an exact conversion with a name and an exact conversion without one.
The recommendation to also ship it as `From` is refuted in section 6.

**Four, `rescale`.** Does not survive, and its flagship example is misdescribed. `131:602-612` defines it as
"the exponent changes, which multiplies or divides by a power of two and can drop digits off the bottom", and
gives `UFixed<13, 3> -> UFixed<8, 8>` as the case. That pair drops nothing off the bottom: the grid **refines**
from $2^{-3}$ to $2^{-8}$, which is exact, and what fails is the range, from $[0, 8191.875]$ to
$[0, 255.996]$. It is a pure range event and `131` reads it as a pure grid event. Splitting a conversion by
"the exponent changed" cuts across the two things that actually matter, which is why the classification has to
be by which inclusion fails rather than by which coordinate moved.

**Five, refused.** Not a relation. It is what a consumer gets by there being no impl, which in Rust is the
default rather than a design act. The design's work is deciding where **not** to write an impl, and the
diagnostic quality of the refusal is section 7.

### The two `131` does not name

**Heterogeneous operands.** `131:239` shows `add(a, b)` over two operands of one numeral. If the operation
takes two numerals, the antichain pair needs no conversion to be added, multiplied, or folded, because the
operation's own numeral-level map absorbs the difference. This is not speculative: `h1_heterogeneous.rs`
compiles `add` and `mul` over two independent numerals and two independent strategies, with the sum numeral as
the join plus a carry and the product numeral as the coordinatewise sum, and `h2_heterogeneous_gatefree.rs` is
the same file with the feature gates deleted, also exit 0.

```rust
pub fn mixed_add(a: UFixed<13, 3, Warm>, b: UFixed<8, 8, Warm>) {
    let _s: UFixed<14, 8, Warm> = add(a, b);
}
pub fn mixed_strategy(a: UFixed<13, 3, Hot>, b: UFixed<8, 8, Precise>) {
    let _s: UFixed<14, 8, Precise> = add(a, b);
}
```

Neither line writes a conversion, and the second resolves the strategy to the join through `Resolve`. The
`addnum` map is $(\max(I_1,I_2) + 1, \max(F_1,F_2))$, which is the lattice join of section 3 plus one carry
digit, and it is the reason the join was worth establishing. The negative control refuses a wrong annotation
through the named-item law op adopted at `130b:69-80`, with six numbers in the law's own order
(`h3_negative.rs`):

```
error[E0080]: evaluation panicked: add: the result's fraction digits must be the join of the operands'
   evaluation of `SumFormat::<13, 3, 8, 8, 14, 3>::HOLDS` failed here
note: the above error was encountered while instantiating `fn add::<13, 3, 8, 8, 14, 3, Unsigned, Warm, Warm>`
  --> h3_negative.rs:167:35
```

**The strategy retag.** A conversion at a fixed numeral that changes only `S`. The value set is identical, the
map is the identity on values, and the tree ships ten of them (section 2). `131` does not mention it, which
means its five relations and arvo's shipped five conversions are disjoint sets that share two words.

### The relation set, stated

| Name | Defined when | Total | Exact | Composes freely | Written or free |
|---|---|---|---|---|---|
| the order `A <= B` | a proposition, always decidable | n/a | n/a | transitive | checked, never written |
| `embed` | `A <= B` | yes | yes | yes (C3) | written |
| `quantise` | always | yes for `Hot`/`Warm`/`Cold`, fallible for `Precise` | only on the embedding region | **no** (C4) | written |
| `retag` | always, at a fixed numeral | yes | yes on values | yes | written (section 6) |

Four rows, and the first is not a map. `131` had five rows of which three were not maps.

---

## 5. The narrowing, designed

`131:625-628` sets it aside on the ground that it "is lossy in a way the strategy has to adjudicate (wrap for
Hot, saturate for Precise), so it is not a conversion at all; it is an operation carrying a policy, and it
belongs with the arithmetic rather than with the conversions."

The premise is right and the conclusion inverts it. That a strategy adjudicates the loss is not a reason to
move the operation elsewhere. It is the whole design, already built, sitting one section away in the same
document, and the reason nobody used it is that it was written for arithmetic results and a conversion is the
same situation with a different operation in front of it. The parenthesis is also wrong on both cells it
names: `Precise` does not saturate, it refuses (`110:2707`), and saturating is `Warm` and `Cold`.

### The design, in one sentence

**A narrowing conversion is the quantiser applied to the exact value, with the operation set to the identity,
resolved by the target's strategy.**

`110:3427-3433` defines the quantiser as one map over five situations, "because rounding and overflow are not
two axes but the in-range and out-of-range halves of one map from an exact value onto the representable set".
The five situations are relative to the representable set: strictly between two neighbours below their
midpoint, on the midpoint, strictly between them above the midpoint, past the top, past the bottom
(`110:3431-3433`).

A conversion presents an exact value, since a value of the source numeral is exact by construction, and asks
for the target's representable set. Five situations, the same five, and `Quantisation`'s five associated types
answer all of them:

$$\text{narrow}_{A \to B, S}(v) \;=\; \text{classify}_{B,S} \circ \text{round}_{B,S} (v)$$

where `round` uses `UnderMidpoint` / `OnMidpoint` / `OverMidpoint` and `classify` uses `OverRange` /
`UnderRange`. Nothing else. No new marker, no new axis, no new decision.

Two consequences follow with no further argument.

**The order of the two steps is round-then-classify, and it is not a choice.** Q4 and Q5 (`110:1261-1266`)
state that the overflow band is the set of exact values that lie in range and round to a point outside it,
which is a statement about a value that has already been rounded. So the narrowing inherits Q4 and Q5 without
restatement, and an implementation that classifies before rounding is not implementing this map.

**The quantiser reads the value, not the datum.** That is what makes C3 hold: `embed` then `quantise` equals
`quantise` because both read the same exact value, and `embed` changed no value. An implementation that
shifted the raw digits first and quantised the shifted digits would break C3 and the consumer would get
different answers depending on whether an intermediate format was named. C3 is therefore the law an
implementation is most likely to break, which is why section 8 writes it down.

### What each strategy does, and why that follows from what it is

Read straight off the ratified fixed-point table (`110:2704-2711`). Every cell below is that table's cell; the
column of reasons is the table's own derivation applied to a conversion rather than to an arithmetic result.

| | in-range | out-of-range | so a narrowing | because |
|---|---|---|---|---|
| `Hot` | `TowardNegative` | `ReduceModulo` | truncates toward negative infinity and wraps | an arithmetic right shift is the truncation and a mask is the wrap. Both are free, which is what "as fast as possible" means |
| `Warm` | `ToEven` | clamp | rounds nearest, ties to even, and clamps | "behaves as f32 and f64 would with no framework on top" (`110:2677-2679`), and that is what a float narrowing does |
| `Cold` | `ToEven` | clamp | rounds nearest, ties to even, and clamps | identical to `Warm` here. `Cold` differs on `StoredWidth` and `Layout` only, and the cold-path half of its intent is what pays for the rounding (`110:2717-2722`) |
| `Precise` | `ToEven` | `Refuse` | rounds nearest, ties to even, and returns through the fallibility projection | "a hardware instruction is unconditional and infallible by construction and `Precise`'s identity requires a refusing branch" (`110:2714-2716`) |

Compiled as a model and checked, all four strategies, every ordered shape pair at $I + F \le 6$, every value
of the source (`q1_quantise.rs`):

```
shapes 28 strategies 4
C1 checked 3076 failures 0        quantise A to A is the identity
C2 checked 8464 failures 0        on the embedding region, quantise == embed
C3 checked 236992 failures 0      embed then quantise == quantise
C5 checked 2081968 monotonicity failures per strategy:
   Hot 72778 | Warm 0 | Cold 0 | Precise 0
C4 checked 2411584 disagreements 800157   quantise then quantise != quantise
```

C5 is the one result in that block that is not an equality, and it is the sharpest. **`Hot`'s narrowing is not
monotone**, because `ReduceModulo` is not order-preserving, while the other three are because rounding,
clamping and refusing all are. By M7 (`110:1657-1662`), which says distributivity over the lattice operations
holds exactly when the operation is monotone, a narrowing under `Hot` therefore does not distribute over `min`
and `max`. That is a new refutation and section 8 files it as N5, next to N1, which says the same thing about
wrapping addition for the same reason.

**The load-bearing observation is that `Warm` and `Cold` are the same map.** They differ nowhere in the
quantiser for fixed point, so the narrowing has three behaviours over four strategies, and that is the ratified
table's own shape rather than something this design introduced. `q2_retag.rs`'s matrix confirms it
independently from the other direction: the `Warm` to `Cold` retag is the only off-diagonal pair with zero
non-commuting cases.

### The one cell the tables do not settle, and it is a conversion-specific question

`Precise` refuses out of range and rounds in range. For an arithmetic result that is clearly right: refusing
every inexact multiply would make the preset unusable, since almost every product is inexact at the format
width. For a **conversion** the reading is less obvious, because the consumer explicitly asked to move, and
"the most precise" plausibly means "tell me when this loses a digit" rather than only "tell me when it leaves
the range".

The grade machinery can express it. `inexact` is one of the five clause-7 generators (`110:2106-2107`) and the
quantiser already raises it, so a resolution keyed on the event rather than on the situation is stateable.

**My recommendation is that it should not be a variant of the same operation, on one-rule grounds.** The
narrowing is quantisation with the operation set to the identity, and if `Precise`'s narrowing refused on
inexactness it would no longer be that map, so the uniformity that gives C1 through C3 for free would be
bought back at the price of a special case for one strategy. What a consumer who wants exactness instead wants
is a different question, "is this value representable in that numeral", which is the order applied to a value
rather than to a type, and it deserves its own name and its own return type rather than a strategy-keyed
variation on a conversion.

So: one narrowing, four strategies, three behaviours, and a separate exactness test for the consumer who needs
one. I am one expert and this is exactly the shape of call the record shows getting overturned, so section 9
carries it to op rather than deciding it.

### Signedness, and the half nobody mentions

`UnderRange` is a column in the table and a narrowing of a signed numeral fires it. Narrowing
`IFixed<12, 3>` to `IFixed<8, 3>` loses range at **both** ends, so `Hot` wraps in both directions, `Warm` and
`Cold` clamp toward `TowardPositive` at the bottom and `TowardNegative` at the top (which is what `110:2707`'s
"clamp (`TowardNegative`/`TowardPositive`)" spells), and `Precise` refuses on both. The unsigned case fires
`UnderRange` only where the source is signed and the target is not, which is a sign-domain change and is a
different conversion from either of the two this section designs. I have not designed the sign-domain change
and section 10 records it.

### What it costs

Stated as a shape rather than as a measurement, because I have no harness run and will not call a compile a
bench.

`Hot`'s narrowing is a shift and a mask, both unconditional, no branch. `Warm` and `Cold` add a
compare-and-increment for the tie rule and a compare-and-select for the clamp, which is the same cost
`110:2741-2743` already records `Cold` paying on every store and gives the same reason: a stored value that
wraps silently is worse than a slower store. `Precise` adds a branch that leaves through
`Quantisation::Fallibility`, so its narrowing composes with `?` at the consumer and the other three do not
need to.

The one real cost question is where the rounding intermediate lives when the grid coarsens by more than the
container has spare bits, and that is `StoredWidth`'s job rather than the conversion's: `Warm` and `Precise`
carry doubled storage for fixed point precisely so a chain retains more than one operation's exactness before
a narrow forces a decision (`110:2716-2717`). A narrowing is that decision, named.

---

## 6. What is implicit and what is written

Op's instinct, verbatim at `130b:33-35`: downstream consumers will rely on implicit castability through the
typestate, because writing the typestate explicitly at every site is too verbose.

The instinct is right about the problem and reaches for a mechanism Rust does not have. That is worth saying
plainly rather than working around, because the design's actual answer is better than the mechanism he was
reaching for and a reader who thinks a coercion is being provided will misread the whole chapter.

### There is no implicit conversion, so the question is which conversions read as free

Rust has no user-extensible implicit coercion. `Deref` and unsizing are the only automatic ones and neither
applies. So a design has exactly three registers:

1. **Nothing to convert.** The operation produced the type, and the consumer wrote an annotation.
2. **Reads as free.** `From` and `Into`, spelled `.into()`, and reachable without the consumer naming the
   target at all: through a generic bound `T: Into<U>`, and through `?` on an error type.
3. **Reads as an operation.** A named method or function.

Register 2 is what "implicit" can mean here, and it is where the design's decision actually sits.

### Register one is the answer, and it is free

Op's site-by-site verbosity is dissolved, not paid, by the operations taking their output coordinates as
parameters and their operand numerals independently. A consumer writes

```rust
let p: UFixed<26, 6, Warm> = mul(a, b);
let s: UFixed<14, 8, Warm> = add(a, b);      // a is Q13.3, b is Q8.8
let t: UFixed<14, 8, Precise> = add(c, d);   // c is Hot, d is Precise
```

and converts nothing anywhere. `131:583-594` found this and called it relation two; it is not a relation, it
is the mechanism that makes the conversion chapter small. It compiles gate-free
(`h2_heterogeneous_gatefree.rs`, exit 0, no `-Znext-solver=globally`), which matters because the container
projection does not, so **the part of the answer op most cares about is also the part that costs nothing.**

The residue after register one has done its work is one situation: a consumer storing a value into a format
they named, where the format is not one the operation would have produced. That is the whole conversion
chapter, and it is a site where a consumer has explicitly chosen a format, which is the site where writing
something is least objectionable.

### Register two: the criterion, and both refutations

The criterion for a conversion reading as free is four conditions, and the fourth is the one nobody stated:

1. **Total.** Defined on every value of the source.
2. **Value-preserving.** The denotation is unchanged.
3. **Coherent under composition.** Any two routes from A to B give the same map. `embed` satisfies this by
   being the identity on rationals restricted, and inclusions compose to inclusions.
4. **Commuting with the operations.** $c(\mathrm{op}_A(x, y)) = \mathrm{op}_B(c\,x, c\,y)$ wherever both sides
   are defined. Without this, whether the consumer converted before or after changes the answer, and a
   conversion that reads as free must never change an answer.

`embed` satisfies all four. The strategy retag satisfies the first three and fails the fourth. And the
language admits exactly the wrong one of them.

**`embed` cannot be a `From` impl.** A generic `From` between two numerals of one family conflicts with core's
identity impl, because rustc cannot know the two numerals differ (`e1_from_overlap.rs`):

```
error[E0119]: conflicting implementations of trait `From<Fixed<_, _, _, _>>` for type `Fixed<_, _, _, _>`
   = note: conflicting implementation in crate `core`:
           - impl<T> From<T> for T;
```

Conditioning it does not rescue it. A witness with a single enumerable impl does rescue coherence
(`e2_from_conditioned.rs`, exit 0), which is what makes the next result a real finding rather than an
artifact: the same impl conditioned on the **computed** order witness, in the Pattern C shape the container
projection uses, gets the identical `E0119` (`e3_embed_gca.rs`). Coherence cannot evaluate
`<Pair<I, F, I, F> as Tagged>::TAG` at free const parameters, so it cannot see that the bound fails at the
reflexive pair. Under GCA, with `-Znext-solver=globally`, on the pinned toolchain.

The escapes are all worse than the thing they escape. Enumerating `From` impls per width pair is
$O(W^4)$ and is the enumeration op refused at `127b:36-50`. And there is one that works and should be refused
on design grounds rather than on mechanism ones: an impl that moves the numeral **and** names two distinct
concrete strategies has no reflexive overlap and compiles (`e4_routes_after_refusal.rs`, route B, exit 0):

```rust
pub fn route_b(a: UFixed<13, 3, Hot>) -> UFixed<20, 8, Warm> {
    a.into()   // implicit, and only because the strategy moved too
}
```

So `Q13.3 Hot` into `Q20.8 Warm` could read as free while `Q13.3 Warm` into `Q20.8 Warm` could not. That
asymmetry has no semantic content whatsoever; it is a shadow of the coherence rule falling across the API.
Shipping it would be an ad hoc restriction wearing an ergonomic face, and the consumer who hit the second case
after using the first would have no way to understand why.

**The strategy retag should not be a `From` impl.** It is one today, six times per family
(`ufixed.rs:361-395`, `ifixed.rs:381-415`). It changes no value. It changes the resolution of every subsequent
operation, and therefore fails condition four:

```
checked 87360 non-commuting 26670
first counterexample: width 1, Hot -> Warm, a=1 b=1: retag(op_S) Raw(0), op_T(retag) Raw(1)
per ordered pair (rows = source, cols = target), Hot Warm Cold Precise:
  Hot       0   2667   2667   2667
  Warm    2667      0      0   2667
  Cold    2667      0      0   2667
  Precise    2667   2667   2667      0
```

(`q2_retag.rs`, same-format addition, every ordered strategy pair, widths one through six, every pair of
values.) 2,667 of 5,460 operand pairs per off-diagonal cell, which is 48.8 percent. The `Warm` to `Cold` zeros
are the two strategies sharing both quantiser rows, as section 5 noted.

There is one serious counter-argument and it deserves to be answered rather than waved at. Under `Resolve`
(`arvo-strategy/src/lib.rs:210-244`) a mixed-strategy binary operation resolves to the more conservative
strategy, so the coercion of the lower operand into the join is a designed step and its non-commutation is the
resolution rule working. True, and it does not license the public `From`, for two reasons. The operation does
not need one: `h1_heterogeneous.rs`'s `add` takes both strategies and names `<SA as Resolve<SB>>::Out` as its
output, so the join is reached at the signature with no conversion anywhere. And a public `From` is reachable
where no resolution is happening, through `T: Into<U>` and through `?`, which is exactly the site where a
consumer moves a value up the chain and keeps operating under semantics they did not choose.

### The ruling I would put to op

**No `From` impl between arvo numerals, on either axis.** `.into()` on an arvo numeral means what core means
by it and nothing more. `embed` is a named method because it cannot be anything else, `quantise` is a named
method because it is lossy, and `retag` is a named method because what it drops is a promise rather than a
digit. The current `From` and `TryFrom` families become `retag` and, since the `TryFrom` family is fallible
only through the perimeter hole section 1 found, `retag` is total in both directions and the `TryFrom` half
disappears.

That also resolves the `Cold` hole for free. `widen.rs:318-328` defers `Cold` from the widen lattice while
`Resolve` promises joins through it, and the resolution is not to write the eight missing impls, it is that a
retag at a fixed numeral is the identity on values and needs no per-pair bridge at all.

**The cost of this ruling to a consumer is one word per site**, `a.retag()` against `a.into()`, and the sites
are rare because register one covers the arithmetic. The benefit is that no arvo value ever changes its
overflow policy without a word saying so.

---

## 7. The diagnostic

The design's bar, from `130b:69-80` and `131:614-623`, is that a refusal names both coordinates. Three shapes
are available and they are not equally good.

**Plain assignment**, which is what a consumer writes by accident, gives what `131:617-623` reports
(`e5_refused.rs`, third error):

```
error[E0308]: mismatched types
82 | pub fn refused_c(a: UFixed<13, 3, Warm>) -> UFixed<8, 8, Warm> {
   |                                             ------------------ expected `Fixed<8, 8, Unsigned, Warm>` because of return type
83 |     a
   |     ^ expected `8`, found `13`
   = note: expected struct `Fixed<8, 8, _, _>`
              found struct `Fixed<13, 3, _, _>`
```

Both coordinates, at the consumer's line, and it says nothing about what to do. It is rustc's default and the
design cannot improve it, because there is no arvo item involved.

**A refused `embed`** is where the design can say something, because the refusal goes through an arvo trait
that can carry `#[diagnostic::on_unimplemented]`. The antichain pair, both directions (`e5_refused.rs`):

```
error[E0277]: this numeral does not embed into that one
  --> e5_refused.rs:74:7
   |
74 |     a.embed()
   |       ^^^^^ no exact embedding here
   |
   = note: an embedding needs the target's integer digits and fraction digits to be both at least the
           source's. Where either shrinks, the conversion is lossy and is spelled `quantise`, whose
           resolution the strategy names.
help: the trait `EmbedWitness<1>` is not implemented for `Picker`
      but trait `EmbedWitness<0>` is implemented for it
note: required for `Fixed<13, 3, Unsigned, Warm>` to implement `Embed<Fixed<8, 8, Unsigned, Warm>>`
```

The custom text lands at the call site, the `required for` note names both numerals in full, and the note
points at the remedy. One blemish: `EmbedWitness<1>` against `EmbedWitness<0>` is internal machinery, and the
integers mean nothing to a consumer. It is the same class of leak `131:537-540` flags for the GCA equality
message, and unlike that one it is ours to fix.

**Routing the tag through a two-row decision table removes it**, at a cost of one trait and two impls
(`e6_named_verdict.rs`):

```
help: the trait `EmbedWitness<DoesNotEmbed>` is not implemented for `Picker`
      but trait `EmbedWitness<Embeds>` is implemented for it
   = help: for that trait implementation, expected `Embeds`, found `DoesNotEmbed`
```

Same Pattern C shape as everything else, no new mechanism, and every line of the message now reads. Small,
situational, cheap to keep, and it is the kind of thing that is never done later.

**What the design should not do is refuse `quantise`.** `quantise` is total: every ordered pair of numerals
has one, and the target strategy says what happens to what does not fit. A consumer who writes a lossy
conversion gets a value, not an error, and the loss is the strategy's declared behaviour rather than a
surprise, except under `Precise` where it is a refusal through the fallibility projection. That is the whole
ergonomic case for the two-map design: the exact map refuses loudly and the total map never refuses at all, so
a consumer is never stuck.

---

## 8. The laws these relations need

Written to `135`'s seven-column standard (`135:67-95`). The family letter is **X**, which is unused by `135`'s
nine and `136`'s three. Fifteen rows: thirteen for the conversions, one addition-family row the heterogeneous
operation needs, and one refutation that belongs in `135` section 3.4 rather than here.

**The key schema needs no extension, and that is a result rather than a convenience.** `110:1454-1458`
enumerates a law's key as the operation marker, the operand numerals, the result numeral for a widening
operation, the `Quantisation` resolutions, the `Direction`, and for a fold the accumulator and arity. A
conversion's key is the identity operation marker, the source numeral, the target numeral, the target
strategy's five resolutions and its in-range `Direction`. Every column already exists. If the narrowing were a
new mechanism it would have wanted a new key column, and it does not, which is the cheapest available
evidence that it is the quantiser rather than something beside it.

### The order

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| X1 | `V(N1)` is a subset of `V(N2)` exactly when the target's grid is at least as fine and its range at least as wide, which for dyadic fixed point is `I1 <= I2` and `F1 <= F2` | pairs of numerals | identity | `N1`, `N2`, bias, adjustment, radix, sign domain | asserted, compiled exhaustively over all 2,025 ordered shape pairs at `I + F <= 8`, both signs, zero failures (`o1_order.rs`) | **nowhere** |
| X2 | The numerals form a lattice under X1, with meet the componentwise minimum and join the componentwise maximum, and `V` preserves meets exactly while strictly overshooting joins | pairs of numerals | identity | as X1 | asserted, compiled: meet exact at every pair, join least among all shapes, strict at all 1,080 incomparable pairs (`o1_order.rs`) | **nowhere** |
| X3 | Every equal-precision family is an antichain of size `P + 1` | numerals of one precision | identity | precision `P` | **derived from X1**, compiled at nine families, zero violations | **nowhere**, and it is the structural form of the withdrawal at `130b:11-30` |

### The two maps

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| X4 | `embed` exists exactly when X1 holds, and is total, injective and the identity on values | the value set of the source | identity, and it raises no event | source numeral, target numeral | asserted, compiled as the oracle for X5 | **nowhere**; `131:596-600` asserts one arm of it and its probe admits the opposite direction (`x1_cv1_gap.rs`) |
| X5 | Restricted to the region where X1 holds, `quantise` equals `embed` | the value set of the source, over embedding pairs | identity | source, target, resolutions, `Direction` | asserted, compiled: 8,464 checks over all embedding pairs, four strategies, zero failures (`q1_quantise.rs` C2) | **nowhere**, and it is the row that says there is one relation rather than two |
| X6 | `quantise` after `embed` equals `quantise` directly, for every `A <= B` and every `C` | triples of numerals, over the source's value set | identity | the three numerals, resolutions, `Direction` | asserted, compiled: 236,992 checks, four strategies, zero failures (`q1_quantise.rs` C3) | **nowhere**, and it is the law an implementation breaks by quantising the datum instead of the value |
| X7 | `quantise` after `quantise` does **not** equal `quantise` directly | triples of numerals | none; it fails at the weak equation level | as X6 | **REFUTED**, compiled: 800,157 disagreements in 2,411,584 triples, 33.2 percent, distributed 216,042 / 217,310 / 217,310 / 149,495 over the four strategies (`q1_quantise.rs` C4) | it is Q6's double rounding (`110:1318-1323`) reaching the conversion chapter, stated for arithmetic and never for conversions |
| X8 | `quantise` from a numeral to itself is the identity | the value set | identity | the numeral, resolutions | **derived from X5** at `B = A`, compiled: 3,076 checks, zero failures (`q1_quantise.rs` C1) | **nowhere** |
| X9 | `quantise` back after `embed` is the identity | the value set of the source | identity | source, target, resolutions | **derived from X6 and X8**, no separate witness owed | `135:552` names it as one of the two obvious conversion laws. It is a corollary rather than an axiom, and its converse fails because `quantise` is not injective |
| X10 | `quantise` is monotone exactly when the out-of-range resolution is not `ReduceModulo` | ordered pairs in the source's value set | identity | source, target, `OverRange`, `UnderRange`, `Direction` | asserted, compiled: 2,081,968 ordered pairs, zero failures for `Warm`, `Cold` and `Precise`, **72,778 failures for `Hot`** (`q1_quantise.rs` C5) | **nowhere** |

X10 is the row with the longest reach. M7 (`110:1657-1662`) says distributivity over the lattice operations
holds exactly when the operation is monotone, and O6 (`136:458`) is the row that supplies M7 the lattice. So
X10 plus M7 gives a refutation immediately, and it belongs in `135` section 3.4 with the other four rather
than here:

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| N5 | A narrowing under `Hot` does not distribute over the lattice operations | pairs | none | `Hot`, `ReduceModulo` | **derived from X10 and M7**, and it is the conversion-side twin of N1 | **nowhere** |

N1 says wrapping addition does not distribute; N5 says wrapping narrowing does not either, for the same reason
and by the same biconditional. That is one more axiom `Hot` fails, and it strengthens rather than changes N4's
conclusion that no preset is a dioid.

### The strategy axis

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| X11 | A retag at a fixed numeral preserves every value and raises no event | the value set | identity | the numeral, the two strategies | asserted; and the shipped `TryFrom` family contradicts it by being fallible, for a reason section 1 traces to the open perimeter rather than to the design | **nowhere**; the impls are at `arvo/src/ufixed.rs:396-430` and `ifixed.rs:416-450` |
| X12 | A retag does **not** commute with the operations | pairs of in-range operands | none | the numeral, the two strategies, both resolution sets | **REFUTED**, compiled: 26,670 non-commuting cases in 87,360, which is 2,667 of 5,460 per off-diagonal ordered pair, and exactly zero for `Warm` against `Cold` (`q2_retag.rs`) | **nowhere**, and it is the whole case for writing the retag rather than letting it read as free |
| X13 | The strategy order is the chain `Hot < Warm < Cold < Precise` and `Resolve` is its join | pairs of strategies | identity | the strategy pair | asserted; the table ships at `arvo-strategy/src/lib.rs:223-244` and the chain property is asserted nowhere. `arvo/tests/cross_width.rs:18-41` samples five of its sixteen cells | the ordering ground is `110:2941-2943` and the table is source only |

X13's status line is the sampled-law shape `135:266` warns about, in the tree rather than in the design: five
cells of sixteen, chosen, and the eleven not chosen include every cell that would establish associativity or
idempotence of the join. A four-element chain's join is sixteen cells and asserting all of them is one loop.

### The addition family

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| A3 | The exact sum numeral of two numerals is their join plus one carry digit, `addnum((I1,F1),(I2,F2)) = (max(I1,I2) + 1, max(F1,F2))` | pairs of numerals | identity | `N1`, `N2` | asserted, compiled as a named-item law with its negative control (`h1_heterogeneous.rs`, `h3_negative.rs`) | **nowhere**; A1 and A2 (`135:139-140`) are stated over one numeral and the heterogeneous case has no row |

A3 belongs in `135` section 3.2 rather than in this family, and it is the addition counterpart of M3
(`135:153`), which already gives the product numeral over two numerals. The asymmetry is why nobody noticed:
multiplication's numeral map was written because the product numeral is obviously not the operand numeral,
and addition's was not because it looks like it might be.

### What the conversion laws inherit and what they do not

**They inherit Q4 and Q5 without restatement**, because the narrowing classifies the rounded value rather than
the exact one, which is the situation Q4's overflow band is defined over (`110:1261-1266`).

**They inherit the `Lowering` charter's first clause.** `110:1062-1065`: "`Lowering` changes no value.
`Encoding`, nested inside it, may change which datum carries a value." A conversion is a value-level operation
and may not read an `Encoding`, which the structural enforcement at `110:1067-1071` already delivers by making
the name fail to resolve. So the crossing contract's C1 through C3 and the conversion family are independent,
and neither needs a clause about the other.

**Four of them do not transfer from a model width, under one family.** `135:214-219` identifies the four rows
whose key contains `EMIN` or `EMAX` and which therefore cannot carry a model-width check to a real width by
the uniformity argument. For the dyadic fixed-point family my keys contain neither, so X1 through X10 transfer.
For a `Ranged` numeral the range endpoints **are** `EMIN` and `EMAX`, so **the same rows over a `Ranged`
numeral join that set**, taking it from four to fourteen. That is an extension of `135`'s single most useful
finding and it is not a comfortable one, because the conversions between a float format and a fixed-point one
are exactly what a consumer reaches for.

---

## 9. What is op's, separately from what I decided

**Mine, and compiled.** That the numeral order is componentwise on `(I, F)` and that this is the whole
subtyping question for the dyadic family, exhaustively over the shape matrix at both signs. That the numerals
form a lattice whose meet `V` preserves exactly and whose join it strictly overshoots at every incomparable
pair. That every equal-precision family is an antichain, which is the structural reason the withdrawn
requirement was wrong and is a stronger reason than either of the two in the record. That a generic `From`
between two numerals of one family is refused by coherence against core's identity impl, that conditioning it
on a computed witness does not rescue it, and that a witness with one enumerable impl does, so the refusal is
about the projection rather than about where clauses. That `131:598`'s recommendation to ship the embedding as
a `From` is therefore not available. That the one route which does compile requires the strategy to move as
well, producing an asymmetry with no semantic content. That the narrowing is the quantiser with the identity
operation, needing no new marker, no new axis and no new key column, with the four strategies' rows read off
the ratified table. That `quantise` agrees with `embed` on the embedding region, that an exact step before a
lossy one changes nothing, and that a lossy step before a lossy one changes the answer a third of the time.
That `quantise` is monotone for three strategies and not for `Hot`, which gives N5 through M7. That a strategy
retag preserves every value and fails to commute with the operations on 48.8 percent of operand pairs per
off-diagonal cell, and that `Warm` against `Cold` is the sole exception because they share both quantiser
rows. That the heterogeneous operation compiles gate-free, so the mechanism that answers op's instinct costs
nothing. That `131`'s `rescale` example is a range event described as a grid event. That `131`'s own
conversion probe admits a narrowing as a widening.

**His, because it is his own withdrawn ratification.** Whether section 3's restatement is what he meant. I
agree with `131:570-573` and would extend it, because the sentence as written says what the two numerals are
not and a consumer's next question is what they are:

> Two numerals of equal precision have the same `Precision`, and `Precision` is a type. They are not the same
> numeral, and neither converts into the other exactly: the equal-precision family is an antichain in the
> order, and its members are the numerals furthest from each other at that precision rather than the nearest.

**His, because it changes what `.into()` means across the whole surface.** Whether arvo ships **no** `From`
impl between numerals on either axis, and the six `From` plus four `TryFrom` per family in the tree become a
named `retag`. My reading is that it should, on X12, and that the cost is one word per site at sites that are
rare because the operations absorb the arithmetic. Against it: the `Resolve` reading, under which the upward
retag is the resolution rule's own coercion. I answered that in section 6 and the answer is a reading rather
than a proof, because whether a public `From` may exist for an internal rule's benefit is a taste question
about API surface and not a theorem.

**His, and it is the narrowest live call in this file.** Whether `Precise`'s narrowing refuses on `inexact` or
only out of range. The ratified table says out of range, and the table was derived for arithmetic results
where refusing every inexact product would make the preset unusable. A conversion is a site where the consumer
asked, so the reading could go the other way. My recommendation is that it stays out of range only, and that a
consumer wanting exactness gets a separate operation with its own name and its own return type, on the ground
that one rule beats a per-strategy variant. Under `143b`'s framing this may be an arm question rather than a
fork: the ratified row is one arm, and a second arm keyed on the event rather than the situation is writable
later without disturbing it.

**His, because it touches ratified structure.** Whether the general numeral's order condition is what I say
its shape is. I compiled the unbiased dyadic case. The general case is that the source's lattice is a subgroup
of the target's coset and the source's range fits, and the classification into a grid half and a range half
survives because a value set is a lattice intersected with an interval regardless of bias. That reasoning is
not compiled and section 10 says so. It is his because `136:383-387` names this exact failure pattern twice
already, at `AddClosed` and at `tag_one_representable`, and a third instance in a chapter being written now
would be the design repeating a mistake it has already diagnosed.

**Owed under the two-expert rule.** I am the second read on `131` section 5 and I disagree with it on four of
its five rows, on my own compiled work. I am the **first** read on everything in sections 3, 5, 8 and on the
`From` refusal, and none of it should enter the canon on one expert's word. A second read should attack the
premise I have taken for granted, which is **that a conversion is a map between value sets at all.** If the
design's numeral is really about data rather than values, then a conversion is a map between datum sets, the
crossing contract's C1 through C3 govern it rather than the quantiser, and my whole classification is on the
wrong side of `110:1530-1534`'s distinction. I believe it is not, because `110:1530-1534` says the law's noun
is the value set and the crossing contract is explicitly the one family where it is not (`135:188-190`), but
believing is not checking and the shipped `from_raw` perimeter means the tree currently behaves as though the
answer were the other one.

---

## 10. What I did not check

- **The general numeral's order.** Compiled for unbiased dyadic fixed point only. Bias, adjustment, and radices
  other than two are argued and not run. This is the single largest gap in the file and it is the one
  `136:383-387`'s pattern predicts someone will inherit.
- **The float and decimal families.** Every claim here is stated over a fixed-point numeral. A conversion
  between a `Ranged` numeral and a fixed-point one is what a consumer reaches for first and it has no row in
  section 8. The `EMIN`/`EMAX` transfer note is the shape of the problem rather than its answer.
- **The sign-domain change.** Converting `IFixed` to `UFixed` or back fires `UnderRange` in a way neither
  section 5 nor the preset table discusses, because the source's negative half has no image at all rather than
  an out-of-range one. Not designed.
- **Whether the retag ruling breaks anything downstream.** `hilavitkutin` and `vehje` consume arvo, and
  removing ten `From` impls per family is a source change I did not survey for call sites, because
  `mock/crates` is out of bounds for writing and I did not read the consumers at all.
- **The cost of the narrowing, as a number.** Section 5 states shapes and refuses to call a compile a bench.
  The question that would need the harness is whether `Warm`'s clamp costs what `110:2741-2743` records `Cold`
  paying on every store, and whether the tie rule folds into the same select.
- **Whether `Embed` and `Quantise` as separate traits cost anything in the bound-propagation tax `131:365-374`
  measured.** My probes are one crate. A width-generic consumer calling `embed` inherits the witness bound the
  same way it inherits `Store`, and I did not build the consumer arm.
- **Whether the `E0119` refusal has an upstream escape.** Negative reasoning, or coherence learning to
  evaluate a `type const` at free parameters, would both reopen the `From` question. I did not search for an
  issue, and if one is close the ruling in section 6 changes shape.
- **The `Resolve` table's own laws.** X13 is asserted and I did not compile associativity, commutativity or
  idempotence of the join over the sixteen cells, because it is one loop and it belongs with whoever writes
  the strategy chapter rather than in a conversion file.
- **Whether the shipped `TryFrom` family has a consumer that depends on its fallibility.** I established the
  fallibility is an artifact of `from_raw`. Whether some consumer uses it deliberately as a value check is a
  call-site question I did not ask.
