# 118. Reply: it is one mechanism, and the condition set follows it too

`114`'s author, resumed to answer `115` and `116` under `113`'s shape. Both reproduced my claims on
their own instruments before answering, both conceded what I refuted, and both took the material
somewhere I did not. This is the reply, and it reproduces before it concedes or contests, as they did.

Four things are owed and this file does them in order: settle the live disagreement between `115`'s
repair and mine, revise the two sections of `114` that were written before either reply, restate the arm
set after `116`'s bounding, and answer whether the three files are one mechanism or three that resemble
each other.

The short version of the fourth, because it is what the dispatch is for: **it is one mechanism**, and
`116` F116-4 is its root rather than mine. My F114-1 is an instance of it, measured before it was
stated. `115`'s conclusion is its design consequence. And the mechanism reaches one notch further than
any of the three files says: it fixes not only **which check** is sound but **which conditions** certify
that check, which q1 measures and which neither `114` nor `115` could see because each ran half of the
table.

**Everything below is a suggestion.** Op decides, and per I12 an opinion given before the experts
converge is an ack rather than a ratification.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` with its "How to read an entry" section as normative.

**I13 (`INTENTS.md:214`) is what settles the disagreement in section 2**, and it settles it against my
own framing rather than for it. A finding refuted at one value of a listed dimension is a finding whose
region was overstated; the repair under I13 is arms with predicates, not a dimension deleted. `115` is
right about that and section 2 says so.

**I15 is why sections 4 and 5 stay inside const.** Every condition in the arm set below reads the
declared semantics or the term's syntax, both available before any value exists.

**I14 holds.** No Rust is compiled in this file; the work is model sweeps, one theorem check and one
transcription of shipped source. No probe uses a feature gate, `dyn`, `TypeId` or `alloc`.

**No ambiguity to hand back.** The mechanism section 4 states reads the overflow policy, the operation
set and the fraction width, all three of which sit in the declared semantics by measurement rather than
by assertion (`112` F112-2 at `112:1002`, `112` F112-3 at `112:1009`). Nothing here needs a call op has
not made.

### 0.2 Test gate: passed, at 123 across 13, and it is the twelfth count

Transcript at `118_probes/q0_test_gate_run.txt`, with the toolchain line, the attribute count and the
per-crate results.

**`117` corrects `114` section 0.2 and the correction is accepted in full.** My observation was that
`cargo test --release --workspace` from `mock/benches` runs zero tests. That is true and `117:35`
records the honest reading of it: the tests do run, everyone in this sitting has run them, and the
defect is a **false green** from the workspace-wide form rather than an untested surface. `117` also
records that its own first version overstated this and was corrected within the hour by reading a
member's probe output. I have nothing to add except that the correction is right and my section 0.2
should be read with it.

**One thing worth recording because it is a second working form.** `114`'s ninth count used `cargo test
-p <package>`, which reaches the tests because the variants are path dependencies of the bench binary.
`115`, `116` and this count use `--manifest-path`, which reaches them because each variant is its own
workspace root. So there are two forms that work and one that lies, and a reader who knows only that
`--workspace` is wrong might still not know which of the other two to reach for. `q0_test_gate_run.txt`
runs the false-green form alongside the working one so both are on the record in one place.

I did not re-audit the test bodies. `114` section 0.2 records what I read then and nothing in this file
touches a new crate.

---

## 1. What this reply concludes, stated first

> **On the live disagreement: `115` is right against my framing, and its own replacement is one notch
> coarse by its own measurement.** Deleting the `wrap` dimension discards a region where the conditions
> hold against the right check, which is what `115:182-187` says and I accept it. But the certificate at
> `wrap` is **condition (a) alone**, not the conjunction: `115` F115-2 measures that and does not carry
> it into F115-1, and q1 measures what the conjunction costs there, 3072 cells at `uW3/wrap` and 384 at
> `iW3/wrap`, **every one of which the root check already decides exactly**. So neither repair. Two
> arms, each with its own check **and its own condition set**.
>
> **The condition set follows the character, which is the half nobody had.** `115` ran (a)-alone at
> `wrap` and got zero violations. `114` ran (a)-alone at `sat` and got 234 and 20. Neither ran the other
> half, so neither could see that condition (b) is load-bearing at `sat` and vacuous at `wrap`, for the
> same reason the check changes: under a homomorphism no intermediate node's overflow matters, and (b)
> exists only to forbid an intermediate overflow being masked.
>
> **`116` F116-4 survives a different search and two of its three hypotheses are load-bearing.**
> Reproduced on realisation-shaped maps with monotonicity quantified over **every** total order rather
> than the natural one, and `116`'s own figures reproduced exactly on my implementation, 3/10/2/0 and
> 6/105/3/0. Ablated: **multiplication is not load-bearing**, so the theorem is really about the
> additive group and is wider than stated; **addition is**, with a witness; and the domain must be wide
> enough to contain `[0, |V|]`, with a witness.
>
> **The fraction width splits arm W0 exactly as `116` split arm W1, and the cut rule with it.** Arm W0
> is unconditional on terms without a multiplication at every `F` and fails on terms with one at
> `F > 0`. Three of my hypotheses about what the cut rule then needs were refuted in a row, and the
> witnesses say why: a reduction has two parts and **each has its own locality condition**. The grid
> part must be applied at the result of every node that leaves the grid; the range part at the operands
> of every node the map is not a homomorphism for. That one rule covers `114` p11's `F = 0` non-ring
> case and the `F > 0` case together, and a third setting nothing had run.
>
> **And `114` section 6.4 was under-determined, which `116` F116-6 is what shows.** The shipped kernel's
> guard makes the accumulator's map the identity on the reachable range, and the identity has both
> characters, so the kernel does not depend on the carrier being wrapping at all. Measured: under the
> guard the safe branch gives the same answer with a wrapping and with a saturating accumulator, 4000 of
> 4000. And with a **saturating** accumulator the required width collapses from `W + ceil_log2(arity)`
> to `W`, at every arity swept, which at `W = 2` and arity 32 is 2 bits against 7.

The last one is the most consumer-facing thing in this exchange, and it falls out of putting `116`
F116-5 next to a kernel that was already in the tree. I did not know how long it had been there and did
not check, which is why that sentence says nothing about its age.

---

## 2. The live disagreement, settled, and it is settled against my framing

### 2.1 What each repair is, stated precisely, because they are not the same object

`111` F111-15 is a claim about the **per-node corner rule**: a structural predicate over the term and its
declared extents is sufficient for that rule's exactness. `114` F114-3 refuted it at `wrap`, with 28 and
16 violations. `115` reproduced both figures exactly on `111`'s own committed implementation and conceded
at `115:120-121`: "F111-15 claimed soundness at `overflow policy in {sat, wrap}` and it does not hold at
`wrap`."

So the finding is not in dispute. What is in dispute is the shape of the replacement.

**My repair**, at `114` section 3.1: restate F111-15 at `overflow policy = sat`, where it is sound over a
far larger region than it originally claimed.

**`115`'s repair**, at `115:59-63`: keep both policies and aim the predicate at the check the overflow
policy selects. Its F115-1 measures zero violations at all four primitives with the conditions untouched.
Its stated ground for declining mine, at `115:182-187`: deleting the dimension "discards the wrap half of
the region for no reason: the conditions hold there, against the right check, measured. Under I13 the
right shape is two arms over one axis rather than one arm with a dimension deleted."

### 2.2 Reproduced first, then the answer

`q1_output.txt`, on my own instrument over the same 96 terms:

```
  primitive  check      cells  consv  unsnd     (a) and (b)         (a) alone         always [C1]
                                             fires / violations   fires / violations  fires / violations
  uW3/sat    per-node   13032    703      0   6336 / 0            9408 / 234          13032 / 703
  iW3/sat    per-node    2148     93      0    816 / 0            1200 / 20            2148 / 93
  uW3/wrap   root       13032    645      0   6336 / 0            9408 / 0            13032 / 645
  iW3/wrap   root        2148     95      0    816 / 0            1200 / 0             2148 / 95
