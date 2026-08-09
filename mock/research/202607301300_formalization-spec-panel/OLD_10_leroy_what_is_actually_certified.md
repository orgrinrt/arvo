# Panel 10: what is actually certified, and what is merely trusted

**Persona:** Xavier Leroy, certified-compilation and trusted-computing-base lens. Tenth member; read
`01_knuth_mathematical_rigour.md` through `09_chlipala_enforcement_and_attack.md` in full, all three
op checkpoints (`04b`, `06b`, `08b`), and the probe directories `02_probes/` through `09_probes/`
including every README, before starting.
**Date:** 2026-07-30

**What I read in full:** the spec (`202607301200_topic.the-formalization-spec.md`), all twelve prior
panel files, the panel brief, the governing panel rule (`panels-argue-the-intent-not-the-wording.md`),
`08_probes/a_union.rs` line by line at the surfaces this dispatch is about (the `Resolve`/`Rec`
machinery at lines 47-222, the carriers and deliveries at 229-337, the law and aggregate at 630-698),
`09_probes/README.md` and both of 09's root probes, `07_probes/a_witness_typestate.rs` at its trait
declarations. **What I read in part:** the talk and the inherited-state file at the passages the spec
and prior members cite; `arvo/tests/strategy_semantics.rs` and `arvo-strategy/src/identity.rs` at the
lines prior members cite, re-read in my own hand where a claim below leans on them.

**Directory listing done** across `mock/design_rounds/` (the three flat files at root are this round,
nothing newer), `mock/research/` (nothing postdates the panel directory), `mock/research/sketches/`,
and the panel directory including all seven probe subdirectories. Nothing supersedes the brief.

**Gates.** I re-ran the whole suite rather than inheriting a count: 654 passed, 0 failed, 9 ignored,
122 binaries, matching every prior member who ran it. I did not re-audit test bodies six prior members
already read in their own hands; I re-confirmed the two `#![feature(generic_const_exprs)]` gates every
member has flagged are still present today (`arvo/src/lib.rs:25`, `arvo-strategy/src/lib.rs:11`), and
the working tree is clean apart from this file's probes.

**Brief-breaking.** The brief's central premise is 09's finding, and per this panel's own record
(five findings overturned by a later member compiling), I did not inherit it: I rebuilt
`09_probes/d_delivery_disconnected_from_phi.rs` from source and ran it. Output reproduced exactly:
`ReduceModulo::phi(9, min=0, max=7) = 1` against `add() under Hot/ReduceModulo returned Total(7)`.
I also confirmed the mechanism in the union's own text: `add` at `08_probes/a_union.rs:692-698` calls
`C::over(max)` with the literal bound on every out-of-range branch, `over` at `a_union.rs:667-683`
checks the classification and then calls `Deliver::refuse(nearest)`, and `AsSum`'s
`Deliver<False>::refuse` at `a_union.rs:276-281` returns `Total(nearest)` unconditionally. No call to
`phi` for a value exists anywhere on the executed path. The premise holds and I proceed.

**Separation of evidence.** Sections marked *verified* rest on the four probes committed at
`10_probes/` (each compiled, and run where a runtime question is at stake, under `nightly-2026-05-28`)
or on source read at a `file:line`. Sections marked *reasoned* are argument. I carry more than one
reading wherever the evidence does not force one, and I rule on nothing.

---

## 0. My lens, and the exact statement of what went wrong

The panel built, across 03, 07 and 08, a mechanism whose one-sentence description was: the
mathematical facts are checked against the same definition the arithmetic executes. 09 compiled the
two halves against each other and found the sentence false. My subject is the discipline that
sentence belongs to, so let me restate 09's finding in that discipline's terms before extending it,
because the terms make the repair visible.

A verification claim has three parts: a specification, an implementation, and a theorem connecting
them. The panel's machinery has the first two and never stated the third. `phi` is a specification
(five recovery maps, `a_union.rs:139-214`). The `add`/`over`/`Deliver` chain is an implementation
(`a_union.rs:667-698`). The theorem that was proved, by the witness at the door, is "the declared
classification markers agree with `phi` over the model domain." The theorem that was *needed* is
"the executed arithmetic agrees with `phi`." Those quantify over different things, and no amount of
strengthening the first approaches the second, which is why 09 is right that the width ceiling
question (08 section 11) is orthogonal: checking the classification at 8 bits instead of 3 buys a
better-checked classification of a function the runtime never calls.

