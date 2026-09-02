# Op: the eight questions, answered, and none of them is a lock

**Date:** 2026-08-08. **Position:** after `27` was dispatched and while it runs.
**Required reading for every member from here.**

The eight questions `MORNING.md` reserved for op were put to him before he slept,
in two batches of four. He answered all eight. **Not one answer is a lock**, and
the shape of the answers matters more than their content: he declined to settle
seven of the eight and named the method he wants instead.

Every question is recorded below with **the options it was choosing among**,
because an answer of "option 1" with the options living only in a tool call is
worthless to every later reader, and this project has already lost a settled
answer of his that way permanently.

## The method he named, which governs the rest of this file

Three of his four answers in the first batch reduce to one instruction:

> Whatever is convenient. Let's not get married here, let's consider all options
> as per our standing rules.

And on the denotation question, where the panel handed him three:

> Option 1, but don't even restrict the panel to these three. Free reign to
> converge by theory and logic to the best one that serves all other parts of
> arvo best.

**The selection criterion is stated there and it is not "which is best in
isolation".** It is which shape serves the other parts of arvo best. That is the
same criterion the sibling panel is running under, where op put it as: keep the
options written down and open, reflect each new question over every one of them,
say which would have fit well and which would not, and the path forward appears
when one shape turns out to serve most of the good options elsewhere.

He also pointed at where that method is already written down:

> Read the rules dir and all of its files if you are unclear with the standing
> rules. Or if this new panel didn't get the memo tarina did, check the tarina
> repo's similarly restarted panel and the brief they have, adapt the
> instructions and learnings to arvo as they apply here, and keep them in mind.
> Not sure what all of it was made into actual rules now that I think about it.

That memo had not reached this panel. It has now, and `00_brief.md` carries it.

## Why he is refusing to settle, in his own words

On the family question, which was his own opening question to this panel:

> My instinct says one. But I'm not a mathematician, and we saw how the last
> panel ended in a mess by me making ratifications or calls too early and got us
> locked into a mess. Now we are avoiding those mistakes.

**The failure being avoided is ratifying at the wrong time, not answering
wrongly.** A call made before its consumers and synergies are known can be a
perfectly reasonable call and still lock the design into a shape that does not
fit, and there is no way to know which at the moment it is made. That is why
being more careful about individual answers does not prevent it, and why the
method has to differ.

So his instinct on families is recorded here as an instinct. **It is not a
ruling, and a member citing it as one is making the previous panel's mistake with
his words instead of its own.**

## Batch one

### Q1. Which reading of "then validate" did you mean?

The three readings, from `17`, each demanding a different instrument:

1. **Admissibility.** The typestate refuses declarations it cannot serve. Panel
   evidence: none. Not one of the fifteen expected-to-fail probes is about
   whether a declaration is admissible.
2. **Usage.** The typestate refuses operations that violate the declared
   invariants, with a diagnostic a consumer can read. Panel evidence: real and
   substantial, and all fifteen refusal probes are of this kind.
3. **Self-validation.** The derived container actually holds the declared range,
   checked at derivation time rather than assumed. Panel evidence: incidental.

His answer, selecting all three:

> Usage, Admissibility, Self-validation, All that makes sense. Again, since this
> is the new panel, we learnt from last night, this isn't a strict call, so it
> can be challenged if truly not worth it, but my instinct is that the more
> robust it is, the better it'll serve us.

**All three, and challengeable.** So the clause is not one of three readings but
their conjunction, and the panel's evidence covers one third of what it now has
to cover. The two uninstrumented readings each owe an instrument: a
two-directional admissibility sweep, and a range assertion per declaration.

The challenge route is open and named: a member may argue a reading is **truly
not worth it**, and the bar is that phrase rather than mere cost. His stated
prior is that robustness serves the design.

### Q2. Does a consumer write the integer width, or the total width?

The options: total-and-fraction, which is what the machinery wants and what makes
the negative-width corner naturals; integer-and-fraction, which the surface can
keep through the door at zero cost, at the price of the numbers typed differing
from the numbers stored; or both, with each reflective surface declaring which
pair it shows.

> Whatever is convenient. Let's not get married here, let's consider all options
> as per our standing rules.

Recorded verbatim because the phrase "whatever is convenient" is easy to misread
as indifference. Read against the method he names in the same sentence, it means
**the coordinate system is chosen by what serves the rest of arvo**, and that
answer is not available yet. `15` argued this question is prior to the family
question because it changes what the shape space is a space of; that argument
survives and is now unresolved on purpose.

### Q3. Does the design have a mixed-numeral addition at all?

