# Panel context: the formalization spec

**Date:** 2026-07-30
**Repo:** arvo, branch `feat/arvo-shape-topic`
**Subject:** `mock/design_rounds/202607301200_topic.the-formalization-spec.md`

This is the shared brief every member reads first. Read it in full, then read
`~/Dev/clause-dev/.claude/rules/panels-argue-the-intent-not-the-wording.md`, which governs how panels
in this workspace are supposed to behave and exists because several have not.

## Two things before your lens

**First, `ls` the directories around every path named here.** You do not have to read everything you
find. You do have to know what is there, what is newer, and whether something supersedes what you
were handed. This brief names what its author thought was relevant, and its author did not know what
they had missed. Any time the picture feels partial, read more. `mock/design_rounds/` and
`mock/research/` both have a lot in them, much of it archived rounds, some of it recent.

**Second, try to break this brief.** Check its cheap factual claims against source. Every "arvo
currently does X" below is a claim, not a fact you inherit. This matters here specifically: on
2026-07-28 an arvo research panel was briefed on the claim that arvo depends on `generic_const_exprs`
pervasively, which was false, the feature being forbidden. Every pass inherited the premise, and op
stopped the panel. Its own coverage note records that the baseline was wrong and two passes had to be
revised. Nobody violated a rule; the brief simply asserted something untrue and nothing tested it.

If a premise this brief rests on turns out to be false, say so and stop. That is a successful
dispatch and the most valuable thing available.

## What this panel is for

A design round produced a spec restructuring arvo's numeric core: three contracts, ten axes, one type
where four families used to be, and a set of mathematical properties derived rather than declared. It
was arrived at by two people in one conversation, today.

The mandate is to review, audit, stress-test and poke holes, and past that to propose improvements and
specialisations that fit arvo better and use the type system harder than the draft does. **A much
better spec that looks nothing like this one is the best outcome available, not a failure.** If the
shape is wrong, say the shape is wrong.

## There is no ratified canon here, and that changes your posture

This matters, so it is stated rather than left to inference.

Some material in this workspace is ratified canon: documents recording the lead designer in the loop,
marking human calls inline, which are defended rather than weighed. **This is not that.** The spec is
one day old. Its calls are op's, made live in conversation, and op's standing position is that only
op's calls are final and even those go stale the moment something better surfaces or new material
appears.

So what governs here is the **intent and the spirit**: what arvo is for, what problem this
restructuring exists to solve, what the project is trying to be. That is arguable, including arguable
against. The documents are a paper trail of how the thinking went, not the thinking itself. A
panellist that stops at "this was decided" has not finished; decided is not the same as right.

What *does* govern, and is not up for discussion, is the workspace's long-settled discipline:
`arvo-toolbox-not-policer.md` (expose the choice, never police the consumer; `Cold` is not optional),
`arvo-always-optimal-internals.md` (the public surface has rules, the implementation does not),
`arvo-compile-time-last.md` (runtime first), `unstable-features.md` (`generic_const_exprs` and full
`specialization` forbidden, the `min_` variants and the const-traits family allowed),
`no-bare-primitives.md`, `no-alloc-no-std-framing.md`, `no-legacy-shims-pre-1.0.md`, plus arvo's
`mock/PRINCIPLES.md.tmpl` and the layer discipline in `CLAUDE.md`. A proposal violating one of those
is wrong on its merits. Verify rather than assume: the round found the shipped source disagreeing with
its own documents twice.

## How to spend the dispatch

**Riff and muse, in preference to checking.** Opening a direction nobody considered is worth more than
confirming something already written. A panel that only verifies has under-delivered.

**Go wide by category before deep by variant.** Enumerating families of approach nobody touched beats
permuting the axes already in hand. That failure is on record here too: two overnight sessions
produced only permutations and declared done twice, until a fresh agent listed whole untouched
categories in minutes.

**Never one angle.** Carry more than one reading of anything substantive, say what would distinguish
them, and do not resolve where the evidence does not force it. Your file is not a thesis with
supporting evidence.

**Suggest; do not legislate.** Opinions, critiques, thoughts, proposals. Do not write as though your
finding closes the question. Op decides.

**Consider the whole and beyond it.** What surrounds this, what it forecloses, what it will have to
live with, what a neighbouring field already solved. A finding scoped exactly to the question asked
has usually missed the useful part.

**Assume everyone before you was wrong**, including the other panellists, including op, including this
brief.

## Reading

The subject, then its derivation:

1. `mock/design_rounds/202607301200_topic.the-formalization-spec.md`, around 400 lines.
2. `mock/design_rounds/202607301100_topic.the-formalization-talk.md`, the transcript it derives from,
   carrying the reasoning, several corrections where a call was made and withdrawn, and the research.
3. `mock/design_rounds/202607301000_topic.inherited-state-from-the-formalization-round.md`, the prior
   round's decisions.

The source the spec would replace, which is claims to test:
`mock/crates/arvo-strategy/src/{lib,axes,arith,container,cross_strategy,identity}.rs`,
`mock/crates/arvo/src/{ufixed,ifixed,float,markers,fixed_scale}.rs`,
`mock/crates/arvo-storage/src/bits.rs`.

Two sketches whose findings the spec cites and whose shape it has moved past:
`mock/research/sketches/202607300500_format-as-exponent-function/`,
`mock/research/sketches/202607300600_law-markers-derived-over-composition/`.

## The test gate

Run arvo's suite, then read the *bodies* of tests covering the surfaces this spec touches. Names and
counts are not evidence. If you find tautological tests, setups feeding only inputs the implementation
handles, laws asserted over a sample of widths rather than the matrix, or fundamental properties
asserted nowhere, report that instead of the assigned work. Precedent: a compile-fail fixture named
`warm_bound_negative` passed for months while asserting nothing, because its `.stderr` snapshot had
captured a typo's error rather than the contract's.

## Starting list, not a scope limit

Members are expected to find things this does not name.

- Whether the identity, semantics and cost split is the right decomposition, or a plausible one.
- Whether ten axes is right, in either direction.
- The claim that fixed point and floating point differ only in where the exponent lives.
- The quantisation five-tuple, and whether its vocabulary survives a mode neither IEEE nor SystemC
  names.
- The faithfulness derivation, the spec's strongest claim, resting on one property.
- Whether the derived properties are derived, or whether an assertion is hiding in a constructor.
- The presets, redefined from intent, which change behaviour for every existing consumer.
- Compile-time cost, which nothing here has measured.
- Whether the type system is being used as hard as it could be. Op's standing position is that it is
  not, anywhere, ever.

## How this panel runs

Sequentially and cumulatively. Each member reads every prior member's file and engages with it,
building on it, sharpening it, or rebutting it explicitly. This is one investigation carried forward,
not opinions to be averaged. A member repeating an earlier finding without adding has wasted a
dispatch.

Files are numbered in order: `01_<slug>.md`, `02_<slug>.md`. Each names its persona, states what it
read, and separates what it verified from what it reasoned about. The last file is a synthesis stating
what the panel changed and what it left standing; if the conclusion is that the spec should be
rewritten, it says that rather than listing amendments.
