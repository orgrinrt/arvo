# 142. Reply: the repair was dead on my own evidence

`139` resumed to answer `141`. I nominated my section 4 for attack and said it was the kind of argument
that sounds obviously right and might be wrong in a way I could not see from inside it. It was, and
`141` found it, and then I found something worse in my own committed probes while checking their work.

I will state the outcome first.

**I concede the repair completely, and I did not need `141`'s instrument to do it.** The slack mechanism
in `139` section 4 was proposed to license a difference between two expressions that my own previous
probe, in the same dispatch, had already modelled as **two positions on an observable policy axis**. The
two expressions are character-identical across the two files. So I built the axis, counted its positions
in my class table, and then forty lines later called the same pair one policy lowered two ways and asked
for a new field on every policy to permit the difference. That is not a subtle error and `141` is right
that it buys nothing, because there was never anything to buy.

**I confirm `141`'s mechanism and I generalise it past the mode it tested.** Translation equivariance is
the property, and it partitions the rounding axis three against three rather than isolating one mode.
Floor, ceiling and nearest-half-up have it. Toward-zero, away-from-zero and **nearest-half-even**, which
is the IEEE default, do not.

**I contest one of `141`'s five replacements, on evidence, and the contest strengthens its own argument.**
Replacement B calls swapping toward-zero for floor a spelling change with no semantic content on
non-negative values. The cell it is invoked for is signed, and on signed values the swap changes between
12.50% and 44.53% of multiply answers.

**I accept the rescoping and I reproduce it, with a refinement neither model had.**

**I do not defend the inflated third direction.** The coordinator's brief claimed op's sentence that Cold
can use the same paths Hot uses corroborates the convergence. `141` is right that it is a claim about
paths against a claim about answers, and right that I5 makes those different things. My phase one never
cited that sentence and I am not going to start now.

---

## Gates

### Canon gate: passed

Checked against `INTENTS.md` entry by entry, same as `139`. Nothing here argues for dropping or
downgrading the storage-minimising concern (I17, `INTENTS.md:363-383`); section 5 argues it reaches one
step further than either cold derivation measured, which is the opposite. I1 is OPEN
(`INTENTS.md:51-61`), so deriving the set is licensed. My replacement for the repair is I13's mechanism,
`INTENTS.md:214-235`, which is the one RATIFIED entry, and `141` reached the same place first.

One entry now bears on this harder than it did on my phase one. **I16, `INTENTS.md:317-331`: the canon
does not police what shape a law takes.** `141` flags that its own replacements A through C name a shape
and offers them as measurements rather than as a rule. My section 7 has the same exposure and I mark it
the same way.

### Test gate: inherited with attribution, per the dispatch

`139` established the livelock in `bitpack-write-contend-shared` and the flag that fixes it; `141`
reproduced it independently at 46.65s against `139`'s 7.97s, which is machine load. All 123 tests pass
with that crate run under `-- --test-threads=1`. I did not rerun it, because the dispatch told me to
inherit it and rerunning would have been an hour spent confirming a thing two files already agree on.

One thing I owe from my own phase one, since `141` quoted the count question back at me. `141`'s command
gives 124 grep hits and 123 tests and identifies the extra as the doc comment at
`stress.rs:68`. That is the same conclusion I reached and it is now established twice.

---

## 1. The repair is dead, and my own committed code killed it

`141` section 3.7 says the capability the slack mechanism was buying is an axis position the design
already has, and establishes it by building a fused arm and an exact-intermediate policy and finding them
bit-identical.

**I did not need to check `141`'s probe, because the identity is between two of my own files.**

`139_probes/p1_policy_classes.rs`, `Op::Madd`, the two positions of the axis I named `Intermediate`:

```
Intermediate::Exact    => reduce(rshift(a * b + (c << f), f, rd), s, ov)
Intermediate::Stepwise => { let t = reduce(rshift(a * b, f, rd), s, ov);
                            reduce(t + c, s, ov) }
```

`139_probes/p2_firewall.rs`, the two arms whose disagreement my section 4 proposed a slack field to
license:

```
madd_fused   => red(rshift_trunc(a * b + (c << s.f), s.f))
madd_unfused => { let t = red(rshift_trunc(a * b, s.f)); red(t + c) }
```

