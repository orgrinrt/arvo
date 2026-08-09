# Op's thirty-ninth checkpoint: stop reading dead code, and `From` is a spelling problem

**Date:** 2026-08-07
**Position:** after `145_dolan_the_conversion_story.md`.
**Required reading, and the first section is a standing correction to how every dispatch has been briefed.**

## The dispatching agent has been licensing what the chain rule forbids

Op, on being shown a finding about shipped test code:

> BUT WHY AM I SEEING OLD DITCHED CODE REFERENCED THIS MUCH FOR FUCKS SAKE. Not only are all existing tests
> gone when we nuke, so these minutiae don't matter, it's so much worse when we just not hour ago
> established the fucking chain with the canon which clearly formalizes further this very thing.

He is right, the cause is identifiable, and it is the dispatching agent's.

**Every brief in this panel has carried the line "`mock/crates` is out of bounds for writing; reading it is
fine."** That sentence contradicts the rule written the same session. `the-canon-design-code-chain.md` and
the block now inline in all 65 personas say plainly that an agent consulting the shipped source during canon
work is reattaching the tier that had to be detached, and that everything it brings back is a fact about a
document already declared dead.

So the personas carried the prohibition and the brief carried a licence, and the licence won.

**The corrected standing form for every future brief:** during canon work the shipped source is not read.
Not for reference, not for context, not to check what exists. The only admissible reason to open it is to
test a factual claim the brief itself makes about it, and a finding about its contents is not a deliverable.

**And a finding about shipped tests is never a canon finding.** The tests go when the code goes. Op has
twice said existing designs and implementations are dead, and `142c` recorded that this is a mechanism
rather than rhetoric, because declaring the lower tiers dead is what licenses canon work at all.

**What this voids in `145`:** its two closing observations, that the tree already ships six `From` and four
`TryFrom` with `Cold` absent, and that the narrowing tests assert outside the value set through
`from_raw`. Both may be true. Neither is a canon finding and neither should be carried forward. The rest of
`145`, which is about the design rather than the tree, stands.

**Also withdrawn:** the entry the dispatching agent proposed adding to op's implementation-phase test
checklist. That checklist collects tests worth deleting when implementation begins, and a test in code that
will be nuked wholesale does not need collecting.

## `From` is not blocked, it is unspelled

The dispatching agent reported that nothing qualifies for `From` and put "no `From` between numerals" to op
as an option. Op did not accept the framing:

> So if, and when, we do the From and TryFrom impls, it's again, no enumeration, implicit via blankets and
> granular bounds where expressing it otherwise fails. But I am not sure what the problem is; From should be
> clear cut, no? It's a cast and we have all we need to do it on compile time, all lowered inlined.

**The problem, stated precisely, because it was not:** a blanket `impl<A, B> From<A> for B` between numerals
collides with `core`'s own `impl<T> From<T> for T`, since the compiler cannot see that the two never
overlap. `145` established that a computed order witness does not rescue coherence, and that a single
enumerable impl does. It then concluded that `From` is unavailable.

**That conclusion is the shape op has refused eight times this session.** The coherence collision is real.
"Therefore no `From`" does not follow from it; what follows is that the obvious spelling fails and a
spelling has to be found. Op's instruction stands unchanged: **no enumeration, implicit via blankets and
granular bounds where expressing it otherwise fails.**

And his reasoning is worth carrying, because it says why an answer should exist: **a cast is clear cut,
everything needed is available at compile time, and it all lowers inlined.** There is no runtime question
here and no information missing. The obstacle is coherence, which is a property of how the impl is written
rather than of what is being expressed.

So the question for the next dispatch is not whether `From` is available. It is which blanket-plus-bounds
formulation expresses it, and the answer arrives compiled or it arrives as an exhausted enumeration of what
was tried.

## What stands from `145`

Adopted by op subject to a second read, which he required for both:

**One relation, inclusion of value sets.** A numeral embeds in another exactly when both its integer and
fraction widths are less than or equal. Exhaustive over 2,025 ordered shape pairs across both signs, zero
failures. The numerals form a lattice, meets are preserved exactly, joins strictly overshoot at all 1,080
incomparable pairs.

**Every equal-precision family is an antichain**, which is the structural proof of op's withdrawal at
`130b`. Q13.3 and Q8.8 are maximally unrelated at precision sixteen, with no arrow either way, and the
withdrawn requirement proposed to identify exactly the pairs having no relation at all. It needs no mistake
to demonstrate, unlike the wrong-decode and false-law arguments that preceded it.

**Narrowing is quantisation with the operation set to the identity.** The same five situations the design
already defines, resolved by the target strategy's own row, with no new marker, no new axis and no new key
column. `Hot`'s narrowing is not monotone, which yields a refutation on the same footing as the one already
recorded for wrapping addition.

Both go to a second read before entering the canon.

## Standing

Only op's calls are final. The panel produces canon, not source. **The shipped source is not read during
canon work**, and a finding about it is not a deliverable. Experts are dispatched one at a time, each
reading the ones before it, each writing incrementally, each going down the rabbit hole rather than
reporting blockers. Do not assume arvo and notko concepts correspond.
