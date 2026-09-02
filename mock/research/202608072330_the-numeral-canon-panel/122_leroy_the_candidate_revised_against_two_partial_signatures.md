# 122. The candidate revised: the domain is a dimension, and saturation has a deferral arm

`119`'s author, revising it against two partial signatures. `120` cosigns 4.7 entire and dissents on
three points, all in 4.10. `121` signs eight clauses and dissents on 4.2 and 4.4, and both of its dissents
are one defect: a dimension load-bearing everywhere in this topic appears in no predicate in the sitting,
including in its own.

**`119` stays as landed and is not edited.** Both signatures cite it by line and the audit trail keeps
what was written before it was corrected.

## What this supersedes and what stands

**Superseded, and the superseding text is in section 4 below.** `119` 4.2's predicate, which admits a
counterexample and is replaced rather than widened. `119` 4.4's saturating clause, which is false and is
contradicted by a green test in this repository. `119` 4.3, 4.5, 4.6 and 4.7, each amended for the same
missing dimension. `119` 4.10's mechanism sentence and predicate. `119` section 3's doability list, where
one entry restores a hedge its author marked and I dropped.

**Stands, unchanged and not restated except by reference.** `119` 4.1, 4.8, 4.9. `119` section 5 entire,
which `121` signs and says it would not change a word of. `119` section 2, section 6's method, and every
ledger entry not named in section 2 below.

**And one thing neither signatory dissented on, which both pointed at and which u4 measures.** `119` 4.5
says a declared restriction is the **only** mechanism that makes both licence families available at once.
It is not. A one-signed domain is a second one, and it needs no declaration.

Everything below is a suggestion. Op ratifies, and per I12 an opinion given before the experts converge is
an ack.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` with its "How to read an entry" section as normative.

**I13 (`INTENTS.md:214`) is the standard both dissents are made under and the standard this revision is
written to.** A predicate lists only what holds; a dimension absent claims nothing anywhere it is
present. `121`'s dissent is that a predicate of mine admits a counterexample, which under I13 makes it
**wrong** rather than narrow, and the replacement is a new claim rather than an edit. `120`'s second
dissent is a dropped dimension, which under the same rule widened a claim rather than narrowing it.

**I15 bears on `120`'s third dissent.** A clause asserting a certificate is computable from a
derivation's syntax is a claim about what is available before any value exists, which is exactly what I15
makes load-bearing, and `115` marked it as an inference while `119` stated it flat.

**I14 holds.** The one Rust compilation here is `114_probes/p9`'s own generator with the recursion-limit
attribute varied. Zero feature gates, no `dyn`, no `TypeId`.

**No ambiguity to hand back.** Every judgement below is a measurement or a citation.

### 0.2 Test gate: passed, at 123 across 13, and it is the fifteenth count

`u0_test_gate_run.txt`, per crate by `--manifest-path`, attribute count 123 agreeing with the run.

**One test in that suite is evidence in this file rather than only a gate result**, which `121` section
3.4 is the first to notice. `warm-clamp-shared/src/lib.rs:1105` is
`clamping_is_a_retraction_on_non_negative_addition_at_every_swept_width`, and `u0` runs it on its own and
records it passing. I opened it at source before building on it: it folds non-negative terms, clamping at
every step, against the exact sum clamped once, and asserts they are equal over the whole swept matrix.
That is the homomorphism identity for addition on a non-negative domain, asserted in shipped code.

`119` 4.4 says a saturating map is a homomorphism for no operation. **The clause and the test cannot both
be right, and the test is right.**

---

## 1. Each dissent, reproduced before it is accepted

Both signatories reproduced my claims before answering them. The same is owed back, and in two cases the
reproduction found more than the dissent claimed.

### 1.1 `120` on 4.10's mechanism sentence: accepted, reproduced on my own generator

`119:566` attributes the compile-side cost to "a recursion depth in the trait solver, proportional to the
derivation's size **times** the carrier's per-node state". `120` says that gives a depth the cell count's
law, and that `114`'s own explanation of the wall is spine depth **plus** vector length.

Reproducing on `115`'s towers would test my reading of `115`. `u1` runs the question on
`114_probes/p9`'s own generator, mine, with only the recursion-limit attribute swept:

```
    limit  cheap wall  expensive wall  cheap/limit  expensive/limit
       16          15               7        0.938            0.438
       24          23              11        0.958            0.458
       32          31              15        0.969            0.469
       48          47              23        0.979            0.479
       64          63              31        0.984            0.484
       96          95              47        0.990            0.490
