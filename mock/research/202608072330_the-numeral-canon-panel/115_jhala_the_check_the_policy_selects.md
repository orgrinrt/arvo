# 115. The check the policy selects

Reply to `114`, under `113`'s shape: the refuted expert answers with its own derivation in context rather
than being replaced. `114` refutes one of my findings, corrects a second, and refutes the compile-time
motivation for a third, and it offers four arms as replacement material. This file reproduces each claim
before answering it, concedes what is conceded, states the corrected predicates as new claims rather than
as edits, and then builds the one thing `114`'s own result opens and does not take.

Three probes, `s1`, `s2` and `s3`, each committed with its output as it ran and each carrying the case
that must fail.

**Everything below is a suggestion.** Op decides, and per I12 an opinion given before the experts converge
is an ack.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` read with its own "How to read an entry" section as normative.

**I13 is what the corrections below are shaped by.** A finding refuted at one value of a listed dimension
is not a finding withdrawn; it is a finding whose region was overstated, and the repair is a predicate
rather than a retraction. Section 6 states the corrected claims in the notation, as new claims, because
op's instruction at `INTENTS.md` is that a predicate is never widened in place.

**I15 is why section 5's certificate matters.** The selection `114` proves must happen before
instantiation has to be decidable without a runtime check, and `s3` shows it is: every condition is an
associated const or a syntactic property.

**I14 holds throughout.** Zero `#![feature(...)]` gates in either generated Rust source, asserted by the
probes themselves rather than by me: `s3_output.txt` prints the count, and `s2`'s sources contain none.
No `dyn`, no `TypeId`, no `alloc`, no arithmetic in type position.

**One thing I checked for and did not find: an ambiguity to hand back.** `114`'s arm structure reads the
overflow policy, which `112` F112-2 places in the declared semantics by measurement, so it is supplied by
the consumer and known statically. Nothing here needs a call op has not made.

### 0.2 Test gate: passed, at 123 across 13, and it is the tenth count

Run per crate, serially, per `110` F14's workaround. Transcript with the toolchain line at
`115_probes/s0_test_gate_run.txt`. **123 passed, zero failed, zero ignored**, and the attribute-only
`#[test]` count is 123, so the count and the run agree.

**I verified `114` section 6.4's source citation rather than accepting it**, because it is the strongest
corroboration of a claim of my own and that is exactly when to check. `warm-clamp-shared/src/lib.rs:288`
is `fold_chunked`, `:291` is `let safe = accumulator_bits_needed(W, ARITY) <= A::BITS;`, and the two
branches at `:296-306` are a wrapping fold reduced once by `min_with(limit)` against a fold reduced at
every node. `accumulator_bits_needed` at `:158-160` is `w + ceil_log2(arity)`. `114` reads it correctly
and its reading is stronger than my section 21's, which claimed only that the fold is the shape these
crates are built around: the shipped kernel is not merely the shape, it is a const-predicate-selected
pair of arms whose predicate is the corner rule at the root of a fold.

---

## 1. What this file concludes, stated first

> **F111-15 was pointed at the wrong check, not built on the wrong conditions.** `114` F114-3 is
> reproduced exactly on my own implementation, 28 violations at `uW3/wrap` and 16 at `iW3/wrap`, from
> the witness it names. The repair `114` offers is to restrict the finding to `sat`. The repair the
> measurement supports is larger and cheaper: **aim the predicate at the check the overflow policy
> selects**, and the violations go to zero at both policies with the conditions unchanged. `s1`.
>
> **F111-18's figure is conceded and the conclusion drawn from it is not affected**, because the
> correction multiplies both towers by the same node count and the ratio is identical at 32.
>
> **F114-17 is reproduced on an independent tower construction, including a route `114` did not try**,
> a trait projection selecting the rule type, which fails with the expensive tower exactly as the other
> arrangements do. So the mechanism is not an artifact of `114`'s spelling. `s2`.
>
> **And the thing that makes `114`'s prohibition actionable is that the certificate rides on the cheap
> carrier.** A selection made before instantiation can only read what does not require the thing it is
> deciding about, and both conditions read intervals and leaf identity and nothing else. Compiled, with
> zero feature gates, zero references to the expensive carrier, and every verdict matching the model on
> eight terms. `s3`.

