# Sketch 03: `LinearOperator<F, N>` + `SparseLaplacian` operator

**Status**: DESIGN-PROBE-DURING-SRC

## Question

The current `arvo-spectral::fiedler_vector<const N: Cap, W, F>` takes a dense `&Matrix<W, N>`. The round proposes a sealed `LinearOperator<F, N>` trait whose `apply(&self, x: &[F; cap_size(N)], out: &mut [F; cap_size(N)])` is the only thing spectral methods consume. Two impls land: the existing `Matrix<W, N>` (treats itself as `L = D - A` and applies dense SpMV) and the new `SparseLaplacian<T, W>` where `T: SparseAdjacency<N>` (computes `L * x` directly from CSR adjacency).

## Proposed shape

```rust
mod sealed_op { pub trait Sealed {} }

pub trait LinearOperator<F, const N: Cap>: sealed_op::Sealed
where
    [(); cap_size(N)]:,
{
    fn apply(&self, x: &[F; cap_size(N)], out: &mut [F; cap_size(N)]);
}

pub struct SparseLaplacian<'a, T, W>
where
    T: SparseAdjacency<N>,
    W: ...,
{
    adjacency: &'a T,
    weights: &'a [W; ...],  // per-edge weights, indexed by CSR position
    _phantom: PhantomData<F>,
}

impl<'a, F, const N: Cap, T, W> LinearOperator<F, N> for SparseLaplacian<'a, T, W>
where
    T: SparseAdjacency<N>,
    W: Into<F> + Copy,
    F: Add<Output = F> + Sub<Output = F> + Mul<Output = F> + FromConstant + Copy,
    [(); cap_size(N)]:,
{
    fn apply(&self, x: &[F; cap_size(N)], out: &mut [F; cap_size(N)]) {
        // L * x = D * x - A * x
        // For each row i:
        //   degree_sum = sum over j in successors(i) of weight(i, j)
        //   adj_sum    = sum over j in successors(i) of weight(i, j) * x[j]
        //   out[i] = degree_sum * x[i] - adj_sum
        let n = cap_size(N);
        let mut i = 0;
        while i < n {
            let mut degree_sum = F::from_constant::<{ USize(0) }>();
            let mut adj_sum    = F::from_constant::<{ USize(0) }>();
            // Walk row i's successors via the trait.
            for (edge_pos, j) in self.adjacency.successors_indexed(NodeId::new(USize(i))) {
                let w: F = self.weights[edge_pos.0].into();
                degree_sum = degree_sum + w;
                adj_sum    = adj_sum    + w * x[j.0.0];
            }
            out[i] = degree_sum * x[i] - adj_sum;
            i += 1;
        }
    }
}
```

`successors_indexed` is a paired method on `SparseAdjacency` that yields `(EdgeIndex, NodeId)` so the implementation can index into the per-edge weights slice in the CSR case. Bitmask impls can implement it by counting set bits to produce a synthetic edge index.

## Why this is a design probe, not a live compile probe

Three reasons:

1. `LinearOperator<F, N>` is a plain trait (not `pub const trait`). The trait-solver complexity is one step less than S1, which already passed.

2. The `apply` method body is straightforward arithmetic; the only abstraction it touches is `SparseAdjacency::successors_indexed`, which is a small extension of the already-validated S1 surface.

3. Power iteration and Fiedler's outer loops invoke `op.apply(&current, &mut next)` repeatedly, replacing the current `mat_vec_mul(weights, x)` call. The substitution is mechanical.

## Open question for SRC phase

Whether to expose `successors_indexed` as a trait method or as a free helper that consumers implement alongside. Trait-method form is cleaner; free-helper form is easier for ad-hoc adjacency representations. Resolution: trait method with a default impl that walks `successors` and counts edges off the row offset; CSR overrides for the cheap O(1) edge-index path.

## Risk if this design-probe is wrong

If `LinearOperator` collides with the existing `Matrix<W, N>` impl shape (e.g., the dense Matrix cannot match the trait without changing its constructor), the fallback is to ship `SparseLaplacian::fiedler_iterate` as a standalone function alongside `fiedler_vector`, with no operator trait. Less elegant; still unblocks hilavitkutin.