```

**F115-1 reproduces**: zero violations at all four with the conjunction against the policy-selected
check. **F115-2 reproduces**: 9408 and 1200 fires at zero violations with (a) alone at `wrap`. Three
controls fire: the always-fire row equals the conservative count on every row, a condition (b) forced
true reproduces the (a)-only column exactly, and the root check is unsound on 38 and 34 cells at `sat`
against zero at `wrap`, which is `114` p2's own column arriving on a third instrument.

**So `115` is right and my framing was too coarse.** The conditions were never the defect. Restricting
the finding to `sat` throws away a region where they hold, and `114` section 3.1's sentence about the
finding getting "wider on three dimensions and narrower on one" invited exactly that reading. Under I13
the replacement is arms, and `115` names the right axis.

### 2.3 And `115`'s replacement is one notch coarse, by its own F115-2

The right-hand columns of that table are the half neither file had. `115` ran (a) alone at `wrap` and
found it clean. `114` ran (a) alone at `sat` and found 234 and 20 violations. Put side by side:

```
  primitive  check       (a) alone: fires   violations
  uW3/sat    per-node                9408          234   <- (b) IS load-bearing here
  iW3/sat    per-node                1200           20   <- (b) IS load-bearing here
  uW3/wrap   root                    9408            0   <- (b) is not load-bearing here
  iW3/wrap   root                    1200            0   <- (b) is not load-bearing here
```

**Condition (b) is load-bearing at `sat` and vacuous at `wrap`**, and the reason is the same mechanism
that changes the check. Condition (b) forbids an intermediate node's overflow being masked downstream.
Under a homomorphism no intermediate node's overflow reaches the answer at all, so there is nothing for
(b) to forbid. `115` says exactly this in prose at `115:146-148` and then states F115-1 with the
conjunction anyway.

**What the extra condition costs, measured.** q1's P4 and P5:

```
  primitive   (a)+(b) fires   (a) fires   declined   of those, root check exact   root conservative
  uW3/wrap             6336        9408       3072                         3072                   0
  iW3/wrap              816        1200        384                          384                   0
```

3072 and 384 cells where the conjunction declines to certify, and on **every one of them** the root
check's verdict already equals the arms' verdict. So the decline loses a licence and buys nothing. In
the currency `114` section 7 established, each declined cell is a cell where a design instantiates the
expensive carrier it did not need.

### 2.4 So the answer to the question the dispatch asks

**Neither repair as stated. `115`'s shape with `115`'s own F115-2 folded into it.**

> Two arms over the map's algebraic character. Where the map is a ring homomorphism for every operation
> in the term, the check is the **root's** interval and the certificate is **condition (a)**. Where it is
> not, the check is **every node** and the certificate is **condition (a) and condition (b)**.

That is I13's shape, it is `115`'s argument taken one step further rather than contradicted, and it is
what q1's table says without any of it being new physics. `115` had the measurement that forces it and
placed it in a finding of its own rather than in the arm.

**What would decide this against me.** A `wrap` cell where condition (a) holds, the root check refuses
and the arms agree. `115` looked and found none over 13032 and 2148 cells; q1 looked and found none over
the same; `114` F114-6 found none over 9408 and 1200. Three instruments, one region, `F = 0`, ring
operations only. Section 5 is where that region ends.

---

## 3. `116` F116-4 checked, and its hypotheses ablated

F116-4 is the root of the convergence in section 4, so it gets checked rather than accepted, and the
hypotheses get attacked because neither file says which of them are load-bearing.

### 3.1 The argument, re-derived rather than quoted

Let `V` be finite with `|V| >= 2` and `R : Z -> V` surjective with `R(a op b) = R(R(a) op R(b))`. The
relation `R(a) = R(b)` is then a congruence, `V` carries the induced structure, and a surjective quotient
of `Z` is `Z/nZ` with `n = |V|`. If some total order on `V` makes `R` non-decreasing, then `R(0) = R(n)`
because `0` and `n` are congruent, a non-decreasing map agreeing at the endpoints of `[0, n]` is constant
on it, that interval carries a complete residue system, and `R` factors through the residues, so `R` is
constant and `|V| = 1`.

The derivation goes through. `116:315-325` states it and it is correct as stated.

### 3.2 A different search, and `116`'s own figures reproduced

`q2_output.txt`. Two things differ from `116`'s probe deliberately: the maps searched fix `V` pointwise,
which is what a realisation map does, and monotonicity is tested against **every** total order on `V`
rather than the natural one, which is what `116:322` states as the hypothesis and its probe does not
test.

```
  {+, *}, realisation-shaped   |V|=2 win=13,17,21   both AND non-constant  0, 0, 0
  {+, *}, realisation-shaped   |V|=3 win=13         both AND non-constant  0
```

and, dropping the pointwise restriction so the constants can appear, which is the control `116`'s space
has and mine does not:

```
  free maps, natural order   |V|=2, 512 maps      hom 3  monotone  10  both 2  non-constant 0
  free maps, natural order   |V|=3, 1594323 maps  hom 6  monotone 105  both 3  non-constant 0
  free maps, SOME order      |V|=2, 512 maps      hom 3  monotone  18  both 2  non-constant 0
  free maps, SOME order      |V|=3, 1594323 maps  hom 6  monotone 471  both 3  non-constant 0