Same expressions. `142_probes/q1_my_own_two_readings.rs` transcribes all four from the files they appear
in and sweeps every input triple at `W` in {4, 6}, every `F`, both signednesses, both overflow positions:

```
  swept 6356992 input triples
  A1  p1 Intermediate::Exact    vs p2 madd_fused    : 0 differences
  A2  p1 Intermediate::Stepwise vs p2 madd_unfused  : 0 differences
  C1  p1 Intermediate::Exact    vs p2 madd_unfused  : 757954 differences (control)
  C2  nonzero results: 6271942
```

The control matters: if all four agreed the identity would be vacuous, and the cross pairing differs at
757,954 inputs, so it is a fact about the pairing rather than about everything collapsing.

**So `139` asserted two incompatible things about one pair of expressions, two probes apart.** If the
pair is an axis, the difference is a declared policy difference and no mechanism is needed to permit it.
If the pair is one policy lowered twice, the difference is a firewall violation. It cannot be both. My
section 2 counted them as two classes in the class table, and my section 4 proposed a slack field on
every policy and a conformance obligation on every arm to license the very difference that made them two
classes.

**That is the whole of `141`'s section 3, arrived at from inside the file it attacks.** Their p6 and my
q1 are two instruments on one claim and they agree, which is the second instance the rung wants, and I
should say plainly that they got there first and by a harder route: they had to build both arms and
identify them, where I only had to read my own two probes side by side and notice.

The honest post-mortem is that I never read them side by side. `p1` was about counting classes and `p2`
was about the firewall, and I held them as separate questions because they were separate probes. The pair
of expressions was the same in both and I did not see it, in my own code, in one sitting.

---

## 2. The mechanism, confirmed and then generalised

`141` chased my signed wrapping row through a refuted model, an absorption theorem and two failed
mechanism controls to land on translation equivariance. I re-derived it before opening their probe code,
having read their prose claim, so this is **a re-derivation and a re-measurement, not a blind instance,
and it does not earn the two-expert rung.** Saying otherwise would be exactly the inflation the
coordinator just corrected upstream of me.

My derivation, which is four lines. Write `x = a*b / 2^F` as an exact rational and let `c` be an integer.
Under wrapping, reduction is a ring homomorphism mod `2^W`, so the two arms are `R(rnd(x) + c)` and
`R(rnd(x + c))`. They agree for all inputs exactly when `rnd(x + c) = rnd(x) + c`, which is translation
equivariance on integer shifts. Floor has it. Toward-zero does not, because `trunc(y)` is `floor(y)` for
`y >= 0` and `floor(y) + 1` for negative non-integral `y`, so the two differ exactly when `x` and `x + c`
straddle zero and `x` is non-integral.

That is `141` F5, and I confirm it.

### What I want past confirming it: the axis, not the mode

`141` tested two modes. The rounding axis has more positions than two, and if equivariance is what
licenses relocating a rounding across an integer addition, then it partitions the axis and **the
partition is the arm's predicate**, which is a better object than a rule naming one mode.

`142_probes/q2_equivariance_partitions_the_rounding_axis.rs`, part A, over exact integers:

| mode | equivariant | violations | first witness |
|---|---|---|---|
| floor | **yes** | 0 of 166725 | |
| ceiling | **yes** | 0 of 166725 | |
| toward-zero | no | 37512 | `p=-63 c=32 F=1` |
| away-from-zero | no | 37512 | `p=-63 c=32 F=1` |
| nearest-half-up | **yes** | 0 of 166725 | |
| nearest-half-even | no | 15872 | `p=-255 c=-31 F=1` |

Three of six, so the C1 control holds: the checker returns different answers for different modes and is
therefore measuring something.

**Two of these were not obvious and I recorded both as predictions before running.** Nearest-half-up is
equivariant, because it is `floor(x + 1/2)` and floor is, which puts a nearest mode on floor's side of
the partition. Nearest-half-**even** is not, because its tie break reads the parity of the result and
adding an integer changes that parity: `rne(1/2) = 0` but `rne(1/2 + 1) = 2`, not 1. **The IEEE default
rounding mode does not permit relocating a rounding across an integer addition.**

Part B checks that the partition predicts the thing it is supposed to predict, on the fusion difference
at `W = 6` signed wrapping, exhaustively over all 262144 triples per cell:

| mode | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| floor | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| ceiling | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| toward-zero | 0.00% | 1.64% | 5.54% | 12.34% | 22.22% | 33.40% |
| away-from-zero | 0.00% | 1.64% | 5.54% | 12.34% | 22.22% | 33.40% |
| nearest-half-up | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| nearest-half-even | 0.00% | 12.50% | 12.50% | 9.38% | 6.25% | 3.91% |

**Six of six agree with the partition.** Equivariant modes are zero everywhere; non-equivariant modes are
nonzero at every `F > 0` and zero at `F = 0`, which is the C3 control holding, since no rounding occurs
at `F = 0` and a mode showing a difference there would mean the instrument was measuring something other
than rounding.

The toward-zero row reproduces `139`'s original signed wrapping row digit for digit, which is the third
independent reproduction of it after `141`'s p3.

**So the predicate is not "rounding = floor". It is "the rounding position is translation equivariant",
which is a const-checkable property of an axis position and covers three modes rather than one.** That is
I13's shape with a wider region than either of us had, and it is the useful form for a canon: a design
that names six rounding modes can record which three carry the property, and an arm gates on the property
rather than on a mode list that has to be maintained.

---

## 3. Contesting replacement B, which is the one thing here I disagree with

`141` replacement B: spell the fractional shift as an arithmetic shift right rather than an integer
division, because "it is translation equivariant so the relocation question does not arise, and it has no
semantic content at all on non-negative values."

The second clause is true and it is doing work it should not. **The cell replacement B is invoked for is
signed**, and on signed values the swap has plenty of semantic content.

`139`'s own `p1_out.txt` already contains this without my having noticed it. At `W=6 F=3 signed` the
twelve labels split into twelve classes, with `Wrap/Trunc/Exact` at class 0 and `Wrap/Floor/Exact` at
class 4: toward-zero and floor are **observationally distinct policy assignments**. At `W=6 F=3 unsigned`
they merge into one class. So the distinction exists exactly on the signed shapes and nowhere else, which
is precisely where replacement B operates.

`q2` part C quantifies it, over every multiply at `W = 6`:

| | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| unsigned, wrapping | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| unsigned, saturating | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| signed, wrapping | 0.00% | 12.50% | 25.00% | 34.38% | 40.62% | 44.53% |
| signed, saturating | 0.00% | 2.93% | 9.57% | 20.51% | 34.33% | 44.53% |

**Up to 44.53% of all products change answer.** Calling that a spelling is the same move as calling
fusion a lowering, which is the move `141` correctly refused in my file. Rounding is component one by
every membership test in this panel including `141`'s own, so choosing floor over toward-zero is
choosing a policy, and a consumer who wanted toward-zero gets a different program.

**And the contest strengthens `141` rather than weakening it.** Their own section 3.6 makes exactly this
argument against my mechanism: "a cost model permitted to spend it is a cost model permitted to change
the rounding mode. Rounding is component one." Replacement B spends the same coin from the design side
instead of the cost-model side. The honest version of B is B-prime below, and it is C's shape.

**B-prime. Ship floor as a rounding position and let the consumer select it.** Then the equivariant
region is reachable by declaration, a consumer who needs toward-zero on negatives keeps it, and nobody's
answers move without their having asked. `141` half-states this itself in B's closing clause, and my
measurement says the half it states as a fallback is the whole of it.

There is a separate and real point inside B that survives intact: **the arithmetic shift right is what
the hardware does, and Rust's `/` is not it.** That is `131`'s F131-3 and it means the two spellings of
"truncation" are two positions and the design has to say which one a name denotes. That is a naming
obligation, not a free conversion.

---

## 4. The cross-topic link, which I was asked to check

The question: is `141`'s toward-zero non-equivariance the same fact as the rounding topic's finding that
"truncation" is ambiguous between bit-drop and toward-zero?

I read `131_leroy_formalising_the_rounding_axis.md:315-330` and its F131-3 at `:535-539`:

> Two's complement bit-drop is `floor`. It is not `toward_zero`, and the two differ on signed domains
> and nowhere else.

**They are not the same statement, and they are two consequences of one fact.** The shared fact is that
floor and toward-zero agree on non-negatives and differ by exactly one on negative non-integers.

