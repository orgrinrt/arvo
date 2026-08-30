# Panel 05: fallibility, and what a refusal costs the code around it

**Persona:** Daan Leijen, effects and resource-discipline lens. Fifth member; read
`01_knuth_mathematical_rigour.md`, `02_kiselyov_type_level_encoding.md`,
`03_jhala_what_is_provable.md` and `04_torvalds_does_it_earn_its_keep.md` in full, plus every probe
under `02_probes/`, before starting.
**Date:** 2026-07-30

**What I read in full:** the spec (`202607301200_topic.the-formalization-spec.md`), panel files 01
through 04, all fourteen probes under `02_probes/`, the panel brief, the governing panel rule, notko's
`consttry_const_path.rs` / `just_consttry_const.rs` / `outcome_consttry_const.rs` / `just.rs`, and
`arvo-graph/src/{lib,rank,spanning}.rs`. **What I read in part:** the talk and the inherited-state file
at the passages the prior members cite; `arvo-comb/src/binpack.rs`, `arvo-spectral/src/{fiedler,
operator,laplacian}.rs`, `arvo-graph/tests/rank.rs`, `arvo-numeric-contracts/src/lib.rs`,
`arvo-bitmask/src/matrix.rs`.

**Directory listing done** across `mock/design_rounds/` (94 entries, three flat files at root are this
round, nothing newer), `mock/research/` (nothing postdates the panel directory), and
`mock/research/sketches/` (nineteen entries). Nothing supersedes the spec.

**Gates.** I re-ran the suite rather than inheriting a count: 654 passed, 0 failed, 122 binaries,
matching 01, 02 and 04. I read test bodies on the surface my lens touches; `arvo-graph/tests/rank.rs`
asserts concrete rank values on named DAG shapes with `UFixed<8,0,Hot>` weights, which is a real
contract test and is the exact shape probe E below reuses. No tautological tests found. I confirmed
that the two `#![feature(generic_const_exprs)]` gates 02 and 03 and 04 all flag are still present.

**Separation of evidence.** Sections marked *verified* were compiled and run under
`nightly-2026-05-28`, or read at a `file:line`. Probes are committed alongside this file at
`05_probes/`. Sections marked *reasoned* are argument. Impressions are labelled. I carry more than one
reading wherever the evidence does not force one, and I rule on nothing.

---

## 0. One inherited claim I disprove by compiling, before anything else

The thread I was dispatched on rests partly on 02's section 7 conclusion, stated in its own words:
"**arvo cannot have one generic `add` over all compositions.** Either the operation splits into a
total path and a fallible path selected by the composition, which duplicates every arithmetic body, or
the recovery constructor moves onto the resolution itself."

The first half of that sentence is false and the second half is the reason it is false. 02 named the
repair and then wrote the consequence as if the repair did not exist. `05_probes/a_handler.rs`
compiles and runs, one body, both instantiations, no `ConstFromResidual` bound anywhere:

```rust
pub trait OverRangeRule {
    type Carrier<T: Copy>: ConstTry<Output = T>;
    fn over<T: Copy>(max: T) -> Self::Carrier<T>;
}

fn add<R: OverRangeRule>(a: u16, b: u16, max: u16) -> R::Carrier<u16> {
    match a.checked_add(b) {
        Some(v) if v <= max => <R::Carrier<u16> as ConstTry>::from_output(v),
        _ => R::over(max),
    }
}
```

```
A: total=1000 refusing_err=true refusing_ok=true
```

The reason `f2_refusal.rs` hit a wall is not that the two paths cannot share a body. It is that
`f2_refusal.rs`'s body **constructs the refusal itself**, which forces the bound
`Q::Fallibility<T>: ConstFromResidual<Outcome<Infallible, OutOfRange>>` onto the carrier, and `Just<T>`
correctly refuses it. Move the construction to the resolution and the body never names a refusal
constructor, so the bound never arises. 02's own probe proves the bound is unsatisfiable for `Just`;
it does not prove that a body needs the bound.

This matters beyond the correction, because it is the shape the rest of my file rests on: **a
resolution is a handler, the carrier is the evidence that the handler can return where it wants to
return, and an operation performs an effect by calling the handler rather than by branching on which
handler it has.** That is not an analogy. It is the same translation, and the reason it costs nothing
here is the same reason it costs nothing in an evidence-passing effect compiler: the handler is
selected statically, so the call is a known target and inlines.

## 1. Two positions, and the carrier becomes a row. Verified.

