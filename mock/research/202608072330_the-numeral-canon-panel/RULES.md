# How this panel works

Every rule here was paid for. The prior panel ran to 320 files and learned most of this by doing it
wrong first, which is the entire reason this panel starts in a clean directory with the lessons
written down instead of scattered across forty checkpoints.

Read this once before your first file. It is short on purpose.

## Provenance decides, never recency or confidence

Three rungs, and a claim's rung governs what may be built on it.

**RATIFIED.** The lead designer ruled it, **after the experts had converged on it**. This governs, is
defended rather than weighed, and a later expert file that conflicts with it is drift.

Ratification is the last step and it is rare. It happens when the experts agree this is the best
achievable, every angle has been considered, alternatives have been laid out, and nothing is open.
Only a converged thing is brought to op to ratify. See `01` section 0 for his statement of it.

**ACK.** Op said the direction checks out, before the experts converged. Wanted, worth asking for,
and **not terminal**. An ack does not close a question, does not stop work, and may never be quoted
later as though it did. The previous panel's most expensive habit was filing acks as closures and
then reopening them when a later expert found the gap.

The distinction is not about how firmly he spoke. It is about whether the experts had stopped
disagreeing before he spoke.

**TWO EXPERTS.** Two experts agreed, each having derived its own answer **before** reading the other.
That ordering is the whole content of the rung.

**ONE EXPERT.** Asserted once and never contested, including a long cumulative chain where each file
was dispatched to read and correct its predecessor. Inherited agreement is not found agreement, and
copying a predecessor's framing is how the prior panel drifted. Op, on classifying those
conservatively:

> One expert is a good stance I think, because it forces a double check.

So this rung is a queue entry, not a doubt about the author. A row sitting at ONE EXPERT is asking
for the second read it has not had.

**PERSONA.** The `orgrinrt` persona standing in while op is away. It steers and offers a
designer-shaped perspective, and it has **no authority at all**. A persona call never ratifies, never
becomes canon, is never quoted to an expert as a warrant, and always lands in `PERSONA_CALLS.md`
marked persona-decided. Where it appears to conflict with op's recorded words, his govern and the
conflict is written down rather than resolved silently.

**Agreement among unratified artifacts is not corroboration.** It is shared drift, because agents
copy each other's framing.

### Do not escalate a measurement dispute

Op, on being asked to rule between a bench result and a doubt about it:

> At some point, somebody has to be confident enough about their take on it to write the benches, and
> once benches exist, it's hard to deny what they tell, and if it isn't, then the thing is still
> settling and there's no reason to rule on it

So a contested magnitude is not a question for him. It is answered by someone being confident enough
to build the arm. If the answer stays contested after that, the work is unfinished, and unfinished
work is not escalated either.

### Converge before escalating anything else, too

The same shape applies past measurements. Where experts are still attacking each other's answers, the
question is not ready for op. Bring him a converged thing, with the angles considered and the
alternatives laid out, and ask then.

## What a canon is

Intent, theory, shape. Not a spec sheet.

**Permitted:** pseudocode, accepted mathematical and computer-science notation, LaTeX where it
renders, naming things, and stating requirements, relations, associations, what composes with what
and what excludes what.

**Forbidden:** the concrete spelling of an implementation. No signatures, no field orders, no
parameter lists, no snippet a reader could paste and then believe is the required shape.

**Two tests decide every sentence.** *Permanence*: would this still be true and useful after the
implementation is rewritten from scratch? *Equivalence*: would three teams implementing this
independently produce things that behave the same? If the first fails it is describing an
implementation. If the second fails the canon has said too little, and the gap is a missing intent
rather than a missing snippet.

**The canon must say which things are doable**, which is what probes are for. The evidence lives in
the audit trail and the canon points at it rather than carrying it.

## Dispatch conduct

**A dispatch is an order to go down the rabbit hole.** Report a blocker, then attack it, then the
next one. A blocker reported and left is not a deliverable however well characterised.

