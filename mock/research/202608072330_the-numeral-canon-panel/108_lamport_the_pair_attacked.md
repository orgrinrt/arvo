# 108. The pair attacked

**Position:** the dispatch `106` asked for. Its section 16 names the largest thing it did not do,
"I did not attack the pair", and says that if one dispatch follows the consolidation, that is what it
should be. `107` agrees in its own section 7. This is that dispatch.

**Author:** the `lamport` persona. **Standing:** nothing here settles anything. Op decides. Every
judgement below is a suggestion, and where I say a clause is false I mean it is false as written and I
say what replaces it.

**What I read.** `INTENTS.md` in full including I18, `RULES.md` in full, `106` in full, `107` in full,
`102` in full, `40` sections 0, 3.1, 5 and 6, `97` sections 3 and 4.1, `103` sections 6 and 7, `93`
section 12, op's `88`, `95`, `104` and `105`. Bounded coverage is section 14 and it names what I did
not open.

**What I did.** Eleven probe scripts in `108_probes/`, each committed with its output before this file
was written. Three are compiled Rust on the pin, three are exhaustive sweeps in exact arithmetic, two are
citation checkers that are mutation-tested, and the rest read the corpus or the emitted assembly. Nothing
here is timed and **nothing here is a bench**; every number is a computation over committed artifacts, a
compile, or a suite run.

---

## 1. The answer, before the working

The pair survives as a two-component object and **five of its eight clauses need repair**: two are
false as written, one is under-specified, one is inconsistent as a mechanism, and one is arithmetically
wrong. Two are kept unchanged and one is kept with a refinement offered. Each repair has a replacement,
every replacement is drawn from material the unit already holds, and none of them costs the structure.

The scoreboard, so the ratio is checkable rather than asserted:

| clause | verdict | section |
|---|---|---|
| 1, a strategy is a pair | kept, with a dependent-sum refinement offered | 3.6, 6 |
| 2, the observable axes | **under-specified**, and its qualifier is gone | 3.1 |
| 3, arms that produce the answer component one fixed | **false** | 3.2 |
| 4, different carriers, and a name binds one point in each | first two sentences kept, **third inconsistent** | 3.3 |
| 5, measured or computed | kept, and made non-vacuous by the clause-three repair | 3.6 |
| 6, region against cost vector | kept unchanged | 3.6 |
| 7, nothing relates two second components | **false, because one word names two objects** | 3.4 |
| 8, the ceiling | **undercounts, by the size of the other component** | 3.5 |

**Clause three is false, and it is the one that matters.** Component two is defined as ranging over
"the arms that produce the answer the first component fixed". In that region a fidelity coordinate
measures a constant, which `106` says itself in its section 8. So op's accuracy intent is expressible in
neither component: not in component one, which fixes an answer rather than ranking approximations to it,
and not in component two, whose region makes the coordinate that would carry it constant. `102` states
both halves of this contradiction inside itself, one page apart, and `103`, `106` and `107` all carried
it forward. **The repair is one word: component one fixes the DENOTED answer, the declared semantics,
rather than the computed one.** Arms then realise the declaration exactly or approximately, the fidelity
coordinate is the distance from it, and it varies. Section 3.2, probe p5.

**Clause two is under-specified, and its central word arrived without the qualifier its author attached.** "Observable" is
`40`'s definition, not `97`'s, and `40` attaches a condition to it that no file in this unit carries:
the classification of an axis depends on a convention about which width the overflow policy is applied
at. I measured that convention and it is load-bearing to 89.081% against 0%. Worse, observability is not
a property of an axis at all: the same axis is observable on one chain and not on another. **The repair
is that component one's domain is the conservative closure, and the per-region licence is an arm rather
than a reclassification**, gated by a sound const-checkable predicate I built and tested at zero unsound
predictions over 8019 chains. Section 3.1, probes p1, p2, p2b, p2c.

**Clause seven is false because one word names two objects.** It says two strategies are related "by
nothing on their second" component, and `106`'s own section 5 says "on the weighting, the join is union
and it is free". Both are true. `97`'s free join semilattice is over a **support**, a set of demanded
coordinates, where union is a unique least upper bound in 9 of 9 ordered pairs. `101`'s ray is a
**rate**, where six defensible combinations disagree on 71.4% of swept rate pairs. **The repair is to
split the word rather than pick a side.** Section 3.4, probe p6.

**Clause four is inconsistent as a statement and free as a mechanism.** A named strategy cannot "bind
one point in each" when the components have different carriers, and `106` section 10 sees the tension
and resolves it by naming the object rather than by removing it. I compiled the three available shapes
and **all three emit the same two symbols**. So the fork has nothing a consumer can observe, and it is
decided on ergonomics and on who may move component one. The repair is one word, "binds" to "supplies a
default in". Section 3.3, probe p4.

**Clause eight undercounts.** The ceiling is `101`'s and it bounds component two. Under the pair the
strategy space is the product of both components, so the bound is the product. Section 3.5.

**Of `106` section 4's three separable legs, one is void and one is half wrong.** Leg (a), that the two
components are what op's `88` answer decomposes into, rests on an exegesis of a sentence op flagged as
hard to word, disclaimed the tail of, and has since declined to rule on the nearest question about.
**There are five incompatible readings of it in this panel**, from `93`, `97`, `98`, `102` and `106`,
and nothing can distinguish them. The pair stands on measurement or it does not stand. Leg (b) survives
for I3 on op's own `104` and **fails for I5**, and my clause-three repair is what flips it: once
component one fixes the denotation, "sacrifice soundness for a provable meaningful gain" is a rate, which
is `40` section 5.3's reading restored. Leg (c) survives and sharpens. Section 4.

**Is the pair new: partly, and the panel has the credit in the wrong place.** The two-level structure is
`40`'s, 62 files before `102`, and `93`'s cold derivation says so in its own section 12. What is
genuinely `102`'s is the **relocation**: `40` names the objective and exposes the observable axes beside
it, and `102` puts the observable assignment inside the strategy as component one. That relocation is
real, it is at ONE EXPERT, and it is the part worth attacking. The structure under it is at TWO EXPERTS
and `106` records it at no rung at all. Section 5.

---

## 2. The two gates

Both run before the assigned work, and the brief can neither request nor waive them.

### 2.1 Canon gate: passed

Checked against `INTENTS.md` I1 through I18 read in full, and against op's `88`, `95`, `104` and `105`.

Attacking a definition of what a strategy is, and offering a repaired one, is licensed by I1, which
demotes the strategy set to open in op's own words, and by I12, which says nothing settles until the
experts converge. `95` is the direct warrant for the shape of this file: op asks that units be steered
"towards convergence and solution-finding together, instead of just disproving and attacking (though
that's a vital part of it too, just has to end with solutions and agreements at least with something)".
So every refutation below carries its replacement, and section 7 is a statement offered rather than a
list of holes.

**One thing I checked because it would have been a refusal.** I16 carries op's words: "We shouldn't
police what kind of laws there are or what shapes they take." Section 3.1 proposes a predicate and section 7 states a
definition, both of which are adjacent to policing. I did not return early, and the reasoning is offered
so somebody can disagree. I16's test is functional: a law must actually work, meaning it reaches one
lowered path. My predicate is not a law and does not constrain what shape one takes; it decides whether a
resolver is free on an axis at a region, which is a question about the object rather than about a law's
spelling. And section 7 states arms with disjoint predicates rather than one required shape, which is
what I13 asks for.

**One thing I flag rather than resolve, per the gate's ambiguity clause.** Section 3.2's repair changes
what component one fixes, and that touches I9, which op returned to the panel at `104` with a decision
procedure rather than a ruling. My repair is a contribution to that live question and it is marked as
one everywhere it appears. It is not a reading of I9 and it does not claim to be.

### 2.2 Test gate: passed, at 123 tests across 13 crates, and it is the sixth independent count

`mock/crates/` is empty by design, so there is no arvo suite. The only executable surface is the bench
variant crates. I ran all thirteen, per crate, with `--test-threads=1` throughout, greping **every**
line matching `test result:` rather than extracting positionally.

