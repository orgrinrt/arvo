# 82. Lifting a measured region into a declaration, and what decides whether it lifts

**Author lens:** Jhala. Refinement types. A type is an ordinary type paired with a predicate, the predicate
is carried through the program by the checker, and the arithmetic is handed to a solver. The question I am
usually asking is which facts can be stated at the type and which cannot, and why.

**Position:** fifth expert in the derived-algebraic-laws unit, after `76`, `77`, `79`, `80` and the
coordinator's checkpoint `81`. My assigned task is the one `80` names as the cheapest next instance and
reports nobody has attempted: take a trajectory predicate this panel has measured and find out whether
there is a declaration a consumer would actually write that turns it into a fact known before the program
runs.

**Probes:** eleven instruments in `82_probes/`, committed with their sources, transcripts and emitted
assembly as they ran, before this file. Two were rewritten after their first version measured the wrong
thing, and both versions are on disk.

---

## 0. Gates, and a steer that arrived while I was working

**Canon gate: passes, situation two.** No canon exists. `mock/canon/` is absent, `mock/crates/` is empty by
the declared mutation order, and this panel is writing the first canon. `INTENTS.md` holds one RATIFIED
entry, I13, ratified narrowly on op's own instruction that it means no more than he said
(`INTENTS.md:211-214`). The other eleven are STATED under the standing instruction that nothing about them
is absolute (`INTENTS.md:40-41`). Nothing below settles anything.

**Test gate: no suite exists.** The mock workspace has no members. The substitute is the probe discipline,
applied to my own instruments first. Two of mine failed their first run in ways that would have produced a
confident wrong sentence, and both are recorded rather than quietly replaced: `p5`'s length-axis table
printed `none <= 7` on every row where its own enumeration bound had cut the walk off earlier than seven,
which overstated coverage on exactly the widest windows (`p5_output_FIRST_RUN.txt` against `p5_output.txt`);
and `p4`'s first loop-attribution script picked the last label in each function, which is usually an
epilogue, and reported a 94-instruction inner loop for an arm whose real loop is four instructions. The
second is fixed by `p4_count_loops.py` finding backward branches instead, and the full bodies are dumped to
`p4_loop_bodies.txt` so every stride below can be read rather than trusted.

### The steer, and where it landed in this file's timeline

Op wrote into the panel at `83` while I was in flight. I was told by a message pointing at the file rather
than paraphrasing it, and I read it at the source. His words:

> Let me just add there that the above collapses to whatever is available at const time: Making the
> predicates const expressions for example, allows using const functions and pipe in some data that is
> outside the typestate. However, being const time expressions, typestate is usable there too

**It arrived after `p1` through `p5` had run and before `p6` through `p9` existed.** So the first half of
this file was built inside the framing op rejects, and the second half was built because he rejected it.
Nothing is deleted. What changed is stated in section 5, at the point in the argument where it changed.

The short version: my brief, following `80`, framed the axis as **typestate against trajectory** and asked
whether a condition over values could be lifted into a fact about a **type**. Op's answer is that the axis
is **const-available against not**, and that the typestate is one source of const-available data rather
than the only admissible one. That does not invalidate what `p1` and `p2` measured, because both measured
what a **declaration** reaches and a declaration is const-available under either framing. It does
invalidate a route I had closed without examining, which section 6 reopens and measures: at a given call
site the **operands themselves** may be const, and then there is nothing to lift at all.

**Three things stay outside this file**, and I do not answer them. Whether a condition that is genuinely
not const-available may gate an arm at all, which `83` explicitly says his words do not reach. Which
binding time op's verb "validate" carries, which is Q-A and sits on a different axis. And whether the
long-standing `no_std` and no-`dyn` constraints are his intents; nothing below depends on either answer,
and I say where in section 14.

---

## 1. Verification before argument

