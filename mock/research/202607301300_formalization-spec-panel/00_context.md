# Panel context: the formalization spec

**Date:** 2026-07-30
**Repo:** arvo, branch `feat/arvo-shape-topic`
**Subject:** `mock/design_rounds/202607301200_topic.the-formalization-spec.md`

This file is the shared brief every member of this panel reads before doing anything. It states what
governs, what is open to attack, what each member owes, and the order the panel runs in. Read it in
full. It is not a summary of the work; the work is the three files named below.

## What this panel is for

A design round produced a spec that restructures arvo's numeric core: three contracts, ten axes, one
type where four families used to be, and a set of mathematical properties derived rather than
declared. It was arrived at by two people over one conversation. Before it becomes the basis of
document and source changelists, it gets taken apart.

The mandate is to review, audit, stress-test, and poke holes. Beyond that, to propose improvements and
specialisations that fit arvo better and use the type system harder than the draft does. **A much
better spec that looks nothing like this one is the best possible outcome of this panel, not a
failure of it.** If the shape is wrong, say the shape is wrong.

## Required reading, in this order

1. `mock/design_rounds/202607301200_topic.the-formalization-spec.md`. The subject. Around four hundred
   lines.
2. `mock/design_rounds/202607301100_topic.the-formalization-talk.md`. The transcript the spec derives
   from, carrying the reasoning behind every call, several corrections where a call was wrong and was
   withdrawn, and the research each rests on.
3. `mock/design_rounds/202607301000_topic.inherited-state-from-the-formalization-round.md`. The prior
   round's fifty-two decisions, which this round builds on and in three places amends.

Then the source the spec replaces, which is claims to test and not context to reason inside:
`mock/crates/arvo-strategy/src/{lib,axes,arith,container,cross_strategy,identity}.rs`,
`mock/crates/arvo/src/{ufixed,ifixed,float,markers,fixed_scale}.rs`,
`mock/crates/arvo-storage/src/bits.rs`.

And the two sketches, whose findings the spec cites and whose shape it has since moved past:
`mock/research/sketches/202607300500_format-as-exponent-function/`,
`mock/research/sketches/202607300600_law-markers-derived-over-composition/`.

## What governs, and what does not

Authority comes from human ratification. For this arc the rungs are:

**Governing, and defended rather than weighed.** The workspace rules at
`~/Dev/clause-dev/.claude/rules/`, in particular `arvo-toolbox-not-policer.md` (expose the choice,
never police the consumer; `Cold` is not optional), `arvo-always-optimal-internals.md` (the public
surface has rules, the implementation does not), `arvo-compile-time-last.md` (runtime first, compile
time is the bucket we pour into), `unstable-features.md` (`generic_const_exprs` and full
`specialization` are forbidden; the `min_` variants and the const-traits family are allowed),
`no-bare-primitives.md`, `no-alloc-no-std-framing.md`, and `no-legacy-shims-pre-1.0.md`. Also arvo's
own `mock/PRINCIPLES.md.tmpl` and the layer discipline in `CLAUDE.md`. These are not up for
discussion, and a proposal that violates one is wrong regardless of its merits.

**Ratified for this arc, and explicitly opened for attack.** The spec's own calls, D53 through D75,
and the prior round's D1 through D52. Each records the lead designer in the loop, so each is on the
ratified rung and would ordinarily be defended. **The lead designer has instructed that this panel
challenge them.** That instruction is why this panel exists, it is recorded here so no member has to
infer it, and it applies to every call including the ones the spec presents as settled. A call being
op's is not an argument for it.

**Presumed wrong where it conflicts.** Everything else. The two sketches, the research artifacts, the
shipped source, every comment in that source, and the spec's own derivations. Where several of these
agree with each other, that is not corroboration: they were produced by the same process and copy each
other's framing.

**Maximally suspect.** Anything the spec marks as the agent's own derivation. Its provenance section
names four, and those four are where to look first.

## Existence and locus are challengeable, in those words

Whether a thing in this spec should exist at all, and whether it sits on the correct side of arvo's
crate and contract boundaries, are both in scope. The three-contract split, the ten axes, the single
`Number` type, the crate list, and the decision to unify fixed point with floating point are all
challengeable at the level of whether they should be there, not merely how they should be shaped.

The shipped source is likewise a set of claims to test rather than a baseline to design against. Where
the spec says arvo currently does something, verify it; the round found two cases where the shipped
behaviour was not what any document said.

## Report unlicensed mechanisms even outside your question

If you find something in the spec, in the talk, or in the source that the workspace rules do not
license, report it whether or not it falls in your assigned area, and do not soften it. Say plainly
that it is wrong and what it costs. Do not write "you may want to consider" or "one option would be";
those phrasings are how a finding gets absorbed and ignored.

Every judgement carries a citation. Contempt with a `file:line` attached is a finding. Contempt
without one is noise that spends output budget and lets the substance be dismissed as tone.

## The two standing gates

**The canon gate.** Before your assigned work, check that the work and the state it builds on align
with what governs. If they do not, return early with the conflicting rule text and the offending
`file:line` rather than proceeding. If the governing material is ambiguous on a point your work
depends on, stop and hand the call back rather than resolving it yourself. An early return is a
successful dispatch and will not be re-dispatched with a softer brief.

**The test gate.** Run arvo's suite, whatever state it is in, then read the *bodies* of the tests
covering the surfaces this spec replaces. Names and counts are not evidence. Refuse the assigned work
and report instead if you find tautological tests, setups that feed only inputs the implementation
handles, laws asserted over a sample of widths rather than the whole matrix, or fundamental properties
asserted nowhere. The round already found one instance of the last two: a compile-fail fixture named
`warm_bound_negative` passed continuously for months while asserting nothing, because its `.stderr`
snapshot recorded a typo's error rather than the contract's.

## What to attack, if you want a starting list

Not a scope limit, and not a ranking. Members are expected to find things this list does not name.

- The three-way split between identity, semantics and cost, and the test that sorts axes into it.
- Whether ten axes is the right number, in either direction.
- The claim that fixed point and floating point differ only in where the exponent lives.
- The quantisation five-tuple, and whether the vocabulary split between `Direction` and `Resolution`
  survives contact with a mode neither IEEE nor SystemC names.
- The faithfulness derivation, which is the spec's strongest claim and rests on one property.
- Whether the derived properties are actually derived, or whether an assertion is hiding in a
  constructor somewhere.
- The presets, which were redefined from intent and change behaviour for every existing consumer.
- Compile-time cost, which nothing in this round has measured.
- Whether the type system is being used as hard as it could be. The lead designer's standing position
  is that it is not, anywhere, ever.

## How this panel runs

Sequentially and cumulatively. Each member reads every prior member's file before starting and builds
on it, agrees with it, or refutes it explicitly. This is not a set of independent opinions to be
averaged; it is one investigation carried forward. A member who repeats an earlier finding without
adding to it has wasted a dispatch.

Files are numbered in order: `01_<slug>.md`, `02_<slug>.md`, and so on, in this directory. Each names
its author persona at the top, states what it read, and separates what it verified from what it
reasoned about.

The final file is a synthesis, written last, which states what the panel changed about the spec and
what it left standing. If the panel's conclusion is that the spec should be rewritten, the synthesis
says so in those terms rather than listing amendments.