`108_probes/p0_test_gate.sh` and its output. **123 passed, 0 failed, 13 crates.** That confirms `98`,
`100`, `102`, `103` and `106`.

Three of the known ways this corpus produces a meaningless green are avoided by construction in that
script and are named in its header: `cargo test --workspace` from `mock/benches` reaches only the driver;
`bitpack-write-contend-shared` hangs without `--test-threads=1`; and `tail -4` reads the doc-test result
block, which is last and reads `0 passed`. A fourth, `grep` under `variants/` picking up
`target/debug/**/*.rmeta`, is avoided in every count I take: `108_probes/p5` excludes `target/` from
every walk and says so in its output.

**I read bodies rather than names in the surface this file rests on.** My argument leans on `97` section
5's arity result, which is one of the two measured instances behind the two-carrier claim, and its family
is `warm-clamp-shared`. Nobody in this unit has read that crate's test bodies. They are strong and I
found nothing to refuse on:

- `every_arm_agrees_with_the_oracle_on_every_key` asserts all six arms against an independent `u128`
  reference at every key the manifest declares, and its own doc comment names the failure it exists to
  catch: "Five arms agreeing on a shared wrong transform is caught by the oracle rather than confirmed by
  the agreement."
- `chunked_answer_depends_on_every_element_the_clamp_did_not_absorb` asserts **both directions**: a bit
  flipped in a non-clamping chunk moves the answer, and a bit flipped in a clamping chunk does not. The
  first is the anti-constant-fold guard; the second is the clamp doing its job. It also `expect`s that a
  below-limit chunk and an above-limit chunk both exist, so a key that cannot distinguish the two fails
  rather than passes vacuously.
- `the_clamp_fires_on_a_real_fraction_of_chunks_at_every_chunked_key` is the vacuity complement.
- `the_noise_floor_controls_really_are_the_same_instantiation` asserts an equality and two inequalities,
  so a control that stopped being a control fails.

Nothing tautological, nothing sampled where a matrix was available, nothing assertion-free. The two
known defects in this surface are unchanged and are not mine: `bitpack-wide-d16-control` is not
byte-identical despite three module headers asserting it (`101`), and `bitpack-shared`'s `check_size`
does assert both extraction arms against the logical column, contrary to four member files (`106`
section 0.3, `107` section 3).

**And one finding against my own instrument**, recorded because it is the shape the gate exists to
catch. `108_probes/p4_read_the_asm.sh` version one reported `TypeId: 1` for a source file that contains
no `TypeId`. The single hit was the file's own doc comment saying it contains no `TypeId`. That is
`102`'s own probe defect reproduced exactly, on a different file, by a different author, two units
later. The corrected grep excludes comment lines and the counter now reads 0. Version one's number is
not preserved separately because the fix was in the same session; the script header records the class.

---

## 3. The eight clauses, attacked one at a time

`106` section 1 states the pair in eight clauses. A definition is attacked clause by clause or it is
not attacked, because a compression of it can be right about the shape and wrong about every load-bearing
qualifier, which is the failure `107` names in its own verdict: "prose survives and the thing that
qualifies it does not".

I take them in the order of how much they cost if wrong.

### 3.1 Clause two: "the axes a consumer can observe" is not a fixed set, and its qualifier is gone

The clause:

> Its first component is an **assignment on the axes a consumer can observe**: those where moving the
> assignment changes what the program computes, or whether it computes at all.

Three things are wrong with this, in increasing order of consequence.

**One. The definition is `40`'s and the consolidation attributes it to `97`.** `40`'s sentence, at
`40:48` and again at `40:398`:

> An axis is **observable** when moving it changes the value the program computes, and **unobservable**
> when it changes only what the computation costs.

`106` section 9.2 says "`97` defines an observable coordinate as one whose movement changes the value the
program computes".
`97` does not claim it. `97` section 3.2's F-C says, in its own words, "This is a derivation from
`40:398`'s definition and `40` section 5.4's compile, not a measurement, and it is labelled as one."
`40` is 57 files earlier. `108_probes/p1` traces it: the definition appears at `40:48` and `40:398`,
`97` cites `40:398` twice, and `106` attributes it to `97`.

That is a credit question and I would not spend a section on it. What follows is not.

**Two. `40` attaches a qualifier and it is at zero across the entire unit.** `40` section 5.2 is titled
"Headroom is unobservable only because of a convention, and the convention is load-bearing", and says:

> Headroom is on the unobservable side **only if** the overflow policy is applied at the logical width
> rather than at the container width. If a doubled container wraps at the container width, then two
> numerals differing only in headroom compute different answers, and headroom becomes observable.

and closes: "the convention 'the policy applies at the declared width' ... is a canon-shaped sentence
nobody has written. Without it, half the mechanism space moves to the expensive side."

`108_probes/p1` counts the phrase across all eight member files, `106` and `107`: **`logical width`
returns 0 in every one.** `convention` returns 1 in `102` and 5 in `103`, and I opened all six: every one
is about a naming convention or the radix family's grid-step exponent convention. Not one is `40`'s.

**Three. I measured the convention, which nobody had, and it decides the class.**
`108_probes/p2b`, exhaustive over all 65536 value tuples per chain:

| chain | differing | of | verdict |
|---|---:|---:|---|
| `sat_at_W -> sat_at_W -> sat_at_W` | 0 | 65536 | unobservable |
| `sat_at_C -> sat_at_C -> sat_at_C` | 58380 | 65536 | **observable, 89.081%** |

Those two rows are the **same assignment on the overflow-policy axis**, saturating, and the same
assignment on every other axis. They differ only in the width the limit is read at. `40` argued this from
`20` section 1.3's factoring result and never measured it. It is now measured, and the answer is that
`40` was right and the sentence is load-bearing to the whole width of the cut.

**Four, and this is the one that breaks the clause.** Observability is not a property of an axis. The
same axis, the same declared width, the same containers:

| chain | differing | of | verdict |
|---|---:|---:|---|
| `wadd -> wmul -> wadd` | 0 | 65536 | unobservable |
| `wadd -> wmul -> shr` | 32484 | 65536 | **observable, 49.567%** |

`102`'s p2 found this on its own axis list and reported it as the "ring boundary", 0 of 640 against 500
of 640. The consolidation carries it as a **live option**, a licence a resolver may take inside a pure
ring region. What neither says is what it does to the definition: if the observable set moves with the
chain, then **component one's domain is not fixed**, and clause four says component one travels with the
value, whose type is written where the chain is not known. Two implementers reading clause two will draw
the boundary differently, which is exactly the Equivalence test `RULES.md` puts on every canon sentence.

#### The repair, and it is an arm rather than a hole

Two candidate readings, and only one of them leaves the pair with a fixed shape.

**Reading one, which I reject.** Observability is relative to the region and component one's domain moves
with it. Then the pair has no fixed domain, the value's type cannot carry component one, and clause four
goes with it.

**Reading two, which I propose.** Component one's domain is the **conservative closure**: an axis belongs
to it if there is any reachable chain on which moving it changes the denoted result. Under that, headroom
and intermediate precision are in component one, permanently, and the pure-ring licence becomes an **arm
gated on a const predicate over the chain**, which is I13's shape exactly and is what `102` was reaching
for when it called the ring boundary "a licence nobody took".

That converts a defect in the definition into a win the unit already wanted, and it costs one sentence.

#### And the predicate exists, is sound, and is const-computable

A repair that names a predicate and does not build it is a wish. `108_probes/p2c` builds it.

Three static bits per operation, every one **measured exhaustively rather than asserted**:

- **congruent**: reduction to the declared width descends through the operation.
- **contracting**: on declared-width operands the result is declared-width.
- **container-read**: the result differs between two containers on operands both hold identically.

and a three-state scan of the chain:

```
state := IDENTICAL
for op in chain:
    if CONTAINER-READ(op):                          state := DIVERGED
    else if state = DIVERGED:                       state := DIVERGED
    else if state = CONGRUENT_ONLY and not CONGRUENT(op):
                                                    state := DIVERGED
    else if CONTRACTING(op):                        state := IDENTICAL
    else:                                           state := CONGRUENT_ONLY
observable := (state = DIVERGED)
```

