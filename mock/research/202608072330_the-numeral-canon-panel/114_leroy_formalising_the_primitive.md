# 114. Formalising the refinement, and the two arms that turned out to be four

My job here is construction rather than attack. The sitting has converged on where the refinement
lives, and what remains is to plug the holes, determine the bounds, and state the thing exactly with
its predicate. That is what most of this file does.

One part of it is a refutation, and I would rather not have found it. `111` F111-15's structural
predicate carries `overflow policy in {sat, wrap}` in its predicate and is **unsound at `wrap`**. The
same axis inverts a second claim in the same run: `112` section 9's offered canon sentence, "Checking
only the derivation's result rather than every node is unsound", is false at `wrap` over ring
operations, where checking only the root is sound on every cell I swept and exact wherever the
predicate's first condition holds. Both are one mechanism, and naming it is the largest thing I have.
The rest of the file is the formalisation the brief asked for, with that axis threaded through it.

Everything below is a suggestion. Op decides, and per I12 an opinion given before the experts converge
is an ack rather than a ratification.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` as it stands, read in full including its normative "How to read an entry"
section.

**I13 (RATIFIED) is what this file is for.** Op's ratified words are that the work is "a bunch of arms
with const predicates that optimize each little 'sometimes'", that a universal solution is explicitly
rejected, and that every finding must be predicated so it is exact rather than shifting. The
deliverable below is four arms with const predicates and no universal, and every finding carries a
predicate in the notation the entry demands, with unmeasured dimensions unstated per op's further
instruction at `INTENTS.md:240-242`.

**I15 is respected and is load-bearing here.** Every predicate I state is a compile-time const. Nothing
in section 4 admits a runtime check, and the Rust probe at `114_probes/p7` shows both gated arms
erasing to symbol aliases with no residue.

**I16 is the reason this file states arms rather than a winner.** Op, at `INTENTS.md:322-326`: "We
shouldn't police what kind of laws there are or what shapes they take. The law is defined as makes
sense and is applicable in each situation on a case by case basis." So the discharge check is not
ranked. Section 4 states which check is correct in which region and stops there.

**I14's constraints hold throughout.** `p7` and `p9` compile with zero feature gates, no `dyn`, no
`TypeId`, and no arithmetic in type position. `p9` records where the naive spelling of a type-level
vector hits `generic_const_exprs` in `112`'s own audit trail and does not repeat it.

**One thing I checked for and did not find: an ambiguity to hand back.** The predicate this file builds
reads the overflow policy, and the overflow policy is placed in the declared semantics by measurement
rather than by assertion. `112` F112-2 (`112:1002-1005`) counts **zero** directions admitting a
denotation-preserving map for it, which under `112` section 9's own classification (`112:937`) is what
"part of the declared semantics" means; and `112` F112-3 (`112:1009`) finds two assignments of it
computing 120 of 256 answers differently. Both offered statements' observable-axis clause
(`106:172`, `108:822`) is the definition those measurements are read against. So it is supplied by the consumer, known
statically, and admissible in a const predicate under op's instruction at `INTENTS.md:252-254` that
"the above collapses to whatever is available at const time". Nothing here needs a call op has not
already made.

**And I did not treat `112` section 9's offered statement as canon.** It is an offered statement in an
agent file, on the presumed-wrong rung, so correcting it is not reopening anything.

### 0.2 Test gate: passed, at 123 across 13, and it is the ninth independent count

`mock/crates/` is empty by design, so the surface is the bench variant tree. Run serially per `110`
F14's workaround, which I took rather than reproducing the hang:

```
bench-quantiser-fadd-shared           1     bench-bitpack-write-contend-shared   15
bench-quantiser-radix-shared          3     bench-warm-container-shared          15
bench-bitpack-shared                  3     bench-warm-clamp-shared               7
bench-bitpack-plan-shared             5     bench-wide-rung-shared               30
bench-bitpack-footprint-shared        6     bench-satfold-shared                 11
bench-bitpack-carrier-shared          9
bench-bitpack-contend-shared         12     total                               123
bench-bitpack-wide-shared             6     across                               13 crates
```

All pass. `cargo test --release --workspace` from `mock/benches` runs **zero** tests, because the
variants are path dependencies rather than workspace members, so the count only appears under `-p
<name>` per crate. `112` F112-16 records the adjacent trap that the package names carry a `bench-`
prefix the directory names do not; the workspace-versus-package trap is the same shape and I record it
beside it.

**I read bodies rather than counting names**, in the two crates my own argument leans on. `111` section
21 claims the fold is the shape `satfold-shared` and `warm-clamp-shared` are built around, and section
5 of this file rests on that. It checks out at source:
`warm-clamp-shared/src/lib.rs:296-300` is `let mut acc = A::ZERO; for &x in chunk { acc =
acc.wadd(...) }`, a left-nested chain of one operation over distinct leaves, which is exactly condition
(a) of the structural predicate holding by construction.

`satfold-shared/src/lib.rs:1115-1192` carries four `assert_ne!` tests that run deliberately broken
kernels and assert the oracle catches them, which is the shape `110` R0 praises and the shape this
sitting has repeatedly failed to apply to its own probes. I found no tautological arm, no
assertion-free smoke test and no sampled law in what I read.

---

## 1. The answer, stated first

> **The discharge check is not one rule. It is four arms over two axes, and the axes are the overflow
> policy and the term's structure.**
>
> **Arm W0, free.** Where the realisation map is a ring homomorphism and the operation set is contained
> in the ring operations, dropping every intermediate reduction and reducing once at the end is
> **unconditionally** equal to reducing at every node. No declaration is needed and none is read. This
> is a licence nothing in the panel has named, and it is the cheapest thing in this file.
>
> **Arm W1.** Under the same map, licensing an arm that does not reduce **at all**, so the result is
> handed on as a wider value, needs the **root's** interval and nothing else. Checking every node there
> is conservative rather than load-bearing: it forgoes 7% to 30% of the available licences across my
> sweeps and buys nothing.
>
> **Arm S1.** Where the map is not a homomorphism, saturation being the case in hand, the check must
> visit **every node**, which is `112` F112-21, and the structural predicate certifies that a refusal
> from it is honest rather than conservative.
>
> **Arm S2.** Where that predicate does not fire, a tighter propagation rule can recover licences, and
> the tightest sound one I could build is the **per-node intersection** of the interval rule and
> `112`'s one-sided affine form. It is beaten by neither on any row I swept, including the rows where
> the one-sided form alone is beaten.

Four further results, each of which breaks something or finishes something.

**`111` F111-15's predicate is unsound at `wrap`**, 28 violations at `uW3/wrap` and 16 at `iW3/wrap`
over an enumeration of every term at two and three leaf slots. It is sound at `sat` across every sweep
I ran, including two-endpoint declarations and arity four, which nobody had tested.

**`112` F112-24's "beaten on none" is a fact about one-sided declarations.** Under two-endpoint
declarations the one-sided form is beaten by the plain interval rule on 92 cells at arity two and 593
at arity three, and the mechanism is nameable: a leaf declared entirely below zero gives the form a
negative constant, which flips the sign of every coefficient it scales.

**The lifting rule is a formula, not a table.** `112` F112-14's three constructions are three instances
of one rule over the structure constants: the L1 norm of each output component's row, plus one bit for
whether that row has a negative entry, read against the base's signedness. Exact, 0 unsound and 0
conservative, over 1024 bilinear constructions on pairs and over the quaternions at dimension four.

**And the compile-time question is answerable without a clock, and the answer refutes the shape
`111` section 21 proposed.** Static selection at the use site does **not** avoid the expensive arm's
compile cost: `selection` refuses at the same fold length as `disjunction`, and so does an impl that
merely defines a const reading the expensive tower. Selecting **before instantiation** does work, and
compiles as far as the cheap arm alone. That is a real design instruction and it cost no duration to
obtain.

The rest of this file is that taken apart. Sections 2 to 7 are the working, section 8 is what I would
hand a canon writer, and sections 9 onward are bookkeeping.

---

## 2. What I verified before building on it, and what I did not

`113` asks the later dispatches to formalise rather than re-open, and the brief names the convergence.
Formalising something means resting on it, so I checked the load-bearing parts rather than accepting
them, and I say which.

**Re-derived on my own instrument.** The corner rule's soundness (0 unsound over every sweep in `p1`,
`p3` and `p4`, with a deliberately unsound halved rule reported unsound on 245 to 5689 cells so the
counter is live). `111` F111-15's sufficiency at `sat`, which holds and which I extend from twelve rows
to 96 terms, two declaration shapes and arity four. `112` F112-21's root-only unsoundness, which
reproduces at a different width on my implementation, 38 cells at `uW3/sat` and 10064 at arity three.

**Reproduced from `112`'s own committed code, not rebuilt.** The corner rule, the symmetric affine
rule and the reachable-set oracle come from `112_probes/p7`, and the one-sided form from
`112_probes/p9`, imported by path. The brief instructs reuse and reuse is also the honest choice: a
domination claim tested against my reimplementation of the rule would be testing my reading of it.

**Accepted without checking.** `112`'s thirteen-row domination result as reported, `110`'s congruence
result, `109`'s chain result, everything about the number-system question, and every claim in `106` and
`108` other than the two clauses I quote. I opened `108:825`, `112:928`, `112:1029`, `112:1039`,
`112:1113`, `112:1142` and `111:951` at source. Nothing else.

**One correction to the record before I build on it.** The brief states, and `111` section 26 states,
that the refinement question was settled twice by wrong routes and that `108:825` is the criterion.
That is right and I have nothing to add. What I would add is that the criterion's own application in
`111` R1 uses one primitive, `uW3/sat`, and moves the declaration on six terms. The verdict is not in
doubt; the sweep is narrow, and section 4 gives a second reason for the same verdict that does not
depend on it: a refinement's discharge check has a **different shape per overflow policy**, and an axis
whose treatment is a function of another axis is not a coordinate beside it.

---

## 3. The mechanism the whole file turns on

Stated before the arms, because every predicate in section 4 reads it.

**A wrapping realisation map is a ring homomorphism. A saturating one is not.**

Reduction to a container spanning `n` values, `R(v) = ((v - lo) mod n) + lo`, is the canonical
representative of `v`'s residue class, so `R(v) = v (mod n)` for every `v`. For any ring operation,

    R(R(a) op R(b)) = R(a op b)

because both sides represent the same class. Saturation destroys the class and the loss is carried
forward.

Checked rather than asserted, exhaustively over the whole ambient range the operands can reach
(`p2`, C3):

```
  primitive    ops                    mismatches         of
  uW3/wrap     add, sub, mul                   0       1728
  iW3/wrap     add, sub, mul                   0       1728
  uW4/wrap     add, sub, mul                   0       6912
  uW3/sat      add, sub, mul                 400       1728
  iW3/sat      add, sub, mul                 464       1728
