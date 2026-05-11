# Sketch 02: Generic-over-`T: SparseAdjacency` block-diagonal body

**Status**: DESIGN-PROBE-DURING-SRC

## Question

The current `arvo-sparse::block_diagonal<const N: Cap>` body is hardcoded to `BitMatrix<Bits<64, Hot, Unsigned>, N>`. The round proposes lifting it to generic-over-`T: SparseAdjacency<N>`. Does the body shape carry over cleanly?

## Proposed shape

```rust
pub fn block_diagonal<const N: Cap, T>(
    adjacency: &T,
) -> (USize, [USize; cap_size(N)])
where
    T: SparseAdjacency<N>,
    [(); cap_size(N)]:,
{
    let mut block_id: [USize; cap_size(N)] = [USize(0); cap_size(N)];
    let mut visited: VisitedMask<N> = VisitedMask::empty();
    let mut next_id = USize(0);

    let mut seed = 0usize;
    while seed < cap_size(N) {
        if visited.contains(seed) {
            seed += 1;
            continue;
        }
        let id = next_id;
        next_id = next_id + USize::ONE;

        let mut stack: [NodeId; cap_size(N)] = [NodeId::ZERO; cap_size(N)];
        let mut sp = 0usize;
        stack[sp] = NodeId::new(USize(seed));
        sp += 1;
        visited.insert(seed);
        block_id[seed] = id;

        while sp > 0 {
            sp -= 1;
            let node = stack[sp];

            // Use the trait's successor iterator. Need predecessor too for
            // undirected components; this uses BidirectionalSparseAdjacency.
            for n in adjacency.successors(node) {
                if !visited.contains(n.0) {
                    visited.insert(n.0);
                    block_id[n.0] = id;
                    stack[sp] = n;
                    sp += 1;
                }
            }
            for n in adjacency.predecessors(node) {
                if !visited.contains(n.0) {
                    visited.insert(n.0);
                    block_id[n.0] = id;
                    stack[sp] = n;
                    sp += 1;
                }
            }
        }

        seed += 1;
    }

    (next_id, block_id)
}
```

The `VisitedMask<N>` here stands in for the existing `Mask<Bits<64, Hot, Unsigned>>` only when `N <= 64`. For larger `N`, the round introduces a wider visited mask shape (likely `[USize; (N + 63) / 64]` or a `BitMatrix`-row-style chunked representation). Resolution in src CL.

## Why this is a design probe, not a live compile probe

Three reasons:

1. The generic-over-trait body is a one-line substitution (`adj.successors(...)` for `adj.successors(...)` literal trait call) once `SparseAdjacency` is in place. No new rustc machinery is exercised beyond what S1 already validated.

2. The body shape (BFS / DFS with array stack + visited mask) is unchanged from the existing impl. The substitution is mechanical.

3. Live probing requires either (a) introducing `SparseAdjacency` into arvo-sparse during the probe, polluting source state for the trait-solver test, or (b) writing the probe as a standalone replica with stub types, replicating the entire CSR + BitMatrix surface in 200+ lines for marginal additional confidence over S1.

If the SRC-phase substitution surfaces a rustc problem the topic does not foresee, that's recorded in the SRC CL per `cl-claim-sketch-discipline.md` and the topic gets revised.

## Risk if this design-probe is wrong

If the generic substitution does not compile, the fallback is to keep `block_diagonal` hardcoded to `BitMatrix` and ship a parallel `block_diagonal_csr` for the CSR-shaped consumer. The function-pair-per-representation shape is acceptable; the trait shape is the cleaner design but not load-bearing for unblocking hilavitkutin. Resolution in src CL.