```

**The cheap wall is exactly `limit - 1` at all six limits and the expensive wall exactly `limit / 2 - 1`.**
Both relations are exact, which is additive with zero residual once the offset is taken; the mean absolute
error of 1.00 the probe reports against a bare `limit / 2` is that offset rather than model error. The
multiplicative model scores 5.71. `120`'s figures are `limit` and `limit / 2` on its own towers, which
differ from mine by one node, so the two files agree on the law and differ on a constant.

**Accepted.** The correction is `120`'s wording at its section 5.1 and it is in 4.10 below.

Controls fired: a variant naming neither tower compiles at every length at every limit, and the cheap
wall moves across six distinct values, so the instrument reads the obligation chain rather than the file.

### 1.2 `120` on 4.10's dropped dimension: accepted, and it is one instance of a class

`recursion limit` is in `115` F115-4's predicate and absent from `119` 4.10's. `120` is right that under
I13 a dimension not listed claims nothing anywhere it is present, and a recursion limit is always
present, so 4.10's predicate as written claims nothing about any compile. `120` is also right that `114`
F114-17 has the same omission, so this is a compression carrying a defect forward.

`u1` makes the repair possible rather than only necessary: with the wall now measured as a function of
the limit, 4.10 can state the law and list the dimension rather than list a set of lengths.

### 1.3 `120` on the dropped hedge: accepted, and it is the dissent I am most glad to have received

`115:322` marks the syntax half of F115-5 as "an inference from both conditions being syntactic, not a
measurement, and I mark it as one". `119:559` states both halves flat. `119` section 3's doability list
carries only the measured half, so two parts of the candidate disagreed about what was established.

No measurement is needed and none is offered: this is a fact about two documents and both are open above.
**Accepted**, and section 4.10 and section 3 below carry the mark.

### 1.4 `120`'s missing ledger entry: accepted

Nothing in `119`'s ledger covers the re-aiming itself. A2 gives which check, A9 gives which conditions,
and neither gives that the certificate aimed at the check the character selects is **sound**, which is
what makes 4.7's closing sentence a claim. `119` files it under framings retired at `119:308`, which is
where a reader is told what to stop citing.

`120`'s proposed entry is right and is section 2's A18.

### 1.5 `121` on 4.2's predicate: accepted, and the counterexample reproduces exactly

`u2` recomputes it. `R(v) = clamp(v, 0, 15)` onto sixteen values on the domain `0..47`:

```
    homomorphism for add: YES  (0 failures of 2304)
    homomorphism for mul: YES  (0 failures of 2304)
    homomorphism for sub: no   (976 failures of 2304)
    order-preserving : True
    non-constant     : True

    `119` 4.2's condition 1, a complete residue system  : True
    `119` 4.2's condition 2, the interval 0..16 present : True
    closed under negation                               : False
```

**It satisfies every condition `119` 4.2 states and falsifies the theorem under that predicate.** The
control is the same map on a domain straddling zero, where the homomorphism fails on 1062 pairs with the
witness `a = -24, b = 1`.

**This is a wrong predicate and not a narrow one**, and the two acts are different. A widening adds a
region to a claim that was true where it was measured. A predicate admitting a counterexample was never
true as stated, so it is **replaced**. 4.2 below is written that way and says so.

### 1.6 `121` on my own ablation: accepted, and my instrument was confounded

`121` says my F118-5's witness does not isolate the operation it names. Reading `118_probes/q2` at its
`main` confirms the accusation before any measurement: its `{+, *}` and `{+}`-alone rows use windows
straddling zero, and the `{*}`-alone rows that produced witnesses use **non-negative** windows. The two
conclusions came from cells differing in two dimensions, and the cell that separates them was never run.

`u2` runs it, with the window sizes **matched** so the only difference between the rows is the sign:

```
   |V| window                     ops            maps    hom   mono   both  NON-CONSTANT BOTH
     2 straddling zero            {add, mul}       32      3     10      2                  0
     2 straddling zero            {add}            32      4     10      2                  0
     2 straddling zero            {mul}            32      4     10      2                  0
     2 non-negative, same size    {add, mul}       32      4     10      3                  1
     2 non-negative, same size    {add}            32      5     10      3                  1
     2 non-negative, same size    {mul}            32      6     10      3                  1
     3 straddling zero            {add, mul}     2187      8    129      3                  0
     3 straddling zero            {add}          2187     10    129      3                  0
     3 straddling zero            {mul}          2187     36    129      3                  0
     3 non-negative, same size    {add, mul}     2187     10    129      6                  3
     3 non-negative, same size    {add}          2187     12    129      6                  3
     3 non-negative, same size    {mul}          2187     55    129      6                  3
```

**Every straddling row is empty and every non-negative row is not, at every operation set**, and the
witness printed on every non-negative row is the same map. So the operation set does not separate the
rows and the domain does.

Two controls. C1: each half of the conjunction is non-empty on its own on every row, 3 to 55 homomorphic
and 10 to 129 monotone, so a zero in the "both" column is a measurement. C2: `119` 4.2's two conditions
are **True on both rows**, so neither of them distinguishes the case where the theorem holds from the case
where it fails, while closure under negation is the only column that does.

And P4: closing a non-negative window under negation empties it again, at every operation set and both
sizes.

**So my F118-5 is refuted as attributed and I withdraw it.** F118-4's number survives, because dropping
multiplication on a straddling window does still give zero, but its framing does not: what it measured was
the domain. F118-6 is the same fact at a third size, since a window narrower than the value set is also
not closed under negation.

**One precision I would keep against `121`'s wider phrasing.** `121` F121-1 states the operation set is
not load-bearing at all. That is right as a measurement and it is not a consequence of the proof, which
runs through the additive group and covers the additive case only. The `{mul}`-alone rows are measured
empty at `|V| in {2, 3}` on windows of 5 and 7 points, and the theorem does not cover them. 4.2 below
keeps the two apart.

### 1.7 `121` on 4.4: accepted, and the clause is mine and was read off my own control column

`u3` reproduces the reconciliation on my own instrument:

```
  primitive    ambient domain                   add           sub           mul
  uW4/sat      non-negative 0..45         0/1089      391/1089        0/1089
  uW4/sat      straddling -48..48       720/2401      751/2401      256/2401
  uW4/wrap     non-negative 0..45         0/1089        0/1089        0/1089
  uW4/wrap     straddling -48..48         0/2401        0/2401        0/2401
