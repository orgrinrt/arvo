# Sketch 04: Wider return shapes for DM and `spectral_bisection`

**Status**: DESIGN-PROBE-DURING-SRC

## Question

`dulmage_mendelsohn` currently returns a `DulmageMendelsohn<N>` struct with three `Mask<Bits<64, Hot, Unsigned>>` fields. `spectral_bisection` returns `(Mask<Bits<64, Hot, Unsigned>>, Mask<Bits<64, Hot, Unsigned>>)`. Both cap at 64 nodes regardless of `N`. The round proposes:

```rust
// Replaces DulmageMendelsohn<N> + dulmage_mendelsohn(...) -> DulmageMendelsohn<N>
// New shape mirrors block_diagonal's existing return:
pub fn dulmage_mendelsohn<const N: Cap, T: BidirectionalSparseAdjacency<N>>(
    adjacency: &T,
) -> (USize, [USize; cap_size(N)])
where [(); cap_size(N)]:,
{ ... }

// Class IDs: 0 = horizontal (sinks), 1 = vertical (sources / isolates),
// 2 = square (core). `partition_count` is always 3 for non-empty inputs;
// returning it as `USize` keeps the function shape parallel with
// block_diagonal and spectral_bisection.

// Replaces spectral_bisection<...> -> (Mask, Mask):
pub fn spectral_bisection<const N: Cap, T: SparseAdjacency<N>, F>(
    op: &impl LinearOperator<F, N>,
    iterations: USize,
) -> (USize, [USize; cap_size(N)])
where ...
{ ... }

// Class IDs: 0 = positive Fiedler, 1 = non-positive Fiedler.
// `partition_count` is always 2 unless the bisection degenerates (one half empty),
// in which case partition_count = 1 (everything in class 0).
```

## Why this is a design probe, not a live compile probe

Three reasons:

1. The new shape is identical to `block_diagonal`'s existing return shape (`(USize, [USize; cap_size(N)])`). That signature is already in arvo-sparse's source and compiles. No new rustc shape is introduced.

2. The breaking-change effect is on test-file migration, not on rustc acceptance. The src CL captures the test rehab; no probe is needed for that.

3. The optional `pack_to_mask(&self) -> Maybe<Mask<W>>` helper on the result array is a plain free function. Trait-solver complexity is zero.

## Migration shape for the optional Mask helper

```rust
/// Pack a classification array into a Mask, returned in Maybe::Is when N <= W::WIDTH.
/// Returns Maybe::Isnt when N exceeds the mask width.
pub const fn classification_to_mask<const N: Cap, W>(
    class: &[USize; cap_size(N)],
    target_class: USize,
) -> Maybe<Mask<W>>
where
    W: BitPrim,
    [(); cap_size(N)]:,
{
    if cap_size(N) > W::WIDTH as usize {
        return Maybe::Isnt;
    }
    let mut mask = Mask::<W>::empty();
    let mut i = 0;
    while i < cap_size(N) {
        if class[i] == target_class {
            mask.insert(USize(i));
        }
        i += 1;
    }
    Maybe::Is(mask)
}
```

Lives at the arvo-sparse module root. Consumers that have known-small `N` use it to recover the existing `Mask` semantics; consumers at scale skip it and operate on the array directly. No deprecation alias.

## Open question for SRC phase

Whether to keep `DulmageMendelsohn<N>` as a typed struct (with the three classification arrays separated out into fields) or return the flat `(USize, [USize; cap_size(N)])` pair. The typed-struct form is friendlier to consumers reading the classification; the pair form is parallel with `block_diagonal`. Resolution: ship a thin newtype wrapper struct around the pair so consumer code reads `dm.classification[i]` while the underlying layout stays uniform with `block_diagonal`. Defer to src CL.

## Risk if this design-probe is wrong

Lowest-risk of the four sketches; the new shape is already validated by `block_diagonal`. Failure modes are all SRC-time migration issues (test rehab, callers in arvo / hilavitkutin / vehje), captured by the src CL claim grammar.