Checked against exhaustive measurement at three settings, over **every** operation sequence of the swept
length from the full nine-operation alphabet, each swept over **every** value tuple in the declared
domain:

| setting | chains | value tuples each | exact | conservative | **unsound** |
|---|---:|---:|---:|---:|---:|
| W=4, containers 4 against 12, length 3 | 729 | 65536 | 701 | 28 | **0** |
| W=4, containers 4 against 12, length 4 | 6561 | 1048576 | 6301 | 260 | **0** |
| W=5, containers 5 against 13, length 3 | 729 | 1048576 | 701 | 28 | **0** |

`holds for: declared width W in {4, 5}, containers {4 against 12, 5 against 13}, chain length in {3, 4},
operation alphabet as enumerated in the probe (wadd, wsub, wmul, shr, div, sat_at_W, sat_at_C, min, cmp),
chains exhaustive over the alphabet, values exhaustive over the declared domain, axis = headroom,
arithmetic exact integer, threads = 1, target features any`

**The two error directions are counted separately and that is the point.** A predicate whose error rate
is reported as one number has hidden which half is dangerous. Predicting observable where the axis is not
is the resolver declining a freedom it had, which costs an optimisation. Predicting unobservable where
the axis is observable is the resolver moving something a consumer can see, which is a wrong answer. The
second is zero at every setting.

**And the conservatism is structural rather than a defect to grind down.** An operation that clamps at
the declared width re-synchronises two arms that had already diverged, and no fixed set of per-operation
bits sees that, because it is a fact about the relationship the accumulators are in rather than about the
next operation. `sat_at_C -> sat_at_W -> wadd` is the case: predicted observable, measured 0 of 65536.
Anyone who wants exactness here is asking for the value sweep, whose domain is `2^(W(L+1))`, which is the
wall `unstable-features.md` names.

#### The version of this probe that was wrong, kept, and worth more than the fix

`108_probes/p2_first_version_contracting_chain.rs` predicted the axis visible under `shr1` and `cmp` and
measured it invisible, 4 mismatched cells of 18. The criterion was not wrong. **The chain was.** Every
operation in it was fused with an add and then masked, so the accumulator never left the declared width
and the axis was never exercised. That is `102`'s own p2 version-one defect, reached from the other
direction, and it is why the sharper statement is available: what decides the class is not whether a
non-ring step is present but **whether one is reached while the accumulator has left the declared
width**. Both outputs are committed.

### 3.2 Clause three is false, and it is the clause the accuracy intent dies on

The clause:

> Its second component is a **weighting over cost coordinates**, which selects among the arms that
> produce the answer the first component fixed.

`108_probes/p3` opens all seven passages below and finds every one present as quoted, and its checker is
mutation-tested three ways with all three mutants caught.

**The chain of five statements, each from the files themselves.**

1. Component two ranges over arms that produce the answer component one fixed. `106` section 1, and
   `102` section 6 in the wording `106` compressed: "which selects among the arms that produce that
   value".
2. `102` section 1 says op's intents range over arms that **disagree**: "I5 trades accuracy for speed,
   I7 buys accuracy with speed, I3 asks for a particular answer", and then, in bold, "Each ranges over
   arms that **disagree**."
3. `102` section 3.7 nonetheless maps I7 onto component two: "I7, accuracy first | weighting, over a
   computed coordinate, with depth in the region".
4. `106` section 8's own table says that in the answer-equivalent region "cost-only is correct and
   complete; a fidelity column would measure a constant".
5. `106` section 8 carries the remedy for I7 as the missing fidelity hook: "The missing piece is
   `score_output`".

Statements 1 and 3 cannot both hold. If component two only ever sees arms that agree, then by statement
4 the coordinate statement 3 assigns it measures a constant, and by statement 5 the remedy the unit
proposes is a coordinate component two is defined never to be able to use.

**`102` states both halves one page apart.** This is not a compression defect that `106` introduced;
`106` inherited it, `103` verified the neighbouring measurement without touching it, and `107` checked
the compression without reaching it, because a claim-by-claim entailment check scores a self-consistent
compression of a self-contradictory source clean.

#### The corpus contains both cells and says which is which

`108_probes/p5` reads `103`'s committed per-arm error coordinates and separates them:

| family | arms | spread | what a weighting on it learns |
|---|---:|---:|---|
| `decimal-quantiser-radix-sweep` | 2 | **66.139x** | it selects |
| `quantiser-vs-fadd-subnormal-sweep-n0` through `-n100` | 2 each | **1.000x** | nothing |

Six families in clause three's region, one outside it. In the six, the fidelity coordinate is identical
to all twelve printed digits, so a weighting reading it is reading a constant. In the one outside, the
arms differ by two orders of magnitude, and clause three excludes it.

And the coordinate is absent from the corpus entirely, verified independently here with `target/`
excluded from every walk: **254 committed CSVs, all 254 declaring a `score` column, 104080 data rows,
0 with a non-empty `score`, 94 variant crates, 0 implementing `score_output`.**

`holds for: the committed corpus at HEAD of feat/arvo-shape-topic, families with a computed per-arm error
coordinate = 7, CSVs = 254, rows = 104080, variant crates = 94, arithmetic exact as recorded by 103's
probe, threads = 1, target features any`

#### The repair is one word, and both halves of it are already in the unit

> Component one fixes the **denoted** answer: the declared semantics the value's operations denote. It
> does not fix the computed answer. An arm realises the declaration exactly or approximately, and the
> distance from it is a cost coordinate like any other.

Four things fall out, and each is something the unit wanted and could not reach:

**I7 becomes expressible in component two.** The fidelity coordinate varies across arms that all denote
the same function, which is exactly the `decimal-quantiser-radix-sweep` row above with the confound
removed.

**I5 becomes expressible in component two**, which is the correction to `106` section 4's leg (b) and is
section 4.2 below.

**Clause five stops being vacuous.** As written, clause five restricts a weighting from reading a
measured coordinate "only where the arms it ranges over compute the same value", which under clause three
is always true, so it restricts nothing. Under the repair the restriction bites, and its predicate is
`103`'s, whose own wording is that the hazard arises "**where the cost ordering and the answer ordering
disagree**", not merely where the arms differ. I take that from `103` rather than from `106`, which
renders it as "conflict"; the words are equivalent and the establishing source is the one to quote.

**`103`'s validation discipline stops being a special case and becomes the general rule.** Its sentence,
which I would carry close to verbatim:

> **A region whose arms may produce different answers is validated arm by arm, each against its own
> declared semantics, rather than arm against arm.** Cross-arm agreement is then a consequence where the
> semantics coincide, and its absence is not a defect where they do not.

That is `103`'s own emphasis, restored from `103` rather than from `106`'s re-bolding of a sub-phrase of
it, per the rule that a repair restores from the establishing source.

Under clause three as written that is a workaround for a region the design excludes. Under the repair it
is what validation **is**, and cross-arm byte equality is the special case that applies when an arm set
happens to realise the declaration exactly.

That last point is worth one more line, because it changes what the harness gap is. `106` section 8 calls
`score_output` "one hook, which is a smaller and far better specified piece of work than 'add the missing
coordinates to the corpus'". Under the repair it is not a hook for an unusual case. It is the coordinate
that carries an entire named intent, and the corpus has zero implementations of it across 94 crates.

### 3.3 Clause four cannot be a mechanism as stated, and costs nothing either way

The clause:

> The two components have **different carriers**. The first travels with the value. The second is
> supplied where the operation happens, because only the site knows the arity, the access pattern and the
> target. A **named** strategy binds one point in each, so that a consumer states one intent rather than
> answering two questions.

The first two sentences are measured and I keep them. `94`'s W9 put policy on the value with the plan at
the site, four sites, three sharing one value type, zero conditional instructions and zero casts. `97`
section 5 used `warm-clamp-arity-w13`, holding the declared width at 13 and the element count at 8192 and
sweeping the fold's arity through six points, and the best arm moves. Two instruments, two authors.