Per the panel rule that a probe is cited for what it proved and presumed flawed until checked, I re-ran the
instrument my task is about rather than trusting `79`'s or `80`'s account of it.

`79_probes/p1_compositional_predicate_search.rs`, copied to `82_probes/p0_rerun_of_79_p1.rs` and rebuilt on
the pinned toolchain, reproduces to the digit: the composed law `(a+b)-c == a+(b-c)` for unsigned saturating
`u8` holds on 2,894,336 of 16,777,216 triples, fails on 13,882,880, 82.7484%, and P4's four-way case split
has zero sufficiency violations and zero necessity violations while P0 through P3 each miss a direction
(`82_probes/p0_rerun_output.txt`).

I also confirm `80`'s reading of that probe's shape at the source: it contains no `const fn` and no `const`
item, and every candidate is an ordinary function of `(a, b, c)` evaluated in `fn main`. `80` is right about
what `79` built.

---

## 2. Route one: lifting P4 through declared operand ranges

The obvious lifting replaces P4's per-value conditions with per-operand **declared intervals**. Instead of
asking whether this triple satisfies the case split, ask whether every triple drawn from
`[La,Ha] x [Lb,Hb] x [Lc,Hc]` does. The interval bounds are const-available whether they come from a type
parameter, a module const, or a const function, so this route survives op's reframing unchanged.

`82_probes/p1_box_lifting_of_p4.rs` derives a characterisation of the fully-holding boxes by hand and then
cross-checks it against brute force over **every** box at each width in a model band, which is `80`'s
section 4.3 shape applied to a different claim.

**The characterisation.** A box is fully holding exactly when one of three clauses holds:

1. `Ha + Hb <= MAXV` and `Lb >= Hc`, which says no clamp can fire anywhere in the box.
2. `La == Ha == 0`, which says `a` is identically zero.
3. `Lc == Hc == 0`, which says `c` is identically zero.

**Cross-checked, zero disagreements**, over 1,000 boxes at width 2, 46,656 at width 3, 2,515,456 at width 4
and 147,197,952 at width 5 (`82_probes/p1_output.txt`). The check can fail: three perturbations of the
closed form disagree with brute force on 483,302, 268,997 and 15,651 boxes respectively over widths 2 to 4.

**What it reaches at the shipped width.** Maximising each clause:

| clause | maximal box | volume | share of the holding set |
|---|---|---|---|
| (i) no clamp can fire | `a in [0,85]`, `b in [85,170]`, `c in [0,85]` | 636,056 | 21.98% |
| (ii) `a` identically zero | `a = 0`, `b` and `c` free | 65,536 | 2.26% |
| (iii) `c` identically zero | `a` and `b` free, `c = 0` | 65,536 | 2.26% |

So a single declared-range box reaches at most **636,056 of P4's 2,894,336 holding triples, 21.98%**, and
3.79% of the domain.

**And every non-degenerate box it reaches is clamp-free.** That is the part that matters, and I measured it
directly rather than deriving it from the characterisation, because deriving it would have made the closed
form load-bearing twice. `82_probes/p1b_is_every_lifted_box_degenerate.rs` counts, at each model width,
fully-holding boxes in which a clamp can fire and which are neither of the two degenerate shapes. The
residue is **zero at widths 2, 3, 4 and 5**, against a control of 916, 44,940, 2,461,192 and 144,873,168
boxes in which a clamp fires at all, so the clamp detector is not silently returning false
(`82_probes/p1b_output.txt`).

**Reading.** The declared-range lifting of P4 works, is sound, and lands entirely in one of two places.
Either no clamp can fire, in which case saturating arithmetic and exact arithmetic compute the same function
on the box and the law holds because integers associate; or an operand is pinned to zero, in which case one
of the two operations is the identity and the law reads `a+b == a+b` or `b-c == b-c`. In neither case is the
licensed arm about saturation.

