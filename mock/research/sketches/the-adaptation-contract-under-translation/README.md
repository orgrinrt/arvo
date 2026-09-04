# The adaptation contract under translation and under reflection

Outcome: WORKS. Both laws are stateable over the shipped map, both regions
decide them, and the const predicate a consumer would gate on agrees with the map
on every cell of the cross. What this unblocks is the design round that writes
the laws into `crates/arvo-format/DESIGN.md.tmpl` and the source changelist that
moves the substance of these sweeps into the crate's own tests. What it blocks is
nothing.

What this establishes, and what it refutes. Five steps, thirty-nine verdict
lines, all in `out/` and all reproducible with `./run` from this directory.

## The question

A grid has two symmetries: translation by a whole number of quanta, and
reflection through zero. The adaptation is a map onto that grid. Does it commute
with either, and where.

Both questions are stated over the map alone and need no arithmetic operation.
That is the point of asking them here rather than about a multiply-add. A fusion
verdict is the relocation law read at the positions and translations one
operation happens to reach, so the operation decides which region of the law it
sits in and decides nothing else.

```
relocation:  adapt(position + c) == adapt(adapt(position) + c)
reflection:  adapt(-position)    == adapt(-adapt(position))
```

Both are written with the right side re-adapting, because the translate or the
negation of an admitted slot need not itself be admitted. A two's complement
range has one more slot below zero than above it, so `-MIN` is not a slot.

## What the steps measure

- **`s01_rounding_equivariance`.** Fixes a residue, walks the slot across a band,
  and asks whether the offset the mode adds is the same at every slot. Five
  domains, three bands, four dithers. `W1` to `W8`.
- **`s02_completion_homomorphy`.** Walks a value two spans past each end of the
  range against every representable translation. Four excursion sets, three
  ranges. `C1` to `C9`.
- **`s03_relocation`.** The law over the whole map, against the two region
  properties measured on exactly the sets the law reached. 432 cells,
  30,756,672 triples. `R1` to `R6`.
- **`s04_reflection`.** The same shape at the other symmetry. `N1` to `N7`.
- **`s05_the_predicate`.** The const predicate as it is meant to ship, against
  the map, cell by cell. `P1` to `P7`.

Nothing reimplements a rounding mode or a range policy. Every number comes out of
`arvo_format::apply::adapt`. The rounding region is isolated by a format wide
enough that no completion fires, measured by `W1`, and the completion region by
handing the map a position already on the grid, measured by `C1`.

## The results

**The rounding region commutes with translation exactly where what the mode reads
besides the residue is not there.** Three of the six shipped modes read nothing
besides it and commute on every domain; toward-zero reads the sign at every
off-grid position; half-up reads the sign at a tie; half-even reads the parity at
a tie. The four-domain cross plus the even-slot domain pins each of the three
classifications by measurement rather than leaving one inferred.

**The completion region commutes with translation always under wrapping**, which
is the ring homomorphism, **and under a clamp exactly where no translation can
point back at the range from the side an excursion left on.**

**The relocation law holds in a cell exactly when both do.** 432 of 432 cells,
`R1`, and `R5` shows neither property alone decides it.

**The same shape holds at the reflection**, `N1`, 54 of 54 cells.

**And the two symmetries partition the six shipped modes.** Every mode has
exactly one of them; none has both and none has neither, `N3`. Nothing in
`mock/registry/*.toml` carries a row about reflection: `equivarian` matches seven
rows in `law-the-later-topics.toml`, and negation, reflection, odd symmetry and
sign fixup together match one row over the whole registry and it is about a
conversion door.

## What it refutes, which is the useful half

**The first revision of `src/predicate.rs` was unsound and `P1` caught it in six
cells.** The predicate asked its sign question about the positions a caller
declares, and the relocation law rounds the translated position too, so a
non-negative position with a negative translation reaches a negative position the
reach never named. Toward-zero and half-up under wrapping on the two ranges that
admit a negative slot. `Reach::translated_low` is the repair.

**And a structural argument in the corpus is narrower than it reads.**
`259_fallin_whether_the_fused_result_composes.md` section 3 argues that under
saturation on a non-negative domain the completion commutes, because every
translation is non-negative and an excursion above the top stays above it. That
covers the high side. `C5` is a low-side witness on the unsigned range: `y = -5`
with `c = 10` gives 5 directly and 10 staged. Seat 259's instrument never reached
it because an unsigned multiply-add produces no negative product, so the argument
and its measurement agree with each other and both are narrower than the sentence
reads. `C6` is the restriction that repairs it and `C7` is the control that the
repair is about the sign of the translations rather than about the restriction.

## What this is not

A spike. Its arities, its namings and which crate a thing would sit in are
scaffolding for the check rather than design decisions. Nothing here is a
benchmark, no timing is taken, and the release profile is a runtime convenience.

`SymmetricSlots` in `src/lib.rs` is a slot range no shipped format has, declared
here because reflection of the completion region is a question about a symmetric
range. That it compiles and reaches the map is a second thing this sketch shows:
the open inventory is open in fact and not only in the documentation.
