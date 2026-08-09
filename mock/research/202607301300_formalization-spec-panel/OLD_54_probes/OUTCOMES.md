# 54_probes: outcomes, verbatim commands and reproduced figures

Toolchain for every build below: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`. Verified from this directory before any build was run, per file 52's finding
that a bare `rustc` outside the repo tree resolves to stable:

```
$ rustc --version
rustc 1.98.0-nightly (57d06900f 2026-05-27)
$ rustc +nightly-2026-05-28 --version
rustc 1.98.0-nightly (57d06900f 2026-05-27)
```

Gates for this dispatch:

```
$ cd mock && cargo test --workspace   # summed per binary
binaries=122 passed=654 failed=0 ignored=9

$ grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"   # exit 1, empty
$ grep -rln "FullRange\|UTerm\|AddWidth" mock/crates/ --include="*.rs"  # exit 1, empty
```

## Shared modules

| file | provenance |
|---|---|
| `vu_nat_sealed_adj.rs` | unmodified copy of `50_probes/vu_nat_sealed_adj.rs`, itself file 46's copy of file 42's sealed tower |
| `vu_bias_sealed_adj.rs` | unmodified copy of `50_probes/vu_bias_sealed_adj.rs` |
| `numeral.rs` | new. The exponent machinery is file 50's `probe_3`, unmodified in substance; `Radix`, `Specials`, `Underflow`, `SignDomain`, `Fl`, `Fx`, `MulNum` and `AdjProduct` are new |
| `crossing.rs` | new. The three-statement crossing contract as a `const fn` model |
| `report.rs` | new. Runtime reporter over `crossing.rs`, counts only, no timing |

`grep -n "Reduce" numeral.rs probe_*.rs` finds three hits, all in comments. The projection-chain
constraint (`49:306-324`) holds by construction: nothing in `ESum`, `SignedDiff`, `NegE`, `NAdd`,
`AtLeastTwo`, `MulNum` or `AdjProduct` names `Reduce`, and every impl pattern-matches on constructor
heads.

## Probe 1: carriers born sealed

```
$ rustc --edition 2021 --crate-type lib probe_1_carriers_born_sealed.rs
(clean)
$ rustc --edition 2021 --crate-type lib --crate-name vu54 probe_1_carriers_born_sealed.rs -o libvu54.rlib
(clean)
```

WORKS. Every const assertion passes: `Two::R == 2`, `Ten::R == 10`, `Sixteen::R == 16`, a
downstream-named radix three and one hundred, the four `Specials` corners, and the real formats
`Binary32`, `Fp8E5M2`, `Fp8E4M3` (NaN and no infinity, `EMAX == 8`), `Decimal64` (radix 10, p 16,
e in [-383, 384]). `mulnum` composes at radix two and at radix ten and is forced through a signature.

One repair worth recording: the first draft's `P383` nest was written with the bit order reversed and
the const assertion caught it (`assertion failed: <P383 as Pos>::VAL == 383`). The generator in
`price/gen.py` had the same bug and it was caught the same way. A `Pos` literal written by hand is
exactly the defect file 47 named for numeral declaration, and the const assertion is the only thing
between a wrong nest and a silently different numeral.

## Probes 1b, 1c, 1d: the negative controls, one crate per route

```
$ rustc --edition 2021 --crate-type lib --extern vu54=libvu54.rlib probe_1b_radix_one_refused.rs
error[E0277]: the trait bound `H: AtLeastTwo` is not satisfied
  --> probe_1b_radix_one_refused.rs:19:6
   |
19 |     <Rad<H> as Radix>::R
   |      ^^^^^^ the trait `AtLeastTwo` is not implemented for `H`
   = note: required for `Rad<H>` to implement `numeral::radix_sealed::RadixSealed`