That is not nothing. An arm gated on "no clamp can fire here" is a real arm and a useful one, because it
licenses dropping the clamp instructions entirely. But it is a **different arm from the one P4 describes**,
and calling it a lifting of P4 would overstate what happened: what lifted is the sub-case of P4 on which P4
is not doing any work.

---

## 3. The declaration a consumer would actually reach for is unsound

Before section 2's answer can be read as a limitation of declarations in general, the natural declaration
has to be checked, because it is not the one section 2 uses.

Asked to declare something that makes `(a+b)-c == a+(b-c)` safe, a competent consumer says **"my values
never overflow the result."** That is `79`'s P1, and it is wrong.

At the value level, over the full `u8` domain, the declaration is true and the law is false on **8,355,840
triples, 49.80% of the domain** (`p0_rerun_output.txt`, reproduced independently in `p1_output.txt` section
4). At the box level over widths 2 to 4, 447,831 boxes satisfy it, and the law fails somewhere inside
**391,767 of them, 87.5%**, with the first at width 2, `a in [0,1]`, `b in [1,3]`, `c = 1`.

The witness is small enough to check by hand. Take `a = b = c = 255`. The exact result `255 + 255 - 255` is
255, comfortably inside range, so the declaration holds. But the left form saturates `a+b` to 255 and then
subtracts 255, giving 0, while the right form computes `b-c = 0` and returns 255.

**So the intuitive declaration licenses a rewrite that is wrong on half the domain.** It is the shape of
declaration a design would most likely offer and a consumer would most likely write, it reads as exactly the
right precondition, and it does not imply the law. Section 2's clause (i) is sound because it constrains the
**intermediates**, not the result, and those are different conditions.

---

## 4. Why P4 does not lift, stated as a property rather than as a result about P4

The characterisation in section 2 says the thing more generally than its own table does.

P4's holding region is a statement about **which clamp events occur**. A declaration is a statement about
where the operands live. For a declaration to imply a statement about clamp events, it has to constrain the
operations' inputs so that the clamp cannot occur, because a declared interval says nothing about where an
intermediate lands. And a saturating operation's intermediate leaves the declared interval by construction:
that is what saturation is for.

Put as a property: **the region P4 names is not closed under the operations P4 is a law about.** A
declaration that implies it must therefore forbid the operations from leaving it, and forbidding a
saturating operation from saturating is exactly the degenerate case section 2 measured.

That suggests the discriminator, and section 7 tests it: a trajectory condition should lift when the region
it names **is** closed under the operations in question, because then a declaration can constrain the entry
point and the region maintains itself.

I record that this criterion is the same one both cold derivations reached for a different question. `77`
puts it as "multiples of a quantum are closed under addition; they are not closed under multiplication
without widening" (`77:250`), and `76` adopts it as "whether an operation needs chain-level machinery is a
fact about whether that operation's rounding step is closed under the algebraic structure its own outputs
feed back into" (`76:370-372`). I reached it from `p1`'s box characterisation before reading either file,
and read both afterwards; that ordering is why I claim it as a third instance of the criterion rather than
as agreement with them.

---

## 5. Op's steer, and what it changed

This is where `83` arrived. Sections 2, 3 and 4 above were already built.

**What survives unchanged.** Everything in `p1` and `p1b`. They measure what a declared interval reaches,
and an interval's bounds are const-available under any reading. The numbers, the characterisation and the
cross-check do not move.

**What was wrongly framed.** I had been treating "is it a fact about the type" as the question. It is not.
The question is whether the datum the predicate reads is available at const time, and a type is one source
of such data among others.

**What reopens.** I had closed, without examining, the possibility that P4's own conditions could be
const-available. Under the typestate framing that was reasonable, since `a`, `b` and `c` are values and
values are not types. Under op's framing it is a live route: at a call site where an operand is a literal, a
`const`, or a const function's result, a condition reading it **is** a const expression, and nothing is
lifted because nothing had to be. Section 6 measures it.

