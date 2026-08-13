# `arvo`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/arvo.svg)](https://github.com/orgrinrt/arvo/stargazers)
[![Crates.io](https://img.shields.io/crates/v/arvo)](https://crates.io/crates/arvo)
[![docs.rs](https://img.shields.io/docsrs/arvo)](https://docs.rs/arvo)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/arvo.svg)](https://github.com/orgrinrt/arvo/issues)
![License](https://img.shields.io/github/license/orgrinrt/arvo?color=%23009689)

> Numeric and analysis primitives with strategy tags dictating speed, precision and layout per call site. `no_std`, no alloc, no platform deps.

</div>

`arvo` ships fixed-point types tagged by a `Strategy` marker that picks the trade between speed, precision and layout at every call site. Four strategies cover the spectrum: `Hot` (minimum container, wraps on overflow), `Warm` (default; container twice the logical width, so a single op cannot overflow), `Cold` (minimum container, bitpacked at the column-store layer; widens to operate, narrows on store), and `Precise` (twice-width container, saturates on overflow). `UFixed<I, F, S>` is unsigned with `I` integer bits and `F` fractional bits; `IFixed<I, F, S>` is its signed counterpart. Floats are a separate family without a strategy tag: `FastFloat<F>` carries fast-math semantics, `StrictFloat<F>` holds IEEE 754 bit-exact semantics, and the `Float<F>` alias resolves between them by build cfg.

Above the numerics, `arvo-bitmask` ships the `Mask<W>` chassis and `BitMatrix<W, N>` for set membership and adjacency; `arvo-tensor` ships `Array<T, N>` and `Matrix<W, N>` for arrays and matrices of fixed size; `arvo-hash` ships `Fnv1a`, `XxHash3`, the `Hasher<N>` trait, and `ContentHash` for hashing.

On top of those, the analysis crates are generic over trait bounds rather than concrete numeric types. `arvo-graph` covers DAG operations (topological sort, rank, waist, spanning tree); `arvo-sparse` covers sparse matrix layouts (CSR, RCM, block-diagonal, Dulmage-Mendelsohn); `arvo-comb` covers combinatorial optimisation (DP, greedy grouping, bin-packing); `arvo-spectral` covers spectral methods (Laplacian, Fiedler vector, power iteration). Each is independently usable.

Pairs with [`notko`](https://github.com/orgrinrt/notko)'s `#[profile]` attribute, which rewrites strategy tags on arvo types named directly in a function body. The proc-macro pass handles direct uses; aliases and generic positions pass through unchanged. Reaching further (into aliases, generics, and types that arrive through traits) is planned via `hilavitkutin-build`'s MIR-level rewrite hooks. `arvo` is `#![no_std]`, no alloc, no platform deps.

## Strategy markers

Types name the exact width: `Uint<3>`, `Uint<47>`, `Int<13>`. The strategy tag on the type, together with the width, picks the container, the carrier width for arithmetic, and how the impl widens, narrows, or saturates. The presentation at the assignment site looks the same across strategies; the distinction lives at every level below.

`Hot` optimises for density and operation throughput. The container is the minimum byte-aligned width that fits the logical bits; arithmetic wraps on overflow, one instruction per op, and LLVM vectorises freely.

`Warm` is the default, development-friendly path: store big, operate fast. The container is twice the logical width, so a single add, sub, or mul of values within their logical range cannot overflow the container.

`Cold` optimises for storage density: store small, operate carefully. Minimum container, bitpacked for sub-byte values; at the column-store layer above the per-value container, `Cold` columns bitpack adjacent records rather than padding to the container width. Arithmetic widens to twice the logical width before operating and narrows back on store.

`Precise` stores exactly and operates exactly. The container is twice the logical width (same physical layout as `Warm`), and arithmetic saturates: overflow clamps to the logical min or max rather than wrapping. Above the native container ladder, all four strategies route through `WideBits<BYTES>` byte-sequence storage (16-byte aligned for `Hot`, byte-exact for the rest).

Dispatch happens at monomorphisation. `Uint<16, Hot>` and `Uint<16, Cold>` compile to different code at every call site; the trade is picked at the call site, and the callee never branches on it. The sign axis on the underlying `Bits<N, S, Sign>` is independent of the strategy axis: signed and unsigned types share container projection and bit contracts, with sign handling layered on top.

## Compile-time construction

Most arvo arithmetic and bit operations are callable from `const fn` bodies. Typed `UFixed<I, F, S>` and `IFixed<I, F, S>` constants build directly in const contexts, without unwrapping to bare bits and back. The `from_constant` family handles typed-value construction, with strategy projection resolved at the trait solver.

Bridge traits cover the std parallels for use in const contexts: `ConstEq`, `ConstOrd`, `ConstDefault`, `ConstFrom`, `ConstTryFrom`, `ConstDeref`, `ConstAsRef`, and `ConstHash`. Each lives in the lowest crate where its return type is reachable (the bridge-home rule), from `arvo-transparent` up to `arvo-hash`.

## Layered structure

`arvo` is split across multiple crates organised in five dependency tiers. A foundation tier carries storage and the strategy markers; a contract tier declares the `pub const` traits; an implementation tier carries the blanket impls; an L2 tier hosts the concretes (masks, tensors, hashes) plus the analysis crates; a top tier ships the spectral methods. The facade `arvo` re-exports the public surface of the crates beneath it and hosts `UFixed`, `IFixed`, the `Uint<N, S>` and `Int<N, S>` aliases, the IEEE floats, and the `bitfield!` macro. Algorithm crates depend on the facade and the L2 concretes, never reaching back into the underlying contract layer.

## Algorithms

Algorithms ship alongside the primitives because the strategy axis has nowhere to land otherwise. A topological sort or a sparse solver written once over plain numerics ends up with one set of trades forever. The arvo algorithm crates are written generic over trait bounds, with the strategy parameter either flowing through the value type (when the algorithm signature carries `T: TraitBound + Strategy`) or stamped at the call site through `notko`'s `#[profile]` rewrite.

Each implementation is bench-driven. For a given algorithm and strategy, several candidate implementations sit alongside one another: textbook recursion, tight-loop iteration, SIMD-aware variants, raw asm microkernels where the target supports them. Benchmark results decide which implementation a strategy ends up with. The selection is ongoing and not finished. Bench inputs ship with the repo, and the chosen path can shift as targets, intrinsics, or evidence change.

`arvo-graph`, `arvo-sparse`, `arvo-comb`, and `arvo-spectral` each ship their domain (DAG operations, sparse matrix layouts, combinatorial optimisation, spectral methods). They are independently usable and depend only on the `arvo` facade and the peer concretes (`arvo-bitmask`, `arvo-tensor`).

## Installation

```bash
cargo add arvo
```

Or in `Cargo.toml`:

```toml
[dependencies]
arvo = "0.1"
```

The `arvo` facade covers the numeric core: storage primitives, strategy markers, the fixed-point and float types, the refit traits, and the `bitfield!` macro. Masks, tensors, hashes, and the analysis crates sit above the facade; depend on `arvo-bitmask`, `arvo-tensor`, `arvo-hash`, or the relevant analysis crate directly.

## Usage

```rust
use arvo::{Bits, Bool, Cold, Hot, Identity, USize, Uint, Warm};
use arvo_bits::{BitAccess, BitSequence};
use arvo_bitmask::Mask;

// scene a: a dsp integration loop. same logical width, two
// strategies; the tag picks the container and how arithmetic lowers.

type Sample = Uint<23, Hot>;           // u32 container; hot wraps on overflow
type Acc    = Uint<23, Warm>;          // u64 container; a single add cannot overflow

fn integrate(samples: &[Sample]) -> Acc {
    let mut acc: Acc = Acc::ZERO;
    for s in samples {
        acc = acc + Acc::from(*s);     // hot -> warm strategy conversion, lossless
    }
    acc
}

// scene b: a cold-stored sensor frame. 47 bits per record packs
// adjacent on disk; 17 bits saved per frame versus a u64-aligned
// layout, scaling linearly with record count. bit ops on each field
// run on the field's underlying carrier (u8 / u16 / u32).

arvo::bitfield! {
    pub struct Frame<Cold>: 47 {
        timestamp: 27 at 20,           // 27-bit tick counter
        device:     5 at 15,           // 32 device ids
        seq:        9 at  6,           // 9-bit wrapping sequence
        status:     7 at  0,           // 7-bit status register
    }
}

const READY: USize = USize(0);
const FAULT: USize = USize(6);

fn ready(f: &Frame) -> Bool {
    f.status().bit(READY)              // mask-and on u8 carrier
}

fn active_signals(f: &Frame) -> USize {
    f.status().count_ones()            // popcnt on u8, sized to the 7-bit field
}

fn mark_fault(f: Frame) -> Frame {
    f.with_status(f.status().with_bit_set(FAULT))
}

fn fault_count(frames: &[Frame]) -> USize {
    let mut n = USize::ZERO;
    for f in frames {
        if f.status().bit(FAULT).into() {
            n = n + USize(1);
        }
    }
    n
}

// frame visibility cull. the mask chassis carries core::ops::BitAnd.
type Visible = Mask<Bits<64, Hot>>;
fn visible(camera: Visible, alive: Visible) -> Visible { camera & alive }
```

## Status & features

`arvo` is in the design phase. Crate surfaces are scaffolded across the workspace; shipping implementations land next per tier. To experiment with the shape ahead of release, path-dep into the crates directly.

`arvo` tracks unstable rustc features (`adt_const_params`, `generic_const_exprs`, `const_trait_impl`, `const_param_ty_trait`, `try_trait_v2`) as they mature. Features known to have soundness issues are intentionally skipped.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/arvo/blob/main/LICENSE)