This is the oldest failure shape in verified systems, and it has a standard name: the design had
**two semantics for one construct**. `phi` said what a resolution means; `Deliver::refuse` plus the
caller's choice of `nearest` said what it does; nothing anywhere was obligated to make them agree,
so they did not, and every check in the design passed while `Hot` clamped. The union did not fail
because anyone wrote a bad line. It failed because the obligation connecting its two halves was
never stated, and an unstated obligation is discharged by nobody.

The repair discipline is equally standard, and the rest of this file applies it: **either prove the
connection, or make the two definitions one definition so there is no connection to prove.** Rust
without dependent types cannot do the first as a type-level theorem. It can do the second
structurally, and it can back the structure with a bounded exhaustive validation that fires at
compile time. I built both, they compose, and they cost nothing at runtime. Then, because a proof
does not eliminate trust but relocates and shrinks it, section 8 writes down what remains trusted
once the machinery is honest, which the brief asked for and which no member has yet done for the
design as a whole.

## 1. What the panel's machinery actually certifies, stated precisely. Verified against the union's text.

Before the repair, the ledger. Each row is what a mechanism in the union establishes, with the
quantifier it actually discharges, because most of the panel's overclaims have been quantifier
confusions.

| Mechanism | What it establishes | Over what |
|---|---|---|
| totality (`E0046`, 07/09) | every `Resolution` constructor answers every classification question | all constructors, by construction |
| coherence (`E0119`, 03 section 2) | no two contradictory law claims coexist | all impls, by construction |
| the witness (`a_union.rs:121-134, 218-222`) | declared markers equal `phi`'s computed classification | the model domain, [-8, 7] |
| the door (`a_union.rs:671-679`) | the same, un-disarmably, per monomorphisation | the model domain |
| the law fold (`a_union.rs:631-641`) | `AddAssoc` holds exactly when the markers say so | the marker algebra |
| **nothing** | the executed arithmetic agrees with `phi` | **nowhere** |

The last row is 09's finding as a ledger entry. Note what it does to the rows above it: they are all
real, none is wrong, and their combined guarantee chains from the consumer's `a + b` up to exactly
the point where the chain was never attached to the arithmetic. A consumer who reads
"`AddAssoc` is derived and witnessed" and concludes "my fold is associative" has crossed four sound
links and one absent one, and the absent one is invisible because it is an absence.