```

**The saturating rows move with the domain and the wrapping rows do not**, which is `121` section 3.3
exactly, and my own 720 reappears on the straddling row. The cause is in my own probe:
`118_probes/q3`'s ambient range starts at `klo - span`, which is negative even for an unsigned primitive,
so my saturating column measured a straddling domain and I wrote its verdict down as though it were about
saturation.

`121` is right, its own `116_probes/p4` was right, and neither predicate named the span.

---

## 2. What `121`'s dissent opens, which no file in the sitting has taken

This is the part worth the revision. If saturation is a homomorphism for addition and multiplication on a
one-signed domain, then the **deferral licence** `119` 4.3 attributes to the homomorphism is available
under saturation too, on the region where a derivation's exact intermediates cannot leave the domain's
sign. `121` names the structure as a semiring homomorphism at its section 3.6 and stops. Nothing measures
whether arm W0 or the discharge check actually hold there.

`u3` measures it.

```
  setting                                       cells  exact out  W0 differs root unsound  per-node uns
  uW3/sat, one-sided, NO subtraction             5840       4002           0            0             0
  uW3/sat, one-sided, with subtraction           7192       5820        2161           38             0
  uW4/sat, one-sided, NO subtraction            39584      32104           0            0             0
  uW4/sat, one-sided, with subtraction          49072      44247       17840          142             0
  uW3/wrap, one-sided, NO subtraction [C1]       5840       4002           0            0             0
  uW3/wrap, one-sided, with subtraction [C1]     7192       5820           0            0             0
```

**Under saturation with one-signed declarations and no subtraction, arm W0 holds on every cell and the
root-only discharge check is sound on every cell**, over 5840 and 39584 cells with 4002 and 32104 of them
reaching an exact result outside the container, so the comparison is live. With subtraction present both
fail. The wrapping rows are the control and are unaffected by subtraction.

And the confirmation from the other side: every one of the 38 and 34 cells where `118` q1 found the
root-only check unsound at saturation is a term containing a subtraction, and none is subtraction-free.

**But "no subtraction" is not the condition, and the control that says so is the one I built to break it.**
On a signed container with declarations reaching below zero and no subtraction anywhere, arm W0 fails on
656 of 968 and 4416 of 5840 cells. So the condition is that **the derivation's exact intermediates keep
the sign of the domain**, and subtraction-freeness over one-signed declarations is a sufficient syntactic
condition for that rather than the condition itself.

**And this is what the shipped fold uses.** `u3` re-derives the identity
`warm-clamp-shared/src/lib.rs:1105` asserts, at 0 disagreements over all six settings swept. `119` section
8.1 read that kernel as an instance of a discharged declaration restoring the identity. That reading is
right and incomplete: the kernel is **also** inside this arm, and would be correct on a one-signed
subtraction-free fold even without its guard, which is a second and independent reason it works.

### 2.1 So `119` 4.5's "only mechanism" is false, and both signatories pointed at it without pressing

`120` section 8 flagged the clause and left it, saying it belonged to `116`'s author. `121` did not
dissent on 4.5 and supplied the fact that decides it. `u4` measures both characters at once:

```
  policy   domain                       ops               order-preserving   deferral holds
  sat      one-signed 0..7              {add, mul}                    True             True
  sat      one-signed 0..7              {add, sub, mul}               True            False
  sat      closed under negation -7..7  {add, mul}                    True            False
  wrap     one-signed 0..7              {add, mul}                   False             True
  wrap     closed under negation -7..7  {add, mul}                   False             True