```

The saturating rows are the control. A zero on every row would mean the check cannot tell the two maps
apart and proves nothing.

Three consequences follow, and each is measured rather than argued.

**One. Under wrap the arms agree exactly when the root's exact value is in range.** No intermediate
node appears in the condition. `p2` measures the two properties against each other over every term at
two and three leaf slots:

```
  primitive      cells   agree  rootval  P1 gap  pernode  rootchk  ro unsnd  ro consv
  uW3/wrap       13032    3210     3210       0     2247     2565         0       645
  iW3/wrap        2148    1402     1402       0     1245     1307         0        95
  uW3/sat        13032    2950     3210     260     2247     2565        38       423
  iW3/sat         2148    1338     1402      64     1245     1307        34        65
```

The `P1 gap` column is the finding: zero under wrap, nonzero under saturation. The `ro unsnd` column is
the same fact from the other side, and it is `112` F112-21 confined to its own predicate.

**Two. Under wrap the root-only check is exact wherever condition (a) holds**, 0 disagreements over
9408 and 1200 cells, against 28 and 16 at `sat` on the same terms.

**Three. It is the ring, not the wrapping.** Adding `min` to the signature breaks the homomorphism, 168
mismatches of 2304, and the root stops deciding: the gap goes from 0 to 591 of 9496 cells at
`uW3/wrap` and 72 of 1500 at `iW3/wrap`. So the predicate carries the operation set, and a design that
puts a clamp, a comparison, a shift or a division into the chain loses arm W1 at that node.

### 3.1 What this does to `111` F111-15

F111-15's predicate reads, at `111:1385-1388`, `overflow policy in {sat, wrap}`. Under I13's reading a
listed dimension is a region the finding was established across, so the finding claims soundness at
`wrap`. It does not hold there. From `p1`, the same structural predicate over 96 terms:

```
  primitive   cells   conservative   predicate fires   violations
  uW3/sat     13032            703              6336            0
  iW3/sat      2148             93               816            0
  uW3/wrap    13032            963              6336           28
  iW3/wrap     2148            157               816           16
```

A violation is the predicate certifying exactness on a cell where the corner rule is in fact
conservative, which is the only way the predicate can be wrong. The witnesses are all one shape, and
`p2` prints them:

```
  (x - (y - z))        declared [(0, 0), (0, 0), (0, 1)]
      nodes: (x - (y - z)) in (0, 1); (y - z) in (-1, 0)   container [0, 7]
      root-only says: True
```

The inner difference leaves the unsigned container below zero, so the per-node rule refuses. Under
wrapping it wraps and unwraps and the root is right, so the arms agree. The predicate certified the
per-node rule's refusal as honest, and the refusal was not honest, because a better rule exists at
`wrap` and the predicate does not know about it.

**F111-15's twelve rows include two at `wrap`**, `x + y` and `(x + y) - y` at `uW4/wrap`, and neither
can exhibit this: the first has one node and the second's masking is a leaf repeat rather than a
sign excursion. That is the hand-picked-row failure `112` section 6 named, landing on the file that
named it.

**The repair is one clause and it is in section 4.** F111-15 restated at `overflow policy = sat` is
sound over everything I swept, which is a much larger region than it originally claimed, so the finding
gets wider on three dimensions and narrower on one.

### 3.2 And what it does to `112` section 9

`112:928`, inside the offered canon statement:

> Checking only the derivation's result rather than every node is unsound.

Unqualified. It is false at `overflow policy = wrap` with `operations` in the ring operations, where
the root-only check is unsound on 0 of 13032 and 0 of 2148 cells and is exact under condition (a).

**`112`'s own finding is correctly predicated and the canon sentence dropped the predicate.** F112-21
at `112:1116` lists `overflow policy = sat`. So nothing was measured wrongly. What happened is the
thing `every-finding-carries-its-predicate.md` exists to prevent, at the step where it is hardest to
see: a predicated finding was compressed into a sentence, and the compression is where the predicate
went.

I would draw one general lesson for whoever writes the canon text, because it is cheap and it is
mechanical. **Every sentence in an offered canon statement that came from a finding should carry that
finding's predicate, or be checked against it.** The two sentences this file corrects are both of that
shape, and both survived their own authors' review.

---

## 4. The arms, stated exactly

Per I13. Every dimension listed with a range or a fixed value was established across it; **a dimension
absent does not hold anywhere that dimension is present**; unmeasured and unknown go unstated.

Three conventions stated once, so they are not repeated fourteen times. Everything enumerative ran on
one thread and carries `threads = 1`. The model sweeps are exact integer arithmetic that no
instruction selection can move, so they carry `target features any` with that as the argument; the
compiled results carry the host explicitly. Every enumerative result is at `W <= 6` and **I have no
transfer argument to any real width**, so every predicate lists its width as a fixed set. That last
one is not a formality: `unstable-features.md` states that a model-width check needs its own transfer
argument, that closing the `specialization` and `TypeId` doors is necessary and not sufficient for one,
and that the honest default is that there is none.

### Arm W0. The intermediate reduction is free

> Where the realisation map is a ring homomorphism and every operation in the term is a ring operation,
> computing without intermediate reduction and reducing once at the root equals reducing at every node,
> **for every declaration and every value tuple**. No declaration is read and none is needed.

**Predicate.** `W = 3 logical, carrier width in {3, 4, 5, 8}, F = 0, signedness = unsigned, overflow
policy = wrap, carrier overflow policy = wrap, rounding = trunc, radix = 2, operations in {add, sub,
mul}, term shapes = every term at 2 and 3 leaf slots over that signature, arity in {2, 3}, declarations
= one-sided [0, b], threads = 1, target features any`.

**Evidence.** `p8_output.txt`: 0 differing cells of 576 at every carrier width, including the 363 cells
where the root's exact value leaves the logical range, which is what makes it unconditional rather than
licensed. Controls: the same measurement at `sat` differs on 26 cells, and a wrapping logical map over
a **saturating carrier** differs on 189 to 322, so a mixed carrier breaks it and the instrument sees
that.

**And it is visible in the compiler.** `p7_asm_bodies.txt`, at a logical width of five in a `u8`
container: `_general_masked = _cheap_reduced_ungated`, one body of four instructions. LLVM already
proves this for a power-of-two mask, which every wrapping reduction at a bit width is. So on that
target the arm costs nothing to take and nothing to leave, and its value is that a design does not have
to build a declaration mechanism to get it.

**What it does not cover, and this is the part a design would get wrong.** The equality is between
reducing early and reducing late. It is **not** a licence to drop the final reduction, which is arm W1,
and it does **not** survive a carrier that saturates. A five-bit logical value living in a `u8` that
saturates at 255 is the mixed case, and `p8`'s C2 measures it failing on 189 to 322 cells of 576.

### Arm W1. Under a homomorphism, check the root

> Where the realisation map is a ring homomorphism and every operation is a ring operation, an arm that
> performs no reduction at all is licensed exactly when the **root's** propagated interval fits. Every
> node need not be checked, and checking every node is conservative.

**Predicate.** `W = 3, F = 0, signedness in {unsigned, signed}, overflow policy = wrap, rounding =
trunc, radix = 2, operations in {add, sub, mul}, term shapes = every term at 2 and 3 leaf slots, arity
in {2, 3}, declarations in {one-sided [0, b] exhaustive, two-endpoint sampled at 500 per term},
discharge check = root only, threads = 1, target features any`.

**Evidence.** `p2_output.txt`: 0 unsound of 13032 and of 2148. `p5_output.txt` for the licence counts,
where the root check is run with the tightest interval available rather than the loosest:

```
  primitive    decls          cells   agree  pernode hyb  root+corner  root+one  ROOT+HYB  unsound
  uW3/wrap     one-sided      13032    3210         2663         2565      3188      3188        0
  iW3/wrap     one-sided       2148    1402         1306         1307      1389      1389        0
  uW3/wrap     two-endpoint   38256    8744         6080         5132      8656      8688        0
  iW3/wrap     two-endpoint   38256   13056         9824         8585     11999     12697        0
  uW3/sat      one-sided  [C1]   13032    2950         2663         2565      3188      3188      259
  iW3/sat      two-endpoint [C1]   38256   10122         9839         8516     11996     12682     2749