```

FAILS AS INTENDED. Radix one is unspellable.

```
$ rustc --edition 2021 --crate-type lib --extern vu54=libvu54.rlib probe_1c_specials_direct_impl_refused.rs
error[E0277]: the trait bound `ForgedSpecials: numeral::specials_sealed::SpecialsSealed` is not satisfied
  = note: `Specials` is a "sealed trait", because to implement it you also need to implement
    `vu54::numeral::specials_sealed::SpecialsSealed`, which is not accessible; this is usually done to
    force you to use one of the provided types that already implement it
  = help: the following types implement the trait:
            vu54::numeral::NoSpecials
            vu54::numeral::InfOnly
            vu54::numeral::NanOnly
            vu54::numeral::IeeeSpecials
(and the same E0277 for ForgedUnderflow, ForgedDomain, ForgedRadix)
```

FAILS AS INTENDED, four times, one per carrier.

```
$ rustc --edition 2021 --crate-type lib --extern vu54=libvu54.rlib probe_1d_seal_supertrait_and_blanket_refused.rs
error[E0603]: module `specials_sealed` is private
error[E0210]: type parameter `T` must be used as the type parameter for some local type
```

FAILS AS INTENDED, routes two and four. Route three (re-impl on an existing inhabitant) is the orphan
rule and is not repeated; file 46 established it for the tower's carriers and it is a property of
coherence, not of the trait.

## Probe 2: the crossing contract, radix two

```
$ rustc --edition 2021 --crate-type lib probe_2_crossing_binary.rs
(clean)
```

WORKS. Every statement is a `const _: () = assert!(..)` over an exhaustively enumerated datum space.
Figures reproduced by `report.rs`:

| configuration | data | live | values | s1 | s2 | s3 | predicted |
|---|---|---|---|---|---|---|---|
| p=3 e[-2,3] `NoSpecials` | 112 | 56 | 55 | true | true | false | false |
| p=3 e[-2,3] `InfOnly` | 114 | 58 | 57 | true | true | false | false |
| p=3 e[-2,3] `NanOnly`, 2 nan data | 114 | 58 | 56 | true | true | false | false |
| p=3 e[-2,3] `IeeeSpecials`, 2 nan data | 116 | 60 | 58 | true | true | false | false |
| p=3 e[-2,3] `IeeeSpecials`, 4 nan data | 118 | 62 | 58 | true | true | false | false |
| p=3 e[-2,3] unsigned, no specials | 56 | 28 | 28 | true | true | **true** | true |
| E4M3FNUZ shape (p=4, e[-6,8], nan on the -0 datum) | 512 | 256 | 256 | true | true | **true** | true |
| p=3 e[-2,3] abrupt underflow | 112 | 50 | 49 | true | true | false | false |

`s3_predicted` agrees with the exhaustive result in every row.

## Probe 3: radix ten

```
$ rustc --edition 2021 --crate-type lib probe_3_crossing_decimal.rs
(clean)
```

WORKS.

| configuration | data | live | values | s1 | s2 | s3 |
|---|---|---|---|---|---|---|
| decimal p=2 q[-1,1], min-sig section | 600 | 600 | 559 | true | true | false |
| decimal p=2 q[-1,1], max-sig section | 600 | 600 | 559 | true | true | false |
| decimal p=3 q[-1,1] | 6000 | 6000 | 5599 | true | true | false |
| the same numeral IF normalised | 800 | 560 | 559 | true | true | false |
| decimal, single exponent row | 200 | 200 | 199 | true | true | false |
| decimal, single row, unsigned | 100 | 100 | 100 | true | true | **true** |
| BID-shaped, 7-bit significand field | 768 | 768 | 559 | true | true | false |
| radix 16 p=2 q[-1,1] unnormalised | 1536 | 1536 | 1471 | true | true | false |

`sections_agree(D_MIN, D_MAX)` is **false** for radix ten and **true** for radix two, asserted in both
files. The named witness: `encode(min-sig, 1) != encode(max-sig, 1)` and both decode back to 1.

Cohort census, `report.rs`, decimal p=3 q in [-2,2]:

```
  cohort size  1: 8460 values
  cohort size  2: 684 values
  cohort size  3: 54 values
  cohort size 10: 1 values      <- zero: five exponent rows times two signs
