# 07. The adjunction frame, tested

**Date:** 2026-08-08
**Position:** responds to `06`'s closing sentence, that the order is infrastructure and the lattice is a
theorem about it, by asking whether the exact-to-representable pair is a Galois connection, what the frame
reproduces, what it predicts, and what it costs.
**Probes:** `07_probes/`, seven instruments in two languages, with `RUN.md` carrying every command, its
exit code, the one instrument killed for cost and kept as the control, and the one comparison that was
wrong on its first run and is named rather than quietly fixed.
**Reading:** `RULES.md`, `01_op_answers.md`, `04_op_no_settlements_tonight.md`,
`06_kiselyov_where_a_numeral_is_inferred.md`, `03_lamport_the_family_question_and_its_consequences.md`,
`05_dispatcher_note_two_meet_questions.md`, `SETTLED.md`, `DROPLIST.md`, `PERSONA_CALLS.md`, all in full
and in that order, plus `seed/SETTLED_laws.md` in the four passages that bear on rounding, monotonicity
and the lattice claim. `CANON_CANDIDATE.md` grepped for the frame's vocabulary and read nowhere; it is
not cited below. The predecessor panel's tree was not read, only counted.
**Register:** nothing here settles anything, per `04`.

## Verdict, before the argument

**There is not one adjunction here. There are two, at two different levels, and the panel has been
holding them as one object.** That is the first finding and everything else is downstream of it.

**Level one is the fibre.** Fix a numeral, and ask about rounding into its value set against the
embedding back out. The brief's framing ("a rounding operation takes an exact value to a representable
one; an embedding takes a representable value back into the exact") is this level.

**Level two is the index.** Ask which numeral, given a set of exact values. The abstraction is the least
numeral containing the set and the concretisation is the value-set map. `06`'s order and `03`'s lattice
are this level, and no file in either panel has connected them to the other.

**Level two holds, and its condition is exactly the record's two closure conditions.** The least numeral
containing a set exists for every set iff the numerals are closed under intersection, which is the meet
exactness the record already carries at TWO EXPERTS (`SETTLED_laws.md:278-313`), and the two admissions
that result names, the origin and negative integer width, turn out to be precisely the codomain of the
closed form for that least numeral. They are not two lattice repairs doing two jobs. They are one
formula's range, and it needs both because the formula produces both (`p3.out` Q2: the origin at 1 input,
negative `I` at 5,487, both at once at **0**).

**Level one holds only for round toward positive infinity, and only in range.** Measured at 0 failures
over 34,976 pairs for that mode and nonzero for every other, with the two directed modes also failing
once out-of-range values are admitted (`p1.out`). But the interesting result is the one that refuted my
own prediction: **every monotone mode is an adjoint to something**, and what it is adjoint to is what a
datum of that mode denotes. Round to nearest is adjoint to a map that reaches half a cell past its own
value (`p4.out` Q1b), which is the correct reading of a nearest-rounded datum and is a statement the
design has never made.

**The frame reproduces seven results the panel derived separately, and dissolves one distinction rather
than correcting it.** `06` corrects `03`'s "a formula is not a lattice operation" by observing that
within a family the join is a formula too. The frame says more: **the join and the product numeral are
the same function evaluated at different arguments.** Every derived numeral is the least containing
numeral of the operation's exact result set; the join is that function applied to a union. Measured at
120 of 120 (`p3.out` Q3). There is one inference mechanism in this design, and `06`'s eight D1 sites are
eight instantiations of it.

**And `06`'s corrected product form is that mechanism, independently.** `06` derives its tight width by a
total-width inequality; I compute the least containing numeral from the value set directly. The two agree
at 400 of 400 operand pairs (`p3.out` Q4), where the sum-of-widths form is the best answer at 324 and
overshoots at 76. Two derivations, one answer, arrived at from opposite directions.

**What it predicts that the panel does not have**, in the order I would rank them:

**One. The fold's sufficiency check is one inequality, and it is on the wrong coordinate.** The droplist
records that a per-iteration accumulator type cannot work and that the replacement is a compile-time
sufficiency check, without saying what sufficiency is. The frame says post-fixpoint, and the measurement
says the post-fixpoint condition reduces to `F_acc >= F_elem`, with **the range half needing no bound at
all** provided the resolution saturates. Zero unsound sequences on and above that diagonal, nonzero
strictly below, over 65,536 sequences per cell (`p5.out` Q4). It compiles as a bound, gate-free, refuses
at type-check rather than at monomorphisation, and erases to the same instructions as the unguarded fold
(`p6`, `p6_asm.out`).

**Two. Saturation's soundness is a choice nobody has made.** Under the reading where a datum denotes its
own value, saturating addition is exactly as unsound as wrapping, at 512 of 1024 sequences at n=5. Under
the reading where the top denotes "at least this", saturating is sound at 0 failures for every n up to 8
while wrapping grows to 55,085 of 65,536 (`p5.out` Q2, Q3). The arithmetic is identical in both readings.
**The whole difference is what a saturated datum means, and the record does not say.**

**Three. Rounding modes split by what they compose with, and the split is measurable.** Directed modes
compose across nested grids at 0 failures; nearest modes fail even nested, with the classical witness
(`p4.out` Q2, Q3). My prediction that only the two infinity-directed modes would compose was **refuted**
by toward-zero composing anyway, and the refined claim that survived a decisive test is that a mode
composes exactly when its direction changes only at points of the coarser grid: 0 failures at three such
pivots, 7 at four others (`p4.out` Q4).

**Four. The cross-kind case is priced rather than closed.** `03` establishes that no admission repairs the
fixed-point-against-float join. The frame names that "no best abstraction" and names the literature's
standard response, closing the family under intersection. Nobody has costed it. Measured: the closure
adds between 16 and 34 percent more shapes than the two families contain, none of them named by either,
every one a segmented grid (`p7.out`).

**What it costs.** The vocabulary, and nothing else. Every result above is statable without the word
adjunction, and section 5 states them that way. The frame is a derivation instrument, and my
recommendation, offered as one expert's reading and not as a settlement, is to take the theorems and
leave the vocabulary out of the canon.

**One thing I could not settle**, and it is where I would send the next dispatch: whether level one and
level two compose, that is, whether rounding commutes with change of numeral. That is the compatibility
condition of the fibred picture and it is the mathematical content of "narrowing then converting equals
converting then narrowing". I did not build it and section 8 says why.

## 0. Gates

### 0.1 The canon gate

There is no ratified canon for arvo. This panel is writing the first one, and `01` section 0 carries op's
correction that the rows marked RATIFIED in `SETTLED.md` were classified under a superseded reading. So
the defend-the-canon posture has no target and the governing material is the narrow set recording op in
the loop.

Two ratified items bear on this file directly rather than tangentially, and both are checked rather than
gestured at.