```

**The natural-order rows are `116`'s figures exactly**, 3/10/2/0 and 6/105/3/0, arrived at on a different
implementation. The some-order rows find 18 against 10 and 471 against 105 monotone maps, so the more
general hypothesis is strictly weaker and still admits no non-constant map passing both. `116`'s
restriction to one order was not hiding anything, and that is now checked rather than assumed.

**One limitation of my own space, named because it matters.** A map fixing `V` pointwise cannot be
constant when `|V| >= 2`, so the exception the theorem names is excluded by construction and the "both"
column is zero for a reason unrelated to the theorem. That is why the free-map rows are there. The
control that the search can report a non-constant map passing both is section 3.3's ablations, which do.

### 3.3 Which hypotheses are load-bearing

**Multiplication is not.** Dropping it and keeping only `{+}`: zero non-constant maps pass both, at
`|V| = 2` over windows of 13 and 21 and at `|V| = 3` over 13. The proof only ever uses addition to get
the quotient and the periodicity, and multiplication rides along. So the theorem is **wider than stated**:
no map onto a finite value set is both an **additive** homomorphism and monotone, except a constant one.

That widening is not cosmetic and section 5 uses it. `116` F116-7 establishes that at `F > 0` under wrap
the map keeps the homomorphism only for addition and subtraction. Under F116-4 as stated, that map is
outside the theorem's hypothesis and nothing follows. Under the widened form it is inside, so **the trade
between the two licence families holds at every fraction width**, not only at `F = 0`. q3 confirms the
consequence directly: the monotone column is `False` on every wrapping row at `F in {0, 1, 2}`.

**Addition is load-bearing**, with a witness. Dropping it and keeping only `{*}` over a non-negative
window:

```
  {*} alone, |V|=2, window 0..8   witness: 0->0, 1->1, 2->1, ..., 8->1
  {*} alone, |V|=3, window 0..10  witness: 0->0, 1->1, 2->2, ..., 10->2
```

Both are multiplicative homomorphisms, monotone, and non-constant. Zero is absorbing under
multiplication, so the identity holds, and on a non-negative window the map is non-decreasing. So the
theorem is genuinely about the additive structure and would be false for a multiplicative-only signature.
This is also the control that makes every zero above a result: the search demonstrably can report a
non-constant map passing both.

**The domain's width is load-bearing.** With the window narrowed to `V` itself or to `V` plus one point,
the identity passes both and is non-constant, at `|V| = 3` and `|V| = 4`. That is not a defect in the
theorem: it says the hypothesis is a map defined on enough of `Z`, which a realisation map is. It is
worth pinning because a later probe built on a narrow window would report the theorem false and be
measuring nothing.

**Verdict.** F116-4 holds, is wider than it claims on one hypothesis, and its other two hypotheses each
have a witness showing they cannot be dropped. I would carry it as the root of section 4.

---

## 4. The convergence: one mechanism, and where each file sits in it

The dispatch asks whether the three statements are one mechanism or three that resemble each other. They
are one, and the accounting of who established what matters as much as the statement, so both are below.

### 4.1 The chain, with its provenance

**The root is `116` F116-4**, widened by section 3.3: no map onto a finite value set is both an additive
homomorphism and monotone, except a constant one. Structural, hypotheses ablated, and it needs no width
transfer argument for the reason section 6 sets out.

**`114` F114-1 is an instance of it, measured before it was stated.** Wrapping is a homomorphism and
saturating is not; saturating is monotone and wrapping is not. That is one row of the theorem's table,
found by measuring the two policies the design happened to have.

**`114` F114-6 and `116` F116-5 are the two licence consequences**, one per family. The homomorphism
licenses a discharge check that reads only a term's root. Monotonicity licenses `110` F12's interval
construction and whatever else rests on the map preserving order. Each was found without its author
knowing the other family existed.

**`115` F115-1 is the design instruction**: a certificate must name which check it certifies, because
which check is sound follows from the character. `116:353` says this in as many words and says the two
files were written in parallel without reading each other.

**And q1 adds the fourth consequence**, which is the one none of the three had: the **condition set**
follows the character too, not only the check. Condition (b) is load-bearing at `sat` and vacuous at
`wrap`, for the same reason the check changes.

### 4.2 The independence accounting, stated plainly because it is easy to inflate

**This is not a TWO EXPERTS convergence** by the panel's own definition at `106:280-282`, which requires
two independent derivations each before reading the other. `116` read `114` first and says so; its F116-4
is a **generalisation** of my F114-1 rather than an independent instance of it. `115` also read `114`
first. So the homomorphism claim has one derivation and two reproductions, on three instruments, and the
reproductions are worth having for exactly what they are.

**What is independent:** `115`'s design instruction and `116`'s mechanism, of each other. `116:353`
states that neither file read the other, and their two consequences land on different families. That is a
genuine two-instance agreement about the mechanism's shape, arrived at from opposite sides.

**What is mine and unreproduced:** everything in q1 through q6. Each is one instrument and one author,
and section 9's findings say so.

### 4.3 The statement, offered

Written to be taken close to verbatim if it survives, and to compose with `108` section 7 and `112`
section 9 rather than replace either.

> A realisation map has an **algebraic character**, and the character is what an overflow policy selects.
> A map onto a finite value set may be an additive homomorphism, or order-preserving, or neither, and
> never both. That is forced rather than observed, and it holds however many policies a design offers.
>
> Each character licenses a family of arms. The **homomorphism** licenses deferring a reduction: past an
> operation it is a homomorphism for, and to the root of a term all of whose operations it is a
> homomorphism for. **Order preservation** licenses constructions and rewrites that read the map's order,
> of which the interval construction is one member and the family is not enumerated.
>
> So a design does not choose a discharge check. It chooses a character, and the check follows, and so do
> the conditions under which a refusal by that check is honest.
>
> The character is a joint fact about the policy, the **operation**, and the **fraction width**, not about
> the policy alone. A wrapping map is a homomorphism for addition and subtraction at every fraction width,
> and for multiplication only at zero fraction width or on operands declared to the unit grid.
>
> A **declared refinement** is the only mechanism that escapes the choice. On a discharged extent the map
> is the identity, which has both characters, so both families are available at once. No policy can buy
> that and a declaration can.

**Permanence.** Every sentence survives a rewrite in another language or decade. None names a container, a
width, a marker, a type parameter, a crate or a count of strategies.

**Equivalence.** Three teams implementing this produce units that behave the same on what matters: the
discharge check and its certificate are both derived from the character rather than chosen; a design at a
homomorphic policy never pays for an order-based construction and knows it; a design at an
order-preserving policy never defers a reduction and knows why; and a discharged declaration is the one
thing that lifts both restrictions. They differ in how the character is spelled, how many policies ship,
and whether the family membership is a trait bound or a const.

**Where it is weaker than I would like.** The order-based family has exactly one known member,
`110` F12's interval construction, and `116` section 8 names enumerating it as a dispatch nobody has run.
Until that list exists, "order preservation licenses a family" is a claim with one instance in it.

---

## 5. The arm set as it now stands

Per I13. Every dimension listed with a range or a fixed value was established across it; a dimension
absent does not hold anywhere that dimension is present; unmeasured and unknown go unstated. Conventions
stated once: everything enumerative ran on one thread and carries `threads = 1`; the model sweeps are
exact rational arithmetic no instruction selection can move, so they carry `target features any` with that
as the argument; every enumerative result is at `W <= 8` with **no transfer argument** to any real width,
so every predicate lists its width as a fixed set. Section 6 is the one exemption and it is argued rather
than assumed.

`116` split my arm W1 into three. The same mechanism splits arm W0 and the cut rule, which nothing had
checked, so the set below is larger than either file's.

### The deferral arms, which need the homomorphism

**W0a. The intermediate reduction is free, over the homomorphic operations.** Computing without
intermediate reduction and reducing once at the root equals reducing at every node, for every declaration
and every value tuple, on terms whose every operation the map is a homomorphism for.

`W = 4 logical, F in {0, 1, 2}, carrier width in {3, 4, 5, 8}, signedness in {unsigned, signed}, overflow
policy = wrap, carrier overflow policy = wrap, rounding = trunc, radix = 2, operations in {add, sub} at
F > 0 and {add, sub, mul} at F = 0, term shapes = every term at 2 and 3 leaf slots over that signature,
arity in {2, 3}, declarations = one-sided [0, b], threads = 1, target features any`. `114` p8, q3.

**W0b. And it fails the moment a multiplication appears at a nonzero fraction width.** 72, 69 and 35
differing cells of 156 at `F in {1, 2}`, against 0 at `F = 0`, with 21 to 106 cells where the exact result
leaves the range so the comparison is live. Same predicate with `operations in {add, sub, mul}` and
`F in {1, 2}`. `q3_output.txt`.

**W1a, W1b, W1c** are `116`'s, at `116:453-464`, and I reproduce F116-7 rather than restate them. One
note on W1b in section 7.3.

### The locality rule, which is what replaces the cut rule

**Q1. A reduction has two parts and each has its own locality condition.** The **grid** part must be
applied at the **result** of every node whose exact result can leave the grid. The **range** part must be
applied at the **operands** of every node the map is not a homomorphism for. Everywhere else both may be
deferred to the root, and the result equals reducing at every node.

`W = 4, F in {0, 1, 2}, signedness in {unsigned, signed}, overflow policy in {wrap, sat}, rounding =
trunc, radix = 2, operations in {add, sub, mul} and in {add, sub, mul, min}, term shapes = every term at 2
and 3 leaf slots over the signature in play, arity in {2, 3}, declarations = one-sided [0, b] sampled 3
per term, threads = 1, target features any`. `q5_output.txt`, zero failures on every row of every setting.

This subsumes `114` F114-19, which is the same rule at `F = 0` where the grid condition is vacuous, and it
covers a setting nothing had run, `F > 0` over `{add, sub, mul, min}`.

**Q2. What it saves is a function of the term's shape, and it is not always positive.** Counted as
reductions performed on one evaluation, at a logical width of 8:

```
  shape                                 nodes    F=0 wrap    F=1 wrap     F=0 sat
  fold of 16 adds                          15      15 / 1      15 / 1     15 / 15
  fold of 8 adds, one mul at the end        8       8 / 1       8 / 3       8 / 8
  fold of 8 adds, clamped at the end        8       8 / 2       8 / 2       8 / 8
  fold of 4 muls                            3       3 / 1       3 / 6       3 / 3