Probe A has one range position. The spec has two (`OverRange`, `UnderRange`), and they can differ, so
the composition's carrier has to be the join of the two handlers' carriers, and a handler returning
into the smaller carrier has to embed into the larger one. `05_probes/b_carrier_join.rs` builds that
and runs:

```
B: sat=100 precise_err=true mixed_hi_err=true mixed_lo=true
```

The mixed row is the one that decides it: `OverRange = Refuse`, `UnderRange = TowardPositive`. The
composition carries `Outcome`, refuses above, and returns `Ok(clamped)` below. Neither handler had to
know about the other; the join is computed by a type-level `Or` over `CanRefuse` and the embedding is
a `Lift` that is the identity wherever the carriers coincide.

That is row subsumption, and it is worth naming as such because it tells you where the design goes
when a second effect arrives. `Refuse` is one operation in a row of one. If underflow later wants its
own refusal type, or if a division adds a `DivideByZero`, the carrier is the join over a row of
several, the `Or` becomes a union, and the lift becomes an injection. The shape scales; the
enumeration `Just`-or-`Outcome` does not.

**The honest cost, stated with the feature.** The `where` clause on that one function runs to five
lines:

```rust
where
    <Q::Over as RangeRule>::CanRefuse: Or<<Q::Under as RangeRule>::CanRefuse>,
    <<Q::Over as RangeRule>::CanRefuse as Or<<Q::Under as RangeRule>::CanRefuse>>::Out: CarrierOf<T>,
    <Q::Over as RangeRule>::Carrier<T>: Lift<Answer<Q, T>>,
    <Q::Under as RangeRule>::Carrier<T>: Lift<Answer<Q, T>>,
```

04's section 6 impression about maintainer-debugging cost lands squarely here, and I would upgrade it
from impression to measurement: every arithmetic function in `arvo-numeric` inherits some version of
that clause, and a trait-solver failure inside it is what op reads at three in the morning. Three
readings:

Compute the join once behind a blanket extension trait, so `Answer<Q, T>` is a single associated type
on `Quantisation` and every arithmetic body bounds on `Q: QuantisationExt`. The clause lands in one
place instead of at every function. I did not compile this variant and say so.

Declare the carrier, as the spec does at `spec:156`, and accept 01's finding 6 that the declaration can
lie. Cheapest to read, and 03's section 1 already tells us what the type system buys either way: with
the join computed, a new resolution cannot be added without answering; with the carrier declared, it
can.

Take the position that two range positions never disagree in practice, seal the pair to the four
combinations anyone ships, and enumerate. That is the option nobody has stated, and it is not silly:
the mixed case exists mathematically and I could not name a consumer who wants it. If it is
enumerable, the `Or` and the `Lift` both disappear.

## 2. Delivery is a `Lowering` member, and the spec's own sorting test says so. Reasoned, and it is the load-bearing claim in this file.

Here is the reframe I was dispatched to look for.

The spec's D54 test (`spec:32-36`) sorts an axis by asking what changing it changes. "Change it, and
ask whether the set of representable values changed. If it did, the axis is identity. If the same
values are still representable and only the arithmetic differs, it is policy. If neither changed and
only the cost did, it is lowering."

Now hold the policy fixed at `Refuse` and vary only how the refusal arrives:

- as a sum type: `Outcome<T, OutOfRange>`, checked at every step, short-circuiting;
- as an accumulated flag: the value plus a sticky bit, checked once at the end;
- as an absorbing bottom inside the numeral's own spare patterns, propagating by absorption, settled
  once at the end.

Run D54 on that variation. The representable set is unchanged: `Refuse` produces no new value in
either case, and the bottom is not a number, exactly as a NaN is not. The arithmetic is unchanged: the
mathematical function from a pair of inputs to `T ∪ {⊥}` is identical in all three, which is what
makes the deliveries interchangeable rather than merely comparable. What differs is the cost, the
register footprint, the control flow, and the shape of the call site. **By the spec's own test, that
is a `Lowering` member.**

I state that carefully because it is a strong claim and the spec has no such axis. If it holds, three
things follow immediately.

First, the presets table (`spec:250-257`) is missing a column, and `Precise` gains a choice it does not
have today. `Precise` keeps refusing, which is its stated intent and op's call, and a consumer picks
whether the refusal arrives eagerly as control or lazily as data.

Second, the law derivation does not change. All three deliveries are the same ⊥-extension of the
partial operation, so 01's Kleene-equality analysis (its finding 1's `Refuse` rows and its finding 3's
table) applies verbatim to each. I verified this rather than asserting it: see section 4.

