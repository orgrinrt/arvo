# 172. Formalising the chain

My job here is construction rather than attack. The tenth unit has argued and converged: two cold
derivations, an attack that found the unit's sixth instrument defect, two replies that between them
split the definition into a derivable partition and an underivable licence, and a third cold
derivation from an earlier topic (`60`) that all of them have now reconciled against. This file
plugs the holes, determines the bounds each surviving claim holds in, and states the thing exactly
with its predicates.

Three of the holes close with proofs rather than sweeps: the deferral optimum (section 4, closing
O-170-1 and O-170-2), the no-threshold double-rounding claim (section 5, from three enumerated
widths to `F any, M any` by constructive witness families, verified), and one hole nobody had
located, that the licence's profile bound as stated holds only for total operation alphabets
(section 3, compiled). One converged claim narrows in the formalisation and the narrowing is
stated exactly rather than absorbed.

**This file is not the candidate.** That is the next dispatch. Everything below is a suggestion;
op decides, and per I12 an opinion given before the experts converge is an ack.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` read in full across this arc, including its normative "How to read an
entry" section, and `RULES.md` in full. The assignment is licensed: I7 is op's, STATED, and ranges
over exactly this unit's object; I11 makes the compositional contracts the library's purpose; I13
(RATIFIED) is the working method throughout and argued with nowhere. The questions reserved for op
stay open below: the container premise, Q65's marker question, X1 through X4, and additionally this
unit's own op-shaped forks (which accuracy target I7 names, `168` O-168-3; which carrier ships,
Q-C1/C9's directions). Where a clause bears on I18 or I15 it names the branch it holds under.

### 0.2 Test gate: passed, at 123 across 13

Thirteen `-shared` crates, crate by crate at `--release`, `bitpack-write-contend-shared` serialised
(terminates, 15 passed) and otherwise untouched per the standing instruction.
`172_probes/run_test_gate.sh`, output `172_probes/gate_release.out`, script control armed and
firing. The four variants that fail to build on a pre-existing cause are outside the thirteen and
outside this dispatch. Bodies not re-read this dispatch; the standing reliance is the same three
mechanical scans `161` names, one of which is `154`'s over all 123.

---

## 1. The object, exactly: the partition and the licence are different kinds of statement

The unit's largest result is `171`'s split of its own definition, and the formalisation begins
there because everything else attaches to one half or the other.

### 1.1 (P) The partition. A theorem, with one measured premise and one enumeration bound.

> **Every program divides uniquely into maximal stretches of operations none of whose intermediate
> values is bound by anything outside the stretch.** Two realisations of a stretch that induce the
> same boundary function, on the same definedness domain, are contextually indistinguishable.

Provenance and status, exactly:

- The route is `171` 2.1 (the split stated at `171:77-91`): contextual equivalence (a definition, not a principle), plus I14's bans
  (`no dyn`, no `TypeId`, no `core::any`, no `specialization`) removing every intensional
  observation mechanism, plus one **measured** premise: at `debug-assertions = off` the binding
  perimeter and the distinguishing perimeter coincide exactly, including through an opaque
  `impl Trait` boundary whose opacity is itself proved by a refused compilation
  (`171_probes/perimeter/`).
- The maximality gives uniqueness: stretches are the equivalence classes of "connected by an
  unbound intermediate", so the partition is canonical and does not depend on spelling, which is
  what refutes the three syntactic definitions (`168` 2.2; its own definition at `168:168-170`)
  at one stroke.
- **Bounds.** The measured premise is enumeration-bounded: six channels tested, two found (both
  governed by `debug-assertions`), four named and untested (floating-point environment,
  `#[track_caller]` location data, backtrace symbols, linker artifacts), per `171` O-171-2. And it
  is profile-bounded and **alphabet-bounded**, which is section 3.

`holds for: (argument half) any program in the language under I14's bans, threads any; (measured
premise) rustc 1.98.0-nightly (57d06900f), editions 2021 and 2024, aarch64-apple-darwin,
debug-assertions = off, total operation alphabet {+, wrapping -, *}, carriers i32/i64, channels =
the six enumerated in 171_probes/channels/, threads = 1.` The two halves carry two predicates, per
the argument/enumeration split `171` 5.1 named as a class.

### 1.2 (L) The licence. Normative, underivable, and bounded twice.

> **Within a maximal unbound stretch, the design may select any realisation that induces the
> stretch's boundary function on its definedness domain.** This is a licence, not a theorem: it
> does not follow from (P), and three derivation routes (from I15, from I13, from I3/I18) fail for
> the stated reasons (`171` 2.3). It rests on the principle that obligations are bounded by the
> observation surface, which is the auto-loaded workspace rule
> `what-you-can-observe-is-what-you-guaranteed.md` (line 4 of the measured set,
> `157_probes/loaded_rules_157.txt:4`), present in every member's context and declared
> by none of the three derivations until the attack found it.

