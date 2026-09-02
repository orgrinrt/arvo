# Persona checkpoint thirteen: six calls, made in op's place

**Date:** 2026-08-04
**Position:** after `57_aaltonen_the_measurement_debt.md`, before consolidation five.
**Provenance:** same as `48b` and `53b`. Op is asleep and delegated tonight's checkpoints to his
persona at Fable tier. Every call below is **persona-decided, not op-decided**, and every one is a
single line for him to overrule. The final synthesis still waits on his word.

The persona read `49`, op's four checkpoints, both predecessor checkpoints, and files 54 through 57 in
full, and verified four claims at source before deciding: the `FromConstant` shipped impl and its doc
line, the bench-harness fix and the committed CSVs, file 08's missing modules, and both defective lines
at `49:116-117`.

## 1. The strategy axis selects the lowering door: adopted, preset table presumptive

Adopted, and it was never really open once the number landed. The ratified toolbox rule already decides
it: the strategy markers are the knobs a consumer turns on exactly this trade, and a blanket default
that quietly pins every float consumer to a 13x to 17x cost is the policer's posture the workspace rule
forbids by name. File 50's derivation stays intact and is what makes the adoption safe rather than
loose: an unpinned hardware lowering that changes delivered values is not a `Lowering` at all, so the
hardware door is reachable only as the receipt-carrying form file 50 built, where the environment
receipt is the declared requirement and the values are licensed rather than drifted. The strategy
marker selects between two legitimate doors, never between semantics and none.

What each preset carries, **presumptively, because nobody has compiled the threading**: `Hot` float
presets carry the receipt-carrying hardware lowering as their declared environment requirement, because
`Hot`'s whole identity is hardware semantics at hardware speed and 1.2 ns against 20 ns is that
identity measured. The semantics-first tiers, `Precise` certainly and `Warm` as the default, carry the
software quantiser, because the default preset must not quietly change semantics. `Cold` is a storage
axis and follows the semantics-first side unless someone shows otherwise. The mechanism is adopted; the
per-preset assignments go into consolidation five marked reasoned until a member threads `S: Strategy`
through the lowering choice and compiles it.

Two caveats ride along. The 13x to 17x is one host and one pin; the no-subnormal-cliff result is a fact
about Apple silicon rather than about the design, and an x86 target with the legacy microcode trap
changes the hardware column, not the mechanism. And the number does not close file 50's owed radix
sweep: a radix-ten quantiser cannot shift, and file 54 already flagged its cost as unmeasured. The
harness works now, so that sweep is ordered rather than deferred.

## 2. The harness fix changes the debt ledger, not the conclusions

Nothing this review concluded rested on an unrun bench. Every compile-cost figure is `--emit=metadata`
rustc timing, outside the harness. Every instruction-count claim is disassembly. File 50's quantiser
correctness was a 41-million-operation probe rather than a timing. The four pre-existing benches had no
committed CSVs, which under the workspace's own vocabulary means nothing was ever bench-validated
against them, so nothing could have cited them honestly. A grep confirming zero such citations goes
into consolidation five's verification pass.

What changes is the grading of the debt. "The bench is owed, one afternoon" was written repeatedly into
a harness that could not run any bench at all, and nobody knew because nobody had run it.
**Deferral-into-untested-infrastructure is a named failure mode from here: a deferral that names a
mechanism as its resolution path owes one run of that mechanism**, the same way a claim owes its
grounding. The owed-bench list is no longer cheap-and-someday; the excuse is spent, and unrun items
start counting as defects of the review by the standard `48b` set for the test debt.

The fix landing mid-TOPIC is legitimate: `mock/benches/` is infrastructure by the gate's own behaviour,
the bench-harness rule is exactly where that code must live, and the 654-to-655 test delta is accounted
by the correctness test it added.

## 3. Unreproducible claims get a third ground, and one dependent gets a targeted re-derivation

The review already separates "never derived" from "answered in the negative". File 57 found the third
state: derived once, derivation no longer reproducible from the committed trail. That is not struck,
because striking is for claims that were never claims; and it is not silently kept, because a ground
nobody can re-run is exactly what the grounding field exists to expose. **The ground `unreproducible`
is adopted**: the claim stays, its registry row says the committed trail cannot rebuild it, and nothing
new gets grounded on it. File 08's five-shape instruction table takes the marker in consolidation five.

One dependent forces more. File 08 is also where the const-eval width ceiling came from (28.45 seconds
at eight bits, refusal at nine), and that figure is cited in `unstable-features.md`, a ratified
workspace rule, as part of the argument that model-width validation is the only form available and that
the `specialization` and `TypeId` bans are verification infrastructure. **A workspace rule resting on a
possibly-unreproducible measurement is not a state to leave standing.** A member checks whether the
width-ceiling claim's own probes rebuild from the committed trail; they are a different artifact from
the five-shape table's, and the sweep generator is committed. If they rebuild, only the instruction
table carries the marker. If they do not, the ceiling is re-derived fresh, because a workspace rule
does not get to cite a ghost.

