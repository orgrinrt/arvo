# Probe outcomes, file 71

All probes compiled and run in this session against `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`aarch64-apple-darwin`, `--edition 2024`, `-O` for the runnable probes, no other flags. No probe
carries a `#![feature(...)]` line.

## probe_1_far_point_total.rs

WORKS. The far-point projection (`FarPointKind` as a function of `Specials`) is total over the whole
four-member product through one blanket impl with no bound beyond `Specials` itself: `NoSpecials ->
Finite`, `NanOnly -> Finite`, `InfOnly -> Absorbing`, `IeeeSpecials -> Absorbing`. Const-callable
(`const fn far_point_kind::<S>()`), checked by compile-time assertions on all four members plus the
NanOnly/NoSpecials agreement pair. Zero refusals, zero feature gates. Output reproduced:

```
NoSpecials   -> Finite
NanOnly      -> Finite
InfOnly      -> Absorbing
IeeeSpecials -> Absorbing
total over the product, zero refusals, zero feature gates
```

## probe_2_e4m3_model.rs

WORKS, with every assertion holding. Value-exact E4M3 model (max finite 448 confirmed, top-binade
ulp 32 confirmed), three candidate resolutions run over the dispatch's three stress computations.
Output reproduced:

```
boundary: (448, 464] in-range -> 448; above 464 out-of-range
saturate: (448+448)-448 = Val(0.0) | (416*2)/4 = Val(112.0) | sum16x64 = Val(448.0)
nan-mode: (448+448)-448 = Nan | (416*2)/4 = Nan | sum16x64 = Nan
refuse  : (448+448)-448 = Refused | (416*2)/4 = Refused | sum16x64 = Refused
saturate: weakly monotone over 2401-point sweep, total on it
nan-mode: q(460) = Val(448.0), q(470) = Nan: order lost at the boundary
```

True values for the three computations: 448, 208, 1024. The extended-grid boundary assertions hold:
`q(464) = 448` (tie resolved to the even, finite side, because E4M3's max-finite stored mantissa is
`110`, even, the all-ones slot being NaN), `q(464.0001)` is an out-of-range event.

## probe_3_witness_bound_and_join.rs

WORKS. The `FarPointKind` join (silence dominates: `join(a, b) = Finite` unless both are
`Absorbing`) has its four laws (commutativity, associativity, idempotence, identity at `Absorbing`)
checked in const context over the whole two-element carrier, all eight associativity triples, not a
sample. The opt-in `AbsorbingFarPoint` bound accepts both absorbing members.

## probe_3b_negative_control_witness_bound.rs

FAILS AS EXPECTED, twice, `E0277` at both no-infinity call sites: "the trait bound `NanOnly:
AbsorbingFarPoint` is not satisfied" and the `NoSpecials` mirror, each at the exact call site, with
rustc's own help listing the two types that do implement the bound. Both no-infinity members
exercised, not one.
