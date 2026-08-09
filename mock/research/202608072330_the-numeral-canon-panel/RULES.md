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

**Counts are measurements.** Produce every number with a command and say which command.

**Check your own citations before shipping, by opening them.** A reference that resolves is not a
reference that says what you claim. One member found **seven of its own citations wrong** this way and
built a probe that opens every `file:line` and tests its content rather than its resolution; forty-one
citations, zero failures after. That probe lives in `25_probes/` and is the cheapest correctness tool
this panel has. Five separate instances of this failure were recorded across two panels before anyone
wrote it. The prior
panel propagated two floating numbers nobody could reproduce, and one expert's headline counts turned
out to be an artifact of its own enumeration bound.

## A shared source read by nobody is a single point of failure

Two files now rest on `16` without either having read it: `35` and `43` both reach it through
`OPTIONS.md`'s account, and both said so. That is the register working as intended, and it is also
exactly the condition under which one compression's error becomes two files' error and reads as
corroboration.

**Where two or more files depend on the same unread source, the next dispatch on that material reads
the source itself.** Not the register's account of it. The register is a working surface and its
provenance section says so; it was assembled twice and missed a flagged question both times.

State the exposure the way `43` did, by name: which file you leaned on, which you did not read, and
which of your sections would move if the account were wrong. A coverage bound naming the specific risk
is worth more than a claim of completeness, which this workspace has repeatedly caught being unmeasured.

## The cadence: one 4-4-1 is one topic, end to end

**Four experts on the topic, op's checkpoint, four more experts on the same topic, then the
consolidation. Nine files. Then the next 4-4-1 takes the next topic.**

Op stated it plainly on 2026-08-08 after finding the panel doing something else:

> The intended way is that the 4-4-1 is one topic, the whole thing. Then the next 4-4-1 picks up the
> next one. Then the next 4-4-1 too, etc.

**The consolidation is that topic's canon candidate.** This is the answer to "when does the canon get
written": it does not arrive at the end of the panel. **It accretes one topic at a time**, and each
completed 4-4-1 produces the candidate for its own subject.

### What the panel did instead, and it was never right

Not once. The stretches that looked compliant were four experts on **four different subjects** with a
checkpoint after: `03` the family question, `06` where a numeral is inferred, `07` the adjunction
frame, `08` alternative number systems. A checkpoint every four files is not the cadence; it is the
punctuation of the cadence without its content.

After `19` even that stopped, and **no consolidation has ever been written here**. So the mechanism
that produces canon text was never run, and the dispatching agent read the resulting absence as the
explore-do-not-settle mode working. The mode forbids **settling**. It never forbade consolidating, and
conflating the two turned a missed cadence into an excuse.

### Why one topic per unit, rather than one topic per expert

A dispatch per subject is a survey. It reads like a panel, because every file cites its predecessors,
and it cannot do what a panel exists for.

**TWO EXPERTS requires two independent derivations of the same claim. Ratification requires the experts
to have stopped disagreeing.** One expert per subject makes both unreachable by construction: every
finding lands at ONE EXPERT and stays there however strong it is, and nothing can ever be brought to op
as converged. That is a fact about the shape, not about the evidence.

Eight experts on one subject can converge, and can locate exactly where they do not. **Disagreement
surviving eight passes is itself a result** and goes to op as one.

### A consolidation loses live options, structurally, and both of them have

**Two consolidations, two dropped live options**, each found only by the independent check afterwards.

Unit one's dropped a connection **three separate members** had flagged as real-but-unverified; it
appeared in the consolidation zero times. Unit two's dropped an alternative its source states in full
with its own discriminator against the option that survived; it appeared zero times while its sibling
appeared once.

**The mechanism is worth understanding, because it is not carelessness.** A consolidation compresses
what the unit *established*. An option that no member resolved has no result attached, so there is
nothing for a compressor to grip, and it falls out precisely because it is still open. **The options
most likely to be lost are the ones the panel most needs carried**, since a settled question does not
need the register and an open one does.

That is also how this project permanently lost a settled answer once already: the options lived only in
a place nobody kept.

**So a consolidation carries an explicit pass over live options**, separate from its pass over results,
and lists each with its costs and with whatever would distinguish it from its neighbours. The check that
follows diffs the option sets, not only the claims, because a claim-by-claim entailment check scores a
dropped option clean: nothing it asserts is contradicted.

### A rung needs someone who derived before reading, and that gets harder every file

