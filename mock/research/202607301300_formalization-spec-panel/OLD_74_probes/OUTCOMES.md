# Probe outcomes, file 74

All compiles run from inside this directory, inside the repo tree, on the pinned toolchain,
verified immediately before the runs: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
host `aarch64-apple-darwin`. Every crate is `#![no_std]`, `--edition 2024`, and carries no
`#![feature(...)]` line.

## Probe 1: one sealed type-level number crate serves two independent domains across crate boundaries

Hypothesis: the sealed value-unique type-level number vocabulary (the `Nat`/`Pos` half of the
tower's encoding, `68:549-556`) can live in one bottom crate, with the capacity domain and the
numeral domain as two independent downstream crates that each alias it to their own semantics
(D7's pattern), the seal holding across the crate boundary, everything const-readable, no gates.

Four crates:

- `carrier.rs`: the sealed vocabulary. `Pos ::= H | O<P> | I<P>` (value-unique binary: 1, 2P,
  2P+1), `Nat ::= Z | Pz<P>`. Private `mod sealed` supertrait. `const VALUE: u128` on both
  traits, readable in const context.
- `capacity.rs` (`--extern carrier`): local `Capacity` trait with one blanket impl over every
  foreign sealed `Nat` (orphan-rule legal: local trait, foreign type). `type Cap13 =
  Pz<I<O<I<H>>>>` per D7's each-domain-aliases-the-shared-carrier pattern. Declaration-site
  `const _: () = assert!(SIZE == 13)`.
- `numeral.rs` (`--extern carrier`): a `Numeral` trait with `type Precision: Nat`, one numeral
  at p = 13, naming the identical carrier type.
- `unify.rs` (`--extern` all three): the payoff.
  `<<Binary13 as Numeral>::Precision as Capacity>::SIZE == 13` compiles and holds: the
  capacity crate's semantics reach the numeral crate's precision through the blanket impl with
  zero glue, no bridge trait, no conversion, no second encoding. A const type-equality check
  confirms `capacity::Cap13` and `Binary13::Precision` are one type.

Outcome: **WORKS.** All four crates compile clean:

```
rustc --edition 2024 --crate-type=rlib carrier.rs
rustc --edition 2024 --crate-type=rlib capacity.rs --extern carrier=libcarrier.rlib
rustc --edition 2024 --crate-type=rlib numeral.rs --extern carrier=libcarrier.rlib
rustc --edition 2024 --crate-type=rlib unify.rs -L . --extern carrier=libcarrier.rlib \
  --extern capacity=libcapacity.rlib --extern numeral=libnumeral.rlib
PROBE 1: all four crates compile clean
```

## Probe 1b: the seal holds against a foreign crate (EXPECTED FAIL)

`attack.rs` (`--extern carrier`) attempts `impl carrier::Pos for Rogue`. Refused, exit 1,
with rustc's own diagnostic naming the mechanism:

```
error[E0277]: the trait bound `Rogue: carrier::sealed::Sealed` is not satisfied
 --> attack.rs:5:23
  |
5 | impl carrier::Pos for Rogue { const VALUE: u128 = 7; }
  |                       ^^^^^ unsatisfied trait bound
  ...
  = note: `Pos` is a "sealed trait", because to implement it you also need to implement
    `carrier::sealed::Sealed`, which is not accessible; this is usually done to force you
    to use one of the provided types that already implement it
```

Outcome: **FAILS AS REQUIRED.** Value-uniqueness survives the crate split: a seal built from
module visibility seals across crate boundaries too, so putting the vocabulary in its own
bottom crate costs the seal nothing while letting every downstream domain name the types.

Next step unblocked: the one-carrier-crate shape in file 74 sections 3 and 4 is feasible as
stated; what remains open there (naming, exact contents, whether `Bias`/`Exponent`/`Radix`
ride in the same crate) is a design call, not a feasibility question.