Third, and this is the part that would settle 04's section 3, the total-delivery instantiation
satisfies `core::ops::Add<Output = Self>`, so it enters the L2 and L3 crates unchanged. See section 3.

**Two readings I hold against my own claim, because the evidence does not force one.**

The counter-reading that worries me most: a bottom carried in the value is only observationally
identical to a refusal **if the value under bottom cannot be read as an answer**. The moment a
consumer can extract "the clamped value, and also a bit saying it was clamped", the delivery has
produced an answer that `Refuse` never produces, and it is a different policy, not a different
lowering. So the sorting depends on an access discipline: `settle()` or an equivalent must be the only
door out. That is expressible (probe E keeps the raw accessor behind the newtype) and it is a real
obligation on the design rather than a free consequence.

The second reading is more interesting and is a gap independent of my proposal. **"Resolve and also
report" is a mode the spec's vocabulary cannot express at all.** The five-position quantisation models
a resolution as a function from an exact value to a representable one. Every real arithmetic unit is a
function *plus flags*: an ALU sets carry, overflow and sticky bits alongside the result, SystemC's
`sc_fixed` exposes overflow observability, MATLAB's `fi` logs overflow events, and IEEE 754's default
mode returns a value *and* raises a flag. If `ClampAndFlag` is a legitimate fifth `OverRange`
resolution rather than a delivery of `Refuse`, then it is policy, it sits between clamping and
refusing, and the vocabulary needs a slot for a resolution whose codomain is a pair. I do not know
which of the two framings is right and I do not think the panel should decide it from the armchair;
what I am confident of is that a design that can express neither is missing something the field
universally ships.

## 3. The exile is an artifact of the delivery, and I can run the counterexample. Verified.

04's section 3 established that `Precise` returning through `Outcome` fails
`W: Add<Output = W> + TotalOrd + Copy + FromConstant` at `arvo-graph/src/rank.rs:39`, and listed three
ways to land it: accept the exile, give `Precise` a panicking total spelling, or bifurcate the
algorithm crates. I want to first widen the finding and then add a fourth option that 04's three do not
cover.

**Widening it, verified.** The bound is not one site. `Add<Output = W>` appears at
`arvo-graph/src/rank.rs:39` and `:100`, `arvo-graph/src/path.rs:36`, `arvo-comb/src/dp.rs:41`,
`arvo-comb/src/binpack.rs:44`, `arvo-spectral/src/{laplacian.rs:34, operator.rs:49,:135,:171,
fiedler.rs:66,:162}`. Spectral additionally bounds `Mul`, `Sub`, `Recip` and `Sqrt`
(`fiedler.rs:31`, `operator.rs:171`), and under a refusing policy `Recip` at zero and `Sqrt` of a
negative are *also* refusals, so the fallible surface at L3 is four operations wide, not one. 04's
"roughly doubling the generic surface of four crates" is if anything an underestimate.

**The fourth option, verified by running it.** `05_probes/e_refusing_through_graph.rs` holds the policy
fixed at refuse (no clamped answer is ever produced, and the type has no door to one except `settle`)
and changes only the delivery, to an absorbing bottom carried in the numeral's spare patterns. It calls
`arvo_graph::upward_rank` **unmodified**:

```
size_of Refusing = 1, size_of W = 1
size_of Outcome<W, OutOfRange> = 2
clean chain ranks: [4, 3, 2, 1]
heavy chain refused per node: [true, true, false, false]
root settles to err: true
sink settles to ok:  true
```

The heavy row is the demonstration. Four nodes in a chain, weight 80 each, logical maximum 200. Ranks
80 and 160 are fine; 240 and 320 are not, and the two nodes whose rank leaves the range come back
refused while the two below it come back with their values. The algorithm crate did not know any of
this happened. It called `Add` and `total_cmp` and the absorption did the rest.

So the option 04's list does not contain: **keep the refusal, change how it travels, and the exile
disappears with no change to the algorithm crates, no panic, no bifurcation, and no growth in the
intermediate's size.** The refusal becomes control flow exactly once, at `settle()`, which is where a
consumer wanted to look anyway.

**The hazard I found while building it, which is the honest price and which is not small.** An
absorbing element must absorb under **every** operation the algorithm performs, and algorithms select
as well as add. In probe E I sorted bottom above every value, so `upward_rank`'s max-selection
propagates it. That is a design obligation on `TotalOrd`, not a free consequence, and a single total
order cannot serve both directions: `arvo-comb/src/binpack.rs:101-103` performs a fit test,

