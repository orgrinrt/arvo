Source: kaski channel (coord/kaski of clause-dev), briefs/fixup/arvo-warm-priced.md, first posted 2026-11-13 08:48 UTC, read 2026-11-15.

# `Warm` on integer overflow: wrap and saturate, priced

For op, 13 November 2026, after "Bring me both priced". Read only; every number is quoted from a file committed in arvo at `origin/dev` (`2c8e5343`), nothing was run. This file recommends nothing.

The question is what an arvo integer or fixed-point value does in a release build when a result leaves its range and the type was spelled with the default strategy. Wrap keeps the low bits, as Rust's own integers do in release. Saturate stops at the end of the range. The debug abort is a separate axis now (`Num<S, C: DebugCheck = Abort>`), so under either reading a debug build aborts on the overflow by default and a consumer can name `Quiet`; the choice here is only about what the completed value is.

## What the record says, row by row

- `ruling.toml:557`, ratified: "`Warm` is the compromise that suits most default cases and behaves intuitively."
- `ruling.toml:575`, stated: "My standing call is \"It should behave like native primitives in regular old rust would\"... The intent, here, is what matters." Superseded by the next row.
- `ruling.toml:606`, ratified, supersedes `:575`: "Warm does not merely imitate, its intent is to be intuitive best choice for most every use case, and the intuitive part demands it mimics, but it does not make it absolutely required, if mimicking is consistently just worse choice." In its ratification he calls how Rust does it the baseline for intuition (his emphasis marks left out here) and adds "but it's not absolute and there are already clarifications from me on that".
- `ruling.toml:591`, stated: "Neither, it's ergonomics". The row's own note: the imitation "says nothing about where arithmetic boundaries land".
- `ruling.toml:1038`, open, a deferral: "Whether the intuitive-compromise strategy wraps or clamps is not settled. Both readings are explored and both are priced." The options then were explore and price both, clamp "matching the ratified preset table", or wrap "matching the committed bench family as implemented". The answer: "Option 1 but see previous answer too."
- `ruling.toml:1052`, ratified: "Panic is the go-to for what cannot be caught at compile time through the typestate, on debug builds, in every strategy ... What stands in its place on release is the guarantee plus an explicit declaration of the mode, saturate or wrap or whatever applies". Kept loose at his request.
- `ruling.toml:1105`, stated, his correction: "Q2 might be never on hot outside of dev/debug. It's the intent inferrable".
- `ruling.toml:1865`, stated, 13 November: "this should be either a cargo feature or const/generic/trait configurable" (the debug abort).
- `question.toml:153`, open: "Does the `Warm` preset wrap or clamp?" Third option there: "the question dissolves under Q5's two-axis answer, because `Warm` then states a value on the overflow axis and a value on the intermediate-precision axis separately". Its note: the old table's `Warm` cell, which said clamp, he "declared ... stale".
- `question.toml:110`, open, panel: whether the arithmetic column is one axis or two. Its `unblocks`: "Prior to Q6, because it decides what a wrap-or-clamp answer would be an answer about."
- `strategy.toml:49`, `warm`, at `prior_attempt`: "The compromise, and which measurements it weighs is exactly what nobody has established."
- `ruling.toml:1786`, ratified by experts: "addition's saturating top absorbs: `top + x = top` ... Under wrapping no element but the additive identity absorbs anything".
- `ruling.toml:1767`, ratified by experts: a composed saturating add then subtract "fails on 13,882,880 of 16,777,216 triples, 82.7484%" for associativity at 8 bits unsigned; "the identical composition under wrapping reports zero".
- `law.toml:31` and `:52`: additive associativity holds under wrapping; under saturation, signed, 8 bits, "4,177,792 of 16,777,216 triples diverge" (`:70`).
- CODE today, `arvo-strategy/src/lib.rs:139-144`: `Warm` carries `Adapt<HalfEven, Wrap>` under a FIXME saying the cell is "a carried default rather than a settled value".

## The numbers that exist

All from one host, an Apple M1 (recorded in each `meta.json`; `20_fog_what_the_benches_already_know.md:247`), medians of 40 samples per variant, 8192 elements unless said. They are bench families built for the container and fold questions. No committed bench runs arvo's value type, and none compares one element-wise add under wrap against the same add under saturate.

1. A fold that saturates is a different loop from a fold that wraps. `20_fog...md:512-519`, family `precise-widening-theorem-l1`, a column saturating-accumulated into a 64-bit accumulator; the "theorem arm" is the plain wrapping sum, legal where `W + ceil(log2 N) <= 64` proves the clamp dead:

   | W | wrapping sum, ns | saturating as written, ns | ratio |
   |---|---|---|---|
   | 8 | 165 | ~5330 | 32.3x |
   | 13 | 679 | ~5290 | 7.8x |
   | 16 | 701 | ~5250 | 7.5x |
   | 32 | 674 | ~5330 | 7.9x |
   | 60 | 5490 | ~5500 | 1.00x |
   | 64 | 5228 | ~5300 | 1.01x |

   The note's reading (`:525-527`): "A saturating fold is a loop-carried dependence through an operation LLVM will not reassociate, so it is serial at every width".
