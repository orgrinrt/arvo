# 153. Dispatcher note: opening the ninth unit, and what a new coordinator session costs

This panel paused on 2026-08-15 at a clean boundary, after `152` cleared the strategy-object
candidate. It resumes now under a different coordinator session, which is not a detail: one
mechanism this panel depends on does not survive that change, and the first section below records
what was tried, what the error said, and what it means for how the ninth unit is run.

The rest is the frontier as verified against `git log` rather than against the continuation state,
which was two days stale in one direction and eleven files stale in another, and then the
coordinator's call on which topic comes next, made under the criterion op gave at `87` section 3 and
marked attackable on the record exactly as that call was.

Nothing here is a finding about arvo. It is bookkeeping, and it is a numbered file rather than a
chat message because the reading list is what reaches every later member.

## 1. Member handles are session-scoped, and `HANDLES.md` does not say so

`HANDLES.md` records a task id per member and states the property it was built for: a resume by name
stops working once the coordinator compacts, and a resume by task id keeps working. That is true and
it was paid for. What it does not say, because nobody had tried it, is that the id is an address
**inside the session that dispatched the member**.

Tried, from this session, against `109`'s handle:

```
SendMessage to a11df87e2a4a70ded
-> Agent "a11df87e2a4a70ded" could not be resumed:
   No transcript found for agent ID: a11df87e2a4a70ded
```

The transcript exists. It is at
`~/.claude/projects/-Users-orgrinrt-Dev-clause-dev/dea47dac-5762-46b6-956e-0d22cc5d3832/subagents/agent-a11df87e2a4a70ded.jsonl`,
which is the recovery path `HANDLES.md` itself documents, and every member of this panel from `03`
to `152` sits in that same session directory. The file being readable and the agent being addressable
are different things, and only the second one is what a resume needs.

**So the refute-reply loop is unavailable across a session boundary.** Every member of this panel is
now a spent instance whose file stands and whose transcript is a record rather than a correspondent.
An objection to `109` cannot be answered by `109`; it can only be answered by a fresh expert who has
read `109`, which is a new opinion rather than a reply, and the rules are explicit that those are not
the same thing.

Two consequences, and the second is the one worth carrying:

- **Within a session, record handles as before.** The mechanism works and the compaction hazard it
  was written for is real. This unit records its own handles under a new heading naming the session
  they belong to, because a table that silently mixes two sessions' ids is worse than no table:
  every row reads as resumable and most are not.
- **A unit that will outlive its coordinator's session should not bank on being resumed.** That
  changes what a cold derivation is worth, upward. A file written blind is the only artifact whose
  value does not depend on its author still being reachable.

## 2. The frontier, verified against git rather than inherited

Eight topics have run. Each of the last three carries a member-written ledger and an independent
check, and `AGREEMENTS.md` points at them rather than re-compressing them, which is the right shape
and stays that way.

- **Topic one, the format concept.** Candidate `63`, check `64`.
- **Topic two, number systems.** Candidate `74`, check `75`.
- **Topic three, derived algebraic laws.** Candidate `90`, check `91`, priced at `92`.
- **Topic four, the strategy axis.** Candidate `106`, check `107`, attacked after the fact at `108`.
- **Topic five, the primitive.** Cold pair `109` and `110`, attack `111`, `112`, op's steer at `113`.
  A statement is offered at `112:904-945` and **no candidate, no ledger, no signature round and no
  check**. Section 3 is about this.
- **Topic six, the realisation map.** Candidate `119`, superseded clause by clause by `122`, check
  `123`, one open item closed at `124`.
- **Topic seven, the rounding axis.** Candidate `132`, revised at `136`, check `137`, restored at
  `138`.
- **Topic eight, the strategy object.** Candidate `146`, revised at `151`, check `152`.

`OPTIONS.md` is current to `152` and runs to Q64. `mock/canon/` does not exist, which is correct: op
settled at `87` that nothing moves there until every topic is done and the canon is written from the
consolidations as one act.

