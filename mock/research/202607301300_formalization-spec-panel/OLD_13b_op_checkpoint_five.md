# Op's fifth checkpoint: where parallel associativity lives

**Date:** 2026-07-31
**Position in the panel:** written after `13_mcsherry_where_the_laws_belong.md`. **Required reading**
with the numbered files and the earlier checkpoints `04b`, `06b`, `08b`, `12b`.

## The call

File 12 proposed that associativity is the contract of parallel reduction and therefore belongs one
layer up, in hilavitkutin, where the reordering happens. Op's call, verbatim:

> if the parallel associativity is part of the numeric substrate here, it belongs in arvo, but perhaps
> a separate place. Not hilavitkutin. I would guess other downstream users would seek for that in arvo
> too and make use of it, without having to pull in hilavitkutin

So the relocation out of arvo is refused. What remains open, and is the useful part of file 12's
instinct, is that the laws may not belong where the spec currently puts them *inside* arvo. A separate
place within arvo is available and is worth designing rather than defaulting.

The reasoning that decides it is not only op's preference. A downstream consumer wanting the algebraic
vocabulary would have to take an entire pipeline execution engine to get it, which inverts the
dependency direction of the whole stack and contradicts what arvo is for.

**And file 13 independently removed the argument for moving it.** Its COST measurement found the
regrouping already pays at a single thread: four accumulators beat one by roughly two to one, 55ns
against 110ns, from a bench already committed in arvo and never cited during this review. So the
reordering that needs the law is not a scheduler behaviour at all. It happens inside any unrolled
accumulator, within the licence `arvo-always-optimal-internals.md` already grants arvo over its own
implementation bodies. The law belongs where the reordering happens, and the reordering happens here.

## What file 13 did to the question underneath

Recorded because it changes what a later member should build on, and because two of the three things
it overturns were load-bearing for earlier files.

**File 12's premise about the algorithm crates is false.** `arvo-graph`'s `upward_rank` does not fold
weights. `rank.rs:84` is `rank = w + best` with `best` a running maximum over successors, and
`path.rs:81` is the same shape. These are max-plus recurrences: addition applied once per node, with
the grouping pinned by the graph. Associativity is not what makes their answers correct, so the
collision file 12 described does not arise in the form it described.

**The presets sort the opposite way from the design's assumption.** Verified exhaustively over 64
directed acyclic graphs and 625 weight vectors: wrapping is associative but fails distributivity over
maximum, while saturating is non-associative and satisfies it. Under `Hot`, `longest_path` returns a
value that is not the longest path under any grouping. So a gate on associativity would admit the one
preset that breaks these crates and refuse the two that work.

**Order is not the axis; grouping is.** Contiguous chunking preserves element order exactly and still
changes the grouping, so the merge shape the engine already ships needs the law, and a "documented
order" does not substitute for it.

Also: `arvo-spectral/src/power.rs:71` is arvo's one genuine fold over addition, over a float type, so
an associativity gate would refuse it at every strategy.

## Two findings that are not about this design at all

Both surfaced by file 13 and both verified independently before this file was written. Neither waits
on the round.

**hilavitkutin's mock workspace does not load.** `crates/hilavitkutin-api/Cargo.toml:17` declares
`mockspace.workspace = true` and `mock/Cargo.toml` has no such key in its workspace dependencies.
`cargo metadata` exits 101 with a manifest parse failure, so `cargo test --workspace` cannot reach
compilation. **189 engine test functions have been reporting nothing.** Every statement anyone has
made about engine behaviour during this review is a statement about text that has not been run.

**A live defect in `ConvergenceBuffer::combine`** at `resource/accumulator.rs:50-58`: an unbounded
`fn(T, T) -> T`, an `init` unrelated to the constructor's `zero`, and a fold across all slots
including ones no core wrote. With four slots, two cores and a signed payload, `combine(0, max)`
returns 0 where the true maximum is -1. The four shipped tests cannot see it, because every combiner
they exercise is associative, commutative, and has zero as its identity at `u32`, and no test asserts
the property `combine` exists to provide.

These are hilavitkutin's, not arvo's, and they are recorded here because this is where they were
found.

## Standing, unchanged

Only op's calls are final, and even those go stale. Seven of this review's findings have now been
overturned by a later member compiling or measuring something rather than reading, including two in
the file that raised the question this checkpoint answers.