```rust
let after = *used.get(USize(b)) + w;
if !matches!(after.total_cmp(capacity), Ordering::Greater) { ... }
```

and a bottom sorting high compares `Greater`, so the bin is silently skipped, the item lands elsewhere,
and the refusal never reaches the output. That is precisely the defect IEEE 754-2008 shipped in
`minNum` and `maxNum`, which return the non-NaN operand and therefore discard the very thing they were
supposed to propagate, and which 754-2019 replaced with `minimum` and `maximum` that propagate. The
field has already found this bug and already published the fix, which is the strongest argument for
adopting the delivery with eyes open rather than for avoiding it: the correction is known, it is that
selection must be a propagating operation rather than a consequence of a total order, and it means
`TotalOrd` is the wrong contract for algorithm crates that must not lose a bottom.

Two readings on what to do with that. Either the algorithm crates gain a propagating `min`/`max` from
a trait rather than deriving them from `total_cmp`, which is one new contract and touches every
selection site. Or the bottom-carrying delivery is declared incompatible with selection-based
algorithms and the exile is accepted for exactly those, which is narrower than 04's blanket exile and
is at least a sentence that names the boundary. I lean to the first because the second reintroduces the
problem it was meant to solve, but the first has a real cost and I have not counted the sites.

## 4. The law derivation is unchanged across the deliveries, and I compiled 03's mechanism to show it. Verified.

03's section 3 proposed bounded compile-time falsification as the cheap substitute for a solver, and
said plainly: "Sketched, not verified (I did not compile this, and I say so plainly)."
`05_probes/f_const_falsification.rs` compiles six of them and they run at const-eval time on every
build.

Four check 01's translation-stability identity, `phi(phi(x) + c) == phi(x + c)`, exhaustively over a
3-bit unsigned and a 4-bit signed representable set:

```rust
const _: () = assert!(CLAMP_U,  "unsigned clamping should be translation-stable");
const _: () = assert!(!ZERO_U,  "01 finding 1: SubstituteZero is NOT stable");
const _: () = assert!(MOD_U,    "modular reduction should be translation-stable");
const _: () = assert!(!CLAMP_S, "signed clamping is NOT stable");
```

All four hold. **01's finding 1 is now reproduced mechanically rather than by hand arithmetic**, and I
checked that the check bites by flipping one polarity and rebuilding:

```
error[E0080]: evaluation panicked: 01 finding 1: SubstituteZero is NOT stable
  --> src/lib.rs:67:15
```

Two more check that the ⊥-extension is exactly the Kleene semantics 01 analysed, by brute force over
the whole cube:

```rust
const _: () = assert!(ASSOC_BOT_U,  "bottom-extended unsigned addition is a semigroup");
const _: () = assert!(!ASSOC_BOT_S, "bottom-extended signed addition is NOT");
```

Both hold, which is what licenses section 2's claim that the delivery does not move the mathematics:
the absorbing-bottom carrier has exactly the associativity the refusing sum type has, unsigned yes,
signed no, and 01's `(127 + 1) + (-1)` counterexample is the witness in both.

**Two findings for 03's mechanism itself, from having built it.** First, `const fn` cannot call through
a `fn` pointer ("function pointer calls are not allowed in constant functions"), so the oracle cannot be
a parameter and the one-statement-per-check shape has to be a macro. That is a small constraint and
worth writing into the proposal before someone rediscovers it. Second, and more usefully, the check is
cheap enough that the width-uniformity argument 03 wants may not always be needed: the signed case at
4 bits is 17 cubed iterations and const-eval did not blink. Running the algebraic laws at two or three
widths costs nothing and is strictly more evidence than one.

## 5. What a refusal costs in layout, which is arvo's whole identity. Verified.

No panellist has priced the fallible return in bytes, and for a substrate whose reason to exist is
control over storage that is the omission I find most surprising.
`05_probes/c_layout.rs`, measured:

| type | size | align |
|---|---|---|
| a 16-bit fixed-point payload, bare | 2 | 2 |
| `Just<payload>` | 2 | 2 |
| `Outcome<payload, ZST error>` | 4 | 2 |
| `Outcome<payload, 2-byte error>` | 4 | 2 |
| `[payload; 8]` | 16 | 2 |
| `[Outcome<payload, ZST>; 8]` | 32 | 2 |
| `u64` against `Outcome<u64, ZST>` | 8 against 16 | 8 |
| `u128` against `Outcome<u128, ZST>` | 16 against 32 | 16 |