The options: none exists, in which case addition joins the consumer-determined
block and the whole inference surface collapses to multiplication plus the
container; it exists and must be inferred; or it exists only through an explicit
conversion, which collapses the surface the same way while keeping the operation
reachable.

> See previous answer. Keep all options open and explore.

### Q5. Is the arithmetic column one axis or two?

The options: two axes, on the evidence that three presets state an overflow
policy and say nothing about intermediate precision while the fourth does the
reverse; one axis, treating that asymmetry as a wording problem in the preset
table; or two-plus, with the arithmetic column being a product of however many
policy axes exist and the four presets being named sections over it.

> Same as prior answers.

## Batch two

### Q4. What does a datum stand for?

The options offered: explore all three; point with the absorbing behaviour
written as a restriction; the constructor-level reading, where the denotation
clause is a statement about the constructor wearing the grammar of a statement
about every datum; or sets admitted, which is a canon change because the
value-level total order is a precondition of the law layer.

> Option 1, but don't even restrict the panel to these three. Free reign to
> converge by theory and logic to the best one that serves all other parts of
> arvo best.

**The option set itself is not a boundary.** A fourth answer nobody has written
down is admissible and is what he is asking for, and the convergence route he
names is theory and logic rather than measurement. This is the one question of
the eight where he actively widened the space rather than declining to narrow it.

### Q6. Does `Warm` wrap, or clamp?

The options: explore both and price both, on the grounds that two committed bench
families implement the two readings and disagree in direction and `20` says the
record supports both; clamp, matching the ratified preset table; or wrap,
matching the committed bench family as implemented.

> Option 1 but see previous answer too. Write these all down too as my answers.

The instruction to write them down is what produced this file.

### Q7. Which carrier is the substrate's packing claim about?

The options: state the inequality with no fixed carrier and no threshold; the
claim is about resident footprint rather than throughput, where the five-fold cut
holds whether or not a loop gets faster; the eight-byte carrier, against which it
measures true; or hold until the contention run lands.

> Explore, wait for Fog.

**The one answer of the eight that names a blocker rather than a preference.**
`26` named concurrency as its largest gap and said it points in packing's favour,
and every measurement this panel holds is single-threaded while arvo's own rules
declare the workload concurrent. So the question is not deferred out of caution;
it is deferred because the regime the evidence was taken in may be the wrong one.

### Q8. One numeral family, or several?

The options: explore, on the grounds that it stopped being the fork it looked
like once `06` showed D3 is empty and cannot be inhabited; several families,
which is `03`'s verdict since reading A's promise needs three conditions and was
offered carrying one; or hold until the coordinate question settles.

> My instinct says one. But I'm not a mathematician, and we saw how the last
> panel ended in a mess by me making ratifications or calls too early and got us
> locked into a mess. Now we are avoiding those mistakes.

Recorded in full because the first sentence is quotable and the rest is what
makes it safe to quote. **His instinct is one family. His instruction is not to
act on it.**

There is a live tension worth naming rather than smoothing: `03`'s analysis
concludes that reading A appears to be reading C wearing A's clothes, which
points away from one family. A member finding a route to one family that survives
`03`'s three conditions would be resolving something real, and a member assuming
one family because he said it would be manufacturing agreement.

## What follows for the panel

**Nothing here settles anything.** Seven of the eight are explicitly open and the
eighth, the conjunctive reading of validate, is open to challenge on a named bar.

Three obligations replace the settlements the panel was hoping for.

**Every option stays written down, in full.** An option referenced by number is
worthless. This file is the record of the twenty-four that were live when he
answered.

**Later questions are evaluated against every live option**, and a file should
say which options its finding fits well, which it fits badly, and which it kills.
That is the work, and it is different from deciding.

**The path emerges rather than being chosen.** When one shape turns out to serve
most of the good options elsewhere, it will be visible rather than argued.

And the standing terms are unchanged from `04`: his statements are direction
unless he names them as locked, he has said he will name the locks, and the
`orgrinrt` persona's calls are the persona's rather than his.

## The horizon, stated after the eight answers

Asked nothing further, he extended `04` from one night to the panel's first
stretch:

> We will adopt the very same "explore, don't settle" angle for the first 100
> expert stretches I think

**So the eight open questions are not a backlog waiting to be cleared.** They are
the shape of the work for roughly a hundred files. A member that finds itself
wanting to close one has misread the assignment; a member that finds a fourth
option nobody had, or kills a live one with a diagnostic, is doing exactly what
was asked.

The number is approximate by his own wording and is a horizon rather than a
counter. What it settles is the expectation, and the expectation is that the
option space shrinks from the bottom, by routes being closed with evidence, while
nothing gets chosen at the top.
