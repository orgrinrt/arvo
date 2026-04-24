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

`arvo` is the numeric and analysis substrate underneath the hilavitkutin stack. It replaces bare integer and float primitives with `UFixed<I, F, S>` / `IFixed<I, F, S>` / `FastFloat<S>` / `StrictFloat<S>`, where `S: Strategy` selects the precision and throughput tradeoff at monomorphisation time. Downstream crates (`hilavitkutin`, `clause`) compose arvo types into their own signatures; the substrate never reaches for `std`, `alloc`, or runtime dispatch.

The `arvo-types-only` lint in consumer crates is the backstop for this discipline. Every `u8`..`u128`, `i8`..`i128`, `f32`, `f64`, `bool`, `usize`, `isize` in downstream public API positions must flow through `UFixed` / `IFixed` / `FastFloat` / `StrictFloat` / `Bool` / `USize` / `Cap` or a semantic alias built on them. The Strategy axis (`Hot` / `Warm` / `Cold` / `Precise`) carries the per-call-site tradeoff without forcing the caller to pick a host width.

Above the numerics sit bit-level contracts (`arvo-bits`), typed fixed-shape tensors (`arvo-tensor`), bitmask concretes (`arvo-bitmask`), hash-domain containers (`arvo-hash`), and a handful of algorithm crates (`arvo-graph`, `arvo-sparse`, `arvo-comb`, `arvo-spectral`). Algorithm crates stay generic over trait bounds rather than over `UFixed` directly; the substrate is usable without any one algorithm crate.

## Status

**Design phase.** Crates are scaffolded in `mock/` with authoritative `DESIGN.md.tmpl` files and per-crate deep dives. No shipping implementations yet; the next step is implementing L0 + L1 in the mockspace, validating, then graduating to root-level `crates/`.

Consumers that want to experiment against the shape should path-dep into `mock/crates/*` for now.

## Contents

| Crate | Layer | Purpose |
|---|---|---|
| `arvo` | L0 | `UFixed<I, F, S>` / `IFixed<I, F, S>`, `FastFloat<S>` / `StrictFloat<S>`, strategy markers, semantic aliases. |
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

**L0 — `arvo`.** Fielded numeric types. Every `pub struct` or `pub type` with a precision/throughput tradeoff carries `S: Strategy`. Default is `Warm`. The `strategy-marker-required` lint enforces this.

**L1 — `arvo-bits`.** Contracts only: traits with default methods and blanket impls. No arithmetic fielded structs live here; opaque-bit concretes on a small allowlist (today: `Bits<const N: u8>`) are permitted. New additions go through a design round.

**L2 — concrete storage and generic algorithms.** `arvo-bitmask` / `arvo-tensor` / `arvo-hash` for storage; `arvo-graph` / `arvo-sparse` / `arvo-comb` for algorithms. Algorithm crates take trait bounds (`T: UArith<BITS>`, `T: Boundable`, ...), not concrete `UFixed<...>`.

**L3 — `arvo-spectral`.** Built on L2. Spectral analysis over sparse weight types. No dependency back toward L2 peers it does not need.

Dependencies flow strictly L0 → L1 → L2 → L3. The per-crate `forbidden-imports` lint configuration is the backstop.

## Strategy markers

Strategy markers are zero-sized types passed as the `S` parameter:

| Strategy | When to use | Cold-path cost |
|---|---|---|
| `Hot` | Invariants proven at construction. Fits a host primitive (`u64` max). | None. Operations compile to bare-primitive ops. |
| `Warm` | Default. Good tradeoff between width and speed. | Ordinary branch on saturating/checked op. |
| `Cold` | Correctness matters more than speed. Wider logical range permitted. | Full overflow/saturation handling. |
| `Precise` | Width exceeds any host primitive. Multi-limb backing. | Multi-limb arithmetic. |

The dispatch happens at monomorphisation. `UFixed<16, 0, Hot>` and `UFixed<16, 0, Cold>` compile to different code at every call site; the caller chooses the tradeoff without the callee branching.

## Installation

```bash
cargo add arvo
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
arvo = "0.1"
```

As part of the hilavitkutin stack, `arvo` is reached transitively through `hilavitkutin` or `clause`. Add it directly when a downstream crate needs `UFixed` / `IFixed` / `FastFloat` / `StrictFloat` / strategy markers without pulling in a pipeline engine.

## Usage

```rust
use arvo::{UFixed, IFixed, Hot, Warm, Cold};

// Warm by default; the caller picks strategy explicitly when it matters.
let ordinary: UFixed<16, 0, Warm> = UFixed::from(42u16);

// Hot: proven-range value, no saturation. Compiles to bare u16 ops.
let hot: UFixed<16, 0, Hot> = UFixed::from(7u16);

// Cold: wider logical range, full saturation on overflow.
let cold: IFixed<32, 16, Cold> = IFixed::from_bits(0x0001_0000);
```

## Positioning

`arvo` sits directly above [`notko`](https://github.com/orgrinrt/notko) (foundation primitives: `Just` / `Maybe` / `Outcome` / `Boundable` / `NonZeroable`) and is consumed by [`hilavitkutin`](https://github.com/orgrinrt/hilavitkutin) (pipeline engine) and the `clause-*` compiler and runtime crates. `arvo` provides the `Boundable` / `NonZeroable` impls for its own `UFixed` / `IFixed` types; `notko` defines the traits.

Public APIs in the downstream crates use `UFixed` / `IFixed` / `FastFloat` / `StrictFloat` / `Bool` / `USize` / `Cap` (or semantic aliases built on them) in place of bare std numerics. Bare primitives appear only in trait method signatures fixed by the language.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/arvo/blob/dev/LICENSE)
