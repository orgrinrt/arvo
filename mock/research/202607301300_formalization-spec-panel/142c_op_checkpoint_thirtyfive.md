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

---

# Correction and clarification, same day, from op

The section above describes the profile as "what the build declares about the deployment". **That is wrong**
and it is corrected here rather than edited away, because the wrong reading is the natural one and a later
reader will arrive at it again.

## Op's words

> The notko profiles are not cargo profiles. The notko profiles roughly translate to arvo strategies. Where
> the notko/hv attribute is above something using arvo, the macro rewrites it to change strategy as per
> function "profile" (optimize_for value, pretty much analogous to our strategy) IF no specific config is
> being used that declares intends use of specific thing. If arvo is used implicitly, typestate inferred,
> then notko/hv will just rewrite the strategies as per the attribute or whatever else it'll figure out
> later.
>
> This is the full intent. Not to declare laborious typestate downstream on consumer end, rather write
> implicitly, almost invisibly, as if all worked through bare rust primitives, and then do targeted function
> or item scope optimizations that rewrite the scoped content.

## What the profile actually is

**A lexical scope knob, not a build knob.** `#[optimize_for(...)]` sits above a function or an item, and the
macro rewrites the arvo use *inside that scope*. The unit is the item, not the compilation.

**Its values are roughly the strategies.** `optimize_for(hot)` is analogous to `Hot`. So the profile is not
an orthogonal axis multiplying the strategy table; it is a second way of *supplying* the strategy, at a
different site and a different granularity.

**Explicit declaration wins.** Where a specific config declares intended use of a specific thing, the
attribute does not override it. The rewrite applies where the use is implicit.

**And the rewrite is not limited to what the attribute names.** Op's "or whatever else it'll figure out
later" leaves the macro room to derive more than the one value it was given, which is a door left open
rather than a mechanism specified.

## The intent, which is the part that matters most and which the panel has been designing against

> Not to declare laborious typestate downstream on consumer end, rather write implicitly, almost invisibly,
> as if all worked through bare rust primitives, and then do targeted function or item scope optimizations
> that rewrite the scoped content.

So the primary consumer surface is **inference**. A consumer writes code that reads like plain Rust with
plain primitives, the typestate is inferred rather than spelled, and optimisation is applied by annotating a
function or an item rather than by threading parameters through every declaration.

This is consistent with what op said when he first reopened `Warm` at `137b`, and nobody connected it:

> Just using arvo without using the typestate at all, downstream, like just using the algos that then
> implicitly work on bare primitives for downstream callers via the typestate and rust type inference.

That sentence was read at the time as naming an edge case worth protecting. It was describing **the normal
path**.

## What this does to the surface question the panel has spent four files on

Files `129`, `130`, `133`, `134`, `139` and their checkpoints all worked the question of how a consumer
writes a width in type position: `UFixed<13, 3, Warm>` against a macro invocation against a type-keyed
magnitude, with the const-to-type bridge as the wall. The whole argument assumed the explicit spelling is
the surface.

Under the intent above **it is not the primary surface**, and the frequency ordering the panel assumed may
be inverted: the common path writes no typestate at all, and the explicit spelling is what someone reaches
for when they want to pin something specific.

That does not dissolve the bridge question, because the explicit spelling still has to exist and still has
to work. It changes what the question is *for*, and therefore what an answer may cost. A mechanism that is
slightly awkward at a site consumers rarely write is a different trade from one that is awkward everywhere,
and every dispatch so far has priced it as the latter.

**This is flagged rather than asserted.** Whether inference can actually carry the common path, what it
infers from, and where it hands off to the explicit spelling, are all unestablished in the standing base and
need working out before anything is concluded from this paragraph.

## What is now owed, and it is bigger than the container question

The panel does not have the profile mechanism in its standing base at all, and it turns out to be the
consumer-facing half of the design. What is owed:

- **What the attribute rewrites**, concretely, and at what granularity.
- **What the typestate is inferred from**, when a consumer writes nothing.
- **The precedence rule**: what counts as "a specific config declaring intended use", and how an explicit
  declaration and an enclosing attribute compose.
- **The relationship between a profile value and a strategy**, given op says "roughly translate" and
  "pretty much analogous" rather than "are".
- **What this means for the preset tables**, since a cell reached by inference and a cell reached by
  declaration must agree or the design has two answers.
- **Where the boundary sits between arvo and notko/hv** for this mechanism, since the rewrite lives in one
  and the types live in the other.

Nothing further on the container or the surface until that is established. The prior section's closing
sentence stands with its reason corrected: the panel keeps optimising a surface that is not the primary one,
and cells that are not constants.
