# 184. The demand side, read from outside the canon

A first pass at what arvo is asked for, taken from the consumers' own design documents rather
than from anything this panel has written. It exists to be cited: the `obligation` rows in the
registry point here, and a citation into another repository is not resolvable from this one.

**Why the demand side is read from outside.** Every check that walks the canon can only report
that the canon is consistent with itself. An obligation nobody enumerated is invisible to all of
them, so the enumeration has to come from somewhere the canon does not reach. Op set the bar this
serves, at `181`: the canon has to be exhaustive enough that a full design and then a full
implementation can be done from it. That is checkable exactly to the extent that the demands are
written down, and no further.

**This pass is not the sweep.** It reads three consumer repositories' design documents and op's
`I11`. It does not read this panel's own 2106 occurrences of the word "consumer" across 436 files,
which is a separate and larger job and is owed. **So an obligation absent from the registry today
means nobody has enumerated it yet, and never that arvo does not owe it.**

## What was read

- `hilavitkutin`, at `mock/DESIGN.md.tmpl` and `mock/crates/*/DESIGN.md.tmpl`. The pipeline
  execution engine, and the heaviest consumer.
- `vehje`, at `mock/DESIGN.md.tmpl`. The Clause language toolchain.
- `kolli`, at `mock/DESIGN.md.tmpl`. Command-line plumbing, which `workspace.md` records as
  depending on notko and arvo. **Its design names arvo nowhere**, so it contributes no row, and
  that absence is recorded rather than treated as nothing to say.
- `INTENTS.md` `I11`, op's own statement of what arvo is for.

Read on the trunk each repository publishes, which for `vehje` and `kolli` is `main`, because
**neither has a `dev` branch on its remote** and the workspace's own branch policy expects one.
Noted rather than acted on: those are not repositories this session is working in.

## The names are the dead tier's and the needs are not

Every consumer names arvo by the crate decomposition that was deleted when the canon work opened:
`arvo-bitmask`, `arvo-graph`, `arvo-sparse`, `arvo-spectral`, `arvo-comb`, `arvo-hash`,
`arvo-bits`. **Those names are not evidence about what arvo should ship**, and reasoning from them
is reattaching a tier that had to be detached.

**What is evidence is the need underneath each one**, which the same documents state in their own
terms: what the consumer is doing, at what point in its own work, and why. So every row below is
written as a need and none of them names an arvo crate. Where a consumer's document names only the
crate and never the use, no row is written and the gap is recorded at the end.

## What hilavitkutin asks for

Its scheduler's plan is a twelve-step chain, and its design names a foundations dependency per
step. `mock/DESIGN.md.tmpl`:

> `build_dag → topo_sort → upward_rank → waist → RCM → block-diag → spectral → fiber group →
> morsel size → phase config → column classify → dirty masks`

> Each step names its foundations crate dependency: `arvo-bitmask` for set ops, `arvo-graph` for
> DAG / RCM, `arvo-sparse` for CSR DependencyGraph (bench-locked canonical at all N),
> `arvo-spectral` for the Fiedler partition step, `arvo-comb` for the cost DP.

Five needs, and the chain is what says when each is wanted rather than that each is wanted in the
abstract.

Three more, from the ecosystem crates, and these are the ones that reach the numeral itself
rather than the analysis surface:

- **A platform-sized unsigned integer at an API position.** `hilavitkutin-linking`'s
  `DESIGN.md.tmpl` carries OS error codes as `arvo::USize` values, and states the property it
  needs: "non-negative on both unix errno and Windows GetLastError ranges". That is a demand about
  a range rather than about a name.
- **An exact-width bit container used as somebody else's alias.** `hilavitkutin-persistence`:
  "ContentHash comes from arvo_hash (alias for arvo_bits Bits<28>)", with "the rkyv Archived form
  is a bare u32 to pin the on-disk byte layout". So the demand is a 28-bit container that a
  consumer can name, alias, and convert to a fixed on-disk width at a boundary it controls.
- **A content hash.**

## The one that is not a type at all

`hilavitkutin-build`'s `DESIGN.md.tmpl` lists, among the flags its rustc wrapper emits:

> `--cfg arvo_fast_math`

and, on the crate's own cfg surface:

> The only cfg this crate emits: `arvo_fast_math`, when the FastMath pragma is active. arvo uses
> this to gate fast-math float semantics.

**A consumer's build system sets a cfg and arvo's float semantics change under it.** That is an
interface, it is not a function signature, and it is the demand side of exactly the hazard this
panel has already measured from the other direction: one source one flag apart gives one type name
two policies, and a compile-time-only check cannot catch it because each build emits one lowered
path and satisfies the check while the denotation differs between builds.

It is recorded here as an obligation because a consumer depends on it today. **Whether arvo should
have such a surface at all is a design question this note does not touch**, and the corpus's own
finding argues against it.

## What vehje asks for

`mock/DESIGN.md.tmpl`:

> `arvo` (sibling repo). numeric primitives + analysis (arvo-graph for pass DAG, arvo-bitmask for
> artifact read/write sets).

Two needs, both already named by hilavitkutin under different uses, which is worth recording
rather than collapsing: a graph surface wanted for a compiler's pass ordering and one wanted for a
scheduler's dependency ordering are the same surface asked for twice, and that is what makes it an
obligation rather than one consumer's convenience.

It also names `arvo::UWire` once, in passing, with no statement of what it is for. **No row**, and
it is in the gap list.

## What op asks for

`INTENTS.md` `I11`, `STATED`: arvo is a library rather than a program, and its selling point is
the algorithm crates downstream consume, and **the composition contracts for units bigger than a
numeral**, not the numeral alone.

The second half is the obligation with no consumer document behind it, because it is about what
the consumers cannot write for themselves. It is also the one the panel's own chain topic reaches
independently: a chain is not a value, so an accuracy intent stated over chains is not expressible
over a per-value primitive.

## And one from a check that was written and never landed

The debug smoke check at `mock/research/sketches/202605111110_debug-smoke/` walks a spread of
widths and strategies, writes `{:?}` into a fixed-size `core::fmt::Write` buffer with no alloc, and
asserts the output is non-empty. Its own `FINDINGS.md` names the obligation that outlives the file:
debug output from every numeral shape, at every width and under every strategy, without alloc and
without std.

## The gaps, which are findings rather than omissions

- **`kolli` names arvo nowhere in its design**, though the workspace records it as a consumer. Either
  the dependency is not real, or its design does not say what it uses arvo for. Neither is resolvable
  from here.
- **`arvo::UWire` is named by vehje with no stated use.**
- **`tarina` and the interface stack were not read.** `tarina` is recorded as depending on notko,
  arvo and hilavitkutin, and it is the consumer least like the other two, so it is the one most
  likely to demand something nobody here has enumerated.
- **This panel's own corpus was not swept**, and it is where the majority of the enumeration will
  come from.
- **Nothing here is priced.** An obligation says a consumer needs a thing; it says nothing about
  how much it costs to meet, and no row below carries a magnitude.