**What his words do not reach**, and I do not extend them: whether a condition over genuinely runtime data
may gate an arm. P4's non-degenerate holding region is a condition on clamp events over runtime operands. So
after section 6 the honest position is that P4's interesting region is neither liftable nor const-available,
and whether that makes it unusable is a question `83` leaves open and I leave open.

---

## 6. Route two: which const-available operands decide P4

`82_probes/p6_const_availability_lattice.rs` enumerates it exhaustively. For every subset of `{a, b, c}` and
every assignment of const values to that subset, it asks whether the law's truth is constant over the
operands not in the subset. Three outcomes: **licensed** (constant true, an arm may be selected), **refused**
(constant false, the arm is provably wrong and a const expression can say so), and **undecided**.

| const operands | configurations | licensed | refused | undecided | licensed volume | share of holding |
|---|---|---|---|---|---|---|
| none | 1 | 0 | 0 | 1 | 0 | 0% |
| `{a}` | 256 | 1 | 0 | 255 | 65,536 | 2.26% |
| `{b}` | 256 | 0 | 0 | 256 | 0 | 0% |
| `{c}` | 256 | 1 | 0 | 255 | 65,536 | 2.26% |
| `{a,b}` | 65,536 | 256 | 0 | 65,280 | 65,536 | 2.26% |
| `{a,c}` | 65,536 | 511 | **32,640** | 32,385 | 130,816 | 4.52% |
| `{b,c}` | 65,536 | 256 | 0 | 65,280 | 65,536 | 2.26% |
| `{a,b,c}` | 16,777,216 | 2,894,336 | 13,882,880 | 0 | 2,894,336 | 100% |

Three things in that table.

**Partial const-availability of operands reaches 4.52% at best**, against the declared range's 21.98%. Both
are const-available and both are therefore admissible under `83`; they differ by a factor of five in reach.

**The single-operand licensed values are exactly `a = 0` and `c = 0`**, printed explicitly in section 2 of
the probe's output. So this route lands on the same two degenerate regions the declared-range route landed
on, arrived at by a completely different mechanism. Knowing `b` licenses nothing at any value.

**The `{a,c}` row has 32,640 refused configurations**, and that is the first time this panel has
instrumented the **admissibility** cell. `80` section 1.2 reports that of the three-by-two grid formed by
op's three validated things and the two binding times, the panel's law-layer evidence occupies one cell, and
that nothing anywhere has instrumented admissibility of a law declaration. These 32,640 configurations are
exactly that: a const expression that proves the arm is wrong for every runtime `b`, and can refuse it
rather than fall back. The count is checkable by hand: it is the pairs with `a >= 1`, `c >= 1` and
`a + c > 255`, which is `255 * 256 / 2 = 32,640`.

Section 3 of the probe runs the same lattice for signed saturating associativity at width 4 and finds one
const operand licenses 1 configuration of 16, two license 80 or 144 of 256, and **no partial assignment ever
proves the law universally false**. So the asymmetry in the P4 table is a fact about P4 rather than about
clamp conditions in general.

---

## 7. Route three: a region that does lift, and it is not P4's

Section 4 predicted that a trajectory condition lifts when its region is closed under the operations. The
panel has a measured region of exactly that shape and nobody had connected it.

**The bracketing evidence, both reached through `OPTIONS.md` and named as such.** `35` measured that signed
saturating folds diverge under reassociation on 70.1% of vectors at n = 8, against zero for the other three
sign-and-policy combinations (`OPTIONS.md:1093-1096`). `55b` measured that of 952 divergent signed triples,
**zero have all-same-sign operands**, and that every divergence is a clamp event followed by an operand
moving the partial sum back toward the interior (`OPTIONS.md:1117-1135`).

"All operands same sign" is not a property of a trajectory. It is a property of an **interval**: an interval
either straddles zero or it does not, and which it does is decidable from its bounds. So the lifting
candidate writes itself:

> a declared operand interval `[LO, HI]` with `LO >= 0` or `HI <= 0`
> implies
> every parenthesisation of a fold over operands from `[LO, HI]` agrees.

`82_probes/p2_sign_uniform_lifting.rs` tests it over **every** interval of the representable set at widths 2
through 6, against brute force. The closure is computed to a fixpoint rather than assumed, because the law
is about a fold whose intermediates leave the declared interval.

| width | intervals | associative on closure | sign-uniform | sufficiency violations | necessity violations |
|---|---|---|---|---|---|
| 2 | 10 | 8 | 8 | 0 | 0 |
| 3 | 36 | 24 | 24 | 0 | 0 |
| 4 | 136 | 80 | 80 | 0 | 0 |
| 5 | 528 | 288 | 288 | 0 | 0 |
| 6 | 2,080 | 1,088 | 1,088 | 0 | 0 |

**Exact, in both directions, at every width in the band.** And the instrument can fail: four weakenings of
the predicate were run as negative controls, and each breaks it. Allowing one negative gives 26 sufficiency
violations, allowing one positive gives 30, dropping the non-positive arm gives 215 necessity violations,
and a narrow-interval predicate gives 284.

**It survives arity**, computed rather than assumed by enumerating every parenthesisation directly. At width
5, the full straddling range diverges on 0, 7,920, 478,185 and 20,509,425 tuples at arities 2 through 5;
the non-negative half and the non-positive half diverge on **zero at every arity**.

**And at a shipped width, sampled at the shape `35` used.** Four million length-8 vectors, left fold against
balanced tree, signed `i8`:

| declared window | divergent | share |
|---|---|---|
| `[-128, 127]` straddles | 2,544,825 | 63.62% |
| `[0, 127]` non-negative | 0 | 0% |
| `[-128, 0]` non-positive | 0 | 0% |
| `[-1, 127]` straddles by one | 31,331 | 0.78% |

My 63.62% is not `35`'s 70.1%. Both are samples of a space far too large to enumerate, drawn by different
generators from different constructions, and I do not treat the gap as a disagreement or as a reproduction.
What reproduces exactly is the arity-3 count at width 4: 952 tuples, matching `80`'s p6 and the count `74`
corrected onto the right operation.

### Why this one lifts and P4 does not

The probe's section 4 makes it mechanical. The closure of `[0, 7]` at width 6 is `[0, 31]`, still
non-negative. The closure of `[-7, 0]` is `[-32, 0]`, still non-positive. The closure of `[-1, 1]` is
`[-32, 31]`, the whole range, and non-associative. The closure of `[3, 5]` is `[3, 31]`, still non-negative.

A sign-uniform window is closed under the operation in the sense that matters: the clamp it can reach is
absorbing, and no admissible operand moves a clamped partial sum back off the endpoint. Which is exactly
`55b`'s measured pullback mechanism, stated as a property of the declaration rather than of the trajectory.

**One consequence I want to name because it retro-explains a result a predecessor called surprising.**
`76:51-53` reports that saturating addition on an unsigned domain "turned out to be universally
associative, which contradicted my working assumption going in". Under this reading it is not a separate
fact: an unsigned type's whole representable set is non-negative, hence sign-uniform, hence its only clamp
is absorbing. **`76`'s unsigned universality and this section's signed conditional result are one theorem
with the sign domain moved from the container to the declaration.**

### And it is `57b`'s H1, instantiated

`79:112-121` brings `63`'s C6 into this unit, whose first hypothesis is *"the ambient operation is
associative on the reachable set"*. A declared operand window **is** a declared reachable set, once its
closure is taken. So the construction below is H1 turned from a fact requiring measurement per type into a
const predicate over a declaration. I did not set out to instantiate C6 and only saw the connection after
`p2` ran, which is why I state it as a fit rather than as a derivation from it.

---

## 8. The construction, and four refusals

