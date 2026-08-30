# Outcomes, file 62 probes

Toolchain for every command: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, named explicitly as
`+nightly-2026-05-28` because most of these commands run outside the repo tree where
`rust-toolchain.toml` does not apply. That trap fired once for real in this dispatch: the first
`cargo check` on the copied-out crate resolved to stable and failed with `E0554` (three errors in
notko's cfg-gated feature attributes) before the pin was named. Host `aarch64-apple-darwin`, Apple
M1. Compile-time wall clocks via `/usr/bin/time -p`; no runtime measurement anywhere in this file.

## 1. The union crate rebuilds from the committed trail (`rebuild_union.sh`)

Recipe followed verbatim from `08_probes/README.md:8-11` (`a_union.rs` as `src/lib.rs`,
`b_spare_pattern_decides_delivery.rs` as `src/spare.rs`, `c_split_does_not_bind.rs` as
`src/fusion.rs`, `e`/`g` under `src/bin/`; `f_error_surface.rs` excluded because its documented
outcome is a compile failure). Result:

```
Finished `release` profile [optimized] target(s) in 0.85s
```

`g_classification_table` reproduces file 01's table exactly:

```
ReduceModulo     stable1=true  stable2=true  refuses=false
TowardNegative   stable1=true  stable2=false refuses=false
TowardPositive   stable1=true  stable2=false refuses=false
SubstituteZero   stable1=false stable2=false refuses=false
Refuse           stable1=true  stable2=false refuses=true
```

`e_codegen.rs` compiled against the rebuilt rlib at `-C opt-level=3`, whole-function counts from
the emitted asm (`count_shapes.py`; file 08's table counts loop body only, so the instruction
column is a coarser superset while the branch column is directly comparable):

```
u_bot            {'instr': 82, 'branch': 6}
u_bot_inpayload  {'instr': 15, 'branch': 2}
u_prec           {'instr': 15, 'branch': 3}
u_raw            {'instr': 11, 'branch': 2}
u_sat            {'instr': 14, 'branch': 2}
```

Against `08:225-233`: the 87-instruction, 6-branch companion-flag row is an 82-instruction,
6-branch whole function here; the 10-instruction spare-pattern row is a 15-instruction whole
function; branch structure matches row for row.

## 2. The width-ceiling sweep, fresh derivation (`drv_3.rs` through `drv_9.rs`)

Five constructors, one one-sided stability check each (`stable::<R>(0, 2^N - 1, false)`), forced
by `const` items, compiled as a driver crate against the rebuilt union rlib:

```
rustc +nightly-2026-05-28 --edition 2024 --extern union=<rlib> -L <deps> --emit=metadata drv_N.rs
```

| width | wall (this rebuild) | wall (08:445-450) |
|---|---|---|
| 3 | 0.10s | 0.53s |
| 4 | 0.07s | (not in 08's table) |
| 5 | 0.20s | 0.84s |
| 6 | 0.72s | 2.26s |
| 7 | 2.90s | 8.65s |
| 8 | 11.64s | 28.45s |
| 9 | refused | refused |

Width 9 verbatim, after 38.87s of wall clock:

```
error: constant evaluation is taking a long time
  = note: `#[deny(long_running_const_eval)]` on by default
error: constant evaluation is taking a long time
error: constant evaluation is taking a long time
error: aborting due to 3 previous errors
```

Ratios between adjacent widths from 5 up: 3.6x, 4.0x, 4.0x. The refusal at nine is a
deny-by-default lint on const-eval step count, not a wall-clock timeout.

## 3. The shipped GCE gates are structural (`strip_gate_experiment.sh`)

`arvo-strategy` copied out of the tree, `#![feature(generic_const_exprs)]` deleted from
`src/lib.rs` (the line at `arvo-strategy/src/lib.rs:11`), nothing else touched, deps pointed back
at the unmodified tree:

```
16 error: generic parameters may not be used in const operations
16     = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
 1 error: could not compile `arvo-strategy` (lib) due to 16 previous errors
```

Same crate with `#![feature(min_generic_const_args)]` inserted where the deleted gate was:

```
16 error: complex const arguments must be placed inside of a `const` block
 1 error: could not compile `arvo-strategy` (lib) due to 16 previous errors
```

The `arvo` facade, identical experiment (gate at `arvo/src/lib.rs:25` deleted):

```
478 error: generic parameters may not be used in const operations
  1 error: could not compile `arvo` (lib) due to 478 previous errors
```

`probe_min_gca_const_block.rs`, the const-block escape the min feature's own error message
suggests, on a reduced `BitsContainerFor`-shaped bound:

```
error: generic parameters may not be used in const operations
   = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

## 4. The `Rad<P>` seal probes re-run, plus a fifth route (`probe_atleasttwo_for_h.rs`)

File 54's probe 1 series, re-run verbatim from `54_probes/` with the support modules present
(`vu_bias_sealed_adj.rs`, `vu_nat_sealed_adj.rs`, `numeral.rs`):

```
probe_1  (positive)                          exit 0
probe_1b error[E0277]: the trait bound `H: AtLeastTwo` is not satisfied
probe_1c error[E0277]: ...ForgedDomain: numeral::dom_sealed::SignDomainSealed... (x3 carriers)
probe_1d error[E0210] (blanket route) and error[E0603]: module `specials_sealed` is private
```

The fifth route, this file's own: implement `AtLeastTwo` (and its private supertrait) for `H`
from outside the tower crate, which would make `Rad<H>` (radix one) legal:

```
error[E0603]: module `radix_sealed` is private
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
```

## 5. File 55's probe 2b re-run

`probe_2b_the_arity_of_an_unbounded_loop.rs` compiled against freshly rebuilt `libtower.rlib` and
`libgrade_lib.rlib` from `55_probes/`: zero errors. The two blankets
(`impl<Hd: Pos + Cmp<A>, A: Pos> InteriorSafety<A> for Hd` at line 69 and
`impl<Hd: Pos> InteriorSafety<Unbounded> for Hd` at line 84) coexist with no specialisation.

## 6. Primary-source extracts

See `primary_sources.md` in this directory for the verbatim OCP OFP8 and IEEE 754-2019 clause 5.2
quotes and their provenance.
