# Probes for panel file 09

Two families. Reproduce all of them under `nightly-2026-05-28`, the workspace pin.

## `crate-boundary/`: does a real crate split enforce law-independence-from-Lowering

Genuine separate compilation units, linked with `--extern`, not files sharing one
`crate::*`. This is the thing `08_probes/c_split_does_not_bind.rs` never tested: that
probe used `use crate::*;`, one crate, so it could not exercise the orphan rule, crate
privacy, or dependency-graph absence at all.

Build order (`-L .` so rustc finds sibling `.rlib`s):

```
rustc +nightly-2026-05-28 --edition 2024 numeral.rs
rustc +nightly-2026-05-28 --edition 2024 policy.rs
rustc +nightly-2026-05-28 --edition 2024 lowering.rs
rustc +nightly-2026-05-28 --edition 2024 algebra.rs --extern numeral=libnumeral.rlib --extern policy=libpolicy.rlib
rustc +nightly-2026-05-28 --edition 2024 numeric_honest.rs -L . --extern numeral=libnumeral.rlib --extern policy=libpolicy.rlib --extern lowering=liblowering.rlib --extern algebra=libalgebra.rlib
rustc +nightly-2026-05-28 --edition 2024 numeric_dishonest.rs -L . --extern numeral=libnumeral.rlib --extern policy=libpolicy.rlib --extern lowering=liblowering.rlib --extern algebra=libalgebra.rlib
rustc +nightly-2026-05-28 --edition 2024 downstream_hostile.rs -L . --extern numeral=libnumeral.rlib --extern policy=libpolicy.rlib --extern lowering=liblowering.rlib --extern algebra=libalgebra.rlib --extern numeric_honest=libnumeric_honest.rlib
rustc +nightly-2026-05-28 --edition 2024 algebra_logical.rs --extern numeral=libnumeral.rlib --extern policy=libpolicy.rlib
rustc +nightly-2026-05-28 --edition 2024 numeric_via_logical.rs -L . --extern numeral=libnumeral.rlib --extern policy=libpolicy.rlib --extern lowering=liblowering.rlib --extern algebra_logical=libalgebra_logical.rlib
rustc +nightly-2026-05-28 --edition 2024 numeric_via_logical_hostile.rs -L . --extern numeral=libnumeral.rlib --extern policy=libpolicy.rlib --extern lowering=liblowering.rlib --extern algebra_logical=libalgebra_logical.rlib
```

`a_leak_attempt.rs` is compiled deliberately WITHOUT `--extern lowering`, to show the
resolve failure. `algebra_macro.rs` / `numeric_macro.rs` are the macro-closure attempt
that does not work; `b_refusal_check.rs` / `c_dishonest_refusal_check.rs` are small
downstream files exercising `numeric_honest` / `numeric_dishonest` from outside.

| File | Question | Outcome |
|---|---|---|
| `numeral.rs`, `policy.rs`, `lowering.rs` | minimal stand-ins for D72's three axis crates | build clean |
| `algebra.rs` | can the (N, P)-only fact be computed in a crate that never depends on `lowering` | YES, builds clean, zero mention of Lowering possible |
| `a_leak_attempt.rs` | what happens if that crate tries to reference Lowering anyway | `error[E0432]: unresolved import lowering`, at the point of writing the line, not at review time |
| `numeric_honest.rs` | the one authorized forwarding impl for the real, physically-laid-out `Number<N,P,L>`, no L-condition | builds and folds `Warm`; refuses `Hot` (SubstituteZero) per `E0277: False: IsTrue`, matching 01/08 |
| `numeric_dishonest.rs` | the same impl, at the SAME legitimate site, ALSO conditioned on `L::Layout: IsDense` | builds clean. Reproduces 08's finding at the crate that actually owns `Number`, not a fused single-crate stand-in |
| `c_dishonest_refusal_check.rs` | does the same (N, P) fact now differ purely by L | YES: `Number<Fix13_3Signed, Warm, MinWidth>` folds, `Number<Fix13_3Signed, Warm, DoubleWidth>` refuses (`E0277: Bitpacked: IsDense`), identical N and P |
| `downstream_hostile.rs` | can a THIRD, foreign crate add a second impl of the law for `Number` | `E0117`, orphan rule, unconditional on this round's redesign, was already true |
| `algebra_macro.rs` / `numeric_macro.rs` | can a macro exported from the Lowering-blind crate close the gap | NO: the macro must restate `L: Lowering` to satisfy `Number`'s own struct bound (Rust does not imply struct bounds into impl headers), so the macro itself would need to name `Lowering`, defeating its own premise. `E0277: L: Lowering is not satisfied` when the bound is omitted |
| `algebra_logical.rs` | a type the law targets with `L` completely UNCONSTRAINED (no `Lowering` bound at all, anywhere) | builds Lowering-blind. The `where` clause has no slot to attach an `L`-condition to, because `L` carries no bound and no methods |
| `numeric_via_logical.rs` | instantiate the logical marker with a real `Lowering` type, from a crate that has Lowering in scope | both `MinWidth` (Dense) and `DoubleWidth` (Bitpacked) fold identically, by construction |
| `numeric_via_logical_hostile.rs` | can ANY crate, even one with `Lowering` in scope, add a second, L-conditioned impl for the logical marker | `E0117`, unconditionally, because the type and the trait are both foreign to every crate but the one that declared them |

## Root-level files: does the witnessed recovery map connect to the code that runs

`d_delivery_disconnected_from_phi.rs` is `08_probes/a_union.rs` verbatim (the `pub mod
spare; pub mod fusion;` tail dropped, since those sibling files are not needed to
reproduce this), with a `fn main` appended that calls `ReduceModulo`'s witnessed `phi`
directly and separately calls the union's own `add()` under `Hot` (whose declared
`OverRange = ReduceModulo`) with the same out-of-range inputs, and prints both.

```
rustc +nightly-2026-05-28 --edition 2024 d_delivery_disconnected_from_phi.rs -o d_repro
./d_repro
```

Output:

```
ReduceModulo::phi(9, min=0, max=7) = 1  (wrap-around answer)
add() under Hot/ReduceModulo returned Total(7)
phi says wrap gives 1; the runtime delivery ignores that and returns the caller's hardcoded `max` regardless of which resolution is configured.
```

`e_totality_still_holds.rs` is the same base file with a `StochasticRound` constructor
missing all three `Resolution` associated types appended, confirming `E0046` still
fires at the union's real three-member trait shape (not just the smaller shape
`07_probes/c2_totality.rs` checked it on).