File 57's restraint is ratified as convention: nobody reconstructs a missing module and presents it as a
reproduction of someone else's build. The audit trail is corrupt at that point, and the honest repair is
a fresh derivation under the new member's own name, or nothing.

## 4. `FromConstant`: carried in three places, intent adopted, vehicle held for its second reads

The defect is real, verified in the tree rather than trusted from the file. It is worse than a missing
check: the trait's own rustdoc says "out-of-range constants truncate at the container", so the
partiality was noticed and papered over with a sentence instead of a type. A documented perimeter breach
is still a breach, and `what-you-can-observe-is-what-you-guaranteed` has no carve-out for admitting it
in prose.

How it is carried, given TOPIC phase locks the source:

1. **Consolidation five opens a live-defect registry section** and this is its first entry: the defect,
   the witness raw value, the tree citation, grounded `tree`, with the doc line quoted as the
   aggravation. The section exists from here on and is for tree defects as against design findings.
2. **The round's topic file records it**, so the doc CL inherits it and the mockspace trail carries it
   independently of the panel.
3. **The first act of IMPL phase is the failing test**, whole-matrix per `catalogue-edge-cases-as-tests`,
   before the fix. The probe in `55_probes/` is the durable interim artifact.

**The intent is adopted**: representability becomes a bound, the constant moves into the trait's key,
`E0277` at the call site replaces a wrong bit pattern at runtime, and the pre-1.0 churn rule covers the
break. The general lever file 55 found (wherever a numeral mismatch can be a bound rather than an
equality, the error is readable for free) goes into consolidation five beside file 56's matching
finding, because two members reached the same lever from different directions.

**The vehicle is not settled.** Per-constant impls cover what the algorithm crates write into their
where-clauses, but the full representable set cannot be enumerated as impls, a bounded enumeration is
the hardcoded threshold the toolbox rule bans, and a blanket impl gating on a computed condition walks
into the spine-rule wall file 56 confirmed a fifth time. The candidates get the two independent reads
file 55 itself asked for before the CL locks.

## 5. File 54's table edits: all three adopted, each with its named residue

**`Radix` sealed via `Rad<P>` over `AtLeastTwo`: adopted.** The open-trait spelling at `49:110` admits
two instances that falsify the float model's own founding sentence; the replacement is the shape the
review has already sealed and attacked twice, and the refusal lands at the bound. Residue: one pass, its
author's. The next stretch's float-model member runs the four-route attack as the second read. A
ratified-table edit does not stand on one file however clean, and the two-expert convention is not
suspended because the mechanism is familiar.

**`Specials` as a product: adopted.** The independence argument is mathematical and does not rest on the
witness; the witness (E4M3, NaN and no infinity, in silicon) shows the chain's missing corner is
deployed rather than hypothetical, and `InfOnly` carrying `unknown` rather than a plausible sentence
honours file 53's demand exactly. Residue: the OFP8 facts are secondary-sourced and file 54 said so, so
a member confirms the `emax` figure against the specification before consolidation five hardens it.
Same treatment for the clause 5.2 preferred-exponent characterisation, which is load-bearing and gets
the file-39 treatment against the standard's text before it calcifies.

**The `Pos` ceiling: adopted as a spec sentence.** Both walls compiled, and the attribution was done
properly: the depth-130 refusal reproduces with no `Gcd` in the chain, so it is `Pos` against the
recursion limit rather than the reduction machinery. The general fact, that the tower is a
small-magnitude encoding and a quantity needing a larger magnitude is expressed as an exponent rather
than absorbed into a rational, is the kind of never-stated constraint that costs a future consumer a
week. The `u64` readout ceiling at 2^63 goes on the open list as raised: a declarable-but-unreadable
bias denominator is a real surface question and file 54 was right not to answer it from a depth sweep.

**`49:116` and `49:117` are corrected together in consolidation five.** File 54's compile of the
`Implicit` half was needed rather than inferrable, because a single const surviving where a pair did not
was a reasonable expectation and is now a tested false one. This is the second exhibit for the
table-diff obligation inside two stretches: three sequential readers each inherited the previous
reader's scope, and a diff catches both lines at once. The obligation is executed on consolidation five
by its own author before the document stands. File 54 overturning its own file 36 sentence, quoted, with
the compile, is the review working as designed and goes into the consolidation's narrative as such.

File 54's split of the `49:881-885` open item is taken: the crossing contract has now been typed from
the outside and yielded, so the item shrinks to the two surfaces still untouched, the dithered entry
point and the membership predicate, rather than staying at full width.

## 6. The cadence call was wrong, it cost little, and the loop is restated

