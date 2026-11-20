# Findings: the capacity spellings, from an outside consumer with no feature line

**Date:** 2026-11-20
**Toolchain:** nightly-2026-05-28, the repository's pin
**Context:** step 9 of `mock/research/202611130950_canon-double-check/PLAN.md`, job `arvo-capacity`, after the obligation row `obligation::a_count_fixed_at_build_that_sizes_arrays_from_generic_code`
**Outcome:** WORKS for arm A, a count as a type with its array projection and a const count, sealed to `Dim<N>`; FAILS WITH `Cap is forbidden as the type of a const generic parameter` and `generic parameters may not be used in const operations` for arm B, the count as a value at a const generic position, wherever the consumer declares the parameter or sizes an array from it; arm C, a bare `usize` per dimension, WORKS one parameter at a time and FAILS WITH `generic parameters may not be used in const operations` once bundled.
**Unblocks:** the capacity crate's design round, which starts from arm A sealed; the two questions in "What is not established" go with it.

## Hypothesis

The obligation row asks that a consumer size real arrays from code generic over a capacity, bundle many capacities into one type parameter, nest one capacity's array inside another's, read the count in a constant, and never name a feature of the toolchain to do it (`obligation::the_unstable_machinery_does_not_reach_a_consumer`). Three spellings were open:

1. Arm A, a count as a type. A trait with an associated array type and a count constant, implemented by one marker `Dim<const N: usize>`. This is the shape hilavitkutin's canon already carries, as `fixed_capacity_is_one_generic_dimension_marker_implementing_the_capacity_trait_and_the_macro_rung_ladder_is_dropped` (ratified, op's refinement), and its own sketch `202605301145_capacity-as-type` built it against the old crate. Two variants: the trait sealed to `Dim<N>`, and the trait open.
2. Arm B, a count as a value of a stack type at a const generic position, `const N: Cap`. hilavitkutin's api still carries this for `Seq`, `Map`, `Reach` and `Window`.
3. Arm C, a bare `const N: usize` per dimension, which the value-stance proposal allows at const generic positions and which names no stack type at all.

Each arm is judged on one thing: does `consumer/`, a crate with no `#![feature(...)]` line, size a real array through it from generic code. The earlier feasibility probe declared the count as a bare constant and never reached the array, which `retirement::dl_feasibility_probe_compiled_the_load_bearing_path` records, so every arm here builds an array and fills it. The naive spelling, the shared carrier answering directly for the backing array, is `retirement::dl_capacity_unification_naive_spelling` and is not tried again.

## What was tried, in order

The crates: `capacity/` is arm A, with no feature line; `value-cap/` is arm B's provider, taking `adt_const_params`, which the workspace allows; `consumer/` is the outside crate. Every command was run from this directory; the output files under `out/` are the compiler's words, quoted whole.

1. Arm A sealed, `consumer/src/lib.rs` driven by `consumer/examples/run.rs`. A `PlanDims` with three capacities, two budgets (`Dim<256>`, `Dim<8>`, `Dim<16>` and `Dim<4>`, `Dim<2>`, `Dim<3>`), a plan with a one-dimensional projection and a nested `<Cores as Capacity>::Array<<AccumsPerCore as Capacity>::Array<u32>>`, filled and walked through `AsRef<[T]>` and `AsMut<[T]>` in a function generic over the bundle. The first build was refused, `out/attempt-1-nested-without-copy-bound.txt`:

   ```
   error[E0277]: the trait bound `<<D as PlanDims>::AccumsPerCore as capacity::sealed::Capacity>::Array<u32>: Copy` is not satisfied
   ```

   The outer array is filled by copying the inner one, and the trait cannot say that an array of `Copy` elements is `Copy` at every capacity. hilavitkutin states the same bound by hand at `mock/crates/hilavitkutin/src/thread/assignment.rs:53-54` on `dev` at `56a4ba4f`. With the bound stated on the generic function, the run is `out/run.txt`:

   ```
   WORKS: default sum=32647 cores=8; small sum=13 cores=2; window count=4
   ```

2. The case that had to fail: under `--cfg capacity_mutant` the provider's `Dim<N>` reports `N + 1` as its count. The consumer's `agrees::<C>()` reads `size_of::<C::Array<u8>>()` against `C::COUNT.host()` in an inline constant, and the build is refused at every capacity the generic code instantiates, six of them, `out/mutant-count-one-past.txt`:

   ```
   error[E0080]: evaluation panicked: the capacity's array and its count disagree
   ```

3. Arm A, a count that is not a constant, `consumer/examples/refused_a_runtime_count.rs`, `out/refused_a_runtime_count.txt`:

   ```
   error[E0435]: attempt to use a non-constant value in a constant
   error[E0747]: unresolved item provided when a constant was expected
   ```

4. Arm A sealed, an outside capacity whose array is four long and whose count says eight, `consumer/examples/refused_a_sealed_outside_impl.rs`, `out/refused_a_sealed_outside_impl.txt`:

   ```
   error[E0277]: the trait bound `Liar: sealed::seal::Sealed` is not satisfied
   ```

