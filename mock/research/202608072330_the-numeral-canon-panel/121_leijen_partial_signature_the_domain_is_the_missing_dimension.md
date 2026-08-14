# 121. Partial signature: I sign eight clauses, dissent on two, and correct one rung in my own favour's opposite direction

`116`'s author, checking `119` as one of the members it was built from. The draft asks for a cosignature
or a dissent, and says a partial one naming exactly which clauses I stand behind is the better answer when
that is the truth. It is.

**I sign section 4's clauses 4.1, 4.3, 4.5, 4.6, 4.7, 4.8, 4.9 and 4.10, and section 5 entire.** I dissent
on **4.2** and **4.4**, which are the two clauses carrying my own work, and the dissent is the same defect
in both: a dimension that is load-bearing everywhere in this topic appears in no predicate in the sitting,
including mine.

**And I correct one ledger rung against myself.** A13 credits me with an independent arrival that I do not
think survives `RULES.md`'s own standard.

The draft's section 7 asks me to look hardest at the rung column and says A1 in particular is where it may
have flattered the sitting. A1 is fine. A13 is the one, and it flatters me rather than the sitting.

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` with its "How to read an entry" section as normative. Nothing here is
contradicted by I1 through I18, and I13 (`INTENTS.md:214`) is what both dissents are made under: a
predicate lists only what holds, and a dimension absent from it is not a claim about that dimension.

### 0.2 Test gate: passed, at 123 across 13, and it is the fourteenth count

Every test-bearing bench variant crate, per crate, serially. Transcript at `121_probes/t0_test_gate_run.txt`.
All 13 pass, zero failures, zero ignores, attribute count 123 agreeing with the run. Not the workspace-wide
form, which `117` records as a false green.

One test in that suite turns out to be evidence in section 3 rather than only a gate result, which is the
first time in this topic that a shipped test has borne on a canon clause.

---

## 1. What I sign, and what I do not

**Signed without reservation:** 4.1 (definitional), 4.3, 4.5, 4.6, 4.7, 4.8, 4.9, 4.10, and section 5's
statement of what the topic did not settle. Section 5 is the strongest part of the draft and I would not
change a word of it.

**Signed with one note:** 4.7. Its two predicates are correct and its split into two is right. The note is
in section 5 below and it is a gap rather than a defect.

**Dissent, with a replacement clause:** 4.2 and 4.4.

**Ledger correction:** A13.

---

## 2. Dissent on 4.2: the predicate admits a counterexample, and the missing condition is the domain

### 2.1 What the clause says

`119:392-397`:

> **No realisation map onto a finite value set is both an additive homomorphism and order-preserving,
> unless it is constant.**
>
> *holds for: value set finite with at least two elements; operations including addition; domain
> containing a complete residue system and the interval from zero to the value set's size.*

The statement is mine and I stand behind it. **The predicate is not mine and it is wrong**, in the
direction that matters: it names two domain conditions that do not do the work and omits the one that does.

### 2.2 The counterexample, which is a saturating map

`t1` section E. Take `R(v) = min(v, 15)` onto a 16-element value set, on the domain `0..47`:

```
  saturating R(v)=min(v,15) on 0..47: hom(add) True, hom(mul) True, hom(sub) False,
                                      monotone True, non-constant True
    the two conditions `119` 4.2 names: complete residue system True, interval 0..16 present True
    so it satisfies the stated predicate and is a counterexample: True
```

It is an additive homomorphism in the reduce-early-equals-reduce-late sense, it is order-preserving, it is
not constant, and it satisfies both domain conditions the clause names. Under the predicate as written the
theorem is false.

**On a domain that straddles zero the same map is not a homomorphism**, and the witness is two lines long:

```
    R(v)=clamp(v,0,15) on -32..32: hom(add) False, monotone True
    witness: a=-1, b=1: R(a+b)=0, R(R(a)+R(b))=1
```

### 2.3 The condition that does the work, and it is in my own proof

My proof's first step is that an additive homomorphism onto a finite set has kernel `nZ`, so `R(0) = R(n)`.
That step needs the image to be a **group**, and the image is a group only if every element of the domain
has an additive inverse in the domain. A non-negative domain gives a monoid, saturation has no inverses in
it, and the step does not run.

So the condition is **closure under negation**, and `t2` measures that it is the only one that separates
the cases:

```
                        add+mul   add only   mul only
    |V|=2 straddles 0          0          0          0
    |V|=2 non-negative         1          1          1
    |V|=3 straddles 0          0          0          0
    |V|=3 non-negative         3          3          3
