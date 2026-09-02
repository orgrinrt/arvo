# 85. Op: never any runtime checks, stop policing law shapes, and the constraints were always intents

Op, in this panel, on 2026-08-13, answering four questions put to him while `84` was in flight. His words
are verbatim and complete in each section. What he was choosing among is recorded beside each, because the
options lived in a tool call nobody kept and the answer alone would not be actable.

Three of the four correct the question rather than answering it as posed. That is the content.

## 1. Never any runtime checks, ever

**The question.** Which binding times the design wants for "validate", given that op had answered on *what*
is validated (admissibility, usage, self-validation, "all that makes sense") and not on *when*, and that
`68` had argued a runtime ingest check is forced for a representation because stored bits are not
self-describing.

**He was choosing among:** (1) compile time only, per type, where all the panel's evidence sits; (2) both,
at named places, compile time per type plus a runtime check at a declared ingest boundary; (3) runtime
ingest as primary with the compile-time acts as consequences.

**He took the first, and sharpened it past what the option said:**

> Option 1, but I should be clear on this: Runtime code can exist, it's only that the branching of it
> should be done as much as possible, with const-time ifs that get erased via monomorphisation and just
> const time solving and ultimately llvm. Never any runtime checks, ever. We catch invalids on compile
> time, and unused paths we clear out when lowered. Period.

**Q-A is closed.** The runtime column of the three-by-two grid does not exist. Not as a cost to be weighed,
not as a door for untrusted data, not as a fallback where a compile-time answer is unavailable.

What this settles, and it reaches further than the question asked:

- **`68`'s ingest-boundary argument is refused as a design option.** It remains a correct observation that
  stored bits are not self-describing; it does not license a runtime check. Whatever answers it has to
  answer it at compile time or not at all.
- **`80` section 5.1's value-gated arm is out**, and its measurement is now the reason rather than a cost
  to trade. That shape materialises both lowerings and selects with a `csel`, which is precisely a branch
  that survived to runtime instead of being solved and erased.
- **The target is one lowered path.** Branching is const-time, erased by monomorphisation, with unused
  paths cleared at lowering. "As much as possible" governs the branching; "never, ever" governs the checks,
  and those are two different strengths in one sentence.

## 2. Stop policing what shape a law takes

**The question.** `82` measured that "available at const time" is not one thing: four constructions all
count as const-time and give four different guarantees, and the one `76`, `77` and `80` all built is the
weakest, accepting a false declaration that sits in an unreached `pub fn`. It asked what a permission must
be, offering: the strongest rung only; any const-available rung; or leave it to the design tier.

**He rejected the framing:**

> I do not think I get the framing. Monomorphisation and const solving should lead everything to go through
> one lowered path, that's it. So if a law is a law, it should be expressed so that it actually works, be
> it typestate or const expressions or whatever. We shouldn't police what kind of laws there are or what
> shapes they take. The law is defined as makes sense and is applicable in each situation on a case by case
> basis.

So the requirement on a law's expression is **functional, not structural**. It must actually work, meaning
it must reach one lowered path through monomorphisation and const solving. Which construction gets it
there is a case-by-case matter and the canon does not rank the constructions or mandate one.

This is `arvo-toolbox-not-policer.md` arriving at the law layer, and the coordinator's question was asking
the canon to do the policing that rule forbids.

**`82`'s measurement is untouched and is re-read rather than discarded.** Four rungs with four binding
times is a fact about those four constructions. What changes is what follows from it. The
reachability-dependent rung is not a category to be banned; it is a construction that **fails the
functional test in that use**, because a permission a consumer can hold while being wrong has not actually
worked. A different use where it does reach one lowered path is fine. The finding is now a caution for
whoever picks a construction, not a rule about which constructions exist.

## 3. The long-standing constraints were always intents, and the provenance trace was beside the point

**The question.** Q-B, open since checkpoint six: `no_std`, no `alloc`, const sizes, no `dyn`, no `TypeId`
appear nowhere in `INTENTS.md`, and the coordinator had traced them to a design document in another
repository predating this one. The question offered: they are his intents; they are inherited assumptions
the canon must re-derive or drop; or some are and some are not.

**He answered, and corrected the premise:**

> They are very explicitly also arvo intents and rules. The mockspace already contains the lints it wants
> and the workspace and repo's own rules all direct this work. No std, no alloc, all that is explicitly
> already in place and not to be questioned. You can write them as intents though, since they are also
> that, but it should've been clear. It doesn't trace to polka-dots, it traces to this fucking workspace
> and its rules... and this very mockspace of arvo's own...

**Q-B is closed and the panel was wrong about it.** These are intents and rules, they are in force, and
they are not to be questioned. They are entered into `INTENTS.md` on his explicit direction here.

**The coordinator's error, named, because it propagated.** The provenance hunt found an old design document
containing the same sentences and concluded that the constraints were "inherited receipts" and therefore
"unratified ground". That reasoning treated a rule's *earliest textual appearance* as its authority, when
the authority was the workspace rules and arvo's own mockspace configuration the entire time, both of them
live, both enforced, both sitting in the same tree the search ran in. Finding an older copy of a rule does
not demote the rule.

The consequence was not confined to one file. Checkpoint `69` raised it as a question for op; `76`, `77`,
`79`, `80` and `81` each carried some version of "the erasure argument rests on unratified ground"; and
arvo's own generated agent instructions were edited to say these constraints are not op's intents and must
not be cited as ratified. That last one is the worst of the set, because it is a live surface that every
future agent in the repo reads, and it was made wrong by an edit intended to make it right.

**Nothing that rested on the constraints needs redoing.** Every argument built on `dyn` and `TypeId` being
absent stands, and stood the whole time. What was wrong was the hedge attached to it.

## 4. The twenty-one D-numbered decisions are dead

**The question.** Twenty-one decisions attributed to op sit outside `INTENTS.md`, in the formalization
talk's topic file, numbered to D75 and marked "Decision (op, ...)". Offered: dead, do not mine them; mine
them case by case and re-quote anything still meant; or they are live as they stand.

**He chose the first: dead, do not mine them.**

They belong to the failed lineage with the rest of the prior calls. Useful for inferring taste and for
giving experts things to test, never authority. Nothing is promoted from them and the panel re-derives
anything it needs.

## Rung

**All four are op's own statements in this panel, in his voice.** He did not mark any of them as a separate
ratification and none is entered as one. Sections 1, 3 and 4 close questions the panel had put to him;
section 2 declines to answer one and says why, which is itself the answer.

Required reading for every file after it. `84` was dispatched before he spoke, was told by a message
pointing here rather than paraphrasing, and its brief listed two of these as questions it was not to
answer. Both are now answered and neither is its to reopen.