```

Checking the root with the tightest interval reaches 99.3%, 99.1%, 99.4% and 97.2% of what the arms
actually permit. The per-node rule reaches 83%, 93%, 70% and 75%. So the cheaper check is also the
better one, which is unusual enough to be worth stating twice.

**The `[C1]` rows are the control and they fire**: the same rule at `sat` is unsound on 259 and 2749
cells. Without them the zeros above would be a dead branch.

**Exactness, separately.** Where condition (a) of section 4's structural predicate holds, the root
check disagrees with the arms on **0** of 9408 cells at `uW3/wrap` and 0 of 1200 at `iW3/wrap`, against
28 and 16 on the same terms at `sat`. `p2_output.txt`.

### Arm S1. Without a homomorphism, check every node, and the structural predicate certifies the refusal

> Where the realisation map is not a homomorphism, the check visits every node. Where every leaf occurs
> at most once **and** no internal node has an ancestor multiplication whose sibling interval contains
> zero, the per-node interval rule is **exact**: it refuses only where an enumerating oracle also
> refuses.

This is `111` F111-15 with `wrap` removed and four dimensions added. Conditions (a) and (b) are
`111`'s, unchanged.

**Predicate.** `W in {2, 3}, F = 0, signedness in {unsigned, signed}, overflow policy = sat, rounding =
trunc, radix = 2, operations in {add, sub, mul}, term shapes = every term at 2 and 3 leaf slots
exhaustively and 120 of 2025 sampled at 4, arity in {2, 3, 4}, declarations in {one-sided [0, b]
exhaustive, two-endpoint exhaustive at arity 2, two-endpoint sampled 4000 per term at arity 3},
discharge check = per node, threads = 1, target features any`.

**Evidence.** `p1_output.txt`, violations zero on every saturating row:

| sweep | cells | conservative | fires | violations |
|---|---|---|---|---|
| uW3/sat, one-sided, arity 2 and 3 | 13032 | 703 | 6336 | 0 |
| iW3/sat, one-sided, arity 2 and 3 | 2148 | 93 | 816 | 0 |
| uW3/sat, two-endpoint, arity 2 | 3996 | 28 | 3888 | 0 |
| iW3/sat, two-endpoint, arity 2 | 3996 | 10 | 3888 | 0 |
| uW3/sat, two-endpoint, arity 3, sampled | 360000 | 36629 | 66661 | 0 |
| iW3/sat, two-endpoint, arity 3, sampled | 360000 | 35039 | 58555 | 0 |
| uW2/sat, one-sided, arity 4, sampled | 5964 | 575 | 1024 | 0 |
| uW3/sat, one-sided, arity 4, sampled | 22516 | 1843 | 1500 | 0 |

**Three controls, all firing.** Dropping condition (b) produces 234, 20, 534, 385, 172 and 69
violations on the six rows where it can; dropping condition (a) produces 307, 49, 28, 10, 32343, 27258,
106 and 379 on all eight; and a predicate that always fires produces violations equal to the
conservative count on every row. So neither condition is decoration and the sweep contains cells the
predicate could have been wrong about.

The two rows missing from the first list are the two-endpoint arity-two sweeps, where dropping (b)
produces zero violations. That is not the control failing, it is (b) being **vacuous** at arity two: a
two-leaf term has one internal node, that node is the root, and condition (b) excludes the root because
a root has nothing downstream to be masked by. So (b) has no work to do there and its control cannot
fire, which is a fact about the arity rather than about the condition.

**The stronger claim, which is the one a design wants.** Where the predicate fires and the per-node
rule refuses, an **enumerating oracle also refuses**, on 5384 of 5384 cells at `uW3/sat`, 316 of 316 at
`iW3/sat`, 63037 of 63037 at arity three unsigned, 50258 of 50258 at arity three signed, and 2988 and
2408 on the two-endpoint arity-two rows. So the predicate does not merely certify "the corner rule is
not losing anything here". It certifies "no rule could do better here", which is what makes it a
selector rather than a hint.

**Why this is a proof and not a count.** The argument is written out in `p1`'s header and is worth
carrying, because a canon that says a predicate holds should say why. Condition (a) makes the two
children of every node vary independently, so the corner enumeration attains the reachable extremes and
the propagated interval's endpoints **are** the reachable extremes; hence "the rule refuses" is exactly
"some node overflows on some tuple". Condition (b) forbids the only masking available in this
signature, so "some node overflows" is exactly "the arms disagree". Chaining the two gives
"refuses if and only if the arms disagree", which forbids conservatism.

**The weaker half is condition (b), and I say so.** It names the multiplicative annihilator because
that is the only absorbing element in `{add, sub, mul}`. A signature carrying an idempotent, a clamp or
any other absorbing element needs (b) restated, and this sweep cannot see that because it does not
sweep such a signature.

### Arm S2. Where the predicate does not fire, intersect per node

> Where the map is not a homomorphism and the structural predicate does not fire, the tightest sound
> rule I could build is the **per-node intersection** of the interval rule and `112`'s one-sided affine
> form: at every node, propagate both and keep the intersection, which is sound because both
> over-approximate the same reachable set.

**Predicate.** `W = 3, F = 0, signedness in {unsigned, signed}, overflow policy in {sat, wrap},
rounding = trunc, radix = 2, operations in {add, sub, mul}, term shapes = every term at 2 and 3 leaf
slots, arity in {2, 3}, declarations in {one-sided [0, b] exhaustive, two-endpoint exhaustive at arity
2, two-endpoint sampled 500 per term at arity 3}, discharge check = per node, threads = 1, target
features any`.

**Evidence.** `p4_output.txt`. Unsound on 0 cells on every row, with a deliberately unsound halved rule
reported unsound on 245 to 5689 so the counter is live. Beaten by the interval rule on 0 cells and by
the one-sided form on 0 cells, on every row, including the two-endpoint rows where the one-sided form
alone is beaten on 92 and 593.

**It is strictly better than disjoining the two verdicts**, which is what `112` section 9 offers, and
the margin is small: 1, 4, 1 and 2 cells across the rows where it appears at all, always on a
multiplication term with a correlated leaf. The reason to prefer it anyway is that it is free. A design
that disjoins is already computing both forms, and intersecting at each node rather than at the verdict
costs nothing extra and cannot be worse.

**One of my controls here was mis-designed and I am reporting it as such.** I predicted that taking the
**union** of the two intervals instead of the intersection would be unsound, and it is not: a union of
two sound over-approximations still contains the reachable set, so its containment test can only be
conservative. It cannot go unsound and my expectation was wrong on the arithmetic. What the union row
does show is that the direction matters, licensing 2247 against the intersection's 2663 and 3156
against 5669. That is a real control on the direction and a dead one on soundness, and the live
soundness control is the halved rule.

---

## 5. The bounds, named where they are and named where they are not

The brief asks where each arm holds, where the selecting predicate decides correctly, and where the
boundary is unmeasured. Taking those in order.

### 5.1 Where the union of the two sweeps reaches, and where it does not

`111` swept twelve hand-picked rows at `W = 4` with one-sided declarations. `112` swept thirteen at `W
<= 4`, all one-sided from zero by its own F112-23, with three rows whose leaves straddle zero. This
file sweeps every term at two and three leaf slots over `{add, sub, mul}` with every leaf
identification, which is 96 terms, plus 120 of the 2025 at four leaf slots, at `W in {2, 3}`, under
both declaration shapes and both overflow policies.

**Together they cover:** arity 2 to 4; `W` 2 to 4; both signednesses; both overflow policies; one-sided
and two-endpoint declarations; the ring operations.

**Together they do not cover, and nothing in this panel does:**

- **`F > 0`.** Every sweep in this unit is at `F = 0`. `112` F112-4 is the one exception and it is
  about which region of the realisation map a grade switches off, not about propagation. The corner
  rule and the affine rule both propagate intervals, and a fraction grid adds a second quantity that
  `111` F111-10 names and nobody has swept propagation against.
- **Non-uniform value sets**, which is `110`'s stated largest gap and is where arvo's float side lives.
  Every primitive in this file has uniform spacing.
- **Any real width.** `W <= 6` with no transfer argument, as above.
- **Signatures outside the ring.** `p2`'s C2 measures that `min` breaks arm W0 and W1, which bounds
  them; it does not tell you what replaces them.
- **Arity above 4**, except for the folds in section 6.2, which are a single shape.

### 5.2 Where the selecting predicate decides correctly, and what it costs when it does not

The predicate is **sufficient and not necessary**, which `111` says. Nobody had measured how far from
necessary, and the answer is: far.

From `p1`, the column counting cells where the corner rule is **not** conservative and the predicate
declines to say so:

| sweep | exact cells | predicate declines | disjunctive form declines |
|---|---|---|---|
| uW3/sat, one-sided | 12329 | 5993 | 4698 |
| iW3/sat, one-sided | 2055 | 1239 | 494 |
| uW3/sat, two-endpoint, arity 3 | 323371 | 256710 | 225930 |
| uW2/sat, one-sided, arity 4 | 5389 | 4365 | 3303 |

The third column is a repair I would offer. **`corner licenses OR (condition (a) and condition (b))`**
is sound wherever the predicate is, fires strictly more often, and is the predicate a selector actually
wants, because a cell where the cheap rule already licensed is a cell where nothing was lost whatever
the shape. It closes 12% to 60% of the gap and it is const in exactly the same way.

**But incompleteness is not the number that decides anything, and I want to be careful not to sell it
as one.** The operative question is how often a design reaches the expensive arm and how often the
expensive arm pays when reached. `p5` measures the three-step selector directly:

```
  primitive    decls           cells   step1   step2   step3  step3 pays   lost
  uW3/sat      one-sided       13032    2247    5384    5401         416    287
  iW3/sat      one-sided        2148    1245     316     587          61     32
  uW3/sat      two-endpoint    30756    2860    7202   20694        2031    143
  iW3/sat      two-endpoint    30756    5840    5727   19189        2043    242
