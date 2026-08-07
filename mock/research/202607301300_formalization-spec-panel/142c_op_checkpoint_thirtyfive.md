# Op's thirty-fifth checkpoint: the strategy cells are functions of the profile, and the panel lost that

**Date:** 2026-08-07
**Position:** after `142b_op_checkpoint_thirtyfour.md`. **Required reading with the standing base.**

This checkpoint records a loss of sight rather than a design decision. It is filed in the audit trail
because the thing lost was load-bearing, because it was lost for an entire stretch of work, and because a
later reader needs to know which of that stretch's reasoning was conducted on a false premise.

## What op said

The prior checkpoint recorded that `Warm` inherits Rust's profile-dependence, and framed that as a property
of `Warm`. Op's response:

> Well... none of the table items or strategies are the same through profiles. That's one of the main points
> of notko and its Just/Maybe/Outcome as well as the entire reason we have strategies in the first place.
> Please tell me we haven't lost sight of what arvo is and intents to be?

## The scour, and what it found

The claim was checked against the workspace rules and against the panel's own standing base rather than
answered from memory.

**The rules carry it, plainly and in more than one place.**

`no-bare-primitives.md:25`, on what determines the container:

> `UFixed<I, F, S>` and `IFixed<I, F, S>` are `#[repr(transparent)]` and lower to bare primitives at codegen,
> with the actual container chosen by `Strategy` (`Hot` / `Warm` / `Cold` / `Precise`) **and any active notko
> optimisation profile**. [...] Bare primitives in source defeat all of this: they pin the container,
> **bypass the strategy/profile pipeline**, and leak codegen concerns into consumer code. They are the
> failure mode the substrate exists to prevent.

and at `:27`:

> The stack ships nightly + custom LLVM passes + **notko profile rules**.

`arvo-always-optimal-internals.md:19`, naming the mechanism:

> **notko profile hooks**: AST-rewriting macros (`#[optimize_for]`) that **retarget the substrate per build
> profile**.

`hilavitkutin--cookbook.md:37-41`, showing it working, and this is op's Just/Maybe/Outcome point made
concrete:

> Return `Just<T>` or plain `T`. Mark the function with `#[optimize_for(hot)]` to get the macro rewrite
> (AST-level Ok→Just, Err→panic in internal release).

**The panel's standing base carries none of it.** `110_consolidation_eleven.md` matches "profile" seven
times and `124_consolidation_twelve.md`'s matches are all *deployment* profile and *workload* cost profiles
(`124:2107`, `124:3535-3563`). Not one mention of a build profile retargeting anything. The `70b` preset
tables are written as a fixed grid of cells, and every dispatch in this stretch has treated those cells as
constants to be argued over.

So op is right, and the answer to his question is no, we had lost it.

## The resolution

**Every strategy cell is a function of the active profile, not a constant.** The strategy axis and the notko
profile pipeline are one mechanism with two knobs: the strategy is what the consumer declares about intent,
the profile is what the build declares about the deployment, and the cell is what falls out of the pair. A
preset table that presents cells as fixed values is describing one profile's column and calling it the
table.

That is what the strategy axis exists for. It is also why notko sits alongside arvo rather than underneath
it: `Just` / `Maybe` / `Outcome` is the same construction on the fallibility axis, where the same declared
intent lowers differently per profile, `Ok`→`Just` and `Err`→panic under `#[optimize_for(hot)]` in internal
release.

**The error was worse than a missing sentence.** The prior checkpoint's framing, that `Warm` inherits Rust's
profile-dependence, made a general property of the axis sound like a special property of one strategy. And
the whole stretch, `131` through `142`, has been benchmarking and re-deriving cells of a table whose entire
point is that it varies.

## What this does to the stretch's work

Not everything is void, and the distinction matters.

**What survives.** The structural findings are about mechanism and hold under any cell values: that the
container derivation splits into a step that costs nothing and a step that costs a feature; that the erasure
holds when the payload is one limb of a register-width type; that a written artifact standing in for a
derivation is a defect; that the fold accumulator is the quantity `StoredWidth = doubled` approximates.

**What is conditional.** Every measured comparison in `141` and `142` was taken at one implied profile,
unnamed, and therefore prices one column rather than the table. The numbers are not wrong; their scope was
never stated.

**What was a false premise.** Arguing about which value a cell should hold, as though the answer were a
value. Several dispatches did exactly that, and op refused three of their answers in a row for reasons that
now read as him seeing the shape they were missing.

## The extras, recorded so they are not lost again

**The same complaint appears at three levels, and that is the pattern.** `142` found `StoredWidth = doubled`
is a constant standing where a derivation belongs, one level below the cells. The profile axis is a constant
standing where a function belongs, one level above them. And the standing check adopted at `139b`, that a
written artifact standing in for a derivation is a defect named at the point it appears, is the general form.
Three instances is a rule rather than a coincidence, and the canon should say so where the tables are
introduced.

**This is the third intent statement to go missing.** The aliases at `138b`, the Warm definition at `140b`,
and now the profile axis. All three are intents rather than mechanisms, all three are recorded somewhere a
later member does not look, and in all three op had to restate rather than cite. The canon has a
presentation defect specifically around intent, and `138b`'s instruction to "find a way to make it stick"
now covers a class rather than one case.

**Nothing further on the container until the profile axis is in the canon.** The honest next step is not
another expert on cell values. It is establishing what the profile axis is in this design, what it
retargets, what a preset table means once cells are functions, and how a consumer and a build declare their
halves. Otherwise the panel keeps optimising constants that were never meant to be constant.

## Standing

Only op's calls are final and they go stale when their evidence moves. The panel produces canon, not source;
`mock/research/` and `mock/benches/` are its ground and `mock/crates` is out of bounds. The mockspace bench
harness fix is in flight upstream and precedes further panel work, per `142b`.
