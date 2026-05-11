# Round 202605111719 findings: graph + spectral CSR-driven algorithms

Bench-findings landing zone. Populated by task #433 before the src CL
locks. Two bench bundles per the topic file Subtopic F.

## Bundle 1: structural-decomposition

RCM, block-diagonal, and Dulmage-Mendelsohn bench results across:

- N in {32, 128, 512, 2048}
- Average out-degree in {2, 5, 20}
- Structured shapes: random, linear chain, full fan-out, layered (width 8)
- W-axis: `Bits<64>` vs `Bits<256>` (where N allows) vs CSR-driven path

Per algorithm:

- RCM: bandwidth before, bandwidth after, runtime.
- block_diagonal: number of components found vs ground truth, runtime.
- Dulmage-Mendelsohn: classification stability under permutation, runtime.

Cross-comparison: BitMatrix-driven vs CSR-driven at N where both fit.

(Populated by task #433.)

## Bundle 2: spectral-bisection

Power iteration and Fiedler vector convergence + partition quality:

- Inputs: same DAG shapes as Bundle 1, plus weighted variants with
  weights drawn from {uniform, exponential} distributions.
- Operator-axis: dense `Matrix<W, N>` Laplacian vs sparse
  `SparseLaplacian<T, W>` operator.

Per setting:

- Iterations to converge (eigenvalue gap).
- Final cut quality (number of edges crossing the partition).
- Runtime.

(Populated by task #433.)

## Decisions driven by these benches

Filled when benches land. Drives concrete src-CL choices:

- Default iteration count for `power_iteration` / `fiedler_vector`
  per workload shape.
- Threshold where `SparseLaplacian` beats `Matrix<W, N>` (informs
  consumer-facing guidance, not algorithm selection).
- W-choice guidance: where `Bits<256>` beats `Bits<64>` on the same
  algorithm (informs the default-W alias choice).