Two bounds, and both are exact:

- **The profile bound.** A binding-free distinguishing channel exists at `debug-assertions = on`
  (the overflow panic, and const evaluation, which follows the assertion flag and not the
  opt-level), and none was found at `off` among the channels tested. So (L) is **false in a
  development build and true in a shipped artifact**, which is precisely I18's build bound reached
  from a direction I18 was not derived from. That is a convergence between a measured fact about
  the language and an op intent, and it is stated as one: the region's freedom and I18's
  dev-only panic have the same boundary because they are the same boundary.
- **The definedness bound, which is this file's addition** (section 3): with a partial operation in
  the alphabet, a distinguishing channel exists at **every** profile, so the licence's "induces
  the boundary function" must be read over values **and definedness**, not values alone. The
  unit's statements were measured on total alphabets and are exact there.

**The rung, carried forward exactly as the replies left it and as the brief instructs.** The
observation-bounded definitional convergence is **two instances, not three**: `60`'s definition is
not observation-bounded at all (observation vocabulary at 0.27 per thousand words against 1.50 and
1.53, the table at `171:249-253`, defining sentences opened, `171_probes/thirdfile/`). Of the two, `170` reports dependence on
the rule for its whole delimiter, and `171` reports a split: rule-free for (P) with the route
shown, dependent for (L). So the panel's record is: **(P) at two instances, one carrying a
rule-free derivation; (L) at zero independent instances, resting on the rule**; and `60` is a
definition of a different shape that composes (section 2). O-169-2/O-171-4, the cold dispatch with
the rule removed, remains the decider and remains open.

### 1.3 The relation, in one sentence each way

(P) without (L) is a fact nobody may act on: the realisations are the same program and the design
still has no licence to roam among them. (L) without (P) is a licence with no perimeter: nothing
says where the roaming stops. The chain's design story needs both, they have different provenance,
different bounds, and different failure profiles, and a canon sentence that fuses them inherits
the weaker provenance and the tighter bound of the pair. `167` section 1 fused them in one sentence (`167:127-131`), which is the defect `171` located;
the formalised form keeps them apart.

---

## 2. The composition with `60`, exactly

`60` defines by contents (`60:28-29`): **a chain is a composition of exact operations together
with a schedule of adaptation points**, the schedule part of the function's meaning because two schedules over the
same ops compute different functions (three schedules, three functions, `60` probe A). The unit
defines by bounds. The two compose with no residue, and the composed statement is the one the
candidate should carry:

> **A chain's extent is a maximal unbound stretch (1.1); its content is a set of exact operations
> with a schedule of adaptation points (`60`); and the boundary theorem relates them: an
> adaptation point on an edge whose value is bound is forced and part of the meaning; an
> adaptation point on an unbound edge is free and is the design's to place under (L).** The
> pipeline whose intermediates are all stored is the degenerate case where the schedule is fully
> forced (`168`'s kind 3 restated in `60`'s vocabulary), and the single operation is the
> length-one stretch whose interior schedule is empty.

Everything else the unit and `60` established slots into that frame without adjustment:

- **The five obligations** (`60` section 3, with `168`'s five and `167`'s three mapping onto them,
  three shared): an intermediate format, a schedule, an association and order statement, a count
  bound, and an error bound **composed per adaptation point**, the schedule being the index set of
  the error sum, so fewer adaptation points is a structurally shorter error sum and not merely
  cheaper rounding. The fifth has no counterpart in either cold file and both said so; it is
  carried at `60`'s single instance.
- **The grades** (`60` section 4): composite correct rounding (a), stepwise (b, the only
  compositional grade and its ceiling), bounded drift (c), and structural exactness (s), fixed
  point's own possession, which converts chain correctness from analysis into bookkeeping inside
  the width algebra.
- **The window** (`60` section 5): addition composes headroom logarithmically, multiplication
  composes width linearly, so the exact multiplicative intermediate is real and not worth holding;
  the window factors a chain into `ceil(k/w)` adaptations; its capacity is static. Bounded by
  `168` N6: the capacity formula is a function of the ordered sequence once the window is
  heterogeneous (commuting steps needing different per-step widths, 24 orderings spanning 11 to 13
  bits), so the formula holds for homogeneous windows and the derivation for a mixed window
  consumes the flattened ordered sequence, which is `43`'s flatten-before-deriving answer arriving
  at the same place from the aggregate side.

---