**A fallible return doubles every intermediate**, and it doubles it whether the error carries
information or is a ZST, because the discriminant has nowhere to live. For a substrate that ships
`Cold` bitpacked columns and whose stated purpose (`arvo-toolbox-not-policer.md`) is that "every saved
bit compounds across the entity count", a policy that doubles the width of every value in flight is a
larger fact than "call sites unwrap" (`spec:269-271`).

The row that points at the cure is this one:

| type | size |
|---|---|
| a 4-valued enum, validity range known to rustc | 1 |
| `Outcome<that, ZST error>` | 1 |
| `Maybe<that>` | 1 |

Where rustc knows the valid pattern range, **the refusal costs zero bytes**, because it lives in the
niche. And arvo's identity is exact widths: `UFixed<13, 3>` in a 16-bit container has no spare pattern,
but `UFixed<13, 3, Precise>` stores at doubled width and therefore has sixteen bits of spare pattern
space, of which the refusal needs one.

Which brings a structural alignment I want to flag as a hypothesis rather than a result, because I
noticed it rather than derived it. Look at the preset table again (`spec:250-257`). `Precise` is the
**only** preset whose out-of-range resolution is `Refuse`, and it is one of the two whose
`StoredWidth` is `DoubleLogical`. The widening `Precise` already pays for intermediate headroom is
exactly the space a bottom pattern lives in. `Hot` and `Cold` store at minimum width and have no spare
pattern to spend, and neither of them refuses. The one preset that needs the niche is the one that
already has it.

Two readings. Either that is a real structural fact worth encoding, in which case the availability of
the bottom-carrying delivery is conditioned on a `Lowering` member, which is consistent with section
2's sorting and inconsistent with anyone who wants delivery to be policy. Or it is a coincidence of
this particular preset table, which is one day old and which 04 has already argued should change, in
which case leaning on it would be building on sand.

**One mechanism note, so nobody chases the wrong route.** Getting rustc to niche-fill an `Outcome` over
an arvo primitive would need the validity range declared, and the only declaration mechanism is
`rustc_layout_scalar_valid_range_end`, which is compiler-internal, has no tracking issue, and is exactly
the shape `unstable-features.md` calls perma-unstable by intent. So the free-niche route is closed
under this workspace's rules, and the route that is open is the one probe E takes: encode the bottom
yourself in the numeral, which needs no feature gate at all. That is a cheaper answer than the one I
went looking for and it is available today.

## 6. What each delivery costs in emitted code. Verified by reading the assembly, and no timing is claimed.

`05_probes/d_delivery_codegen.rs`, four shapes of one summation, compiled at `-C opt-level=3` on
aarch64 under the pinned nightly. I read the instruction sequences; I took no timings, because a timing
claim is a bench and belongs in `mock/benches/` per `bench-and-sketch-discipline.md`. The
instruction sequence is the artifact.

Refusal as control (`sum_outcome`) emits an eleven-instruction loop body with **two conditional exits
per element**:

```
LBB1_1: ldrh w8,[x0,x9] / add w10,w8,w1 / and w11,w10,#0xffff / mov w8,#1 /
        cmp w11,w1,uxth / b.lo LBB1_5 / cmp w11,#1,lsl #12 / b.hs LBB1_5 / ...
```

Refusal as an absorbing value (`sum_poison`) emits a ten-instruction loop body with **no branch but the
back edge**, the whole resolution collapsing into a conditional-compare chain:

```
LBB2_1: ldrh w11,[x0,x9] / bic w12,w10,w8 / add w8,w11,w8 / and w13,w8,#0xffff /
        cmp w13,#4095 / ccmp w11,w10,#4,ls / ccmp w12,#0,#4,ne / csinv w8,w8,wzr,ne / ...
```

The clamping baseline (`sum_saturate`) is eight instructions, so the absorbing delivery costs **two
instructions per element over saturating**, with the same control-flow shape.

Refusal as an accumulated flag (`sum_flag`) is the one LLVM did something with on its own: it unrolled
four times and split the flag into **four independent `orr` accumulators** combined by three `orr`
after the loop. That is the shape the brief calls "read once at the end", and it unrolled precisely
because the flag chain is an independent reduction. The `Outcome` version structurally cannot be
unrolled that way, because its per-element exit is control flow rather than data.

Two consequences I would put more weight on than the instruction counts.

