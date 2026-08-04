# 61_probes outcomes

Verbatim build commands and outcomes, reproduced fresh from this directory.
Toolchain: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, resolved via the repo's `rust-toolchain.toml` (verified: `rustc --version` from the repo root matches this line exactly).

## probe_1: macro_rules! cannot decompose a literal into digits
```
$ rustc --edition 2024 probe_1_digit_decomposition_is_impossible.rs -o /tmp/probe1 && /tmp/probe1
A(37)   = one tok: 37
A(3.14) = one tok: 3.14
B(37)   = string stayed one tok: 37
```

## probe_2: value-to-type recursion over the magnitude is walled under every permitted door
### bare language
```
error: generic parameters may not be used in const operations
  --> probe_2_value_to_type_escape_also_walled.rs:39:58
   |
39 | impl<const V: u64> FromU64<V> for () where (): FromU64<{ V / 2 }> {
   |                                                          ^ cannot perform const operation using `V`
   |
   = help: const parameters may only be used as standalone arguments here, i.e. `V`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
  --> probe_2_value_to_type_escape_also_walled.rs:40:35
   |
40 |     type Out = S<<() as FromU64<{ V / 2 }>>::Out>;
   |                                   ^ cannot perform const operation using `V`
   |
   = help: const parameters may only be used as standalone arguments here, i.e. `V`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0119]: conflicting implementations of trait `FromU64<0>` for type `()`
  --> probe_2_value_to_type_escape_also_walled.rs:39:1
   |
37 | impl FromU64<0> for () { type Out = Z; }
   | ---------------------- first implementation here
38 | #[cfg(not(any(feature = "gce", feature = "mgca")))]
39 | impl<const V: u64> FromU64<V> for () where (): FromU64<{ V / 2 }> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `()`

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0119`.
```
### min_generic_const_args (the one further permitted opener)
```
warning: the feature `min_generic_const_args` is incomplete and may not be safe to use and/or cause compiler crashes
 --> probe_2_value_to_type_escape_also_walled.rs:9:39
  |
9 | #![cfg_attr(feature = "mgca", feature(min_generic_const_args))]
  |                                       ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #132980 <https://github.com/rust-lang/rust/issues/132980> for more information
  = note: `#[warn(incomplete_features)]` on by default

error: generic parameters may not be used in const operations
  --> probe_2_value_to_type_escape_also_walled.rs:31:66
   |