## 3. The hole this file adds and closes: the licence's definedness clause

**The claim under formalisation.** `171` section 3 bounds (L) by build profile: binding-free
channels exist at `debug-assertions = on` (overflow panic, const eval) and none at `off`. Its
alphabet was `{+, wrapping -, *}`: **every operation total**. Rust has partial operations whose
refusal is not governed by `debug-assertions`: division and remainder by zero panic in every
profile. If an interior realisation choice can move a divisor onto zero, a binding-free
distinguishing channel exists at `off`, and (L) as stated is false in a shipped artifact too.

**The case that must fail, declared before the run.** The control arm replicates `171`'s total
alphabet (the divisor is an input, not an intermediate); its two realisations must be
indistinguishable at `off` on every input including the wrap-exercising ones. If the control
distinguishes, the instrument is detecting something other than the partiality and proves nothing.

`172_probes/p3_definedness/` compiles both arms at `opt-level = 3, debug-assertions = off`
(`definedness_off.out`, with the `on` head beside it):

- **Partial arm**: `a / ((b + c) - d)` with the divisor an interior intermediate, at `a = 0` so
  the witness is a **pure definedness difference**: `0 / t = 0` for every nonzero `t`, so on that
  slice the two realisations agree on every input where both are defined and differ only in
  **where** they are defined. At the constructed input (divisor exactly `2^32`: nonzero exact,
  wrapped `i32` zero) the narrow realisation panics and the wide one returns a value, at
  `debug-assertions = off`, with only the final value bound.
- **Control arm**: same shape, divisor an input: 200,000 inputs, 0 disagreements, 0 panics on
  either realisation. The control does not distinguish, so the instrument detects the interior
  partiality and nothing else.
- **And the trap the clause exists to close, demonstrated in the same run**: a value-only
  equivalence checker that skips inputs where a side panics, which is what a `catch_unwind`
  harness naturally does, certifies this pair from 200,000 random inputs (0 disagreements, 0
  skipped, because a random divisor wrap is a `2^-32` event), and the constructed input refutes
  the certificate. A licence checked by value agreement alone would ship a panic.

**The formalised clause:**

> The boundary function that (P)'s equivalence and (L)'s licence quantify over is the function
> **with its definedness domain**. Two realisations agree only if they agree on where the
> computation is defined. A partial interior operation imports a definedness constraint that binds
> at every profile; the overflow-panic channel is `debug-assertions`-gated and the partiality
> channel is not. The unit's measured statements hold exactly as recorded over total alphabets,
> and the licence for a stretch containing a partial operation is (L) with agreement-on-definedness
> as part of the boundary function.

This is a narrowing of the converged text, not a refutation: `171`'s finding was predicated on its
alphabet and its own notation already said it held nowhere else. What was missing was that the
generalisation a reader would naturally make is false, and the clause that makes it exact.

`holds for: rustc 1.98.0-nightly (57d06900f), edition 2021, aarch64-apple-darwin, opt-level = 3,
debug-assertions = off, alphabet {+, -, /} with the divisor interior for the partial arm and input
for the control, carriers i32 narrow and i64 wide, threads = 1. The clause itself is an argument
over the language's definedness rules and carries: partial operations in {div, rem}, profile any,
threads any.`

---

## 4. The deferral optimum, from a sweep to a theorem

**The claim as the unit left it.** Full deferral is pointwise optimal where the boundary resolution
is a nearest-point projection: zero counterexamples over every placement (after `169` widened `168`'s
two-placement search and `170` reproduced the widening on its own instrument), with matched
non-nearest controls firing at 317, 395 and 443 win-chains. `170` O-170-1 asks whether it is
provable rather than swept; O-170-2 asks whether it survives adversarial tie-breaking or
non-idempotence. **Both close, by the same proof.**

### 4.1 The theorem

> **Setting.** A chain of total steps `f_1 .. f_n` over exact values; a boundary format whose
> representable set is a nonempty `S`; a boundary resolution `pi` that, for every exact `x`,
> returns **some** element of `argmin_{s in S} |s - x|` (any fixed selection; the tie rule is
> arbitrary). A **placement** applies resolutions at any subset of interior edges and applies `pi`
> at the boundary; the fully deferred placement applies only the boundary one.
>
> **Theorem.** For every chain, every placement `m`, and every input `x`:
> `|pi(exact(x)) - exact(x)| <= |out_m(x) - exact(x)|`.
>
> **Proof.** Whatever `m` does in the interior produces some value `x'` at the boundary, and
> `out_m(x) = pi(x')` is an element of `S`, because the boundary resolution fires last in every
> placement. The deferred output is `pi(exact(x))`, which attains `min_{s in S} |s - exact(x)|` by
> the definition of a nearest-point selection. An arbitrary element of `S` cannot be closer than
> the minimum. Qed.