```

Every straddling row is empty at every operation set. Every non-negative row is non-empty at every
operation set. And adding closure under negation back to a non-negative window empties it again, at both
value-set sizes.

**And `119`'s two stated conditions hold on both rows**, so neither of them distinguishes the case where
the theorem holds from the case where it fails:

```
  |V|=2 straddles 0   : complete residue system True, interval 0..2 present True
  |V|=2 non-negative  : complete residue system True, interval 0..2 present True
```

### 2.4 My own predicate had the same gap, one file earlier

This is not a defect the draft introduced. **My F116-4's predicate says "window = 9 and 13 consecutive
integers" and does not say they straddle zero**, and my probe's window did. My statement said
`R : Z -> V`, which is safe, and the predicate under it under-specified the domain. The draft inherited
that gap and, in trying to state the domain condition explicitly, replaced an under-specification with
something falsifiable.

That is the failure mode I named in my own `116` section 2.2 about a different sentence, arriving at my own
predicate rather than at my prose. I did not catch it then and I would not have caught it now without
`118`'s ablation forcing me back to the hypothesis.

### 2.5 The replacement clause

> **No realisation map onto a finite value set is both an additive homomorphism and order-preserving,
> unless it is constant.**
>
> *holds for: value set finite with at least two elements; domain closed under negation; operations any
> non-empty subset of {addition, multiplication}.*

Three changes, each measured.

**"Domain closed under negation" replaces the two conditions that do not separate.** `t2` P3.

**"Operations any non-empty subset" replaces "operations including addition".** `t2` P1: on a domain closed
under negation the conjunction is empty under `{add, mul}`, under `{add}` alone and under `{mul}` alone. So
the operation set is not load-bearing at all once the domain is right, which is **wider than `118` F118-4
claims and wider than 4.2 states**.

**Per I13 this is a new claim rather than an edit**, and it is stated with its own provenance below as
F121-1 and F121-2. `116`'s F116-4 stands as written.

### 2.6 What this does to `118` F118-5, which is where I disagree with the ablation rather than with the draft

`118` F118-5 reads "Addition is load-bearing, with witnesses", and its witness is a multiplicative
homomorphism on a non-negative window. **The witness is real and reproduces**: `t1` section B finds it at
both value-set sizes, the map sending zero to the bottom and everything else to the top.

**The witness does not isolate the operation set.** The same map is a counterexample over `{add}` alone on
the same window, and `t1` section B measures that too, at 1 and 3 non-constant maps on both rows. So what
separates the row from its neighbour is the window's sign, not which operation was dropped, and F118-5
attributes to the operation what belongs to the domain.

F118-4, F118-5 and F118-6 are three findings about three dimensions, and `t2`'s two-by-two says two of them
are one finding about one dimension. F118-6 is the same fact at a third size: a window narrower than the
value set is also not closed under negation.

---

## 3. Dissent on 4.4: the saturating clause is refuted by my own data and by a shipped test

### 3.1 What the clause says

`119:430-436`:

> A wrapping map is a homomorphism for addition and subtraction, and for multiplication only where the
> fraction width is zero. **A saturating map is a homomorphism for no operation** and is order-preserving.
>
> *holds for: W = 4, F in {0, 1, 2}, signedness in {unsigned, signed}, overflow behaviour in {wrapping,
> saturating}, ...*

The wrapping half is mine, it is correct, and I sign it. **The saturating half is false on a non-negative
domain**, and the predicate that carries it widens my F116-7 from `overflow policy = wrap` to both
behaviours without the evidence for the added half.

### 3.2 It is refuted by the table in my own probe

`116_probes/p4_output.txt:13`, which I printed in `116` and did not read as a finding:

```
  uW4F0/sat                0/2116     885/2116       0/2116
```

Zero failures for addition. Zero for multiplication. 885 for subtraction. **Saturation on a non-negative
domain is a homomorphism for two of the three operations**, and the one it fails is the one that leaves the
domain.

### 3.3 Two files measured this cell and got different answers, and neither predicate says why

`118_probes/q3_output.txt:13` reports the same cell at **720/2304 for addition** where mine reports
**0/2116**. Same primitive, same operation, same identity, opposite verdicts.

`t3` reconciles them, and the reconciliation is the point:

```
  policy   span                            add          sub          mul   monotone
  sat      non-negative 0..45           0/1081     885/1081        0/273       True
  sat      straddling -45..45        3120/6211    2445/6211      182/909       True
  wrap     non-negative 0..45           0/1081        0/723        0/273      False
  wrap     straddling -45..45           0/6211       0/6211        0/603      False
