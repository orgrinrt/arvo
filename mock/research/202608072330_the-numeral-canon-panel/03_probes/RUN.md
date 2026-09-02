# How every number in 03 was produced

Pin, verified rather than assumed:

    $ cat ../../../../rust-toolchain.toml
      channel = "nightly-2026-05-28"
    $ rustc +nightly-2026-05-28 --version
      rustc 1.98.0-nightly (57d06900f 2026-05-27)

Every command below was run from this directory. Exit codes are recorded. Nothing here is
expected to fail, so a nonzero exit is a broken instrument rather than a finding.

    $ python3 i1_shape_space.py > i1.out
      exit 0
    $ python3 i2_corrected.py  > i2.out
      exit 0
    $ rustc +nightly-2026-05-28 --edition 2024 i3_independent.rs --out-dir out
      exit 0, no diagnostics (i3_build.out is empty)
    $ ./out/i3_independent    > i3.out
      exit 0
    $ python3 i4_categories.py > i4.out
      exit 0
    $ python3 i5_scaling.py    > i5.out
      exit 0

## What each instrument is, and why there are three of them for the load-bearing claims

**i1** is Python, exact rationals (`fractions.Fraction`), value sets as `frozenset`, and every
order question decided by real set operations. **Its Q1 is wrong and is kept.** It reported
zero disagreements between the four-condition predicate and true inclusion, which would have
refuted `02_carried` section 1.6, and the reason is setup that helps: its enumeration
contained exactly one numeral carrying fewer than two values, and that one had the coarsest
declared step in the box, so the predicate was never offered the case that breaks it. It is
kept in place rather than corrected because the failure is the useful part.

**i2** corrects that and asks the questions i1 could not. Same language and same
representation, so it is not independent of i1; it is i1 done properly.

**i3** is Rust under the pin, integers scaled by 2^12, sorted vectors and a merge test.
Different language, different number representation, different containment algorithm,
different enumeration order. It is the independent arrival, and it agrees with i2 on all four
claims it re-derives, including agreeing on the count 36 for the meets lost by refusing the
origin shape.

**i4** measures the categories op's three readings do not cover: whether the radix is the
seam, whether a small antichain of minimal upper bounds makes a tie-break cheap, and whether
the uniform-grid boundary is a better seam than the declared kind.

**i5** re-measures i4's two headline numbers at three box sizes, because a width or a count
taken inside one bound is exactly the kind of number the prior panel found to be an artifact
of its own bound. The antichain width does not move.

## The bound, stated because it is load-bearing

Every count here is over a bounded enumeration. Two guards are in place and both matter.

**Join failures are split into two kinds and only one of them is trustworthy.** A pair with
no upper bound *inside the box* may have one outside it, since an upper bound is a larger
shape and the box truncates from above. `i2` Q11 classifies every such pair by computing the
shape the join would need and asking whether it lies outside the box: all 126 do, and 0 are
structural. So "the join is total in the unsigned fixed-point family" is a statement the
instrument supports and "126 joins are missing" is not.

**Failures of the other kind are structural and survive enlargement.** A pair with upper
bounds present and none least cannot be repaired by adding larger shapes, since a new shape
above the existing minimal ones is not below them. Those counts are the ones the file relies
on.

## Modelling choices, listed so a reader can disagree with one

- Floats are normals plus zero. No subnormals, no infinities, no NaN. Adding them changes the
  value sets and does not change any question asked here, since every question is about
  whether a set is a member of a shape space.
- The asymmetric-low family is modelled as requiring at least two codes, so it has no
  singleton. That is a modelling choice and it is exactly the choice `i2` Q9's meet failure
  turns on, so it is flagged in the file rather than buried here.
- The symmetric family leaves one code unspent at an even radix, which is the shape
  `02_carried` section 1.5 records. `S<0,2>` therefore carries three values, not four.
- Shapes are quotiented by equal value set before any order question, which `150:55` requires
  and which the instruments report as "label collisions folded".

## What is not here

The compiled binary from `i3_independent.rs` is not committed. The source, the command, the (empty)
compiler diagnostics in `i3_build.out` and the run's full output in `i3.out` are, which is what a reader
needs to rebuild it and to see whether their rebuild agrees.