**A refusing operation with a short circuit is not constant time, and the spec derives `ConstantTime`
without naming the axis that decides it.** `spec:238-241` files `ConstantTime` under derived
properties keyed on the composition. The `sum_outcome` loop exits early on a data-dependent condition;
the `sum_poison` loop does not. Same policy, same numeral, opposite answer on `ConstantTime`, decided
entirely by a delivery choice that appears nowhere in the ten axes. 02's section 11 and 03's section 7
both argue `ConstantTime` does not belong in the derived family; I would add a sharper version of the
same point, which is that as currently keyed it is not merely unenforceable, it is **keyed on the
wrong data**, and a refusing composition cannot honestly claim it under the sum-type delivery at all.

**A short-circuiting refusal leaks the data through timing.** For most consumers that is nothing. For
the one consumer who turned `ConstantTime` on, it is the property they asked for, defeated by the
preset that sounds most careful.

## 7. What the numerical-computing field already decided here, and why. Reasoned.

The spec is proposing, as its most careful preset, the mode IEEE 754 specified and the field
abandoned. That is worth knowing before the round locks, and it cuts both ways.

IEEE 754 has exactly this fork. Its *default non-stop* mode returns a value (an infinity, a NaN, or a
clamped result under some attributes) and raises a **sticky flag** the program reads later. Its
*alternate exception handling*, including trapping, is fully specified. Essentially no numerical code
uses the trapping mode, essentially every language exposes the default one, and the reason is the shape
in section 6: a per-operation trap is control flow in the middle of an arithmetic dependency chain, and
it forecloses every restructuring the hardware and the compiler exist to perform. `Refuse` delivered
through `Outcome` is the trapping mode, per operation, in the type.

Two readings, and I hold both.

The field's reasons were partly about hardware that arvo does not have to care about. A monomorphising
compiler with no allocator and no unwinding is not a floating-point unit; the branch is a real branch
rather than a trap handler, and the substrate's consumers are not all running dependency-chain-bound
kernels. Taking the abandoned road deliberately, for a preset whose stated intent is "the most precise
at the price of both storage and compute" (`spec:246-248`), is defensible on exactly that price.

The field's other reason does carry over, and it is the composition argument this whole panel keeps
rediscovering from different angles. A per-operation refusal changes the *type* of every intermediate,
which is what exiled `Precise` from the algorithm crates (04 section 3, section 3 above), which is what
doubled the layout (section 5), which is what forced the carrier join (section 1). A sticky flag
changes none of those, which is exactly why the standard chose it.

**And the mechanism that makes the sticky flag unusable here is worth naming, because it is the one
place my own lens says the field's answer does not transfer.** IEEE's flags are ambient mutable state
in a status register. arvo cannot have that: a global breaks determinism (`spec:234-236` derives
`Deterministic` per composition), breaks thread-safety without atomics, breaks the const model, and
would be exactly the kind of hidden machinery a substrate should not ship. The alternative to ambient
state is to thread the flag explicitly, and threading state explicitly through a computation so the
consumer can see its cost is evidence passing, which is the same move as section 0's. The
accumulated-flag delivery in probe D **is** the threaded status register, with the accumulator as its
carrier, and the absorbing-bottom delivery in probe E is the same thing with the flag folded into the
value so that no second register is threaded at all.

Two other bodies of prior art worth a look before the round locks, neither of which I have gone into:
Ada's `Constraint_Error` on fixed-point overflow, which is the trapping model with a language-level
handler and which is generally regarded as having cost Ada in numeric code; and the interval and
affine arithmetic libraries, which carry a "possibly invalid" flag alongside every value for exactly
the reason section 5 gives, and whose experience with when the flag gets ignored is the strongest
available evidence about the silent-until-read hazard I raise against my own proposal in section 2.

## 8. Putting the totality distinction on the operation, and installing a handler locally. Reasoned, offered.

The brief asks whether the totality distinction belongs on the operation rather than on the type. I
think the interesting answer is that it belongs on both, and that effect systems have known the shape
for a while: there is an **ambient** handler, installed by the context, and a **locally installed**
one, scoped to an expression. The spec has only the first, `S` on the type, and Rust's own numerics
have only the second, `checked_add` / `wrapping_add` / `saturating_add` on one type.

Under monomorphisation-only dispatch a local install is free. `a.under::<ReduceModulo>() + b` passes a
ZST, selects a different impl, and emits the same code the type-level choice would have emitted, because
the handler is known statically in both cases. That is the whole cost statement: one turbofish, zero
instructions, one more monomorphisation instance.

What it buys is worth more than the ergonomics. It gives a **third** dissolution of the `Precise`
exile, distinct from bifurcating and from changing delivery: the algorithm crate does not need a
fallible surface at all if the consumer can install a total handler for the accumulation and settle at
the boundary. A numerically careful person writing a longest-path computation over `Precise` weights
wants exactly that: accumulate in a widened total carrier, refuse once when the answer is written back,
and never refuse in the middle of a scan where the refusal tells them nothing they can act on.