```

**A saturating map on a one-signed domain has both characters at once, with no declaration.** The
controls fire in both directions: closing the domain under negation costs the deferral column, and every
wrapping row loses the order column.

So there are **two** escapes from the trade rather than one, and 4.5 below says so.

**One defect of my own in that probe, fixed in view.** Its first version read order-preservation on the
operand domain alone, and where that domain fits inside the container the map is the identity there and
every row came back monotone, including the wrapping ones. That is the map not being exercised, and it is
the same class of defect as `121` found in my q3. Fixed to read the range of exact results, and the
wrapping rows then read False as they should.

---

## 3. Ledger deltas

Every entry in `119` section 1.1 not named here stands. `121` checked A1, A2, A3, A5, A6, A7, A8, A14 and
A15 against what it did and reports them accurate, and confirms A2's credit of the saturating half to
`112` F112-21 independently and first; `120` cosigns A2, A9, A11, A16 and A17 as they concern
it.

**A13, corrected against its own beneficiary.** The entry is about arm S2, which is `114` F114-11,
reached after `112` F112-24's domination was bounded by `114` F114-10 and widened by `116` F116-3. `119` credits `116` with an independent arrival at arm
S2. `121` will not sign it: `116:247` is a claim about its own reasoning process rather than about
ordering, and its coverage section records reading `114` sections 1 to 6.3 before writing, which includes
the section arm S2 is in. `RULES.md:262` excludes agreement inherited by reading. **A13 now reads: one
derivation with measurement, one restatement after reading.** The mechanism `116` gave for why its form
loses is worth keeping because it explains the repair rather than only endorsing it, and it is not a
second instance.

**A3 and A4, corrected.** A3 recorded `118` F118-3 as a reproduction of `116` F116-4 on a different
search, which stands. A4 recorded F118-4, F118-5 and F118-6 as three ablations pinning three hypotheses.
**F118-5 is withdrawn**, F118-6 is the same fact as F118-5 at a third size, and F118-4's number survives
with its framing replaced. The rung on the ablation is now: one derivation of the domain condition
(`121` F121-1, on `121_probes/t2`), one reproduction with matched window sizes (`122` F122-3, on `u2`),
and one withdrawn claim of mine.

**A18, new, and it is `120`'s.** Aimed at the check the character selects, the structural certificate is
sound at both behaviours. Derived: `115` F115-1 on `115_probes/s1`, zero violations and zero unsound
cells at all four primitives, with an always-fire control producing violations equal to the conservative
count and a root-check-at-saturation control unsound on 38 and 34. **Rung: ONE EXPERT**, on the same
enumeration A9 and 4.7 rest on. `119` filed this under framings retired, which is where a reader is told
what to stop citing rather than what a clause rests on.

**A19, new.** Saturation carries a deferral licence on the region where a derivation's exact intermediates
keep the sign of the domain. Derived: `122` F122-5 on `u3`, with `121` F121-4 and F121-5 as the
homomorphism facts underneath it and the shipped test as a third instrument. **Rung: ONE EXPERT for the
arm**, resting on `121`'s two findings and on a green test in the tree.

**A20, new, and it is the correction to a clause rather than to an entry.** Both licence families are
available at once on a one-signed domain with no declaration. Derived: `122` F122-6 on `u4`, with two
controls. **Rung: ONE EXPERT**, and it refutes `119` 4.5's "only mechanism", which `120` section 8 flagged
and `121` section 3.6 supplied the material for without either pressing it.

---

## 4. The statement, revised

Clauses are marked. **[STANDS]** means `119`'s text is unchanged and is not restated. **[REPLACED]** means
the previous predicate was wrong. **[AMENDED]** means the clause was true and is now narrower or wider on
a dimension it did not name.

Conventions as `119`: everything enumerative ran on one thread and carries `threads = 1`; the model sweeps
are exact integer arithmetic no instruction selection can move, so `target features any` with that as the
argument; every enumerative result lists its widths as a fixed set and has **no transfer argument** to any
real width, except where a clause says it rests on an argument instead.

### 4.1 The map and its character: [STANDS]

`119` 4.1, unchanged. `121` signs it as definitional.

### 4.2 The characters are mutually exclusive on a domain closed under negation: [REPLACED]

> **No realisation map onto a finite value set is both an additive-group homomorphism and
> order-preserving, unless it is constant.**

*holds for: value set finite with at least two elements; **domain closed under negation**; operations
including addition.*

**The predicate is replaced rather than widened, and the two acts are different.** `119`'s named a
complete residue system and the interval from zero to the value set's size, and `u2` measures that both of
those hold on a domain where the theorem fails, so neither separates the cases. A predicate admitting a
counterexample was never true as stated. The replacement is `121` F121-1's and the reproduction is
`122` F122-3.

**What is measured beyond the theorem, and kept separate from it.** On a domain closed under negation the
conjunction is also empty over addition alone and over multiplication alone, at `|V| in {2, 3}` on windows
of 5 and 7 points. The theorem's proof runs through the additive group and covers the additive case; the
multiplicative-only case is a measurement with that predicate and not a consequence of the proof, and
`121` F121-1's wider phrasing should be read with that distinction rather than without it.

**No width, signedness, fraction width or behaviour appears, and that is still deliberate.** The claim
quantifies over finiteness and over the domain's closure, and its argument does the same, so it holds at
every width. That exemption needs both the statement and the argument to be width-free and is available to
almost nothing else here.

### 4.3 Each character licenses a family, and the disjointness is conditional on the domain: [AMENDED]

> The additive-group homomorphism licenses **deferral**: a reduction may be omitted at a node whose
> operation the map is a homomorphism for, and taken later. Order preservation licenses constructions and
> rewrites that read the map's order.
>
> On a domain closed under negation a map has at most one of the two, so a design has at most one of the
> two families, and may have neither. **On a one-signed domain that exclusion does not hold**, because
> what a saturating map carries there is a semiring homomorphism rather than the additive-group one the
> theorem trades against.

*holds for: W = 3, F = 0, signedness any, overflow behaviour in {wrapping, saturating, flush-to-zero,
reflecting}, rounding = truncation, radix = 2, operations in {add, sub, mul}, construction = interval,
**ambient domain closed under negation**, threads = 1, target features any.*

The added dimension is the domain. `116` F116-5's four-behaviour table is what the predicate covers, and
the amendment is that its disjointness is a fact about that domain rather than about behaviours in
general. The naming of the third structure is `121` section 3.6's.

The order-preserving side still has one known member and enumerating the rest is still open.

### 4.4 The character is a joint fact about the behaviour, the operation, the fraction width and the domain: [REPLACED]

> A **wrapping** map is a homomorphism for addition and subtraction at any fraction width, and for
> multiplication only where the fraction width is zero or the operands are declared on the unit grid. Its
> character does not depend on the domain.
>
> A **saturating** map is a homomorphism for exactly those operations that cannot carry an exact result
> out of the domain's sign. On a domain closed under negation that is no operation. On a one-signed domain
> it is addition and multiplication, and not subtraction.

*holds for: W = 4, F in {0, 1, 2}, signedness in {unsigned, signed}, overflow behaviour in {wrapping,
saturating}, rounding = truncation, radix = 2, operations in {add, sub, mul}, **ambient domain in
{one-signed, closed under negation}**, threads = 1, target features any.*

The wrapping half is `114` F114-1 with `116` F116-7 and `116` F116-9, reproduced at `118` F118-7, and is
unchanged, as is `114` F114-7's control that the character is about the operation set rather than about
the wrapping. The
saturating half of `119` 4.4 was false, was read off my own control column measured on a straddling
domain, and is contradicted by `warm-clamp-shared/src/lib.rs:1105`, which asserts the addition case
directly and passes in all fifteen counts of this sitting. The replacement clause is `121` section 3.5's
and the reproduction is `122` F122-4.

**Stated separately as a widening rather than folded into the predicate**, and this answers `119`'s C2:
the wrapping map's addition and subtraction half holds at **any** fraction width, on the argument that the
grid is closed under those operations so they never enter the rounding region. `121` section 5 agrees the
exemption applies and for the same reason 4.2's does. So it is stated on the argument with the sweep as
its control, and marked.

### 4.5 Two mechanisms restore what a behaviour forfeits, not one: [AMENDED]

> On a restriction the map does not move, the map is the identity there, and the identity has both
> characters. **A domain the map cannot carry a value out of the sign of does the same**, for the
> operations that cannot leave it.
>
> So a design has two ways to hold both families at once: **declare a restriction**, or **work in a domain
> and an operation set the behaviour is a homomorphism on**. Neither is a choice of behaviour, which is
> what cannot buy it.

*holds for: W = 3, F = 0, signedness in {unsigned, signed}, overflow behaviour in {wrapping, saturating},
rounding = truncation, radix = 2, operations in {add, mul} and {add, sub, mul}, arity in {2, 3},
**ambient domain in {one-signed, closed under negation}**, restrictions = upper bounds in {1, 3, 7},
threads = 1, target features any.*

The first mechanism is `116` F116-6, unchanged. `119` said it was the **only** one. `120` section 8 flagged that the quantifier
ranged over mechanisms rather than over maps and left it; `121` section 3.6 supplied the second mechanism
without connecting it to the clause. `u4` measures both characters at once and the clause is corrected.

The second predicate of `119` 4.5, on the unit grid at a nonzero fraction width, stands unchanged with the
domain dimension added.

### 4.6 A reduction has two parts and each has its own locality condition: [AMENDED]

> A reduction composes a **grid part** and a **range part**. The grid part must be applied at the result
> of every node whose exact result can leave the grid. The range part must be applied at the operands of
> every node the map is not a homomorphism for. Everywhere else both may be deferred to the derivation's
> root.

*holds for: W = 4, F in {0, 1, 2}, signedness in {unsigned, signed}, overflow behaviour in {wrapping,
saturating}, rounding = truncation, radix = 2, operations in {add, sub, mul} and {add, sub, mul, min},
term shapes = every term at 2 and 3 leaf slots over the signature in play, arity in {2, 3}, declarations =
one-sided, **ambient domain one-signed**, threads = 1, target features any.*

The rule is `118` F118-11, which subsumes `114` F114-19, unchanged, and its predicate gains the domain, because **which operations the
map is a homomorphism for is now known to depend on it**. What changes with it is `118` F118-12's account
of the rule degenerating and `118` F118-13's count of what it saves per shape, both of which were computed
against the homomorphism test 4.4 has now replaced. `119` said the rule degenerates to reducing at every node
under an order-preserving behaviour because no operation is a homomorphism there. That is true on a domain
closed under negation and false on a one-signed one, where addition and multiplication are homomorphisms
and the rule defers exactly as it does at wrapping. My own q5 measured the saturating rows as degenerating
because its homomorphism test was the one 4.4 has now replaced.

The rule stays **sound** either way, since reducing more often than necessary cannot change an answer.
What was wrong was the account of how much it saves.

### 4.7 The discharge check and its certificate both follow the character: [AMENDED]

> Where the map is a homomorphism for every operation in a derivation, the discharge check reads the
> **root's** propagated bound alone, and a certificate that a refusal by that check is honest needs only
> that every leaf occurs at most once.
>
> Where it is not, the check reads **every node**, and the certificate needs additionally that no node
> sits beneath a multiplication whose sibling bound contains zero.
>
> Neither the check nor its certificate is chosen. Both are consequences of the character, and the
> character is a joint fact about the behaviour, the operation, the fraction width and the domain.

*The character split holds for: W = 3, F = 0, signedness in {unsigned, signed}, overflow behaviour in
{wrapping, saturating}, rounding = truncation, radix = 2, operations in {add, sub, mul}, term shapes =
every term at 2 and 3 leaf slots with every leaf identification, arity in {2, 3}, declarations =
one-sided exhaustive, **ambient domain closed under negation for the saturating arm**, discharge check =
root under a homomorphism and per node otherwise, threads = 1, target features any.*

*The non-homomorphic half alone holds for:* as `119` 4.7's second predicate, unchanged, with the same
domain dimension added.

The certificate is `118` F118-1, its region at saturation is `114` F114-4, and the oracle agreement that
makes a refusal honest is `114` F114-5. The root arm itself is `114` F114-6. All four stand.

**The amendment is the domain, and it changes which arm a saturating behaviour lands in.** `119` read
"saturating" as always taking the per-node arm. `u3` measures that with one-signed declarations and no
subtraction the root-only check is sound on 5840 and 39584 cells with zero unsound, and that every cell
where it was unsound at saturation in `118` q1 contains a subtraction. So a saturating behaviour takes the
root arm on the region where its intermediates keep the domain's sign, and the per-node arm elsewhere.

**And "no subtraction" is not the condition.** On a signed container with declarations reaching below zero
and no subtraction present, the deferral fails on 656 of 968 cells. The condition is that the exact
intermediates keep the sign of the domain; subtraction-freeness over one-signed declarations is a
sufficient syntactic condition for it. `119` would have stated the syntactic form as the condition and
that control is what stopped it.

**One widening available and not taken.** `121` section 5 notes that its F116-8 supports the discharge
check's behaviour at `F in {0, 1, 2}` while 4.7 is pinned at `F = 0`, and does not press it. I decline it
too, and for a reason worth stating: taking an unreproduced widening in the same revision that repairs a
false clause on the same axis is how a corrected document acquires its next defect.

### 4.8 Propagation rules are not unique and compose by intersection: [STANDS]

`119` 4.8, unchanged, resting on `114` F114-11. Signed by `121`.

### 4.9 A construction carries its own transformer, and for a bilinear one it is a formula: [STANDS]

`119` 4.9, unchanged, resting on `114` F114-12 and reproduced at `116` F116-10. Signed by `121`, and its
own author declared its own table superseded by it.

**One connection worth naming and not a change.** The formula's negative-entry bit, which says a row with
a negative structure constant over a value set with no negative values is not dischargeable by any
magnitude bound, is the same domain-sign condition 4.4 and 4.5 turn on, arriving on the composite layer.
That the two are one condition was not visible when either was written.

### 4.10 A rule's cost is paid where its carrier is instantiated: [AMENDED]

> A design chooses between propagation rules by choosing **which carrier a derivation is given**, not by
> choosing which verdict is read. A choice made after both carriers exist has already paid for both, and
> no arrangement of the choice at the point of use recovers that.
>
> The certificate in 4.7 is computable from the cheaper carrier. **Both its conditions are also functions
> of the derivation's syntax, which is an argument rather than a measurement and which nothing has
> built.**
>
> The cost is a **recursion depth in the trait solver**, equal to the derivation's spine depth **plus**
> the carrier's per-node state rather than times it, so a carrier with per-node state reaches a fixed
> limit at a fraction of the derivation size a stateless one does. The **product** of the two is the count
> of associated-const cells, which is a different quantity and is `114` F114-18's.

*holds for: toolchain = the pinned nightly, edition 2021, crate type = library, feature gates = none,
term = a left-nested fold, **recursion limit in {16, 24, 32, 48, 64, 96}**, threads any, target features
any.*

Three changes, all `120`'s. The mechanism sentence is corrected from a product to a sum, measured at
`u1` with the cheap wall exactly `limit - 1` and the expensive exactly `limit / 2 - 1` at every limit. The
predicate gains `recursion limit`, which is the dimension that discriminates the two laws and which `114`
F114-17 also omits. And the syntax half of the certificate carries the mark its author put on it.

**No duration is claimed and none was measured.** The harness has no compile-time mode and a figure taken
anywhere else could not be called a measurement.

---

## 5. The sweep, because a defect in two clauses is evidence of a class

Both dissents are the same missing dimension. Fixing only the two clauses where it was caught would leave
the class in place, so `u4` extracts every predicate in `119` mechanically and checks each for a dimension
naming the domain, its sign, or the ambient span.

**Eleven predicates. One names a domain-ish dimension, and that one is 4.2's, which is the wrong
condition. Ten name none.**

Of those ten, **nine needed it** and are amended above: 4.3, 4.4, both of 4.5's, 4.6, both of 4.7's, 4.8
and 4.9. The tenth is 4.10, which is a compile-time claim with no domain in it and which needed a
different missing dimension instead.

4.8 and 4.9 are marked [STANDS] because their **statements** are unaffected: the intersection is sound on
any domain and the bilinear formula already carries signedness, which is the domain-sign condition wearing
a different name. What both gain is the naming, and I have added the dimension to neither predicate rather
than widen a claim on an axis I did not re-sweep for them. **That is a gap and it is deliberate**: the
honest state is that their predicates do not name the domain and their measurements were taken on one, and
a later pass should sweep them rather than have this one assert them.

**The extractor's controls fired.** It reports zero on a predicate with no domain dimension, does not fire
on the word "declarations" which is present nearly everywhere and is a different dimension, and does fire
on a real one. And it found two predicates my first version of it missed, in 4.7, because they are written
with text before "holds for" inside the same emphasis span.

---

## 6. Findings, each with its predicate

**F122-1. The compile wall is additive in the recursion limit, not multiplicative.** On `114_probes/p9`'s
own generator the cheap tower's wall is exactly `limit - 1` and the expensive tower's exactly
`limit / 2 - 1`, at six limits from 16 to 96. Mean absolute error 1.00 against a bare `limit / 2`, which
is the offset, against 5.71 for a square-root model fitted mid-range. `toolchain = nightly-2026-05-28,
edition 2021, crate type = library, feature gates = none, term = a left-nested fold of adds, recursion
limit in {16, 24, 32, 48, 64, 96}, fold length searched from 2 to 400 by bisection, threads any, target
features any`. `u1_output.txt`. Controls: a variant forcing neither tower compiles at every length at
every limit, and the cheap wall moves across six distinct values. **Reproduces `120` F-t3 on an
independent generator and refutes `119:566`.**

**F122-2. `119` 4.2's stated predicate admits a counterexample.** A map clamping to sixteen values on the
domain `0..47` is a homomorphism for addition and multiplication, fails for subtraction, is
order-preserving, is non-constant, and satisfies both stated domain conditions. `value set size = 16,
domain in {0..47, -24..24}, map = saturating clamp, operations in {add, sub, mul}, threads = 1, target
features any`. `u2_output.txt`. **Reproduces `121` F121-2.**

**F122-3. With window sizes matched, the domain's sign separates the theorem's cases and the operation
set does not.** Every straddling row empty and every non-negative row non-empty, at `{add, mul}`, `{add}`
and `{mul}` alike, at `|V| in {2, 3}` over windows of 5 and 7 points; and closing a non-negative window
under negation empties it again. `value set size in {2, 3}, window in {straddling zero, non-negative,
non-negative closed under negation} at matched point counts, operations in {{add, mul}, {add}, {mul}},
monotonicity under some total order, threads = 1, target features any`. `u2_output.txt`. Controls: each
half of the conjunction non-empty on its own on every row, and both of `119` 4.2's conditions true on both
rows. **Reproduces `121` F121-1 with the sizes matched, and withdraws my own F118-5.**

**F122-4. The saturating homomorphism verdict moves with the ambient domain and the wrapping one does
not.** Saturating at `uW4`: 0 of 1089 for addition and multiplication and 391 of 1089 for subtraction on
`0..45`; 720, 751 and 256 of 2401 on `-48..48`. Wrapping: zero on every operation on both. `W = 4, F = 0,
signedness = unsigned, overflow behaviour in {sat, wrap}, rounding = trunc, radix = 2, operations in {add,
sub, mul}, ambient domain in {0..45, -48..48}, threads = 1, target features any`. `u3_output.txt`.
**Reproduces `121` F121-4 and reproduces my own 720 as the straddling row.**

**F122-5. Saturation carries the deferral licence and the root-only discharge check on the region where a
derivation's exact intermediates keep the domain's sign.** Zero deferral failures and zero root-only
unsound cells over 5840 and 39584 cells on subtraction-free terms with one-signed declarations, with 4002
and 32104 cells reaching an exact result outside the container; 2161 and 17840 deferral failures and 38
and 142 unsound with subtraction present. All 38 and all 34 of `118` q1's saturating root-only unsound
cells contain a subtraction and none is subtraction-free. `W in {3, 4}, F = 0, signedness in {unsigned,
signed}, overflow behaviour in {sat, wrap}, rounding = trunc, radix = 2, operations in {add, mul} and
{add, sub, mul}, term shapes = every term at 2 and 3 leaf slots, arity in {2, 3}, declarations in
{one-sided, two-sided reaching below zero}, threads = 1, target features any`. `u3_output.txt`. Control:
on a signed container with declarations reaching below zero and no subtraction, the deferral fails on 656
of 968 and 4416 of 5840, so subtraction-freeness is a sufficient condition and not the condition.
**This is new and rests on `121` F121-4.**

**F122-6. Both licence families are available at once on a one-signed domain with no declaration.** A
saturating map on `0..7` over `{add, mul}` is order-preserving and carries the deferral licence, 0 of 2176
failures. `W = 3, F = 0, signedness = unsigned, overflow behaviour in {sat, wrap}, rounding = trunc, radix
= 2, operations in {add, mul} and {add, sub, mul}, term shapes = every term at 2 and 3 nodes over the
signature, ambient domain in {one-signed, closed under negation}, threads = 1, target features any`.
`u4_output.txt`. Controls: closing the domain under negation costs the deferral column, adding subtraction
costs it, and every wrapping row loses the order column. **Refutes `119` 4.5's "only mechanism".**

**F122-7. Ten of eleven predicates in `119` name no dimension for the domain, its sign or the ambient
span.** The one that does is 4.2's and names the wrong condition. `document = 119 as landed, extractor
pattern as stated in the probe`. `u4_output.txt`. Controls: the extractor reports zero on a predicate with
no domain dimension, does not fire on "declarations", and fires on a real one.

**F122-8. The shipped retraction test's identity re-derives.** Eager and deferred clamping agree on 0 of
64, 512, 256, 4096, 1024 and 32768 non-negative addition folds at `W in {3, 4, 5}` and arity in `{2, 3}`.
`W in {3, 4, 5}, F = 0, signedness = unsigned, overflow behaviour = sat, operation = add, arity in {2, 3},
operands exhaustive over the declared range, threads = 1, target features any`. `u3_output.txt`,
against `warm-clamp-shared/src/lib.rs:1105`. **Reproduces `121` F121-5.**

**Unpriced.** Every duration. No bench harness ran and no claim here depends on a magnitude.

---

## 7. What I withdraw, and what the revision did not settle

**Withdrawn.** `118` F118-5, whose witness does not isolate the operation it names. `119` 4.2's
predicate, 4.4's saturating clause, 4.5's "only mechanism", and 4.10's mechanism sentence. `119` section
3's flat statement of the syntax half of `115` F115-5, which now carries its author's mark.

**Corrected rather than withdrawn.** `118` F118-4's framing, its number standing. `118` F118-6, which is
F118-5's fact at a third size. `119` A13's rung, on `121`'s own reading against itself.

**Not settled, and carried from `119` section 5 which stands entire.** No transfer argument to real widths
for anything enumerative. Nothing at a non-uniform value set. Whether derivations are trees or graphs,
which both signatories still depend on and neither has looked at. The order-preserving family's
membership. Every duration.

**And three things this revision adds to that list.**

**4.8's and 4.9's predicates still do not name the domain**, and their measurements were taken on one.
Section 5 says why I did not add it: I did not re-sweep them, and asserting a dimension I did not measure
is the defect this whole revision is about.

**The region in 4.7's saturating arm is stated by a sufficient syntactic condition rather than by the
condition itself.** "The exact intermediates keep the sign of the domain" is what holds; "no subtraction
over one-signed declarations" is what a compiler can check. Whether a tighter checkable condition exists
is open and nothing has looked.

**And `121` F121-1's wider claim, that the operation set is not load-bearing at all, is measured and not
proved.** My theorem's proof covers the additive case. The multiplicative-only case is empty at two value
sizes on two small windows and that is all anyone has.

---

## 8. Anchor accounting

Counted on `119`'s own patterns, with this section excluded from the computation, so the two numbers are
comparable. The instrument is `119_probes/r1`, reused rather than rebuilt and pointed at this file
instead. `u5_output.txt` carries the per-class lists.

```
                     in the union   in the doc   not carried
  119 as landed, against the four it compressed
    finding ids                68           65             4
    probe stems                29           25             7
    line anchors               34           18            24

  122, against the seven it revises across
    finding ids                76           45            39
    probe stems                32            6            26
    line anchors               55            5            50