```

## Probes 4 and 4b: `Implicit`'s exponent

```
$ rustc --edition 2021 --crate-type lib probe_4_implicit_exponent_as_type.rs
(clean)
```

WORKS. `Fx<Two, P8, ENeg<P4>, 1/4, BZero, Symmetric> * Fx<Two, P4, ENeg<P2>, 1/8, ...>` gives
precision 12, exponent -6, adjustment 1/32; a three-step chain gives exponent -8; a decimal fixed
numeral squares from exponent -2 to -4. All forced through `mul_fixed<N1, N2>()`.

```
$ rustc --edition 2021 --crate-type lib probe_4b_implicit_exponent_as_const_refused.rs
error: generic parameters may not be used in const operations
  --> probe_4b_implicit_exponent_as_const_refused.rs:23:21
   |
23 |     type Out = Fx<{ P1 + P2 }, { E1 + E2 }>;
   |                     ^^ cannot perform const operation using `P1`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions

$ rustc ... with #![feature(min_generic_const_args)]
error: complex const arguments must be placed inside of a `const` block

$ rustc ... with min_generic_const_args and the expressions in `const { }` blocks
error: generic parameters may not be used in const operations
   = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

FAILS AS INTENDED under every permitted route. `generic_const_exprs` is forbidden;
`generic_const_args` requires `-Znext-solver=globally`, which the workspace records as mutually
exclusive with the rest of the arrangement.

## price/: the radix-axis depth and compile walls

`gen.py` emits the two spellings of one decimal grid; `sweep.sh` times them with
`rustc --edition 2021 --crate-type lib --emit=metadata`, min-of-3, the shape files 36, 41, 42 and 53
used. `results.csv` and `single.csv` are committed.

Nest depths, analytic, `python3 gen.py depth`:

```
k=1    depth(k)=1   depth(10^k)=4
k=4    depth(k)=3   depth(10^k)=14
k=8    depth(k)=4   depth(10^k)=27
k=19   depth(k)=5   depth(10^k)=64
k=38   depth(k)=6   depth(10^k)=127
k=398  depth(k)=9   depth(10^k)=1323
```

Walls, both compiled:

```
$ rustc --edition 2021 --crate-type lib --emit=metadata gen/absorbed_20.rs
error[E0080]: attempt to compute `2_u64 * 12500000000000000000_u64`, which would overflow
  --> vu_nat_sealed_adj.rs:67:22
   |
67 |     const VAL: u64 = 2 * P::VAL;
   (the diagnostic prints the full sixty-four-constructor type path)

$ rustc --edition 2021 --crate-type lib --emit=metadata gen/absorbed_one_39.rs
error[E0275]: overflow evaluating the requirement `O<O<O<O<O<O<O<...>>>>>>>: Pos`
```

k=38 (depth 127) compiles; k=39 (depth 130) refuses. Attributed to `Pos` rather than to `Gcd`: a `Pos`
of depth 130 placed on the **exponent** axis, with no `Gcd` or `Adjustment` in the chain, refuses
identically (`gen/deep_exp_130.rs`), and depth 127 compiles (`gen/deep_exp_127.rs`).

Single-numeral timings, min-of-3, `single.csv`, timing the `rustc` subprocess only:

| kind | k | ms min | status |
|---|---|---|---|
| radix_one | 10 | 60 | ok |
| absorbed_one | 10 | 60 | ok |
| radix_one | 38 | 60 | ok |
| absorbed_one | 38 | 67 | ok |
| radix_one | 40 | 60 | ok |
| absorbed_one | 40 | 86 | REFUSED |
| radix_one | 398 | **64** | ok |
| absorbed_one | 398 | 154 | REFUSED |

The radix-and-exponent spelling is flat across the whole range; the absorbed spelling climbs and then
stops compiling.

