# Op: Warm imitates plain Rust, and the strategy axis is what makes an answer correct

**Date:** 2026-08-08. **Position:** after `36`, closing the hole it named and correcting its framing.
**Required reading.**

> **Corrected by `38`, and this file may not be read without it.** Two things below are wrong. This
> file sorts the carrier question into answers of different **kinds**, with `Cold` "decided by intent,
> and possibly not by measurement at all" and "a measurement could not overturn this". Op: "All of
> them should be decided by measurement, just measuring different things, and, this is I think the
> mental unlock: They weigh different measurements differently." And this file reads `Warm` as defined
> by imitation, where its intent is to be the intuitive best choice, with mimicry downstream of that
> and droppable if consistently worse. His instruction on the framing: "You should not write these as
> clear cut and settled." Full correction in
> `38_op_the_strategies_weigh_measurements_differently.md`.

## His words, verbatim

> Warm is what regular old rust would do. In fact, this is the one thing you can check from the prior
> panel. I think the strategy conceptually was well enough defined and split. The strategies aren't
> orthogonal to the threaded question you had, or its answer, strategies are the variables that change
> what the "correct" answer is for what we choose as the path.

## First, the part that is about this panel's competence rather than about arvo

`36` reported `Warm` as a named hole because op had described three strategies and there are four. **The
hole was not real. This panel was already carrying the answer, ratified, in its own seed**, and the
dispatching agent asked op for something on the shelf.

`seed/SETTLED_strategy.md` section 3, marked **RATIFIED**, is op's own words quoted from the prior
panel at `140b:16-21`:

> My standing call is "It should behave like native primitives in regular old rust would"... The
> intent, here, is what matters. The mechanisms and theory may live freely and shift under and around
> it, the intent is what remains and matters.

The same file's section 2, also **RATIFIED**, carries the full four-way intent statement:

> `Hot` is as fast as possible, `Cold` stores as small as possible, `Precise` is the most precise at
> the price of both storage and compute, `Warm` is the compromise that suits most default cases and
> behaves intuitively.

And the seed already records that this specific call **kept failing to stick**: "It was re-stated by op
twice in two days because it kept failing to stick in the standing base (`140b:37-52`), which the panel
itself flags as a presentation defect rather than a content dispute."

Tonight is at least the third such restatement. **The presentation defect the prior panel diagnosed is
still live, and this panel reproduced it within a day of opening.** The seed exists precisely so nobody
reads 320 files, and the strategy sweep is one of four carried files, and it still was not consulted
before the question went to op.

That is the fifth instance in this panel of the same shape: the largest apparent hole turning out to be
something the repository already knew and had stopped carrying forward.

## What tonight adds beyond the ratified statement

The ratified claim 2 is four one-line intents. `34` and `36` are substantially richer, and the extra
content is new rather than a restatement:

- **`Hot`** may sacrifice soundness, as its explicit purpose rather than as tolerated imprecision,
  bounded by a provable meaningful gain. "As fast as possible" does not say that.
- **`Cold`** has **leeway to be non-efficient because the path is cold**. That is a licence granted by
  the intent, not a cost grudgingly accepted, and "stores as small as possible" does not say it.
- **`Precise`** throws out **both** the hot and the cold optimisations, and holds **within chains and
  ops, not only alone**. The composed-computation clause is entirely new.

So the ratified rung and tonight's files are not in conflict; tonight is a refinement of a standing
call, and where they differ in detail the refinement is the later word from the same person.

`Warm` alone gains nothing tonight beyond confirmation, and `Warm` alone is the one defined by
imitation rather than by an objective, which is the asymmetry the prior panel named as load-bearing.

## The correction that matters most, and it is not about Warm

> The strategies aren't orthogonal to the threaded question you had, or its answer, strategies are the
> variables that change what the "correct" answer is for what we choose as the path.

`32` recorded arvo as adapting to the cores it finds, doing "what is most efficient" per situation, and
the dispatching agent treated the strategy axis as sitting beside that question. **That framing is
wrong.**

**The strategy is what defines what "most efficient" means.** It is not a separate dial applied after
an efficiency answer is computed; it is the variable that determines which answer counts as correct.
So the adaptation rule in `32` is **parameterised by strategy**, and it has as many correct answers as
there are strategies.

Three consequences, each testable and none settled here:

**A single answer to a strategy-spanning question is now suspect by construction.** `34` said this
about soundness and `36` extended it to the axis. This states the general form: any question whose
answer could differ per objective has four candidate answers, and one answer means the axis was
collapsed somewhere.