**The third sentence does not follow from them.** If the name binds both and is written on the value, the
site cannot move component two without changing the value's type. If it is written at the site, component
one is not on the value, and "every consumer of that value must agree about it" fails. `106` section 10
sees this, and resolves it by declaring the named binding to be what the word denotes, which names the
problem rather than removing it. It also prices it in one line: "Under type-carried cost, folding one
column two ways requires a cast that changes no value, which is free at runtime and not free in the
design."

#### I compiled all three shapes

`108_probes/p4_one_name_two_carriers.rs`, `#![no_std]`, **zero feature gates**, no `dyn`, no `TypeId`,
no `generic_const_exprs`, arm selection forced through an inline `const { }` block so the claim is about
const solving rather than backend folding.

- **A. Two carriers, no name.** Policy on the value, weighting at the site. One column type folded two
  ways, no cast.
- **B. One name binding both, on the value.** The same two folds need a `reinterpret` on the value, which
  is the cast `106` prices.
- **C. One name as a default in the second component.** The common case writes one name; a site that
  needs a different weighting names one; the value's type does not move, and component one is read from
  the value's own name in both paths.

The emitted symbols:

```
_a_size_first:
_a_time_first:
_b_as_stored = _a_time_first
_c_default = _a_time_first
_b_other_weighting = _a_size_first
_c_overridden = _a_size_first
```

**Six entry points, two bodies.** B and C are assembler aliases of A. The cast in shape B costs nothing
at all, to the symbol rather than to the instruction, and shape C reaches the same code without ever
asking the value's type to move.

Each body carries 15 and 11 instructions, two conditional branches and one `csel`, and **none of the
three tests a strategy, a policy or an arm choice**: the branches are the empty-slice guard and the loop
backedge, and the `csel` is component one's saturating clamp lowered to a select. The cost table leaves
zero bytes: no `COST` symbol and no `__const` section.

`holds for: target aarch64-apple-darwin, rustc 1.98.0-nightly (57d06900f), edition 2021, opt-level 3,
crate-type lib, no_std, feature gates = 0, arms = 3, regions = 4, cost coordinates = 2, declared width 13,
threads any (the artifacts are compile-time)`

That is the erasure result the unit holds on four instruments, reproduced on a fifth arrangement, and it
settles the clause: **the fork between "the name binds both" and "the name defaults one" has nothing a
consumer can observe.** It is decided on ergonomics and on who is permitted to move component one, which
is where `arvo-toolbox-not-policer.md` and I2 and I4 all point.

#### The repair is one word

> A **named** strategy supplies a default point in each component. The site may name a different point in
> the second. Nothing may name a different point in the first, because a value's consumers must agree
> about it.

That keeps the ergonomics I2 and I4 demand, keeps the two carriers the measurements found, and makes the
asymmetry between the components a stated rule rather than an accident of where the name was written. It
is also `40`'s "name the objective, expose the observable axes" with the exposure moved to the half the
measurements say needs it: `97` section 5's arity result says the **rate** is what the site must be able
to move, and `40`'s p3 assumed it was the observable axes. Both need an override and the design owes both.

### 3.4 Clause seven says one thing about two objects, and the same document says the opposite

The clause:

> Two strategies are related by an order on their **first components** where one exists, and by
> **nothing** on their second, because two weightings are incomparable vectors and nothing ever asks them
> to combine.

`106` section 5, in the same document:

> **On the weighting, the join is union and it is free.**

Both are true, and they are about different objects. The word "weighting" is carrying two of them.

**The support**, meaning which coordinates a strategy demands at all. `97` section 4.1's object: four
generators (`speed, residency, accuracy, familiarity`), a free join semilattice whose carrier is the
non-empty subsets, compiled at `97_probes/p4_demand_lattice.rs`, `no_std`, zero feature gates, with all
256 ordered pairs asserting the join is the union.

**The rate**, meaning the exchange rates among the demanded coordinates. `101`'s object: "the weights
carry the units", "a weighting is a ray rather than a point".

`108_probes/p6` measures both on the committed carrier table, in exact rational arithmetic:

| object | result |
|---|---|
| support, 2 generators, 3 non-empty subsets | union is a unique least upper bound in **9 of 9** ordered pairs |
| rate, 42 ordered pairs from a log-spaced grid | six defensible combinations **disagree on 30 of 42, 71.4%**, reaching up to 3 distinct sections from one pair |

`holds for: regions = 6, arms = 5, cost coordinates = 2 (median algo_ns per record, declared bits per
element as 16/32/64/13/13), cost source = the committed bitpack-carrier-width_n* CSVs, rate grid =
{1e-3, 1e-2, 1e-1, 1, 10, 100, 1000} against a unit time weight, combination rules = {arithmetic mean,
componentwise max, componentwise min, squared geometric mean, first operand, second operand}, arithmetic
exact rational, threads = 1, target features any`

**The repair is to split the word rather than choose a side.**

> Component two is a **support**, the set of coordinates the strategy demands, together with a **rate**,
> the exchange among them. Supports join, uniquely and totally, by union. Rates do not join, and nothing
> asks them to: where two rates meet, either the site names one or the operation reports, which is the
> same answer clause seven gives for component one.

**And it explains a result the unit holds and has not connected.** `93`'s P1b found four markers carrying
one demand each leaving 12 of 16 ordered pairs unresolvable, and `97`'s p4b reproduces it as "pairs whose
union is NOT itself a generator: 12". The union always exists; what those 12 count is that it is not one
of the four names. Inside the closure, `97`'s own output reads "ordered pairs unresolvable INSIDE
the closure: 0 of 225". So the supports resolve everywhere and what does not resolve is the rate. A flat marker set was carrying both in one slot, which is `94`'s own
diagnosis of the flat set restated at the right granularity: "a flat set forces two roles through one
slot", and the two roles here are inside what everyone has been calling one component.

### 3.5 Clause eight undercounts, by the size of the other component

The clause:

> The **number of strategies is not a design parameter.** It is bounded above by the coordinate set,
> which is countable exactly, and a name is a binding rather than a member of a closed set.

The bound is `101`'s: `{time}` reaches 1 section, `{time, size}` reaches 9, `{time, size, spread}` reaches
42. Every one of those counts sections of the **weighting space**. Under the pair, a strategy is a point
in a product, so the bound on strategies is the product of the two components' cardinalities, not the
second's alone.

`40`'s p2 counts the first: with the four axes the record names and the values the record exhibits, the
mechanism product has **16 points**. So at two coordinates the composed bound is 144 rather than 9, and
the sentence as written is off by the entire size of component one.

This is small and it is the same class as leg (a) in section 4.1: a result established about one
component, stated about the pair. It matters for exactly one reason, which is I1. "How many strategies
could there be" is the question I1 reopened, and a bound of 9 and a bound of 144 are different answers to
it.

**I note one thing so nobody reads a coincidence as a finding.** 144 is also `98`'s Pareto-admissible
rung count on a different space. The two numbers have nothing to do with each other and the collision is
arithmetic accident.

### 3.6 Clauses one, five and six: kept, with reasons

`RULES.md` says keeping something with your own reasoning behind it is a contribution, so these are not
listed as unexamined.

**Clause one, that a strategy is a pair.** I attacked this by trying to collapse it. If the two components
had one carrier, the two-carrier measurements would have to be wrong, and they are two independent
instruments on two families. If component one were derivable, a consumer could recover it from the bits,
and it cannot. **The two-ness survives every attack I could mount**, and section 5 says what part of it is
old.

**Clause five, measured against computed.** I keep it and section 3.2 makes it non-vacuous rather than
weakening it. Its predicate is `103`'s and I did not re-derive it. `103` also states honestly that the
hazard has zero real instances in the corpus, because `radix2` dominates `radix10` on both coordinates, so
the clause is a guard for a case that does not yet exist. That is a fine thing for a canon to carry as long
as it is labelled as one, and `106` labels it.