**A measurement caveat, stated because it cost a wrong number in the first draft.** The `sweep.sh` and
`bias_sweep.sh` harnesses time with a shell wrapper that spawns `python3` twice per point, and the
second spawn lands inside the measured interval, adding roughly 30 ms of constant offset to every
absolute figure in `results.csv` and `bias_results.csv`. Every cost figure quoted in file 54 is a
DIFFERENCE between two points of the same harness, where the offset cancels; the absolute figures in
this section were re-measured with the wrapper removed, which is why they are lower than the same
points in `results.csv`.

Source sizes for the decimal64 bottom grid: 519 bytes radix+exponent, 4486 bytes absorbed.

## price/: decimal bias composition cost

`bias_gen.py` reuses file 53's generator shape and forcing discipline (every composition asserted
against a Python-computed reduced `Fraction`). `bias_sweep.sh`, min-of-3, `--emit=metadata`,
`bias_results.csv` committed.

| profile | baseline | mid | high | marginal ms per composition |
|---|---|---|---|---|
| dyadic (`2^a / 2^b`, a,b to 15) | 72 (n=0) | 118 (n=20) | 164 (n=40) | **2.3** |
| dec_quantum6 (`1/10^a`, a,b to 6) | 84 (n=0) | 235 (n=15) | 289 (n=30) | **6.8** |
| dec_quantum (a,b to 9) | 83 (n=0) | 373 (n=20) | 664 (n=40) | **14.5** |
| dec_slope (2-digit numerators, denominators to 10^5) | 78 (n=0) | 450 (n=20) | 919 (n=40) | **21.0** |
| dec_wide (numerators to 10^6, denominators to 10^6) | 92 (n=0) | 1670 (n=20) | 3248 (n=40) | **78.9** |
| distinct16 (all four operands in [2^15, 2^16)) | 73 (n=0) | 2879 (n=20) | 5803 (n=40) | **143.3** |

Linear in every profile, checked at two counts each. The first and last rows are file 53's own controls
(`53:129-137`: 2.1 ms and 143 ms), reproduced from a separately written generator on the same host.

One generator defect worth recording, because it wasted a sweep: the first `dec_quantum` draw sampled
exponents from 1 to 6, giving 36 possible distinct quads, and the "draw until `count` distinct" loop
hangs forever at `count = 40`. The symptom was a sweep that produced no row and no error, which is the
worst shape a harness failure can take. `bias_sweep.log` records the run it killed.

## Regenerating

`libvu54.rlib`, `report` and `price/out/` are build outputs and are not committed. To reproduce:

```
$ cd 54_probes
$ rustc --edition 2021 --crate-type lib --crate-name vu54 probe_1_carriers_born_sealed.rs -o libvu54.rlib
$ rustc --edition 2021 -O report.rs -o report && ./report
$ cd price && ./sweep.sh && ./bias_sweep.sh
```

`price/gen/` holds the generated sources for every point in the CSVs, including `deep_exp_127.rs` and
`deep_exp_130.rs`, the two fixtures that attribute the depth wall to `Pos` rather than to `Gcd`.

## What these artifacts do NOT establish

- The `.rs` files here are probes, not shipping tests. The seal fixtures would move to
  `mock/crates/arvo/tests/ui/` in the shape file 52 specified, and the `.stderr` capture is
  trybuild's bootstrap, not this dispatch's.
- `s3_predicted` is checked against exhaustive enumeration at model widths, not proved. It enumerates
  four sources of a second datum and the enumeration is "every source the model can produce", which is
  weaker than "every source that exists", exactly the grading file 52 applied to the seal's four routes
  (`52:317-322`).
- `crossing.rs` models the datum space abstractly, as (sign, exponent row, significand) plus reserved
  regions. It is not a bit-level layout and it does not model DPD declets, only the non-canonical-code
  consequence that distinguishes BID from DPD at this level.
- The BID non-canonical rule is read from secondary sources, not the standard.
- Nothing here is a runtime measurement. Every millisecond figure is compile time under
  `--emit=metadata`.