**The acceptance criterion** at `SETTLED.md:65-71` requires the typestate to derive container and
representation, validate, and **erase on lowering**. Everything this file proposes lives at compile time
and section 4.4 measures the erasure rather than asserting it: the guarded fold and the unguarded fold
are instruction-for-instruction the same function, and the two guarded arms are folded onto one symbol
(`p6_asm.out`).

**The toolbox rule.** `arvo-toolbox-not-policer.md` forbids the substrate from hardcoding a consumer
policy. Section 4.2 raises a question about what a saturated datum denotes, and that is a question about
what the design *says*, not a proposal to remove a consumer's choice of resolution. All three resolutions
stay. Gate passed.

### 0.2 The test gate

Re-run rather than inherited from `06`, and the two counts that overlap agree, which is worth recording
as an independent arrival:

    $ find mock/crates -path '*tests*' -name '*.rs' | wc -l
          91
    $ grep -rl '#\[test\]' mock/crates --include='*.rs' | wc -l
          83
    $ grep -rniE 'galois|adjoint|postfix|post_fix|Sufficient' mock/crates --include='*.rs' | wc -l
           0

There is no suite to audit for this question, because the surface has no source. **I did not run the
suite and I am saying so rather than implying it passed.** The brief separately declares `mock/crates`
nuked and forbids citing it as evidence about what is correct; a count of zero is the one thing a nuked
tree can honestly report.

### 0.3 The brief's cheap factual claims

