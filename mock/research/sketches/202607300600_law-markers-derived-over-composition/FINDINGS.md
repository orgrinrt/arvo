# Sketch findings: algebraic laws derived by blanket impls over the composition

**Date:** 2026-07-30
**Outcome:** **WORKS**, both halves. The laws derive, and more importantly the composition the
mathematics forbids is refused with a precise error and no custom diagnostic attribute.
**Unblocks:** D51 of `202607300400`, which carried this as a sketch obligation.

## Hypothesis

D51 decided law markers are computed from the axes rather than declared per type, and flagged the
open question: whether blanket impls resolve without overlap across six axes.

The hard part is not the count of axes. It is that a law depends on **more than one axis jointly**.
Saturating addition is associative for unsigned operands and not for signed ones, because two-sided
clamping is the cause and one-signed operands never walk a clamp back. So the derivation cannot
condition on the overflow policy alone.

## What was tried

Run from inside the repository so the pinned toolchain applies.

**`01_derived_laws.rs`. Compiles and runs, no feature gates.** The unit of truth is the
`(Overflow, Signedness)` **pair**, as a type-level tuple carrying the law markers:

```rust
impl AddAssoc for (Wrap, Unsigned) {}
impl AddAssoc for (Wrap, Signed) {}
impl AddAssoc for (Saturate, Unsigned) {}
// deliberately absent: impl AddAssoc for (Saturate, Signed)
```

The structures are then blanket impls over the whole composition, conditioned on the pair:

```rust
impl<Fmt, Sign, Round, Over, Grow> Semigroup<Add> for Num<Fmt, Sign, Round, Over, Grow>
where (Over, Sign): AddAssoc {}
```

Three rungs were stacked (`Semigroup`, `CommutativeSemigroup`, `AbelianGroup`) with progressively
more law bounds, and all resolve. No coherence conflict arises, because every impl is on a local type
and the conditioning is a where-clause rather than an overlapping blanket.

Asserted: wrapping folds and inverts under both signednesses; unsigned saturating folds and does not
invert, since nothing undoes a clamp.

**`02_refusal.rs`. Fails to compile, which is the result.** Asking to fold a signed saturating type
produces:

```
error[E0277]: the trait bound `(Saturate, Signed): AddAssoc` is not satisfied
   |
59 |     let _ = fold_requires_assoc::<SatI>();
   |                                   ^^^^ the trait `AddAssoc` is not implemented for `(Saturate, Signed)`
   |
help: the following other types implement trait `AddAssoc`
29 | impl AddAssoc for (Wrap, Unsigned) {}
30 | impl AddAssoc for (Wrap, Signed) {}
31 | impl AddAssoc for (Saturate, Unsigned) {}
note: required for `Num<Fixed<8>, Signed, Trunc, Saturate, FullPrecision>` to implement `Semigroup<Add>`
   |    (Over, Sign): AddAssoc,
   |                  -------- unsatisfied trait bound introduced here
note: required by a bound in `fold_requires_assoc`
```

**The diagnostic quality is itself a finding.** With no `#[diagnostic::on_unimplemented]` attribute at
all, the compiler names the exact pair that fails, lists the pairs that hold, and traces the chain
from the fold's bound back through the blanket impl to the missing law. A consumer who writes the fold
is told which axis combination they chose and which ones would work.

## What this establishes

- D51's mechanism works, and the six-axis count was never the difficulty. Conditioning on a
  type-level pair is what makes a multi-axis law expressible.
- The refusal is real rather than theoretical. This is the `202607292300` D46 finding turned into a
  compile error, which is what op described as making the formalization click by necessity.
- Symmetric saturation is included and also refused for signed operands, confirming by construction
  that `SAT_SYM` does not restore associativity. The asymmetric negative extreme was never the cause.
- The absence of an impl is the carrier of the mathematical fact, the same mechanism
  `OneRepresentable` already uses to remove `Identity<Multiplicative>` from a purely fractional type.

## What it does not establish

The probe hand-wrote the pair table. Whether the table should instead be **derived** from a more
primitive statement (an overflow policy declaring that it clamps at one bound or two, with
associativity following) is a design question this does not answer, and the derived form would be
closer to D51's spirit than an enumerated table is.

Only addition was modelled. Multiplication, and the distributivity that pairs them into a ring, are
untested and are where the interaction with rounding will first bite, since rounding has no effect on
addition's associativity but plainly does on multiplication's.

Nothing here measures trait-solver cost. Whether the pair conditioning scales across the real axis
value counts is a bench question per `bench-and-sketch-discipline.md`.