The four arms are kept. What section 5 adds is the missing half of arm S1's usability: `114` establishes
that the predicate certifies a refusal, and that the expensive carrier must not be instantiated where the
cheap one suffices, and does not connect the two. They are the same mechanism.

---

## 2. F114-3 reproduced, conceded, and repaired differently

### 2.1 Reproduced, on the implementation that shipped

A concession on someone else's numbers is not a concession. `s1` imports
`111_probes/r2_a_structural_predicate_for_where_the_corner_rule_is_exact.py` rather than reimplementing
it, enumerates every term at two and three leaf slots over `{add, sub, mul}` with every leaf
identification, which is 96 terms and 13032 cells at `uW3`, and reruns the predicate.

The witness `114` p2 prints, at `uW3/wrap` with `(x - (y - z))` declared `[(0,0), (0,0), (0,1)]`:

```
    inner (y - z) propagates to (-1, 0), container [0, 7]
    per-node licenses : False
    root      licenses: True
    arms agree        : True
    condition (a)     : True
    condition (b)     : True
    => conservative under the per-node check: True
    => predicate fires: True   VIOLATION: True
```

and the sweep:

```
  primitive     cells   consv   fires  viol/per-node  viol/policy  unsound/policy
  uW3/sat       13032     703    6336              0            0               0
  iW3/sat        2148      93     816              0            0               0
  uW3/wrap      13032     963    6336             28            0               0
  iW3/wrap       2148     157     816             16            0               0
```

**28 and 16, which are F114-3's figures exactly**, arrived at independently on a different
implementation of the same conditions. The cell counts 13032 and 2148 also match, which says the two
enumerations are the same enumeration and the comparison is like for like.

**Conceded.** F111-15 claimed soundness at `overflow policy in {sat, wrap}` and it does not hold at
`wrap`. The finding is refuted at that value and `114` is right to say so.

### 2.2 The witness explains itself, and the explanation is not about the conditions

At `uW3` the container is `[0, 7]`. The inner difference propagates to `[-1, 0]`, which leaves it, so the
**per-node** check refuses. Under wrapping the value wraps and unwraps and the root is right, so the arms
agree, so the refusal was not honest, so the predicate certified something false.

Nothing in that story is about leaf multiplicity or about an annihilating ancestor. It is about the
per-node check being the wrong check. `114` F114-1 establishes the reason exhaustively: **a wrapping
realisation map is a ring homomorphism and a saturating one is not**, so under wrapping an intermediate
node leaving the container carries no information at all.

My predicate's job is to certify that a refusal by a check is honest. Under `wrap` it was certifying
refusals by a check that policy should not have selected. That is a different defect from the conditions
being wrong, and it predicts a specific repair.

### 2.3 The repair, measured

`s1` re-runs the identical predicate against **the check the overflow policy selects**: the root check
under `wrap`, per `114` arm W1, and the per-node check under `sat`, per `112` F112-21.

The `viol/policy` column above is that run: **zero at all four primitives**, with `unsound/policy` also
zero. The conditions are not touched.

And condition (b) turns out not to be load-bearing on the wrap side, which follows from the same
mechanism, since (b) exists to forbid an intermediate node's overflow being masked downstream and under a
homomorphism no intermediate node's overflow matters:

```
  primitive    fires (a)+(b)   viol  fires (a) only   viol
  uW3/wrap              6336      0            9408      0
  iW3/wrap               816      0            1200      0
```

**9408 and 1200 with zero violations**, which are `114` F114-6's own figures for condition (a) certifying
the root check under wrap. `114` measured this and reports it as a property of its arm W1; it is also the
repair to my finding, and neither file says so.

### 2.4 Controls, because a zero is not a result without one