**Conceding outranks a manufactured answer.** "I could not find it" is complete and honest. Say what
was tried, why each route closed, where the wall is, and what kind of help would move it. Never
re-propose something already ruled out, and never dress a refused shape as a least-bad option.

**Never stop at the first well-supported answer.**

**Keeping something is a result.** Everything is open, but a panel told only that churns vocabulary
and calls it progress. Where a prior answer survives your derivation, saying so with your own
reasoning behind it is a contribution.

**Say what you did not cover.** Bounded honest coverage beats a claim of completeness. "I read nine
files in full, grepped the rest, and did not verify X" is worth more than silence about the edges.

## Evidence

**Evidence lives in the repo or it never happened.** A spike outside the panel's own `NN_probes/`
may not be named, referenced, or reasoned from, and anything resting on one is void.

**Every probe is a spike.** Cite it for what it proved, never for how it was written. Its names,
arities and field orders are scaffolding, not decisions. Presume it flawed, and **check its feature
gates**: a probe in the prior panel rested on features the design forbids, which was found only when
a later expert looked.

**One instance of evidence is never enough.** Three independent ones is the bar, and independence
means arrived at differently, not three probes sharing one model.

**A measurement outside the bench harness is an ad-hoc quick spike with no substance**, and is called
that. Living in `benches/` is not the test; using the harness is, with real competitor arms rather
than a strawman. Where nothing has been measured on the harness, the magnitude is **unpriced**, and
that word is used rather than reaching for a number.

**Counts are measurements.** Produce every number with a command and say which command. The prior
panel propagated two floating numbers nobody could reproduce, and one expert's headline counts turned
out to be an artifact of its own enumeration bound.

## Writing it down

**Record the options beside any answer that picks among them**, or inline them into the quote as
`[bracketed text]`. "Option 1 is sensible" is unusable once the options live only in a tool call
nobody kept, and this has already sent a settled question back to op.

**Op's input is a numbered file in the panel, at the position where it happened**, and is required
reading for every later member. Folding his steer into the next dispatch prompt instead reaches one
member and paraphrases him; a file reaches every later member in his own words.

**A written artifact standing in for a derivation is a defect**, and is named where it appears.

## The consolidation

It is the canon candidate, promoted whole when op says so.

**A full standalone versioned writeup, never a delta.** Where a section did not change, its content
is written out anyway. The prior panel drifted into deltas until fourteen of twenty-eight
subsections were content-free pointers and roughly eight thousand words sat outside the document
that claimed to contain them.

**The entailment check is run by someone other than its author**, because the author is the person
who believes it entails, and reads the missing content back in from memory. Then repair, then it
stands.

**Count the citations before and after, and diff the sets.** A compression preserves prose and
discards addresses, because addresses mean nothing to the compressor and are the whole value to the
reader. One prior consolidation kept every claim and lost 61 of 78 citations. A rising total is not
reassurance: on a later round the count rose from 100 to 120 while nine unique targets vanished
underneath the additions.

**Repairs restore from the establishing source**, never from the document that lost it.

## Panel mechanics

**One expert at a time, never in parallel.** Sequential and cumulative: each reads the ones before it.

**Every dispatch runs in the background**, so a stall costs a message and is resumed rather than
re-dispatched with its context lost.

**Every member writes its file to disk early and extends it in place.** Several prior dispatches died
holding a complete investigation in a single final write.

**Curated reading, not the whole tree.** `SETTLED.md`, the latest consolidation, the immediate
predecessor, and op's files. The prior panel reached 210,000 tokens of markdown, at which point
telling a member to read the panel was telling it to exhaust its context before starting.

**Native personas**, whose definitions withhold the Agent tool, so a member cannot nest and lose its
work underneath.

## The canon gate, which every member runs before its assigned work

Check that the work, and the state it builds on, aligns with what is ratified. Misaligned means
refuse and return early with the conflicting text and the offending reference. Ambiguous means stop
and hand the call back rather than resolving it.

**An early return is a successful dispatch**, and the most valuable thing a dispatch can produce. It
is never re-dispatched with a softer brief until someone answers, because reframing until you get an
answer converts a refusal into permission.