`80` asks for a declaration a consumer would actually write. Sections 8 and 9 are that, built and compiled
on the pinned toolchain, with the refusals that show the gate bites.

### 8.1 The first construction, and the hole in it

`82_probes/p3a_the_construction.rs` is the shape both cold derivations built, applied here: a `Window` trait
carrying the declared bounds, a `ReassociableFold` permission whose associated const runs the verdict, and
a consumer bounded on the permission. The verdict is a closed form, cross-checked at compile time against a
swept verdict over a model band of widths 2 to 4, with the population counters asserted non-degenerate so
the cross-check cannot pass by visiting nothing or by answering one way on everything. It is `#![no_std]`,
and it uses no forbidden feature.

It compiles. A perturbation of the closed form to `p2`'s mutant M1 is refused with the named diagnostic
(`p3c`). And then it fails the test that matters:

**A straddling declaration inside an unreached `pub fn` compiles clean.** `p3b` adds
`Win<-128, 127>` and a consumer for it, changing nothing else, and rustc accepts the crate. `nm` on the
archive shows why: the only text symbol emitted from the crate's own code is `_agreement_check`, and the
straddling consumer was never codegen'd, so its associated const was never evaluated. Making the same
function `#[no_mangle] extern "C"` so it is reached produces the refusal immediately, with the intended
message (`p3b2`, transcript in `p3_transcript.txt`).

So the const-assert permission is a **monomorphisation-time, reachability-dependent** refusal. It is not
unsound, because unreached code computes nothing, but it is much weaker than it reads: the gate fires where
the arm is used and nowhere else, so a library shipping a wrong declaration in an unused path is told
nothing.

### 8.2 Lifting the predicate into a bound, refused

The stronger form would be a type-check-time refusal, and the obvious spelling puts the predicate in a
bound. `82_probes/p3e_bound_position_refused.rs` attempts it two ways and both are refused:

```
error: generic parameters may not be used in const operations
   |     Cond<{ closed_verdict(LO, HI) }>: IsTrue,
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

Four errors, two per spelling, each pointing at the forbidden feature by name
(`p3e_compile_output.txt`). Recorded as a compiled refusal rather than as an argument.

### 8.3 The decomposition, which does refuse at type check

The workspace's standing move on a refused bound is to decompose the constraint into named contracts rather
than force an expression into a position the language will not take it. That is available here for a
specific reason: `LO >= 0 || HI <= 0` is not arbitrary arithmetic, it is **a disjunction of two shapes**,
and a shape can be a type.

`82_probes/p3d_structural_permission.rs` carries three declaration shapes. `NonNeg<const LO: u8, const HI:
u8>` is a window whose bounds cannot be negative because the const parameters are unsigned.
`NonPos<const MAG_LO: u8, const MAG_HI: u8>` is its mirror. `Win<const LO: i32, const HI: i32>` is general
and may straddle. The permission is implemented for the first two and for nothing else, so there is no
assertion in it at all: the shape is the predicate.

The verdict cross-check moves to a crate-level `const _: () = { ... }`, which is evaluated whether or not
any generic function is ever instantiated.

Both halves then bite where the first construction did not:

- The straddling window in **dead code**, under `--emit=metadata` with no codegen, is refused with
  `error[E0277]` and the `#[diagnostic::on_unimplemented]` text, plus rustc listing the two shapes that do
  implement the permission (`p3d_refusal_output.txt`).
- The perturbed closed form, also under `--emit=metadata`, is refused by the crate-level const
  (`p3d_perturbed_output.txt`).

### 8.4 The other shape op names: data from outside the typestate

`83` says a predicate may "pipe in some data that is outside the typestate". `82_probes/
p7_window_from_outside_the_typestate.rs` builds that: the window is an ordinary module `const` in the
consumer's own code, one of them computed by a const function from other consts, read by an inline `const`
block inside a **non-generic** arm. No type carries the bounds.