5. Arm A open, the same outside capacity against the open trait, `consumer/examples/open_a_count_disagrees.rs`, `out/open_a_count_disagrees.txt`. It builds and runs:

   ```
   BUILDS: open Liar array length=4 count=8; last element by count=None
   ```

   Generic code that takes the count as the array's length reads past the array; here `get` returns nothing where an index would panic. Nothing was constructed through a door, so no check at one could have seen it. That is the route `retirement::dl_capacity_checked_at_the_one_construction_door` names, reproduced at rank one with the trait's own method.

6. Arm B at its best case, in `consumer/src/lib.rs`: the provider declares `Seq<T, const N: Cap>` and the consumer names `value_cap::Seq<u32, { value_cap::Cap::new(4) }>` and reads its count in a constant. It builds with no feature line in the consumer; the `window count=4` in `out/run.txt` is it. It holds no array.

7. Arm B, the consumer declaring a `Cap` parameter of its own, as hilavitkutin's `Reach<C, const N: Cap>` does, `consumer/examples/refused_b_declare_cap_param.rs`, `out/refused_b_declare_cap_param.txt`:

   ```
   error: `Cap` is forbidden as the type of a const generic parameter
      = note: the only supported types are integers, `bool`, and `char`
   help: add `#![feature(min_adt_const_params)]` to the crate attributes to enable more complex and user defined types
   ```

8. Arm B, the consumer taking `adt_const_params` after all and sizing an array by the value, `consumer/examples/refused_b_array_from_cap.rs`, `out/refused_b_array_from_cap.txt`:

   ```
   error: generic parameters may not be used in const operations
      = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
   ```

   `generic_const_exprs` is forbidden in this workspace, so no feature line the consumer is allowed to write gets arm B to an array.

9. Arm C, one bare `const N: usize` per dimension, `consumer/examples/arm_c_bare_const.rs`, `out/arm_c_bare_const.txt`, one-dimensional and nested:

   ```
   WORKS: arm C sum=13
   ```

10. Arm C bundled, the dimensions as associated constants of one parameter, `consumer/examples/refused_c_bundled_const.rs`, `out/refused_c_bundled_const.txt`:

    ```
    error: generic parameters may not be used in const operations
       |
    14 |     pub order: [u32; D::UNITS],
       |                      ^ cannot perform const operation using `D`
    ```

## The result

Holds for: toolchain `nightly-2026-05-28`, edition 2024, the consumer with no feature line, capacities 2, 3, 4, 8, 16 and 256, element types `u8` and `u32`, ranks one and two.

Arm A sealed meets every clause of the obligation row that a build can check: real arrays from generic code, many capacities bundled in one parameter, one capacity's array nested in another's, the count read in a constant, and a count that is not a constant refused at build. Its count is the stack's own `USize`, read through `host()` in a constant. The array and the count come from the one `N`, so they cannot disagree, and a mutant that makes them disagree is refused at build by a check the consumer writes once.

Arm A open builds a capacity whose count is not its array's length, and generic code has no way to tell. Sealing closes it without narrowing what a consumer can ask for, since `Dim<N>` is every exact count; what sealing does refuse is a capacity of the consumer's own, a rung or a rounding of its own.

Arm B reaches a consumer only as a name the provider declared. A consumer that declares a `Cap` parameter needs a feature line, and an array sized by one needs a forbidden one. So arm B cannot be the capacity that sizes containers. What it still carries is the count at a value position read in a constant, hilavitkutin's `cap_size(K) <= cap_size(N)` and `cap_size(N) * bytes` in `footprint.rs:45`, and arm A carries both as `C::COUNT`.

Arm C sizes arrays with nothing from the stack, one parameter per dimension, and cannot be bundled, which is the reason hilavitkutin left it for `PlanDims` (`mock/crates/hilavitkutin/DESIGN.md.tmpl:1114-1121`).

## What is not established

- Whether the capacity trait is sealed. The sketch shows what each variant buys and what it refuses; the toolbox rule is against deciding a policy for the consumer, and a sealed trait refuses a consumer's own capacity type. This is the round's first call, and the sketch does not make it.
- The nested `Copy` residue. A bound the consumer states on every function that fills a nested array. Whether the trait can carry it (a second associated type, a `Copy` array projection for `Copy` elements) was not tried.
- A count type-level all the way down, a Peano or binary natural with a recursive array projection, was not tried. Arm A answers the obligation without one.
- Whether a value-position count in hilavitkutin's api (`Seq<T, const N: Cap>`, `Reach<C, const N: Cap>`, `Window::before<const K: Cap>`) moves to arm A's `C: Capacity` or to a bare `const N: usize`. That is the port's call, and the obligation row records only what the use sites need.
- Anything about cost. Array length from an associated type and from a const generic are the same to the code generator by the earlier sketch's reading; nothing here measured it, and `retirement::r173_rk_the_static_length_lever` is not cited for or against any arm.
- The element types are bare `u8` and `u32` in the sketch only, as a probe; the crate's own tests will use the stack's types.
