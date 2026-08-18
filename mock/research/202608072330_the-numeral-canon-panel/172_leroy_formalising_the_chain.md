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

- The route is `171` 2.1: contextual equivalence (a definition, not a principle), plus I14's bans
  (`no dyn`, no `TypeId`, no `core::any`, no `specialization`) removing every intensional
  observation mechanism, plus one **measured** premise: at `debug-assertions = off` the binding
  perimeter and the distinguishing perimeter coincide exactly, including through an opaque
  `impl Trait` boundary whose opacity is itself proved by a refused compilation
  (`171_probes/perimeter/`).
- The maximality gives uniqueness: stretches are the equivalence classes of "connected by an
  unbound intermediate", so the partition is canonical and does not depend on spelling, which is
  what refutes the three syntactic definitions (`168` 2.2) at one stroke.
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
> `what-you-can-observe-is-what-you-guaranteed.md`, present in every member's context and declared
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
1.53, defining sentences opened, `171_probes/thirdfile/`). Of the two, `170` reports dependence on
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
the weaker provenance and the tighter bound of the pair. `167` section 1 fused them, which is the
defect `171` located; the formalised form keeps them apart.

---

## 2. The composition with `60`, exactly

`60` defines by contents: **a chain is a composition of exact operations together with a schedule
of adaptation points**, the schedule part of the function's meaning because two schedules over the
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

