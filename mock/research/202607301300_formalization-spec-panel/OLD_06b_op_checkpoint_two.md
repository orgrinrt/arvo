# Op's second checkpoint, after six

**Date:** 2026-07-30
**Position in the panel:** written after `06_muratori_the_consumer_surface.md`, before the seventh
member. **Required reading for every subsequent member**, with the numbered expert files and `04b`.

The panel continues. Op will say when it is ready for synthesis; no member should treat itself as the
last or write a synthesis unless dispatched to do so.

## The correction, and it is the important part of this checkpoint

Panellist 06 reported, and ranked highest of everything it found, that `hilavitkutin`'s
`dispatch_codegen.rs` carries four domain newtypes over arvo aliases with twenty
`lint:allow(no-bare-numeric)` escapes, in four categories all corresponding to arvo surface that does
not exist, none of which the ten-axis spec addresses. The finding was put to op as evidence that the
redesign may be solving the wrong problem.

Op's answer, verbatim:

> Hmm. The fact existing consumers do things one way, might just be because no better existed (we know
> this, this is why we are here). Should be irrelevant, we focus on the optimal, what the consumers
> would ideally deal with and in

This is a standing correction on how downstream evidence is read, and it applies to every member from
here.

**What a consumer currently writes is evidence of what was absent when they wrote it.** It is not
evidence of what they need, and it is certainly not a requirement to preserve. A workaround in
`hilavitkutin` tells you arvo lacked something at the moment that line was written; it tells you
nothing about what the right surface is. Reading it as a requirement inverts the entire reason this
work exists.

So the question a member asks about any downstream observation is not "what does the consumer do", nor
"what would break if this changed", but **"what would the consumer ideally be dealing with, and in
what terms"**. Where the ideal answer breaks every existing call site, that is a migration cost to
state plainly and not an argument against the answer.

This does not make the observation worthless. Twenty escape hatches in one file are a real signal that
something is missing. The error is only in treating the *shape* of the workaround as the *shape* of
the need.

## The two other calls at this checkpoint

**Thread C, leaf truth, is carried by the next general lens** rather than given a dedicated dispatch.
Its state: panellist 03 established that the type machinery delivers totality and coherence but never
the truth of a leaf fact, and proposed solver-free const checks; panellist 05 compiled six of them,
reproduced 01's counterexample mechanically, and found the constraint that a `const fn` cannot call
through a function pointer so the oracle must be macro-instantiated; panellist 06 found that under the
computed-truth encoding the diagnostic attribute never fires and a consumer reads `False: IsTrue`,
repairable in four lines. What nobody has yet done is what op asked for: find a shape where the check
**is** the typestate rather than sitting beside it.

**The panel's two standing design results are proposals, not settled.** Op's call, given that the
panel has already reversed two of its own findings:

- that delivery of a refusal is a `Lowering` member rather than a `Policy` one, which un-exiles
  `Precise` while leaving it refusing (panellist 05, verified by running a refusing policy through
  unmodified `arvo_graph::upward_rank`)
- that named types at every axis position make the ten axes free in the diagnostic surface
  (panellist 06, verified by compiling the rendered forms)

Later members attack both as freely as anything else in the spec. Neither is a foundation to build on
without argument.

## Standing note, restated because it keeps mattering

Only op's calls are final, and even those go stale when something better surfaces. The spec's calls
are one day old and two of the panel's own findings have already been overturned by a later member
compiling something. Argue with anything; assert nothing.
