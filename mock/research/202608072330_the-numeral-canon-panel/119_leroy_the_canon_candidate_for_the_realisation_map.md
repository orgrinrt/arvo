# 119. Canon candidate: the realisation map, its character, and what each character licenses

**This is a candidate and a draft, not a canon file.** It does not move to `mock/canon/`, nothing in it
is settled, and op ratifies. Per I12 an opinion given before the experts converge is an ack rather than a
ratification, and no opinion has been given here at all.

**It is written for two signatures that have not been given.** `115` and `116` built most of what is
below and are resumed after this to check it and to dissent. Where I state something either of them
would put differently, I have tried to say so at the point rather than smooth it; where I have failed to,
the dissent is theirs to enter and this file should be read as owing it.

Order below is the order the dispatch asks for: the agreement ledger first and in full, because a
compression that leads with its conclusion has already decided what to drop; then what is contested and
what is retired; then the statement; then what this topic did not settle; then the anchor accounting, so
the drop is a measured number rather than an impression.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` with its "How to read an entry" section as normative.

**I13 (`INTENTS.md:214`) governs the shape of section 4 entirely.** Every clause there is an arm with a
const predicate over a region, and there is no universal. Two clauses are stated on an argument rather
than on a sweep and say which; every other clause carries a predicate in the notation, with unmeasured
dimensions unstated per op's instruction at `INTENTS.md:240`.

**I15 is why every predicate in section 4 is compile-time.** Nothing there admits a runtime check. What
the checks read is the declared semantics and the derivation's syntax, both available before any value
exists, which is `INTENTS.md:252`'s "whatever is available at const time".

**I16 is why section 4 states arms rather than a winner.** The candidate does not rank the discharge
checks or the propagation rules; it says which is correct in which region and stops. That is op's
instruction at `INTENTS.md:322` read at this layer.

**I14 holds.** No Rust is compiled in this file. The probes it rests on compile with no feature gates,
no `dyn` and no `TypeId`, which their own outputs assert.

**I1 and I17 are not touched.** Nothing below presumes a strategy count, a strategy name, or a
decomposition. The candidate speaks of an overflow behaviour and a declared semantics, both of which
survive the strategy set being reshaped, renamed or resized.

**No ambiguity to hand back.** Every axis section 4 reads sits in the declared semantics by measurement
rather than by assertion (`112` F112-2 at `112:1002`, `112` F112-3 at `112:1009`), read against the
membership criterion the previous unit states at `108:825`.

### 0.2 Test gate: passed, at 123 across 13, and it is the thirteenth count

`r0_test_gate_run.txt`, per crate by `--manifest-path`, with the toolchain line and the attribute count
agreeing with the run at 123. The workspace-wide form is run beside it and returns its false green
again, per `117:35`.

No test body was re-audited. `114` section 0.2 records what was read then and nothing in this file
touches a new crate.

---

## 1. The agreement ledger

**A rung is carried by naming each instance's instrument, not by the label.** So each entry below names
who derived it, on what, and whether a later instance is an independent derivation or a reproduction.
The distinction is load-bearing and this sitting has one large case of it: the root mechanism has **one
derivation and two reproductions**, because both reproducing parties read `114` first, and it is not a
TWO EXPERTS convergence however many files agree.

### 1.1 Agreed, with each instance's independence stated

**A1. The realisation map's algebraic character is what the overflow behaviour fixes: wrapping is a ring
homomorphism, saturating is not.**

- Derived: `114` F114-1, exhaustive over the ambient range on `114_probes/p2`. 0 of 1728 and 0 of 6912
  at wrap, 400 and 464 of 1728 at sat, with the saturating rows as the control.
- Reproduced: `116` F116-1's mechanism block on `116_probes/p1`, an independently written enumeration,
  0 of 1452 at wrap against 189, 376 and 885 at sat. **`116` read `114` first and says so**, so this is
  a second sighting rather than a second instance.
- Reproduced: `115` s1 indirectly, in that the repair it builds works only if this holds.
- **Rung: one derivation, two reproductions, three instruments.** Not two independent derivations.

**A2. Under a homomorphism the discharge check reads the derivation's root; without one it must read
every node.**

- The saturating half has an **independent and earlier** derivation: `112` F112-21, on
  `112_probes/p7c`, with a hand witness where a root-only check licenses a wrong answer, 9 of 9 tuples.
  That predates `114` and owes it nothing.
- The wrapping half: `114` F114-6, on `114_probes/p2` and `114_probes/p5`, 0 unsound of 13032 and 2148
  at wrap against 38 and 34 at sat. Its mechanism is `114` F114-2, which measures that under a
  homomorphism the arms agree exactly when the root's exact value is in range, with no intermediate node
  in the condition at all.
- Reproduced three times: `116` F116-1 at 76896 and 12432 cells on its own enumeration; `115` F115-3 at
  38 and 34; `118` F118-1's C3 column at 38, 34 and 0, 0.
- **Rung: the two halves have separate provenance.** The saturating half is `112`'s, independently and
  first. The wrapping half is one derivation with three reproductions.

**A3. No map onto a finite value set is both an additive homomorphism and order-preserving, except a
constant one.**

- Derived: `116` F116-4, structural, with the argument at `116:315` and an exhaustive search over 512
  and 1594323 maps on `116_probes/p3` presented as a control on the argument rather than as its evidence.
- Reproduced: `118` F118-3 on `118_probes/q2`, on a different space (maps fixing the value set pointwise)
  and with monotonicity quantified over **every** total order rather than the natural one, which is what
  `116:320` states and its probe does not test. `116`'s own free-map figures reproduce exactly, 3, 10, 2,
  0 and 6, 105, 3, 0.
- **This is `116`'s claim and not a reproduction of `114` F114-1.** It generalises A1 rather than
  confirming it, so A1's instrument count does not transfer to it.
- **Rung: one derivation, one reproduction on a different search.**

**A4. Multiplication is not load-bearing in A3's hypothesis; addition is; and the domain must be wide
enough to contain the interval from zero to the value set's size.**

- Derived: `118` F118-4, F118-5, F118-6, on `118_probes/q2`. Dropping multiplication leaves zero
  non-constant maps passing both; dropping addition produces witnesses; narrowing the domain produces
  witnesses.
- **Rung: ONE EXPERT, unreproduced.** It widens A3 on one hypothesis and pins two others.

**A5. The two licence families are disjoint per behaviour, and a behaviour may have neither.**

- Derived: `116` F116-5 on `116_probes/p3`, four behaviours, of which two carry one character each and
  two carry neither. The two extra behaviours are there so the table is a measurement rather than a
  restatement of two points.
- **Rung: ONE EXPERT, unreproduced.** `118` section 10 says explicitly that it did not reproduce it.

**A6. A declared restriction the map does not move restores both characters at once.**

- Derived: `116` F116-6 on `116_probes/p3`: on a discharged extent the map is the identity and preserves
  order, at both behaviours swept.
- One consequence measured independently at a different object: `118` F118-14 on `118_probes/q6`, where a
  shipped kernel's guard makes its accumulator's behaviour free, 4000 of 4000 chunks agreeing between a
  wrapping and a saturating accumulator. That is a consequence rather than a reproduction and is counted
  as such.
- **Rung: ONE EXPERT for the general claim, with one independently instrumented consequence.**

**A7. The character is a joint fact about the behaviour, the operation and the fraction width.**

- Derived: `116` F116-7 on `116_probes/p4b`: at a nonzero fraction width wrapping stays a homomorphism
  for addition and subtraction and stops being one for multiplication, 608 and 1234 of 2116.
- Reproduced: `118` F118-7 on `118_probes/q3`, 0 of 2304 for add and sub at every fraction width and
  1152 and 1600 of 2304 for mul, with the zero-fraction row as the control.
- The licence consequence is `116` F116-8: the root check stays sound on derivations without a
  multiplication at every fraction width and goes unsound on 2079 to 16063 cells on derivations with one.
- That the character is about the operation rather than about the wrapping is `114` F114-7 and `116`
  F116-2, which each add a non-ring operation and watch the homomorphism and the root check fail with it.
- **Rung: one derivation, one reproduction, with the operation clause separately instrumented twice.**

**A8. The multiplicative homomorphism at a nonzero fraction width is restored on a unit-grid declaration,
and not by products merely avoiding requantisation.**

- Derived: `116` F116-9 on `116_probes/p4b`, which **refuted its own author's predicted mechanism**
  recorded before the run.
- Reproduced: `118` F118-7's grid section on `118_probes/q3`: 1600 of 2304 failures at a quarter grid,
  256 of 576 at a half grid where every product is already on the fine grid, and 0 of 144 at the unit
  grid.
- **Rung: one derivation, one reproduction, including the refutation.**

**A9. The certificate that a discharge check's refusal is honest follows the character too: the
second structural condition is load-bearing without a homomorphism and vacuous with one.**

- Derived: `118` F118-1 on `118_probes/q1`, with `118` F118-2 measuring what the coarser certificate
  costs where the finer one applies: 3072 and 384 cells declined, on every one of which the check is
  already exact.
- **Its two halves were measured separately, by different authors, before the joint claim existed.**
  `114`'s p1 measured the first condition alone at saturation, 234 and 20 violations. `115` F115-2
  measured it alone at wrapping, 9408 and 1200 firing at zero violations. Neither ran the other half.
- **Rung: ONE EXPERT for the joint claim, with each half independently prior.**

**A10. The certificate is sufficient and not necessary, and where it fires and the check refuses an
enumerating oracle also refuses.**

- Derived: `114` F114-5 on `114_probes/p1`, 5384 of 5384, 316 of 316, 63037 of 63037, 50258 of 50258,
  2988 and 2408. The region it holds over is `114` F114-4, which is far wider than the finding it
  repairs originally claimed.
- **Rung: ONE EXPERT, unreproduced.**

**A11. The structural certificate's conditions were right and its aim was wrong.**

- `111` F111-15 claimed soundness at both overflow behaviours. Refuted at wrapping: `114` F114-3, 28 and
  16 violations, on `114_probes/p1` and `114_probes/p2`.
- Reproduced exactly, on `111`'s own committed implementation: `115` s1, 28 and 16, with the cell counts
  13032 and 2148 also matching so the enumerations are like for like.
- Conceded by its author at `115:120`.
- **Rung: one derivation, one exact reproduction on the original instrument.** This is the strongest
  reproduction in the sitting.

**A12. A reduction has two parts and each has its own locality condition.**

- Derived: `118` F118-9, F118-10, F118-11, on `118_probes/q4` and `118_probes/q5`. The two failing sets
  are disjoint, 660 and 648 with an overlap of 0, and one witness of each kind is printed with every
  intermediate.
- The decomposition it uses is not new: `112` F112-4 established that the map has a grid part and a
  range part and that each switches off one region of the map. What is new is that each part has its own
  **locality** condition, which is that decomposition arriving at the deferral question.
- Subsumes `114` F114-19, which is the same rule at zero fraction width where one condition is vacuous.
  That is one author twice, not two instances.
- **Rung: ONE EXPERT for the locality claim, resting on `112` F112-4 for the decomposition.**

**A13. Sound propagation rules compose by per-node intersection, which is at least as tight as
disjoining their verdicts.**

- Derived: `114` F114-11 on `114_probes/p4`, beaten by neither rule on any row, with a halved-radius rule
  reported unsound on 245 to 5689 cells so the counter is live.
- Arrived at independently: `116` section 3.3 reaches the same repair from its own analysis of why its
  form loses, and says it arrived from the mechanism rather than from `114`'s file. **That is a second
  instance of the repair**, on reasoning rather than on measurement.
- **Rung: one derivation with measurement, one independent arrival without.**

**A14. A form's domination is bounded by the sign of the declared lower bound.**

- `112` F112-24 claimed domination on thirteen rows. Bounded: `114` F114-10 on `114_probes/p3`, beaten on
  92 and 593 cells under two-endpoint declarations, with the mechanism named.
- Widened, as a new claim: `116` F116-3 on `116_probes/p2`, beaten on 0 cells wherever every declared
  lower bound is at or above zero, which properly contains the one-sided declarations `112` swept.
- Conceded by its author at `116:200`, whose own `112` F112-23 had already recorded that one-sided
  extents were all that file swept without either finding pointing at the other.
- **Rung: one bounding, one widening, each on its own instrument, neither reproduced by a third.**
  `118` section 10 records that it did not reproduce the widening.

**A15. A construction carries its own transformer, and for a bilinear one it is a formula over the
structure constants rather than a table.**

- Derived: `114` F114-12 and F114-13 on `114_probes/p6`: exact, zero unsound and zero conservative, over
  1024 bilinear products on pairs and over the quaternions at dimension four, with two controls firing.
- Reproduced: `116` F116-10 on `116_probes/p5`, against what `116`'s own `112_probes/p5b` **measured**
  rather than against `114`'s account of it, six of six with three distinct verdicts so the check is live.
- Its author declares its own table superseded, at `116:274`.
- **Rung: one derivation, one reproduction against the artifact it replaces.**

**A16. The cost of a propagation rule is paid where its carrier is instantiated, not where its verdict is
read.**

- Derived: `114` F114-17 on `114_probes/p9`, eleven variants at nine fold lengths, with a variant forcing
  neither tower as the control.
- Reproduced: `115` F115-4 on `115_probes/s2`, an independently built pair of towers at a lowered
  recursion limit, eight variants at seven lengths, six predictions recorded before the run and all six
  holding.
- **Rung: one derivation, one independent reproduction.** With one provenance correction at `118`
  F118-16: the trait-projection route `115` reports as untried was tried, at `114_probes/p9:244-256`,
  which makes F115-4 a second instance rather than a first.

**A17. The certificate is computable from the cheaper carrier and from the derivation's syntax, so the
choice can be made where A16 says it must be.**

- Derived: `115` F115-5 on `115_probes/s3`, compiled with no feature gates and no reference to the
  expensive carrier, eight verdicts matching the model with three of them false, and two mutations each
  moving exactly one verdict.
- **Rung: ONE EXPERT, unreproduced.**

**A18. Arm W0 and its extension.** Dropping every intermediate reduction is unconditionally equal to
reducing at every node where the map is a homomorphism for every operation present; it extends across a
non-homomorphic operation by cutting rather than by weakening; and it splits at a nonzero fraction width
exactly as the discharge check does.

- Derived: `114` F114-8 and F114-9 on `114_probes/p8`; `114` F114-19 and `114` F114-20 on
  `114_probes/p11`; `118` F118-8 on `118_probes/q3`. The fold's own behaviour under the certificate is
  `114` F114-14, exact at every length checked, and what the rule saves per shape is `118` F118-13.
- **Rung: ONE EXPERT throughout, unreproduced.** All three are the same author and that is not three
  instances.

### 1.2 Contested, each with what would decide it

**C1. Whether arm W1's cost accounting should be stated within a behaviour or across a design.**
`116:359-363` prices the cheap check as free within the behaviour and the behaviour as not free across
the design. `118` section 10 accepts the substance and adds one precision: the behaviour sits in the
declared semantics and is supplied by the consumer, so the resolver never faces the trade and the cost
belongs in what a consumer is told a behaviour costs. **This is a located precision rather than a
disagreement**, and it is listed here because neither party has answered the other. **What would decide
it:** whether any real consumer wants both the root-check licence and an order-based construction on the
same values. That is I11's territory and nothing in the repository measures it.

**C2. Whether `116`'s W1b should be stated on its argument or on its sweep.** Its prose at `116:455` says
any fraction width and its predicate at `116:462` says three of them. `118` section 6 argues the gap is
the transfer exemption applying, because the argument that addition and subtraction never enter the
rounding region is fraction-width-free, and proposes stating it on the argument and saying so. **`116`
has not answered.** **What would decide it:** `116`'s own reading of whether that argument is its
evidence or a gloss on its sweep.

**C3. Whether the order-based family has more than one member.** `116` section 8 proposes enumerating it
and expects a list rather than a number. Until that exists, A5's word "family" has one instance in it and
section 4's clause 3 is weaker than it reads. **What would decide it:** the survey.

**C4. Whether a consumer's derivations are trees or directed acyclic graphs.** If a value is used twice
at the source level and the compiler sees a shared node, leaf multiplicity is not a syntactic property
and A9's first condition is not computable the way A17 computes it. `114` section 5.4 names this as the
assumption it is least comfortable with and `115:409` names it as what would decide its own section 5
against itself. **Two files now depend on it and neither has looked.**

### 1.3 Closed, retired, or corrected, so a later reader knows what to stop repeating

**Refuted and not to be cited at the value refuted.**

- `111` F111-15 claims soundness at `overflow policy in {sat, wrap}`. It is false at `wrap`. Cite it at
  `sat` or not at all. (`114` F114-3, reproduced by `115` s1, conceded at `115:120`.)
- `112` section 9's sentence at `112:928`, that checking only a derivation's result rather than every
  node is unsound, carries no predicate and is false where the map is a homomorphism. Its source
  finding F112-21 is correctly predicated, at `112:1116`, and is the thing to cite. (`114` F114-6, reproduced by `116`
  F116-1, conceded at `116:141-153`.)
- `112` F112-24's "beaten on none" holds only where every declared lower bound is at or above zero.
  (`114` F114-10, widened by `116` F116-3.)

**Superseded, and the superseded form should not be extended.**

- `112` F112-14's per-construction table is superseded by the formula in A15. Adding a construction to
  the table is work that the formula does for free, and its own author says so at `116:274`.
- `111` F111-18's "2 against 64" read as a compile-side cost is superseded by `114` F114-18's
  `L(2L - 1)` against `2(2L - 1)`. `115` section 3 concedes it and `115` F115-6 shows the ratio is
  unchanged at 32, so the composition argument that rested on the ratio survives.

**Framings retired, which is different from findings refuted.**

- `114` section 3.1's framing, that the repair to A11 is to delete the `wrap` dimension. Superseded by
  `115:182-187`'s shape and then by `118` section 2.3's condition split. **Do not cite the dimension
  deletion as the repair.**
- `114` section 6.4's attribution of a shipped kernel to arm W1. Under-determined: the guard makes the
  map the identity, and A6 is what the kernel is an instance of. (`118` section 8.1.)
- `115` F115-4's clause that the trait-projection route was untried in `114`. The finding stands and is
  better than its own claim; the clause is wrong. (`118` F118-16.)
- `117`'s first version, which called the bench variant tests an untested surface. Corrected within the
  hour by its own author against a member's probe output: the tests run and the defect is a false green
  from one command. (`117:26-44`.)

**Mechanisms proposed and refuted, listed so nobody re-proposes them.**

- That cutting at a non-homomorphic operation's operands suffices; that cutting at its result suffices;
  that requantising at it suffices; that the rounding mode is the discriminator. All four measured wrong
  at `118_probes/q3`, with the measurements kept in the probe headers.
- That the two-part rule fails under saturation. It degenerates instead. (`118` F118-12.)
- That the grid condition is restored once products avoid requantisation. Refuted by its own author's
  probe. (`116` F116-9.)
- That the fraction grid would be additive to the propagation question. `114` section 5.4 recorded it as
  an expectation and `116` F116-7 refuted it.

**And one thing retired before this topic that stays retired.** `110` F3's third bullet, withdrawn by its
author at `110` R0 as a dead branch whose guarded increment was unreachable. Nothing here revives it.

---

## 2. What the candidate rests on, and one thing it does not

Every clause in section 4 rests on model sweeps, compiled probes, or a structural argument. **No clause
rests on a bench-harness measurement.**

That is worth stating because `117` records that every number this repository's bench directory has
produced was taken at cargo's default release profile rather than the fat-LTO, one-codegen-unit profile
the harness documents, and that which findings rest on a harness artifact is a pass nobody has done.
**For this candidate the answer is none**, so `117`'s defect does not reach it. The one place a shipped
bench crate appears is A6's consequence and A18's shape, where what was read is the crate's **source**
rather than any number it produced, and that source reading was performed independently by two members
(`114` section 6.4 and `115` section 0.2).

---

## 3. Where the doability was established

A canon says which things are doable and cites where that was established rather than reproducing the
proof. For this topic, five:

- **The certificate is expressible under the operating constraints**, with no feature gate, no `dyn` and
  no `TypeId`. Its top-down-looking second condition reduces to a local test at each multiplication.
  Established at `114` F114-15 on `114_probes/p7`; the compiled certificate riding on the cheap carrier
  alone is `115` F115-5 on `115_probes/s3`.
- **The discharge check can be selected by the character at compile time**, and both arms erase to symbol
  aliases with no residue. `114` F114-16, `114_probes/p7_asm.s`. The per-node form's own expressibility
  and erasure is earlier, at `112` F112-22.
- **The expensive carrier cannot be avoided by any arrangement of the choice at the point of use**, and
  can be avoided by choosing before instantiation. `114` F114-17, `115` F115-4.
- **The bilinear transformer is computable from the structure constants alone.** `114` F114-12,
  reproduced at `116` F116-10.
- **The mutual exclusion of the two characters is a theorem rather than an observation**, so it needs no
  width transfer argument. `116:315`, ablated at `118` F118-4 through F118-6.

---

## 4. The statement

Written in the canon's own register: what a thing is, what it requires, what it excludes. Every clause
carries its predicate. Where a clause is stated on an argument rather than on a sweep it says so, and
where a widening is warranted it is a separate sentence rather than an edit to the predicate.

### 4.1 The map and its character

> A primitive's **realisation map** takes an exact result back into the primitive's value set. Beyond
> what it computes, the map has an **algebraic character**: the set of algebraic properties it possesses.
> Two of them decide what a design may do with it.
>
> The map is a **homomorphism for an operation** when reducing before that operation and reducing after
> it agree. It is **order-preserving** when it never inverts the order of two exact results.

*Definitional; no predicate.*

### 4.2 The characters are mutually exclusive, and that is forced rather than observed

> **No realisation map onto a finite value set is both an additive homomorphism and order-preserving,
> unless it is constant.** So no choice of overflow behaviour makes both available, and this is a
> theorem about finite value sets rather than a gap in the behaviours anyone has tried.

*holds for: value set finite with at least two elements; operations including addition; domain
containing a complete residue system and the interval from zero to the value set's size.*

Three notes, each of which is a bound rather than a decoration.

Multiplication is **not** required in the hypothesis; the claim is about the additive structure.
Addition **is** required: over multiplication alone a non-constant order-preserving homomorphism exists.
And the domain's width is required: on a domain narrower than the value set the identity satisfies both,
so a probe built on a narrow window would report this false and be measuring nothing.

**No width, signedness, fraction width or behaviour appears in that predicate, and the omission is
deliberate.** The claim quantifies over finiteness rather than over size, and its argument does the same,
so it holds at every width including real ones. That exemption is available to a claim only when both its
statement and its argument are width-free, and it is available to almost nothing else in this topic.

### 4.3 Each character licenses a family, and the families do not overlap

> The homomorphism licenses **deferral**: a reduction may be omitted at a node whose operation the map is
> a homomorphism for, and taken later. Order preservation licenses constructions and rewrites that read
> the map's order.
>
> Because a map has at most one of the two characters, a design has at most one of the two families. A
> behaviour may also have **neither**, and then it has no algebraic licence at all.

*holds for: W = 3, F = 0, signedness any, overflow behaviour in {wrapping, saturating, flush-to-zero,
reflecting}, rounding = truncation, radix = 2, operations in {add, sub, mul}, construction = interval,
threads = 1, target features any.*

The word "family" is honest about its plural only for the deferral side. **The order-preserving side has
one known member**, `110` F12's interval construction, whose predicate `112` F112-12 then showed a
declared restriction can discharge. Enumerating the rest is named as open at C3.

### 4.4 The character is a joint fact about the behaviour, the operation and the fraction width

> A wrapping map is a homomorphism for addition and subtraction, and for multiplication only where the
> fraction width is zero. A saturating map is a homomorphism for no operation and is order-preserving.
> So "which character does this design have" is not a question about the overflow behaviour alone.

*holds for: W = 4, F in {0, 1, 2}, signedness in {unsigned, signed}, overflow behaviour in {wrapping,
saturating}, rounding = truncation, radix = 2, operations in {add, sub, mul}, threads = 1, target
features any.*

**Stated separately as a widening rather than folded into the predicate above:** the addition and
subtraction half holds at **any** fraction width, on the argument that the grid is closed under those
operations so they never enter the rounding region. That is an argument rather than a sweep and is
labelled as one. Whether to state it that way is C2, and it is `116`'s call rather than mine.

### 4.5 A declaration restores what a behaviour forfeits

> On a restriction the map does not move, the map is the identity there. The identity has both
> characters. **A declared restriction is therefore the only mechanism that makes both families available
> at once**, and no choice of behaviour can buy that.

*holds for: W = 3, F = 0, signedness = unsigned, overflow behaviour in {wrapping, saturating}, rounding
= truncation, radix = 2, operation = add, arity = 2, restrictions = upper bounds in {1, 3, 7}, threads =
1, target features any.*

The same shape recurs one level down. At a nonzero fraction width a declaration **to the unit grid**
restores the multiplicative homomorphism. Not because products then avoid requantisation, which is
refuted: a grid on which every product is already exact still loses the homomorphism.

*holds for: W = 4, F = 2, signedness = unsigned, overflow behaviour = wrapping, rounding = truncation,
radix = 2, operation = mul, declared grid step in {1/4, 1/2, 1, 2}, threads = 1, target features any.*

### 4.6 A reduction has two parts and each has its own locality condition

> A reduction is a composition of a **grid part**, which returns a value to the representable spacing,
> and a **range part**, which returns it to the representable extent. They are not interchangeable and
> they are not local for the same reason.
>
> The grid part must be applied at the **result** of every node whose exact result can leave the grid.
> The range part must be applied at the **operands** of every node the map is not a homomorphism for.
> Everywhere else both may be deferred to the derivation's root, and the deferred computation agrees with
> the fully reduced one exactly.

*holds for: W = 4, F in {0, 1, 2}, signedness in {unsigned, signed}, overflow behaviour in {wrapping,
saturating}, rounding = truncation, radix = 2, operations in {add, sub, mul} and in {add, sub, mul, min},
term shapes = every term at 2 and 3 leaf slots over the signature in play, arity in {2, 3}, declarations
= one-sided, threads = 1, target features any.*

Two consequences, each stated because each was reached by a wrong route first.

The condition is about the **operation's character here**, not about the operation's name. Where the map
is a homomorphism for every operation present, both conditions are vacuous and nothing need be reduced
before the root.

Under an order-preserving behaviour the rule **degenerates** to reducing at every node rather than
failing, because no operation is a homomorphism there. What it saves is therefore a function of how many
homomorphic nodes a derivation has: large on a fold, and negative on a derivation dense in
non-homomorphic operations, which `118` F118-13 counts per shape.

### 4.7 The discharge check and its certificate both follow the character

> Where the map is a homomorphism for every operation in a derivation, the discharge check reads the
> **root's** propagated bound alone, and a certificate that a refusal by that check is honest needs only
> that every leaf occurs at most once.
>
> Where it is not, the check reads **every node**, and the certificate needs additionally that no node
> sits beneath a multiplication whose sibling bound contains zero.
>
> So neither the check nor its certificate is chosen. Both are consequences of the character.

**The clause is two claims measured over different regions, so it carries two predicates rather than one
widened to cover both.** Folding them would claim the character split at a width where only half of it
was measured, which is the quiet widening this notation exists to prevent.

*The character split, that which check and which conditions apply follows the map, holds for: W = 3,
F = 0, signedness in {unsigned, signed}, overflow behaviour in {wrapping, saturating}, rounding =
truncation, radix = 2, operations in {add, sub, mul}, term shapes = every term at 2 and 3 leaf slots with
every leaf identification, arity in {2, 3}, declarations = one-sided exhaustive, discharge check = root
under a homomorphism and per node otherwise, threads = 1, target features any.*

*The non-homomorphic half alone, that the two conditions certify the per-node check, holds for: W in
{2, 3}, F = 0, signedness in {unsigned, signed}, overflow behaviour = saturating, rounding = truncation,
radix = 2, operations in {add, sub, mul}, term shapes = every term at 2 and 3 leaf slots with every leaf
identification and 120 of 2025 sampled at 4, arity in {2, 3, 4}, declarations = one-sided exhaustive and
two-endpoint exhaustive at arity 2 and sampled at arity 3, threads = 1, target features any.*

The certificate is **sufficient and not necessary**, and it is so by necessity rather than by choice: an
exact condition would quantify over the whole declaration box, whose size grows with the width, so it
cannot be a compile-time predicate at a real width. Where the certificate fires and the check refuses, an
enumerating oracle also refuses, so the refusal is honest rather than merely unlucky.

### 4.8 Propagation rules are not unique and compose by intersection

> A propagation rule is **sound** when it over-approximates the reachable set at every node it is checked
> at. No sound rule is uniquely best: one loses where a leaf repeats, another where quantities multiply,
> a third where a declared bound lies below zero.
>
> Sound rules **intersect at each node** into a sound rule that is at least as tight as either, and at
> least as tight as disjoining their verdicts. A design carrying two therefore carries their intersection
> rather than their disjunction.

*holds for: W = 3, F = 0, signedness in {unsigned, signed}, overflow behaviour in {wrapping, saturating},
rounding = truncation, radix = 2, operations in {add, sub, mul}, term shapes = every term at 2 and 3 leaf
slots, arity in {2, 3}, declarations = one-sided exhaustive and two-endpoint exhaustive at arity 2 and
sampled at arity 3, threads = 1, target features any.*

### 4.9 A construction carries its own transformer, and for a bilinear one it is a formula

> A construction on primitives does **not** inherit its base's rule for transforming a declaration, and
> borrowing the base's rule is unsound.
>
> For a **bilinear** construction the transformer is determined by two things read off the structure
> constants: the L1 norm of each output component's row, and whether that row carries a negative entry,
> the second read against the base's signedness. A row with a negative entry over a value set with no
> negative values is not dischargeable by any magnitude bound at all.

*holds for: W in {3, 5, 6}, F = 0, signedness in {unsigned, signed}, overflow behaviour in {wrapping,
saturating}, radix = 2, dimension in {2, 4}, structure constants in {-1, 0, 1} with one or two nonzero
entries per row, operation = mul, arity = 2, declarations = a uniform magnitude bound on every component,
threads = 1, target features any.*

Scope, stated because it is narrower than it reads: **bilinear** constructions. A construction whose
product is a hull rather than a bilinear form is its own obligation and its predicate is a monotonicity
condition rather than a norm.

### 4.10 A rule's cost is paid where its carrier is instantiated

> A design chooses between propagation rules by choosing **which carrier a derivation is given**, not by
> choosing which verdict is read. A choice made after both carriers exist has already paid for both, and
> no arrangement of the choice at the point of use recovers that.
>
> The certificate in 4.7 is computable from the cheaper carrier and from the derivation's syntax alone,
> so the choice is decidable at the point where it must be made.

*holds for: toolchain = the pinned nightly, edition 2021, crate type = library, feature gates = none,
term = a left-nested fold, fold length in {2, 8, 16, 24, 32, 48, 64, 96, 128}, threads any, target
features any.*

The cost is a **recursion depth in the trait solver**, proportional to the derivation's size times the
carrier's per-node state, and it is a configurable limit rather than a hard ceiling. **No duration is
claimed here and none was measured**, because the bench harness has no compile-time mode and a figure
taken anywhere else could not be called a measurement.

---

## 5. What this topic did not settle

Stated at length, because an honest open question is worth more than a sentence that reads settled.

**No transfer argument to real widths, for anything enumerative.** Every clause in section 4 except 4.2
and the widening in 4.4 lists a fixed set of widths and claims nothing outside it. The two exceptions are
argued rather than swept and say so. The test for whether a claim needs one is whether a width appears in
its statement **and** in its argument; almost everything here fails it.

**Nothing at a non-uniform value set.** Every primitive in this topic has uniform spacing. That is where
the floating-point side lives and no probe in this sitting reached it.

**Whether derivations are trees or graphs.** C4, and two files depend on it.

**The order-preserving family's membership.** C3.

**The one control that did not discriminate.** `114`'s cut rule at zero fraction width behaves under
saturation in a way its own control could not distinguish, on 2880 mixed-term cells. The wrapping result
is supported by every control; the saturating side is not established either way and no mechanism is
offered for it.

**Every duration.** The compile-side cost of section 4.10; whether the narrower accumulator `118`
F118-15 makes available is faster; what a reduction costs at all. The harness could price the second
and did not run. The first it cannot price at all without a mode it does not have.

**Whether the rounding mode has the same character-selecting role as the overflow behaviour.** `116`
section 7 names it as the first place it would look. Nothing measured it, and section 4.4 lists rounding
at a fixed value everywhere for that reason.

**And what only op can decide.** Whether this is the right decomposition of the topic at all; whether the
character belongs in the declared semantics as section 4 assumes; how any of this meets the strategy
axis, which I1 leaves open at every level including how many strategies there are and what they are
called. Nothing here presumes an answer to any of those and nothing here should be read as proposing one.

---

## 6. Anchor accounting

The compression's own measurement, per `a-compression-is-checked-by-someone-else.md`, with the set
difference computed rather than the count alone and with this section excluded from the computation so
that listing a dropped anchor cannot make it present.

`r1_output.txt` carries the numbers, the patterns they were counted under, and the full dropped and
carried lists per class. What it reports, after two passes in which anchors the candidate turned out to
rest on were carried back in:

```
  class                in the four   in 119   dropped
  finding ids                   68       65         4
  probe files (stems)           29       25         7
  line anchors (panel)          34       18        24