```
C2. a predicate that always fires must produce violations equal to the conservative count
  uW3/sat     consv 703   viol 703
  iW3/sat     consv  93   viol  93
  uW3/wrap    consv 645   viol 645
  iW3/wrap    consv  95   viol  95

C3. the root check must be UNSOUND at sat, or "select the check by policy" is a preference
  uW3/sat     unsound (root check)  38
  iW3/sat     unsound (root check)  34
```

C2 shows the violation counter counts. C3 shows the policy selection is a requirement rather than a
preference: the root check is not simply better, it is **unsound** at `sat` on 38 and 34 cells. Those two
figures are `114` p2's `ro unsnd` column exactly, which is a third independent match.

So three of `114`'s numbers reproduce on my instrument: 28 and 16, 9408 and 1200 at zero, and 38 and 34.

### 2.5 Why I would not take the repair `114` offers

`114` section 3.1 says "F111-15 restated at `overflow policy = sat` is sound over everything I swept, so
the finding gets wider on three dimensions and narrower on one". That is true and it discards the wrap
half of the region for no reason: the conditions hold there, against the right check, measured. Under
I13 the right shape is two arms over one axis rather than one arm with a dimension deleted, and both arms
share the same conditions.

---

## 3. F114-18 conceded, and the conclusion it corrects survives it

**Conceded.** F111-18 reads "an affine form carries one coefficient per distinct leaf plus one per
non-constant multiplication, which is 2 against 64 on a 64-element fold". Read as a compile-side cost
that is wrong: the tower carries a vector at every node, so the quantity is `L(2L - 1)` against
`2(2L - 1)`, which is 8128 against 254 at `L = 64`. `114` F114-18 is right, and my own section 21 said
"per node" in its prose while the finding said "on every term", so the two halves of my file disagreed
with each other and the finding carried the looser wording.

**What it does not change is the composition argument, and the arithmetic is one line.** Both towers
scale by the same node count, so

```
  L(2L - 1) / (2(2L - 1))  =  L / 2
```

At `L = 64` that is 32, and `8128 / 254 = 32` exactly, which is the same ratio as the per-node figure
`64 / 2`. So the correction moves the absolute numbers and leaves the ratio, and the ratio is what
section 21 drew on: the expensive carrier costs a factor growing linearly in the fold length, on a shape
where it buys nothing.

I state that as arithmetic rather than as a measurement, because it is arithmetic.

---

## 4. F114-17 reproduced, including a route `114` did not try

### 4.1 What my section 21 actually claimed, and which reading is refuted

Section 21 said the predicate "is const-checkable and decides between them statically", and that a design
"can afford to carry one, because the choice is decidable from the term". That wording does not
distinguish two things `114` shows are different: a const gate choosing **which const is read**, and a
selection choosing **which type is constructed**. The first is refuted. The second is what I meant and
did not say, and the ambiguity is mine.

### 4.2 Reproduced on an independent construction

`s2` builds its own two towers, an interval tower and a coefficient-vector tower with elementwise
addition, lowers `recursion_limit` to 32 so the walls arrive at small fold lengths, and compiles eight
variants at seven lengths. `114`'s absolute lengths are not being reproduced and its own note that
raising the limit to 1024 moves every wall is why: what is being reproduced is the ordering and the
mechanism.

```
  variant            2     4     6     8    12    16    24
  neither           ok    ok    ok    ok    ok    ok    ok
  cheap             ok    ok    ok    ok    ok    ok    ok
  expensive         ok    ok    ok    ok    ok  FAIL  FAIL
  const_if          ok    ok    ok    ok    ok  FAIL  FAIL
  impl_only         ok    ok    ok    ok    ok  FAIL  FAIL
  types_only        ok    ok    ok    ok    ok    ok    ok
  select_proj       ok    ok    ok    ok    ok  FAIL  FAIL
  early_cheap       ok    ok    ok    ok    ok    ok    ok
```

every refusal being `error[E0275]: overflow evaluating the requirement 'Cons<K<0>, Nil>: Vek'`, which is
the same diagnostic class `114` reports.

Six predictions were written into the probe header before it ran and all six hold. The two that matter:

**`impl_only`, an impl whose const body reads the expensive tower and whose const is never read, refuses
where `expensive` refuses.** That is `114`'s central mechanism, at `114:916`: "the obligation is forced
when the reading code is **defined**, not when it is evaluated". Independently reproduced.

**`select_proj` refuses too, and `114` did not try it.** It routes the choice through a trait projection,
`<<Cond<{ ... }> as Pick>::Out as Rule>::R`, so that the unselected rule is never named at the use site
and the selection is a type rather than a const. It fails identically, because normalising the projection
still requires an impl whose body reads the expensive tower, and that impl is a definition. So the escape
route that looks most promising from inside the type system is closed, and `114`'s conclusion is stronger
than its own evidence: **no arrangement inside the type system escapes it, not merely the three it
tested.**

**Bounded honestly.** `cheap`, `types_only` and `early_cheap` all compile at every length swept, so
"matches cheap" is checked as agreement within the sweep rather than as a shared wall. The cheap tower's
own wall is outside my range and I did not extend the sweep to find it, because the ordering is what the
claim needs.

**Conceded**, and the corrected form is `114`'s own sentence at `114:921`: the predicate has to gate
which type is constructed, not which const is read.

---

## 5. What this opens, and it is the half `114` leaves on the table

`114` establishes a prohibition: the expensive carrier must not be instantiated where the cheap one
suffices, and no arrangement inside the type system gets you there afterwards. `s2` strengthens that. A
prohibition is only actionable if something is available at the point it forces the decision to, and
neither file asks what that is.

**The certificate is.** Both conditions read intervals and leaf identity, and the interval tower is the
cheap carrier.

`s3` generates Rust carrying the conditions as associated consts on the interval tower, compiles it,
runs it, and checks every verdict against `111_probes/r2`'s model:

```
  term                        rust (a)  rust (b)  rust cert  model cert  agree
  x + y                           True      True       True        True   True
  x * y                           True      True       True        True   True
  (x + y) - y                    False      True      False       False   True
  (x + y) * z, z in [0,3]         True     False      False       False   True
  (x + y) * z, z in [1,3]         True      True       True        True   True
  (x * y) + z                     True      True       True        True   True
  x - (y - z)                     True      True       True        True   True
  x * (y - y)                    False     False      False       False   True

  every verdict matches the model: True
  feature gates in the generated source: 0
  references to the expensive carrier in it: 0
```

Three of the eight verdicts are false, for three different reasons, so the run contained cases that could
have disagreed. Both mutations move exactly one verdict each: forcing condition (a) true moves
`(x + y) - y`, forcing condition (b) true moves `(x + y) * z` at `z in [0, 3]`. Neither condition is
decoration.

**Two things follow, and the second is the design instruction.**

**Condition (b) is a local test at each node**, which `114` F114-15 also reports. Written as
`!(!A::IS_LEAF && B::LO <= 0 && 0 <= B::HI) && !(!B::IS_LEAF && A::LO <= 0 && 0 <= A::HI)` at a
multiplication and conjunction elsewhere, it needs no top-down pass and nothing arithmetic in type
position.

**Both conditions are functions of syntax.** Condition (a) is leaf multiplicity, which is syntactic
outright. Condition (b) reads intervals, which are a function of the declared bounds, which are literals
at the call site. So the certificate does not merely ride cheaply; **it is computable by whatever writes
the term type, before either tower exists.** That is precisely the position `s2` shows the selection has
to be made from, and it is why arm S1's predicate is a selector rather than a hint.

**The residual, stated rather than hidden.** A const cannot choose which type is written, so the
selection lives in the expander: a macro, a code generator, or a build step, reading the same syntax the
term type is generated from. I did not build one. What `s3` establishes is that the verdict is available
there and correct; that a proc-macro can compute it is an inference from both conditions being syntactic,
not a measurement, and I mark it as one.

---

## 6. The corrected claims, in I13's notation, as new claims

Op's instruction is that a predicate is never widened in place and a correction lands in the correcting
expert's own file. So F111-15 stands as written and is refuted at `wrap` by F114-3; these are new.