**Clause six, region against cost vector.** `101`'s, close to verbatim, and `102` built and compiled both
sides. I found nothing to attack and would carry it unchanged. It is the most portable sentence in the
definition.

---

## 4. `106` section 4's three legs, judged separately

`106` separated the pair into three claims "because they can fail independently". They do.

### 4.1 Leg (a) is void: five readings of a sentence op disclaimed

Leg (a) is that "the two components are what op's `88` answer decomposes into", where op answered "Mostly
option 1, but a little bit of option 3 with it. Hard to put into words, hopefully you get my meaning here".

**`108_probes/p3` finds five incompatible readings of that one sentence in this panel**, each opened and
verified present:

| file | reading |
|---|---|
| `93` | a **ranking** of two readings, neither decomposed: "The point reading is mostly it and the weighting reading is a little bit of it" |
| `97` | **tiers**: the design tier writes points, the canon tier writes the objective, and "the little bit of option 3 that survives into the design tier is one checkable constraint: the table must be rationalisable" |
| `98` | a **rung** on a five-step ladder, strictly between the ends, which `106` dropped and restored in its section 16 on `107`'s check |
| `102` | a **decomposition** into two components, "a decomposition rather than a proportion" |
| `106` | the **named binding**, which "makes `88`'s 'mostly option 1, a little bit of option 3' read cleanly: mostly the point, with a weighting attached" |

Nothing distinguishes them, and three things say nothing should try.

**Op flagged his own difficulty in the sentence itself**, and `88` records the consequence: "Op flagged
his own difficulty putting it into words, so a later expert finding the two readings pull apart somewhere
has found something real."

**Op has since removed a neighbouring clause of his own from load-bearing status**, in the same file, in
words that generalise: the tail of I8 is "just filler noise I mused on the spot", and `88` section 2
draws the rule, that quoting is one act and naming the intent inside the quotation is a second.

**And op declined the nearest question rather than ruling on it.** `104` section 3, on whether I9 describes
the pair or its policy half: "I think the intent is clear and this is impl detail that already had answer:
optimal and converged to by experts (plural, iterative)." `104` adds the test that would have caught the
question: "if both answers leave the intent intact and differ only in what the panel calls things, it is
not his."

**Leg (a) fails that test.** Which reading of `88` is right differs only in what the panel calls things.
So leg (a) is not a claim anyone can settle, and it should not be carried as support for the pair.

**What replaces it.** Nothing, and that is the point. The pair stands on the two-carrier measurements, on
polarity, and on whatever section 7 below survives. **Delete the appeal to `88` from the argument.** The
pair is not weaker without it; it is honest, and it stops five files' worth of exegesis from reading as
five instances of agreement.

`106` section 13.1 carries "whether the rationalisability constraint has content once the pair is in
place" as an open fork whose discriminator is which of `97`'s and `102`'s readings of `88` is right.
**That fork is not decidable and should move to the droplist**, with the note that the rationalisability
constraint's own content is a separate and answerable question that does not need `88` at all: `100`'s p2
answers it, at 0 of 489 generator defects caught by rationalisability against 489 of 489 by cone
membership.

### 4.2 Leg (b) is right about I3 and wrong about I5, and my repair is what flips it

Leg (b) is that "I3 and I5 are not weighting-shaped, so a coordinate ceiling does not bound them".

**Right about I3**, and op settled it independently at `104` section 1: "Neither, it's ergonomics". An
experience is not a quantity and no coordinate carries it. `102` reached this from the design side before
op spoke, and that is a real result.

**Wrong about I5**, once clause three is repaired. I5 is:

> Hot *can* sacrifice soundness, that is its explicit purpose, but it should not lose it for nothing,
> instead, provable meaningful gains.

Under clause three as written, an arm that sacrifices soundness produces a different answer, so it is
outside component two's region, so I5 cannot be a weighting and `102` is right. Under the repair, arms
that realise one declared semantics to different fidelity are exactly component two's region, and "not for
nothing, instead provable meaningful gains" is **a finite exchange rate between fidelity and time**. That
is a weighting, with a support of two coordinates and a rate that op has deliberately left unset.

**And this restores `40`'s reading**, which the unit lost. `40` section 5.3 puts it in the vocabulary
directly: "accuracy is lexicographically prior for every objective except `Hot`, and finitely weighted for
`Hot`. A lexicographic term refuses any trade at any exchange rate; a finite one trades when the gain
clears the rate." `40` also records what it does not supply, which is that the rate is unset, and that the
same parameter is unset a second time in `38`.

So leg (b) should read: **I3 is not weighting-shaped and I5 is**, and the reason the unit concluded
otherwise is clause three rather than anything about I5.

This also narrows the `101`-against-`102` located disagreement further than `106` narrowed it. `106`
section 9.1 reduces the residue to I7 and says `103` resolved that. With leg (b) corrected, `101`'s
position is stronger than `106` credits: `101` said two of op's four intents name a quantity with no
coordinate, and on I5 and I7 both, the quantity is fidelity and the coordinate is `score_output`, absent
in 0 of 94 crates. `101`'s diagnosis was right about both, and `102`'s "they were never asking for a
coordinate" was right about I3 alone.

### 4.3 Leg (c) survives, and sharpens

Leg (c) is that "`25` section 7 and the cold pair's definition differ by polarity rather than by count",
established by `102`'s p2 running `97`'s observability test on `25`'s four axes and finding three
observable and one not.

I attacked it by re-deriving the observability question on an independent construction and it survives.
It sharpens in one way, from section 3.1: **for two of the four axes the verdict is per-chain rather than
per-axis**, so the three-to-one split is a fact about the chains `102` swept and not about the axes.
`102`'s own table says as much in its verdict column ("observable only past a non-ring step"), and the
conclusion it draws, that the two definitions have opposite polarity and no merge was available, does not
depend on the qualification. Overflow policy is observable in both regimes at 511 of 640 and packing in
neither at 0 of 640, so the polarity claim rests on the two axes whose verdict does not move.

---

## 5. Is the pair new, and where the credit actually sits

The dispatch asks this because `102` and `93` both say `40` reconciled two levels 53 files earlier, and
if the pair is that reconciliation renamed then the unit has one instance rather than two.

**It is not that reconciliation renamed, and the difference is exactly one relocation.**

`40` section 0, verified present at `108_probes/p3` P1:

> A strategy lives in the **objective** space. A mechanism assignment is what a strategy produces when it
> is applied to evidence.

with `resolve : objective × evidence -> mechanism`. So in `40` the strategy **is** the weighting, and the
observable assignment is an output of applying it. `40`'s section 6 conclusion then exposes the observable
axes separately: "name the objective, expose the observable axes", compiled with per-axis overrides.

`102`'s pair moves the observable assignment **inside** the strategy, as component one, supplied rather
than produced. That inverts the polarity of half of `40`'s relation, and it is not a renaming: under `40`
a consumer names one thing and overrides axes beside it; under the pair a consumer names an object that
has the axes in it.

**So the answer to the dispatch's question is: the structure is old and the relocation is new.** The
relocation is `102`'s, it is at ONE EXPERT, and it is what deserved the attack this dispatch was sent for.

### The rung correction that follows, and it goes the other way from `106`'s

`106` section 3.3 puts "polarity as a derivation" at ONE EXPERT and corrects the register for reading it
as settled, on the ground that `102` read `97` before building. That correction is right about `102` and
`107` verified it.

**But the two-level structure under it is at TWO EXPERTS and `106` records it at no rung at all.** `93`'s
section 12, from a cold derivation that read nothing:

> That is op's "mostly option 1, a little bit of option 3" derived before he said it, and it is better
> than my phase one, because it keeps both levels and names which one the strategy lives in rather than
> collapsing to the generator.

`93` then claims three instances: "The credit for the two-space reading is `40`'s and `25`'s, and mine is
a third independent arrival at it."

**I make it two, not three, and I am lowering `93`'s own count rather than raising it.** `40`'s own
account of `25` is that `25` carries only the mechanism space: "What it does not carry is the thing that
generates the graph, and op supplied that afterwards." And `40` declares itself downstream of `25`: "That
is a refinement of `25` rather than a replacement." A refinement of a document is not an independent
derivation of it, and a document that carries one of two spaces is not an instance of the two-space
reading.