It is also the toolbox posture rather than the policer posture, in the exact terms of
`arvo-toolbox-not-policer.md`: the substrate exposes the choice at the granularity the consumer knows
something about, and the consumer is the one who knows whether this particular fold can overflow.

**Costs, stated.** Two places a policy can come from is two places a reader has to look, and the
diagnostic hazard 04 documents in its section 2 gets worse, not better, when the composition in an
error message is not the one spelled at the definition site. Monomorphisation instances multiply by the
number of distinct local installs, which is a compile-time cost and therefore the cheapest one arvo
has (`arvo-compile-time-last.md`), but is not zero. And a locally installed handler that differs from
the ambient one is a genuine correctness hazard if it silently widens what the type promised, so the
install probably wants to be restricted to handlers that are *no weaker* than the ambient one, which is
a subtyping condition on the row and is more machinery than the sketch above admits.

## 9. `Growth` and the refusal interact, and the spec does not say how. Reasoned.

One gap that falls in my lens and that I have not seen raised. `Growth` (`spec:166-179`) decides how
much of the exact intermediate is kept. Under `Exact`, the intermediate never leaves its own range, so
the only site a refusal can arise in a binary operation is the final quantisation back to the numeral.
Under `Narrowed<W, A>`, the intermediate can itself go out of range, so a refusal can arise **twice**
in one operation, at the intermediate narrowing and again at the result.

Three things follow that nothing in the round addresses. The carrier join of section 1 has to cover the
intermediate's resolutions too, not just the result's. The count of refusal sites per operation is a
`Growth` fact, and `Growth` is on `Policy`, so the fallibility of an operation depends on two policy
members rather than one. And 01's finding 11 (when quantisation fires) is the same seam approached from
the other side: if the answer to "when does quantisation fire" is "per operation", then `Narrowed`
growth has two firings per operation and the law derivation has to say which one the algebra is about.
02's section 13 listed "whether `Growth` belongs on `Policy` at all" as something it did not get to; I
would add that whatever the answer, the refusal-site count is the concrete question that decides it.

## 10. Engagement with the first four, kept short

**01's finding 3 and its replacement predicate.** I endorse `TranslationStable` over `Faithful`, and
section 4 above is the mechanical confirmation 01 could not run in a review file. One addition from my
lens: 01 notes the identity proves associativity only, and that multiplication and distributivity will
need it re-instantiated. Under the handler reading that is not an inconvenience, it is the correct
shape: a handler is per operation, so a resolution's lemma set is naturally indexed by which operations
it hosts, and re-instantiating per operation is the honest cost of a design where different operations
reach the range ends differently.

**01's finding 14, the Kulisch reframing, and 04's dissent on scope.** I side with 04 on scope and with
01 on substance, and I would add a third position neither states: the projection-with-properties frame
is worth adopting exactly where a property is *checked* rather than named, which after section 4 is
now a mechanical possibility. Adopt the frame in the const-check oracles, where it costs one function
per resolution and buys the width-uniformity argument, and leave the public vocabulary alone.

**02's section 7.** Corrected in section 0. I want to be precise about what survives: 02's finding that
the *declared* `Fallibility` is the wrong place for the classification is right, its proposed repair is
right, and its stated consequence does not follow from its own probe. That is one sentence out of a
strong file, and the reason it matters is that 04 built section 3 on top of it and the panel was about
to inherit a false constraint. This is the 2026-07-28 shape at panel scale rather than at brief scale,
and it argues for the sequential panel doing what it did here: recompiling the prior member's
conclusion, not just reading it.

**02's section 5, the parameter split.** I endorse it and add one reason from my lens that is stronger
than the honesty argument: if delivery is a `Lowering` member (section 2), then a `Lowering` member now
appears in the *return type* of every arithmetic operation, and the claim that laws may not condition on
`Lowering` becomes load-bearing rather than tidy. With `Number<N, P, L>` the impl header cannot mention
`L` and the invariant is typed. With the fused parameter it is prose, and probe `d_fusion.rs` already
shows the prose does not bind.

**03's section 1 and 2, and the demolition of "cannot lie".** Fully agreed, and section 4 is my
contribution to it: totality and coherence catch omissions and contradictions, and the *only* mechanism
in this design that catches a falsehood is the bounded const check, which now exists and bites. I would
put that more strongly than 03 does: after this panel, the sentence the round can honestly write is not
"derived properties cannot lie" but "every case must be answered, no two answers may disagree, and the
answers we could check exhaustively at small width, we did, and here is the file".

