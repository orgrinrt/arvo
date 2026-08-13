# 81. Checkpoint seven: unit three, first half

Written by the coordinator after `76`, `77`, `79` and `80`, the first four files of the derived-algebraic-
laws unit, and the first four written under I13. Nothing here is evidence and nothing here is a decision.
It states what the four established, what they disagree about, what one of them corrected in a live
workspace rule, and the one new question that is op's rather than the panel's.

## The new question, and it is about the ratified entry's own words

### Q-C. May an arm's predicate read data, or only the typestate

I13 says the work is "a bunch of arms with **const predicates** that optimize each little 'sometimes'"
(`INTENTS.md:184-185`). The unit's two attackers found that the panel has been producing two different
kinds of law region and calling both predicates, and only one of them is const.

**Typestate predicates** are functions of the type: `F == 0`, sign domain, overflow policy,
representable-set symmetry, container width. Known at monomorphisation, gateable, erasing.

**Trajectory predicates** are functions of the values flowing through: no clamp event occurred, an operand
is zero, the running accumulator did not reach an endpoint. Known only when the program runs.

`79`'s headline result is the second kind, and `79` did not notice. Its P4 carves the holding region of
`(a+b)-c == a+(b-c)` for unsigned saturating `u8` with zero residue in both directions, and its four cases
are whether `a+b` clamped at the ceiling, whether `b-c` clamped at the floor, and in the mixed cases
whether a specific operand is zero. `80` opened `79_probes/p1_compositional_predicate_search.rs` and
reports it contains no `const fn` and no `const` item; every candidate is an ordinary function of
`(a, b, c)`. So `79:322-325`'s claim that "every predicate this file states is a compile-time reading
claim" is not right about its own result, and the coordinator confirmed the probe's shape at the source.

**This is not a demotion of P4 and the relocation is the general finding.** The question it raises is
op's, because it is a question about what his own ratified sentence licenses:

- If an arm may be selected on a value, P4 is an arm that runs where the law holds and falls back where it
  does not, at a per-datum price `80` measured (section 5.1: the value-gated form materialises **both**
  lowerings and picks with a `csel`, 13 instructions against 6 and 3 for the two static arms, so it is
  worse than either, not worse than the better one).
- If an arm may only be selected on the typestate, then P4 is a characterisation and not an arm, and so is
  `42`'s reachability condition in its value-level form, and every other trajectory region this panel has
  measured is unusable until somebody lifts its conditions into declarations. Nobody has tried to construct
  such a lifting.

A third shape exists and nobody has costed it: typestate-only for selection, with data permitted at a
declared ingest boundary, so a trajectory condition is checked once where values enter and is a typestate
fact afterwards. That is the same door Q-A opens, arriving from a different direction.

**The register carries this as Q39 and O-G.** It is put here because I13 is op's and the reading of "const
predicates" is his to give.

## Q-A: the criterion is traced, and the question turns out to have been under-specified

### The sentence is op's, verbatim, and `80` found it

`77` opened its cold derivation by refusing to treat the acceptance criterion as op's words without a
source, and asked whoever reconciled it to trace the wording. Two files went by without an answer. `80`
traced it, and the coordinator confirmed the quote at the path:

> There *is* a way to express usage through bits and bytes *and* have the typestate derive the matching
> container and numeral representations, then validate, and erase on lowering to be exactly what you
> describe before that caveat. Anything less than that, no caveats left, is unacceptable for this design
> and canon.

It sits at `seed/OLD_SETTLED_container.md:33-36`, quoting op at `135b:12-16` in the closed formalization
panel. So it is his, and it comes out of the body this panel has demoted: `RULES.md:525-548` says op's
prior calls "are not calls, not ratified intents, and not canon" and are "explicitly connected to a
*failure*", and `INTENTS.md:27-33` says the `seed/` file's own RATIFIED marking is not to be trusted and
must not be imported again.