Plainly: deferring consolidation five was not the right reading. `53b` call 5 opens "consolidation five
is written first". The practiced loop confirms it: `48b` said the same about consolidation four, and
`49` was written immediately after `48b`, before file 50. The pattern is **checkpoint, then
consolidation, then the four.** The reading that justified the deferral matches neither the
predecessor's explicit direction nor the record one stretch earlier, and op's overnight override was
about the checkpoint mechanism rather than the cadence.

The result was four dispatches run against a base carrying two known-defective lines that every brief
had to route around, and two members spent real lines restating the known defect status. The cost was
bounded, file 54 even turned the stale base into a finding, and nothing needs unwinding. The deviation
is named because the alternative is that the loop shape becomes something each dispatcher re-derives to
taste.

**Consolidation five is written now, before anything else, absorbing files 50 through 57**, eight files,
which is the cost of the skip made visible. It carries, beyond the standing absorption: both exponent
corrections; the cost-model paragraph with file 54's numerator clause (the per-composition coefficient
is dominated by operand numerators, the one term an importer controls); the live-defect registry with
`FromConstant`; the strategy-door mechanism with its preset table marked presumptive; the
`unreproducible` ground and file 08's marker; file 54's seven spec sentences and file 56's four, folded
rather than appended; file 55's consolidation paragraph with `foldnum` and `Unbounded` marked as owing
second reads; the table-diff executed on itself; and the deferral-owes-one-run convention from call 2.

**Then the four, ordered by unblocking:**

1. **Thread `S: Strategy` through the lowering choice and compile it.** The mechanism is adopted, the
   number exists, the preset table is presumptive until this lands. The same member takes the radix-ten
   quantiser sweep in the now-working harness, since both questions share the quantiser.
2. **The `TotalOrd` fork, attempted, both forks compiled.** File 55 raised its stakes correctly: if
   `TotalOrd` is datum-level, none of the algorithm crates' outputs is law-expressible and the whole
   consumer story is stated about something the algebra cannot see. Per the `34b` posture it gets built
   both ways rather than surfaced as a question. If the compiles genuinely do not decide it, it returns
   as op's with the evidence attached. The widened-result-numeral question from file 55 rides with it,
   answered by reading hilavitkutin's use sites rather than by more type theory.
3. **The notation vehicle sketch, carrying file 56's face recommendation**, with the face-sibling
   pricing file 56 named as owed run before the macro's vehicle locks.
4. **The reads-and-audits dispatch, pinned to a name this time.** Second reads on `foldnum` and
   `Unbounded`; the four-route attack on `Rad<P>`; the membership second read, owed since file 39 and
   slipped through two stretches; the `float_algebraic` second vetting read, same; the file 08
   width-ceiling reproduction check; the OFP8 and clause 5.2 primary-source confirmations. "Whichever
   member has slack" has meant nobody twice running, so this stops being a ride-along.

Division stays held as `44b` left it. Per-application against per-value-moved stays op's, declined a
third time.

## What none of the questions asked

**File 55's reframing of file 04 is the stretch's second product, beside the harness find.** "The exile
was never the problem, the admission is" is compiled rather than argued: the crate hilavitkutin reads
for plan-stage ordering silently inverts an ordering on a four-node graph under the default-adjacent
preset, and the preset the old question was trying to readmit is the one that would have degraded
loudly instead. Every prior framing of the file 04 item, including op's own checkpoints carrying it
forward, inherited the wrong polarity. Consolidation five states the reframe at the top of the consumer
section rather than buried in it.

**The setup-that-helps finding in the shipped tests is a tree defect, not colour.** Single-digit weights
against a u8 container in `arvo-graph/tests/rank.rs`, and an L3 crate whose ten test files never touch
an arvo numeral. Both join the live-defect registry beside `FromConstant`, and the IMPL-phase work order
includes raising the rank-test weights and adding a `FastFloat` instantiation to the spectral tests.

## Loudest for op's morning read

1. **The strategy-door adoption and its presumptive preset table** (call 1). The mechanism follows the
   toolbox rule; the per-preset assignments are the persona's.
2. **Three more ratified-table edits**: `Radix` sealed, `Specials` as a product, both exponent lines
   corrected (call 5). Each is one line to restore.
3. **The `FromConstant` breaking change adopted in intent** (call 4), vehicle held for second reads.
4. **The cadence correction** (call 6). If op's own loop reading was the dispatcher's, the restatement
   dies.
5. **A workspace rule may be citing an unreproducible measurement** (call 3): `unstable-features.md`'s
   width-ceiling citation into file 08 is under a reproduction check.

## Standing

Convergence and the novelty posture hold unchanged. The intent outranks every instruction, is vague on
purpose, and is inferred rather than read literally. Only op's calls are final, and none of the above is
one.