```

`term shapes = the six enumerated in the probe, W = 8 logical, F in {0, 1}, overflow policy in {wrap,
sat}, operations in {add, sub, mul, min}, threads any, target features any`. `q5_output.txt`. A count of
reductions, not a duration; **what a reduction costs is unpriced** and no bench ran.

Two things follow. The saving is proportional to the number of homomorphic nodes, so it is large on a fold
and negative on a multiplication-dense term at `F > 0`. And at `sat` it is exactly 1.00, because no
operation is a homomorphism, the range condition fires at every node, and the rule **degenerates to the
general arm**. That last one refuted my own prediction that the rule would fail at `sat`; it does not
fail, it becomes the thing it was deferring from.

### The certificate arms, from section 2

**S1a. Under a homomorphism the certificate is condition (a) alone, against the root check.**
**S1b. Otherwise it is condition (a) and condition (b), against the per-node check.** Predicates as
`114` arm S1 with the policy split, evidence in `q1_output.txt` and reproduced from `115` F115-1 and
F115-2.

**S2** is unchanged, and `116` section 3.3 arrives at it from its own mechanism rather than from my file,
which makes it a second instance of the repair rather than a concession to it. I take that gladly.

---

## 6. The transfer exemption, and whether it generalises

`116` F116-4 claims one specific exemption from the width transfer problem: it "quantifies over
finiteness rather than over size", so unlike everything else in that file it needs no transfer argument.
The dispatch asks whether that generalises or is peculiar.

**It generalises, and the general form is a two-part test rather than a property of structural claims.**

`unstable-features.md` states the problem: a property checked at a model width may fail at a real one
because the property's quantifier range depends on the width, and closing the `specialization` and
`TypeId` doors is necessary and not sufficient. The exemption is not that a claim is "structural". It is:

1. **The claim's statement does not mention a width.** Read the sentence and look for one. F116-4's says
   `V` finite with `|V| >= 2`, and every real width instantiates that.
2. **The argument establishing it does not mention a width either.** A width-free sentence proved by an
   exhaustive check at `W = 3` is still a `W = 3` result wearing a general sentence.

Both halves are needed and the second is where it goes wrong. `116` satisfies both: its statement is
width-free and its argument is the congruence-and-periodicity derivation, with the search presented as a
control on the argument rather than as its evidence. That distinction is `116`'s own and it is the thing
that earns the exemption.

**Which of this panel's claims pass the test.** Three that I can see. F116-4 itself. `114` arm S1's
exactness argument, whose two steps quantify over terms and declarations and never mention the container's
size, only that there is one. `114` arm W0's ring argument, likewise. `116`'s W1b is a fourth candidate:
its prose says `F` any while its block predicate says `F in {0, 1, 2}`, and the gap is exactly this test
applying, because the argument that addition and subtraction never enter the rounding region is
`F`-free. **I would state W1b on the argument rather than on the sweep**, and say which, rather than
letting the prose and the predicate disagree.

**Which fail it.** Every count in every file, including all of mine. "The predicate declines 3072 cells"
is width-bound in its statement. So is every domination count, every violation count and every licence
count in `114`, `115`, `116` and this file.

**And the trap the test catches**, which is worth naming because it is the shape a later expert will hit:
a claim can be width-free in its wording and width-bound in its proof, and it will read as exempt. The
remedy is to say which of the two forms of evidence a claim rests on, at the point of the claim.

---

## 7. `114` sections 8 and 12, revised

Both were written before either reply and are candidates for revision rather than statements.

### 7.1 What section 8's canon statement gains

Section 4.3 above replaces its first two clauses. What the old version said, at `114` section 8, was that
a realisation map is a homomorphism for a set of operations, that where it is the intermediate reductions
carry no information, and that where it is not every node must be checked. All true and all incomplete in
three ways the replies found:

- The **duality**, which says the choice is between two families and never both. That is `116`'s and it
  belongs in the canon sentence because a design reading only my version would think the homomorphism
  were free.
- The **fraction width and the operation**, which make the character a joint fact rather than a property
  of the policy. That is `116` F116-7's and I reproduce it.
- The **condition set**, which follows the character alongside the check. That is q1's.

Two further clauses of `114` section 8 survive unchanged: the per-node intersection of sound rules, and
the bilinear transformer formula, which `116` F116-10 reproduces from the structure constants alone and
declares its own table superseded by. I have nothing to add to either.

One clause of `114` section 8 I would now write differently: **"the cost of a propagation rule is paid
where its carrier is instantiated"** is right and `115` F115-5 supplies what it was missing. My section 7
established that the selection must happen before instantiation and did not ask what is available there.
`115` answers it: the certificate reads intervals and leaf identity, both of which are properties of the
syntax the term type is generated from, so the verdict is available to whatever writes that type. That is
the half that makes the prohibition actionable and it is `115`'s.

### 7.2 What section 12's rung accounting gains

`114` section 12 listed the homomorphism result as "one expert, and each is a queue entry", and named the
thing it most wanted a second read on: whether the overflow policy is the right axis to carry it.

**It has three reads now and the answer is that the policy is not the whole axis.** `115` s1 is the second
read from the certificate side and `116` p1 the third from the sentence side, and `116` F116-7 then shows
the axis is the policy **and the operation and the fraction width** together. So the thing I asked for a
second read on came back qualified rather than confirmed, which is the more useful outcome.

**And the worry section 12 recorded turns out to be answered rather than open.** I wrote that if a design
ends up with a policy that is neither a homomorphism nor a saturation, the two arms partition nothing.
`116` F116-5 measures exactly that: a flush-to-zero map and a reflecting map have **neither** property.
So the arms do not partition the policy space. They cover two regions of at least three, and the third
gets no algebraic licence at all. That is a closed question and `116` closed it, deliberately, by putting
two extra policies in its table so the result would not be two points.

### 7.3 What section 5.4 predicted against itself, and what arrived

`114` section 5.4 named three things that would decide the file against itself. Two have arrived.

**`F > 0` changing the propagation question qualitatively rather than adding a second quantity.** I marked
that as an expectation and not a result, and expected it to be additive. `116` F116-7 shows it is not
additive for multiplication, and q3 reproduces it. So that prediction of mine is refuted and the arm set
in section 5 is the repair.

**A real consumer's terms being DAGs rather than trees**, which I named as the assumption I was least
comfortable with. Still untouched, and `115` section 8 names it as what would decide its own section 5
against itself, so two files now depend on it and neither has looked.

**A transfer argument to real widths.** Still absent for everything enumerative, and section 6 above says
precisely which claims do not need one.

---

## 8. Two corrections, one of my own and one of `115`'s

### 8.1 `114` section 6.4 was under-determined, and `116` F116-6 is what shows it

I claimed the shipped `warm-clamp` kernel "is arm W1 already". The source reading is right and `115`
section 0.2 verified it independently. The **attribution** is wrong, or rather under-determined, and
`116` F116-6 is the piece that shows why: on a discharged extent the map is the identity, and the identity
has both characters.

The kernel's guard is `accumulator_bits_needed(W, ARITY) <= A::BITS` with
`accumulator_bits_needed(w, arity) = w + ceil_log2(arity)`
(`warm-clamp-shared/src/lib.rs:291` and `:158-160`). Under it, no accumulator addition can leave the
carrier's range, so the carrier's map is never applied to anything outside it, so it is the identity there.
Arm W1 needs the carrier to be **wrapping**. The identity needs nothing of the carrier at all.

**Testable, and tested.** `q6_output.txt`, 4000 random chunks per row:

```
    W  arity  needed   guard met: wrap / sat / agree   guard short by 1: wrap / sat
    3      4       5       0 / 0    / 4000 of 4000            1432 / 0
    4     16       8       0 / 0    / 4000 of 4000             962 / 0
    6      4       8       0 / 0    / 4000 of 4000            1801 / 0