The pin is as stated: `rust-toolchain.toml` carries `channel = "nightly-2026-05-28"` and
`rustc +nightly-2026-05-28 --version` reports `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.

**The brief's claim that the frame is new is mine to check and it holds.** Counts only, and the
predecessor tree's contents were not read:

    $ ls ../202607301300_formalization-spec-panel | wc -l
    320
    # Galois, adjunc, adjoint, "abstract interpretation", "closure operator",
    # coreflection, comonad, residuat, concretis, concretiz:
    #   files=0 hits=0 for every one, across all 320
    $ grep -ciE "<same terms>" CANON_CANDIDATE.md      # 0 for every term
    $ grep -rciE "<same terms>" seed/                   # 0 for every term

One near-hit exists and it is worth naming precisely so nobody later thinks the ground was covered. The
prior-art survey at `202607281616_prior_art/02_number_systems_and_fixed_point.md:415` mentions abstract
interpretation once, citing an external tool (POP, Ben Khalifa and Dorra) that infers minimal input
precision. That is a citation of somebody else's use of the technique, in a survey, not a use of it as a
frame for this design. **The ground is genuinely new.**

**One claim in `06` that I checked and that holds.** `06` section 6.1 says its agreement with `03` is
inherited rather than found, because it read `03` before deriving. I read both before deriving too, so
the same discount applies to me and section 8 says where.

**One claim in the brief that I would qualify.** It says `06` reports that at least one strategy's
narrowing is not monotone. That is right and it is stronger than the brief makes it: the claim is in the
record at TWO EXPERTS, `seed/SETTLED_laws.md:336` and `:346`, "`Hot`'s narrowing is not monotone", and
`seed/SETTLED_laws.md:340-342` puts it "on the same footing as the already-established refutation of
wrapping-addition distributivity". Section 3.7 does something with that rather than noting it.

## 1. Setting the two sides up, and why there are two of them

The brief asks which direction is which and warns that the choice changes what the adjunction says. It
does more than that: **there is no single correct choice, because there are two adjunctions on two
levels, and the design's operations split cleanly along them.**

### 1.1 The definitions, once

A monotone Galois connection between posets $C$ and $A$ is a pair of monotone maps
$\alpha : C \to A$ and $\gamma : A \to C$ with

$$\alpha(c) \sqsubseteq_A a \iff c \sqsubseteq_C \gamma(a) \qquad \text{for all } c \in C,\ a \in A.$$

$\alpha$ is the lower (left) adjoint and $\gamma$ the upper (right) one. Three consequences are used
below and none of them is assumed:

- Each adjoint determines the other uniquely.
- $\gamma \circ \alpha$ is a closure operator on $C$ (extensive, monotone, idempotent), and
  $\alpha \circ \gamma$ is an interior operator on $A$.
- $\alpha$ preserves all joins and $\gamma$ preserves all meets. Conversely, between complete lattices,
  a monotone $\gamma$ has a lower adjoint **iff** it preserves all meets.

That last biconditional is the whole of section 3.1 and it is why this frame reproduces the record's
results rather than restating them.

### 1.2 Level one: the fibre

Fix a numeral $N$ with value set $V = V(N)$, a finite set of rationals. Take

$$C_1 = (\mathbb{Q} \cap [\min V, \max V],\ \le), \qquad A_1 = (V,\ \le), \qquad \gamma_1 = \iota,$$

the plain embedding, and $\alpha_1$ a rounding mode. This is the brief's framing exactly.

Two things about the choice of $C_1$ are load-bearing and are stated rather than assumed. The order is
the numeric one, not inclusion, because both sides are single values. And the concrete side is **the
numeral's own range**, not all of $\mathbb{Q}$. Section 2.2 measures what happens when the restriction is
dropped, and the answer is that the overflow band is exactly the region where the adjunction stops
holding, which makes the restriction a finding rather than a convenience.

### 1.3 Level two: the index

Take

$$C_2 = (\wp_{\mathrm{fin}}(\mathbb{Q}),\ \subseteq), \qquad A_2 = (\mathcal{N}/{\equiv},\ \sqsubseteq),
\qquad \gamma_2(N) = V(N),$$

where $\mathcal{N}$ is the admitted shape space, $\equiv$ identifies numerals with equal value sets, and
$\sqsubseteq$ is inclusion of value sets. Then $\alpha_2(S)$, if it exists, is the least numeral whose
value set contains $S$.

The quotient is not hygiene. `SETTLED.md:122` records the order as being on value sets, and in this frame
that is exactly the condition $\alpha_2 \circ \gamma_2 = \mathrm{id}$, which makes the connection a
**Galois insertion**. Without it there are numerals the abstraction cannot distinguish and the interior
operator is not the identity. So the quotient the record already carries is the insertion condition, and
section 3.5 says what that buys.

### 1.4 The two levels are not the same question and the design splits along them

`06`'s enumeration divides the design's inference sites into D0 (the consumer names the target), D1 (the
operands determine it), D2 (the design determines it by a rule) and D3 (nothing determines it, empty).
Against the two levels:

- **Every D0 site is a level-one question.** The numeral is given, so the only question is where an exact
  value lands inside it. `quantize`, narrowing, conversion, `sqrt`, `recip`, in-numeral arithmetic: all
  of them are $\alpha_1$ in a fibre the consumer chose.
- **Every D1 site is a level-two question.** The numeral is not given, so the question is which fibre.
  `mulnum`, the product numeral, `Abs`, exact scaling, the bitfield parent: all of them are $\alpha_2$ on
  the operation's exact result set.
- **D2 sites are where $\alpha_2$ does not exist** and something outside the order picks. The container
  tie-break `06` finds at its section 2.3 is exactly this shape, and section 3.6 says so.

That is a clean split and I did not expect it to be clean. It also says something about `06`'s own
carve: the D0/D1 line is not a taxonomy imposed on the sites, it is the fibre-versus-index line falling
out of the mathematics.

### 1.5 The fibred picture, and the one thing it asks that nobody has

Putting the two together, the honest object is a family of adjunctions indexed by an adjunction: for each
numeral $N$ a fibre-level $\alpha_1^N \dashv \iota$, sitting over an index-level $\alpha_2 \dashv
\gamma_2$.

A fibred structure like that has a **compatibility condition**, and it is the only interesting thing the
picture asks:

$$\alpha_1^{N_2} \circ \alpha_1^{N_1} \;=\; \alpha_1^{N_2} \quad \text{whenever } V(N_2) \subseteq V(N_1).$$

In words: rounding into a numeral and then narrowing equals narrowing in one step. That is exactly the
double-rounding question, section 2.3 measures it, and it is exactly the condition under which the
design's narrowing composes. **The record has narrowing as "the quantiser with the operation set to the
identity" (`seed/SETTLED_laws.md:336`, TWO EXPERTS) and says nothing about whether two narrowings
compose.** The frame says the question exists and section 2.3 answers it.

## 2. The round-trip laws, per rounding mode

### 2.1 The prediction, and where it was wrong

I predicted before running anything that round toward $+\infty$ would satisfy the lower-adjoint law,
round toward $-\infty$ the upper-adjoint one, and that the other four would satisfy neither. The first
two hold. **The third is right about the embedding and wrong about adjointness**, and the correction is
the more useful half.

`p1` tests the defining biconditional pointwise over 23 numeral shapes at exact rational arithmetic,
34,976 pairs in range:

| mode | lower-adjoint law fails | upper-adjoint law fails |
|---|---|---|
| toward $+\infty$ | **0** | 891 |
| toward $-\infty$ | 891 | **0** |
| toward zero | 747 | 144 |
| away from zero | 144 | 747 |
| nearest, ties even | 457 | 434 |
| nearest, ties away | 345 | 546 |

So against the **embedding**, exactly one mode is a lower adjoint and exactly one is an upper adjoint.
The witnesses are small and checkable: `toward -inf` fails the lower law at $x = 1/8$, $\alpha(x) = 0$,
$v = 0$, where $\alpha(x) \le v$ holds and $x \le v$ does not.

Two further columns from the same instrument, because they separate things that are easy to conflate.
**All six modes satisfy $\alpha \circ \gamma = \mathrm{id}$**, at 0 of 320 representable points
(`p1.out` Q2), so the insertion condition is not what distinguishes them. And extensivity splits exactly
as the biconditional does: only toward $+\infty$ has $\gamma(\alpha(x)) \ge x$ everywhere, only toward
$-\infty$ has $\le$ everywhere (`p1.out` Q3).

### 2.2 Overflow is outside the adjunction, measured

Re-run with out-of-range exact values admitted and clamped, which is what a total rounding map must do:

| mode | lower fails | upper fails |
|---|---|---|
| toward $+\infty$ | 184 | 1075 |
| toward $-\infty$ | 1075 | 184 |

**Both directed modes lose the adjunction, and they lose it by exactly the same amount.** The 184 are the
clamped pairs. So the frame draws the line the design already draws: the quantiser is one thing, the
overflow resolution is another, and the adjunction is a statement about the first. That is a structural
argument for a separation the record has on other grounds, and it arrives without any appeal to taste.

### 2.3 What each mode is actually adjoint to, which refuted my prediction

`p4` asks the question a different way. If a monotone map $\alpha$ has a right adjoint at all, that
adjoint is forced:

$$\gamma'(v) = \max\{\,x \in C : \alpha(x) \sqsubseteq v\,\}.$$

So "is it an adjoint" splits into "does $\gamma'$ satisfy the biconditional" and "is $\gamma'$ the
embedding". Measured, at 0 biconditional failures for **all six modes**. Every one of them is a lower
adjoint; the forced candidate works exactly when the map is monotone, and all six are monotone.

That refutes the framing I brought and replaces it with a better question. `p4.out` Q1b prints the gap
$\gamma'(v) - v$, on a quarter-step numeral with a $1/32$ concrete tick:

| mode | $\gamma'(v) - v$ |
|---|---|
| toward $+\infty$ | $\{0\}$ |
| toward $-\infty$, toward zero, away from zero | $\{0,\ 7/32\}$ |
| nearest, both tie rules | $\{0,\ 3/32,\ 1/8\}$ |

$7/32$ is a full cell less one tick; $1/8$ is half a cell. So:

> **A rounded datum does not denote its own value. It denotes the set of exact values that produced it,
> and that set is the mode's upper adjoint.** For round toward $+\infty$ the set is a point, which is why
> that mode alone is adjoint to the embedding. For a directed mode the set is a cell; for a nearest mode
> it is a half-cell either side.

**This is the single most useful sentence the frame produces about rounding, and the design has never
made it.** It is not an error bound bolted on afterwards; it is what the type means. And it composes:
section 4.2 is the same observation applied to the saturating top, where the design's silence has a
measurable cost.

### 2.4 Composition, the prediction that over-fired, and the claim that survived

Adjunctions compose, so a mode that is an adjoint to the embedding must satisfy the double-rounding law
on nested grids. Measured (`p4.out` Q2), counts of exact values where two-step rounding differs from
one-step:

| grid triple | $+\infty$ | $-\infty$ | zero | away | nearest even | nearest away |
|---|---|---|---|---|---|---|
| nested $1/16 \to 1/4 \to 1$ | 0 | 0 | 0 | 0 | 8 | 8 |
| nested $1/32 \to 1/8 \to 1/2$ | 0 | 0 | 0 | 0 | 16 | 16 |
| nested $1/64 \to 1/16 \to 1/4$ | 0 | 0 | 0 | 0 | 32 | 32 |
| non-nested $1/12 \to 1/4 \to 1/3$ | 12 | 12 | 12 | 12 | 8 | 8 |
| non-nested $1/36 \to 1/9 \to 1/4$ | 24 | 24 | 24 | 24 | 16 | 16 |
| non-nested $1/30 \to 1/6 \to 1/5$ | 40 | 40 | 40 | 40 | 24 | 24 |

Nested grids: the four directed modes compose, both nearest modes do not, with the classical witness at
$x = 9/16$ rounding to 1 in one step and to 0 through the quarter grid (`p4.out` Q3).

**And toward-zero composes, which it should not have if being an adjoint to the embedding were the
condition.** Re-run over a signed range so toward-zero stops coinciding with floor (`p4.out` Q2b): still
0. So my prediction over-fired and the honest statement is that **being an adjoint to the embedding is
sufficient for the composition law and is not necessary.**

Attacking that rather than reporting it: toward-zero is floor above zero and ceil below it, so it is an
adjoint on each side of a point that lies on every grid here. The refined claim is that a mode composes
across nested grids exactly when its direction changes only at points of the **coarser** grid. That is
decidable by experiment, so `p4.out` Q4 builds a family of "away from pivot $p$" modes and moves the
pivot:

| pivot | on the coarse grid | composition fails at |
|---|---|---|
| 0, 1, 2 | yes | **0**, three times |
| 1/2, 1/4, 3/4, 3/2 | no | **7**, four times |

Decisive, and the refined claim is what I would carry forward:

> A rounding mode composes across nested grids exactly when its direction switches only at points of the
> coarser grid. Round toward $\pm\infty$ never switches. Round toward zero switches at zero, which is a
> point of every anchored family, so it composes. Round to nearest switches at every cell midpoint of the
> finer grid, none of which is a coarse point, so it never composes.

That is stronger than the adjunction argument, it explains the classical result rather than restating it,
and it hands the design a usable rule: **narrowing composes iff the mode's pivot set is contained in
every grid it will narrow into.**

## 3. What becomes a corollary

The test the brief sets is whether the frame reproduces results the panel derived separately. Seven, and
I am counting only ones where the frame produces the result rather than agreeing with it.

### 3.1 The two admissions are the codomain of one formula

`SETTLED_laws.md:278-313` (TWO EXPERTS) records meets as exact within a family "subject to two closure
conditions the design must grant (admit the zero-width numeral; admit negative integer width)". `03`
section 3.1 finds them doing two different jobs, one restoring existence and one restoring exactness, and
says no document it read connects them. `06` section 7.2 finds negative integer width needed at
multiplication and observes that the mechanism is identical to `03`'s and the caller different, without
saying why.

The frame says why, in one step. $\gamma_2$ has a lower adjoint iff it preserves meets; preserving meets
is exactly meet exactness; so **the record's two closure conditions are the condition for $\alpha_2$ to
exist**, and every site that infers a numeral is a site that computes $\alpha_2$. One caller, eight
instantiations.

Better than that, the closed form makes it concrete. For a finite nonempty set $S$ of nonnegative
dyadic rationals, the least $U\langle I,F\rangle$ containing $S$ is

$$F^\* = \min\{F : \forall s \in S,\ s \in 2^{-F}\mathbb{Z}\}, \qquad
I^\* = \min\{I : 2^{I} \ge \max S + 2^{-F^\*}\},$$

least because the two coordinates minimise independently and the order is componentwise. **This is a
total function into the $(I,F)$ plane. It never fails.** So a best abstraction never fails to exist for a
mathematical reason; it fails only when the pair it names is not admitted.

Validated at 0 disagreements against enumeration over 3,163 comparable sets (`p3.out` Q1). And the
Moore condition itself is measured independently by enumeration in a box, where closure under
intersection and meet existence are computed by two separate routines and agree exactly: 36 of 351
pairs fail both at the strict admission policy, **0 of 378 once the origin is admitted**, and 0 at every
wider policy (`p2.out` Q1 against Q3). Codomain measured over 2,796,636 sets (`p3.out` Q2): the origin is the answer for exactly **1** input, namely
$S = \{0\}$; negative $I$ is the answer for **5,487**; and **0** inputs need both at once.

That last zero is the finding. The two admissions are disjoint conditions on one formula's range, which
is exactly why the record found them doing two different jobs and could not say what related them.

**Restated for the canon, without the frame's vocabulary:**

> The tightest numeral holding a given set of values is computed by taking the coarsest grid that carries
> every value and then the smallest reach that covers the largest. That computation always has an answer.
> Its answers include the shape that holds only zero, and shapes whose integer width is negative. A design
> that refuses either is a design whose tightest answer sometimes does not exist.

### 3.2 The join and the product numeral are the same function

`03` section 7.3 argues the design's operations derive result numerals by formula rather than by least
upper bound. `06` corrects it: within a family the join is a formula too, so the contrast does not carry
the argument, and rebuilds the argument on coherence grounds instead.

The frame dissolves the distinction rather than correcting it. $N_1 \vee N_2$ is by definition the least
numeral whose value set contains both, which is the least numeral containing $V_1 \cup V_2$, which is
$\alpha_2(V_1 \cup V_2)$. Measured at 120 of 120 comparable pairs, 0 differ (`p3.out` Q3), which is
worth measuring precisely because it is true by definition and a definition nobody wrote down is not
available to a reader.

So:

> **The design has one inference mechanism.** Every derived numeral is the tightest numeral containing
> the exact result set of the operation. The join is that mechanism applied to a union, the product
> numeral is it applied to a product set, `Abs` is it applied to an image, exact scaling is it applied to
> a translate. They are not a formula and a lattice operation competing for a role. They are one function
> at different arguments.

`06`'s D1 class is exactly the set of operations for which the design computes this, and its observation
that every non-empty class's answer is a function is this statement with the mechanism unnamed.

### 3.3 `06`'s corrected product form is that mechanism, arrived at independently

`06` section 7.1 derives a corrected total width for the product, by an inequality on total widths:
$W_{\text{out}} = W_1 + W_2 - 1$ when $2^{W_1} + 2^{W_2} - 2 \ge 2^{W_1+W_2-1}$, else $W_1 + W_2$, with
$F_{\text{out}} = F_1 + F_2$.

I did not use that derivation. I compute the exact product set and apply the closed form of section 3.1.
Over 400 operand pairs (`p3.out` Q4):

- the sum-of-widths form is the best abstraction at **324** and overshoots at **76**;
- **`06`'s corrected form is the best abstraction at 400 of 400.**

Two derivations from opposite directions landing on the same formula. Under `RULES.md` that is genuine
corroboration rather than inheritance, because the method is different and I did not use `06`'s
inequality to compute anything: it is compared against, not applied.

One difference worth stating so nobody reads it as a discrepancy. `06` reports 15 residual pairs where its
form disagrees with the least **admitted** shape; I report 0 against the raw codomain. The difference is
the floor at $I \ge 0$: my Q4 measures 9 of 400 pairs whose best answer needs $I < 0$, with the witness
$U\langle 0,1\rangle$ squared giving $(I,F) = (-1,2)$, which is `06`'s own witness. Consistent, and the
floor is the whole gap.

### 3.4 `03`'s three failure modes are three named conditions

`03` section 1.2 splits "the operation is partial" into F1 (no bound at all), F2 (bounds present, none
extremal) and F3 (an extremum that is not the exact intersection), and observes that each takes a
different repair. The frame names all three and explains why the repairs differ:

- **F1 is a missing top.** $\alpha_2(S)$ has no candidates. Repaired by admitting a shape above, which is
  why `03` finds it repairable by admitting an endpoint.
- **F2 is the Moore condition failing at the top end**, two incomparable minimal upper bounds. **No best
  abstraction exists**, and adding shapes above cannot help because a shape above the minimal ones is not
  below them. This is a named and studied situation, not a defect: a domain without a best abstraction is
  worked with by dropping $\alpha$ and keeping $\gamma$ with a soundness relation. Section 6.6.
- **F3 is $\gamma_2$ failing to preserve meets**, which is exactly the biconditional in section 1.1
  failing, which is exactly $\alpha_2$ not existing. Repaired by admitting shapes in between, which is
  what negative integer width is.

`03` says the repairs differ and could not say why. The frame says F1 is about the poset's endpoints, F3
is about $\gamma$'s meet preservation, and F2 is about both at once and repairable by neither.

### 3.5 The quotient is the insertion condition

`SETTLED.md:122` records the order as being on value sets, and `03` section 7.5 closes the route of
weakening it, on the grounds that antisymmetry is what makes a result numeral well defined. `06` section
5.2 lists the quotient as one of four uses of the order that are not callers.

The frame gives it a name and a consequence. $\alpha_2 \circ \gamma_2 = \mathrm{id}$ makes the connection
an insertion, and an insertion is exactly a connection with no redundancy in the abstract domain. Without
the quotient, the interior operator $\alpha_2 \circ \gamma_2$ is a nontrivial idempotent, two numerals
denoting one set are two abstract elements, and "the tightest answer" is not unique. So the quotient is
not hygiene next to the order; **it is the condition that makes the order's extrema single-valued at
all**, which is upstream of every D1 formula being a function.

### 3.6 The container's tie-break is a D2 site because no best abstraction exists there

`06` section 2.3 finds the container axis has incomparable elements at equal width, because the wide
payload is parameterised on bytes and alignment, and finds the design answering with a named rule keyed
on the strategy. It calls that a precedent worth more than an argument.

The frame agrees and says what kind of precedent it is: the container domain is a product of two
coordinates, the union of the two orderings is not a Moore family, so $\alpha$ does not exist there, so
the choice must come from outside the order. **That is F2 at the container**, and the design's answer to
F2 is already on record. Which means `03`'s reading D, a stated tie-break, is not an exotic fourth option;
it is the design's own existing answer to the same mathematical situation, applied once more.

### 3.7 Monotonicity is a precondition, and its absence explains a droplist entry

The droplist records, without a mechanism, that gating the algorithm crates on associativity "admits the
one preset (`Hot`, wrapping) whose recurrences return wrong answers under these algorithms' own stated
specifications, and refuse the two (`Warm`/`Cold`, saturating) that compute correctly". `SETTLED_laws.md`
carries `Hot`'s narrowing not being monotone at TWO EXPERTS.

The frame supplies the mechanism, and it is one line: **a non-monotone map is not an adjoint to anything,
so no soundness argument runs over it at all.** Not "the bound is loose", not "the law fails in a corner":
there is no abstraction relation to be sound with respect to.

Measured independently rather than inherited (`p5.out` Q1), over the resolved step $a \mapsto R(a+p)$:

| resolution | not monotone at |
|---|---|
| saturating | **0** of 224, 0 of 224, 0 of 1920 |
| wrapping | 84 of 224, 84 of 224, 680 of 1920 |
| substitute zero | 84 of 224, 84 of 224, 680 of 1920 |

The wrapping result re-derives what the record has. **The substitute-zero result is new here**: it fails
monotonicity at counts identical to wrapping's, at every shape tried. The droplist has substitute-zero
breaking associativity where clamping and modular reduction preserve it; it does not have it failing
monotonicity, and by this measurement it fails it exactly as badly as wrapping does.

## 4. What it predicts that the panel does not have

Four, ranked by what I would build first.

### 4.1 The fold's sufficiency check is one inequality, on the fraction width

The droplist entry is precise about what cannot work and silent about what replaces it:

> **Growing an accumulator's own *type* on every iteration of a runtime-bounded loop**: cannot work in
> principle, not merely unbuilt, since a type cannot depend on a value only known at runtime. Replaced by
> fixing the per-element product's type and checking accumulator sufficiency as a compile-time bound.

`06` site 10 calls this the clearest caller the order has and notes it did not compile anything.

**The frame supplies the shape, and the shape is the standard one for reasoning about an unbounded loop
with a fixed abstract element: the accumulator is sufficient exactly when it is a post-fixpoint of the
abstract step.** If $\mathrm{step}^\sharp(A, P) \sqsubseteq A$, then by induction on the trip count and
monotonicity of the step, the concrete accumulation after any number of iterations lies inside
$\gamma(A)$. **The trip count never appears**, which is precisely the property the droplist entry needs
and precisely why abstract interpretation works on unbounded loops at all.

That is a shape, not yet a check. `p5.out` Q4 turns it into one by sweeping the accumulator's fraction
width against the element's, at $n = 4$, under a saturating resolution, counting unsound sequences:

| $F_A \backslash F_P$ | 0 | 1 | 2 | 3 |
|---|---|---|---|---|
| **0** | 0 | 240 | 4080 | 65520 |
| **1** | 0 | 0 | 3840 | 65280 |
| **2** | 0 | 0 | 0 | 61440 |
| **3** | 0 | 0 | 0 | 0 |

Zero exactly on and above the diagonal. So:

> **The accumulator sufficiency condition is $F_{\mathrm{acc}} \ge F_{\mathrm{elem}}$, and the range half
> needs no bound at all.**

The second clause is the surprising one and I want it read carefully. Under a saturating resolution with
the top read as absorbing, the accumulator does not have to be **wide** enough for any number of
additions. It has to be **fine** enough. That inverts the intuition the phrase "accumulator sufficiency"
carries, where the worry is always overflow, and it says the real obligation is on the grid.

### 4.2 Saturation's soundness is a concretisation choice, and the design has not made it

The clause "with the top read as absorbing" in section 4.1 is doing all the work, and isolating it is the
finding.

`p5.out` Q2 and Q3 run an $n$-step fold and ask whether the abstract answer over-approximates the exact
sum, under two readings of what a datum denotes:

- **point reading**: a datum denotes exactly its own value;
- **absorbing reading**: the top denotes $[\text{top}, \infty)$, everything else itself.

| resolution | unsound, point, $n=5$ | unsound, absorbing, $n=5$ | absorbing, $n=8$ |
|---|---|---|---|
| wrapping | 512 / 1024 | 511 / 1024 | 55,085 / 65,536 |
| **saturating** | 512 / 1024 | **0** | **0 / 65,536** |
| substitute zero | 512 / 1024 | 512 / 1024 | 56,313 / 65,536 |

**Under the point reading saturating is exactly as unsound as wrapping.** Under the absorbing reading it
is perfect, at every trip count from 1 to 8, and the other two are not helped at all.

The arithmetic is identical in both rows. The whole difference is what a saturated datum means, and:

> The design's own algorithm crates already rely on the absorbing reading, because the droplist records
> that the saturating presets "compute correctly" under those algorithms' stated specifications. But the
> reading is nowhere in the record as a statement, and section 2.3's measurement of what a rounded datum
> denotes is the same question one level down.

This is the question I would most want asked, because it is cheap to answer and everything about the
fold's correctness turns on it. It is also not a policy question the toolbox rule would forbid: all three
resolutions stay, and the canon simply says what the top of a saturating numeral denotes.

### 4.3 The cross-kind case is priced, not closed

`03` section 3.2 establishes that reading A's two admissions do not close the join across kinds, by a
counting argument plus three instruments producing the same two minimal upper bounds. That is F2, and F2
is repairable by neither admission.

The frame names the standard response, which is to close the family under intersection so a best
abstraction exists again, and **nobody has priced it.** `p7` does, structurally.

The fixed-point family with the origin admitted is already closed, at 0 added sets. The union with a
float family is not. Swept against the float exponent span with the fixed-point box held at total width
5:

| float exponent span | float shapes | union | closure adds | added / union |
|---|---|---|---|---|
| 1 | 9 | 28 | 1 | 0.036 |
| 2 | 30 | 49 | 10 | 0.204 |
| 3 | 63 | 82 | 27 | 0.329 |
| 4 | 108 | 127 | 43 | 0.339 |
| 5 | 165 | 184 | 53 | 0.288 |

So the completion is a **constant-factor enlargement**, settling around a fifth to a third, rather than an
explosion into arbitrary finite sets. And every added shape is one neither family names: the smallest is
$\{0, 1/4, 1/2, 3/4, 1, 3/2\}$, a grid that changes step partway up. **Zero of the added sets are already
named by either family**, at every box tried.

That converts `03`'s "reading A across kinds is not delivered" into something op can actually weigh:

> Reading A across kinds is deliverable, and its price is a third family of segmented numerals whose size
> is roughly a fifth to a third of the two it joins. It is not a closed door; it is a door with a number
> on it.

I am not proposing it. `03`'s reading C and reading D are both cheaper by a wide margin and section 3.6
says the design already has a precedent for reading D. But "impossible" and "possible at this price" are
different answers and the record has been carrying the first.

`p7` also reproduces `03`'s witness independently in a third instrument: $U\langle 0,1\rangle$ against
$U\langle 2,0\rangle$ has exactly two minimal upper bounds across both kinds, $U\langle 2,1\rangle$ at 8
values and the float at precision 2 and exponents $-1$ to $1$ at 7, and the union itself is named by
neither.

### 4.4 Soundness against bestness, which sharpens `06`'s third question to op

`06` asks whether a canon sentence may claim the derived numeral is the tightest honest answer, and
measures the natural formula not tight at 461 of 6561 pairs. It frames the choice as between changing the
formula and changing the sentence.

The frame says there are two sentences, they are different claims, and only one of them is expensive.

$$\text{soundness:}\quad \alpha_2 \circ f \circ \gamma_2 \;\sqsubseteq\; f^\sharp
\qquad\qquad
\text{bestness:}\quad \alpha_2 \circ f \circ \gamma_2 \;=\; f^\sharp$$

**Soundness is what correctness requires.** It says the derived numeral holds every value the operation
can produce. It is cheap, it is what every formula in the design already satisfies including the
sum-of-widths one, and it does not need the shape space to be closed under anything.

**Bestness is what "tightest" claims.** It needs $\alpha_2$ to exist, which needs the Moore condition,
which needs both admissions of section 3.1.

So the choice is not formula-against-sentence, it is:

> State soundness, which is always true, needs no admissions, and is what correctness rests on. Or state
> bestness, which is a stronger claim, requires the corrected formula, and requires admitting the origin
> and negative integer width.

That is a cleaner fork than the one `06` presents, and it is decidable on grounds other than measurement:
a canon that states soundness has said what makes the design correct, and a canon that states bestness
has additionally promised not to waste a bit. Both are defensible and they cost differently.

**And it retires the framing that the overshoot is a defect.** The sum-of-widths form is sound and not
best. That is a named position, not a bug, and calling it one is what makes a correctness question out of
a size question.

### 4.5 The whole of it compiles as a bound and erases

A predicted check nobody can build is a wish. `p6` builds the section 4.1 condition two ways and measures
what each costs.

**Arm A** puts the inequality in a post-monomorphisation assert. It compiles for any pair and fails only
when the offending instantiation is reached:

    error[E0080]: evaluation panicked: accumulator grid is coarser than the element grid
    note: the above error was encountered while instantiating
          `fn fold_arm_a::<U<3, Sk<Z>>, U<1, Sk<Sk<Sk<Z>>>>>`

The record already rules on that hole (`SETTLED.md:98`, RATIFIED), so this arm exists to be the thing the
other is better than.

**Arm B** follows the workspace's own rule for a refused bound: break the constraint into named contracts
that hold on their own and compose. A sealed inductive order on a type-level nat, two impls and no
enumeration of widths anywhere, bound in the `where` clause. It refuses at type-check:

    error[E0277]: the trait bound `Sk<Sk<Z>>: Le<Z>` is not satisfied
    note: required for `U<3, Sk<Z>>` to implement `GridSufficientFor<U<1, Sk<Sk<Sk<Z>>>>>`

**Gate-free.** The file carries no `#![feature(...)]` at all, which is stated positively because it is
checkable: if any of it needed a forbidden feature the file would not build.

