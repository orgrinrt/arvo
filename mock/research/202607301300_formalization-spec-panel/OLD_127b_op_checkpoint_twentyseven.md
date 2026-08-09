# Op's twenty-seventh checkpoint: the enumeration broken, and canonicity ratified

**Date:** 2026-08-06
**Position:** after `126_dolan_only_the_widths_that_are_used.md`. **Required reading** with the standing base.

Op is present and every call below is his own. This checkpoint covers a long working session in which the
panel repaired its own archive, converged four contract questions, and then had one of those convergences
overturned by op's own objection.

## The governing pressure, stated first

**Convergence.** Op, this session:

> Let's try to force convergence on a accurate enough, valid, efficient shape and design canon. Compromises
> might have to be made, but let's try to get this thing wrapped and impls under way; There are downstream
> consumers depending on this work getting done and pushed through, and all that downstream work is
> currently halted, waiting on us to fix our shit.

Read every call below against that. A defensible answer today beats a perfect one next week, and the panel
is closing its open list rather than opening ground.

## Canonicity is required

**Ratified.** Two numerals of equal precision are the same type. Without it, a numeral reached by `13 + 3`
and one reached by `8 + 8` are distinct types with equal precision, and the compiler reports `E0308` where a
consumer expects agreement.

This had never been written down anywhere. It arrived as a side effect of a representation choice and was
never stated as a requirement, and file `126` established that it is what the width enumeration was actually
buying. It is now a requirement in its own right, and the mechanism that delivers it is open: op's
instruction is to **find a cheaper way to it**, not to accept the enumeration as its price.

## The width enumeration is not forced, and op's objection is what found it

The panel converged on a generated table, one impl per admitted width over a chosen range, with two
independent expert agreements, three compiled refutations of alternatives, and a full pricing exercise. Op
rejected the mechanism on instinct and named the precedent that broke it:

> The const param is probably the right *intent* call, just not executed the best way. Usually this is all
> about finding the workaround that abstracts further, like in the original arvo we are redesigning, the
> Capacity was one to allow the exact same constant time guarantees and checks as the prior one that relied
> on the forbidden const expr full feature, but just wrote it a different way that the solver was happy
> with.

And on the cap:

> This is almost certainly doable so that only used widths realise on const time, but resolve just the same.
> Which would theoretically allow any arbitrary widths to be defined and still work, without us choosing any
> actually legitimately arbitrary caps or ranges for valid widths. We shouldn't police it and make the
> decisions, and better yet, we shouldn't explicitly have to define each and every step on the way.

He was right on both counts. `Capacity`'s move is that **a const may be carried and read, never transformed
on the way into a type**. Applied to widths it gives three impls, no feature gates, exit 0, arbitrary widths,
no cap, and 0.04 s on sixty-four four-digit compositions against 0.06 s through the table. File `119`'s
stated reason for the table being forced is false, compiled.

**The dual spelling is refused.** Op called it convoluted and file `126` agreed independently: a second
surface existing to escape the first one's ceiling is an admission the ceiling should not be there.
`UFixed<13, 3, Warm>` is untouched, and D48 stands.

## What comes next, in this order

**Both, in that order.** An independent second read on `126` first, because it overturns a call two experts
had converged on and nothing enters the canon on one expert's word. Then the **container dispatch**, which
`126` recommends taking first among the design questions: choosing a container from a width is foundational,
it is what the forbidden `generic_const_exprs` is still being used for in shipped code
(`arvo-strategy/src/lib.rs:11`, listed as drift), and if any enumeration survives anywhere it belongs there,
over the native container ladder the hardware actually has, rather than over widths.

Everything else is held. Op:

> Do none of that until we settle this gut feeling I have that we aren't being proper and as efficient as we
> could.

So consolidation twelve is not updated, the cold reads are not re-run, and the archive sweep does not start,
until the width and container questions settle.

## The process finding, and op's answer to it

The table survived two independent agreements and a compiled refutation of three alternatives and was still
wrong, because both experts reasoned inside the same unexamined premise. That is the shared-drift failure the
workspace rules already name, and the two-agreement rule did not catch it.

**Adopted, and deliberately not as a hard gate.** Every dispatch brief carries an instruction to attack the
premise that agreeing experts shared, alongside the trust-nothing framing already there. Op:

> I wouldn't make it too strict, just something to add to the dispatching prompts and instructions for the
> experts. They already get "trust nothing and assume everything is wrong" framing, it just doesn't seem to
> work in practice. But such is the nature of this kind of work. It *is* iterative by nature, and the answers
> aren't easy, solutions don't come by themselves, it's a constant ebb and flow back and forth, and we can't
> change that, because not only are humans fallible, so are machines. What we can do is try to define and
> contain the process and adapt as we go, but, meaningfully, understand that the process is, in fact, a
> process, and never a perfect resolution magical machine.

## Earlier in the same session

**Converged with two independent agreements each, and in the standing base**: the three-way contract split
with the perimeter of what it does and does not guarantee; `S: Policy + Lowering`, a bound never stated
anywhere in six thousand lines; the supertrait prohibition, stated directionally, because a
`Strategy: Policy + Lowering` convenience trait silently undoes the enforcement mechanism while passing the
check used to verify it; and the preset key on the exponent form, which two experts reached separately after
both rejecting the two spellings originally offered.

**On his own reserved call**, the fused-versus-split question he held at `08b:47-51`, op declined to rule and
applied his own staleness principle: the reservation was made under evidence that has since moved, so the
panel converges it rather than escalating it.

**Round-qualified citations adopted.** File-qualification cannot disambiguate the register's two live
`D1-D4` sequences because both are in the same file. The round heading can, it is derivable by reading
upward, and it is op's own existing practice in three places in his frozen text, so it is recovered rather
than designed. Renumbering the inherited sequence and minting numbers for his unnumbered decisions were both
declined, on the grounds that they edit frozen text and put agent-assigned identifiers in his namespace.

**`PrecisionOf` stands** as the bridge's result type. Op on the naming rule: he does not read "precision of"
as gratuitous abbreviation, and defers whether it is semantically accurate to the panel, which is why the
panel exists.

## The width ceiling, superseded before it was ruled

Op's instinct was no ceiling, document ordinarily, on the grounds that anyone using a framework like this
knows what they are doing and the design should not police them. File `123` derived that no-ceiling is
unavailable *per written width* under an enumeration, since an enumeration has a largest row, and offered a
ceiling nobody meets.

File `126` then removed the enumeration, and with it the question. Recorded here because the reasoning is
sound and will matter again if any enumeration returns anywhere.

## Standing

Only op's calls are final, and by his own principle (`108b:11-20`) they go stale when their evidence moves.
The panel produces canon, not source: `mock/research/` and `mock/benches/` are its ground and `mock/crates`
is out of bounds until the canon is complete and earmarked as arvo's first full canon. Experts are dispatched
one at a time, never in parallel, and each reads the ones before it.
