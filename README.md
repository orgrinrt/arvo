# `arvo`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/arvo.svg)](https://github.com/orgrinrt/arvo/stargazers)
[![Crates.io](https://img.shields.io/crates/v/arvo)](https://crates.io/crates/arvo)
[![docs.rs](https://img.shields.io/docsrs/arvo)](https://docs.rs/arvo)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/arvo.svg)](https://github.com/orgrinrt/arvo/issues)
![License](https://img.shields.io/github/license/orgrinrt/arvo?color=%23009689)

> Fixed-point numerics plus analysis algorithms. `#![no_std]`, no alloc, no platform dep. Every numeric type carries a `Strategy` marker, monomorphisation is the dispatch.

</div>

## What it is

`arvo` replaces bare integer and float primitives with fixed-point types that carry a strategy marker. `UFixed<I, F, S>` is an unsigned value with `I` integer bits, `F` fractional bits, and `S: Strategy` selecting the precision-versus-throughput tradeoff at monomorphisation time. `IFixed<I, F, S>` is its signed counterpart, sharing the same `Bits<{1+I+F}, S, Signed>` storage primitive through a Sign axis. `FastFloat<F>` and `StrictFloat<F>` cover floating-point calls where a consumer needs `f32` / `f64` shape under arvo's typed-numeric umbrella, parameterised by the IEEE float width `F`.

Strategy markers are zero-sized types that the compiler uses to pick implementation at each call site. `Hot` assumes invariants are proven and compiles to bare-primitive arithmetic on a host integer up to 64 bits, or a 16-byte aligned wide container above that. `Warm` is the default and takes a pragmatic tradeoff between width and speed. `Cold` prefers correctness over throughput. `Precise` supports widths up to 256 bits via multi-limb backing. The same `UFixed<16, 0, ?>` compiles to different code depending on `?`, letting the caller set the tradeoff without the callee branching.

arvo is built around six contract crates and a facade. The facade re-exports the consumer-visible surface; the contract crates carry trait declarations as `pub const trait` so calls dispatch at compile time. Above the numerics, a small set of crates layers fixed-shape tensors, bitmask concretes, hash-domain containers, and a handful of generic analysis algorithms. Algorithm crates take trait bounds rather than concrete `UFixed` or `IFixed`, so the foundations stay usable without pulling in any one algorithm crate.

## Status

**Design phase.** Crate surfaces are scaffolded; shipping implementations land next. Consumers that want to experiment with the shape ahead of release can path-dep into the crates directly.

## Contents

| Crate | Layer | Purpose |
|---|---|---|
| `arvo-transparent` | L0 | `Transparent` const unsafe trait, the typed unwrap door for every `repr(transparent)` arvo primitive. Free `arvo::raw` const fn. |
| `arvo-strategy` | L0 | `Strategy` marker plus the four ZST markers `Hot` / `Warm` / `Cold` / `Precise`. `BitsContainerFor<N, Sign>` projection from (strategy, width, sign) to dispatched container. Hosts the `Width` newtype. |
| `arvo-storage` | L0 | `Bits<N, S, Sign>` opaque storage primitive (`repr(transparent)`). `WideBits<BYTES>` byte-sequence storage at align-1 (Warm/Cold/Precise above 128 bits). `AlignedWideBits16<BYTES>` at align(16) (Hot above 128 bits). `Bool`, `USize` platform wrappers. `Unsigned` / `Signed` Sign markers. |
| `arvo-bits-contracts` | L0.5 | `HasBitWidth`, `BitAccess`, `BitSequence`, `BitLogic`, `BitPrim`, `IBitPrim`, `Narrow<T>`. All `pub const trait`. |
| `arvo-mask-contracts` | L0.5 | `Mask<const W: u8>` const trait surface for fixed-width bit set / clear / test / count. |
| `arvo-numeric-contracts` | L0.5 | `Abs`, `Recip`, `Sqrt`, `TotalOrd`, `FromConstant`, `Predicate`, plus the `IsZero` / `IsPositive` / `IsNonZero` predicate family. All `pub const trait`. |
| `arvo-bits` | L1 | Concrete blanket impls of every L0.5 bit-level trait on `Bits<N, S>` and the bare primitive containers, plus the bit-storage aliases (`Bit`, `Nibble`, `Byte`, `Word`, `DWord`, `QWord`). |
| `arvo-bitmask` | L2 | `Mask64`, `Mask256`, `BitMatrix`. Implements `Mask<const W: u8>` from arvo-mask-contracts. |
| `arvo-tensor` | L2 | `Array<T, N>`, `Matrix<W, N>` fixed-shape typed tensor wrappers. `Cap` newtype plus `Enumerator` trait for USize-indexed iteration. |
| `arvo-refit` | L2 | Re-export gateway for the bit-width refit family. Re-exports `Narrow<T>` / `Widen<T>` / `Narrowed` / `Widened`. |
| `arvo-hash` | L2 | Hash family (`Fnv1a` plus future better defaults). `Hasher<const N: u8>` per-N impls. `ContentHash` typed alias. |
| `arvo-graph` | L2 | DAG algorithms: topo sort, rank, waist, spanning tree. Generic over weight types. |
| `arvo-sparse` | L2 | Sparse matrix storage: CSR, RCM, block diagonal, Dulmage-Mendelsohn. Generic over value types. |
| `arvo-comb` | L2 | Combinatorial optimisation: DP, greedy grouping, bin-packing. Generic over cost types. |
| `arvo-spectral` | L3 | Spectral methods: Laplacian, Fiedler, power iteration. Built on `arvo-sparse` plus `arvo-tensor`. |
| `arvo` | facade | Re-exports the consumer surface. `UFixed<I, F, S>`, `IFixed<I, F, S>`, `Uint<N, S>` / `Int<N, S>`, `FastFloat<F>` / `StrictFloat<F>`, `bitfield!` declarative macro. |

## Five layers, one rule each

The stack has five dependency-ordered layers. Each layer has a single architectural invariant.

**L0 foundation (`arvo-transparent`, `arvo-strategy`, `arvo-storage`).** Storage primitives plus the `Transparent` unwrap door, strategy markers, and container projection. Every `pub struct` or `pub type` with a precision-throughput tradeoff carries `S: Strategy` from this layer. Default is `Warm`.

**L0.5 trait contracts (`arvo-bits-contracts`, `arvo-mask-contracts`, `arvo-numeric-contracts`).** Trait declarations only. Every consumer-walkable trait is declared `pub const trait` so call sites can dispatch in const contexts. Default-method bodies live here; concrete impls live in L1.

**L1 blanket-impl carrier (`arvo-bits`).** Concrete blanket impls of L0.5 bit-level traits on `Bits<N, S>` and bare primitive containers. Bit-storage aliases (`Bit`, `Nibble`, `Byte`, `Word`, `DWord`, `QWord`).

**L2 concretes plus generic algorithms.** Mask concretes (`arvo-bitmask`), value-storage tensors (`arvo-tensor`), refit gateway (`arvo-refit`), hash family (`arvo-hash`). Algorithm crates (`arvo-graph`, `arvo-sparse`, `arvo-comb`) take trait bounds (`T: UArith<BITS>`, `T: Boundable`, and similar), not concrete `UFixed<...>`.

**L3 spectral methods (`arvo-spectral`).** Built on L2. Spectral analysis over sparse weight types. No dependency back toward L2 peers it does not need.

The facade `arvo` sits above L3 and re-exports the consumer surface. Dependencies flow strictly L0 to L0.5 to L1 to L2 to L3, then up to the facade. Algorithm crates depend on `arvo` plus `arvo-bitmask` plus `arvo-tensor`, never on `arvo-bits` directly.

## Strategy markers

| Strategy | When to use | Storage path | Cold-path cost |
|---|---|---|---|
| `Hot` | Invariants proven at construction. | Native primitive up to 64 bits; `AlignedWideBits16` (align 16) above. | None. Operations compile to bare-primitive ops. |
| `Warm` | Default. Good tradeoff between width and speed. | Native primitive up to 128 bits; `WideBits` (align 1) above. | Ordinary branch on saturating or checked op. |
| `Cold` | Correctness matters more than speed. | Same shape as `Warm`; semantics flip toward saturating / checked on overflow. | Full overflow / saturation handling. |
| `Precise` | Width exceeds any host primitive. | `WideBits` multi-limb backing up to 256 bits. | Multi-limb arithmetic. |

Dispatch happens at monomorphisation. `UFixed<16, 0, Hot>` and `UFixed<16, 0, Cold>` compile to different code at every call site; the caller picks the tradeoff. The Sign axis on `Bits<N, S, Sign>` is independent of the Strategy axis: signed and unsigned types share container projection and bit-level contracts, with sign-aware semantics layered on top.

## Const-callable trait surface

Every consumer-walkable trait in arvo's contract surface is declared `pub const trait`. `BitAccess` / `BitSequence` / `BitLogic` / `Abs` / `Sqrt` / `Recip` / `TotalOrd` / `FromConstant` / `Predicate` from L0.5 are all callable in `const fn` bodies, alongside the typed-const `Bounded` / `Identity` / `SignedIdentity` family that lives in `arvo-strategy` at L0. Bridges from std traits (`ConstPartialEq` over `Bool`, `ConstOrd`, `ConstDefault`, `ConstFrom<T>`, `ConstTryFrom<T, E>`, `ConstDeref`, `ConstAsRef`) sit in the appropriate L0 / L0.5 crate per the trait's home-rule placement (return type's lowest reachable layer).

The `from_constant` family is callable at compile time: typed-const value construction, strategy projection at the trait-solver level, and the const `where` clauses that gate Warm at 33+ bits with a clean diagnostic all compose into one const-callable spine.

## Installation

```bash
cargo add arvo
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
arvo = "0.1"
```

The facade pulls in the full surface. Consumers that only want bit-level contracts can depend directly on `arvo-bits` or any specific L0 / L0.5 crate; the algorithm crates (`arvo-graph`, `arvo-sparse`, `arvo-comb`, `arvo-spectral`) are independently usable subject to their layer position.

## Usage

```rust
use arvo::{UFixed, IFixed, Hot, Warm, Cold};

let ordinary: UFixed<16, 0, Warm> = UFixed::from(42u16);
let hot: UFixed<16, 0, Hot> = UFixed::from(7u16);
let cold: IFixed<32, 16, Cold> = IFixed::from_bits(0x0001_0000);
```

## Positioning

`arvo` sits directly above [`notko`](https://github.com/orgrinrt/notko), which supplies the foundation primitives (`Just`, `Maybe`, `Outcome`, `Boundable`, `NonZeroable`) that `arvo` types satisfy. Consumer crates downstream use `UFixed` / `IFixed` / `FastFloat` / `StrictFloat` / `Bool` / `USize` / `Cap` in place of bare integer and float primitives.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/arvo/blob/dev/LICENSE)