**The packing measurements answered the question for one objective, not for the design.** `26` and `27`
compared packed against dense by **throughput**, across one and four cores, and produced break-even
carriers. Throughput is `Hot`'s objective. It is explicitly the axis `Cold` is licensed to lose, and
`Precise` throws out both hot and cold optimisations, so neither file's answer is `Cold`'s answer or
`Precise`'s. Both files independently noted they never priced footprint, which is the axis `Cold` is
named for. **The measurements are not wrong; their scope is narrower than it read.**

**The regime question and the strategy question compose rather than stacking.** Detected core count
and chosen strategy together select the answer, so the space is at least two-dimensional, and `27`'s
one-core and four-core bands are two cells of a larger table rather than two rival answers.

## Two further clarifications, same moment, and they resolve the carrier question per strategy

> And as to cold: It's intent is that it is for cold paths, cold use. Which means, it should remain
> small for memory or disk storage, because it's just sitting basically.

> If throughput gives *performance* wins and efficiency gains, then that is for Hot, and that is why
> Hot would choose those options.

**`Cold`'s objective is residency size, and the reason is that the data is at rest.** "Because it's
just sitting basically" is the justification for the licence, and it names **memory or disk**, so the
objective spans both and is not only a cache-occupancy question. That is a stronger and narrower claim
than "stores as small as possible": it says *why*, and the why is what makes the non-efficiency
leeway coherent rather than merely permitted.

**And the throughput measurements are `Hot`'s evidence, precisely.** Not scope-narrow and therefore
weak. `26` and `27` measured which arrangement is faster, and faster is what `Hot` optimises for, so
those two files answer **`Hot`'s** carrier question with real numbers. They are the strongest evidence
in the panel and they are evidence about one strategy.

Which changes what Q7 is. It is not one question with a contested answer. It is at least four
questions, and they have different **kinds** of answer:

- **`Hot`**: decided by measurement, and largely decided already. `27`'s break-even carrier bands, per
  detected core count, are the answer, subject to `34`'s condition that a soundness trade needs a
  provable meaningful gain.
- **`Cold`**: decided by **intent**, and possibly not by measurement at all. If the objective is
  smallest residency for data that is sitting, then the packed form wins by definition of the
  objective, and the throughput deficit is the leeway the intent already granted. **A measurement
  could not overturn this**, which is worth stating plainly, because the panel's instinct on finding
  an unmeasured axis is to go and measure it.
- **`Precise`**: decided by neither of the above, since it throws out both the hot and the cold
  optimisations. Its carrier question is whatever accuracy across chains requires, and nobody has
  asked it.
- **`Warm`**: decided by imitation. Whatever plain Rust does with a native primitive is the answer,
  which makes it the only one of the four whose answer is looked up rather than derived.

**So the footprint gap named above is not a missing measurement for `Cold`.** It may be a category
error to look for one. What is genuinely missing is whether `Cold`'s intent picks a *unique*
arrangement or leaves a choice among several small ones, and that is a design question rather than a
bench.

## What is now owed, and by whom

**The seed's strategy sweep should have been read before the question was asked**, and every remaining
question that touches the strategy axis should be checked against it before it goes anywhere. That is a
dispatcher failure and the correction is procedural rather than a design finding.

**`Precise`'s objective has no measurement and no analysis at all.** Accuracy across chains is named
in the intent and, per `18`, is precisely where the absorbing-endpoint result says soundness is
conditional. That is a real gap.

**`Cold`'s is not the same kind of gap**, per the clarification above: its answer may follow from its
intent rather than from evidence, so the correction to this file's earlier framing is that footprint
being unpriced is not obviously a defect. What is open is whether the intent picks one arrangement or
several.

## Standing

Op says the strategy concept "was well enough defined and split", and that these descriptions can go
into the canon while remaining open for discussion and exploration.

The four intent slots and `Warm`-by-imitation are **RATIFIED** on the prior panel's record, quoted
above with their provenance. Tonight's refinements to `Hot`, `Cold` and `Precise` are **not**: they are
direction of high confidence, marked canon-bound, and under op's own correction an opinion given before
the experts converge is an ack rather than a ruling. Nothing here may be cited as settled beyond the
two ratified claims, which may be cited as exactly what they say and no further.

The register is not edited from this file because a member is reading it. The edit follows when that
dispatch lands, and it must carry the strategy parameterisation into the carrier question rather than
leaving `26` and `27`'s answers reading as unconditional.