```

**The four dropped finding ids are named**, because a bare count invites the reader to assume the worst
of them: `F111-10`, whose two-quantity split is better addressed by `112` F112-4 which is carried;
`F112-16`, a package-naming trap in a test gate; `F116-11` and `F14`, both test-gate bookkeeping. None
is load-bearing for any clause and none is retired; they live where they were written.

**The seven dropped probe stems are all test-gate transcripts and citation checkers**, `p0`, `q0`, `s0`,
`p10`, `q7`, `s4`, plus `p8c` whose claim the carried `112` F112-22 addresses. No clause rests on one.

**The line-anchor drop is the largest and is the one to read carefully.** Twenty-four of thirty-four
panel line anchors are not carried, and that is deliberate rather than incidental: a canon candidate
addresses a claim by its **finding id**, which survives a file being edited above it, rather than by a
line number, which does not. The candidate keeps a line anchor only where the claim is about a
document's exact wording, which is the one case a finding id cannot serve: op's intents, the two
sentences section 1.3 retires, the two places `116`'s prose and predicate differ, and the concessions.

Four notes on the instrument itself, since a compression that measures itself badly is worse than one
that does not measure.

**One of the dispatch's three stated counts reproduces and two do not.** The 66 distinct finding ids
reproduce exactly under the `F<number>-<number>` pattern, which is 66 of the 68 above, the other two
being the bare `F<number>` form `110` uses. The probe-file count is 73 by filename and 29 by stem,
neither of which is the stated 52; the line-anchor count is 47 over any target and 34 into panel files
once ranges are normalised, neither of which is the stated 11. The probe prints both patterns so a reader
can see which counting is meant, and the set difference is computed on one pattern consistently on both
sides, so the preservation check does not depend on which reading is preferred.

**The set difference over exact anchor strings overstates the drop**, because a later file citing the
same target with a different range reads as a different address. Normalising an anchor to its target and
first line moves the panel line-anchor drop from 30 to 24, so the artifact is real and is smaller than it
might have been. Both figures are in the probe.

**A dropped anchor is not automatically a defect here, and this is where a canon candidate differs from
an audit-trail compression.** `the-canon-is-intent-not-implementation.md` says the canon points at where
doability was established rather than reproducing it, so the anchors a candidate owes are the ones its
own clauses rest on. What it must not do is drop those silently, which is what the set difference
measures, and two passes of that measurement moved twenty finding ids and three line anchors back into
the text.

**The measurement excludes this section, and the guard fired.** A candidate that lists the anchors it
dropped makes them present in its own text and disables the check. The probe strips any section headed
as anchor accounting before computing and reports both counts, and here they differ: 65 finding ids in
the body against 69 including this section. The four extra are exactly the four named two paragraphs
above. So this is not a hypothetical guard that happened to be quiet; it caught the disabling on the
first run of the finished text, which is the only reason the 65 above is a measurement rather than a
number this section manufactured about itself.

**And the citations are opened rather than trusted.** `r2_output.txt` checks all 24 `file:line`
references this candidate makes against the substring each claim depends on, with three deliberately
wrong entries as controls. 24 of 24 pass and all three controls fail. Every one of those anchors is
load-bearing by construction, because a candidate keeps a line anchor only where the claim is about a
document's exact wording: op's intents, the sentences retired, the concessions, and the two places a
file's prose and its predicate differ.

## 7. For the two signatures this draft is written for

`115` and `116` are resumed after this. Four things I would most want each to look at, because they are
where I am most likely to have compressed something out of shape.

**To `115`.** Section 4.7 states the certificate as two condition sets rather than one, which is your
F115-2 folded into your F115-1 and is a change to your own finding's shape. Section 1.1 A9 records the
joint claim as mine and both halves as separately prior, one of them yours. And section 1.3 retires the
provenance clause in F115-4 while keeping the finding; if you would rather restate it yourself, that is
yours to do and this file should follow.

**To `116`.** Section 4.2 states your theorem without multiplication in the hypothesis, on the strength
of an ablation of mine rather than of your argument, and section 4.4's widening is a proposal about how
to state your own W1b that you have not answered. Section 1.1 A3 records your F116-4 as one derivation
and one reproduction, deliberately not as a convergence with my F114-1, because it generalises that
rather than confirming it. If you read the relationship differently, the ledger is where it should be
corrected.

**To both.** Section 1.1's rung column is where I am most likely to have flattered the sitting. A1 in
particular is one derivation and two reproductions and reads, in prose, like three files agreeing. If any
entry there claims more independence than its instruments support, that is the correction worth making
before anything else in this file.

---

## 8. Coverage, and the probe index

**Read in full.** `114`, `115`, `116`, `117`, `118`, `INTENTS.md`.

**Read in part.** `115_probes/s1` and its output; `116_probes/p3` at its search structure;
`114_probes/p9` at the variant definitions; `warm-clamp-shared/src/lib.rs` at the four cited line ranges.

**Not read.** `115_probes/s2`, `s3`, `s4`; `116_probes/p1`, `p2`, `p4`, `p4b`, `p5`, `p6`; `OPTIONS.md`,
`AGREEMENTS.md`, `DROPLIST.md`, `RULES.md`; every panel file before `108` except through `114`'s account.

**Reproduced nothing new in this file.** The candidate is a compression. Its two probes are the test gate
and the anchor instrument; every measurement it cites was taken in `114`, `115`, `116` or `118` and is
attributed there.

**Probes.**

- `r0_test_gate_run.txt`. Thirteenth count, per crate by `--manifest-path`, with the false-green form run
  beside it.
- `r1_the_anchor_inventory_and_what_the_candidate_carries.py`, `r1_output.txt`. The anchor inventory
  across the four source files, the dispatch's three counts checked against it, the set difference this
  candidate leaves, and three controls including one that excludes this file's own accounting section
  from the computation.
- `r2_check_my_own_citations.py`, `r2_output.txt`. Every `file:line` this candidate leans on, opened, with
  deliberately wrong controls.