**And it erases.** Compiled at `-O` with one codegen unit, the guarded fold and the unguarded baseline are
the same ten instructions after normalising local label indices, and the two guarded call sites are
folded onto one symbol:

    __RNv...12call_b_equal   10 instructions
    __RNv...9fold_bare       10 instructions
    identical after label normalisation: True
    __RNv...6call_a = __RNv...12call_b_equal
    __RNv...6call_b = __RNv...12call_b_equal

The acceptance criterion's erasure clause is satisfied by measurement rather than by argument.

**The first version of that comparison said False**, because it compared raw text and `LBB0_3` against
`LBB1_3` read as a difference. The wrongness is named in the comparison tool's own docstring rather than
fixed silently, on the same footing as the instrument that was killed for cost.

## 5. What it costs the canon

The honest answer is: the vocabulary, and nothing else. Every result above is statable without it, and
the test is whether the statement survives the translation.

### 5.1 Every result, without the frame

- **Section 3.1**: "The tightest numeral holding a given set of values is computed by taking the coarsest
  grid carrying every value and the smallest reach covering the largest. That computation always has an
  answer. Its answers include the shape holding only zero and shapes of negative integer width."
- **Section 3.2**: "Every derived numeral is the tightest numeral holding the operation's exact results.
  There is one such rule and every operation is an instance of it."