Three consequences of the proof's shape, each of which the unit's material had as an open question:

- **The tie rule is irrelevant and idempotence is a consequence, not a hypothesis** (O-170-2
  closed): the proof uses only that `pi(x)` is in the argmin, so any fixed selection qualifies,
  adversarial ties included; and for `s in S` the argmin at `s` is `{s}`, so every such `pi` is
  idempotent for free. What the hypothesis genuinely excludes is a resolution that is not a
  function of its argument at the boundary, and a design that admits a history-dependent boundary
  resolution has left the theorem's setting and every sweep's too.
- **The boundary of the theorem is the resolution, not the chain** (`168`/`169`/`170`'s measured
  half, carried): a non-nearest boundary projection gets nothing, and the counterexample counts
  are already measured (truncate 317/13527, wrap 395/21108, coarse truncate 443/17100).
- **Interior resolutions are unconstrained by the theorem**: it compares outputs at the boundary
  and says nothing about which interior placement is cheapest, which is where the harness's
  crossover results live and remain.

### 4.2 The theorem checked at the generality of its hypotheses

The unit's sweeps drew `S` as grids and ranges. `172_probes/p1_deferral_theorem/` draws `S` as
**random subsets** (sizes 2 to 64 of a 256-value space) with **adversarial fixed tie-breaking**
(a per-set hash selection), random chains of depth 2 to 5 over the unit's alphabet, every
placement, every input. The case that must fail was declared before the run: a downward projection
onto the same random sets must produce placement wins, or the harness cannot see one.

```
nearest (random S, adversarial ties): 0 win-chains, 0 win-inputs, 320 exercised of 400
CONTROL down-projection onto same S : 145 win-chains, 15304 win-inputs
C1 PASS   C2 PASS (control fires)   C3 PASS (placements move outputs)
```

With the proof in 4.1, this run and the unit's three are **confirmations of a theorem rather than
evidence for a generalisation**, which is the reclassification O-170-1 asked for.

`holds for: (theorem) S any nonempty finite set, pi any fixed nearest-point selection, chains any
composition of total steps, placements any, inputs any, F any, W any, family any, threads any; an
argument, marked as such per Q65's open distinction. (confirmation runs) the predicates already
recorded at 168 7.1, 169 2, 170 3, and this probe's header.`

---

## 5. The no-threshold double-rounding claim, from three widths to a theorem

**As the unit left it.** `167` 4.1: no intermediate width `M` strictly between `F` and `2F` gives
zero double-rounding disagreements, enumerated at `F in {6, 8, 10}`; `169` extended to `F in
4..=10` and found the closed form `2^(F-1)` at `M = 2F-1`; `171` 5.2 characterised the disagreeing
set exactly (tie-adjacency: `a*b mod 2^(F+1) in {2^(F-1)+1, 3*2^(F-1)-1}`) and named the class:
where a finding carries an argument and an enumeration, the predicate takes the weaker half unless
the author separates them. The argument half was stated at `F any` and its "some such pair lands on
a rounding boundary" clause was asserted rather than constructed.

**The construction, which discharges the clause.** Three explicit witness families, one per band
of `M`:

> - `M = 2F-1`: the pair `(1, 2^(F-1)+1)`. The product is odd and tie-adjacent: the first rounding
>   (grain 2) ties and goes to `2^(F-1)` by ties-to-even (its neighbour `2^(F-1)` is divisible by
>   4 for `F >= 3`); the second rounding ties at exactly half of `2^F` and goes to `0`; the single
>   rounding of `2^(F-1)+1` goes to `2^F`. `0 != 2^F`, for every `F >= 3`.
> - `M = F+1`: the pair `(1, 3*2^(F-2)-1)`. The product sits just below the midpoint between
>   `2^(F-1)` and `2^F`; the first rounding (grain `2^(F-1)`) pulls it down to `2^(F-1)`, the
>   second ties to `0`; the single rounding goes to `2^F`.
> - `F+2 <= M <= 2F-2`: the pair `(2^(2F-M-1), 2^(M-F)+1)`, product `2^(F-1) + g/2` with
>   `g = 2^(2F-M)` the first grain: exactly on a first-level tie whose even side is `2^(F-1)`
>   (even because `M >= F+2`), which then ties to `0` at the second rounding, against `2^F` for
>   the single rounding. Operands in range for `M <= 2F-2`.