Every agreement in the panel's first proper unit turned out to be a **read**. Four files agreed, and not
one derived before reading its predecessors, so the claim sat at a weaker rung than the file count
suggested. The dispatching agent then wrote the inflated rung into the register, which is the third
instance of that failure in one night.

**So a unit needs at least one cold derivation, and the sooner the better.** The protocol:

**Phase one.** The expert reads only the premises: op's intents, the acceptance criterion, the workspace
rules. **No panel files, no register, no probes, no git log, no commit messages.** It derives its own
answer, builds its own evidence, writes it, and **commits** it.

**Phase two.** Only then does it read the panel, and it appends a reconciliation saying where it agrees,
where it does not, and what it would change. **Phase one is never rewritten.** Its whole value is that
it was written blind, and editing it afterwards destroys exactly the property it was dispatched for.

A cold derivation that turns out wrong, honestly reported, is worth more than a right one that read the
answer first. The first is evidence; the second is an echo.

**Dispatch it early.** Every file added makes a cold derivation harder to obtain, because the pool of
experts who have not been told the answer shrinks and the answer leaks through the reading list. A unit
that reaches its checkpoint with no independent instance has to spend one of its remaining four slots
buying one.

### The eight are an argument, not a relay

Op, immediately after stating the cadence:

> The experts have to have time to argue, support, fight each other, then converge over several back
> and forths

**So a topic is not eight people speaking once each in order.** That is a relay, and a relay converges
only by luck: the eighth file inherits seven framings and agrees with them, which is the shared-drift
failure wearing the cadence's clothes.

What is wanted is an exchange. An expert states a position. A later one **attacks** it, or **supports**
it with its own derivation, and the first is **brought back to answer**. Positions move, or they harden
with the reason for hardening now on the record. Convergence is what happens when the attacks stop
landing, and it is **reached rather than scheduled**.

Three things follow for how dispatches are made.

**An expert is resumed, not replaced.** A background dispatch continues from its own transcript with a
message, so the expert that made a claim is the one that answers the objection to it, with everything
it derived still in context. Re-dispatching fresh loses the position and the reasoning behind it, and
produces a new opinion rather than a reply.

**Support counts as much as attack.** An expert that independently derives a predecessor's claim and
says so has produced the TWO EXPERTS rung, which is the whole point of the rung and cannot be reached
any other way. Agreement arrived at independently is a result; agreement inherited by reading is not.

**Several rounds, not one.** A single attack and a single reply is not convergence, it is one exchange.
The topic runs until a pass adds nothing, and if that has not happened by the eighth file, the honest
output is the located disagreement rather than a manufactured settlement.

### Consequences for how a topic runs

**A topic is finished when a later expert adds nothing**, not when a file exists about it.

**The checkpoint sits in the middle for a reason.** Four files in, op can redirect the second half
while it still costs four files rather than nine.

**The consolidation is standalone and versioned, never a delta**: a section that did not change has its
content written out anyway. And it is checked by someone who did not write it, working from the member
files forward, because the author of a compression is the person who believes it entails.

## The mode: explore, do not settle

Standing for roughly the panel's first hundred files, on op's instruction. Full statement in
`00_brief.md`; his own words and the eight open questions are in `28_op_answers_two.md`.

**A file that closes a question on its own authority has misread the assignment.** One that opens
three well-argued directions and says what would distinguish them has read it correctly.

**Every option is carried in `OPTIONS.md`, written out in full.** Your file says which options your
finding fits well, which it fits badly, and which it kills. All three are results, and fitting badly
is not killing: the difference is whether the option survives at a cost or does not survive.

**Killing an option is still allowed and still valuable.** Not settling means not choosing among live
options, never carrying dead ones. A closed route moves to `DROPLIST.md` with its diagnostic and with
what would have to be overturned to reopen it. The option space shrinks from the bottom the whole way
while nothing is chosen at the top.

**Adding an option is the most valuable single act available.** An option set is never a boundary. Op,
widening one of the eight: "don't even restrict the panel to these three. Free reign to converge by
theory and logic to the best one that serves all other parts of arvo best." That last clause is the
selection criterion, and it is why every option has to stay visible at once: an option's value is
partly a fact about the others.

**Exploring is not declining to conclude.** A file still owes findings with citations, probes that
establish or refute, and a plain statement of what its evidence supports. The line is between "the
evidence closes this route, here is the diagnostic" and "therefore the design should be X".