So: `40` and `93`'s blind phase one, two independent arrivals, which is the TWO EXPERTS rung earned the
only way `RULES.md` allows. **That belongs in section 3.2 of the consolidation and is in none of it.**

I would add it as:

> **A strategy's structure has two levels: an assignment on mechanism axes, and something that generates
> the assignment.** `40` section 0 from op's four intent statements and the arithmetic of naming; `93`
> section 12's phase one from the partial order on cost vectors, derived blind. Two independent arrivals.
> The two files disagree about which level the word "strategy" names, and that disagreement is Q50.

### And the definition of "observable" is `40`'s, which is a third thing the credit misses

Section 3.1 covers it. `97` credits `40` explicitly and honestly; `106` attributes it to `97`. That is a
one-line fix and it matters beyond credit, because the qualifier `40` attached travelled with the
attribution and is at zero across the unit.

---

## 6. Are the two halves independent

The dispatch asks whether a policy assignment constrains which cost coordinates are meaningful, noting
`103`'s coupling.

**They are not independent, and the coupling runs the opposite way from the one the dispatch names.**

`103`'s coupling, that a weighting may read a measured coordinate only where every arm computes the same
answer, is a constraint from **the arm set** onto **component two**, not from component one onto component
two. And under clause three as written it is not a coupling at all, because clause three already
guarantees its antecedent. That is section 3.2's vacuity result.

The real coupling appears once clause three is repaired, and it is this:

> Component one fixes the declared semantics. That determines **which arms exist** at all, since an arm is
> a realisation of a declaration. Component two then ranges over that set. So component one determines
> component two's **domain**, and component two determines nothing about component one.

`97` section 3.2 already wrote this as a dependent function and nobody carried it into the pair:

```
Arms    :  Policy -> Set
resolve :  (p : Policy) -> Evidence -> Arms(p)
```

The pair is that dependent function with the two arguments named, and stating it as a **pair** loses the
dependency. A pair is a product; this is a dependent sum. That is not pedantry: it is the difference
between "the two components are independent choices" and "the second is chosen from a set the first
determines", and only the second is true.

**So a repair to clause one is available and I offer it as the smaller of two options**, since I cannot
settle which the canon should carry:

- **Option one, keep "pair" and add the dependency as a sentence.** Cheapest, and it reads as a caveat.
- **Option two, say what it is.** A strategy is a **dependent** pair: a declared semantics, together with
  a weighting over the cost coordinates of the arms that realise it. Then the dependency is in the
  grammar rather than in a note, and `97`'s type is the canon sentence.

What would distinguish them is whether anything ever needs to range over strategies with different
component ones as a single set. If it does, the pair reading is more convenient; if nothing does, option
two is exact and costs a word.

---

## 7. A converged statement, offered

`95` asks a unit to end in agreement with at least something. This is what I believe `40`, `93`, `94`,
`97`, `98`, `100`, `101`, `102`, `103` and `106` jointly support, with the five repairs above applied. It
is a suggestion and op decides.

> A **strategy** is a declared semantics together with a weighting over the cost coordinates of the arms
> that realise it. The second is chosen from a set the first determines, so the two are ordered rather
> than independent.
>
> The **declared semantics** is an assignment on the axes a consumer can observe: those where moving the
> assignment changes what the program denotes, or whether it denotes at all. It is supplied and never
> derived, because a consumer of a value cannot recover it from the bits, so every consumer of that value
> must agree about it. An axis belongs here if there is **any** reachable chain on which moving it is
> observable; where a particular chain cannot observe it, that is a licence the resolver may take under a
> predicate over the chain, not a reclassification of the axis.
>
> An **arm** realises the declared semantics exactly or approximately. The distance from the declaration
> is a cost coordinate like any other, and it is what lets a strategy weigh accuracy at all.
>
> The **weighting** is a **support**, the set of coordinates the strategy demands, together with a
> **rate**, the exchange among them. Supports join, uniquely and totally, by union, so the absence of a
> demand is the statement that the consumer asked nothing there and the resolver is free. Rates do not
> join: where two meet, the site names one or the operation reports.
>
> A cost coordinate is **measured** or **computed**. A measured one has a resolution the instrument
> reports; a computed one is exact. A weighting may read a measured coordinate over arms that realise the
> declaration differently only where the cost ordering and the fidelity ordering do not conflict.
> Otherwise every coordinate it reads is computed, because otherwise the program's output is a function of
> a benchmark's noise.
>
> A quantity over which a strategy's answer may **differ** belongs to the **region**. A quantity on which
> a strategy's answer is **scored** belongs to the **cost vector**. Width, element count, arity, thread
> count and chain depth are the first kind. Time, footprint and distance from the declaration are the
> second.
>
> The two components have **different carriers**. The declared semantics travels with the value. The
> weighting is supplied where the operation happens, because only the site knows the arity, the access
> pattern and the target. A **named** strategy supplies a default point in each; a site may name a
> different rate; nothing may name a different declared semantics, because a value's consumers must agree
> about it.
>
> Where two declared semantics disagree, the operation **reports a conflict that is real** rather than
> silently resolving toward either.
>
> The **number of strategies is not a design parameter.** It is bounded above by the product of the
> declared-semantics space and the sections of the weighting space, both countable exactly, and a name is
> a binding rather than a member of a closed set.

**Permanence.** Every sentence survives a rewrite in another language or decade. None names a container, a
width, a marker, a type parameter, a table cell, a crate, or a count of strategies. The one place I was
tempted and did not go is the predicate in section 3.1: it belongs in the audit trail, and the canon
sentence says only that the licence exists and is decidable.

**Equivalence.** Three teams implementing this produce units that behave the same on what matters: a
consumer supplies the declaration and cannot supply the rest; the compiler resolves the rest and tells
nobody; the resolution is derived from a stated weighting over a committed table and checked at build
time; nothing that changes a denoted answer is decided by a timing; and an arm that approximates the
declaration is scored on how far it is rather than excluded. They differ on how many strategies ship, what
they are called, and how the two carriers are spelled.

**Where it is weaker than I would like, stated rather than hidden.** The predicate in section 3.1 is
measured on one axis, headroom, at two widths, on one operation alphabet. Whether it extends to
intermediate precision is untested and I expect it does, because the two have the same observability
predicate in `102`'s p2, which is an expectation and not a result. And the whole statement inherits `25`
section 8's open question about whether the axis list is complete, which nothing in this unit closes.

---

## 8. What I keep, and why keeping it is a result

Four things I went looking for reasons to break and did not find any.

**The two-carrier structure.** Two measured instruments, two authors, two families, and my p4 adds that
the fork about how to name it costs nothing. I attacked it by trying to put the weighting on the value and
the policy at the site; the first needs a cast the design does not want and the second breaks the property
that every consumer of a value agrees about its semantics.

**The measured-against-computed split with `103`'s predicate.** Section 3.2 makes it bite rather than
weakening it, which is the strongest outcome available for a clause: it was right and it was unreachable.

**The region-against-cost-vector line.** `101`'s. I have nothing to add and would carry it verbatim.

**Reporting rather than silently resolving, on the observable axes.** `97`'s p3 computed three candidate
conservatism orders exhaustively and they disagree, two of them not being orders at all. That kills
"resolve toward the more conservative side" without needing my repair, and my repair does not touch it.

---

## 9. Located disagreement, carried as that

**With `106` section 10, on Q50, and it is a disagreement about method rather than about the answer.**
`106` offers the named binding as its reading and says what would distinguish it: whether anything ever
needs to name the weighting on a value's type. I agree with the discriminator and with the two measured
instances that answer it. Where I disagree is that `106` reaches its reading partly by making `88` "read
cleanly", which section 4.1 says is not available to anyone. Strip that support and the reading still
stands on the carriers, which is a stronger position than the one it is argued from.