```

Step one is the cheap rule licensing. Step two is the predicate certifying an honest refusal, which
needs no expensive machinery. Step three is where the coefficient vector is paid for.

So the expensive arm is reached on 27% to 67% of cells and **pays on 8% to 11% of those**, which is 3%
to 7% of all cells. The `lost` column is the ceiling nothing per-node reaches: 32 to 287 cells, the
annihilation residue.

That is the number a design should be looking at, and I do not think anyone had it.

### 5.3 `112` F112-24's domination, bounded

`112` F112-24 reports the one-sided form "beaten on none" over thirteen rows. Over the systematic
enumeration (`p3_output.txt`):

```
  uW3/sat, one-sided declarations       beaten on   0 cells
  iW3/sat, one-sided declarations       beaten on   0 cells
  uW2/sat, one-sided, arity 4           beaten on   0 cells
  uW3/wrap, one-sided                   beaten on   0 cells
  uW3/sat, TWO-ENDPOINT, arity 3        beaten on  43 cells
  iW3/sat, TWO-ENDPOINT, arity 2        beaten on  92 cells, all on x * y and x * x
  iW3/sat, TWO-ENDPOINT, arity 3        beaten on 593 cells
```

**So the domination is a fact about one-sided declarations**, and `112` F112-23 already records that
those are the only ones that file swept. The two findings sit fourteen lines apart in the same list and
neither points at the other.

**The mechanism, because a count is not a finding.** A leaf declared `[lo, hi]` becomes `lo + (hi -
lo) e` with the constant at the lower bound and a non-negative coefficient. Multiplying two such forms
scales each side's coefficients by the **other side's constant**, so a negative constant flips their
sign and the interval spreads both ways. Worked, at `iW3` with both leaves declared `[-4, -1]`:

    x = -4 + 3 e1,  y = -4 + 3 e2
    x*y = 16 + (3 * -4) e1 + (3 * -4) e2 + cross,   cross in [0, 9]
        interval [16 - 12 - 12, 16 + 9] = [-8, 25]
    corner: [(-1)(-1), (-4)(-4)] = [1, 16]          exact

`112`'s three straddling rows use leaves declared `[-4, b]` with `b` positive, which keeps a large
positive coefficient and makes the loss small. A leaf declared entirely below zero has no positive part
at all, and that is the case its sweep could not contain.

**And the repair is arm S2**, which recovers all 92 and all 593.

### 5.4 What would decide any of this against me

Stated because I could be wrong in ways the sweeps cannot see.

**If a transfer argument to real widths does not exist**, everything enumerative here is a statement
about `W <= 6` and the design gets nothing from it directly. I have no such argument and I did not
look for one. The two structural results, arm W0's ring argument and arm S1's exactness proof, are
width-independent by construction and are the only things in this file I would carry to a real width
without further work.

**If `F > 0` changes the propagation question qualitatively**, rather than adding a second propagated
quantity, then arm S1 and arm S2 are about a special case. `111` F111-10 and `112` F112-4 both suggest
the fraction grid is a separate part of the declaration switching off a separate region of the map,
which would make it additive. That is an expectation and not a result.

**If a real consumer's terms are not trees**, because a value is used twice at the source level and the
compiler sees a DAG, then leaf multiplicity is not a syntactic property and condition (a) is not
computable the way `p7` computes it. Nothing in this panel has looked at that and it is the assumption
I am least comfortable with.

---

## 6. The holes the brief named, plugged

### 6.1 Is the structural predicate necessary as well as sufficient

**No, and the gap is large**, quantified in section 5.2. It is sufficient at `overflow policy = sat`
across every sweep in `p1`, and it is unsound at `wrap`, which section 3.1 covers.

Two things worth adding.

**The disjunctive form is the better selector** and is stated in 5.2. It is sound, strictly more
complete, and const in the same way.

**Necessity is the wrong thing to want here.** A necessary and sufficient condition for "the corner
rule is exact on this cell" would be a decision procedure for a property that quantifies over the whole
declaration box, and `112` section 6 already establishes that the enumerating oracle "is not available
as a const predicate at a real width" because its domain is `2^(W k)`. So the structural predicate is a
sound incomplete decision procedure by necessity rather than by choice, and the useful question is how
much it declines, not whether it declines.

### 6.2 Does the corner rule's two-number state claim hold at fold lengths other than 64

**Yes, and the more useful answer is that the exactness holds too, which nobody had checked at any
length.** `111` F111-18 reports the state counts; the claim the cheap arm actually rests on is that the
corner rule stays exact as the fold grows, and that had no evidence at all.

`p5_output.txt`, at `uW2/sat`, declarations sampled without replacement:

```
  term                            cells   consv  unsound   fires  root uns  corner  affine
  fold of 2 adds                     16       0        0      16         0       2       2
  fold of 3 adds                     60       0        0      60         0       2       3
  ...
  fold of 8 adds                     60       0        0      60         0       2       8
  fold of 3, alternating +/-         60       0        0      60         0       2       3
  ...
  fold of 8, alternating +/-         60       0        0      60         0       2       8