2. Under saturation the accumulator width decides the cost. `20_fog...md:196-203`, family `warm-clamp-arity-w13`, clamping fold at 13 bits, ns by fold arity: the minimum container goes 255, 318, 524, 1085, 7134, 9789 from arity 2 to 256; the arm with the accumulator sized by the term count (`accfit`) goes 258, 273, 536, 286, 208, 190. The author marks this subsection "contaminated and worth an independent read" (`:188-191`) because he had read a commit subject carrying the conclusion; `OPTIONS.md:305-309` says the numbers "reproduce exactly and are not themselves disputed".
   At 32 bits, read directly from `mock/benches/warm-clamp-arity-w32_n320040_findings.md:71-78` (one run): `accfit` 416 ns, 64-bit accumulator 421 ns, minimum container 2284 ns, a 5.1x spread.
3. Under wrapping a wider container is only overhead. `20_fog...md:97-104`, family `warm-container-*`, wrapping, 3 operations per element: the minimum container against the doubled one is 44.24x faster at 8 bits, 21.02x at 16, 7.03x at 32, 2.45x at 64, and 0.98x and 0.99x at 13 and 60 where the mask after each operation blocks vectorisation either way.
4. Under wrapping the interior reductions can be dropped and the chain collapses. `20_fog...md:477-483`, `warm-affine-density-w13`: projecting once at the end against after every operation is 23.9x at 1 operation per element and 33.0x at 16. The stated reason (`:491-492`): "Reduction modulo `2^W` is a ring homomorphism, so arvo has that licence and LLVM does not." Saturation has no such licence (`:179-181`).
5. A saturating fold written by hand for the target is far from the fold as written: `mock/benches/satfold-length-l1_n8000_findings.md:71-73` and `:62` (one run), 27,233 ns as the iterator fold against 605 ns for the NEON arm. The wrapping twin's whole field sits within a few percent of itself (`satfold-length-l1-wrap_n10001_findings.md:70-78`). I did not decode the two files' workload keys, so these two files are not paired against each other here.

## Reading one: `Warm` wraps

Gives: the behaviour a Rust programmer already expects from `u32` or `usize` in release, with the debug abort where Rust has its debug panic. Addition stays a ring, so sums reassociate, vectorise and collapse (items 1, 3, 4), `a + b - b` returns `a` even through an overflow, and a narrower container is never worse. It matches the cell the code carries today.
Costs: an overflow that reaches release produces a value far from the true one with no trace. No element absorbs, so a min-plus computation has no top (`ruling.toml:1786`). The old ratified preset table said clamp for `Warm`; he has called that cell stale.
For a game engine's fixed-point quantity (a health, a position, a colour channel): an overflow in release turns a large value into a small or negative one. A consumer who wants the stop names `Saturate` on that alias.
For an index or a count: an overflowed index becomes a small valid-looking one, as the host's `usize` does; a count restarts. hilavitkutin's 186 `USize` lines get the semantics they had from the host type.

## Reading two: `Warm` saturates

Gives: an overflow lands on the nearest representable value, so an error in release is bounded by the range and monotone; the top absorbs, which a min-plus fold needs (`ruling.toml:1786`). It matches the old preset table and the `warm-clamp-*` bench family's premise.
Costs: addition is no longer associative for signed values (4,177,792 of 16,777,216 triples at 8 bits) and a composed add then subtract differs on 82.7% of triples unsigned, so the compiler and arvo may not reorder a chain, and a fold is serial unless the type proves the clamp dead or the accumulator is sized by the term count (items 1, 2; 7.5x to 32x at 8 to 32 bits, nothing at 60 and 64). `a + b - b` is not `a` after a clamp. It departs from Rust's release behaviour, the stated baseline for intuition.
For a game engine's fixed-point quantity: a health or a colour channel stops at its maximum, which is usually what was meant; a position clamps at the world's edge and stays wrong until something subtracts.
For an index or a count: an overflowed index sticks at the maximum, which a bounded container refuses as out of range; a count stops at the top and stays there. A consumer who wants host behaviour names `Wrap` on `USize`.

## What the record nearly derives, and why the row holds it open

Wrap for integers is two rows from derived: `ruling.toml:606` makes Rust the baseline for intuition, and `ruling.toml:1052` puts the debug panic in every strategy and a declared mode on release, which together are Rust's own integer behaviour. The same row at `:606` is also the escape: mimicry goes "if mimicking is consistently just worse choice", and items 1 and 2 show cases where a clamp with a sized accumulator is the better-behaved fold, while items 3 and 4 show cases where wrapping is the faster one. `ruling.toml:591` says the imitation does not decide where boundaries land. And `ruling.toml:1038` is his own deferral, which handed it to measurement; the measurements came back pointing both ways, by workload, and `question.toml:110` (is overflow one axis or two) stands in front of it unanswered. So the record stops one step short on purpose.

Not established: any measurement on `Num` itself; an element-wise wrap against saturate comparison; any host other than one M1; whether fixed point and integers want the same cell, which no row asks.

## The question

When a `Warm` value overflows in a release build, what does it become?

1. It wraps, for integers and fixed point alike.
2. It saturates, for integers and fixed point alike.
3. Integers and platform-width indices wrap; fixed-point quantities saturate.
4. `Warm` names no overflow behaviour; every alias keeps naming its adaptation, as it does today.