- `131` derives from it a question of **denotation**: which function the word "truncation" names, and
  therefore that a predicate spelled with the word is not a predicate until a reader knows whether the
  probe implemented a shift or a division.
- `141` derives from it a question of **soundness**: which relocations of a rounding are answer-
  preserving.

So it is a cross-topic convergence, and the honest form is that two topics independently hit the same
asymmetry from different sides rather than that one restated the other. Neither would have found the
other's consequence.

**And q2 says neither statement is the general form.** `131`'s is about a name; `141`'s is about one
mode. The general form is a property of the axis, it partitions six modes three against three, and it
puts nearest-half-even on the toward-zero side, which is a fact neither topic's framing predicts and
which matters more than the toward-zero case does, because half-even is what a reader assumes is safe.

**That gives `131`'s topic something it could not have derived and should have.** Naming the six modes is
necessary and not sufficient. The canon should also record, per mode, whether it is translation
equivariant, because that is the property an arm's predicate reads and it is not recoverable from the
mode's name. A design that ships six modes and does not say which three carry it has left every
relocation question to be re-derived per site.

---

## 5. The rescoping: accepted, reproduced, and refined

`141` F2 says the accumulator width is answer-visible exactly at `signedness = signed, overflow =
saturating`, and flags it as the one genuinely new claim in the file, resting on one instrument, with the
specific risk named: whether its accumulator is a shape any consumer would build and whether the
operations were routed the way a real kernel routes them.

**I accept the rescoping and I am the second read.**

The scope objection is correct and it lands on my `p6` squarely. `139:244` generalises from packing to
"the storage-minimising concern", and packing is where a value is put. Minimisation does not stop at the
column, an accumulator in a column store is an array too, and my model had no accumulator in it at all.
That is a real gap and no predicate of mine would have narrowed it, because no dimension of mine reached
it.

`142_probes/q3_an_independent_accumulator_model.rs` is deliberately a different construction. `141` added
an accumulator dimension to a set of operations and counted classes; mine is a **fold over a sequence**,
which is the shape a column kernel has: an accumulator carried across `n` steps, reduced at the
accumulator's width every step, narrowed to the declared width once at the end. Sequence length is a
dimension here and is not one there, and the final narrowing is a separate reduction rather than the same
one. Exhaustive over every sequence at `W = 4`.

| cell | accumulator visible | lossy control | witness |
|---|---|---|---|
| unsigned wrapping, n = 1 to 4 | 0 | fires | |
| unsigned saturating, n = 1 to 4 | 0 | fires | |
| signed wrapping, n = 1 to 4 | 0 | fires | |
| signed saturating, n = 3 | **1428** | fires | `[-8, -8, 1]` pinned=-7 widened=-8 |
| signed saturating, n = 4 | **44163** | fires | `[-8, -8, -8, 1]` pinned=-7 widened=-8 |

All four of my predictions held, the lossy control fires wherever the accumulator is exercised at all, and
the structural control holds: at `n = 1` no width is visible under any policy, because a fold of length
one has no intermediate to reduce.

**So F2 reproduces on a second construction and the cell is exactly the one `141` names.** That is the
second instance it asked for, and the answer is that the finding is real rather than an artifact of its
accumulator model.

### The refinement, which neither of us had

My multiply-accumulate fold showed **zero** at signed saturating, and the honest reading of a zero is that
it is a fact about my arrangement before it is a fact about the world.

The mechanism says why. Visibility requires a saturation that is **followed by a step that could recover
from it**: a narrow accumulator clamps and throws magnitude away, a wide one keeps it, and the difference
only surfaces if something later pulls back toward the range. My multiplier schedule was `[1, -1, 2]`,
which puts the largest step last, so a saturation on the final step is immediately narrowed by the final
reduction either way and the effect cannot appear however hard the sweep looks.

Reversing the schedule to `[2, -1, 1]` and rerunning:

```
  signed=true  Sat  large-last  n=3: accumulator visible at      0 | lossy control 2546 | left range 688
  signed=true  Sat  large-first n=3: accumulator visible at    672 | lossy control 2902 | left range 856
```

Same cell, same widths, same domain, same instrument. **The step schedule decides whether the effect is
reachable.**

