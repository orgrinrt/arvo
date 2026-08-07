# 00. The numeral canon panel

**Opened:** 2026-08-07. **Repo:** arvo. **Predecessor:** `202607301300_formalization-spec-panel`,
closed at 320 files.

## What this panel is

Arvo has no canon. This panel writes it: the primitives become named compositions over one format
concept, with number systems and derived algebraic laws underneath.

It is not a fresh start on the problem. It is a fresh start on the **process**, seeded with what the
previous panel actually established, so that nobody has to read 320 files to contribute. The
predecessor spent much of its life learning how to run a panel while running one. Those lessons are
written down now, in `RULES.md`, instead of being rediscovered.

## What you read, and in what order

Five files, and none of them is long except the last.

1. **`RULES.md`.** How this panel works. Provenance, what a canon may contain, dispatch conduct,
   evidence, consolidation. Read it once before your first file.
2. **`01_op_answers.md`.** Op's founding input, including the correction to what ratification means.
   Section 0 changes how every other document here is read, so it is not optional.
3. **`SETTLED.md`.** The index of what the predecessor established, with provenance per row, the open
   questions by name, and a header stating why its top rung is now wrong.
4. **`DROPLIST.md`.** What was tried and dropped, cumulative, plus same-stretch reversals kept under
   a separate name. Read it before proposing anything, so a dead route is not walked twice.
5. **`CANON_CANDIDATE.md`.** The predecessor's twelfth consolidation, carried whole, with its known
   stale sections listed at the top. A starting text to re-derive from, never a citation.

`seed/` holds the four survivor sweeps of the predecessor's full history, by theme, each with its
own casualty list and its own honest coverage bound. Consult the one covering your question before
relying on a `SETTLED.md` row it covers.

`02_carried_*` is the predecessor's last file, delivered after this panel opened. It is marked as
carried and its citations point into the old panel.

**Do not read the predecessor's tree.** It is 320 files and roughly 210,000 tokens of markdown.
Everything that survived it is in the five files above. If you believe something is missing, say so
rather than going to fetch it.

## What is fixed

**Op's ratified calls**, which are fewer than the previous panel believed. See `01` section 0.

**The acceptance criterion**, which is op's erasure gate: the consumer expresses usage in bits and
bytes, the typestate derives the matching container and representation, it validates, and it erases
on lowering. All four at once, no caveats. Every mechanism proposal answers to this.

**The workspace discipline**: `#![no_std]`, no `alloc`, no `dyn`, no `TypeId`, sizes const, no bare
primitives at public positions, harness the type system, the writing style.

**The forbidden features**: `generic_const_exprs`, `generic_const_args`, full `specialization`, and
`-Znext-solver=globally`. Allowed: `min_generic_const_args`, `adt_const_params`, `min_specialization`
and the const-traits family. Pin `nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.
A bare `rustc` outside the repository tree resolves to stable, so pass the toolchain explicitly.

**Everything else is open**, including the vocabulary, the decomposition, and any name.

## The dead tree

`mock/crates` is being nuked, and its designs with it. Read it for what arvo used to do and for what
not to do. Never cite it as evidence about what is correct, and never report a finding about its
contents as a deliverable. Writing to it is forbidden.

Probes are the evidence, and they live committed in `NN_probes/` beside the file that cites them.

## The shape of a contribution

`NN_persona_topic.md`, with `NN_probes/` beside it where claims need real code. Write the file to
disk early and extend it in place; several predecessor dispatches died holding a finished
investigation in one final write.

Four expert files, then op's checkpoint slot, then four more, then a consolidation, repeating. The
consolidation is the canon candidate.

## What the panel does first

Three tasks are already named, and none of them waits on op.

**The family comparison.** Op asked for the consequences of one-family versus several to be laid out
before he rules. Knuth's carried result is that every structural failure in the record reduces to
this one tie, so this decides more than its size suggests. It must say, for each option, what becomes
derivable, what has to be named, what the canon must state that it otherwise would not, and what it
costs a consumer.

**Fresh eyes on the container-derivation attempt.** Op's standing call, unchanged: the latest attempt
was a very good place, with contracts and typestate working fully, no enumeration, and no forbidden
features. Confirm or improve it. He flags his own recall as approximate, so verify those properties
hold as stated and report if they do not.

**Re-read the `SETTLED.md` rows against the ack correction**, and mark which were ratified after a
convergence and which were acks filed as closures.