```

Zero conservative and the predicate firing on every cell, at every length from two to eight, for pure
addition and for alternating addition and subtraction, reproduced at `uW3/sat` for lengths two to five.
That is what condition (a) predicts, since a fold has distinct leaves and no multiplication.

**Two controls, both firing.** A fold with every leaf identified makes the predicate stop firing, 0 of
4 cells at every length, so it is condition (a) doing the work. A fold under a multiplication is
conservative on 5, 13, 15 and 12 cells with the predicate declining every one, so the exactness check
can report a failure.

The state counts are properties of the term with neither the width nor the declaration in them, so they
extend past any sweep: 2 against L at every L, and 2 against 1024 on a 1024-element fold.

**But the state count is not the compile-side cost**, and section 7 is that correction.

### 6.3 Does the per-construction lifting rule generalise past three constructions

**Yes, and it is a formula rather than a table.** This is the hole I most wanted to close, because the
difference decides what a canon can say: a table is a design obligation that grows with every
construction anyone adds, and a formula is a sentence.

A bilinear construction on `d`-tuples is fixed by its structure constants, with output component `i`
being the sum over `j` and `k` of `c[i][j][k] * a_j * b_k`. Under a magnitude declaration bounding
every component by `m`, each product lies in `[-m^2, m^2]` over a signed base and `[0, m^2]` over an
unsigned one. So:

> over a **signed** base, component `i` lies in `[-N_i m^2, N_i m^2]` where `N_i` is the **L1 norm** of
> that component's structure-constant row;
>
> over an **unsigned** base, if the row has any negative entry the lower end is below zero, which the
> carrier cannot hold, so **no magnitude-only rule discharges it at any nonzero `m`**; otherwise the
> bound is `N_i m^2`.

One number per output component and one bit, read against the base's signedness.

**It reproduces `112` F112-14 without measuring it**: `product2` has rows of norm 1 with no negatives,
`dual` has norms 1 and 2, `complex` has norms 2 and 2 with a negative in row zero. Componentwise, twice
componentwise, twice componentwise over a signed base and nothing over an unsigned one. Exactly the
table `112` measured.

**Tested on the family rather than on those three.** `p6_output.txt`, over 1024 bilinear products on
pairs with structure constants in `{-1, 0, 1}` and one or two nonzero entries per row:

```
  base uW3/sat   cells 8192   arms agree on 1140
    FORMULA          discharges   1140   unsound     0   conservative      0
    C1 componentwise discharges   3072   unsound  1932
    C2 unit norm     discharges   1224   unsound    84

  base iW5/sat   cells 750 (150 sampled constructions, m in 0..4)   arms agree on 458
    FORMULA          discharges    458   unsound     0   conservative      0
    C1 componentwise discharges    600   unsound   142
    C2 unit norm     discharges    600   unsound   142
```

**The formula is exact, not merely sound.** Zero unsound and zero conservative, meaning it discharges
exactly the cells on which the arms agree. I predicted it would be conservative somewhere, on the
ground that two components of one operand cannot independently attain their extremes when a row
contains the same component twice with opposite sign. **That prediction is refuted** and I record it
rather than quietly dropping it.

**At dimension four**, where nothing in the derivation mentions the dimension, Hamilton's quaternion
product has L1 norm 4 on every row and a negative entry on every row. The formula tracks the arms
exactly at `iW3`, `iW4` and `uW3`, while the componentwise rule is unsound at `m = 1` and `m = 2`.

**The scope, stated because it is narrower than it looks.** This covers **bilinear** constructions on
tuples. It does not cover `interval`, whose product is a hull rather than a bilinear form, and which
`110` F12 and `112` F112-12 handle with a monotonicity predicate instead. So the canon sentence is
about bilinear constructions and a non-bilinear one remains its own obligation. I did not find a
formula covering both and I do not think one exists, because the two constructions differ in kind
rather than in structure constants.

**The `iW3/sat` row in the table above is not evidence**, and I say so where it appears. At that base
the container is too narrow for the L1 norm to matter: every rule collapses to `m <= 1` and both
controls report zero unsound. That is a defect in the sweep rather than support for the formula, which
is why the wider signed bases were added.

---

### 6.4 The one grep I owed, and the shipped kernel turns out to be arm W1 already

I nearly left this as an owed check. It is one grep, it closes the loop between four sections of model
sweeps and something that actually runs, and it is the strongest corroboration in this file.

`warm-clamp-shared/src/lib.rs:289-311` is a chunked clamping fold with **two branches selected by a
const predicate**:

```
    let safe = accumulator_bits_needed(W, ARITY) <= A::BITS;
    ...
        if safe {
            for &x in chunk { acc = acc.wadd(...); }
            acc = acc.min_with(limit);
        } else {
            for &x in chunk { acc = acc.sat_add(...).min_with(limit); }
        }
```

The `safe` branch does **wrapping** additions throughout and reduces **once** at the end of the chunk.
The other branch reduces at **every** node. Those are arm W0's interior and the general arm, and the
thing choosing between them is a const predicate on the width and the arity.

**And the predicate is the corner rule at the root of a fold.**
`accumulator_bits_needed` at `lib.rs:158-160` is `w + ceil_log2(arity)`, which is the bit width of
`arity * (2^w - 1)`, which is the upper endpoint of the corner rule's propagated interval for a fold of
`arity` leaves each declared at the full width. So the repository already ships arm W1's root check,
hand-derived, specialised to one term shape, with the leaf declaration left implicit and maximal.

Three things follow, and the third is what the refinement is actually for.

**The fold is where arm S1's structural predicate fires by construction**, which section 6.2 measured
and section 0.2 verified at source: distinct leaves, no multiplication. So the corner rule is exact
there and no more expensive rule can license anything it refuses. The cheap arm is not a compromise on
that shape, it is the whole answer.

**Arm W0 stops at the clamp, and the kernel's own shape shows where.** `min_with` is not a ring
operation, so the licence covers the chunk's interior and not across the chunk boundary. `p2`'s C2
measures exactly this: adding `min` to the signature breaks the homomorphism on 168 of 2304 operand
pairs. A design reading arm W0 as "wrapping arithmetic needs no intermediate reduction" and applying it
across a clamp would be wrong, and the counting is in the probe rather than in an argument.

**What a declared extent adds is the difference between `w` and the declared bound's width.** The
shipped predicate takes every leaf at the full `w` because it has nothing better. A refinement replaces
`w` with the declared upper bound's bit width, and the same formula then licenses the cheap branch in
cases the width computation refuses. At `W = 13` and arity 64 the shipped predicate needs `13 + 6 = 19`
bits and reaches for a 32-bit accumulator; with the leaves declared at most 100 it needs `7 + 6 = 13`
and a 16-bit one suffices. That is the same arithmetic, not a measurement, and **what it is worth is
unpriced**: it is a claim about which accumulator rung is selected, and whether a narrower rung is
faster is exactly what `warm-clamp` exists to measure and what I did not run.

I would put that in front of whoever picks up the consumer side. It is the first place in this sitting
where the refinement machinery has a shipped consumer, a shipped predicate it would replace, and a
harness already built to price the replacement.

---

## 7. The compile-time question, answered without a clock, and the answer refutes a shape

`111` section 26 carries this as the one open item "that a harness rather than an argument closes":
disjunction against static selection, a compile-time cost nothing has priced.

### 7.1 Why the duration cannot be priced, precisely

`evidence-lives-in-the-repo-or-it-never-happened.md` is explicit that where mockspace is used, a
measurement outside `mock/benches/` on the harness "can not be named 'bench' or 'benchmark' or anything
similar", and that a compile-time figure taken anywhere else is "an ad-hoc quick spike with no
substance".

**The harness cannot take this measurement.** Its schema is entirely runtime. The committed CSV header,
from `mock/benches/bitpack-carrier-width_n16384.csv`:

```
run,pass,cooldown_ms,mode,variant,batch_idx,e2e_ns,algo_ns,bridge_ns,batch_count,score,
input_tag,instructions,cycles,setup_ns,first_ns,digest
```

and the meta beside it records `cpu`, `os`, `rustc`, `git_commit`, `timestamp`, `counter_freq`,
`framework`. No compile-time column, no compile-time mode, and no build phase it times: it loads each
variant as a prebuilt cdylib and measures calls into it.

So the duration is unpriced, and the instrument that would price it is a compile-time mode the harness
does not have. Building one is mockspace's work rather than arvo's, and I did not attempt it.

**That is half an answer, and a concession is not the end of a hole.** What follows is what can be
established without a duration.

### 7.2 The count, which is arithmetic and corrects how F111-18 will be read

For a fold of `L` leaves, with `2L - 1` nodes:

```
   fold L   nodes   corner consts   affine cells    ratio
        8      15              30            120      4.0
       64     127             254           8128     32.0
     1024    2047            4094        2096128    512.0
```

**`111` F111-18 reports the affine state as one coefficient per distinct leaf, which is the state at
one node.** The type-level tower carries a vector at **every** node, so the compile-side quantity is
the product, not the factor. At a 64-element fold the honest figure is 8128 associated-const cells
against 254, not 64 against 2. That is arithmetic rather than a measurement and it is a correction to
how a true finding will be read.

### 7.3 The refusal, which is deterministic and is not a duration

Whether a spelling compiles at a given fold length is a reproducible fact about the source and the
pinned toolchain. `p9` generates both towers at increasing lengths and compiles each.

```
  variant                  2     8    16    24    32    48    64    96   128
  neither                 ok    ok    ok    ok    ok    ok    ok    ok    ok
  corner                  ok    ok    ok    ok    ok    ok    ok    ok  FAIL
  affine-types-only       ok    ok    ok    ok    ok    ok    ok    ok  FAIL
  affine                  ok    ok    ok    ok    ok    ok  FAIL  FAIL  FAIL
  disjunction             ok    ok    ok    ok    ok    ok  FAIL  FAIL  FAIL
  selection               ok    ok    ok    ok    ok    ok  FAIL  FAIL  FAIL
  affine-impl-only        ok    ok    ok    ok    ok    ok  FAIL  FAIL  FAIL
  selection-assoc         ok    ok    ok    ok    ok    ok  FAIL  FAIL  FAIL
  selection-early         ok    ok    ok    ok    ok    ok    ok    ok  FAIL
  affine-limit-1024       ok    ok    ok    ok    ok    ok    ok    ok    ok
  disjunction-limit-1024  ok    ok    ok    ok    ok    ok    ok    ok    ok