That is a dimension neither model carried, and it has a consequence for how F2 should be stated. The cell
is real, and its *reachability* is a property of the kernel's accumulation order rather than of the axis
assignment. Two honest ways to write that, and I would take the first:

- **Conservatively**: the accumulator width is answer-visible at signed saturating, because a schedule
  exists that exposes it, and a design cannot know the consumer's schedule. This is the safe reading and
  it is the one a substrate should take under `arvo-toolbox-not-policer.md`.
- **Precisely**: visible at signed saturating **when the schedule places a saturating step before a step
  of opposing sign**, which is a predicate a kernel could gate on but a type cannot.

I flag one thing against my own model, in the same spirit `141` flagged theirs. My sum fold has no
rounding in it, on purpose, so that a rounding effect cannot masquerade as an accumulator effect. That
isolation is why I trust the sum-fold rows, and it is also why the mac fold is the one that found the
schedule dimension: the isolation removed the thing that made the arrangement matter.

---

## 6. The counting claim, conceded

`141` section 5.3 says my "the count is not a property of the design at all" is too strong, because
`shape -> count` is a well-defined function, and that the useful canon sentence is the function rather
than its non-existence.

**Conceded, and my own phase one contains the evidence against me.** `139:166-171` tabulates 2, 3, 8 and
12 classes by shape. A table is a function. What I should have written is the sentence I did write two
paragraphs earlier and then overshot: the count of **distinguishable policy assignments** is determined by
the axis set and the shape, and the count of **presets** is vocabulary. I collapsed the two into one claim
and stated the vocabulary half about the whole thing.

`141` also observes that neither cold derivation noticed `W` does nothing, and that my own table already
contains the pattern: every pair in it differing only in `W` agrees. That is right, it was in front of me,
and their `p5c` earns the narrower statement I could not have made, which is that the invariance is a
property of the axis set rather than of the design, with a width-sensitive axis position exhibited as the
counterexample.

The monotonicity theorem is theirs and I have nothing to add to it. I note only that it makes my
membership procedure well defined rather than merely plausible, which is a load I did not know it was
carrying.

---

## 7. So what does the firewall become

The dispatch asks the right question. The diagnosis survived and the repair did not, so the interesting
part is what replaces the repair rather than whether it dies.

**The firewall stands exactly as first written, with nothing appended.** `141` says the same and reached
it first. The proposition that a cost model must not be able to move an answer is I15 one layer up, and
nothing in either file is against it.

What changes is that **it needs no exception**, and everything I built to give it one was answering a
question that did not exist. Working through the cells with `141`'s replacements and mine composed:

**Where fusion is answer-preserving, it is a legal lowering and gates on a const predicate.** That is
`141`'s replacement A and it is I13's shape. The predicate is now wider than either of us had it, because
q2 generalises the licensing condition: under wrapping the fused arm is free when the rounding position is
translation equivariant, which is three of the six modes rather than floor alone, and it is free under
unsigned regardless of mode by the congruence argument. Nothing is loosened, because in that region there
is no answer difference to loosen anything for.

**Where fusion changes the answer, it is not a lowering at all and is selected by declaring the axis
position.** That is `141`'s replacement C, and my q1 says it more strongly than their p6 does: the two
things are not merely equal functions, they are the same two expressions I had already written as an axis.

**Those two are complementary and they partition the space**, which is worth saying because `141` lists
them as separate items and a reader could take them as competing. A covers the region where the fused arm
is free; C covers the region where it is not. Between them there is no cell left over, which is exactly
why no mechanism is needed: the mechanism existed to serve a residue that turns out to be empty.

I add nothing to `141`'s D and E, and I have replaced B with B-prime in section 3.

**And the caution `141` raises against its own replacements applies to mine.** I16 says the canon does not
police what shape a law takes. A through C and B-prime name shapes. They are what the measurements support
in these cells, not a rule about how a permission must be constructed, and if they read as the latter they
have overreached and should be cut back to the predicate alone.

---

## 8. What I carry forward unchanged, with a count

**Nine positions kept, from one member, plus two of my own that survived attack.** Three I established
independently, and those are marked, because independently reached agreement is the only thing that earns
the two-expert rung and it is the contribution least likely to be reported.

From `141`:

1. **The repair is worthless in every cell.** *Independently established*, `q1`, from `139`'s own committed
   probes rather than from `141`'s instrument.
2. **Fusion is an axis position the design already has.** *Independently established*, same probe, and
   `141`'s p6 got there first and by a harder route.
3. **Translation equivariance is the mechanism behind `139`'s signed wrapping row.** Re-derived and
   re-measured, **not** blind: I read `141`'s prose claim first. This does not earn the rung and I am not
   claiming it.
4. **The absorption theorem**, that reduction mod a power of two absorbs a prior reduction. I did not
   rebuild it. My q2 part B rests on it for the wrapping half of the argument and I say so rather than
   presenting that half as mine.
5. **The accumulator cell.** *Independently reproduced*, `q3`, on a fold-based model with a schedule
   dimension `141`'s did not have.
6. **`shape -> count` is a function and my absence claim was too strong.** Conceded.
7. **`W` does nothing for these axis sets, and that is a property of the axis set.** Conceded; their p5c
   earns the narrow form.
8. **Monotonicity of the class count in the observation set.** Theirs entirely.
9. **The correction to the brief's third direction.** Not defended, per the coordinator.

Of mine that survived `141`'s attack and that I still hold:

10. **The firewall itself.** `141` attacked the repair and endorsed the proposition. It is now two experts
    on the proposition and I state that as `141`'s agreement with `139` rather than as a fresh instance.
11. **The observability procedure belongs to the chain rather than the axis**, which I conceded against
    myself in `139` phase two and which `141` supports from its own T5.

**Zero positions carried from any other panel file.** My reading for this reply was `141` in full, `140`
not at all beyond what `141` quotes, `OPTIONS.md` Q51 from my phase two, `INTENTS.md`, and
`131_leroy_formalising_the_rounding_axis.md:315-330` and `:535-539` for the cross-topic question. I did
not read `140`, so where `141` characterises it I am relying on `141`, and section 6's remarks about
`140`'s F2 are not mine to make and I have not made them.

---

## 9. Findings, with predicates

**F142-1. `139`'s `p1` Intermediate axis positions and `139`'s `p2` fusion arms are the same two
functions.**

```
holds for: numeral fixed-point, W in {4, 6}, F in {0..W-1},
           signedness in {unsigned, signed}, overflow in {wrap, saturating},
           rounding = truncate toward zero,
           operation = multiply-add, arity = 3, chain length = 2,
           container width = declared width,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F142-2. Translation equivariance partitions the rounding axis, and floor, ceiling and nearest-half-up
have it while toward-zero, away-from-zero and nearest-half-even do not.**

```
holds for: rounding in {floor, ceiling, toward zero, away from zero,
             nearest-half-up, nearest-half-even},
           F in {1, 2, 3, 4, 5}, numerator in [-256, 256], integer shift in [-32, 32],
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F142-3. Under wrapping, the fused and stepwise multiply-add agree for exactly the equivariant rounding
positions.** Six of six modes agree with the partition.

```
holds for: numeral fixed-point signed, W = 6, F in {0, 1, 2, 3, 4, 5},
           overflow = wrap, rounding as in F142-2,
           operation = multiply-add, arity = 3, chain length = 2,
           container width = declared width,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F142-4. Replacing toward-zero with floor changes between 12.50% and 44.53% of multiply answers on
signed shapes and none on unsigned ones.**

```
holds for: numeral fixed-point, W = 6, F in {0, 1, 2, 3, 4, 5},
           signedness in {unsigned, signed}, overflow in {wrap, saturating},
           rounding in {truncate toward zero, floor},
           operation = multiply, arity = 2,
           container width = declared width,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F142-5. In a fold, the accumulator width is answer-visible at signed saturating and invisible in every
other cell.** Second independent instance of `141` F2.

```
holds for: numeral fixed-point, W = 4, accumulator width in {W, W+1, W+2, 2W},
           signedness in {unsigned, signed}, overflow in {wrap, saturating},
           fold length n in {1, 2, 3, 4}, operation = sum fold,
           reduction at the accumulator width per step and at the declared width once at the end,
           domain = every sequence over the declared range,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F142-6. Whether that visibility is reachable depends on the accumulation schedule.** Same cell, same
widths, same domain: 0 with the largest step last, 672 with it first.

```
holds for: numeral fixed-point signed, W = 4, F = 1, overflow = saturating,
           accumulator width in {W, W+1, W+2, 2W},
           fold length n = 3, operation = multiply-accumulate fold,
           multiplier schedules {[1,-1,2], [2,-1,1]},
           rounding = floor,
           domain = every sequence over the declared range,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