**Verified rather than trusted** (`172_probes/p2_witness_families/`): all 65 `(F, M)` cells for
`F in 3..=12` disagree; CONTROL A, the on-tie family `(1, 2^(F-1))`, declared before the run as
the case that must fail, agrees on 10 of 10 cells, so the checker can tell a witness from a
non-witness; CONTROL B reproduces `169`'s exhaustive `2^(F-1)` counts at `F in {4,6,8,10}`
exactly, so this model is the unit's model; and a full enumeration at `F in 3..=6` confirms every
interior `M` nonzero.

> **Theorem.** For every `F >= 3` and every `M` with `F < M < 2F`, per-operation correct rounding
> at intermediate width `M` does not compose into chain-level correct rounding: a two-operand
> product exists whose double rounding differs from its single rounding. **There is no threshold
> below exactness.**

`holds for: F >= 3 any (constructive witnesses, verified at F in 3..=12 and exhaustively at F in
3..=6), M any with F < M < 2F, rounding = nearest-ties-even at both roundings, operation =
fixed-point multiply, signedness = unsigned, threads any; the theorem half is an argument, the
verification runs carry the enumerations named. F in {1, 2} have empty or degenerate interior
ranges and are outside the claim.`

This is what `167` 4.1's sentence "a chain-level accuracy guarantee cannot be bought by
strengthening the per-operation guarantee" rests on, now at the width the design ships rather than
at three model widths, and `171` 5.2's tie-adjacency characterisation at `M = 2F-1` stands beside
it untouched.

---

## 6. The two deletion licences, exactly

Deleting an interior resolution is a rewrite and needs a proof. The unit established two kinds and
their independence (`168` 4.3, four firing controls), and the formalised statements are:

> **(A) The range licence.** Every intermediate provably lies where the resolution is the
> identity, so each interior application is a no-op. Premises: widths and a static bound. Reads
> nothing about which operations compose. This is the licence `109` P5's carried range, `82`'s
> operand window, the interior-safety predicate and `60`'s exactness predicate all instantiate,
> and it is the half `109`'s not-an-endomorphism proposal mechanises.
>
> **(B) The algebra licence.** The resolution commutes with, or is absorbed by, the composition,
> so interior applications may be deleted whatever the values are. Premises: the operations and
> the resolution. Reads no bound. Wrapping over ring-affine steps is the standing instance.
>
> **Independence**: each holds where the other fails (an affine wrap chain with 3637 of 4096
> inputs out of range; a rounding resolution with every intermediate on the grid), so they are two
> arms with two predicates and neither subsumes the other.
>
> **(B) is a conjunction over every step.** One `saturating_sub` swapped into an affine chain
> revokes it with endpoints, widths and depth unchanged. Consequently **whatever carries a chain's
> licence must see every step and be composed as the chain is built**; a carrier computed from the
> endpoint types cannot express it. This is the structural twin of the primitive topic's
> per-construction transformer result (`164` clause 12's "neither is inherited"), and the twin
> relation is stated as a shape, not claimed as an instance.

And the licence taxonomy connects to `60`'s carriers exactly as `168` N1 put it: `60`'s exactness
predicate is licence (A) alone, so a format concept carrying `60`'s three things can state (A) and
has nowhere to put (B). That gap is the same statability shape `60` established for I7, one level
down, and it is what the candidate's carrier clause must answer.

---

## 7. The predicate dimensions and the harness results, carried with their profile

A chain adds **depth, shape and arity** to the predicate dimensions a single operation has, and
each is measured to flip the winning arm on the committed harness: the reassociation family's
crossover at the lane count (parity below, 6.85x at it, 177.8x at depth 1024); the fold and the
elementwise chain taking opposite signs on the same widening lever at the same width; the arity
sweep moving winner and spread continuously, gated by width where no rung exists below `u128`.
The mechanism behind the fold's side is isolated and profile-invariant: **the projection on the
loop-carried accumulator is what blocks vectorisation, and the projection on the per-element value
costs nothing** (`168` 12, byte-identical disassembly at both codegen profiles).

Three carriage notes, each already established and each binding on the candidate:

1. **Every harness figure in this unit carries the profile amendment of `168` 12b**: `codegen
   profile = cargo default release: opt-level = 3, lto = false, codegen-units = 16`, not the
   documented fat-LTO profile, and no committed number in that directory is reproducible by
   construction until `117`'s before-and-after run exists. The mechanism results are established
   at both profiles; the magnitudes are established at the default one.
2. **The attribution**: of the 178x reassociation win, the bounds proof alone accounts for at most
   1.30x and the reassociation for the rest (`nolaw` arm), which is why the associativity proof is
   worth carrying in a typestate at all.
3. **The hardware axis**: the cliff at `L = 16` is one vector register on the measured host, a
   predicate dimension the width/strategy/signedness list has no place for (Q-C5, open, closed by
   one run on non-NEON hardware).

And the family axis: **in fixed point, association order has exactly zero accuracy content**
(bit-identical reassociation at every size swept) **and is a speed lever worth up to 178x; in
relative precision it is an accuracy lever worth up to 94.8x** (`167` section 8, with the
overflow-policy dimension of its zero named by `60`'s order-dependence result and recorded in
`167` R12: the fixed-point zero's region is "no adaptation on the additive side occurs", since a
saturating fold is order-dependent where saturation is reachable). A canon sentence about
reassociation that does not name the family and the reachability of the resolution is wrong for
someone.

---

## 8. The graph case, exactly

- **The carrier joins with no excess**: a shared node's value is one value, so its width
  requirement is one number and the join over consumers is a maximum (`168` p6, closing its own
  O-168-1's carrier half negatively).
- **The schedule does not join**: a shared node has one schedule and its consumers can disagree,
  the disagreement forced by carrier capacity; no path-shaped analysis reports the loss, because
  along each path the chosen schedule is the best available to that path. In `60`'s vocabulary:
  **under sharing, a term does not factor into windows uniquely** (`168` T1).
- **The conflict band has a closed form**: `[R, E-1]` of width `E - R`, where `E` is the losing
  branch's exact requirement and `R` its requirement with the shared node resolved; entailed, not
  swept, generic over constructions (band appears iff `E > R`), and it becomes a measured curve
  exactly when the losing branch's own width interacts with the carrier (`169` 3, `170` 9).
- **The residue is three resolutions, all costing something**: materialise twice, resolve and let
  one branch lose, or refuse the region and make the consumer split it; which ships is a design
  call and the third is the one that keeps the loss visible (`168` 18's residue, carried as a
  residue).

So the candidate's delimiter vocabulary should be the region between observations, with the chain
as its path-shaped case, a single operation as its one-node case, and the sharing conflict as the
one genuinely new obligation a DAG adds: **not a wider carrier, but a schedule decision at every
shared node**.

---

## 9. The rungs, carried exactly, including the correction of the correction

The brief instructs that `171`'s count correction not be undone, and it is carried here in the form
the candidate should compress:

- **The locus finding** (the guarantee cannot live in a per-value object; a closed concept makes
  I7 unstatable): **three independent routes plus a corroborating pair.** `60`/`63`, `90` and
  `109` are the three routes from three topics; `167` and `168` are the pair, dispatched on one
  premise set, with both exact heading matches between them dictated through the shared context
  (`170` 10) and the shared premise set named by both.
- **The observation-bounded definition**: **two hats, not three.** `60`'s definition is of another
  shape (contents, not bounds; observation vocabulary 0.27 per thousand words against 1.50 and
  1.53, defining sentences opened at `171_probes/thirdfile/`), and it composes with the partition
  rather than repeating it, which is worth more than a third instance would have been.
- **Within the two**: `170` reports dependence on the observability rule for its whole delimiter;
  `171` reports a split, rule-free for (P) with the route shown and measured, dependent for (L).
  So **(P) carries two instances, one with a rule-free derivation; (L) carries none and rests on
  the rule.** O-169-2/O-171-4 (the cold dispatch with the rule removed) is the decider and stays
  open.
- **The deferral theorem**: established as a proof here (section 4); its sweeps are `168` (two
  placements), `169` (every placement, one instrument), `170` (every placement, second instrument,
  second language), and this file's random-set run. Sweeps confirm; the proof carries.
- **The no-threshold theorem**: `167` enumerated, `169` extended and found the closed form, `171`
  characterised the mechanism, this file constructs the witnesses. One chain of work, four files,
  and the theorem's rung is the argument's, not a count of sweeps.

## 10. O-171-1, answered as the second reader it asked for

`171` O-171-1 asks a second reader to take step 2 of its route alone ("choosing between
contextually equivalent implementations is not a change to the program") and say whether it can be
asserted without any claim about what the design owes. **My verdict, formed from the definitions
before rereading `171`'s own defence: step 2 is descriptive, and the split stands.**

The reasoning, so the agreement is checkable rather than an echo: contextual equivalence with a
fixed observation basis is the standard extensional identification of programs, and the basis in
`171`'s route is not chosen by the designer; it is **whatever a context can do in the language at
the given profile**, which is a fact about the language (and is exactly why the basis shifts with
`debug-assertions` and why partiality sits in it at every profile, sections 1.2 and 3). Where a
normative choice could hide is in choosing the basis; here the basis is inherited, so none does.
What step 2 cannot supply, and does not claim to, is that the design **may** choose among the
identified implementations; that is (L), and `171` already locates the normative content there.

So the two-expert state of the split: `171` proposed it and could not certify its own philosophical
premise from inside; this file, having derived its reading from the definitions first, concurs
with the stated reason. That is a second read in the sense the option requested, and it does not
retire O-169-2's empirical test, which remains the stronger decider.

## 11. Corpus findings, for the record the next unit inherits

**C-X1. Nine instrument defects in one unit, and the last two name classes the first seven cannot
be caught by.** Mechanical (seven: a counter that cannot return nonzero, a hardcoded label, a
wrong-dialect regex, a pre-LTO listing, a line-break-blind `grep -F`, markup surviving whitespace
normalisation, case sensitivity on a mid-sentence quote), **scope** (`169` 2/`170` 2: the search
covered two placements while the claim quantified over all; no control can catch it, because a
control tests whether an instrument measures what it points at, not whether it points at the whole
claim; the check is reading the claim's quantifier beside the loop bound), and **harness** (`171`
3.1: a 2x2 looped over quoted flag strings in a shell that does not word-split, every cell
agreeing; the tell is a table whose every cell agrees, and the control is one cell that must
differ). Both new classes were considered in this file's probes: each probe's claim quantifier is
matched to its loop bounds in the header, and each multi-cell run carries a cell that must differ.

**C-X2. Where a finding carries both an argument and an enumeration, the predicate takes the
weaker half unless the author separates them** (`171` 5.1's class, now applied three times in this
unit: `167` 4.1, `168` F8's placement dimension, and the profile amendment). The honest form is
two predicates, one per half. This file writes every theorem that way and it is the interim
practice pending Q65's marker, not a settlement of it.

**C-X3. The value-only equivalence trap** (section 3): an equivalence checker that skips inputs
where one side panics certifies pairs that differ in definedness, and 200,000 random inputs found
zero occurrences of a `2^-32` event. The licence's checking discipline needs the definedness
column, and a sweep's zero on a rare-event channel is a claim about the sweep.

## 12. What I could not formalise, and why each wall is where it is

1. **(L) itself.** It is normative and underivable; the formalisation states it as a licence
   resting on a named principle with two exact bounds, which is the most that can be done without
   op or a convergence. The rule it rests on is workspace canon of the presumed-wrong rung for
   arvo's own purposes; whether the canon adopts the principle as an arvo intent is op's, and the
   candidate should put it beside the container premise rather than absorb it.
2. **Which accuracy target I7 names** (`168` O-168-3) and **which carrier ships** (Q-C1/C9's three
   directions, with the doability check run for the third at `167` R10 and the backward-facts
   discriminator separating them). Op's, and the unit's material prices the distinction without
   deciding it.
3. **The remaining profile-bound residue**: four untested binding-free channel candidates
   (O-171-2). The definedness clause closes the partial-operation gap; the four remain named and
   untested, and (P)'s predicate carries the enumeration bound.
4. **The fifth obligation's composition with the deferral theorem**: `60`'s error sum is a
   backward-style bound indexed by the schedule; section 4's theorem is a forward optimality
   statement about one schedule. Whether they compose into a bound-per-schedule with the deferred
   point as its minimum is plausible and unbuilt, and it is the cleanest next probe this topic
   owns.
5. **Anything priced.** No harness ran in this dispatch; every magnitude carried is the unit's,
   at the profile named in section 7; Q-C3's cost half, Q55, and the const-time accumulability of
   the licence conjunction (`168`'s "could not") stay open.

## 13. What I am carrying forward unchanged, and from whom. Count: fourteen.

1. `60`'s definition-by-contents and the schedule-is-meaning result (probe A, three schedules,
   three functions). Clause frame, section 2.
2. `60`'s five obligations, with `168`'s and `167`'s mapped onto them. Section 2.
3. `60`'s grade taxonomy (a, b, c, s), with grade s as fixed point's own possession. Section 2.
4. `60`'s window and its cost accounting, bounded by `168` N6's heterogeneity result and `43`'s
   flatten-before-deriving answer. Section 2.
5. `171`'s (P)/(L) split with its three failed routes to (L), formalised in section 1 rather than
   amended.
6. `171`'s I18 convergence (the binding-free channels at `debug-assertions = on` land on op's own
   build bound). Section 1.2.
7. `171`'s two-hats correction and the thirdfile measurement. Section 9, per the brief.
8. `168`'s two-licence split and its conjunction-over-steps result, with the four firing controls.
   Section 6.
9. `168`'s carrier-has-no-local-answer result with its commuting control, and its p6 join-is-max
   result. Sections 2 and 8.
10. `169`'s closed form for the conflict band and `170`'s measured variant. Section 8.
11. `169`'s scope-defect finding and `170`'s concession with the category statement. Section 11.
12. `170`'s wrap control (the strongest matched pair) and its coarse-grid closure of O-169-1.
    Section 4's boundary row.
13. `168` 12's accumulator-projection mechanism result with its profile invariance, and 12b's
    profile amendment. Section 7.
14. `167`'s backward-narrowing licence and bit count, and its correlation finding, both still at
    one expert and named as asking for their second read rather than counted as settled. Section
    7's carriage is of the harness results; these two remain the unit's most valuable unattacked
    claims and the next attack's first targets.

**Amended in the carrying: one.** `167` 4.1 and its chain of extensions, upgraded to a theorem by
construction (section 5), which is a widening stated in this file per the never-widen-in-place
rule; the enumerated halves stay exactly where their files put them.

## 14. What only op decides, unchanged and extended by one

The container premise, Q65's marker question, and X1 through X4 stay open exactly as `164` carries
them; nothing in this unit touches them. This topic adds to the queue rather than closing
anything: **which accuracy target I7 names** (the placement question is now priced at 15.5x
aggregate and 16x worst-case at depth 5, and the theorem in section 4 says which placement is
distinguished, not which target is meant), **whether the observability principle becomes an arvo
intent** (the ground (L) needs), and **which chain carrier ships** (the three directions, with
doability established for the third and the backward-facts discriminator separating them).

## 15. Coverage, bounded honestly

**Read in full this dispatch:** `166`, `167` (both phases and R12 through R14), `168` (all three
passes), `169`, `170`, `171`, `60` sections 1 through 8 and its phase-two verdict and section 1,
`43` section 0, `AGREEMENTS.md` sections 6 and 12, `OPTIONS.md` Q42 and Q54, and `164` (mine,
reread at the sections cited).

**Not read:** `60` phase two sections 2 through 7 beyond the verdict; `43` sections 1 through 10
(its section 0 is its author's summary, the single-source shape `168` pass three names, and my use
of it is bounded to the flatten answer `168` 24 verified); `63`, `90`, `92`, `106` at source (their
chain material reaches me through `AGREEMENTS.md` and the unit's own verifications); every panel
file outside this unit's reading list.

**Reproduced or verified rather than taken:** the deferral zeros and control counts
(independently re-derived on a fresh instrument at wider hypotheses, section 4.2); `169`'s
`2^(F-1)` counts (CONTROL B, exact); the unit's model conventions (CONTROL B is the check that my
model is theirs); the definedness bound (built, both profiles). Not re-run: `171`'s channels and
perimeter probes (their outputs are committed and my section 1 carries their predicates as
recorded); `168`'s harness readings (carried with the 12b profile amendment).

**Which sections would move if something I leaned on is wrong.** Section 1 rests on `171`'s
measured premise as committed; if a fifth channel among the four untested ones distinguishes at
`off`, (P)'s predicate narrows and (L)'s profile bound gains a second exception alongside
definedness, and the clause structure absorbs both without rewriting. Section 2's window carriage
rests on `60`'s own file at the sections read. Section 5's theorem is self-contained modulo the
model convention, which CONTROL B pins to the unit's. Section 7's magnitudes inherit `117`'s
profile situation in full.

**Citations and quotations, checked by opening them.** `172_probes/citecheck.out`: every
`file:line` anchor opened and read. `172_probes/quotecheck/`: every verbatim quotation matched
under the unit's three-layer normalisation (whitespace, markup, case), with a planted-absent and a
planted-present control both behaving. The measured layer report: four of six quotations match
raw, one needed L1 (a wrapped line), and one needed L3 (a mid-sentence lowercased leading capital,
which is exactly `170`'s eighth defect class biting this file and being caught by the adopted
fix). L2 moved nothing here, and per `171` section 9 that zero is a quoting-style fact, not
evidence the layer is dead.

**What this file settled.** The deferral optimum as a theorem with its hypotheses and tie-rule
irrelevance (O-170-1 and O-170-2 closed). The no-threshold claim as a theorem at `F any` by
constructive witness families, verified at 65 cells with both controls firing. The licence's
definedness clause, compiled at `debug-assertions = off` with its value-only-checker trap
demonstrated. O-171-1's second read, delivered with independent reasoning.

**What it moved.** (P) and (L) from one fused sentence to two statements with separate provenance,
separate bounds and a stated relation; the unit's definition and `60`'s from rivals to one
composed frame; the rung ledger to the exact form the candidate should compress.

**What it could not.** Section 12's five items, each with why the wall is where it is.