```

with, at the first refusal,

```
    corner            L = 128   error[E0275]: overflow evaluating the requirement `ILeaf<0, 3>: Iv`
    affine            L =  64   error[E0275]: overflow evaluating the requirement `Cons<Lit<0>, Nil>: Coeffs`
    selection         L =  64   the same
    affine-impl-only  L =  64   the same
    selection-assoc   L =  64   the same
    selection-early   L = 128   the corner diagnostic
```

Six things fall out, and the third is the one that changes a design.

**The wall is trait solving, not const evaluation.** `affine-types-only` names both towers' types and
never reads a const of the affine one, and it compiles as far as `corner` does. So constructing the
types is cheap and requiring the recursive obligation is what costs.

**The affine tower reaches any fixed recursion limit at half the fold length**, because its obligation
chain is the spine depth plus the vector length rather than the spine depth alone. `corner` fails at
`L = 128` and `affine` at `L = 64` against rustc's default limit of 128, which is exactly that.

**Static selection at the use site does not avoid the cost, and `111` section 21 proposed it partly
because it would.** `selection` puts the choice inside a `const` block and fails at the same length as
`disjunction`. `selection-assoc`, which routes the choice through an associated type so the unselected
arm is never named at the site, fails identically. **And `affine-impl-only`, which merely defines an
impl whose const body reads the affine tower and never reads that const, fails at 64 too.** So the
obligation is forced when the reading code is **defined**, not when it is evaluated, and no arrangement
of the choice at the use site escapes it.

**Selecting before instantiation does work.** `selection-early`, where the affine machinery is fully
present in the crate and simply never instantiated at that term, compiles as far as `corner` does and
fails with `corner`'s diagnostic. So the composition is real and the const `if` was in the wrong place:
**the predicate has to gate which type is constructed, not which const is read.**

**And the wall is a configurable limit.** `#![recursion_limit = "1024"]` makes both the affine tower
and the disjunction compile at every length up to 128. So the compile-side difference is not a hard
ceiling; raising the attribute converts it back into a duration, which is the thing that cannot be
priced here.

**My own prediction P4 was refuted.** I predicted static selection would compile wherever the corner
spelling does. It does not, and finding out why is worth more than the prediction was.

**The control fired and caught a defect in my own generator.** The first run had every variant failing
at every length on `error[E0601]: main function not found`, because I compiled without `--crate-type
lib`. The `neither` row is what made that visible immediately rather than after I had written a
conclusion about towers.

### 7.4 So what the design should conclude, and what stays unpriced

**Conclude:** the choice between disjoining and selecting is not free at compile time, the cost is
trait-solving recursion depth, it is proportional to the fold length times the leaf count, and it is
avoided only by keeping the expensive form uninstantiated at that term rather than by choosing between
two instantiated forms.

**Unpriced, and stated as such:** every duration, for every spelling, at every length. No bench ran and
none could. If op wants the duration, the honest next step is a compile-time mode in the bench harness,
which is mockspace's tree.

---

## 8. What the canon should state

Written to compose with `108` section 7 and `112` section 9 rather than to replace either, with the two
clauses this file corrects rewritten and nothing else touched. Suggestions.

> A **realisation map** is a **homomorphism** for a set of operations when reducing at every step and
> reducing once at the end give the same result. Wrapping is one for the ring operations. Saturating,
> clamping and rounding are not.
>
> Where the map is a homomorphism for every operation in a term, **the intermediate reductions carry no
> information and may be dropped with no declaration at all**, and licensing an arm that does not
> reduce at all depends on the **root** alone. Where it is not, every node of the derivation must be
> checked, and checking only the result is unsound. **Which of the two applies is a property of the
> declared semantics and is therefore known before any value exists.**
>
> A **propagation rule** is sound when it over-approximates the reachable set at every node it is
> checked at. No sound rule is uniquely best: one loses where a leaf repeats, another where quantities
> multiply, and a third where a declared bound lies wholly below zero. Sound rules **intersect at each
> node into a sound rule**, which is at least as tight as disjoining their verdicts, so a design that
> carries two carries their intersection rather than their disjunction.
>
> A rule's **exactness has a structural predicate**, decidable from the term and its declared extents:
> where every leaf occurs at most once, and no internal node sits under a multiplication whose sibling
> interval contains zero, the interval rule refuses only where no rule could have licensed. Under a
> non-homomorphic map that predicate is a certificate that a refusal is honest, and it is what lets a
> design decline to instantiate a more expensive rule rather than merely decline to consult it.
>
> A **construction on primitives carries its own grade transformer**, and for a bilinear construction
> the transformer is determined by the **L1 norm of each output component's structure-constant row**,
> together with whether that row has a negative entry, read against the base's signedness. A row with a
> negative entry over a carrier with no negative values is not dischargeable by any magnitude bound.
> Borrowing the base's own rule is unsound.
>
> **The cost of a propagation rule is paid where its carrier is instantiated, not where its verdict is
> read.** A design selects between rules by choosing which carrier a term is given, because a choice
> made after both carriers exist has already paid for both.

**Permanence.** Every sentence survives a rewrite in another language or decade. None names a
container, a width, a marker, a type parameter, a crate, or a count.

**Equivalence.** Three teams implementing this produce units that behave the same on what matters: a
homomorphic map never pays for an intermediate check; a non-homomorphic one always does; a design
carrying two rules intersects them rather than disjoining them; a construction refuses a lift its own
transformer does not license; and the expensive carrier is not built where a cheaper one suffices. They
differ in how the predicate is spelled, how many rules ship, and whether the selection lives in a trait
or a macro.

**Where it is weaker than I would like.** Everything enumerative is at `W <= 6` with no transfer
argument. The homomorphism clause and the exactness proof are the two width-independent parts and are
the two I would defend at a real width. Nothing here covers `F > 0`, non-uniform value sets, or a term
that is a DAG rather than a tree.

---

## 9. Findings, each with its predicate

The three conventions in section 4 apply to all of them.

**F114-1. A wrapping realisation map is a ring homomorphism and a saturating one is not.** Exhaustive
over the whole ambient range: 0 mismatches of 1728 at `uW3/wrap`, 0 of 1728 at `iW3/wrap`, 0 of 6912 at
`uW4/wrap`; 400 of 1728 at `uW3/sat` and 464 of 1728 at `iW3/sat`. `W in {3, 4}, F = 0, signedness in
{unsigned, signed}, overflow policy in {sat, wrap}, rounding = trunc, radix = 2, operations in {add,
sub, mul}, threads = 1, target features any`. `p2_output.txt`. The saturating rows are the control.

**F114-2. Under a homomorphic map over ring operations the arms agree exactly when the root's exact
value is in range, with no reference to any intermediate node.** 0 disagreeing cells of 13032 at
`uW3/wrap` and 0 of 2148 at `iW3/wrap`, against 260 and 64 at `sat`. Predicate as arm W1 in section 4.
`p2_output.txt`.

**F114-3. `111` F111-15's structural predicate is unsound at `wrap`.** 28 violations of 13032 cells at
`uW3/wrap` and 16 of 2148 at `iW3/wrap`, where a violation is the predicate certifying exactness on a
cell where the per-node rule is conservative. Every witness is a term with a sign excursion under an
unsigned container, of which `(x - (y - z))` is the smallest. `W = 3, F = 0, signedness in {unsigned,
signed}, overflow policy = wrap, rounding = trunc, radix = 2, operations in {add, sub, mul}, term
shapes = every term at 2 and 3 leaf slots, arity in {2, 3}, declarations = one-sided [0, b] exhaustive,
discharge check = per node, threads = 1, target features any`. `p1_output.txt`, `p2_output.txt`. This
**refutes F111-15 at one of the two overflow policies its predicate lists**, and it does not touch the
finding at `sat`.

**F114-4. The same predicate is sound at `sat` across a far wider region than it claimed.** Zero
violations over 96 terms at two and three leaf slots exhaustively, 120 of 2025 sampled at four leaf
slots, both signednesses, `W in {2, 3}`, and both one-sided and two-endpoint declarations, totalling
771652 cells. Predicate as arm S1 in section 4. `p1_output.txt`. Three controls fire: dropping either
condition produces violations on every row, and a predicate that always fires produces violations equal
to the conservative count.

