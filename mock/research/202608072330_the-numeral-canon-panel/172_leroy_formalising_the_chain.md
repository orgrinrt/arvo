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

`172_probes/p3_definedness/` compiles both arms at `opt-level = 3, debug-assertions = off`:

- **Partial arm**: `a / ((b + c) - d)` with the divisor an interior intermediate; narrow
  realisation wraps the divisor to zero on constructed inputs where the wide realisation holds a
  nonzero `i64`. The narrow build **panics** ("attempt to divide by zero") where the wide build
  returns a value, with only the final value bound.
- **Control arm**: same shape, divisor an input: 0 disagreements, no panic on either realisation
  over the sweep.

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