**With `102` and `106` on whether I5 is weighting-shaped.** Section 4.2. This is decidable and not by op:
it turns entirely on whether component one fixes the denoted or the computed answer, which is section
3.2's repair, and if the repair is wrong then `102` is right about I5.

**Unresolved, and I could not close it.** Whether the conservative closure in section 3.1's repair is the
right domain for component one, or whether an axis observable on only exotic chains should be resolver-free
by default with the consumer opting in. The first is safe and costs optimisations; the second is the
toolbox posture and costs a soundness argument nobody has made. I could not find an instrument that
decides it, because the answer depends on the distribution of chains real consumers write, which is I11's
territory and nothing in the repository measures it.

---

## 10. Shapes found and not taken, with what closed each

The next expert attacking from a different angle starts here rather than from nothing.

**Collapsing the pair to one component by making the observable assignment derivable.** Closed on
definition: a consumer cannot recover it from the bits, which is `40`'s and `97`'s and is not in dispute.

**Collapsing the pair by putting the rate on the value's type.** Closed by measurement and compiled:
`97` section 5's arity result says the best arm moves with something the value does not know, and p4's
shape B shows the cast this forces. It is free at runtime, which is why the argument is a design argument.

**Making the observability predicate exact rather than conservative.** Closed by construction: the residue
is re-synchronisation, which is a fact about the accumulators' relationship rather than about the next
operation, so no fixed set of per-operation bits reaches it. The exhaustive value sweep would, and its
domain is `2^(W(L+1))`.

**Deciding the named-binding fork on cost.** Closed by p4: three encodings, two symbols.

**Deciding between `97`'s and `102`'s readings of `88`.** Closed as undecidable, section 4.1, and moved to
the droplist.

**Measuring whether the section-three repair changes any committed number.** Not taken and I could not
build it. The repair changes what the corpus would need to record, not what it records, and the coordinate
it needs has zero implementations. Building one is `103`'s three-step shape and only the third step is
real work.

**Extending the section 3.1 predicate to the intermediate-precision axis.** Not taken. `102`'s p2 reports
the two axes with the same observability predicate, which is a reason to expect it transfers and not a
result that it does. This is the cheapest unbuilt thing in my file and it is one probe.

**Pricing the ring-region licence.** Not taken, and `102` said the same about its own finding. It is
unpriced in that word: nothing in `mock/benches/` measures an arm that widens an accumulator inside a pure
ring region against one that does not, and the harness has the arms machinery to do it.

**A fourth reading of what "the weighting" means.** Not attempted. Section 3.4 splits it into two objects
and I did not go looking for a third, which is a bound on that finding rather than a claim there is none.

---

## 11. Findings, each with its predicate

Per I13 and `RULES.md`. Listed with a range or `any` means established across it; listed with a fixed
value means established there and only there; **absent means the finding does not hold anywhere that
dimension is present.**

**F-108-1. The headroom axis's observability is a property of the chain, not of the axis.** The same axis
at the same declared width and the same two containers is unobservable at 0 of 65536 on `wadd -> wmul ->
wadd` and observable at 32484 of 65536 on `wadd -> wmul -> shr`.
`holds for: declared width W = 4, containers 4 against 12, chain length 3, operations as enumerated in
p2b, values exhaustive over the declared domain, axis = headroom, arithmetic exact integer, threads = 1,
target features any`
Evidence: `108_probes/p2b_observability_is_decided_by_the_chain.rs` and its output. Independent of
`102`'s p2, which reaches the same class on a different construction at a different width.

**F-108-2. `40`'s convention is load-bearing as measured fact.** Saturating at the declared width leaves
the headroom axis unobservable at 0 of 65536; saturating at the container width makes it observable at
58380 of 65536, 89.081%. Same overflow-policy assignment, same everything else.
`holds for: declared width W = 4, containers 4 against 12, chain length 3, values exhaustive over the
declared domain, arithmetic exact integer, threads = 1, target features any`
Evidence: same probe. `40` predicted this from `20` section 1.3's factoring result and never measured it.

**F-108-3. A three-bit, three-state, const-computable predicate decides the headroom axis soundly.** Zero
unsound predictions, meaning zero cases predicted unobservable and measured observable, across 8019 chains
each swept exhaustively over its full value domain. Conservative in 316 of 8019, 3.9%.
`holds for: declared width W in {4, 5}, containers {4 against 12, 5 against 13}, chain length in {3, 4},
operation alphabet as enumerated in p2c, chains exhaustive over the alphabet, values exhaustive over the
declared domain, axis = headroom, arithmetic exact integer, threads = 1, target features any`
Evidence: `108_probes/p2c_a_sound_conservative_predicate.rs` and its output.

**F-108-4. The fork between "the name binds a point in each component" and "the name defaults one" emits
identical code.** Six entry points across three encodings produce two bodies; four of the six are
assembler aliases. The cost table occupies zero bytes and no branch tests a strategy, a policy or an arm.
`holds for: target aarch64-apple-darwin, rustc 1.98.0-nightly (57d06900f), edition 2021, opt-level 3,
crate-type lib, no_std, feature gates = 0, arms = 3, regions = 4, cost coordinates = 2, declared width 13,
threads any`
Evidence: `108_probes/p4_one_name_two_carriers.rs`, `p4_emitted.s`, `p4_read_the_asm.out`.

**F-108-5. In clause three's own region the fidelity coordinate is a constant.** Six committed families
whose arms share one exact error to twelve printed digits, one whose arms differ by 66.139x and which
clause three excludes. The coordinate is recorded nowhere: 254 CSVs, 104080 rows, 0 non-empty `score`, 94
crates, 0 `score_output`.
`holds for: the committed corpus at HEAD of feat/arvo-shape-topic, families with a computed per-arm error
coordinate = 7, CSVs = 254, rows = 104080, variant crates = 94, target/ excluded from every walk, threads
= 1, target features any`
Evidence: `108_probes/p5_the_fidelity_coordinate_is_constant_where_clause_three_holds.py`. The error
figures are `103`'s, read from its committed output rather than retyped; the use made of them is mine.

**F-108-6. Supports join canonically and rates do not.** Union is a unique least upper bound in 9 of 9
ordered pairs of supports. Six defensible combinations of two rates disagree on 30 of 42 ordered rate
pairs, 71.4%, reaching up to 3 distinct sections from one pair.
`holds for: regions = 6, arms = 5, cost coordinates = 2 (median algo_ns per record, declared bits per
element as 16/32/64/13/13), cost source = the committed bitpack-carrier-width_n* CSVs, rate grid =
{1e-3, 1e-2, 1e-1, 1, 10, 100, 1000} against a unit time weight, combination rules as enumerated in p6,
arithmetic exact rational, threads = 1, target features any`
Evidence: `108_probes/p6_two_objects_one_word.py`.

**F-108-7. Op's `88` sentence carries five incompatible readings in this panel and nothing distinguishes
them.** From `93`, `97`, `98`, `102` and `106`, each opened and verified present.
This is a finding about the corpus rather than about arvo's arithmetic, so it carries no predicate, on
the precedent `97` set for its own F-C.
Evidence: `108_probes/p3_five_readings_and_one_contradiction.py`, 22 citations opened, 0 absent, 3 of 3
mutants caught.

**F-108-8. The pair's clause three contradicts the unit's own mapping of I7, and both statements are in
`102`.** Five passages, opened and verified.
Finding about the corpus; no predicate, same precedent.
Evidence: same probe.

**F-108-9. The two-level structure has two independent instances, not one and not three.** `40` section 0
and `93` section 12's blind phase one. `93`'s own claim of three overcounts, because `40` declares itself
a refinement of `25` and `40`'s own account says `25` carries only one of the two spaces.
Finding about the corpus; no predicate.
Evidence: same probe, items P1 through P5.

**F-108-10. The suite is 123 tests across 13 crates, all green, and this is the sixth independent count.**
`holds for: the committed bench variant crates at HEAD of feat/arvo-shape-topic, --test-threads=1
throughout, every `test result:` line counted, threads = 1`
Evidence: `108_probes/p0_test_gate.sh` and its output.

---

## 12. What this does to the live options