**03's section 5, the notko cross-crate absence.** I read notko's impls myself
(`just_consttry_const.rs:26`, `outcome_consttry_const.rs:35`) and 03's reading is correct: the
soundness of the fallibility argument rests on `Just<T>` not implementing
`ConstFromResidual<Outcome<Infallible, E>>`, and nothing in arvo pins that. I would add that under
section 0's handler shape **the dependency disappears**, because no arvo body ever asks for that bound.
That is a second reason to prefer the handler shape over the declared-carrier shape, and it is the kind
of reason I trust most: not that the shape is more elegant, but that it needs one fewer fact from a
crate we do not own.

**04's section 3.** Widened and answered in section 3 above. Its section 2 diagnostic finding I take as
the most under-weighted result in the panel so far, and I would attach one number from my own lens: the
composition that appears in an error message under the spec's shape is the *whole row*, including the
delivery member if section 2 is adopted, so the error text grows with every axis anyone adds.
Diagnostics are a cost that scales with the design's generality, which is an argument for 04's newtype
faces that does not depend on taste.

## 11. What I did not get to

The blanket extension trait that would collapse section 1's `where` clause into one place. I named it
and did not compile it, and it is a twenty-minute sketch that would decide whether the computed carrier
is affordable at the surface size arvo needs.

Whether the carrier join composes with `Growth`'s second refusal site (section 9). Probe B has one
refusal per operation; the `Narrowed` case has two and I did not build it.

The propagating `min` and `max` contract that section 3's hazard demands, and a count of the selection
sites across the four algorithm crates that would have to change.

Whether the absorbing bottom survives `Cold`'s bitpacked layout. A bottom pattern in a bitpacked column
costs a pattern out of a field that may be exactly as wide as its logical width, which is the one
configuration where the niche does not exist. Probe C's `InU32` row hints at the answer and I did not
chase it.

And the bench that would put a number on section 6. The instruction sequences are the artifact I am
willing to stand behind; the throughput consequence of two branchless instructions against two loop
exits is a measurement, it belongs in `mock/benches/` under the harness, and it can be written before
`arvo-policy` exists.

---

**Summary for the next member.** One inherited claim is false and I disproved it by compiling: a single
generic arithmetic body does serve both total and fallible compositions, provided the refusal is
constructed by the resolution rather than by the body, which is evidence passing and which makes 02's
section 7 consequence and everything 04 built on it (section 0, `05_probes/a_handler.rs`). With two
range positions the carrier becomes the join of the handlers' carriers and a lift that is the identity
where they coincide, which is row subsumption and which scales to the second effect the design will
meet (section 1, `b_carrier_join.rs`). The reframe I would most like op to weigh: **how a refusal is
delivered is a `Lowering` member by the spec's own D54 test**, since the representable set and the
mathematical function are identical across the deliveries and only the cost and the call-site shape
differ, which means `Precise` can keep refusing and still gain a total delivery (section 2, reasoned).
That is not speculative: a refusing policy delivered as an absorbing bottom runs through
`arvo_graph::upward_rank` unmodified, at one byte against `Outcome`'s two, propagating correctly and
settling once (section 3, `e_refusing_through_graph.rs`), which is a fourth option 04's three did not
cover; its price is that the bottom must absorb under *selection* as well as under addition, and
`arvo-comb/src/binpack.rs:101-103` is where a naive total order would silently discard it, the exact
defect IEEE 754-2008 shipped in `minNum` and 754-2019 replaced. The mathematics does not move across
the deliveries, verified by compiling 03's proposed const-eval falsification checks, six of them,
including 01's `SubstituteZero` counterexample reproduced mechanically and confirmed to bite (section 4,
`f_const_falsification.rs`). A fallible return doubles every intermediate, 2 to 4 bytes and 16 to 32 for
`u128`, unless the validity range is known to rustc in which case it is free, and `Precise` is the only
refusing preset and one of only two that already store at doubled width, so the spare pattern the
bottom needs is one it already has (section 5, `c_layout.rs`). Refusal as control emits two
data-dependent loop exits per element while refusal as data stays branchless at two instructions over
the clamping baseline, from which follows a finding the spec should not ship without: **a refusing
composition cannot honestly claim `ConstantTime` under the sum-type delivery**, and the axis that
decides it appears nowhere in the ten (section 6, `d_delivery_codegen.rs`, instruction sequences read,
no timing claimed). I rule on nothing; op decides.