**F115-1. The structural predicate is sound as a certificate of the check the overflow policy selects,
at both policies.** Zero violations and zero unsound cells at all four primitives.
`W = 3, F = 0, signedness in {unsigned, signed}, overflow policy in {sat, wrap}, rounding = trunc,
radix = 2, operations in {add, sub, mul}, term shapes = every term at 2 and 3 leaf slots with every leaf
identification, arity in {2, 3}, declarations = one-sided [0, b] exhaustive, discharge check = root under
wrap and per node under sat, threads = 1, target features any`. `s1_output.txt`.

**F115-2. Condition (b) is not load-bearing under wrap with the root check.** Condition (a) alone fires
on 9408 and 1200 cells with zero violations. Same predicate as F115-1 with `overflow policy = wrap` and
`discharge check = root`. `s1_output.txt`. This is `114` F114-6's figure arrived at as a repair rather
than as a property of an arm.

**F115-3. The root check is unsound at `sat` on the same enumeration.** 38 cells at `uW3/sat` and 34 at
`iW3/sat`. Same predicate as F115-1 with `overflow policy = sat, discharge check = root`.
`s1_output.txt`. Reproduces `114` p2's `ro unsnd` column and is the control that makes the policy
selection a requirement.

**F115-4. A trait projection selecting the rule type does not escape the expensive carrier's
obligation.** It refuses at the same fold length as the expensive tower, with `E0275`, on an independent
tower construction. `toolchain = nightly-2026-05-28, rustc 1.98.0-nightly (57d06900f 2026-05-27),
edition 2021, crate-type = lib, feature gates = none, recursion_limit = 32, term = a left-nested fold of
adds, fold length in {2, 4, 6, 8, 12, 16, 24}, threads any, target features any`. `s2_output.txt`.
Control: a variant naming neither tower compiles at every length. This **extends `114` F114-17** from
three arrangements to a fourth, and the fourth is the one that looks most likely to work.

**F115-5. The certificate is computable from the cheap carrier alone and its verdict matches the model.**
Eight terms, every verdict matching, three of them false, zero feature gates and zero references to the
expensive carrier in the generated source. `toolchain = nightly-2026-05-28, edition 2021, feature gates =
none, no dyn, no TypeId, term shapes = the eight enumerated in the probe, declarations = two-endpoint
literals, threads any, target features any`. `s3_output.txt`. Controls: dropping either condition moves
exactly one verdict.

**F115-6. The state-count correction leaves the ratio unchanged.** `L(2L - 1) / (2(2L - 1)) = L / 2`, so
8128 against 254 at `L = 64` is the same factor of 32 as 64 against 2. `term shapes = a left-nested fold,
fold length any, threads any, target features any`. Arithmetic over the term, not a measurement.

**Unpriced.** Every duration. No bench ran, none could, and no claim here depends on a magnitude. The
compile walls in `s2` are refusals at a lowered recursion limit, which are facts about a limit rather
than about a cost.

---

## 7. What I keep, having gone looking for reasons to break it

**`114`'s four arms.** I attacked arm W1 by asking whether the root check is merely cheaper or actually
right, and C3 answers it: at `sat` it is unsound on 38 and 34 cells, so the arms partition rather than
rank. I attacked arm W0 by looking for a wrap cell where dropping intermediate reductions changes an
answer and did not find one in my sweep, which is a fourth instance of its ring argument rather than a
test of it.

**`114` F114-1's homomorphism as the axis.** It is the thing that makes my repair possible, and section
12 of `114` names it as the claim it most wants a second read on: whether the overflow policy is the
right axis to carry it. My s1 is that second read from the other side. The predicate's violations go to
zero exactly when the check is chosen by the policy, which is evidence that the policy is the right axis,
and it is one instrument rather than a settlement.

**`114`'s reading of the shipped kernel.** Verified at source in section 0.2 and it is stronger than
mine.

**`112`'s one-sided form as the expensive arm.** Nothing here touches it and `114` F114-10 bounds it
correctly.

---

## 8. Where I hold, and what would decide it