```

**The saturating rows move with the span and the wrapping rows do not.** That is the control: the ambient
domain is what separates the two files' numbers, and neither of us is wrong. My F116-7's predicate lists no
span at all; `118` F118-7's says "ambient span = three times the container" without saying whether it
straddles zero.

So section 2's missing dimension is not confined to the theorem. It is missing from a routine measurement
in two files, where it produced a numerical conflict that would otherwise read as one of us having made an
error.

### 3.4 And the clause contradicts a test in this repository

`warm-clamp-shared/src/lib.rs:1105` is
`clamping_is_a_retraction_on_non_negative_addition_at_every_swept_width`, which asserts that clamping at
every step of a fold equals clamping once at the end, on non-negative addition. That is exactly the
homomorphism identity for addition on a non-negative domain. `t3` P5 re-derives it:

```
    W=3: eager and deferred clamping disagree on 0/1088 folds
    W=4: eager and deferred clamping disagree on 0/8448 folds
    W=5: eager and deferred clamping disagree on 0/66560 folds
```

A clause saying a saturating map is a homomorphism for no operation says that fold's deferral is
unlicensed, and the fold ships, and its test passes, and it has passed in all fourteen counts of this
sitting. **The clause and the suite cannot both be right**, and the suite is right.

### 3.5 The replacement clause

> A **wrapping** map is a homomorphism for addition and subtraction at any fraction width, and for
> multiplication only where the fraction width is zero or the operands are declared on the unit grid.
>
> A **saturating** map is a homomorphism for exactly those operations that cannot carry an exact result
> out of the declared domain's sign. On a domain closed under negation that is no operation. On a
> non-negative domain it is addition and multiplication, and not subtraction.
>
> *holds for: W = 4, F in {0, 1, 2}, signedness in {unsigned, signed}, overflow behaviour in {wrapping,
> saturating}, rounding = truncation, radix = 2, operations in {add, sub, mul}, ambient domain in
> {non-negative, closed under negation}, threads = 1, target features any.*

The added dimension is `ambient domain`, and it is the only change to the predicate other than the
behaviour split the clause already carries.

### 3.6 What this does to my own F116-5, checked because it would be the obvious casualty

If saturation has both characters on a non-negative domain, my F116-5 (the two licence families are
disjoint per behaviour) would be refuted. It is not, and the reason is 2.3: on a non-negative domain the
induced structure is a monoid rather than a group, so what saturation has there is a **semiring
homomorphism** and not the additive-group homomorphism the theorem quantifies over. The deferral licence it
buys is real, which is why `warm-clamp` ships, and it is not the licence family the theorem trades against.

**So the honest statement is that the trade is between the order-based family and the additive-group
homomorphism, and a semiring homomorphism on a one-signed domain sits outside the trade.** That is a third
thing, it is what the shipped fold uses, and no clause in the draft names it. I would add it rather than
leave 4.4 as the place a reader has to infer it.

---

## 4. Ledger correction: A13 credits me with an independence I do not think I have

`119:202-205`:

> Arrived at independently: `116` section 3.3 reaches the same repair from its own analysis of why its
> form loses, and says it arrived from the mechanism rather than from `114`'s file. **That is a second
> instance of the repair**, on reasoning rather than on measurement.
>
> **Rung: one derivation with measurement, one independent arrival without.**

**I would not sign that.** `116:247` is my own sentence and it is a claim about my reasoning process rather
than about ordering. My coverage section in the same file records that I read `114` sections 1 to 6.3 in
full before writing, and arm S2 is in section 4. So I read the repair, then analysed my form's mechanism,
and the analysis arrived where the reading had already put me.

`RULES.md:262` is explicit: "agreement inherited by reading is not" a result. An unverifiable claim about
which of two things caused my conclusion is exactly what that rung exists to exclude, and the fact that the
claim is mine does not make it checkable.

**A13 should read: one derivation with measurement, one restatement after reading.** The mechanism I gave
for why my form loses is still worth keeping, because it explains the repair rather than only endorsing it,
but it is not a second instance and `116` should not have implied it was.

I raise it because the draft asked which entries claim more independence than their instruments support,
and this is the one, and it is mine.

**A1, A2, A3, A5, A6, A7, A8, A14 and A15 I have checked against what I did and they are accurate.** A2 in
particular is generous in a way I want to confirm rather than accept silently: it credits `112` F112-21
with the saturating half independently and first, and that is correct, because `112` was written before
`114` existed.

---

## 5. Whether anything is carried further than my sweeps went

The draft asks specifically about F116-7 through F116-9, my only claims past `F = 0`. Checked dimension by
dimension.

**4.4's predicate widens `overflow policy = wrap` to `{wrapping, saturating}`.** That is section 3's
dissent and it is the one real overreach I found.

**4.5's second predicate lists `declared grid step in {1/4, 1/2, 1, 2}`.** My F116-9 swept three steps and
not the fourth. I checked before objecting: `118_probes/q3_output.txt` sweeps step 2 at 0/36. So the
predicate is the union of two files' sweeps and is supported. **No objection**, and I record that I looked,
because a predicate listing a value neither file swept is exactly what this pass is for.

**4.7 is pinned at `F = 0` on both halves.** My F116-8 established the discharge check's behaviour at
`F in {0, 1, 2}`, and that is not carried into 4.7. **This is a gap rather than an overreach**, and the
draft is more conservative than the sitting's evidence. It is derivable from 4.4 plus 4.6 and a reader has
to do the derivation. If the draft wants it stated, F116-8 supports it at `W = 4, F in {0,1,2}, wrapping,
two-endpoint declarations`; if not, the conservatism costs nothing and I do not press it.

**4.6's predicate lists `declarations = one-sided`.** Mine at that width were two-endpoint. Again more
conservative than the evidence, and again I do not press it.

**On C2, which the draft records as unanswered and mine to answer.** My W1b prose says any fraction width
and my predicate says three of them. `118` argues the gap is the transfer exemption applying, because the
argument that addition and subtraction never enter the rounding region is fraction-width-free.
**I agree, and the exemption applies for the same reason 4.2's does**: the argument quantifies over the
grid's closure under the operation rather than over any width. So W1b should be stated on the argument,
with the sweep as its control, and marked the way 4.2 is marked. That answers C2 and the draft may take it.

---

## 6. Findings, each with its predicate

Conventions: everything enumerative ran on one thread and carries `threads = 1`; the searches are exact
integer arithmetic no instruction selection can move, so `target features any` with that as the argument.

**F121-1. The domain's closure under negation is the load-bearing half of F116-4's hypothesis, and the
operation set is not load-bearing at all.** On a window closed under negation the conjunction is empty
under `{add, mul}`, `{add}` and `{mul}` alike; on a non-negative window it is non-empty under all three, at
1 and 3 non-constant maps. `value set size in {2, 3}, window in {straddling zero at 9 and 13 points,
non-negative at 9 and 13 points, non-negative closed under negation at 9 and 13}, operations in {{add,
mul}, {add}, {mul}}, monotonicity under the natural order, threads = 1, target features any`.
`t2_output.txt`. **Widens F116-4 on the operation dimension and narrows the domain dimension**, and is a
new claim rather than an edit to it.

**F121-2. `119` 4.2's stated predicate admits a counterexample.** A saturating map onto a 16-element value
set on the domain `0..47` is an additive homomorphism, is order-preserving, is non-constant, and satisfies
both stated domain conditions. `value set size = 16, domain in {0..47, 0..63}, map = saturating, operations
in {add, sub, mul}, threads = 1, target features any`. `t1_output.txt`.

**F121-3. `118` F118-5's witness does not isolate the operation it names.** The map sending zero to the
bottom and everything else to the top is a counterexample over `{mul}` alone and over `{add}` alone on the
same non-negative window, at 1 and 3 non-constant maps on each. `value set size in {2, 3}, window
non-negative at 9 and 13 points, operations in {{add}, {mul}}, threads = 1, target features any`.
`t1_output.txt`.

**F121-4. Saturation is a homomorphism for addition and multiplication on a non-negative domain and for no
operation on a domain closed under negation, and wrapping is unaffected by the domain.** Saturating:
0/1081 add, 885/1081 sub, 0/273 mul on `0..45`; 3120/6211, 2445/6211, 182/909 on `-45..45`. Wrapping: zero
on every operation on both. `W = 4, F = 0, signedness = unsigned, overflow policy in {sat, wrap}, rounding
= trunc, radix = 2, operations in {add, sub, mul}, ambient domain in {0..45, -45..45}, threads = 1, target
features any`. `t3_output.txt`. **This reconciles `116_probes/p4`'s 0/2116 with `118_probes/q3`'s 720/2304
without either being wrong**, and refutes `119` 4.4's saturating clause.

**F121-5. The same fact is asserted by a shipped test, and re-derives.** Eager and deferred clamping agree
on 0 of 1088, 0 of 8448 and 0 of 66560 non-negative addition folds at `W in {3, 4, 5}`. `W in {3, 4, 5}, F
= 0, signedness = unsigned, overflow policy = sat, operation = add, fold length in {2, 3}, operands
exhaustive over the declared range, threads = 1, target features any`. `t3_output.txt`, against
`warm-clamp-shared/src/lib.rs:1105`.

**F121-6. Quantifying monotonicity over every total order finds strictly more monotone maps and does not
change the verdict.** 18 against 10 at `|V| = 2` and 471 against 105 at `|V| = 3`, with zero non-constant
passing both either way. `value set size in {2, 3}, window straddling zero at 9 and 13 points, operations =
{add, mul}, monotonicity under {natural order, every total order}, threads = 1, target features any`.
`t1_output.txt`. **Confirms `118` F118-3 and confirms that my own probe tested a weaker theorem than my
statement claimed**, while my proof covered the stronger one.

**F121-7. The suite is 123 across 13 and all of it passes.** Fourteenth count. `toolchain =
nightly-2026-05-28, host = this machine, --release, --test-threads=1, threads = 1`. `t0_test_gate_run.txt`.

**Unpriced.** Everything about cost. No bench harness ran and no claim here depends on a magnitude.

---

## 7. Coverage, bounded rather than claimed

**Read in full:** `119`, `113`, `118` sections 6 and 10 and its findings list, `115` section 1 and its
headings, `INTENTS.md`, and my own `116`.

**Read in part:** `118_probes/q3_output.txt` at its homomorphism and grid tables; `116_probes/p4_output.txt`
at its homomorphism table; `warm-clamp-shared/src/lib.rs` at the one test named.

**Not read, and named because the draft rests on them:** `119_probes` entire, including the anchor
instrument `r1` whose set-difference result I did not check; `118_probes/q1`, `q2`, `q4`, `q5`, `q6`;
`115_probes` entire; `114` except through `116`'s and `119`'s accounts. **So my signature covers section 4
and the ledger entries touching my own work, and does not cover the anchor accounting in section 6.**

**Checked by measurement rather than by reading:** the ablation of my own theorem on my own search; the
counterexample to 4.2's predicate; the two-by-two separating domain from operation set; the reconciliation
of two files' conflicting numbers; the shipped test's claim.

**Not checked:** every clause resting on `114` or `118` alone, which is 4.6, 4.9 and 4.10 in full. I sign
those on the draft's attribution rather than on my own verification, and that is a weaker signature than
the one I give 4.2 and 4.4, where I dissent.

**Citations opened.** `t4_check_my_own_citations.py`, **23 checked, 0 failing**, after one failed on the
first run because the phrase was wrapped across two lines. Mutation-tested three ways.

**One defect of my own in this pass**: `t1`'s section A prediction P2 says multiplication alone admits a
witness, and on a window straddling zero it does not. The probe says so in its own output rather than being
edited, and section B is where the prediction actually holds.

---

## 8. Probe index

All under `121_probes/`, each committed with its output as it ran.

- `t0_test_gate_run.txt`. Fourteenth count, per crate, serially.
- `t1_the_ablation_and_the_predicate_the_candidate_gives_my_theorem.py`, `t1_output.txt`. The three
  ablations on my own search, the some-order gap in my own probe, and the counterexample to 4.2's stated
  predicate.
- `t2_which_hypothesis_is_actually_load_bearing.py`, `t2_output.txt`. The cell neither `118` nor `116` ran,
  as a two-by-two, and the check that 4.2's two stated conditions do not separate its rows.
- `t3_two_files_disagree_on_one_cell_and_the_reason_is_unstated.py`, `t3_output.txt`. The reconciliation,
  with wrapping as the control, and the shipped test re-derived.
- `t4_check_my_own_citations.py`, `t4_output.txt`. Twenty-three citations opened, one corrected, three
  mutations firing.
