# The intent catalogue

**Op's stated intents for arvo, in his own words, with provenance.** This is the material that
graduates into the canon. Everything else this panel produces is working towards it.

**Opened 2026-08-08**, after op asked whether his ratifiable intents had been written into any
catalogue and the answer was no. They had been recorded only in the numbered audit-trail files, because
the panel's explore-do-not-settle mode was applied to them uniformly. That was a category error: the
mode exists so that **agent-derived conclusions** are not settled before the experts converge, and an
intent op states in his own words is not a derived conclusion. It is the source those conclusions are
measured against.

## What may be in here, and what may not

**In:** intents op has stated, quoted rather than paraphrased, with where he said it.

**Out:** every conclusion drawn from an intent, every per-strategy ruling an agent derived, every
mechanism, and every number. Those live in the panel files and in `OPTIONS.md`, and a reader who wants
them should go there rather than find them dressed as intent here.

The distinction is not decorative. Op, 2026-08-08: "we don't ratify these as absolutes, rather,
*intent* as stated by me here and prior." An absolute derived from an intent is exactly the thing that
must not appear in this file.

## How to read an entry

**RATIFIED** means it sits on the record as ratified with op in the loop, with the provenance shown.
**One entry holds this rung: I13**, ratified by op in the sitting that produced it, in the words recorded
there. The three that previously held it were imported from the prior panel's
`SETTLED.md` classification, which `01` section 0 and the panel's own continuation state had already
established was made under the old reading of ratification and is not to be trusted. Op demoted the
first of them within hours (`39`); the other two were rewritten to rest on his current word instead.
**Do not import that rung again.** An entry earns RATIFIED here only from a convergence brought to op
and blessed, which has not happened for anything.

**STATED** means op has said it and marked it canon-bound, and it has not been through the convergence
that ratification requires. Under his own correction, an opinion given before the experts converge is
an ack meaning the direction checks out. A STATED entry is his intent and is not yet a settled answer.

**A standing instruction applies to every entry below**, and it is his: *"You should not write these as
clear cut and settled. The intent is clear I think, but nothing about them is absolute otherwise."*

---

## I1. ~~The strategy set is closed at exactly four~~ DEMOTED TO OPEN

**OPEN.** Demoted 2026-08-08 on op's direct word, hours after this file opened. Kept rather than
deleted so the demotion is legible. Full statement: `39`.

> the strategy set is not closed at exactly four. These are the ones the last panel settled with, and
> what my amateur ass had written for arvo that we are now redesigning (infer from that what you
> will..), so it's entirely open to discussion and exploration

`Hot`, `Cold`, `Warm`, `Precise` are **a prior attempt at the intent, not the intent**. The number, the
names and the decomposition are all open.

## I2. Each preset names a stated intent, not a derived rule

**STATED, and its enumeration is OPEN.** The shape of this claim survives, because op spent 2026-08-08
stating intents rather than deriving them (`34`, `36`, `37`, `38`). The list of four it enumerates does
not, per I1. Quoted at `seed/SETTLED_strategy.md` section 2, from the prior panel at `70:106-109` and
requoted at `124:2578-2580`.

> `Hot` is as fast as possible, `Cold` stores as small as possible, `Precise` is the most precise at
> the price of both storage and compute, `Warm` is the compromise that suits most default cases and
> behaves intuitively.

## I3. Warm behaves as a native Rust primitive would

**STATED, on op's current word rather than on the carried rung.** He restated it on 2026-08-08 and
refined it at `38`, so its authority does not depend on the prior panel's classification. Whether a
strategy named `Warm` exists in the final decomposition is open, per I1. Originally at `140b:16-21`,
carried at `seed/SETTLED_strategy.md` section 3.

> My standing call is "It should behave like native primitives in regular old rust would"... The
> intent, here, is what matters. The mechanisms and theory may live freely and shift under and around
> it, the intent is what remains and matters.

The seed records that this call was re-stated twice in two days because it kept failing to stick, and
op restated it again on 2026-08-08. Four statements of the same call.

## I4. Warm's objective is the intuitive best choice, and imitation serves it rather than defines it

**STATED.** `38`, 2026-08-08.

> Warm does not merely imitate, its intent is to be intuitive best choice for most every use case, and
> the intuitive part demands it mimics, but it does not make it absolutely required, if mimicking is
> consistently just worse choice.

## I5. Hot may sacrifice soundness, for a proven meaningful gain

**STATED.** `34`, 2026-08-08.

> the intent behind Hot is performance, efficiency, even at the cost of accuracy or soundness

> Hot *can* sacrifice soundness, that is its explicit purpose, but it should not lose it for nothing,
> instead, provable meaningful gains.

## I6. Cold is for cold paths and cold storage

**STATED.** `36` and `37`, 2026-08-08.

> Cold is optimised for cold paths and cold storage, which means, it aggressively minimises and
> bitpacks, *but* because it optimises for cold paths, it has more leeway to do things non-efficient.

> It's intent is that it is for cold paths, cold use. Which means, it should remain small for memory or
> disk storage, because it's just sitting basically.

And the bound on that leeway, `38`:

> Cold does not *have to* drop efficiency wins elsewhere. It can use the same paths Hot uses, not
> because it needs to by intent, but nothing in its intent would fight it. But if the path fights the
> intent, then it's not for Cold.

## I7. Precise is accurate across chains, not only per operation

**STATED.** `36`, 2026-08-08.

> Precise on other hand is the one that sacrifices as much performance and efficiency as makes sense,
> to be the most precise possible answer, throwing out all cold or hot axis optimisations to be
> *accurate* and *precise*, especially within chains and ops, not only alone.

## I8. The strategies weigh measurements differently