```

**The two rows are not comparable as totals and the probe says so**, because the union grew by three files
while the document shrank. `122` is a revision rather than a compression: it restates the clauses it
changes and carries the rest of `119` by reference, so a clause marked `[STANDS]` deliberately does not
repeat `119`'s anchors.

**The number that is comparable is the one against `119` itself.** `122` carries 45 finding ids against
`119`'s 65: 12 new ones from `121` and from its own probes, and 32 of `119`'s not restated. Those 32 belong
to clauses marked `[STANDS]`, to `119`'s retirement list, and to ledger entries this file does not touch.
On the other two classes it carries 6 probe stems against 25 and 5 line anchors against 18, and the same
reading applies to both.

**Two passes moved anchors back.** The first draft carried 23 finding ids, and a check of each restated
clause against the evidence it rests on found 18 missing and then a further 4, taking it to 45. Every one is now present at
its point of use, and the check that says so is in `u6`, which fails if a clause's evidence is absent
rather than only if a citation is wrong.

**The line-anchor drop is the largest and is again deliberate**, for `119` section 6's reason: a revision
addresses a claim by finding id, which survives a file being edited above it, and keeps a line number only
where the claim is about a document's exact wording. Here that is op's intents, the two sentences the
dissents quote, the hedge at `115:322`, `116:247`, and the shipped test.

---

## 9. Coverage, bounded

**Read in full.** `120`, `121`, and my own `119`. `INTENTS.md`.

**Read in part.** `115:322` and `116:247`, opened. `118_probes/q2` at its `main` and `118_probes/q3` at
its ambient-range construction, both to confirm accusations against my own instruments before conceding.
`warm-clamp-shared/src/lib.rs:1095-1130`, opened.

**Not read.** `120_probes` and `121_probes` sources; `119_probes` beyond `r1`, which this file reuses;
every panel file before `114` except through `119`'s account.

**Reproduced rather than accepted.** `120`'s recursion-depth law, on my own generator. `121`'s
counterexample, its two-by-two with the sizes matched, its reconciliation of the two files' conflicting
numbers, and the shipped test's identity.

**Not reproduced.** `120` F-t2's dimension diff of my own clause against `115`'s predicates, which is a
fact about two documents and which I checked by reading rather than by running. `121`'s citation check.
Everything in `119` neither signatory disputed.

**Citations opened, and evidence checked as well as citations.** `u6` opens all 17 `file:line`
references and, separately, asserts that each of the fifteen restated clauses and ledger deltas carries the
finding ids it rests on. 17 of 17 pass, 3 deliberately wrong citations fail, 15 of 15 clauses carry their
evidence, and the evidence check's own control fires on a finding id that exists nowhere. One citation was
wrong on the first run, off by two lines, and is corrected.

---

## 10. Probe index

- `u0_test_gate_run.txt`. Fifteenth count, per crate, with the test `121` cites run on its own.
- `u1_is_the_wall_additive_or_multiplicative.py`, `u1_output.txt`. `120`'s law on my own generator, six
  limits, two controls.
- `u2_the_domain_was_confounded_with_the_operation_set.py`, `u2_output.txt`. `121`'s counterexample
  recomputed, the missing cell run, the sizes matched, three controls.
- `u3_saturation_has_a_deferral_arm_and_i_missed_it.py`, `u3_output.txt`. The reconciliation reproduced,
  and the arm nobody had measured, with the control that decides how its region is stated.
- `u4_the_only_mechanism_clause_is_false_and_the_predicate_sweep.py`, `u4_output.txt`. Both characters
  measured at once, and every predicate in `119` swept for the missing dimension. One defect of my own in
  it, fixed in view.
- `u5_the_anchor_count_before_and_after.py`, `u5_output.txt`. `119_probes/r1` reused against this file.
- `u6_check_my_own_citations.py`, `u6_output.txt`. Every `file:line` opened, with three wrong controls,
  **plus a second half no checker in this panel has had**: per restated clause, the finding ids that clause
  rests on must be present in the file. It fails on an absence rather than on a wrong reference, which is
  the failure a revision's shape invites and which the first draft had 22 of while every citation it made
  was correct. Seventeen citations pass, three controls fail, fifteen clauses carry their evidence, and the
  evidence control fires.
