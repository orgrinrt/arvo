# Op's twenty-eighth checkpoint: canonicity withdrawn, and the container is never written

**Date:** 2026-08-07
**Position:** after `130_kiselyov_the_surface_without_compromise.md`. **Required reading** with the standing
base. Supersedes `127b` on one point, stated below.

Op is present and every call here is his own. This checkpoint carries a withdrawal of one of his own
ratifications, which is the first time that has happened in this panel, and it is worth reading for the
reasoning as much as for the ruling.

## Canonicity is withdrawn

At `127b:22-31` op ratified that two numerals of equal precision must be the same type. **That requirement
is withdrawn.** His words:

> So the canonicity was for making I + F evaluate to same type as F + I, which it probably shouldn't. But on
> the other hand, even if it should, that should be implicitly "castable" via the typestate and rust
> autotyping, which would be for the most part what downstream users would rely on (way too verbose with
> the explicit typestate). But the more I read into this, I and F denoting the fraction and integer, as I
> assume, I + F and F + I are explicitly not the same types and shouldn't interexchange anywhere. It's just
> equal in width/precision but... ugh, my head isn't wrapping around this. It seems wrong to me, the premise
> for this, I can't think of a use case where the flipped fraction to i place would be meaningfully
> considered type-equal and have that mean something.

The requirement was inferred from a defect in a mechanism that has since been deleted, so by his own
staleness principle (`108b:11-20`) the ratification does not survive the evidence moving. File `130` is what
forced the issue: keying the numeral on precision alone makes Q13.3 and Q8.8 one type, which compiles
clean and asserts a wrong decode as correct, one decoded as thirty-two. Being the same type there is the
defect rather than the goal.

**What replaces it is open and is a real question rather than a gap.** If two numerals of equal precision at
different scales should ever relate, the relation is a **conversion**, not type identity, and the design
owes a statement of which conversions are implicit and which are written. Op's own instinct is that
downstream consumers will rely on implicit castability through the typestate, because writing the typestate
explicitly at every site is too verbose.

## The container is never written by a consumer

File `130` proposed a `C: Container` parameter on the surface. **Refused.** Op:

> Container naming is explicitly wrong. The entire idea of arvo is that the strategy guides container
> selection, not the user. User writes strategy and arvo optimises accordingly. And also, the same
> semantics and typestate will be used by other optimisation steps, such as the already well designed
> hilavitkutin-build.

Both arities `130` offered are refused, since both name the container, one directly and one folded inside
the strategy marker. The consumer writes `UFixed<13, 3, Warm>` and the container is **projected** from the
strategy together with the widths and the sign.

This is not an ergonomic preference. It is what the crate exists to do, and the ruling has a second half
that binds beyond arvo: the same semantics and typestate are read by downstream optimisation steps,
`hilavitkutin-build` named specifically, so a container the consumer pinned by hand is a decision taken away
from every later stage that would otherwise make it better.

## The surface holds, and one spellout is owed

Op on `130`'s core move, restoring the second coordinate so the alias computes nothing:

> It sounds good, and my gut says it holds, but this seems a bit like it came too easy. It might not
> consider some other bound needing the Nat in future, or Precision. BUT Precision has all it needs to
> derive itself from the separated components, same for Nat. So none of that is a blocker still, just an
> answer waiting for the right spellout.

Not a blocker. The owed work is an enumeration: every position across the design that could need a precision
or a width **at the type level** rather than as a const read, and for each, whether a const read suffices or
a `Nat` is genuinely required, with the derivation compiled where one is. `130` moved `Precision` from a
type to an associated-const read, and that move is only safe if nothing downstream needs it back as a type.

## Both diagnostics, belts and suspenders

> Adopt named-item laws for the diagnostic, adopt the witness set. I think it's almost free to conjure up
> both of those, no? Belts and suspenders.

Both adopted. Laws become named items so a violation prints the law's name and its coordinates in the law's
own order rather than an anonymous constant inside the operation. The witness set is a small fixed set of
instantiation witnesses that refuses every expressible wrong generic wrapper in the author's own crate.

The "almost free" is his estimate and is to be checked rather than assumed: what a law-per-item costs in
items, compile time and diagnostic quality at the real law count, and what the witness set costs to maintain
as laws are added.

## The post-monomorphisation hole, ruled earlier and still standing

> This, against my gut feeling on the priors, we can't design around, it's just monomorphisation working as
> intended. Which means, we might not be able to settle and solve it, *but* we might find a way to deliver
> proper diagnostics or compile error hints still, which we should do.

Accepted as monomorphisation working as intended. Closing it is not the task; making it legible is, which is
what the two diagnostics above are for. Two claims previously reported to op about this hole were wrong and
the position is better than described: rustc does name the author of a bad generic wrapper even across a
crate boundary, and a generic wrapper cannot compute its output coordinates at all, so the space of
expressible wrong wrappers is small enough that a fixed witness set covers it.

## The pattern, now four for four

Op's instinct has overturned a converged panel conclusion four times in this session: the width enumeration,
the surface spelling, the canonicity requirement, and the container parameter. Each time the panel had two
independent expert agreements and compiled evidence, and each time what broke it was op asking why an
assumption was there rather than whether the argument was sound.

The standing instruction he adopted for this (`127b:127-137`) is that every dispatch brief carries an
instruction to attack the premise that agreeing experts shared. It is in every brief now and it is not
sufficient on its own, which he anticipated:

> It *is* iterative by nature, and the answers aren't easy, solutions don't come by themselves, it's a
> constant ebb and flow back and forth, and we can't change that, because not only are humans fallible, so
> are machines. What we can do is try to define and contain the process and adapt as we go, but,
> meaningfully, understand that the process is, in fact, a process, and never a perfect resolution magical
> machine.

## Housekeeping settled this round

Two defects in the workspace feature rule, found by the vetting in `128`, are corrected on the workspace
side: the pinned toolchain hash now reads `1.98.0-nightly (57d06900f 2026-05-27)`, verified against the
installed toolchain, and the feasibility sketch citation now points at
`202607291400_const-args-under-min-gca`, the directory that exists. Both corrections say what was there
before. The forbidden `generic_const_exprs` gates still shipping in `mock/crates` are untouched, since the
panel is out of bounds there and the rule already records them as drift.

`generic_const_args` was vetted (`128`) and came back **WATCH**, not forbidden: no `I-unsound`, no
perma-unstable or stalled marker, and rustc enforces that it requires `min_generic_const_args`, which the
workspace already allows. The design turned out not to need it.

## Standing

Only op's calls are final, and by his own principle they go stale when their evidence moves, which this
checkpoint demonstrates. The panel produces canon, not source: `mock/research/` and `mock/benches/` are its
ground and `mock/crates` is out of bounds until the canon is complete and earmarked as arvo's first full
canon. Experts are dispatched one at a time, never in parallel, and each reads the ones before it.