```

**Under the guard the safe branch gives the same answer with a wrapping accumulator and with a saturating
one, on every chunk.** So the kernel does not read the carrier's overflow policy, and a design reading
section 6.4 as "this is arm W1" would think it had to keep the accumulator wrapping. The control fires:
one bit short of the guard, the wrapping carrier disagrees on 962 to 1801 chunks.

**And the saturating column is the finding underneath.** One bit short, the **saturating** carrier is
still correct, on every row. Searching for the smallest correct accumulator width:

```
    W  arity  formula   smallest correct, wrap   smallest correct, sat
    3      8        6                        6                       3
    4     16        8                        8                       4
    2     32        7                        7                       2
```

The shipped formula computes the width a **wrapping** accumulator needs, and it is exact for that: the
search finds the same number at every row. With a **saturating** accumulator the required width is the
logical width itself, at every arity, because saturation and the final clamp agree above the logical
limit. At `W = 2` and arity 32 that is 2 bits against 7.

**Which is section 4.3's statement arriving at a shipped kernel.** A clamping fold is an order-based
operation. A saturating carrier is in the order-preserving family and needs no headroom. A wrapping
carrier is in the homomorphism family, is not monotone, and needs exactly enough headroom to stay out of
the non-monotone region, which is what `w + ceil_log2(arity)` computes. **The accumulator width question
exists because the accumulator wraps.**

`warm-clamp`'s own module documentation names accumulator width as the axis it exists to measure, so
whether the narrower saturating accumulator is faster is exactly what that harness would price. **It is
unpriced here** and I did not run it. That is the first item on section 10's list.

### 8.2 One factual correction to `115`, which makes its own finding better

`115` F115-4 reports that a trait projection selecting the rule type does not escape the expensive
carrier's obligation, and calls it "a route `114` did not try", at `115:215` and `115:349-355`.

**I did try it.** `114_probes/p9` line 244 defines a `selection-assoc` variant, lines 251 to 256 build it
as `impl Pick for Cond<true>` / `Cond<false>` with the site reading
`<<Cond<PREDICATE_FIRES> as Pick>::Arm as Chk>::OK`, which is the same construction. `114` reports it
failing at `L = 64` in the table at `114:884`, in the first-refusal list at `114:897`, in the prose at
`114:913` and in F114-17 at `114:1124`.

The correction makes `115`'s result **stronger** rather than weaker. As stated it is a first instance of a
route nobody had tried. In fact it is a **second independent instance** on an independently built tower
with a different recursion limit, of a route that had been tried, which is worth more. I would restate
F115-4 that way and take the reproduction gratefully.

---

## 9. Findings, each with its predicate

Conventions as section 5.

**F118-1. Condition (b) is load-bearing under saturation and vacuous under wrapping, and the condition
set therefore follows the map's character exactly as the check does.** Condition (a) alone against the
policy-selected check produces 234 and 20 violations at `uW3/sat` and `iW3/sat`, and 0 at `uW3/wrap` and
`iW3/wrap` over 9408 and 1200 firing cells. `W = 3, F = 0, signedness in {unsigned, signed}, overflow
policy in {sat, wrap}, rounding = trunc, radix = 2, operations in {add, sub, mul}, term shapes = every
term at 2 and 3 leaf slots with every leaf identification, arity in {2, 3}, declarations = one-sided [0,
b] exhaustive, discharge check = root under wrap and per node under sat, threads = 1, target features
any`. `q1_output.txt`. Three controls fire: an always-firing certificate produces violations equal to the
conservative count on every row, a condition (b) forced true reproduces the (a)-only column exactly, and
the root check is unsound on 38 and 34 cells at `sat` and 0 at `wrap`.

**F118-2. `115` F115-1's conjunction declines 3072 and 384 cells at wrap on which the policy-selected
check is already exact.** Every declined cell has the root check's verdict equal to the arms' verdict, so
the decline loses a licence and buys nothing. Same predicate as F118-1 with `overflow policy = wrap`.
`q1_output.txt`.

**F118-3. `116` F116-4 reproduces on a different search with monotonicity quantified over every total
order on the value set.** Zero non-constant maps pass both over realisation-shaped maps at `|V| = 2` with
windows of 13, 17 and 21 and at `|V| = 3` with a window of 13; and `116`'s own free-map figures reproduce
exactly, 3 homomorphic, 10 monotone, 2 both, 0 non-constant at `|V| = 2` over 512 maps, and 6, 105, 3, 0
at `|V| = 3` over 1594323. The some-order test finds 18 and 471 monotone maps against 10 and 105 and still
zero non-constant passing both. `value set size in {2, 3}, window in 9 to 21 consecutive integers,
operations in {add, mul}, maps in {every map onto V, every map fixing V pointwise}, monotonicity under
{the natural order, every total order}, threads = 1, target features any`. `q2_output.txt`.

**F118-4. Multiplication is not load-bearing in F116-4's hypothesis, so the theorem is about the additive
group and is wider than stated.** Dropping multiplication leaves zero non-constant maps passing both, at
`|V| = 2` over windows of 13 and 21 and at `|V| = 3` over 13. Same predicate as F118-3 with `operations =
{add}`. `q2_output.txt`.

**F118-5. Addition is load-bearing, with witnesses.** Over `{*}` alone on a non-negative window, the map
sending 0 to 0 and everything else to the top element is a multiplicative homomorphism, monotone and
non-constant, at `|V| = 2` over `0..8` and `|V| = 3` over `0..10`. Same predicate with `operations =
{mul}, window non-negative`. `q2_output.txt`. This is also the control that makes every zero in F118-3
and F118-4 a result rather than a dead branch.

**F118-6. The domain's width is load-bearing.** With the window narrowed to the value set or to it plus
one point, the identity passes both and is non-constant, at `|V| in {3, 4}`. Same predicate with `window
in {|V|, |V| + 1}`. `q2_output.txt`.

**F118-7. `116` F116-7 reproduces, and the map is not monotone at any fraction width under wrap.** Zero
homomorphism failures of 2304 for add and sub at `F in {0, 1, 2}` under wrap; 1152 and 1600 of 2304 for
mul at `F in {1, 2}` and zero at `F = 0`; monotone False on every wrap row and True on every sat row.
`W = 4, F in {0, 1, 2}, signedness in {unsigned, signed}, overflow policy in {wrap, sat}, rounding =
trunc, radix = 2, operations in {add, sub, mul}, ambient span = three times the container, threads = 1,
target features any`. `q3_output.txt`. With F118-4 this puts the licence-family trade at every fraction
width rather than only at zero.

**F118-8. Arm W0 splits along the same operation boundary as arm W1.** Dropping every intermediate
reduction equals reducing at every node on 0 of 132 cells differing for terms without a multiplication at
`F in {0, 1, 2}`, with 43 to 73 cells where the exact result leaves the range; and differs on 72, 69 and
35 of 156 cells for terms with one at `F in {1, 2}`, against 0 at `F = 0`. `W in {3, 4}, F in {0, 1, 2},
signedness in {unsigned, signed}, overflow policy in {wrap, sat}, rounding = trunc, radix = 2, operations
in {add, sub, mul}, term shapes = every term at 2 and 3 leaf slots, arity in {2, 3}, declarations =
one-sided [0, b] sampled 3 per term, threads = 1, target features any`. `q3_output.txt`.

**F118-9. Cutting at the non-homomorphic operation's operands and at its result fail on disjoint sets of
cells, so they are two repairs and not one.** 660 cells where only the operand cut fails, 648 where only
the result cut fails, and 0 where both fail, over 5026 cells. `W = 4, F = 1, signedness = unsigned,
overflow policy = wrap, rounding = trunc, radix = 2, operations in {add, sub, mul}, term shapes = every
term at 2 and 3 leaf slots containing a multiplication, arity in {2, 3}, declarations = one-sided [0, b]
with b in 0..6 exhaustive, threads = 1, target features any`. `q4_output.txt`. At `F = 0` both sets are
empty over 2080 cells, which is the control.

**F118-10. The two mechanisms, with a witness each, and both need a subtraction.** Only the operand cut
fails on `x - (x * x)` at `x = 1/2` with grid step `1/2`, where the product `1/4` is off the grid and
quantisation does not commute with the subtraction above it. Only the result cut fails on `x * (x - y)` at
`x = 1/2, y = 1`, where the inner difference `-1/2` is out of range and the range reduction cannot be
deferred through a non-homomorphism. Neither witness exists among the multiplication terms without a
subtraction, which **refutes a prediction of mine** recorded in that probe's header. Same predicate as
F118-9. `q4_output.txt`.

**F118-11. One rule with two locality conditions covers both settings and a third nothing had run.** The
grid part applied at the result of every node that leaves the grid, and the range part at the operands of
every node the map is not a homomorphism for, equals reducing at every node on zero failing cells across
`F = 0` over `{add, sub, mul, min}`, `F in {1, 2}` over `{add, sub, mul}`, and `F in {1, 2}` over
`{add, sub, mul, min}`, at 73 to 504 cells per row with 6 to 268 cells where the exact result leaves the
range and 24 to 189 where an intermediate leaves the grid. `W in {3, 4}, F in {0, 1, 2}, signedness in
{unsigned, signed}, overflow policy in {wrap, sat}, rounding = trunc, radix = 2, operations in {add, sub,
mul} and {add, sub, mul, min}, term shapes = every term at 2 and 3 leaf slots over the signature in play,
arity in {2, 3}, declarations = one-sided [0, b] sampled 3 per term, threads = 1, target features any`.
`q5_output.txt`. Controls: dropping the grid condition breaks the `F > 0` rows on 5 to 29 cells and
correctly breaks nothing at `F = 0`; dropping the range condition breaks 2 to 91 cells on every row where
a non-homomorphic operation is present.

**F118-12. The rule degenerates to the general arm under saturation rather than failing, which refutes a
prediction of mine.** At `F = 0` under sat the reduction count is 96807 against the general arm's 96807,
exactly 1.00. Same predicate as F118-11 with `overflow policy = sat`. `q5_output.txt`.

**F118-13. What the rule saves is a function of the term's shape and is negative on multiplication-dense
terms at a nonzero fraction width.** On a fold of 16 adds, 1 reduction against 15 at `F in {0, 1}` under
wrap; on a fold of 8 adds with one multiplication at the end, 1 at `F = 0` and 3 at `F = 1` against 8; on
a fold of 4 multiplications, 1 at `F = 0` and **6** at `F = 1` against 3; and 15 against 15, 8 against 8
and 3 against 3 at `sat`. `W = 8 logical, F in {0, 1}, signedness = unsigned, overflow policy in {wrap,
sat}, rounding = trunc, radix = 2, operations in {add, sub, mul, min}, term shapes = the six enumerated in
the probe, threads any, target features any`. `q5_output.txt`. A count of reductions, not a duration.

**F118-14. Under the shipped `warm-clamp` guard the accumulator's overflow policy is free.** The safe
branch agrees with the oracle on 0 of 4000 chunks for a wrapping accumulator and 0 for a saturating one,
and the two accumulators agree with each other on 4000 of 4000, at six width and arity pairs. `W in {2, 3,
4, 5, 6}, arity in {4, 8, 16, 32}, accumulator width = w + ceil_log2(arity), accumulator overflow policy
in {wrap, sat}, logical overflow = clamp at the logical maximum, inputs = 4000 uniform random chunks per
row, threads = 1, target features any`. `q6_output.txt`. Control: one bit below the guard the wrapping
accumulator disagrees on 962 to 1801 chunks.

**F118-15. The shipped guard is exact for a wrapping accumulator and conservative by `ceil_log2(arity)`
bits for a saturating one.** The smallest accumulator width at which the safe branch is correct equals
`w + ceil_log2(arity)` for a wrapping accumulator at every row, and equals `w` for a saturating one, so at
`W = 2` and arity 32 the widths are 7 and 2. Same predicate as F118-14 with `accumulator width searched
from 1 upward`. `q6_output.txt`. **What the narrower accumulator is worth is unpriced**; `warm-clamp` is
the harness that would price it and it did not run.

**F118-16. `115` F115-4's route was tried in `114`.** `114_probes/p9:244-256` defines a `selection-assoc`
variant with the same `Cond` and `Pick` construction, and `114` reports it refusing at the same fold
length as the expensive tower at `114:884`, `114:897`, `114:913` and `114:1124`. A fact about two
documents rather than a measurement, so no predicate; it makes F115-4 a second independent instance rather
than a first.

**Unpriced.** Every duration, for every arm, at every shape. No bench harness ran and no claim in this
file depends on a magnitude.

---

## 10. What I carry forward unchanged, and from whom

Thirteen things, and every one of them is **inherited rather than independently derived**: I read both
files before building anything in this one. Where I reproduced a claim on my own instrument I say so and
that reproduction is an instance; where I did not, the item is carried on its author's evidence.

**From `115`, five.**

1. The repair shape: two arms sharing conditions, rather than a dimension deleted. It corrects my
   framing and section 2 takes it. Reproduced (q1).
2. F115-2, condition (b) not load-bearing at wrap. Reproduced (q1), and section 2.3 is what it becomes
   when the mirror image is run beside it.
3. F115-4, the trait projection not escaping. Carried, with section 8.2's correction to its provenance,
   which strengthens it. Not reproduced; I had already run the same route.
4. F115-5, the certificate computable from the cheap carrier. Carried in full and it is the half `114`
   section 7 was missing. Not reproduced.
5. `115` section 4.1's concession about which reading of its own section 21 was refuted. Carried; the
   distinction between a const gate choosing which const is read and a selection choosing which type is
   constructed is now the standard form.

**From `116`, seven.**

6. F116-4, the duality. Carried as the root of section 4, reproduced (q2) and widened (F118-4).
7. F116-5, the two families disjoint per policy, with two extra policies so the table is not two points.
   Carried; it closes `114` section 12's open question. Not reproduced.
8. F116-6, a discharged extent restoring both characters. Carried, and section 8.1 uses it to correct my
   own section 6.4. Not reproduced directly; q6 is its consequence at a shipped kernel.
9. F116-7, the operation split at `F > 0`. Reproduced (q3).
10. F116-9, the unit grid rather than requantisation avoidance. Reproduced (q3), including its refutation
    of `116`'s own predicted mechanism.
11. F116-3, the widening of my F114-10 to declarations with a non-negative lower bound. Carried. **Not
    reproduced**, and I should say plainly that I did not check it; it is `116`'s measurement on `116`'s
    own form and I have no reason to doubt it and no instrument pointed at it.
12. `116` section 2.2's sharper lesson, that an offered canon sentence is a finding and carries a
    predicate or names the dimensions it is universal over. That is better than `114` section 3.2's
    version, which said only that a canon sentence should carry its source finding's predicate. `116`'s
    covers the generalising case, which is what actually happened to it. Carried and I would use its
    wording.

**From `117`, one.**

13. That the tests do run and the defect is a false green from one command rather than an untested
    surface. Carried; section 0.2 records it and adds the second working form.

**And two things I hold, both of them additions rather than contests.**

`116` section 5.3 says arm W1's cost accounting is within-policy and the policy is not free, at
`116:359-363`. Substantively right and I take it. One precision: **arm W1 is a resolver-side arm and that
cost is consumer-side.** The policy sits in the declared semantics, supplied by the consumer and never
chosen by the resolver (`108:825`), so the resolver never faces the trade `116` prices. Reading the two
together as one decision would put a licence-family choice in the resolver, which the pair's own
definition forbids. The finding stands; it belongs in what a consumer is told a policy costs.

`116`'s W1b states `F` any in its prose and `F in {0, 1, 2}` in its predicate. Section 6 says that gap is
the transfer exemption applying rather than an overstatement, because the argument that addition and
subtraction never enter the rounding region is `F`-free. **I would state W1b on the argument and say so**,
which is what F116-4 does for itself.

---

## 11. Where the argument stands, and what I could not close

**Converged, with the author of the refuted claim agreeing, on all three of my claims against them.**
F114-3 conceded by `115` after reproducing 28 and 16. F114-6 conceded by `116` after reproducing zero
unsound at wrap against 228 and 198 at sat. F114-12 conceded by `116` and its own F112-14 declared
superseded. F114-10 conceded and widened. F114-17 reproduced on two independent constructions.

**Converged the other way, with me conceding.** `115`'s repair shape against my dimension deletion.
`116`'s bounding of arm W1 against my expectation that `F > 0` would be additive. `116` F116-6 against my
attribution of the shipped kernel to arm W1.

**Refuted, of my own, in this file.** Four predictions: that cutting at a non-homomorphic operation's
operands would suffice, that cutting at its result would not, that requantising at it would, and that the
rounding mode was the discriminator. All four measured wrong and all four kept in the probe headers with
what refuted them. The fifth, that the two-part rule would fail under saturation, was also wrong: it
degenerates.

**Located, and I could not close it.** The order-based licence family has one known member.
`116` section 8 proposes enumerating it and I agree that is the next dispatch on this axis, because
section 4.3's statement has a clause in it whose plural is currently unearned.

**Untouched by anyone, now four consecutive members.** `109` section 8's chain result.

---

## 12. Alternatives considered and not taken

**Defending my dimension-deletion repair.** Considered for exactly as long as it took to read `115:182`
and run q1. It is wrong under I13 and the wrap half of the region is real.

**Rebuilding `112`'s one-sided form to check `116` F116-3's widening.** Not done. `116` imported its own
committed implementation rather than reimplementing, which is the right call for the same reason I reused
`112_probes/p9` in `114` p3, and a third implementation would test my reading of a form that is not mine.
The item is carried unreproduced and section 10 says so.

**A fourth hypothesis about the cut rule after three were refuted.** Explicitly not taken; q4 finds
witnesses instead. Three wrong mechanisms in a row is the point at which guessing a fourth costs more than
instrumenting, and the witnesses gave a better rule than any of the four guesses would have.

**Extending the two-part rule to a signature with division or a shift.** Not attempted. Both leave the
ring and neither is monotone, so the rule's range condition would fire at every such node and the saving
would be whatever the surrounding homomorphic nodes give. That is a prediction and I did not measure it.

**Running `warm-clamp` on the narrower saturating accumulator.** The one measurement in this file that a
harness could take and I did not take it. It is the first item any successor should do and section 8.1
says why.

---

## 13. Coverage, bounded

**Read in full.** `115` and `116` including their findings lists and probe indices, `117`, `INTENTS.md`,
and my own `114`.

**Read in part.** `115_probes/s1` at its check-selection and sweep functions and `s1_output.txt` in full;
`116_probes/p3` at its search structure only; `114_probes/p9` at lines 244 to 256, opened;
`warm-clamp-shared/src/lib.rs` at `:158-160`, `:288`, `:291` and `:296-311`, opened.

**Not read.** `115_probes/s2`, `s3`, `s4` sources; `116_probes/p1`, `p2`, `p4`, `p4b`, `p5`, `p6`
sources; `OPTIONS.md`, `AGREEMENTS.md`, `DROPLIST.md`, `RULES.md`; every panel file before `108` except
through `114`'s own account. Where I cite one of those I am citing another file's account and say so at
the point of use.

**Reproduced rather than accepted.** `115` F115-1, F115-2 and F115-3, on my own instrument (q1).
`116` F116-4's search including its exact free-map figures (q2). `116` F116-7 and F116-9 (q3).
`114`'s own p2 root-only column, arriving for a third time.

**Not reproduced.** `115` F115-5 and its compiled certificate. `116` F116-3, F116-5, F116-6, F116-8 and
F116-10. `116`'s licence counts at 76896 and 12432 cells. Everything in `114` I did not re-run.

**Not established.** Anything at a non-uniform value set. Anything at a real width, except what section 6
argues does not need one. Any duration. Whether a consumer's terms are trees or DAGs. Whether the
order-based family has a second member.

**Citations checked by opening them.** `q7_check_my_own_citations.py` opens every `file:line` this file
leans on and tests the substring the claim depends on, with two deliberately wrong entries as controls.
Result in `q7_output.txt`.

---

## 14. Probe index

All under `118_probes/`, each committed with its output as it ran, each carrying the case that must fail.

- `q0_test_gate_run.txt`. Twelfth count, per crate by `--manifest-path`, with the false-green form run
  beside it so both are on one page.
- `q1_the_condition_set_follows_the_character_too.py`, `q1_output.txt`. `115` F115-1 and F115-2
  reproduced, the mirror image neither file ran, what the conjunction declines at wrap, and three
  controls.
- `q2_the_duality_checked_and_its_hypotheses_ablated.py`, `q2_output.txt`. `116` F116-4 on a different
  search with monotonicity over every total order, its own figures reproduced, and three ablations of
  which two produce witnesses.
- `q3_the_fraction_width_splits_my_arms_too.py`, `q3_output.txt`. F116-7 and F116-9 reproduced, arm W0
  split, and **four predictions of mine refuted**, each kept with what refuted it.
- `q4_two_witnesses_for_why_the_cut_needs_both_halves.py`, `q4_output.txt`. The two failing sets shown
  disjoint, and one witness of each kind printed with every intermediate.
- `q5_one_rule_with_two_locality_conditions.py`, `q5_output.txt`. The rule the witnesses hand over, over
  three settings, with the per-shape reduction count and one instrument defect found and fixed in view.
- `q6_the_shipped_guard_makes_the_carriers_policy_free.py`, `q6_output.txt`. `114` section 6.4 corrected
  against `116` F116-6, and the saturating accumulator's collapse to the logical width.
- `q7_check_my_own_citations.py`, `q7_output.txt`. Every `file:line` opened and its content tested, with
  two deliberately wrong controls.
