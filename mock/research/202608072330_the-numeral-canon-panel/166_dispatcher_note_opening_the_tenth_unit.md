# 166. Dispatcher note: opening the tenth unit on the chain and the composite

The ninth unit closed with `165`'s check and the repairs after it. Topic five has a ledger, which is
what that unit was opened for, and `AGREEMENTS.md` section 12 now points at it.

Three items from that unit are with op and one of them blocks a clause of its own statement. **None of
them blocks this unit**, which is the point of draining a queue asynchronously rather than idling on it.
What follows is the frontier as verified against source, the call on what comes next, and the reasoning
under op's `87` criterion, marked attackable exactly as that call was.

## 1. The call: the chain and the composite

**This is the coordinator's call, made under the criterion rather than by op.** `87` gives the rule:
strictly bottom-up, and among what is available at the current tier, take what settles the most
downstream at once.

The primitive is now settled, modulo what op decides. The tier above it is what primitives compose
into, and the panel's own record says twice that this is where the unattacked material sits.

**The attacker of topic five says so about its own file**, at `111:992`:

> **The largest thing I did not do.** I did not attack `109`'s section 8 chain result or `110`'s P7 and
> P8 composite results at all.

And again at `111:1426`, on the other half:

> **What I did not do.** I did not attack `112` section 8's composite results, which is now three
> members

So the load-bearing chain result and the composite results are both unattacked, by the admission of the
member whose job was attacking them, and the composite side has been untouched through three consecutive
members.

## 2. What is sitting there unattacked, stated as the claims they are

**`109` section 8 says chain accuracy cannot live in a primitive, and calls it a hole in the whole
framing.** Its own words, and it names this as the finding it would most want attacked:

> I7 says the accuracy-first concern is accurate "especially within chains and ops, not only alone". A
> primitive, in the working assumption and in mine, is a property of a **value**. A chain is not a
> value.

Its experiment holds every per-value component fixed and moves only the operator's target type: one
route quantises after every multiply, the other never quantises and narrows once at the end, with the
exact rational as the oracle so neither route is judged by the other. The consequence it draws is that
multiplication should not be an endomorphism.

**That bears directly on an op intent.** I7 is his, it is STATED, and if `109` is right then the intent
is not expressible over a per-value primitive at all, which is a finding about the design rather than
about a file.

## 3. Why this is the bottom-up choice rather than the interesting one

`AGREEMENTS.md` section 6 records the chain finding as one of the panel's strongest cross-topic
convergences: three topics land on the same structural conclusion, computed independently three separate
times, **and none of the three cites the prior two**. The format concept reaches it from the adaptation
schedule, the derived-laws topic from the lifting theorem and the width growth, and the strategy axis
from its own blind cold pair, later sharpened into chain length being a region dimension and the
crossing point between accuracy arms being decided by a weighting.

A claim that three topics reach independently and no attacker has touched is the highest-value target
available, and it is the tier immediately above the one just finished. Composition is also what the
algorithm surfaces are built out of, so everything above this is stated in terms of it.

## 4. What this unit does not inherit

**A cold open cannot be bought from a resumed expert.** Every member of the ninth unit now carries the
whole panel in context, and a cold derivation reads only the premises. So the pair opening this unit is
two fresh dispatches, and the members of the ninth unit are available for the attack and reply phases
where their context is an asset rather than a contaminant.

That is the general shape and it is worth stating once: **reuse for argument, fresh for independence.**

## 5. Handles

Recorded in `HANDLES.md` at dispatch time, under the session that owns them, per the correction that
file gained this week.

---

## 6. The curated reading list was built from memory and it omitted the answer

Added after `167` landed, because the finding is the dispatcher's rather than the member's.

**`167`'s novelty grep found `60_stam_the_chain_derived_cold.md`, a full cold derivation on this
unit's exact question, which section 4's curated reading list did not name.** A second file,
`43_rompf_what_a_composition_is.md`, is on the same question and was also omitted, and it is by the
same persona this unit dispatched as `167`.

Both are found by one command:

```
ls -1 *.md | grep -iE 'chain|compos|fold'
```

which returns four files, two of them the ones I omitted.

**The consequence, and `167` reports it against itself.** Its carried-residual mechanism for product
chains is withdrawn outright, conceded to `60`'s window, which strictly dominates it. Three of its
obligations demote from findings to corroboration of `60`'s five. One result narrows because `60`
exposed a hole in it. That is a substantial fraction of a dispatch spent rediscovering, and the
rediscovery was preventable at the cost of one command.

**What survived is the part that makes the dispatch worth having anyway**, and it is worth separating
from the loss: an intermediate-width sweep showing no threshold below `2F`, a backward-narrowing
licence with its bit count, a correlation finding that came from one of its own probes *failing*, and
a gate-free `#![no_std]` doability check on a direction `60` asserted and never compiled. Three of
those four could not have been produced by reading `60`.

**This is the same shape as the eighteen-file incident `RULES.md` records**, where every brief said
the crate tree was being nuked, every member read that as "the repository is not evidence", and
eighteen files reported a trade as unpriced while a committed harness run measuring exactly it sat in
`mock/benches/`. There the reading list had no slot for the repository. Here it had no slot for the
panel's own prior work on the assigned question.

**The rule that follows is mechanical, which is the only kind that survives a tired coordinator.**
Before writing a curated reading list, grep the panel root for the question's own vocabulary and read
what comes back. A list assembled from what the dispatcher remembers is a list of what the dispatcher
remembers, and this panel is 168 files.

**One further correction, `167`'s own and stated as such.** It wrote that the chain question was
priced and nobody had been told, then found `OPTIONS.md` Q42 already carries it with a better noise
bound than it had derived. A negative claim about evidence is a claim about a place, and it named no
place before making it.
