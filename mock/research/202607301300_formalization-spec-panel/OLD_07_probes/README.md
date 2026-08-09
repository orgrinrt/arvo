# Probes for panel file 07

Seven probes written for `07_spj_is_the_type_story_sound.md`, all compiled (or deliberately failed)
under `nightly-2026-05-28`, the workspace pin. Each is a self-contained crate body needing no
dependencies: `rustc +nightly-2026-05-28 --edition 2024 <file>` reproduces the result. The A-series
uses `#![feature(const_trait_impl)]`, allowed at WATCH tier per `unstable-features.md`; no probe uses
`generic_const_exprs` or any forbidden feature.

The A-series is one design walked through its failure modes: the recovery map is a `[const]` trait
method (the same definition runtime arithmetic calls), a generic `const fn stable<R>` checks 01's
translation-stability identity through that bound, the classification trait declares the truth
markers the law derivation projects, and a witness ties the declared marker to the computed check.

| File | Question | Outcome |
|---|---|---|
| `a_witness_typestate.rs` | does the whole shape resolve with honest classifications | WORKS. Compiles, runs, derivation refuses `SubstituteZero` per 01 finding 1 when uncommented |
| `a2_lie_fails_at_declaration.rs` | a lying marker with an eager per-constructor forcing const | FAILS as intended: `E0080` naming `<SubstituteZero as Resolution>::WITNESS` at the declaring crate's own const |
| `a3_lie_unforced_compiles.rs` | the same lie with no forcing site anywhere | COMPILES AND RUNS. Associated consts are lazy; the witness alone enforces nothing |
| `a4_lie_fails_at_use.rs` | the same lie, forced by an inline `const` block in the one generic entry point | FAILS as intended: `E0080` at `resolve::<SubstituteZero>::{constant#0}`, at monomorphisation of the first use |
| `a6_override_disarms_witness.rs` | can an implementor disarm the default witness body | YES: `const WITNESS: () = ();` in the lying impl compiles clean past the eager forcing const. The overridable member is not the load-bearing site |
| `a7_door_checks_directly.rs` | the door checks the identity directly instead of through the member | FAILS as intended even with the override present: the check cannot be disarmed from an impl |
| `b_bounds_collapse.rs` | do the carrier-join bounds collapse into one blanket impl with the operations as methods | WORKS. `B: sat=100 precise=Refused mixed_hi=Refused mixed_lo=Ok(0)`; both arithmetic bodies carry exactly one bound, `Q: QuantExt` |

Mechanism notes recorded so they are not rediscovered. A generic `const fn` CAN call the recovery
map through a `[const]` trait bound, which dissolves the macro-instantiation constraint 05 recorded
for `fn` pointers. A supertrait spelling of `pub const trait Resolution: [const] Resolve` does not
give the default const body the `const Resolve` obligation it needs; the always-const form
`pub trait Resolution: const Resolve + Sized` does. Associated consts, including defaulted ones in
used impls, are evaluated lazily; every enforcement route below runs through something that is
evaluated: a free `const _` in the declaring crate, or an inline `const { .. }` block in a generic
function, which is evaluated at monomorphisation.