## 3. Topic five's answer was offered once and never put through anything

`109` and `110` were dispatched blind and in parallel on one question, what a primitive is. They
converged on two things and **differed in the shape of their answers**, which `AGREEMENTS.md` records
and which was live when the unit moved. `111` attacked both. `112` took up where a refinement lives,
op's `113` told the members to build it together, and `114` formalised that. From there the work
became the realisation map, which ran to a candidate, a revision and a check as topic six.

That was a reasonable move and it produced a finished topic. **This section overstated what was left
behind in its first draft, and the correction is the useful part.** It said the question had no
convergence at all. It has one: `112:904-945` offers a statement of what a primitive is, and its
author says `109`, `110`, `111`, `108`, `106` and `40` jointly support it with that file's corrections
applied. Its first clause is the answer to the question the topic opened.

What that statement has never had is any of the machinery this panel uses to promote something. **No
signature round, no revision against dissent, no independent entailment check, and no second
derivation that reached it without reading it.** One author offered it, the unit moved to the
refinement half, and the refinement half is what `114` formalised and what `119` and `122` carried to
a candidate. `122` section 4 is the realisation map end to end and defines no primitive.

So the gap is precise rather than total, and it is the gap that matters: the panel's central noun sits
on one file's offer, at one expert, unchecked. `AGREEMENTS.md` has a section for topics one through
four, six, seven and eight, and none for five, which is the same absence stated from the other side.

Under op's `87` that absence is a defect rather than a closed question: the canon gets written from
the consolidations read alongside their members, so a topic with no ledger is a topic whose findings
are reachable only by someone who already knows to look for them.

## 4. The call: the ninth unit is the primitive as a named composition

**This is the coordinator's call, made under the criterion rather than by op, and it is attackable
on the record.** Op declined to pick a topic at `87` and gave the criterion instead: strictly
bottom-up, and among what is available at the current tier, take what settles the most downstream at
once.

The reasoning is a fact about the panel's own record rather than a preference. When topic four was
deferred, the stated blocker was that a primitive is a named composition of a format, a number
system, a law set and a strategy, and that the composition could not be attempted while one of its
inputs was a placeholder. Every one of those inputs now has a candidate: `63`, `74`, `90`, and the
strategy axis twice over at `106` and `151`. The blocker that deferred it is gone, and nothing else
at this tier settles as much at once, because everything above it (which operations exist, what the
surfaces are, how the tree decomposes) is stated in terms of it.

**What the unit does differently, forced by section 1.** It cannot resume `109` or `110`, so it opens
with a **fresh cold pair on the same question**, dispatched blind and in parallel on the two-phase
protocol. That is not a repetition. `109` and `110` are two instances and the panel's bar is three
independent ones, so a blind pair that has read neither is exactly what the question is short of, and
whether the new pair lands on `112`'s offered statement without having seen it is the strongest
evidence available for or against it. The prior files are then read in phase two, where reconciling
with them is the assignment.

## 5. Shared inputs, named up front, because Q64 says to

Q64 records that both of topic eight's cold derivations read the same auto-loaded workspace rules,
one of which states a mechanism they both used, so wherever that mechanism did the work the two files
were one instance. Nobody had told them what they shared.

Every dispatch in this unit names its shared inputs in the brief and asks the member to report which
of its conclusions ran through them. For the cold pair those inputs are `INTENTS.md`, `RULES.md`,
this repository's own `.claude/` rules, the workspace's `.claude/rules/` which load automatically,
`mock/Cargo.toml` and its comments, and `mock/benches/`. That list is in both briefs verbatim.

## 6. Handles for this unit, by session

The ids below belong to session `72eb2106-3756-4021-ac53-3ed816d184dc` and are addressable only from
it. They are recorded here at dispatch time rather than after the fact, per the rule that a handle
written down afterwards is a handle that was missing during the window it was needed.

Recorded in `HANDLES.md` under its own heading naming this session.