- **Section 2.3**: "A rounded value stands for the exact values that produced it, not for itself. Rounding
  in a fixed direction makes that set a single cell; rounding to nearest makes it a half cell either
  side."
- **Section 2.4**: "Narrowing twice equals narrowing once exactly when the rounding's direction changes
  only at values the coarser numeral can hold."
- **Section 4.1**: "A fold's accumulator holds what the fold can produce, for any number of steps, exactly
  when its grid is at least as fine as the element's. Nothing is required of its reach."
- **Section 4.2**: "The top of a saturating numeral stands for every value at or above it. Without that,
  a saturating fold is no more correct than a wrapping one."
- **Section 4.4**: "The derived numeral holds every value the operation can produce" (soundness) against
  "and no numeral smaller does" (bestness).

Not one of those needs the word adjunction, and each passes the permanence test: it stays true under a
total rewrite, and it constrains the outcome tightly enough that independent implementations would agree.

### 5.2 So what is the frame for

It is a derivation instrument, and I would keep it in the audit trail rather than the canon. Its value
tonight was that it produced seven results from one structure and predicted four things nobody had asked,
including two that measurement then corrected. A canon that names it buys a reader who knows the term a
shortcut, and costs every reader who does not a concept they never need.

**One qualification against my own recommendation.** Section 4.4's soundness-against-bestness split is
hard to state crisply without *some* name for the two claims, and a canon that states both and does not
distinguish them will have readers conflating them, which is exactly what has already happened to the
tightness sentence. If any of the vocabulary earns a place, it is that one pair, and even there
"holds every result" against "and nothing smaller does" carries it.