---

## 10. Options

**O-142-A. The canon records translation equivariance per rounding position.**
A design naming six modes and not recording which carry the property leaves every relocation question to
be re-derived per site, and `141`'s route to it took a refuted model, a theorem and two failed controls.
*Closes on*: whether any arm in the design relocates a rounding across an integer addition. If none does,
the property is diagnostic and belongs in the audit trail. `p6`-shaped fusion arms are one such site, so
the answer looks like yes, but one site is not a canon obligation.

**O-142-B. The accumulator's visibility at signed saturating is stated conservatively or with a schedule
dimension.**
F142-6 says the cell is real and its reachability is a kernel property. Conservative means the type says
visible and a consumer who knows their schedule cannot claim the cheaper reading; precise means a
predicate a kernel gates on and a type cannot.
*Closes on*: whether any consumer kernel has a fixed, declarable accumulation order. `satfold-*` and
`warm-clamp-*` under `mock/benches/variants/` already carry accumulator arms, so this is a computation
over committed artifacts rather than a new bench.

**O-142-C. Whether nearest-half-even's failure changes which rounding position is the default.**
It is the mode a reader assumes is safe and the one IEEE made canonical, and it is on the non-equivariant
side. If the design defaults to it, every relocation is unavailable at the default.
*Closes on*: whether the default is chosen for familiarity or for what it licenses. This is op's, and it
is the kind of question I3's ergonomics settlement bears on directly.

---

## 11. Coverage and bounds

**My predictions this round, and which fell.** Ten stated, one fell, and the one that fell produced the
better result: my mac fold predicted the accumulator would be visible at signed saturating and it showed
zero, and chasing that zero rather than reporting it found the schedule dimension. E5 and E6 in q2 were
the two I was least sure of and both held, which I record because a prediction that holds is only worth
noting when it could have gone the other way.

**Everything is `threads = 1`.** No probe touches concurrency, so under the panel's notation none of these
findings holds anywhere threads exist.

**Every measurement is at model widths**, `W` in {4, 6}. No transfer argument to 64 bits and I am not
offering one.

**Container width equals declared width in every instrument**, the same narrowing `139` reported against
itself and `141` inherited. q3's accumulator dimension is declared explicitly and is the one exception.

**I priced nothing.** No claim here is a bench result and none is called one. Whether the fused arm is
faster than the stepwise arm remains **unpriced**, and it does not bear on anything above: my whole
argument is that the fast arm is reachable by declaration, not that it is fast.

**I did not read `140`.** Where `141` characterises it I rely on `141`, and I have made no claim about
`140`'s findings.

**I did not touch the weighting side again.** `139`'s `p4` geometry, the 44.3% mapping difference and the
Pareto-optimal arm no linear weighting can select are all still at one expert and still untested by
anybody. That is now two rounds in which nobody has looked at half of the two-component object, and it is
the largest untouched surface in this topic.

**Where I would want the second pair of eyes.** F142-6, the schedule dimension. It rests on one
instrument, it is the only genuinely new claim here, and the specific thing to check is whether a
multiplier schedule is a fair stand-in for a kernel's accumulation order or whether I have found a
property of my own loop. If a second reader builds an accumulation order that is not a multiplier
schedule and the reachability still moves, it is real; if it does not, F142-6 is an artifact of my
construction and F142-5 stands unqualified.

---

## Appendix: the probes

Three, each committed with its output before this file.

1. `q1_my_own_two_readings.rs`: `139`'s two probes compute the same two functions, 6,356,992 triples,
   zero differences on both identities, 757,954 on the cross pairing that makes it non-vacuous.
2. `q2_equivariance_partitions_the_rounding_axis.rs`: the partition over six rounding modes, the fusion
   table it predicts at six of six, and the cost of replacement B at up to 44.53%.
3. `q3_an_independent_accumulator_model.rs`: `141` F2 reproduced on a fold, and the schedule dimension
   that its first arrangement hid.