**F114-5. Where the predicate fires and the per-node rule refuses, an enumerating oracle also
refuses.** 5384 of 5384, 316 of 316, 2988 of 2988, 2408 of 2408, 63037 of 63037 and 50258 of 50258
across the saturating sweeps. Same predicate as F114-4. `p1_output.txt`. This is stronger than
"the rule is not conservative here" and is what makes the predicate a selector.

**F114-6. Under a homomorphic map the root-only check is sound everywhere and exact wherever condition
(a) holds, and it licenses more than any per-node rule.** 0 unsound of 13032, 2148 and 38256 cells; 0
disagreements over the 9408 and 1200 cells where condition (a) holds, against 28 and 16 at `sat`;
licensing 3188 of an available 3210, 8688 of 8744 and 12697 of 13056 against a per-node hybrid's 2663,
6080 and 9824. Predicate as arm W1. `p2_output.txt`, `p5_output.txt`. Control: the same rule at `sat`
is unsound on 259 and 2749 cells. This **bounds `112` section 9's unqualified sentence at `112:928`**
and is consistent with F112-21, whose own predicate lists `overflow policy = sat`.

**F114-7. It is the ring and not the wrapping.** Adding `min` to the signature gives 168 homomorphism
mismatches of 2304 and moves the root-decides-everything gap from 0 to 591 of 9496 cells at `uW3/wrap`
and 72 of 1500 at `iW3/wrap`. `W = 3, F = 0, signedness in {unsigned, signed}, overflow policy = wrap,
rounding = trunc, radix = 2, operations in {add, sub, mul, min}, term shapes = 60 sampled of 384 at 3
leaf slots, arity in {2, 3}, declarations = one-sided, threads = 1, target features any`.
`p2_output.txt`.

**F114-8. Dropping every intermediate reduction and reducing once at the root is unconditionally equal
to reducing at every node, under a homomorphic map over ring operations.** 0 differing cells of 576 at
carrier widths 3, 4, 5 and 8, including the 363 cells where the root's exact value leaves the logical
range. Predicate as arm W0. `p8_output.txt`. Controls: the same at `sat` differs on 26 cells, and a
wrapping logical map over a **saturating carrier** differs on 189 to 322, so a mixed carrier breaks it.

**F114-9. The unreduced arm's licence is exactly the root check, measured rather than argued.** Under
wrap at carrier widths 4, 5 and 8, the unreduced arm differs from the general arm on exactly 363 cells,
which is exactly the count of cells where the root's exact value leaves range. Same predicate as F114-8.
`p8_output.txt`.

**F114-10. `112` F112-24's domination is a fact about one-sided declarations.** The one-sided form is
beaten by the interval rule on 0 cells under one-sided declarations at `uW3/sat`, `iW3/sat`, `uW2/sat`
at arity four and both wrap bases; and on 92 cells at `iW3/sat` arity two, 43 at `uW3/sat` arity three
and 593 at `iW3/sat` arity three under two-endpoint declarations. Predicate as arm S2 in section 4.
`p3_output.txt`. The mechanism is a negative constant flipping the sign of the coefficients it scales,
worked in section 5.3.

**F114-11. The per-node intersection of the two rules is sound and is beaten by neither, on every row
swept.** 0 unsound; 0 cells where the interval rule licenses and it does not; 0 where the one-sided form
licenses and it does not. It beats the disjunction of the two verdicts on 1, 4, 1 and 2 cells. Predicate
as arm S2. `p4_output.txt`. Control: a halved-radius rule is reported unsound on 245 to 5689 cells.