One sharpening of 09 worth recording, because it bounds the blast radius. The disconnection is not
in `phi`'s classification being unused; the *laws* keyed on the classification are about the
operation `phi` defines, so the law fold is certifying properties of an arithmetic the program does
not run. Under the union as compiled, `AddAssoc` for `Hot` is a true statement about wrapping
addition attached to a type whose addition clamps. That is strictly worse than no law marker,
because clamping unsigned addition happens to be associative too (01 finding 1's table), so even a
consumer's runtime test of associativity would pass while the values are wrong. The failure is only
observable against `phi` itself, which is precisely the comparison nothing performed until 09.

## 2. Is a semantic-preservation obligation statable here? Yes, in two parts, and I compiled both. Verified.

The brief asks whether the obligation is statable at all in this language: no proof assistant, no
dependent types, monomorphisation only, `adt_const_params` and const traits allowed,
`generic_const_exprs` and full `specialization` forbidden, `#![no_std]`, no `alloc`.

As a **type-level theorem**, no. Stating "for all payloads a, b: observe(add(a, b)) = phi(a + b)"
as a bound requires quantifying over values in a type, which is dependent typing, which Rust does
not have and `min_generic_const_args` does not approach. Anyone who claims the type system proves
this is overclaiming, and the round should never write that sentence.

As a **structural property plus a bounded validation**, yes, fully, and the statement is not novel:
it is Kulisch's defining equation of a machine operation, `a op' b = round(a op b)`, which 01
section 14 already put on the table as the frame the spec reinvented in pieces. The two parts:

**Part one: one definition** (`10_probes/a_one_definition.rs`, compiled and run). `phi` becomes a
`[const]` trait method **generic over the payload**:

```rust
pub const trait Resolve {
    fn phi<P: [const] Payload>(x: P::Wide, min: P, max: P) -> Rec<P>;
}
```

where `Payload` is a small const trait carrying widening, wide arithmetic, comparison and narrowing
(the per-width surface arvo's storage layer already owns). The executed pipeline is one body:

```rust
pub const fn pipeline_add<R, P, C>(a: P, b: P, min: P, max: P) -> C
where R: [const] Resolve, P: [const] Payload, C: [const] CarrierC<P>,
{
    let exact = P::wadd(a.widen(), b.widen());
    match R::phi::<P>(exact, min, max) {
        Rec::At(v) => C::from_output(v),
        Rec::Refused => C::refused(),
    }
}
```

The checker instantiates this at a 3-bit model payload; the consumer's arithmetic instantiates it at
the real width. **Two monomorphisations of one text.** There is no `Deliver::refuse(nearest)` for a
delivery to misinterpret, no `C::over(max)` for a caller to hardcode, and no second authored
definition anywhere for the first to disagree with. Run at the real width, 09's reproduction case
comes back correct: `5 + 4` over `[0, 7]` under `ReduceModulo` returns 1, phi's wrap; under
`TowardNegative`, 7, phi's clamp; under `Refuse`, a refusal. This answers what 09's section 4 named
and did not build, including the part it flagged as the open feasibility question: the payload
genericity works, as a method-level `[const]`-bounded generic on the pinned nightly, with no feature
beyond `const_trait_impl`.

**Part two: the preservation door** (same probe). The obligation is then stated once, as a const
function, and discharged by exhaustion over the model:

```rust
// for all a, b in the model domain:
//   observe(pipeline_add(a, b))  ==_Kleene  phi(widen(a) + widen(b))
const _: () = assert!(preserved::<ReduceModulo, Fallible<M3>>());
```

Five such forcing consts compile in the probe, one per (resolution, carrier) pair including the
`Poison` bottom carrier and the `Total` carrier. In real arvo this check sits inside the one generic
arithmetic door, exactly where 07 put the classification check, so it fires per monomorphisation and
cannot be forgotten. The two checks are the same mechanism with different theorems; the panel
already accepted the mechanism, and 09's finding is that it was pointed at the wrong theorem.

Why part two at all, if part one leaves nothing to disagree? Because part one has residual authored
surface: the carrier impls, and the three-line pipeline glue itself, and section 3 shows the carrier
freedom is real. The structure shrinks the obligation; the check discharges what the structure
cannot. This is belt and braces in exactly 07's two-site sense, and each half catches what the
other cannot.

## 3. What a carrier can still do wrong, partitioned, with the type system taking half. Verified.

The carrier layer (`CarrierC`: `from_output`, `refused`, `observe`) is the one authored surface the
single definition does not absorb, so its freedom is worth stating exactly. It partitions in two,
and the partition is clean:

**Value substitution is unwritable by signature.** `refused()` takes no payload argument. The
union's actual lie, returning a caller-chosen clamp value, has nothing to return:
`10_probes/c_substitution_unwritable.rs` attempts `Clamping(max)` and gets `E0425: cannot find
value max in this scope`, and attempts to fabricate a payload from `T: Copy` alone and gets `E0599`.
The signature is the enforcement, which is the strongest kind: the lie is not caught, it is not
expressible. One honest caveat, which is why this is half the answer rather than all of it: an
implementor may add its own bounds (`T: Default` and worse), and through them values become
constructible. Parametricity narrows the function space; it does not close it, and in a workspace
that forbids `specialization` and `TypeId` (arvo's `CLAUDE.md`, `unstable-features.md`) the
narrowing is much stronger than in general Rust, but it is a narrowing.

**Branch lies are writable, and the check catches them.** A carrier can drop the recovered value and
claim a refusal, or map its branches wrongly in `observe`; both bodies are writable with `T: Copy`
alone. `10_probes/b_lying_carrier_caught.rs` writes exactly that carrier and the preservation door
refuses it at const eval:

```
error[E0080]: evaluation panicked: executed arithmetic disagrees with its verified recovery map
```

So the carrier's function space is covered twice over: what the signature admits, the check
verifies. And because the check observes through `observe`, the same function the consumer's
`settle` is, the theorem is stated **up to the consumer's observation interface**, which brings one
locus finding that is cheap now and expensive later: the union's `Poison` carrier has public fields
(`a_union.rs:255-258`, `pub v: T, pub bottom: bool`). A consumer reading `.v` directly observes a
value no check has ever constrained, and the preservation theorem is silently voided for that read.
05 flagged the access discipline as an obligation; I would upgrade it: **the observation functions
define the perimeter of every guarantee in this design, so any exit from a carrier that bypasses
them is a hole in the theorem, not an ergonomic shortcut.** Private fields, `observe`/`settle` as
the only doors, and the carrier set sealed and arvo-owned (where 05, 06 and 07 already arrived from
layout, orphan-rule and grading arguments respectively; this is a fourth independent road to the
same door, from the statement of the theorem).

## 4. What it costs. Verified, and the answer is: nothing at runtime, microseconds at compile time.

The predictable objection is that routing the hot path through a reference semantics costs
performance, which for this substrate would be disqualifying. `10_probes/d_reference_path_codegen.rs`
instantiates the pipeline with the bounds as constants, the way a real composition's type supplies
them, at `-C opt-level=3` on aarch64:

- the wrap pipeline compiles to `add w8, w1, w0` / `and w0, w8, #0xfff` / `ret`, and LLVM proved it
  identical to the hand-written baseline and **aliased the symbols**: the emitted file contains
  `_baseline_wrap = _add_wrap_12bit`. The generic-phi path and the hand-written mask are not
  equally fast; they are the same function.
- the clamp pipeline is the same five-instruction `csel` shape as its hand-written baseline.

No timing claimed; the instruction sequences are the artifact, and the throughput bench remains
owed at `mock/benches/` per 08 section 10. The compile-time side: the preservation check is 64
pairs at the 3-bit model, well under the microsecond-scale const-eval 08 measured for the whole
union (0.15 to 0.21 seconds for the entire crate, `08_fog...md` section 11), and per 08 section 10
each distinct composition costs about 5.2ms and zero symbols. This is squarely the trade
`arvo-compile-time-last.md` licenses.

The check's asymptotic shape matters for what it forecloses: it is quadratic in the model span,
the same shape 08 measured to a hard ceiling at 8 bits with rustc refusing at 9. So the
preservation check at the composition's **actual** width is unavailable for the same reason the
classification check was, and should be recorded as unavailable. But this points at a taxonomy the
round should adopt rather than a single ceiling, because not every obligation is quadratic:

| Obligation | Check complexity | Where it runs |
|---|---|---|
| headroom: `Wide` holds every exact sum | O(1) | **the actual width**, every width |
| grade: this composition's phi never refuses on its domain | O(n) in the span | model width only |
| preservation: executed equals phi | O(n²) | model width only |
| stability: the law's identity | O(n²) | model width only |

The O(1) row is new to the panel and worth having: the adequacy of the exact intermediate (the
"a op b" half of Kulisch's equation) is a per-width fact checkable at width 128 as cheaply as at
width 3, so it never needs the uniformity argument at all. Classify each obligation by its
complexity and run everything at the widest width its complexity permits.

## 5. What the small model's adequacy actually rests on. Reasoned, with the load-bearing parts named.

A check at 3 bits certifies 3 bits. The transfer to width 128 is an argument, not a proof, and the
brief is right that an answer that checks a small model must say what the model's adequacy rests
on. Four legs, in decreasing order of solidity:

**Leg one: parametricity of the width-independent layers, enforced by the workspace's own bans.**
The pipeline body and every carrier impl are generic over the payload with no way to branch on
which payload they received: no `specialization` (forbidden, `unstable-features.md`), no `TypeId`
(banned, arvo `CLAUDE.md`), no reflection. A function that cannot ask "which width am I at" behaves
uniformly across widths, so its validation at one width transfers to all. I want to flag this
plainly because I have not seen it said anywhere in the workspace: **the forbidden-features list is
load-bearing verification infrastructure.** The day someone un-forbids `specialization`, every
small-model transfer argument in this design silently loses its ground, and nothing will fail. A
one-line note in `unstable-features.md`'s forbidden table, recording that the ban is also what
makes model-width validation transfer, would make that dependency visible to the future reader who
proposes relaxing it.

**Leg two: the per-width primitive operations are trusted with tests, not checked by any witness.**
The `Payload` impls (widening, wide add, compare, narrow) are per-width authored code; they are
exactly arvo's existing per-width container and arithmetic tables, covered by the 654-test suite
and by nothing in this panel's machinery. If `Bits<47>`'s widening addition is wrong, the model
check passes and width 47 is wrong. This is the right place for that trust to sit, because it is
small, enumerable, already tested, and O(1)-checkable per width for the headroom half (section 4),
but it must be on the list.

**Leg three: width-uniformity of phi's behaviour.** phi's classification (stable, refuses) is
checked at the model; the claim that a rule stable at 3 bits is stable at 128 is a property of the
rule's shape, arguable in prose per 03's frontier, and never mechanical. The one place this leg
carries runtime weight is the `Total` carrier's `refused()`, which is unreachable exactly when the
grade computed at the model width transfers; my probe makes it a panic with a message naming the
argument it guards, so if the argument is ever wrong the failure is loud and attributed rather than
silent. That panic is the honest residue of the transfer gap, one line, visible.

**Leg four, and the least visible: the agreement of rustc's two evaluators.** The preservation
check runs under the const evaluator (miri's engine); the consumer's arithmetic runs under LLVM
codegen. The design now rests on those two implementations of Rust's semantics agreeing on the
checked functions. For integer arithmetic this is solid ground and rustc's own obligation, but it
is a trust relocation, not an elimination, structurally identical to trusting the assembler below
a verified compiler: named, small, and outside our perimeter. It goes on the list.

## 6. The connection to 09's job one: phi stays Lowering-blind through the payload interface. Reasoned.

09 section 4 flagged that a genuinely shared phi must run where the real arithmetic runs, which is
a `Lowering`-aware crate, and asked whether it can be kept `Lowering`-blind in job one's sense. The
probe's shape answers this more cleanly than I expected when I started: phi's only view of the
world is the `Payload` interface. It cannot name a layout, a stored width or a delivery because
none of those is in its scope; the `Payload` impl it receives is chosen downstream, by the
composition, in the `Lowering`-aware crate. So the same mechanism that makes phi width-generic
makes it Lowering-blind, in the structural sense of 09's section 1 (the symbol has no referent),
provided `Payload` itself is declared in a crate with no `Lowering` edge, which D72's table
permits.

What this creates, honestly stated, is a new agreement obligation one level down: two lowerings of
one numeral (say `Minimum` and `DoubleLogical` storage of a 13-bit value) supply two `Payload`
impls, and the design needs them observationally equal through `to_model`. That obligation is
O(n) per width pair at the model and O(1) per width for the headroom fact, so it is checkable by
the same const machinery, and it is exactly the "one semantics, every artifact answers to it"
shape applied at the next layer down. I did not build it and say so; it is a small probe and
someone should, because it is where the next 09-shaped gap would hide.

## 7. Where the guarantee ends by policy: the optimized paths. Reasoned, and this is a standing ledger, not a one-time fix.

`arvo-always-optimal-internals.md` licenses replacing the reference path with cfg-gated
intrinsics and asm microkernels wherever benches show a win, and this is settled workspace
discipline, not something this panel should reopen. Its consequence for the preservation story
must be stated though, because the const check **structurally cannot see it**: inline asm is not
const-evaluable, so an optimized arm is a second definition by construction, and the single
definition of section 2 holds only for the reference path.

The discipline that fits, and the vocabulary for it, comes from certified compilation's own
practice: where proving is too costly, **validate**. Each optimized arm is a translated artifact
whose specification is the reference pipeline, and the obligation is a per-arm validation test:
exhaustive comparison against the reference at small widths, randomized at large, running in the
ordinary suite, shipped in the same change that adds the arm. `catalogue-edge-cases-as-tests.md`
already demands the shape; what is missing is only the standing rule that **an optimized arm
without its validation test is an undischarged obligation**, and that each arm added is a TCB
growth event, recorded as one. The reference path being provably identical to the hand-written
form at opt-level 3 (probe D, the aliased symbols) also means the bar an asm arm must clear to
earn its existence is higher than anyone has been assuming: for the wrap case there is nothing
left to beat.

## 8. The trusted computing base of this design, written down. The section the brief asked for.

Once the repair of section 2 lands, every claim in the design sorts into exactly one of four bins.
I give the full sort, because the panel has been producing claims faster than it has been sorting
them, and several of the entries below have been implicitly presented as being in a stronger bin
than they are in. The value of the machinery is measured by how small and how explicit the trusted
bin is, so shrinking this list is the round's real scoreboard.

**Machine-checked, by construction (the strongest bin: violation does not compile).**

- Value substitution on the refusal path: unwritable, no payload in scope (probe C, E0425).
- A law conditioned on a `Lowering` member, under 09's `LogicalNumber` closure: unwritable
  (09 crate-boundary probes, E0432/E0117).
- A classification member omitted by a new constructor: E0046 (07, re-verified by 09).
- Two contradictory law claims: E0119 (03).

**Machine-checked, by bounded exhaustion at the model width (violation fails the build).**

- Declared classification against phi (07/08's witness, at the door).
- **Executed arithmetic against phi** (probe A/B, the preservation door). New; this is the row
  whose absence was 09's finding.
- The grade against phi's actual refusal behaviour (08's `ever_refuses`).

**Trusted, named, with the mechanism that watches each (the list that must stay short).**

1. The five `phi` bodies: the specification itself, roughly 25 lines total. Nothing beneath them
   to check against; review is the mechanism, and 01's hand tables are that review, done.
2. The checker identities (`stable`, Kleene equality, the preservation equation): the *statement*
   of the theorems, roughly 30 lines. A wrong statement certifies the wrong thing everywhere;
   this is every verified system's irreducible rim, and it belongs in one file with 01's tables
   cited beside it.
3. The observation functions (`observe`/`settle`): the perimeter of every theorem. Watched by
   privacy (no field exits; the union's `pub` Poison fields at `a_union.rs:255-258` violate this
   today) and by the sealed, arvo-owned carrier set.
4. The per-width `Payload`/primitive impls: trusted with the existing 654-test suite plus the O(1)
   headroom check at actual width (section 4). Width-dependent, enumerable, already the
   best-tested code in the tree.
5. The width-uniformity transfer argument: prose, per obligation, with the `Total::refused` panic
   as its one runtime guard.
6. The parametricity ground: the standing bans on `specialization` and `TypeId`. Watched by
   `unstable-features.md`, which should say it now carries this weight.
7. rustc itself, twice: the const evaluator agreeing with codegen, and the compiler being correct
   at all. Outside our perimeter, named so nobody thinks it was proved.
8. The three-line pipeline glue per operation (widen, op, quantise): covered by the preservation
   door at the model width, trusted above it.

**Validated per artifact (the bin that grows over time and needs a ledger).**

- Every cfg-gated optimized arm, against the reference path (section 7). Zero arms today on this
  surface; each future one adds a row.

**Promised, dischargeable only by measurement (already established by 02/03/05; restated so the
sort is complete).**

- `ConstantTime`, `Deterministic`: claims about emitted code, bench artifacts per
  `bench-and-sketch-discipline.md`, never type-level facts. 05 section 6's finding stands: under
  the sum-type delivery a refusing composition cannot claim `ConstantTime` at all.

What is **hoped, currently, in the spec as written**, and should stop being hoped: that deliveries
agree with the semantics (becomes machine-checked under the repair); that "derived cannot lie"
(becomes the three-rung ladder 07 proposed, which this sort refines into four bins); and that the
D67 convention aliases compute what their vendors compute (becomes bounded validation against
published vendor vectors, per 03 section 6, which slots into the second bin above and is the
cheapest entry in it because the oracles are published).

## 9. Readings I hold against my own proposal, because the evidence does not force it. Reasoned.

**Reading one: ordinary runtime tests would have caught this, and nearly did.** arvo's shipped
discipline pins concrete semantics with concrete values: `strategy_semantics.rs` asserts wrap to
44 and saturation at logical MAX, and a test of that shape written against the union would have
failed on `Hot` clamping. On this reading 09's gap is a defect of a probe, not of a design, and
the cure is the house's existing test discipline applied when `arvo-policy` is written. I hold
this reading honestly and weigh it as follows: tests sample, the door is exhaustive at the model
and fires per monomorphisation without anyone remembering to write it, and the union is the proof
that three careful members in one day connected everything except the one comparison no mechanism
asked for. A discipline that relies on someone writing the right test is the review-discipline
posture this workspace's rules repeatedly reject in favour of structure. But the reading is not
wrong, and the two are not exclusive; the concrete-value tests remain wanted as the check on the
checker.

**Reading two: shrink the surface instead of checking it.** The carrier and delivery diversity
(sum, bottom, flag; the grade lattice; the lifts) is measured value, 05 and 08 both priced it. But
its price now includes a verification obligation this file has just made explicit, and there is a
smaller design available: one arvo-owned fallible carrier, `settle` as its only door, delivery
diversity cut entirely. Under that design the carrier layer nearly vanishes, the preservation
check still exists but guards three lines instead of a family, and the TCB's item 3 shrinks to
one type. 08's measurement says what this costs: the bottom delivery's branchless loop and its
8x-when-wrong hazard both disappear. Whether the delivery axis is worth its verification surface
is exactly the kind of call that is op's, and it should be made with this file's section 8 open
next to 08's section 5, because they are the two sides of the same price tag.

**Reading three: the preservation door is redundant once the definition is single.** If
`pipeline_add` literally calls `phi`, what does the check add? Drift protection: the blanket is
authored code and will be edited; the carrier set will grow; probe B's lying carrier is writable
today. The check costs microseconds and converts "nobody will edit this wrongly" into "editing
this wrongly does not compile". I find the redundancy argument weakest of the three, but a
minimalist could hold it, and if the round adopts only one half, the single definition is the
half to keep: structure beats validation where only one is affordable, because the structure has
no model-width gap.

## 10. Engagement with the prior members, kept short.

**01.** The Kulisch frame (its section 14) has now earned its third operational use: 05's macros,
07's checker, and this file's preservation equation, which is Kulisch's machine-operation
definition verbatim. 04's scope dissent (adopt the frame where it deletes something, cite it
elsewhere) still holds; nothing here requires renaming anything public.

**03.** Its layer distinction (section 4: the predicate is a spec, discharge is mechanical, use
is a third question) is the skeleton this file's TCB sort hangs on. Its section 5 notko pin: under
the one-definition shape the `ConstFromResidual` dependency stays structurally absent, as 07
already noted for the graded shape; the pin remains worth an afternoon for any other surface.

**05.** Its access-discipline caveat (section 2) is upgraded by this file from an obligation on a
proposal to part of the theorem's perimeter (section 3). Its `ConstantTime` inversion finding is
restated in the TCB sort where it belongs, in the promised bin.

**07.** The witness mechanism survives intact; what this file changes is its object. 07's claim
that "the oracle is not a second place to be wrong, because it is the same phi the runtime
arithmetic calls" was, as 09 proved, a description of an architecture 07 had not built. Probe A
builds it, so the claim is now true of an artifact rather than of an intention, and 07's two-site
enforcement discipline (eager consts in the declaring crate, direct check at the door) carries
over to the preservation theorem unchanged.

**08.** Its width ceiling (section 11) is confirmed as governing the preservation check too, same
quadratic shape, and its "record it as unavailable" recommendation extends to this check at actual
width. Its section 5 spare-pattern precondition interacts with reading two above: if delivery
diversity survives, the preservation check must run per (composition, delivery) pair, which the
door does for free per monomorphisation.

**09.** Reproduced, confirmed, and extended. One friendly sharpening: 09's closing frame says the
honest trusted base after its finding is "every `Deliver::refuse` implementation and every
`add`-shaped arithmetic body, none of which is checked against anything." Correct for the union;
the useful addition is that this trusted base was not necessary, and the repair is not primarily a
check but a deletion: the union's `over(nearest)`/`refuse(nearest)` interface is the thing that
manufactured the second semantics, and removing the value parameter removes the place the lie
lived. The check then guards the remainder. 09's job-one closure (`LogicalNumber`, unconstrained
`L`) composes with this file's shape naturally, since phi's Lowering-blindness comes through the
payload interface (section 6), but the composed artifact has not been built, and per this panel's
record, that sentence means it is not established.

## 11. What I did not get to.

The `Payload`-agreement probe of section 6: two lowerings of one numeral, observational equality
through `to_model`, checked at the model width. Small, and it is where the next disconnection of
this exact shape would hide.

Wiring the preservation door into 09's `LogicalNumber` closure end to end, so the type the law is
proven about, the type the arithmetic runs on, and the definition both answer to are one artifact.
Every piece exists in some probe; no probe holds all of them, which after this panel's history I
regard as an unverified claim by default.

The `Growth = Narrowed` case: two quantisation firings per operation (05 section 9), so the
preservation equation gains a compositional step, `observe(op') = phi_result ∘ narrow ∘ phi_mid ∘
exact`. The equation extends mechanically; the probe does not exist.

Multiplication. Every probe in this panel, mine included, checks addition. The preservation
equation is per operation, the quadratic cost is per operation, and `Mul`'s exact intermediate is
the one whose headroom fact (widths add) most needs the O(1) actual-width check.

And the migration of the shipped `strategy_semantics.rs` values into vendor-vector-style oracles
for the new machinery, which is where reading one and the preservation door meet: the old
concrete-value assertions are the best available independent check on the checker, and they should
be carried forward as such rather than deleted with the shapes they currently test.

---

**Summary for the next member.** The panel's machinery, as it stood after 09, certified that a
classification agrees with a reference function no executed code calls; the missing piece was never
a stronger check but a stated obligation connecting specification to implementation, and that
obligation is statable in this language: not as a type-level theorem (no dependent types, and the
round should never claim it), but as one definition plus a bounded validation, both compiled here.
`phi` as a `[const]` trait method generic over a `Payload` const trait is simultaneously the
checked function and the executed function, two monomorphisations of one text, and 09's
reproduction case comes back correct at the real width (`10_probes/a_one_definition.rs`, run). The
preservation equation, Kulisch's `a op' b = round(a op b)` under Kleene equality, is checked
exhaustively at the model width at the door and refuses a lying carrier at const eval (probe B,
E0080), while the union's actual lie, value substitution, becomes unwritable by signature because
the refusal constructor receives no payload (probe C, E0425). The reference path costs nothing:
LLVM aliased the wrap pipeline's symbol to the hand-written baseline's (probe D,
`_baseline_wrap = _add_wrap_12bit`). What remains trusted is written down in section 8 as four
bins, and the load-bearing news for the round is the third bin's shortness and two of its entries:
the forbidden-features list is what makes model-width validation transfer to real widths, so the
bans on `specialization` and `TypeId` are verification infrastructure and should be recorded as
such; and the observation functions define the perimeter of every theorem, so the union's public
`Poison` fields (`a_union.rs:255-258`) are a hole in the guarantee, not a style issue. The check at
the actual width is unavailable (08's ceiling governs here too), but obligations sort by check
complexity, and the O(1) facts, headroom above all, should run at every real width. Three readings
are left open for op: rely on the house's concrete-value test discipline instead (real, weaker,
not exclusive); shrink the carrier and delivery surface rather than verify it (08 section 5 is the
other side of that price tag); and whether the preservation door is worth keeping once the
definition is single (I think yes, for drift; the structure is the half to keep if only one
survives). I rule on nothing; op decides.