**Op's statements are direction unless he names them locked**, and he has said he will name the
locks. His recorded instinct that there is one numeral family is exactly this case: it is an instinct,
he said so, and he said not to act on it. Treating a preference of his as a ruling is the previous
panel's failure committed with his words instead of the panel's own.

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

### The reading list needs a slot for the repository, and did not have one

Found at the third checkpoint of this panel's first night, and it cost eighteen files.

Every brief carried the line that `mock/crates` is being nuked. Every member read that as **the
repository is not evidence**, which is not what it says and not what is meant. The consequence:
eighteen files reported the packed-storage trade as **unpriced**, while `mock/benches/` held a
committed harness run measuring exactly it, with confidence intervals and significance. No file in
the panel named that directory. The panel argued for a night about the half of a trade it could see.

So a brief says explicitly what in the repository **is** evidence and where it lives:

- **`mock/benches/`** is committed harness output and is the only thing in this workspace that can
  price anything. A claim that something is unpriced is a claim about that directory, and it is
  checkable in one command.
- **`mock/research/`**, including closed panels, is the audit trail. Reachable when a specific
  artifact is needed, named as such.
- **`mock/crates`** is the dead tree: readable for what was done and what not to do, never cited as
  evidence about what is correct.

The general lesson is worth more than the fix. **A negative claim about evidence is a claim about a
place**, and it can only be made by someone who was told where to look. "Unpriced" said by a member
who does not know the bench directory exists is not a finding, it is a description of the brief.

### Cumulative reading makes the TWO EXPERTS rung unreachable unless the dispatch says otherwise

Noticed at the first checkpoint of this panel, and it is a defect in these rules rather than in any
file. Cumulative sequential reading is mandated above. Independent derivation is required for the
TWO EXPERTS rung. A member that reads its predecessor first cannot then derive independently, so
**every agreement produced by the default dispatch shape is inherited by construction**, and the
middle rung can never be earned. Three members of the first stretch flagged inheritance about
themselves; none noticed that the shape guaranteed it.

The fix is the dispatch, not the rule. **When a dispatch exists to second-read a specific claim, it
says so explicitly and inverts the order for that claim only:** derive your own answer first, write
it down, and only then read the predecessor and say where you agree and where you do not. Everything
else in the reading list is still read first, because the point is independence on one question
rather than ignorance of the panel.

A dispatch that does not say this produces a ONE EXPERT result no matter how many files agree, and
the dispatcher records it as such.

**And the dispatch must forbid reading the panel's commit log before the answer is written.** Found
the hard way: an independence dispatch ran `git log --oneline` as its first orientation command, and
one subject line carried the predecessor's headline conclusion. Everything downstream of the number
in that subject was contaminated before a line of derivation existed. The expert declared it and
downgraded its own agreement, which is the right handling, and the cheaper fix is to say up front
that the log is off limits until the answer is on disk.

Commit subjects in this panel are written to be informative, which is correct for every other reader
and is exactly what makes them a leak here.

**Native personas**, whose definitions withhold the Agent tool, so a member cannot nest and lose its
work underneath.

## The canon gate, which every member runs before its assigned work

Check that the work, and the state it builds on, aligns with what is ratified. Misaligned means
refuse and return early with the conflicting text and the offending reference. Ambiguous means stop
and hand the call back rather than resolving it.

**An early return is a successful dispatch**, and the most valuable thing a dispatch can produce. It
is never re-dispatched with a softer brief until someone answers, because reframing until you get an
answer converts a refusal into permission.
## Dead files carry an `OLD_` prefix, and that is what makes a bare number safe

**Everything not live is prefixed `OLD_`.** The closed formalization panel, this panel's `archive/`, and
this panel's `seed/`: 328 files. **Nothing in this panel's root is prefixed, because everything in it is
live**, and that was verified rather than assumed when the convention landed.

The prefix does one specific job. Before it, 77 file numbers existed in both panels, and the worst case was
not rare: `63_consolidation_six.md` in the archive against `63_spj_consolidation_the_format_concept.md`
here, **two consolidations, overlapping subject matter, identical citation**. `42` collided the same way and
bit a real dispatch within an hour of the hazard being written down.

Now `ls 63_*` and a glob or grep on a bare number reach the live file only. A dead one cannot be picked up
by accident, because reaching it requires typing `OLD_`, which is a deliberate act.

**So a bare number in this panel means this panel.** That is what the prefix buys, and it is why the
convention is worth more than a warning would have been: the earlier version of this section asked readers
to qualify every citation, which is a rule people forget. This is a rule the filesystem keeps.

