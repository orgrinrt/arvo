# Sketch: generic fixed-point ONE = raw(1<<F) for IFixed/UFixed Identity

Goal: construct `Identity::ONE` for the generic `IFixed<I, F, S>` / `UFixed<I, F, S>` as the fixed-point
one (raw `1 << F`) so that `x * ONE == x` once `Mul` rescales by F. The generic Identity impl currently
routes `ONE` through `Bits::ONE` (raw 1) -- wrong scale for F>0. `Bits<N>` knows total width N but NOT the
F split, so the scaled one cannot route through Bits; it must be built at the IFixed level from F.

Constraint: must be const-evaluable in `const ONE: Self = ...`, generic over (I, F, S), without an E0391
const-eval cycle and without dragging a bound chain that cascades across Identity uses.

## Hypotheses

- H1: `from_raw(Bits::ONE.to_raw() << F.raw())` with a `T: [const] Shl<u16>` bound on the container.
- H2: `BitAccess::with_bit_set(ZERO, USize(F))` + the BitAccess const-bound chain. (ALREADY TRIED in
  ifixed.rs: FAILS WITH E0391 cycle detected -- the multi-predicate const-trait routing re-enters the
  generic_const_exprs `{ifixed_bits(I,F)}` abstract-rep build. This is why arvo routes every other trait
  single-predicate through Bits.)
- H3: move the construction into an inherent `const fn one()` on IFixed (different query than the Identity
  associated-const), Identity::ONE delegates to it.
- H4: a free const fn `fixed_one::<...>()` parameterised so the shift is a plain const expr, no trait.
- H5: per-width macro impls of Identity (concrete $ctype + $f -> `from_raw((1 as $ctype) << $f)`), mirror
  from_constant. Definitely cycle-free (concrete, like from_constant) but enumerates (strategy,ctype,
  width) and the (I,F) space is large; check from_constant's actual coverage.

## Outcomes

- H2 (BitAccess + bound chain): FAILS WITH E0391 cycle. Multi-predicate const-trait routing re-enters the
  generic_const_exprs abstract-rep build.
- H1 (container-T `Shl` bound): FAILS WITH E0391 cycle. Confirms the ROOT CAUSE: any where-bound that
  references the container projection `<S as BitsContainerFor<{ifixed_bits(I,F)}>>::T` cycles predicate
  normalization. The working arvo pattern only ever bounds `Bits<{const}, S, Sign>: Trait` (the type, not
  the T projection) or `S: TraitKeyedOnN<{const}>`.
- H5 (per-width macro mirror of from_constant): NOT TAKEN. from_constant's F>0 family is fully enumerated
  per (strategy,ctype,i,f); a per-width Identity would not cover the full (I,F) space the generic impl
  serves -> coverage regression risk.
- **H9 (double raw-1 F times via `i_add` in a FREE const fn): WORKS.** The construction:
  `const fn ifixed_fixed_one<I,F,S>() -> IFixed<I,F,S> where S: const IArith<{ifixed_bits(I,F)}>,
  Bits<{ifixed_bits(I,F)},S,Signed>: const Identity { let mut acc = Bits::ONE.to_raw(); let mut d=0u16;
  while d < F.raw() { acc = <S as IArith<..>>::i_add(acc,acc); d+=1 } IFixed::from_raw(acc) }`, and
  `Identity::ONE = ifixed_fixed_one::<I,F,S>()`. Three keys that made it compile:
  1. Route the capability through `S: IArith<{ifixed_bits(I,F)}>` (strategy bound keyed on N) + the
     existing `Bits<{const},S,Signed>` bound. NEVER the container-T projection -> no cycle.
  2. Build via repeated `i_add` doubling (a const-trait METHOD), not a shift on T (avoids needing T bounds).
  3. Put it in a FREE const fn, NOT an inherent impl (inherent impls reject `[const]`/`const` trait bounds)
     and NOT directly in the associated-const initializer (which rejects the const-trait METHOD call). Use
     the ALWAYS-`const` bound form (`S: const IArith`, `Bits: const Identity`), not `[const]` -- the
     associated const / const-fn context needs the always-const guarantee, and `[const]` (conditional) is
     "not satisfied" there.
  arvo builds. F==0 -> zero doublings -> raw 1 (integer one, unchanged). Apply the mirror to UFixed
  (u_add / UArith / Unsigned Bits). The fixed-point Mul rewire pairs with this (see task #2).