**But op re-entered this sentence's vocabulary into this panel himself**, at `28:67-95`, answering a
question about which reading of "then validate" he meant. `80`'s reading of the status, which the
coordinator adopts: derive, validate and erase are **op's own live vocabulary in this panel for three
things a typestate does**, and "no caveats left" as a governing acceptance test is a demoted body's
sentence. The panel should keep using the three names and should stop treating the criterion as a gate.

### And the fork Q-A named is one of two axes, not the fork

`80` section 1.2 found that op has already answered a question about "validate" in this panel, and it is a
different question from the one checkpoint six put to him.

Op was given three readings and answered "Usage, Admissibility, Self-validation, All that makes sense"
(`28:82-95`, carried at `OPTIONS.md:57-88`). Those are three **things validated**. Q-A asks about the
**binding time**, compile-time-per-type against runtime-per-datum. `68` stated the relation between them
and no file in this unit had cited it: all three of op's parts are compile-time acts, and the runtime verb
validates a datum at an ingest boundary, which is not among his three at all (`68:126-129`).

So the shape is a three by two grid, op has answered on one axis, and **the panel's entire law-layer
evidence occupies one of the six cells**: usage, at compile time. Nothing anywhere has instrumented
admissibility of a law declaration, self-validation of a law, or any runtime cell.

Q-A stands unanswered and is now better posed. It is also no longer only a cost question: per Q-C above,
its answer decides whether a whole class of this panel's measured regions can be an arm at all.

## What the four established

**The mechanism both cold derivations reported as working does not reach a shipped width, and it fails in
the direction that matters** (`80`, sections 4.1 to 4.4). This is the unit's sharpest result and it lands
against its own predecessors.

The const-eval wall is a curve in (width, arity), not a width. At arity 1 rustc will evaluate width 19; at
arity 3, width 5; at arity 8, width 1. Every first refusal is `long_running_const_eval`.

At a shipped width the asymmetry runs the wrong way. Compiling the same construction at width 8 twice:
the **false** verdict is `E0080` in 0.50s total, because the evaluator hits a counterexample and stops;
the **true** verdict is refused after 4.48s, because there is nothing to stop at and the whole domain has
to be visited. The verdict that licenses an arm is the positive one. So the mechanism produces, at a
shipped width, exactly the verdict that can license nothing and refuses to produce the one that can.

Allowing the guard buys three bits and no more: widths 6, 7 and 8 accept at 5.85s, 49.06s and 370.95s, a
ratio of 8.4x then 7.6x per bit, and width 9 did not finish in fifteen minutes. The guard is not the wall.

**The escape is a closed form cross-checked on a model band** (`80`, section 4.3, at
`80_probes/p2c_closed_form_checked_on_a_model.rs`). The verdict an arm gates on is computed in constant
time from the typestate, and the closed form is cross-checked against the swept verdict at every width the
sweep can reach, at compile time, with the agreement itself an assertion. Perturbing one entry of the
closed form is refused with a named diagnostic. What stays unchecked afterwards is then one named thing,
the transfer from widths 2 through 5 to width 64, rather than the whole verdict. Two costs stated: the
cross-check takes 4.04 seconds on that host, close enough to the guard's own threshold that a slower host
may see it refuse, and a design carrying many laws pays that per law.

**A law stated as a marker is a declaration checked by nothing, and this is `68`'s hole one coordinate up**
(`80`, section 3.1). Two overflow policies declare the same `AssocAdd` marker over a four-bit signed
window; one declaration is false; the compiler raises nothing; the licensed consumer, which reassociates a
fold into a balanced tree, returns a different answer on **16,268 of 65,536 vectors**, 24.8%, with no
signal at the failure site. The arity-2 control is zero for both policies, so the instrument measures
grouping and not something else. Replacing the author-written marker with a blanket impl whose associated
const runs the law over the policy's own map makes the permission unwritable, and the false instantiation
becomes `E0080` naming the reason.