**Citing dead material stays legal and stays visible.** `OLD_109_the_consolidation_drop_audit.md` reads as
what it is at every glance, in prose, in a grep result, and in a diff. The 105 archive citations already
written into `SEED_TALKING_POINTS.md`, the three `SEED_THEORY_*` files and `PRIOR_CALLS.md` were repointed
when the rename landed, checked both directions: no unprefixed archive citation remains, and no `OLD_`
citation names a file that does not exist.

**When something in root dies, prefix it then.** The root's non-prefixed state is a claim that everything in
it is live, and that claim is only true if it is maintained.

**The D-number space is not fixed by this** and still collides three ways inside the archive's own decision
register. Round-qualify those.

## Every finding carries its predicate, and a universal answer is rejected by premise

**No finding in this panel states a result without stating the region it holds in**, as an explicit
predicate over every dimension that could move it. That includes findings believed to hold everywhere: a
universal claim is written out as `N any numeral, S any strategy, target features any, threads any, F = 0`
rather than asserted as universal, so it is checkable and cannot quietly narrow while everyone keeps citing
it.

Op, stating why the goal itself is not a universal answer:

> We explicitly reject a universal solution. That is ass. The strategies themselves as a concept make a
> universal solution impossible by premise. We collect and compound answers to specific regions where a
> predicate holds and write the expression where it holds, and the most optimal things that hold true
> there.

If `Hot` and `Precise` weigh measurements differently (I8), no single expression is best for both, so
asking for one asks the design to stop being what it is. **The work is the arms.** Each applies on its own
region and nowhere else, and "everywhere is optimal" is what they add up to rather than something any one
of them achieves.

**`any` is a claim, and `unmeasured` is a different word.** Most of this panel's corpus is single-threaded,
so `threads any` is almost always false where `threads unmeasured` is true. They look similar and mean
opposite things.

**A law failing in most of its space is a finding about where it holds.** The useful output of
"distributivity fails at `F > 0`" is the predicate `F = 0` and an arm gated on it. Look for the predicate
before writing the prohibition; a prohibition is what a finding becomes when nobody looked for its region.

Full statement: `every-finding-carries-its-predicate.md`.

## Op's prior calls are op's voice and are NOT the top rung

The provenance ladder ranks by human ratification, so op's own words normally outrank everything. **There
is one body of op's words where that is reversed, by op**, and an expert meeting it will rank it top by
reflex unless it knows otherwise.

`PRIOR_CALLS.md` collects op's decisions from arvo's earlier history, including the 21 explicitly attributed
"Decision (op, ...)" in the formalization talk. **They are not calls, not ratified intents, and not canon.**
Op's own framing:

> all my prior calls can be mined and should be collected for reference, but not as calls, not as ratified
> intents, but as historical log of my calls, explicitly connected to a *failure* which means they aren't
> "canon" so to say. All of them I've made in earnest, all of them made sense at the time, but none of them
> relate to this new panel or its convergence or settled intents, and should not act as if it did.

And on how much of it to believe:

> none of it absolute. The answers are likely wrong, and the questions they answer, are also probably
> wrong. So substance itself is only good as extra stuff to consider or explore, nothing more.

**The only ratified material remains `INTENTS.md`, I1 through I12.** A sentence in `PRIOR_CALLS.md`
attributed to op does not outrank a panel finding, does not settle an option, and does not close a
question. It cannot be cited to support a claim.

What it is for, and both uses are real:

**Things to test.** The useful question about a prior call is never "was this right" but **"why did he
reach for that, and does the reason hold here?"** A call often encodes a genuine constraint or worry even
where its conclusion is wrong, and the question it was answering is as suspect as the answer. Where a call
is cheaply re-testable against the current shape, testing it is legitimate work and the result of that test
**is** current evidence.

**Inferred taste.** The corpus shows which way op jumps when two options are balanced, what he reliably
objects to, what vocabulary he rejects, and what he finds distasteful independent of whether it works. That
inference feeds the `orgrinrt` persona and feeds what is worth exploring. It is inference and is labelled
as such, grounded in at least two instances per pattern.

**The failure mode to watch for** is the same laundering the canon-defence rules exist to stop, arriving
from an unexpected direction. An agent that cannot support a position finds a prior op call agreeing with
it and cites that. The citation is real, the attribution is correct, and the conclusion is still
unsupported, because op removed that material's authority himself.