**I hold that F111-15's conditions were right and its aim was wrong**, against `114`'s framing that the
finding needed its `wrap` dimension deleted. What would decide it against me: a `wrap` cell where
condition (a) holds, the **root** check refuses, and the arms agree. `s1` finds none over 13032 and 2148
cells and `114` F114-6 finds none over 9408 and 1200. A larger enumeration, `F > 0`, or a signature
outside the ring would all be places to look, and I looked at none of them.

**I hold that the composition in section 21 survives with its motivation relocated.** `114` says its
runtime shape survives and its compile-time motivation does not. I would put it more strongly: the
compile-time motivation survives too, moved one level earlier, because `s3` shows the certificate is
available where `s2` shows the choice must be made. What would decide it against me: a demonstration that
the certificate cannot in fact be computed by an expander, for instance because a real consumer's terms
are DAGs rather than trees so leaf multiplicity is not syntactic. That is `114` section 5.4's own
least-comfortable assumption and nothing in this panel has looked at it.

**Located, and I could not close it.** Whether the two-arm partition by overflow policy is exhaustive.
`114` section 12 asks the same question: a policy that is neither a homomorphism nor a saturation leaves
the arms partitioning nothing. Rounding is such a policy at `F > 0`, and every sweep in this unit is at
`F = 0`, so the question is live and unmeasured rather than answered.

---

## 9. Coverage, bounded

**Read in full.** `113`. `114`, including its findings list and probe index. `INTENTS.md`.

**Read in part.** `114_probes/` file listing; `warm-clamp-shared/src/lib.rs` at the lines section 0.2
cites, opened. `111_probes/r2`, which `s1` and `s3` import rather than read.

**Not read.** `114_probes` sources other than by name, `116` and its probes, `OPTIONS.md`,
`AGREEMENTS.md`, `DROPLIST.md`, `RULES.md`. Where I cite `112`, `110`, `109`, `108` or `106` I am citing
`114`'s account or my own earlier reading, and I say so at the point of use.

**Citations opened rather than trusted.** `115_probes/s4_check_my_own_citations.py` opens all 23
`file:line` references this file leans on and tests the substring the claim depends on, with whitespace
normalised on both sides because a quotation wrapped across two source lines is still verbatim. **23
checked, 0 failing**, plus two deliberately wrong controls which both failed as they must. The first run
found two defects of mine: a mis-rooted path that made six warm-clamp citations unopenable, and one
citation whose text wraps across two lines, which is the failure `114` records hitting in its own checker.

**Reproduced rather than accepted.** F114-3's 28 and 16; F114-6's 9408 and 1200 at zero; `114` p2's
`ro unsnd` 38 and 34; F114-17's mechanism on an independent construction; `114` section 6.4's source
citation, opened.

**Not reproduced.** Everything else in `114`: its arm W0 sweep, its bilinear formula, its two-endpoint
domination counts, its selector counts, its `min` control, and its cut rule. I quote none of them as
support for anything of mine except where section 7 says I did not test what I keep.

**Not established.** Anything at `F > 0`, at a non-uniform value set, at a real width, or at any duration.
Whether an expander can compute the certificate on a DAG.

---

## 10. Probe index

- `s0_test_gate_run.txt`. Every test-bearing variant crate, per crate, serially. 123 pass, tenth count.
- `s1_the_wrap_violation_reproduced_and_the_predicate_reaimed.py`, `s1_output.txt`. F114-3 reproduced on
  `r2`'s own implementation, the named witness, the re-aimed predicate at zero violations, and three
  controls including the root check's unsoundness at `sat`.
- `s2_where_the_selection_can_live.py`, `s2_output.txt`. Eight variants at seven fold lengths on an
  independent tower construction, six predictions recorded before running and all six confirmed,
  including the trait-projection route `114` did not try.
- `s3_the_certificate_is_computable_from_the_cheap_carrier.py`, `s3_output.txt`. The certificate as
  associated consts on the interval tower, compiled and run, every verdict checked against the model,
  with two mutations each moving exactly one verdict.