It compiles, and a straddling module const is refused with the named message.

### 8.5 The binding-time ladder, measured

`83` collapses the axis onto const-availability. `82_probes/p8_binding_time_ladder.sh` measures that const
time is not one moment. Each construction is compiled twice, type check only and full codegen, with exit
codes captured directly rather than through a pipe (`p8_binding_time_output.txt`):

| rung | construction | dead code, type check only | dead code, full codegen | reached, full codegen |
|---|---|---|---|---|
| 0 | crate-level const carrying the verdict cross-check | REFUSED | REFUSED | REFUSED |
| 1 | structural trait bound (`p3d`) | REFUSED `E0277` | REFUSED | REFUSED |
| 2 | inline const block in a non-generic fn (`p7`) | accepted | REFUSED `E0080` | REFUSED |
| 3 | const assert in a generic's associated const (`p3a`) | accepted | **accepted** | REFUSED `E0080` |

**Every rung is a const predicate and they are not interchangeable.** Rung 3 is what both cold derivations
built and what `p3a` reproduces, and it is the weakest of the four: a wrong declaration in unreached code
compiles clean at every setting. Rung 1 is available only where the predicate decomposes into shapes. Rung 0
is where a verdict check belongs, because it is attached to no function and therefore cannot be skipped.

---

## 9. What the arm unlocks, and the control that had to be run

`80` section 5.2 is the reason this section exists: it found a law that was true and bought nothing, because
at `F = 0` the backend performed the rewrite itself and the two arms became one symbol. So the question is
not whether the law holds but whether it reaches a lowering the backend cannot reach unaided.

`82_probes/p4_what_the_lifted_arm_unlocks.rs` puts four arms and two controls in one crate, each
`#[no_mangle] extern "C"` and `#[inline(never)]`. Strides read by hand from the dumped loop bodies in
`p4_loop_bodies.txt`; the counts are from `p4_count_loops.py`.

| arm | inner loop | elements per iteration | instructions per element | vector saturating add |
|---|---|---|---|---|
| `sat_sum_seq`, the fold as written | 8 | 1 | 8.000 | no |
| `lanes4_indexed`, licensed, no bounds proof | 37 | 4 | 9.250 | partial `sqadd.2s` in scaffolding |
| `lanes16_chunked`, licensed plus bounds proof | 4 | 16 | **0.250** | **`sqadd.16b`** |
| `sat_sum_seq_chunked_no_law`, bounds proof, NO law | 99 | 16 | 6.188 | no |
| `wrap_sum_seq`, control, needs no typestate | 8 | 64 | 0.125 | `add.16b` |

**The control that decides it is the fourth row.** The bounds proof alone, without the law, gets a 16x unroll
with the serial clamp chain fully intact: sixteen copies of `add ; cmp ; csel ; cmn ; csel`, no vector
instruction. So the win is not the bounds proof; the law is load-bearing. That is `80` section 5.2's failure
mode explicitly excluded rather than assumed away.

**And the first licensed attempt loses**, reproducing `80` section 5.3's finding in the signed case and by a
larger margin: `lanes4_indexed` uses the law correctly, indexes the slice so the bounds are not provable,
and emits three bounds-check branches to panic paths plus a two-lane `sqadd` wrapped in
shift-extend-shift-shift-right scaffolding, landing at 9.250 against the unlicensed 8.000.

**Erasure.** The assembler emitted two symbol aliases:

```
_sat_sum_lanes16_nonpos = _sat_sum_lanes16_nonneg
_sat_sum_lanes16_smallgain = _sat_sum_lanes16_nonneg
```

Three different licensed declarations, `NonNeg<0,127>`, `NonPos<0,128>` and `NonNeg<3,40>`, assemble to one
symbol. The declaration is fully erased. That is `68`'s aliasing instrument used to establish erasure rather
than redundancy.

