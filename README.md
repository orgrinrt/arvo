# `arvo`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/arvo.svg)](https://github.com/orgrinrt/arvo/stargazers)
[![Crates.io](https://img.shields.io/crates/v/arvo)](https://crates.io/crates/arvo)
[![docs.rs](https://img.shields.io/docsrs/arvo)](https://docs.rs/arvo)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/arvo.svg)](https://github.com/orgrinrt/arvo/issues)
![License](https://img.shields.io/github/license/orgrinrt/arvo?color=%23009689)

> Numeric and analysis primitives with strategy tags dictating speed, precision and layout per call site. `no_std`, no alloc, no platform deps.

</div>

`arvo` ships fixed-point and float types tagged by a `Strategy` marker that picks the trade between speed, precision and layout at every call site. Four strategies cover the spectrum: `Hot` (fastest path, wraps on overflow), `Warm` (default, pragmatic), `Cold` (widens conditionally on the rare branch, bitpacks at the column-store layer), and `Precise` (widens for every op, narrows back on assign). `UFixed<I, F, S>` is unsigned with `I` integer bits and `F` fractional bits; `IFixed<I, F, S>` is its signed counterpart; `FastFloat<F>` and `StrictFloat<F>` cover IEEE floats with the same strategy tag.

Above the numerics, `arvo-bitmask` ships `Mask64`, `Mask256`, and `BitMatrix` for set membership and packing; `arvo-tensor` ships `Array<T, N>` and `Matrix<W, N>` for arrays and matrices of fixed size; `arvo-hash` ships `Fnv1a`, `Hasher<N>`, and `ContentHash` for hashing.

On top of those, the analysis crates are generic over trait bounds rather than concrete numeric types. `arvo-graph` covers DAG operations (topological sort, rank, waist, spanning tree); `arvo-sparse` covers sparse matrix layouts (CSR, RCM, block-diagonal, Dulmage-Mendelsohn); `arvo-comb` covers combinatorial optimisation (DP, greedy grouping, bin-packing); `arvo-spectral` covers spectral methods (Laplacian, Fiedler vector, power iteration). Each is independently usable.

Pairs with [`notko`](https://github.com/orgrinrt/notko)'s `#[profile]` attribute, which rewrites strategy tags on arvo types named directly in a function body. The proc-macro pass handles direct uses; aliases and generic positions pass through unchanged. Reaching further (into aliases, generics, and types that arrive through traits) is planned via `hilavitkutin-build`'s MIR-level rewrite hooks. `arvo` is `#![no_std]`, no alloc, no platform deps.

## Strategy markers

Types name the exact width: `UInt<3>`, `UInt<47>`, `Int<13>`. The strategy tag on the type, together with the width, picks the container, the carrier width for arithmetic, the operation intrinsics, and how aggressively the impl widens, narrows, or hands off to a hand-rolled microkernel. The presentation at the assignment site looks the same across strategies; the distinction lives at every level below.

`Hot` aims at the fastest possible code for the target. The container often snaps to whatever aligns best for SIMD lanes or register width; some hot ops drop into raw asm microkernels where benches show a win, others trust LLVM. Operations wrap on overflow when wrap is the cheapest behaviour, and the invariants are asserted at the call site.

`Warm` is the default pragmatic path. The container fits the width without aggressive widening; arithmetic saturates or checks on overflow when that is faster than wrap. No microkernel detour.

`Cold` is the rare-path strategy. Same shape as `Warm` for the common operation; on the rare branch (overflow, edge case) it widens conditionally to avoid the wrong answer rather than padding everywhere. At the column-store layer above the per-value container, `Cold` columns bitpack adjacent records aggressively rather than padding to the container width.

`Precise` widens unconditionally before each operation (more expensive, more precise), then narrows or rounds back to the typed width on assign. Where the carrier width exceeds 128 bits, all four strategies route through `WideBits<BYTES>` byte-sequence storage; width is unbounded.

Dispatch happens at monomorphisation. `UFixed<16, 0, Hot>` and `UFixed<16, 0, Cold>` compile to different code at every call site; the trade is picked at the call site, and the callee never branches on it. The sign axis on the underlying `Bits<N, S, Sign>` is independent of the strategy axis: signed and unsigned types share container projection and bit contracts, with sign handling layered on top.

## Compile-time construction

Most arvo arithmetic and bit operations are callable from `const fn` bodies. Typed `UFixed<I, F, S>` and `IFixed<I, F, S>` constants build directly in const contexts, without unwrapping to bare bits and back. The `from_constant` family handles typed-value construction, with strategy projection resolved at the trait solver.

Bridge traits cover the std parallels for use in const contexts: `ConstEq`, `ConstOrd`, `ConstDefault`, `ConstFrom`, `ConstTryFrom`, `ConstDeref`, `ConstAsRef`, and `ConstHash`. Each lives in the contract crate per the bridge-home rule (the trait sits in the lowest layer where its return type is reachable).

## Layered structure

`arvo` is split across multiple crates organised in five dependency tiers. A foundation tier carries storage and the strategy markers; a contract tier declares the `pub const` traits; an implementation tier carries the blanket impls; an L2 tier hosts the concretes (masks, tensors, hashes) plus the analysis crates; a top tier ships the spectral methods. The facade `arvo` re-exports the public surface and hosts `UFixed`, `IFixed`, the `Uint<N, S>` and `Int<N, S>` aliases, the IEEE floats, and the `bitfield!` macro. Algorithm crates depend on the facade and the L2 concretes, never reaching back into the underlying contract layer.

## Algorithms

Algorithms ship alongside the primitives because the strategy axis has nowhere to land otherwise. A topological sort or a sparse solver written once over plain numerics ends up with one set of trades forever. The arvo algorithm crates are written generic over trait bounds, with the strategy parameter either flowing through the value type (when the algorithm signature carries `T: TraitBound + Strategy`) or stamped at the call site through `notko`'s `#[profile]` rewrite.

Each implementation is bench-driven. For a given algorithm and strategy, several candidate implementations sit alongside one another: textbook recursion, tight-loop iteration, SIMD-aware variants, raw asm microkernels where the target supports them. Benchmark results decide which implementation a strategy ends up with. The selection is ongoing and not finished. Bench inputs ship with the repo, and the chosen path can shift as targets, intrinsics, or evidence change.

`arvo-graph`, `arvo-sparse`, `arvo-comb`, and `arvo-spectral` each ship their domain (DAG operations, sparse matrix layouts, combinatorial optimisation, spectral methods). They are independently usable and depend only on the foundation and the peer concretes (`arvo-bitmask`, `arvo-tensor`).

## Installation

```bash
cargo add arvo
```

Or in `Cargo.toml`:

```toml
[dependencies]
arvo = "0.1"
```

The `arvo` facade pulls in the full public surface. To pull only a subset (bit contracts, masks, a single algorithm crate), depend on the relevant crate directly.

## Usage

```rust
use arvo::{UFixed, IFixed, Signed, Uint, Hot, Warm, Cold, Precise, USize, Bool, Bits, Mask64};
use arvo::refit::Widen;
use arvo::bits::{BitAccess, BitSequence};
use notko::profile;

// scene a: a hot dsp integration loop. exact-width samples, wider
// accumulator; strategy picks how the arithmetic lowers.

type Sample = UFixed<14, 0, Warm>;     // u16 native; warm saturates on overflow
type Acc    = UFixed<23, 0, Hot>;      // u32 native; hot wraps on overflow

#[profile(Hot)]
fn integrate(samples: &[Sample]) -> Acc {
    let mut acc: Acc = Acc::ZERO;
    for s in samples {
        let w: Acc = s.widen_to();     // 14 -> 23, single u32 widen, const
        acc = acc + w;                 // hot u32 add, no overflow check
    }
    acc
}

// scene b: a cold-stored sensor frame. 47 bits per record packs
// adjacent on disk; 17 bits saved per frame versus a u64-aligned
// layout, scaling linearly with record count. bit ops on each field
// run on the field's underlying carrier (u8 / u16 / u32).

arvo::bitfield! {
    pub struct Frame: 47 {
        timestamp: 27 at 20,           // 27-bit tick counter
        device:     5 at 15,           // 32 device ids
        seq:        9 at  6,           // 9-bit wrapping sequence
        status:     7 at  0,           // 7-bit status register
    }
}

const READY: USize = USize::from_raw(0);
const FAULT: USize = USize::from_raw(6);

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
            n = n + USize::from_raw(1);
        }
    }
    n
}

// frame visibility cull. mask64 carries core::ops::BitAnd.
fn visible(camera: Mask64, alive: Mask64) -> Mask64 { camera & alive }
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

> You can check out the full license [here](https://github.com/orgrinrt/arvo/blob/dev/LICENSE)