**STATED.** `38`, 2026-08-08. Op calls this the mental unlock.

> All of them should be decided by measurement, just measuring different things, and, this is I think
> the mental unlock: They weigh different measurements differently. For the most part, they probably
> agree, because in general, the best answer fits all, because it fights none of their intent. But
> perhaps my instinct is wrong there, and all truly differ for the most part.

The second half is part of the intent rather than a hedge on it: whether the weightings usually agree
is **open**, and he says so.

## I9. The strategy is what makes an answer correct

**STATED.** `37`, 2026-08-08.

> The strategies aren't orthogonal to the threaded question you had, or its answer, strategies are the
> variables that change what the "correct" answer is for what we choose as the path.

## I10. arvo takes no stance on how many cores it runs on

**STATED.** `32`, 2026-08-08.

> We will run in threads = 1, threads = 2, threads = n where n can be any finite. We don't take stances
> on these. If it gives juice and proves more efficient than the alternatives, we should do that, when
> we can detect we have several cores available. When we don't, we do what is the most efficient thing
> in a single-threaded realm.

Read with I5: the soundness condition in the fuller quote at `32` is not uniform across strategies.

## I11. arvo is a library, and the value is what composes on top of it

**STATED.** `32`, 2026-08-08.

> We are a library, not a program, so we don't know how end users will use us, however, our main
> selling point are the algo crates that hilavitkutin, vehje, pretty much every single repo and project
> I have, downstream, use. As well as the contracts for things that compose to bigger units than just
> numerals alone. But we need this base to work, to build the bigger things.

## I12. An opinion given before the experts converge is an ack, not a ratification

**STATED.** `01` section 0, 2026-08-07. Governs how every entry above is read, including itself.

> we don't need to settle this with so loose base. We can explore more. And where there aren't any
> ratifications, it only makes sense to make the experts work it out until they agree and stop
> attacking each other and concede that this is actually good. Until that time, my word is only thing
> that ratifies shit, and the last panel process taught me that I shouldn't go and ratify anything
> before the experts actually agree and have a converged thing to bring to me, with all the angles
> considered and perhaps some options too, alternatives. Until that time, we are wasting time taking my
> opinions as anything other than "yeah checks out, direction is good" acks.

---


## I13. The work is predicated arms composed, and a universal solution is rejected

**RATIFIED.** Op, 2026-08-09, ratifying his own two statements in the sitting that produced them, with the
instruction that the entry mean **no more than he said** and not extend beyond it. His words, both
statements, verbatim:

> Even if "mostly doesn't hold in arvo", it means that still "sometimes holds in arvo" which means, there's
> a nameable predicate for the sometimes which allows choosing the more optimal lowering and arm for that
> specific case. We have to all remember that we are not writing a generalization, rather a bunch of arms
> with const predicates that optimize each little "sometimes" so that all "sometimes" apply on that
> sometimes and nowhere else, thus "everywhere" is optimal by construction as a composition of those
> sometimes

> Add that predicated arm for small wins that compound together as the heart of the work the experts are
> doing. We explicitly reject a universal solution. That is ass. The strategies themselves as a concept
> make a universal solution impossible by premise. We collect and compound answers to specific regions
> where a predicate holds and write the expression where it holds, and the most optimal things that hold
> true there. We should add a rule where all findings have to be predicated, even "universal" ones, so that
> they are exact; currently universal would then probably mean something like "where n is numeral and
> strategy is any and hardware features are any and threads are any" etc. that forces clearly saying where
> a finding is known to hold instead of assuming an ambiguous "universal" that shifts constantly and is by
> design, rejected and unstable

**Op further specified the mechanism**, in the same sitting, correcting an elaboration of the
coordinator's rather than adding to the intent:

> unmeasured or unknown does not list in the predicate. It's not known, it's assumed not true until proven
> true. No adding "unsure" into the predicate. Unsure or unmeasured etc explicitly go unstated and
> implicitly mean not true

He did not mark that separately as ratified, so it is recorded here as his instruction and is not claimed
for the RATIFIED rung. It replaced a proposal of the coordinator's that a predicate should write
`unmeasured` on a dimension nobody checked. It should write nothing there.

**Op further specified what "const predicates" reaches**, on 2026-08-13, answering a question about the
phrase in his own entry. Verbatim, and recorded in full with its context at
`83_op_the_predicate_is_whatever_is_const.md`:

> Let me just add there that the above collapses to whatever is available at const time: Making the
> predicates const expressions for example, allows using const functions and pipe in some data that is
> outside the typestate. However, being const time expressions, typestate is usable there too

He did not mark this separately as ratified either, so it too is his instruction rather than a second entry
on the rung. What it settles: the admissible category is **whatever is available at const time**, which is
wider than the typestate and reaches const functions and const data from outside it, with the typestate
usable inside a const expression rather than being the only source. It was answering a two-way fork the
panel had built, typestate against values flowing through, and it rejected the fork rather than picking a
side. What it does not settle is what happens to a condition that is genuinely not const-available.

**The scope of this entry is those two paragraphs.** Op ratified the call and said explicitly that it means
no more than he said. Anything further, including the dimension list, the `any` against `unmeasured`
distinction, and the exactness bar for a predicate, is elaboration in
`every-finding-carries-its-predicate.md` and is **not** part of what was ratified.

## What this file is not

Not a canon. A canon states intent in its own voice, having established that what it intends is
doable, and this is a catalogue of quotations with provenance. It is the input to that work.

Not a substitute for the panel files. Each entry names where it came from, and the surrounding
reasoning, the connections to open questions, and the corrections between files are there rather than
here.

Not complete. Anything op has said that is not quoted above is missing rather than excluded, and the
remedy is to add it.