### 5.3 The risk the frame carries, named

A literature identification that turns out false is a specific failure this record already has. The
droplist and `SETTLED.md:117` carry the finest-view mechanism's literature relation as **refuted and
unrepaired**: `136` compile-refuted the identification and op declared prior calls stale over it, and the
mechanism survived while the identification did not.

So the same discount applies here and I am stating it before anyone builds on this file. The
identification I am proposing is that the numeral is a **reduced product of a congruence domain and an
interval domain**, which is section 6.5. **I have not verified that against the literature's own
definitions**, only against the design's four-condition inclusion test, which does decompose that way
(`SETTLED.md:118`: grid, phase, and both endpoints). Everything in sections 2 through 4 is derived from
first principles and measured, and stands whether or not the identification holds; section 6.5 is the one
place where it would matter, and it is marked.

## 6. Neighbouring framings, and where each lands

`04` asks for breadth by category before depth by variant. Six, of which two are the same thing as the
adjunction, one degenerates, one is a genuine alternative for the cross-kind case, one is a literature
identification I am flagging as unverified, and one is my own home ground and I am saying honestly what it
does and does not buy.

### 6.1 The closure operator, which is the same content and reads better

$\gamma_2 \circ \alpha_2$ is a closure operator on sets of exact values: extensive, monotone, idempotent.
Its fixed points are exactly the representable sets. Equivalently, the numerals are the **closed sets** of
that operator.

