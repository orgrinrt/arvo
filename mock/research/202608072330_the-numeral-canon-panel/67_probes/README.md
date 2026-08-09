# 67's probes

Five instruments, all committed with their outputs, all reproducible in one
command each. Pin `nightly-2026-05-28` for the Rust ones, resolved from the
repository's `rust-toolchain.toml`. No `#![feature(...)]` gate anywhere, no
`dyn`, no `TypeId`.

Every one of these is a **spike**. Its names, arities, trait shapes and field
orders are scaffolding chosen to reach one check. None of it is a design
proposal, and a later reader should cite these for what they proved and never
for how they were written.

## p1, the telescope

```
rustc --edition 2024 --crate-type lib p1_telescope.rs     # compiles clean
rustc --edition 2024 --crate-type lib p1_neg_a.rs         # refused, E0271
rustc --edition 2024 --crate-type lib p1_neg_b.rs         # refused, E0277
```

Establishes that the concept's components form a dependent sequence rather than
a product, that the dependency is enforceable by ordinary associated-type
equality, that the completed term names only its last component and recovers the
earlier ones by projection, that the whole apparatus erases to the container's
size, and that a law contract is decided by the prefix (identity, adaptation),
reading neither the encoding nor the container, while being undecided by the
identity alone.

`p1_neg_a.stderr` and `p1_neg_b.stderr` are the committed refusal transcripts.

## p2, the three crossings

```
python3 p2_three_crossings.py
```

Exhaustive at 4 bits. Measures, for each of three crossings, the value-level
agreement and the operation-level agreement separately. Contains one prediction
of its own that its output refutes, kept with the refutation and the corrected
closed form.

## p3, shared parameters over a run

```
python3 p3_shared_parameter.py
```

Applies the panel's own identity condition ("the representable set is a constant
of the type") mechanically to block floating point, to a packed run's stride, and
to a self-contained float as the control. Separates two shapes of shared
parameter by which layer they sit at.

## p4 and p4b, the two law families as two consumer classes

```
python3 p4_two_law_families_two_consumers.py
python3 p4b_recount.py
```

Exhaustive at 4 bits over two windows and three reductions. Extends the panel's
frame to `min`, derives the tropical distributivity condition and checks the
biconditional against monotonicity, and maps the two law families onto two
consumer classes.

`p4b` is an independently written recount, kept because `p4` returned the
identical figure in four cells and a repeated number is the shape of a counting
bug. It is not one; the collision reproduces and is recorded unexplained.

## What none of these establish

Any magnitude. Nothing here is a bench, nothing was timed, and every
cost-flavoured remark in the file that cites them is unpriced and says so.
Transfer past the 4-bit model width is by uniformity of construction, argued and
not measured.