**Composed arms do not inherit their parts' predicates** (`79`, section 2, verified at source by the
coordinator). Unsigned saturating addition is associative on every triple of `u8`, predicate `any`. The
composed `(a+b)-c == a+(b-c)` fails on 82.7484% of the same domain. Four natural candidate predicates each
miss a direction; the fifth, a four-way case split on the two clamp events plus a residual operand
condition in the mixed cases, matches the holding set exactly, zero sufficiency violations and zero
necessity violations over all 16,777,216 triples. A plausible off-by-one mutant of one arm reintroduces
32,640 violations, so the zero residue is not an artifact of a check that cannot fail.

**Where a law pays is strictly narrower than where a law is true** (`80`, section 5.2, and it refuted the
probe's own thesis). The emitted assembly aliases `only_fused_f0` and `only_general_f0` to the same
symbol: at `F = 0` the backend performed the distributive rewrite itself, so in that region the arm bought
nothing. The instrument is `68`'s own symbol-aliasing check, establishing something less comfortable than
what `68` used it for. The question a law layer answers is therefore not whether the law holds, but
whether it lets the design reach a lowering the backend could not reach unaided.

**And where it does pay, it pays large** (`80`, section 5.3). A reduction of saturating additions is the
case a backend structurally cannot reassociate for itself. Inner-loop instructions per element: 6.000 for
the fold as written, **8.500 for the first law-licensed attempt**, 0.250 once the bounds proof was
supplied by iterating `chunks_exact(16)`, 0.141 with a four-accumulator unroll and a tree combine, against
a wrapping control at 0.125. The vector saturating-add instruction appears only in the arms the law
licensed. Two things to keep: the first licensed attempt was **worse than doing nothing**, because the law
was true and the bounds were not provable so the backend abandoned vectorisation; and every magnitude here
is unpriced, since no bench ran and instructions per element is not time.

**Grouping-type chain laws lift from arity 3; schedule-type ones lift from nothing** (`80`, section 4.5,
which is the author attacking its own section 4.1). If a binary operation is associative, every
parenthesisation of a chain of any length agrees, by a theorem of universal algebra, so a grouping
question's arity-n verdict is obtained by lifting the arity-3 verdict and the frontier's arity axis never
touches it. Measured: wrap disagrees on 0 tuples at n = 2, 3, 4, 5; saturate on 0, 952, 28,917, 623,049. A
schedule question does not lift and cannot: at n = 2 the two schedules are the same function, so the
arity-2 verdict is vacuous, and every higher n is a fresh statement with no lower-arity statement implying
it. **And the schedule kind is the kind I7 is stated over.** As a side effect the 952 at n = 3
independently reproduces the count `74` corrected from a consolidation that had attached it to the wrong
operation.

**`(operation, strategy)` is necessary and not sufficient** (`79`, section 5, on `63`'s own cube). Two rows
sharing sign, operation and policy differ only in **range symmetry**, which is a fact about the
representable set and not about any strategy axis assignment. So the predicate's dimension list is at
least operation, sign domain, overflow policy, fraction width, representable-set shape, and whatever named
axis a strategy resolves to. This sharpens I9 rather than contradicting it: I9 says the strategy is what
makes an answer correct, not that it is the only thing that does.

**A law verdict is invariant under any change of encoding or container** (`80`, section 8, read off
`74:144-147` and `74:507-511`). The generator's signature is a proper prefix of the five-choice sequence,
and the invariance is the part that survives a rewrite in another language in another decade. One
consequence nobody had stated: the law layer's compile-time computation is keyed on strictly less than the
container derivation's, so a verdict can be computed once per identity-and-adaptation pair and reused
across every container that pair is realised in. Whether that matters is unpriced.

**A third answer to "is the region derivable or must it be measured"** (`79`, section 3, adopting `63`'s
C6). Neither pole: the region is derived from more primitive facts, each of which is itself measured, and
the derivation composes them by conjunction. C6 was evaluated over 24 cells with zero residue and has
since survived a deliberate attack from the number-system unit. Its stated scope is a single reduction's
induced operation, and `79`'s own P4 shows a composition still needs its own derivation, which is a
boundary of C6 rather than a defect in it.

## What they disagree about

**Whether P4 is an arm.** `79` presents it as I13's own shape; `80` relocates it to a characterisation.
Both are right about what they measured and the disagreement resolves only on Q-C.

**Whether the mechanism the cold derivations built is the answer.** `76` and `77` both report derive,
validate and erase working for a law. `80` agrees it works and shows it works at model widths only, and
that `76`'s own choice of "16 values, 4,096 triples, small enough for const-eval to finish" was the
frontier being observed and reported as a nuisance to route around.

**What the pipeline's stages are.** `80` reads op's three verbs as a staging schedule with a stage missing
at each end, and adds a fourth verb: **derive, validate, select, erase**, where select is the only stage at
which a law does any work, since a law that is derived, validated and erased and then emits the same code
as if it had never been consulted has cost compile time and bought nothing. `80` marks the three-stage
framing as its own synthesis and offers it for attack. Neither cold derivation asked what the law was for.

## Corrections, and one of them is to a live workspace rule

**A rate quoted without its arity does not transfer.** `unstable-features.md` stated that const-eval "cost
quadruples per bit, reaching 28.45 seconds at eight bits", citing a source whose own name describes a
union computation, and `68:209` inherited "quadrupling per bit" while applying it to an arity-3
associativity check. A 4x per bit ratio is an enumeration of size `2^(2W)`; an exhaustive arity-3 check is
`2^(3W)` and must grow 8x, which is what `80` measured. The two figures are not in conflict, they are
about different arities, and the rule stated the ratio without stating the arity.

The rule is now corrected, with the general form carried rather than a second unqualified constant: an
exhaustive sweep costs `2^(W·k)` for arity `k`, the per-bit ratio is `2^k`, and there is no such thing as
"the" rate. The wall itself is untouched and `80` reproduces it. One live instance existed and was fixed;
the panel files that carry the old rate are historical record and are left as written, which is what `80`
already does by correcting it in its own file rather than editing `68`.

This is `74:942-943`'s lesson in a second domain. A number carries what was counted, and here what was
missing was not the operation but the arity.

**A rung correction** (`79`, section 7). `76`'s phase two claimed evidentiary weight for having re-derived
its own earlier file's conclusion under the same persona. `79` is right that this adds no third instance:
the honest tally is two independent instances for "a law is a fact about the pair", `76` and `77` cold and
blind, with `42` as an earlier, weaker, same-author third.

**A shared blind spot both cold derivations named themselves.** Neither considered the runtime reading of
"validate" at all, and both chose the same motivating bug (`UFixed<0, 8, Hot>::ONE`) from the same ambient
memory, so their agreement on the mechanism is genuine and their agreement on the example is one memorised
fact wearing two hats. `76` flagged this about its own file.

## Evidence discipline across the unit

`80` committed 39 artifacts including emitted assembly, a machine-readable frontier walk, and **two probes
committed twice, once broken and once repaired**, with a note naming each defect. One of those failures is
the exact "setup that helps" shape the workspace test gate names: the instrument returned early on the
first counterexample, so at every arity where the law is false the domain was never enumerated, and every
number it printed looked reasonable. Keeping both runs on disk is the discipline working.

`79` re-ran `76`'s and `77`'s probes rather than trusting either file's account of its own instruments, and
reports they reproduce to the digit. It also opened `42` directly rather than through either citation, and
reports both files paraphrase it accurately, which is worth stating in a panel that has caught citations
resolving to the wrong text more than once.

`76` and `77` each committed their probes with outputs. `79` did not re-execute `77`'s Python chain-error
probe and says so as a coverage gap rather than a verification.

## Still owed on this topic

Two more experts, the consolidation, and its independent check. The cheapest next instances, named by the
files themselves: constructing a lifting of one measured trajectory predicate into a declaration a
consumer would actually write, which decides Q-C from the evidence side; an attack on section 4.3's
cross-check mechanism, which `80` names as the piece it would most want broken since it is the only route
to a shipped width it found; and putting section 5.3's microkernel result on the bench harness, since
every magnitude in it is currently unpriced.