**Every magnitude in this section is unpriced.** No bench ran, instructions per element is not time, and the
only qualitative fact a spike can carry is that `sqadd.16b` appears in the arms the law licensed and in no
other arm in the crate.

### 9.1 The arms have to be right, and the check has to be able to fail

`82_probes/p5_agreement_and_the_length_axis.rs` section A runs 200,000 random vectors of length 0 to 199 per
window against the sequential fold:

| declared window | vectors | `lanes4` disagrees | `lanes16` disagrees |
|---|---|---|---|
| `NonNeg<0,127>` licensed | 200,000 | 0 | 0 |
| `NonPos<0,128>` licensed | 200,000 | 0 | 0 |
| `NonNeg<3,40>` licensed | 200,000 | 0 | 0 |
| `Win<-128,127>` refused | 200,000 | 146,323 | 167,875 |
| `Win<-1,127>` refused | 200,000 | 1,190 | 1,588 |

The refused rows are large and non-zero, so the instrument enters the path the declaration exists to forbid.
A zero there would have meant the licensed rows proved nothing.

---

## 10. The length axis, which is the same staging boundary from a new direction

`p2`'s parenthesisation table turned up something that does not fit the closure model: operands drawn from
the straddling window `[-1, 1]` produced **zero divergence at arities 2 through 5**, even though that
window's closure is the whole range and the whole range is not associative.

The explanation is that at a bounded length the reachable set is the n-fold sumset, not the closure, and a
short fold over a narrow straddling window never reaches a clamp. So a second, weaker predicate exists that
reads the window **and the length**, and it licenses strictly more.

`p5` section B measures where the two part company, reporting for each window the first arity at which some
vector diverges:

| width | window | sign-uniform | first divergent arity | associative on closure |
|---|---|---|---|---|
| 4 | `[-8,7]` | false | 4 | false |
| 4 | `[-1,2]` | false | 6 | false |
| 4 | `[-2,2]` | false | 6 | false |
| 5 | `[-3,3]` | false | 7 | false |
| 5 | `[-1,2]` | false | none <= 7 | false |
| 6 | `[-2,2]` | false | none <= 7 | false |
| 6 | `[-3,3]` | false | none <= 7 | false |

`none <= k` means the exhaustive enumeration hit this probe's own 30-million-tuple bound at arity k, not
that no divergence exists above k. The first run of this probe printed `none <= 7` on every such row, which
overstated coverage on the widest windows, and both runs are on disk.

**So a length-aware predicate is a real, strictly larger arm**, and it needs the length at const time. `80`
section 7 puts capacity at stage zero and length at stage one, and derives from that why the capacity-keyed
accumulator relation in Q11 compiles gate-free and the length-keyed one does not. This is the same boundary
arriving from the law layer rather than from the fold layer: **the same declared window licenses more for a
statically-sized fold than for a runtime-length one, and the gap is exactly the arities between the sumset
frontier and the closure frontier.**

I did not build the length-aware construction. It is the cheapest next instance this file leaves.

---

## 11. Attacking `80` section 4.2 inside the region this lifting names

`80` section 4.2 found that at a shipped width the const evaluator produces only **negative** law verdicts:
the false verdict exits at its first counterexample in 0.50s, the true verdict at the same width and arity is
refused after 4.48s, and the verdict that licenses an arm is the positive one. The register carries that as
Q38(a)'s cost.

That is measured over the full representable set. A sign-uniform declaration restricts each operand to a
half, so an exhaustive arity-3 sweep costs `2^(3(W-1))` rather than `2^(3W)`, a factor of eight. So the
restriction buys one bit of width, and the question is whether one bit crosses the line at a width that
matters.

`82_probes/p9_positive_verdict_at_a_shipped_width.py` walks it, emitting each check as a top-level `const`
that **counts** violations rather than returning early, so the domain is visited whatever the verdict is.
Results and reading in section 11.1 below.

---