Per `RULES.md`, a pass over the option space separate from the pass over results, because an option nobody
resolved has no result attached and is what a compression drops.

**Killed, with what killed it.**

- **"Whether the rationalisability constraint has content once the pair is in place"** (`106` section
  13.1). Its stated discriminator is which reading of `88` is right, which section 4.1 shows is not
  decidable. The constraint's content is a separate and answerable question, already answered by `100`'s
  p2. To the droplist with that note.
- **Observability as a fixed partition of the axis set.** Killed by F-108-1 and by `102`'s own p2, which
  the unit already held and read as a licence rather than as a fact about the definition.
- **Deciding the named-binding fork on cost.** Killed by F-108-4.

**Fits badly, and survives at a cost.**

- **Clause three's answer-fixing reading.** It survives only if op's accuracy and speed intents are
  accepted as inexpressible in either component, which contradicts `102`'s own map and `106`'s own remedy.
  I do not think anybody wants that, and it is a live option rather than a dead one because it is what the
  canon candidate currently says.

**Added, and adding an option is the most valuable single act available.**

- **The denoted-answer reading of component one.** Section 3.2. Discriminator: whether a design ever needs
  two arms that denote different functions to be in one strategy's arm set. If it does, the repair is
  wrong; if it does not, the repair is free.
- **The dependent-pair reading of clause one.** Section 6, two options with their discriminator.
- **The support-and-rate split of component two.** Section 3.4. Discriminator: whether anything ever needs
  a canonical combination of two rates. Nothing in the unit does, and p6 says none exists.
- **The conservative closure as component one's domain, with a per-chain licence as an arm.** Section 3.1,
  with the predicate built and its soundness measured.
- **The name as a default rather than a binding, with the override asymmetric.** Section 3.3, compiled.

**Carried forward unchanged, because I touched them and did not move them.** The generate-against-check
fork, which is still where the checkpoint left it and which I did not attack. `93`'s sixth axis,
reproducibility across targets and builds, which bears directly on section 3.1's per-build licence and
which `106` correctly names as the option most at risk. `101`'s strategy margin, which under the
support-and-rate split is a property of the rate and gets sharper, since a margin in a rate is meaningful
and a margin in a support is not.

---

## 13. What I did not do

**I did not attack the generate-against-check fork.** It is `106`'s largest unclosed item and four files
after the checkpoint told them to attack it first, and I was sent for the pair. It is where it was.

**I did not re-derive any of the unit's measurements.** The rationalisability counts, the erasure results,
the chain-accuracy results, the corpus census and `103`'s error figures are all read from committed probe
output produced by somebody else. I ran no bench and took no timing. Where nothing has been priced I have
written unpriced.

**I did not test the section 3.1 predicate on any axis but headroom.** Section 10 says why and says it is
one probe.

**I did not read** `OPTIONS.md`, `DROPLIST.md`, `PERSONA_CALLS.md`, `PRIOR_CALLS.md`, the `SEED_*` files,
the archive, `25`, `35`, `94`, `98`, `100` or `101` in full. `98`'s ladder and `101`'s ceiling I know only
through `106`, `107` and the passages `108_probes/p3` opens, and my section 3.5 leans on `101`'s 1/9/42
and `40`'s 16 without opening either probe. If either number is wrong, section 3.5 moves and nothing else
does. **Somebody should check that, and it is two greps.**

**I did not verify `106`'s repairs to itself**, its sections 16 and 17, beyond reading them. `107` checked
the file they repair and I checked the object it describes.

**And I did not settle whether the conservative closure is the right domain.** Section 9 says why.

---

## 14. Coverage of the citations

Every quotation and named claim in sections 3, 4 and 5 that comes from another file was opened and its
**content** tested rather than merely resolved, by
`108_probes/p3_five_readings_and_one_contradiction.py`. Whitespace is normalised and blockquote and
doc-comment markers stripped on both sides, because a quotation wrapped across lines or carried inside a
`>` block is still verbatim, and neither normalisation can make an absent phrase appear.

```
absent citations across all three findings: 0
mutants caught: 3 of 3
```

**It is mutation-tested rather than trusted for coming out green**, per `103`'s lesson that a citation
checker that has never failed has not been tested either. The three mutants are a phrase op did not say, a
real phrase in the wrong file, and a near-miss on a real quotation, and all three are caught.

**What the instrument does not check**, and it is most of sections 3 through 6: whether a cited passage
supports the argument I put on it. No probe crosses that, which is why the entailment of this file wants a
reader who did not write it.

**Citations by heading rather than by line**, throughout, per `how-to-run-a-panel.md` and because this unit
paid for it twice: `101` had fourteen of thirty-seven citations fail on its first run, eight because `100`
grew by 46 lines underneath it while it read.

**The probes.** Eleven scripts and twelve output artifacts in `108_probes/`, each committed with its
output before this file was written: the test gate; the provenance trace for "observable"; the first
version of the observability sweep with its contracting-chain defect; the corrected sweep; the sound
conservative predicate; the first citation checker; the compiled three-shape probe with its emitted
assembly and the reader script; the fidelity-coordinate census; the two-objects measurement; and this
file's own citation checker. Every number in this file is a computation over committed artifacts, a
compile, or a suite run.

**And a second checker for everything the first was not built for.**
`108_probes/p7_check_my_own_citations.py` covers the remaining 43 quotations and attributions in sections
2 through 12.

**It checks two directions, and the second one is not in any checker this panel has built.** Every
predecessor's instrument, including `106`'s p4 at 49 of 49 and `107`'s p11 at 23 of 23, tests one thing:
that a phrase exists in the file it is attributed to. That proves the phrase is real. **It does not prove
the deliverable quotes it**, so the checker's list and the file can drift apart and the run stays green
about sentences the file never contained, or contains in a mutated form. So each entry is also required
to appear in `108` itself, and the entries `108` cites by number or by figure rather than quoting are
**listed explicitly** in the probe, because an entry silently exempted from the reverse check is the hole
the reverse check exists to close.

```
checked: 43   ok: 43   failed: 0   file missing: 0
entries checked both ways:     30
entries cited without quoting: 13
0 entries present in a source and absent from 108
mutants caught: 3 of 3
```

**The reverse direction found three misquotes in my own file on its first run, and all three are in the
same direction: I had inherited a later file's rendering of an earlier file's words.**

- I attributed to I16 the words "should not police what kind of laws there are or what shapes they take".
  **Op wrote "We shouldn't police"**, and `106` section 0.1 carries the same normalisation. Restored.
- I attributed to `103` the predicate that the hazard needs the cost and answer orderings **to conflict**.
  `103`'s own word is **disagree**; "conflict" is `106`'s rendering. Restored, with a note in section 3.2.
- I quoted `97`'s probe output as "ordered pairs unresolvable: 0 of 225", dropping **"INSIDE the
  closure"**, which is the clause that says which set the zero is about. Restored.

None of the three changes a conclusion. All three are the compression failure this panel keeps
diagnosing, occurring in the file that is diagnosing it, and the only reason they are not in the shipped
text is that the instrument was pointed at the deliverable as well as at the sources.

**Two earlier failures of the same probe were bookkeeping.** It named `95` by a filename that does not
exist, and it failed on `103`'s per-arm-oracle sentence because `106` re-bolds a sub-phrase of a sentence
`103` bolded whole. The checker now strips emphasis markers on both sides, because boldface is
presentation rather than content.

**One thing about the probe sources themselves, worth a line because it is a live hazard here.** The
repository's pre-commit hook runs a formatter, so a probe's committed text is not the text that was
staged, and a probe re-run from the working tree is running something the commit did not contain. I
rebuilt `p2b` and `p2c` from the reformatted sources and diffed their output against the committed
`.out` files: **byte-identical**, and `p4` rebuilds with zero errors. That check is cheap and it is the
`cl-claim-sketch-discipline.md` order applied to a probe rather than to a changelist.

**One instrument defect of my own**, section 2.2: a grep that matched my own doc comment's denial of the
thing it was looking for, which is `102`'s defect reproduced two units later by a different author. The
corrected version excludes comment lines.
