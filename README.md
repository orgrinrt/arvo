# `arvo`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/arvo.svg)](https://github.com/orgrinrt/arvo/stargazers)
[![Crates.io](https://img.shields.io/crates/v/arvo)](https://crates.io/crates/arvo)
[![docs.rs](https://img.shields.io/docsrs/arvo)](https://docs.rs/arvo)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/arvo.svg)](https://github.com/orgrinrt/arvo/issues)
![License](https://img.shields.io/github/license/orgrinrt/arvo?color=%23009689)

> Fixed-point numeric substrate plus analysis algorithms. `#![no_std]`, no alloc, no platform dep. Every numeric type carries a `Strategy` marker, monomorphisation is the dispatch.

</div>

## What it is

`arvo` replaces bare integer and float primitives with fixed-point types that carry a strategy marker. `UFixed<I, F, S>` is an unsigned value with `I` integer bits, `F` fractional bits, and `S: Strategy` selecting the precision-versus-throughput tradeoff at monomorphisation time. `IFixed<I, F, S>` is its signed counterpart. `FastFloat<S>` and `StrictFloat<S>` cover floating-point calls where a consumer needs `f32` / `f64` shape with per-call-site strategy control.

Strategy markers are zero-sized types that the compiler uses to pick implementation at each call site. `Hot` assumes invariants are proven and compiles to bare-primitive arithmetic. `Warm` is the default and takes a pragmatic tradeoff between width and speed. `Cold` prefers correctness over throughput. `Precise` supports widths exceeding any host primitive via multi-limb backing. The same `UFixed<16, 0, ?>` compiles to different code depending on `?`, letting the caller set the tradeoff without the callee branching.

Above the numerics, a small set of crates layers bit-level contracts, fixed-shape tensors, bitmask concretes, hash-domain containers, and a handful of generic analysis algorithms. Algorithm crates take trait bounds rather than concrete `UFixed` or `IFixed`, so the substrate stays usable without pulling in any one algorithm crate.

## Status

**Design phase.** Crate surfaces are scaffolded; shipping implementations land next. Consumers that want to experiment with the shape ahead of release can path-dep into the crates directly.

## Contents

| Crate | Layer | Purpose |
|---|---|---|
| `arvo` | L0 | `UFixed<I, F, S>` / `IFixed<I, F, S>`, `FastFloat<S>` / `StrictFloat<S>`, strategy markers (`Hot`, `Warm`, `Cold`, `Precise`), semantic aliases. |
| `arvo-bits` | L1 | Bit-level contracts: `BitWidth`, `BitAccess`, `BitSequence`, plus opaque-bit `Bits<const N: u8>`. |
| `arvo-bitmask` | L2 | `Mask64`, `Mask256`, `BitMatrix`, bit scanning over the L1 contracts. |
| `arvo-tensor` | L2 | `Array<T, N>`, `Matrix<W, N>` fixed-shape typed storage plus the `Enumerator` trait. |
| `arvo-hash` | L2 | Hash-domain bit-types and algorithms, e.g. `ContentHash`. |
| `arvo-graph` | L2 | DAG algorithms: topo sort, rank, waist, spanning tree. Generic over weight types. |
| `arvo-sparse` | L2 | Sparse matrix storage: CSR, RCM, block diagonal, Dulmage-Mendelsohn. |
| `arvo-comb` | L2 | Combinatorial optimisation: DP, greedy grouping, bin-packing. |
| `arvo-spectral` | L3 | Spectral methods: Laplacian, Fiedler, power iteration. Built on `arvo-sparse` + `arvo-tensor`. |

## Four layers, one rule each

The stack has four dependency-ordered layers. Each layer has a single architectural invariant.

**L0 `arvo`.** Fielded numeric types. Every `pub struct` or `pub type` with a precision-throughput tradeoff carries `S: Strategy`. Default is `Warm`.

**L1 `arvo-bits`.** Contracts only: traits with default methods and blanket impls. No arithmetic fielded structs live here; opaque-bit concretes (today: `Bits<const N: u8>`) are permitted on a small allowlist.

**L2 concrete storage and generic algorithms.** `arvo-bitmask` / `arvo-tensor` / `arvo-hash` for storage; `arvo-graph` / `arvo-sparse` / `arvo-comb` for algorithms. Algorithm crates take trait bounds (`T: UArith<BITS>`, `T: Boundable`, and similar), not concrete `UFixed<...>`.

**L3 `arvo-spectral`.** Built on L2. Spectral analysis over sparse weight types. No dependency back toward L2 peers it does not need.

Dependencies flow strictly L0 to L1 to L2 to L3.

## Strategy markers

| Strategy | When to use | Cold-path cost |
|---|---|---|
| `Hot` | Invariants proven at construction. Fits a host primitive (u64 max). | None. Operations compile to bare-primitive ops. |
| `Warm` | Default. Good tradeoff between width and speed. | Ordinary branch on saturating or checked op. |
| `Cold` | Correctness matters more than speed. Wider logical range permitted. | Full overflow / saturation handling. |
| `Precise` | Width exceeds any host primitive. Multi-limb backing. | Multi-limb arithmetic. |

Dispatch happens at monomorphisation. `UFixed<16, 0, Hot>` and `UFixed<16, 0, Cold>` compile to different code at every call site; the caller picks the tradeoff.

## Installation

```bash
cargo add arvo
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
arvo = "0.1"
```

Consumers that only want bit-level contracts can depend directly on `arvo-bits`; similarly for `arvo-bitmask`, `arvo-tensor`, and the algorithm crates. Each sub-crate is independently usable subject to its layer position.

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