That is the same statement as the adjunction, by a standard equivalence, and for a canon it reads better,
because "the numerals are the sets closed under taking the tightest containing numeral" is a sentence
about numerals rather than about a pair of maps. If any framing here belonged in a canon it is this one.

### 6.2 The insertion, which is where the quotient lands

Section 3.5. $\alpha_2 \circ \gamma_2 = \mathrm{id}$ makes it a Galois insertion, and the record's
value-set quotient is exactly that condition. Lands as: the quotient is not hygiene, it is what makes
"the tightest answer" single-valued.

### 6.3 Monad and comonad, which degenerate, and that is the result

A closure operator on a poset **is** a monad, and an interior operator **is** a comonad. So the
categorical reading is available and it buys nothing, because in a poset there is at most one arrow
between two objects, every coherence diagram commutes automatically, and the monad laws are exactly
extensivity, monotonicity and idempotence restated.

**That is a result and it is worth writing down as one.** It says the machinery does not need to be
reached for, and it forecloses a direction a later member might otherwise spend a dispatch on. The
categorical structure would start earning its keep only if the numerals carried more than an order, for
instance if the maps between them carried data rather than being mere inequalities. They do not.

### 6.4 The graded reading, honestly

My own field, so I will be careful. The record already has a graded structure: `SETTLED.md:117` carries
the finest-view mechanism with a grade monoid at RATIFIED minus one part.

The natural graded question here is whether the loss a rounding incurs forms a grade, so that a composite
operation's grade is computed from its parts. Section 2.3 gives a concrete candidate: the gap
$\gamma'(v) - v$ measured per mode is exactly a per-operation quantity of that shape, and it composes
along a chain of operations.

**I did not build it and I am not proposing it.** Two reasons, and the second is the real one. The gap
composes additively under addition and not under multiplication, so the semiring is not obviously there,
and I have no instrument. And the design already has a grade monoid for a different axis, so introducing
a second graded structure needs a reason beyond its being available. The honest position is that this is
a direction with one concrete instance and no evidence, and a later member should treat it as untested
rather than as a lead.

### 6.5 The congruence-times-interval reduced product, flagged as unverified

`SETTLED.md:118` (TWO EXPERTS) records inclusion as needing four conditions: the grid, the phase, and both
endpoints. That decomposes exactly as a **congruence** constraint (grid and phase, "$x \equiv b \bmod q$")
and an **interval** constraint (the two endpoints). Each is a standard abstract domain with its own
worked-out theory, and their combination is a reduced product.

If the identification holds, three things follow for free rather than needing derivation: the reduced
product of two domains each with a best abstraction need not have one, so F2 is expected rather than
surprising; the reduction operator is exactly $\alpha_2$; and the closure cost measured in section 4.3 is
the reduction's cost.

**I have not verified this against the literature's own definitions**, and section 5.3 says why that
matters here specifically. It is the most promising single lead in this file for someone with the sources
to hand, and it is the one thing here I would not build on before checking.

### 6.6 The concretisation-only reading, which is the real alternative for F2

Where no best abstraction exists, the standard move is to drop $\alpha$ entirely and work with $\gamma$
plus a soundness relation: "this numeral holds this set" without "this is the tightest one".

**That is `03`'s reading D and reading C, in one frame.** Reading C refuses across kinds, which is
$\gamma$-only with the relation left unstated; reading D picks by a rule, which is $\gamma$-only with the
choice named. Both are the same structural position, differing in whether the design names a preference.

And it lands with a consequence worth stating: a $\gamma$-only design **cannot** claim tightness, at any
site where $\alpha$ does not exist. So section 4.4's fork and `03`'s reading choice are the same decision
seen twice, which is a connection neither file has.

## 7. Routes closed, each with the thing that closed it

**"Only the two infinity-directed rounding modes compose."** Closed by `p4.out` Q2b: toward-zero and
away-from-zero compose at 0 failures over a signed range, on three nested triples, and neither is an
adjoint to the embedding. The prediction was mine, it over-fired, and the replacement in section 2.4 is
narrower and was tested decisively.

**"A rounding mode that is not an adjoint to the embedding is not an adjoint."** Closed by `p4.out` Q1:
all six modes satisfy the biconditional against their own forced upper map, at 0 failures. The forced
candidate works exactly when the map is monotone. The useful question is what the upper map is, which is
section 2.3.

**"The Galois insertion condition separates the rounding modes."** Closed by `p1.out` Q2: all six satisfy
$\alpha \circ \gamma = \mathrm{id}$ at 0 of 320 points. It is the biconditional that separates them, not
the insertion.