**F114-12. The bilinear lifting rule is a formula over the structure constants, and it is exact.** Zero
unsound and zero conservative over 1024 bilinear products on pairs at `uW3/sat` (8192 cells),
`iW3/sat` (4096) and `uW3/wrap` (8192), and over 150 sampled at `iW5/sat` and `iW6/sat` (750 each).
The rule is the L1 norm of each output component's structure-constant row, plus whether that row has a
negative entry, read against the base's signedness. `W in {3, 5, 6}, F = 0, signedness in {unsigned,
signed}, overflow policy in {sat, wrap}, radix = 2, dimension = 2, structure constants in {-1, 0, 1}
with one or two nonzero entries per row, operation = mul, arity = 2, declarations = a uniform
magnitude bound on every component, threads = 1, target features any`. `p6_output.txt`. Controls: the
componentwise rule is unsound on 1932 and 142 cells, and a unit-norm variant on 84 and 142.

**F114-13. The same formula holds at dimension four.** Hamilton's quaternion product tracks the arms
exactly at `iW3/sat`, `iW4/sat` and `uW3/sat`, while the componentwise rule is unsound at `m = 1` and
`m = 2`. `W in {3, 4}, F = 0, signedness in {unsigned, signed}, overflow policy = sat, radix = 2,
dimension = 4, construction = quaternion, operation = mul, arity = 2, magnitude bound in 0..3, threads
= 1, target features any`. `p6_output.txt`.

**F114-14. The corner rule is exact on a fold at every length checked, and the structural predicate
fires on every cell of every such fold.** Zero conservative and zero unsound at lengths two through
eight for pure addition and alternating addition and subtraction, with the predicate firing on 100% of
cells. `W in {2, 3}, F = 0, signedness = unsigned, overflow policy = sat, rounding = trunc, radix = 2,
operations in {add, sub}, term = a left-nested fold, arity in 2..8, declarations = one-sided [0, b]
sampled without replacement at 40 to 60 per length, discharge check = per node, threads = 1, target
features any`. `p5_output.txt`. Controls: a fold with every leaf identified makes the predicate stop
firing, and a fold under a multiplication is conservative on 5 to 15 cells with the predicate declining
each.

**F114-15. The structural predicate and the policy-selected discharge check both compile with no
feature gate.** Condition (b), which reads as a top-down property, is equivalent to a local check at
each multiplication node and is therefore an associated const with nothing arithmetic in type position.
`toolchain = nightly-2026-05-28, rustc 1.98.0-nightly (57d06900f 2026-05-27), edition 2021, feature
gates = none, no dyn, no TypeId, threads any, target features any`. `p7_output.txt`. This closes an
expressibility claim `111` section 19.2 explicitly declined to make.

**F114-16. The gated arms erase in both directions, by symbol aliasing.** `_general_masked` aliases
`_cheap_reduced_ungated`, `_wrap_gated` aliases `_cheap_unreduced_ungated`, and `_sat_gated` aliases
`_general_saturating`. `toolchain = nightly-2026-05-28, edition 2021, feature gates = none, target =
aarch64-apple-darwin, target features = host default, opt level = 3, container = u8, logical width = 5,
term = x - (y - z), threads any`. `p7_asm.s`, `p7_asm_bodies.txt`. The instruction counts, 3 against 4
against 15, are an ad-hoc quick spike as far as magnitude goes and price nothing.

**F114-17. The compile-side cost of the affine tower is trait-solving recursion, it reaches a fixed
limit at half the fold length the interval tower does, and static selection at the use site does not
avoid it.** `corner` compiles to `L = 96` and refuses at 128; `affine`, `disjunction`, `selection`,
`selection-assoc` and an impl that merely defines a const reading the tower all refuse at `L = 64` with
`E0275`. Naming the types without requiring the obligation compiles to 96. Selecting before
instantiation compiles to 96 with the interval tower's own diagnostic. Raising `recursion_limit` to
1024 makes every variant compile to 128. `toolchain = nightly-2026-05-28, edition 2021, crate-type =
lib, feature gates = none, term = a left-nested fold of adds, fold length in {2, 8, 16, 24, 32, 48, 64,
96, 128}, threads any, target features any`. `p9_output.txt`. Control: a variant forcing neither tower
compiles at every length.

**F114-18. The affine tower's compile-side quantity is the leaf count times the node count, not the
leaf count.** For a fold of `L` leaves, `L(2L - 1)` associated-const cells against the interval tower's
`2(2L - 1)`, so 8128 against 254 at `L = 64`. This is arithmetic over the term with neither the width
nor the declaration in it, so `term shapes = a left-nested fold, fold length any, threads any, target
features any`. `p9_output.txt`. It corrects how `111` F111-18's "2 against 64" will be read: 64 is the
state at one node.

**Unpriced.** Every duration, for every arm, at every length. No bench ran, none could, and no claim in
this file depends on a magnitude.

---

## 10. What this file refutes, corrects or bounds, listed so the ratio is checkable

**Refuted.** `111` F111-15 at `overflow policy = wrap`, by 28 and 16 measured violations (F114-3). My
own prediction that the union variant in `p4` would be unsound, which is arithmetically impossible. My
own prediction in `p6` that the L1-norm formula would be conservative somewhere. My own prediction in
`p9` that static selection would compile where disjunction refuses.

**Corrected.** `112` section 9's sentence at `112:928`, which drops the predicate its own F112-21
carries (F114-6). `111` F111-18's state figure, which is per node rather than per term when read as a
compile-side cost (F114-18). `111` section 21's proposed composition, whose compile-time motivation
does not survive being compiled (F114-17), though its runtime shape survives intact.

**Bounded.** `112` F112-24's domination, to one-sided declarations (F114-10). `112` F112-21, confirmed
inside its own predicate and shown to invert outside it (F114-6). `112` F112-14, generalised from three
constructions to a formula and simultaneously bounded to bilinear ones (F114-12, F114-13).

**Kept, having gone looking for reasons to break it.** The corner rule's soundness, everywhere, on
three instruments. `111`'s conditions (a) and (b), which are the right two conditions and needed only
their overflow-policy predicate corrected. `112`'s one-sided form, which is the right first member of
any composition and is beaten only where its own basis assumption fails. `108:825`'s criterion, and the
verdict `111` and `112` reached with it. The `111` section 21 composition's runtime half: two arms
under one const predicate, which every sweep here supports.

**Not touched.** `109` section 8's chain result, now declined by four consecutive members. The
number-system question. `110`'s congruence result, which I read and did not test. Everything in `106`
and `108` other than the two clauses quoted.

---

## 11. Alternatives I considered and did not take

**A necessary and sufficient exactness predicate.** Abandoned early, for the reason in section 6.1: the
exact condition quantifies over the declaration box and `112` section 6 already shows the enumerating
oracle is not const-available at a real width. Worth recording as closed rather than untried.

**Widening condition (a) to handle correlated leaves whose over-approximation stays inside the
container**, which is the case `111` names as its over-conservatism. I built it and it is exactly the
disjunctive form in section 5.2, because "the corner interval never leaves the container" is just "the
corner rule licenses". So the widening is real and it is not a new condition.

**A third propagation rule based on tracking each leaf's sign separately**, which would fix the
two-endpoint loss at its root rather than by intersection. I did not build it. The intersection already
recovers every cell either rule reaches, so a third rule would have to beat the **oracle** on some cell,
which is impossible, or reach cells neither reaches, which for the annihilation residue means reasoning
about the whole term rather than any node. That is the direction I would send the next dispatch.

**Extending arm W0 to a non-ring signature by identifying which operations preserve the residue.**
Sketched and dropped: `min`, `max`, division and shifts all break it, and the interesting question is
not which break it but whether a **chain** can be cut at the non-ring operations and the ring segments
licensed separately. That is a real proposal and I did not build it. It would make arm W0 apply to a
clamping kernel's arithmetic between its clamps, which is exactly the shape `warm-clamp-shared` has.

**A compile-time mode for the bench harness.** The only thing that would actually price section 7's
duration. It is mockspace's tree, not arvo's, and I did not touch it.

**Measuring the `F > 0` propagation question.** Named as the largest gap in section 5.1 and not
attempted, because the fraction grid needs a second propagated quantity and building one properly is a
dispatch rather than a section.

---

## 12. Where the argument stands, for whoever writes the canon text

Three things are converged enough that I would carry them into a canon candidate, and I say which
rung each sits on. The rung definitions I am using are the ones `106` section 3 states at `106:280-282`,
which I read; I did not open `RULES.md` itself.

**Two experts, each having derived before reading the other.** Nothing in this file. Everything here
was derived after reading `109` through `112` and is a first instance or a reproduction, and I mark
which in section 2.

**Three or more independent instances.** The corner rule's soundness, now on `111`'s, `112`'s and my
implementations, three instruments and three authors. The refinement's placement outside both component
one and the primitive's coordinates, which `112` concluded, `111` confirmed by `108`'s stated
criterion, and which section 2 gives a third and independent reason for.

**One expert, and each is a queue entry rather than a doubt.** Every finding in section 9. The
homomorphism result in particular is one author on one instrument, and the thing I would most want a
second read on is not the measurement but the **claim that the overflow policy is the right axis to
carry it**, because if a design ends up with a policy that is neither a homomorphism nor a saturation,
the two arms do not partition anything.

**And one thing that is not a disagreement though it reads as one.** `112` section 9 says sound rules
disjoin and a design carries as many as it can afford; `111` section 21 says the choice is decidable
from the term so a design can carry one; this file says they intersect per node and that the affordance
question is decided at instantiation rather than at the verdict. Those three compose. The disjunction
is the right default, the intersection is a free improvement on it, and the selection is real but has
to happen a level earlier than either proposed.

---

## 13. Coverage, bounded rather than claimed

**Read in full.** `113`. `INTENTS.md`. `111` sections 17 through 26 and its findings list. `112`
sections 1, 2, 3, 5, 6, 6b and 9, and its findings list. `110`'s reply, R0 through R8. `106` section 1
and section 2. `108` section 7 and section 8.

**Read in part.** `109`'s headings and section 16.7's title only. `112_probes/p7` and `p9` at their
rule implementations, which I import rather than read end to end. `111_probes/r2` in full, because I
extend it. `112_probes/p8c` at its type-level construction. `mock/benches/variants/satfold-shared`
and `warm-clamp-shared` at the lines cited in section 0.2.

**Not read.** `109` in full, and everything before `106`. `110`'s phase one and phase two. `112`'s
sections 4, 7, 8, 11, 12 and 13. `111`'s sections 1 through 16. `OPTIONS.md`, `AGREEMENTS.md`,
`DROPLIST.md`, `RULES.md`, `PRIOR_CALLS.md` and the seed files. Where I cite any of those I am citing
another file's account of it and I say so at the point of use.

**Not re-run.** `112`'s thirteen-row domination table, its composite results, `110`'s congruence
result, `109`'s chain result, and every figure I quote from another file that I did not reproduce. `p3`
and `p4` reproduce `112`'s rules on `112`'s own code against new terms, which is not the same as
reproducing its numbers.

**Citations checked rather than trusted.** All 33 `file:line` references in this file were opened by
`p10`, each against the substring the claim depends on rather than against whether the reference
resolves. 33 of 33 pass, and two deliberately wrong entries were included as controls and both failed,
so the checker is not matching everything.

**Not established.** Anything at `F > 0`. Anything at a non-uniform value set. Anything at a real
width. Any duration. Whether a real consumer's terms are trees rather than DAGs, which section 5.4
names as the assumption I am least comfortable with.

**What I did not measure, having found where it would be measured.** Section 6.4 identifies the shipped
kernel a declared extent would improve and the harness already built to price the improvement. I did not
run it. That is the next dispatch's first hour and it is the only thing in this file with a consumer
attached.

---

## 14. Probe index

All committed, each with its output beside it, each carrying the case that must fail.

- `p1_the_structural_predicate_on_a_systematic_term_enumeration.py`, `p1_output.txt`. Every term at two
  and three leaf slots rather than twelve chosen rows; the predicate's soundness, its incompleteness,
  the wrap violations, and four controls.
- `p2_the_overflow_policy_decides_which_discharge_check_is_right.py`, `p2_output.txt`. The
  homomorphism, checked exhaustively with a saturating control; the root-decides-everything result; the
  witnesses; and the `min` control that breaks it.
- `p3_does_the_one_sided_form_dominate_off_the_thirteen_rows.py`, `p3_output.txt`. `112`'s own rules on
  a systematic enumeration, the two-endpoint counterexamples to F112-24, and a deliberately unsound
  rule as the soundness control.
- `p4_the_per_node_intersection_beats_both_rules.py`, `p4_output.txt`. The intersection, its
  domination, its small margin over disjoining verdicts, and the union control that could not fire.
- `p5_the_selector_the_wrap_arm_and_the_fold.py`, `p5_output.txt`. The wrap arm with the tightest
  interval, the three-step selector's counts, and the fold at every length with two controls.
- `p6_the_lifting_rule_is_a_formula_over_the_structure_constants.py`, `p6_output.txt`. 1024 bilinear
  constructions, the quaternions, two controls, and the refutation of my own conservatism prediction.
- `p7_the_structural_predicate_and_the_policy_arm_compile.rs`, `p7_output.txt`, `p7_asm.s`,
  `p7_asm_bodies.txt`. The predicate compiled with no feature gate, the policy-selected check, and the
  symbol aliases.
- `p8_dropping_intermediate_reductions_is_free_under_wrap.py`, `p8_output.txt`. Three arms rather than
  two, the unconditional win, and the mixed-carrier control that breaks it.
- `p9_pricing_the_two_spellings_without_a_clock.py`, `p9_output.txt`. The counts, the compile wall,
  eleven variants including four added after the first run, and the control that caught my generator
  defect.
- `p10_check_my_own_citations.py`, `p10_output.txt`. Every `file:line` in this file, opened, with the
  substring the claim depends on.