31 | impl<const V: u64> FromU64<V> for () where (): FromU64<{ const { V / 2 } }> {
   |                                                                  ^
   |
   = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items

error[E0119]: conflicting implementations of trait `FromU64<0>` for type `()`
  --> probe_2_value_to_type_escape_also_walled.rs:31:1
   |
29 | impl FromU64<0> for () { type Out = Z; }
   | ---------------------- first implementation here
30 | #[cfg(feature = "mgca")]
31 | impl<const V: u64> FromU64<V> for () where (): FromU64<{ const { V / 2 } }> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `()`

error: generic parameters may not be used in const operations
  --> probe_2_value_to_type_escape_also_walled.rs:32:43
   |
32 |     type Out = S<<() as FromU64<{ const { V / 2 } }>>::Out>;
   |                                           ^
   |
   = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items

error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0119`.
```
(the `generic_const_exprs` arm of this probe self-conflicts, two overlapping impls both matching `V=0`, a bug in the probe's own test scaffolding, not a finding about the forbidden feature; GCE is forbidden regardless per `unstable-features.md` and this file draws no conclusion from that arm.)

## probe_3: a macro-emitted type cannot cross a seal, even in the same crate
```
error[E0277]: the trait bound `MintedByMacro: Sealed` is not satisfied
  --> probe_3_macro_cannot_cross_a_seal.rs:17:22
   |
17 | impl tower::Bias for MintedByMacro {
   |                      ^^^^^^^^^^^^^ unsatisfied trait bound
   |
help: the trait `Sealed` is not implemented for `MintedByMacro`
  --> probe_3_macro_cannot_cross_a_seal.rs:16:1
   |
16 | struct MintedByMacro;
   | ^^^^^^^^^^^^^^^^^^^^
help: the trait `Sealed` is implemented for `BZero`
  --> probe_3_macro_cannot_cross_a_seal.rs:10:5
   |
10 |     impl sealed::Sealed for BZero {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `Bias`
  --> probe_3_macro_cannot_cross_a_seal.rs:8:21
   |
 8 |     pub trait Bias: sealed::Sealed { const NUM: i128; }
   |                     ^^^^^^^^^^^^^^ required by this bound in `Bias`
   = note: `Bias` is a "sealed trait", because to implement it you also need to implement `tower::sealed::Sealed`, which is not accessible; this is usually done to force you to use one of the provided types that already implement it
   = help: the following type implements the trait:
             tower::BZero

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

## probe_4: the VAL readout wall, bisected precisely
62/63/64 bits: reading Pos::VAL succeeds. 65 bits: E0080 const-eval overflow.
### 62 bits
```
warning: function `type_only` is never used
 --> probe_4_val_readout_62bits.rs:6:4
  |
6 | fn type_only() { let _x: core::marker::PhantomData<T> = core::marker::PhantomData; }
  |    ^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

```
### 63 bits
```
warning: function `type_only` is never used
 --> probe_4_val_readout_63bits.rs:6:4
  |
6 | fn type_only() { let _x: core::marker::PhantomData<T> = core::marker::PhantomData; }
  |    ^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

```
### 64 bits
```
warning: function `type_only` is never used
 --> probe_4_val_readout_64bits.rs:6:4
  |
6 | fn type_only() { let _x: core::marker::PhantomData<T> = core::marker::PhantomData; }
  |    ^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

```
### 65 bits
```
error[E0080]: attempt to compute `2_u64 * 9223372036854775808_u64`, which would overflow
  --> tower.rs:39:22
   |
39 |     const VAL: u64 = 2 * P::VAL;
   |                      ^^^^^^^^^^ evaluation of `<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> as tower::Pos>::VAL` failed here

note: erroneous constant encountered
  --> probe_4_val_readout_65bits.rs:10:13
```

## probe_5: the structural (trait-solver recursion-limit) wall, bisected precisely
Nesting depth 128 succeeds (never reads VAL, pure structural bound). Depth 129 fails.
### 125 bits
```
```
### 126 bits
```
```
### 127 bits
```
```
### 128 bits
```
```
### 129 bits
```
error[E0275]: overflow evaluating the requirement `tower::O<tower::H>: tower::Pos`
  --> probe_5_structural_129bits.rs:6:13
   |
 6 | fn main() { needs_pos::<T>(); }
   |             ^^^^^^^^^^^^^^^^
   |
```
### 130 bits
```
error[E0275]: overflow evaluating the requirement `tower::O<tower::O<tower::H>>: tower::Pos`
  --> probe_5_structural_130bits.rs:6:13
   |
 6 | fn main() { needs_pos::<T>(); }
   |             ^^^^^^^^^^^^^^^^
   |
```
### 131 bits
```
error[E0275]: overflow evaluating the requirement `tower::O<tower::O<tower::O<tower::H>>>: tower::Pos`
  --> probe_5_structural_131bits.rs:6:13
   |
 6 | fn main() { needs_pos::<T>(); }
   |             ^^^^^^^^^^^^^^^^
   |
```
### 200 bits
```
error[E0275]: overflow evaluating the requirement `tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<...>>>>>>>: tower::Pos`
  --> probe_5_structural_200bits.rs:6:13
   |
 6 | fn main() { needs_pos::<T>(); }
   |             ^^^^^^^^^^^^^^^^
   |
```

## probe_5b: the naked wall with no guard, 200-bit magnitude, reading VAL directly (not just naming the type)
```
error[E0275]: overflow evaluating the requirement `tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<tower::O<...>>>>>>>: tower::Pos`
  --> probe_5b_naked_200bit_no_guard.rs:6:13
   |
 6 |     let v = <Huge as Pos>::VAL;
   |             ^^^^^^^^^^^^^^^^^^
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`probe_5b_naked_200bit_no_guard`)
note: required for `O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<...>>>>>>>>>>>>>>>>>>>>>>` to implement `tower::Pos`
  --> tower.rs:38:14
   |
38 | impl<P: Pos> Pos for O<P> {
   |         ---  ^^^     ^^^^
   |         |
   |         unsatisfied trait bound introduced here
   = note: 125 redundant requirements hidden
   = note: required for `O<O<I<I<I<O<O<O<O<O<O<I<I<O<O<O<O<O<O<O<O<O<...>>>>>>>>>>>>>>>>>>>>>>` to implement `tower::Pos`
note: required for `I<O<O<I<I<I<O<O<O<O<O<O<I<I<O<O<O<O<O<O<O<O<...>>>>>>>>>>>>>>>>>>>>>>` to implement `PosSealed`
  --> tower.rs:33:14
   |
33 | impl<P: Pos> sealed::PosSealed for I<P> {}
   |         ---  ^^^^^^^^^^^^^^^^^     ^^^^
   |         |
   |         unsatisfied trait bound introduced here
note: required by a bound in `tower::Pos::VAL`
  --> tower.rs:24:16
   |
24 | pub trait Pos: sealed::PosSealed {
   |                ^^^^^^^^^^^^^^^^^ required by this bound in `Pos::VAL`
25 |     const VAL: u64;
   |           --- required by a bound in this associated constant
   = note: the full name for the type has been written to '/tmp/p5b.long-type-18425152592707484643.txt'
   = note: consider using `--verbose` to print the full type name to the console

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0275`.
```