**"A best abstraction can fail to exist for a mathematical reason."** Closed by `p3.out` Q1 and the closed
form in section 3.1: within the unsigned family the least containing shape is a total function into the
$(I,F)$ plane, validated at 0 disagreements over 3,163 sets. It fails only against an admission policy.

**"Closing the shape space across kinds means admitting arbitrary finite sets."** Closed by `p7.out`: the
Moore completion settles at a 16 to 34 percent enlargement across five exponent spans, reaching a fixed
point every time. It is a third family with a size.

**"The two closure conditions are two repairs to two problems."** Closed by `p3.out` Q2: they are the two
regions of one formula's codomain, needed by 1 input and 5,487 inputs respectively, with **0** inputs
needing both. They are disjoint conditions on one range.

**My own asm comparison, on its first run.** Closed by reading the emitted assembly directly: it reported
the guarded fold as differing from the bare one because it compared `LBB0_3` against `LBB1_3`. The tool
keeps the failure in its own docstring.

**An instrument killed for cost, and kept.** `p3`'s default Q1 box did not finish in 25 minutes; its
three-subset enumeration is cubic in a universe of several hundred rationals. It is unchanged in the file
and the box actually run is a command-line argument, so the file records what was attempted alongside what
was measured.

## 8. Coverage, stated honestly

**What I read.** Everything the brief named, in full, plus `seed/SETTLED_laws.md` in the four passages
bearing on the closure conditions, the lattice claim, narrowing's monotonicity and the additive and
multiplicative closure conditions. `CANON_CANDIDATE.md` was grepped for this file's vocabulary and read
nowhere; nothing here cites it. The predecessor panel's tree was counted, never read.

**The largest bound, and it is the same one `06` names.** Every Python instrument is unsigned fixed point
at radix two with zero bias, except `p7`, which adds a minimal float family. So sections 3.1 through 3.3
are about that family and nothing else. Untested: the ranged family beyond `p7`'s minimal shape, nonzero
bias, the closed-interval adjustment the record names for normalised channels, every radix but two, and
the sign domain's effect on any of it.

**The thing I most wanted to build and did not.** Section 1.5's compatibility condition, whether rounding
commutes with change of numeral in the two-level picture. Section 2.4 measures the special case where both
numerals are grids of the same family, which is the double-rounding question, and says nothing about the
general case where the fibre changes kind. That is the question that would tell the design whether
narrowing and conversion commute, and it is where I would send the next dispatch.

**Not covered at all.** Whether the closed form of section 3.1 has an analogue for the float family, which
would decide whether $\alpha_2$ is computable there too. Whether the post-fixpoint condition of section
4.1 survives a multiplicative fold, where the element type changes per step. Whether the segmented shapes
`p7` produces are expressible in the typestate under the forbidden-feature set, which is the same probe
`03` and `06` both name as owed for reading E and neither has written. Whether the graded reading of
section 6.4 has anything in it.

**The literature identification in section 6.5 is unverified** and section 5.3 says why that specific
kind of claim carries a discount in this record.

**Everything here is unpriced.** No bench harness run bears on any of it. The `p6` assembly read is an
existence claim about erasure, not a measurement. Every number in this file is a count produced by a named
command in `07_probes/RUN.md`, and none of them is a magnitude.

**Owed under the two-expert rule, listed so nothing here is mistaken for agreed.** Every section is a
first read. Where this file agrees with `03` or `06` I read both before deriving, so the agreement is
inherited rather than found; what is independent is the measurement, and it corrects my own predictions in
two places (sections 2.3 and 2.4) and reproduces `06`'s corrected product form by a different route
(section 3.3). Specifically owed a second read: section 1.4's claim that the D0/D1 split is the
fibre/index split, section 3.1's identification of the two admissions with one formula's codomain,
section 3.2's claim that the design has exactly one inference mechanism, section 4.1's sufficiency
inequality and its claim that the range half needs no bound, section 4.2's claim that the concretisation
of the saturating top is unstated in the record, section 4.3's pricing, and section 6.5's literature
identification, which I have already marked as the weakest thing here.

## 9. What appears to be op's, and in what order

Stated as questions, per `04`. None of this is a recommendation and none of it settles.

**One, and it is the cheapest and the one everything else leans on: what does the top of a saturating
numeral denote?** Section 4.2. If it denotes its own value, saturating arithmetic is exactly as unsound
as wrapping under a fold, measured at 512 of 1024 sequences. If it denotes "at least this", saturating is
sound at every trip count tried and the other two resolutions are not helped at all. The arithmetic is
identical either way; the sentence is one line; and the design's own algorithm crates already behave as
though the second answer were given, without the record ever giving it.

**Two: soundness or bestness?** Section 4.4. "The derived numeral holds every value the operation can
produce" is always true, needs no admissions, and is what correctness rests on. "And no numeral smaller
does" additionally requires the corrected product form and requires admitting both the origin and negative
integer width. **This is `06`'s third question restated as a fork with two named sides rather than as a
choice between changing a formula and changing a sentence.** Both sides are defensible.

**Three: are the two closure conditions one thing?** Section 3.1. The record has them as two conditions
doing two jobs, `03` finds them restoring existence and exactness separately, and `06` finds one of them
with a caller the other does not have. On this reading they are the two regions of a single formula's
codomain, needed by disjoint sets of inputs, with **0** inputs needing both. If that holds, the family
question's admissions stop being a lattice-closure convenience and become the statement that the shape
space contains the range of the design's own tightest-answer rule. **That is a reframing of a question
already reframed once by `06`**, and it is his to accept or refuse before anyone builds on it.

**Four: does the design want narrowing to compose?** Sections 1.5 and 2.4. Two narrowings equal one
exactly when the rounding's direction changes only at points the coarser numeral holds. Round to nearest
never satisfies that; the directed modes and round-toward-zero always do. If the design wants composable
narrowing, that is a constraint on which rounding modes a numeral may carry, and nothing in the record
states it. If it does not, the canon owes a sentence saying that narrowing twice is not narrowing once.

**Five: is the cross-kind case closed or priced?** Section 4.3. `03` establishes no admission repairs it,
which is right. This file measures that closing the shape space anyway costs a third family of segmented
numerals, sized at roughly a fifth to a third of the two it joins, with every added shape new. I am not
proposing it and reading C and reading D are both cheaper. But "impossible" and "possible at this price"
are different answers and only the first is currently on the record.

**Six, and it is a caution rather than a question.** This file agrees with `03` and `06` on more than it
disagrees, and under `RULES.md` that is worth less than it looks, because I read both before deriving. The
two places where the agreement is genuinely independent are section 3.3, where a different derivation
lands on `06`'s corrected product formula at 400 of 400, and section 3.7, where a fresh measurement
reproduces the record's wrapping result and adds substitute-zero to it. Everything else in section 3 is a
reframing of things already found, and a reframing that reads as corroboration is exactly the shape this
panel has drifted on before.
