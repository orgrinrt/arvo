# Round 202605111719 findings: graph + spectral CSR-driven algorithms

Two bench bundles ship for this round, both wired through the
canonical `mockspace-bench-harness` v3 pipeline:

- `mock/benches/variants/structural-decomposition/` (cdylib +
  standalone bin)
- `mock/benches/variants/spectral-bisection/` (cdylib + standalone
  bin)

The cdylibs implement the `Routine` shape from
`mockspace-bench-core` and are dispatched by the orchestrator at
`mock/benches/src/main.rs`. Running `cargo run --release -p
arvo-benches` from `mock/benches/` produces per-N CSV traces +
findings reports (median, percentile breakdowns, bootstrap CIs,
per-cooldown deltas) under each bench-name.

The standalone binaries (`src/main.rs` in each variant directory)
remain as a parallel measurement path that runs without the harness
pipeline. They produce the same algorithms over similar inputs, used
during the round as the audit-trail measurements before harness
integration landed. They stay in the tree because they are simple to
read end-to-end and serve as a sanity check on the harness numbers.

## Framework adaptation

The bench harness was extracted from polka-dots where it serviced
RCM, graph colouring, SpMV, and other complex algorithms over typed
inputs. The `Routine` trait carries `Input: Copy` and `Output:
PartialEq + Debug + Copy`, so any flat-layout type qualifies. The
`bench_variant` macro's typed form emits FFI dispatch by `usize`
literal.

arvo's algorithms take const-generic `Cap` parameters
(`BitMatrix<W, const N: Cap>`, `Matrix<F, const N: Cap>`) since
round 202605021200 lifted `Cap` to a `ConstParamTy`. Bridging that
to the bench macro's `usize` shape via a `cap_of(N)` const-fn at
the Routine's associated-type level triggered a rustc ICE in
`arvo_sparse::rcm::rcm_reorder`'s internal `cap_size(N)` evaluation.

The workaround used here keeps the Routine surface usize-parameterised
and bridges to `Cap`-typed call sites via per-N dispatch in the
variant body. Each supported N maps to a named `Cap` constant
(`C16`, `C32`, `C64`) so the algorithm receives a literal `Cap`,
not a const-fn application. The pattern is verbose but the macro
and the substrate stay unchanged. A canonical "tag-bridge" macro
arg in mockspace-bench-macro (e.g. `wrap = "cap_of"`) would let the
verbose per-N body collapse to a single generic body; tracked as
follow-up work outside this round.

## Bundle 1: structural-decomposition

Routine: `Rcm<const N: usize>` with `Input = RcmInput<N>` (row-bit
storage), `Output = [u32; N]` (permutation).
Variant: `rcm-bits64` (`BitMatrix<Bits<64, Hot, Unsigned>, C>`
backing).
N: {16, 32, 64}. Cross-variant comparison reserved for a
follow-up bench round when alternative RCM impls (asm microkernel,
counting-sort degree, BMI2-accelerated path) land.

Harness numbers (mean over 4 passes x 1000 runs/pass, ~50% random
input from build_input):

| N | mean (e2e) | median (algo) | 95% CI (algo) |
|---|---|---|---|
| 16 | 414ns | 358ns | [351, 364] |
| 32 | 2522ns | 2457ns | [2207, 2696] |
| 64 | 19328ns | 19242ns | [17386, 20847] |

The N=16 to N=64 scaling matches the BFS-dominated RCM cost on
~50%-density inputs. The standalone-binary linear-chain numbers
(2.6us at N=64) bound the lower-density end of the same algorithm
on the same substrate impl. The harness median falls between
linear (2.6us) and random (23.6us); build_input here is biased
random, so the harness number tracks the upper bound.

## Bundle 2: spectral-bisection

Routine: `Fiedler<const N: usize>` with `Input = FiedlerInput<N>`
(dense f32 weight matrix), `Output = FiedlerOutput<N>` (u8 partition
id per node).
Variant: `fiedler-bisect-dense` (composes `laplacian` +
`fiedler_vector` + `spectral_bisection` over a dense
`Matrix<TF, C>` Laplacian).
N: {16, 32, 64}. Cross-variant SparseLaplacian-vs-dense bench
reserved for the follow-up bench round (needs a sparse fixture
builder for the input shape).

Harness numbers (mean over 4 passes x 1000 runs/pass, two-cluster
+ bridge input from build_input, 50 power-iteration steps):

| N | mean (e2e) | median (algo) | 95% CI (algo) |
|---|---|---|---|
| 16 | 6104ns | 5538ns | [4941, 6492] |
| 32 | 17726ns | 17589ns | [17457, 17757] |
| 64 | 72197ns | 71991ns | [71697, 72435] |

Scaling near 4x going N=32 to N=64 matches the O(N^2) dense matvec
cost per iteration, with 50 iterations per call. The two-cluster
input has a wider eigenvalue gap than a uniform Laplacian, which
keeps the variance per-pass low (see the tight CI at N=32 and
N=64). The standalone-binary measurement on the same input shape
(67us at N=64) is the closest comparison; the small overhead delta
in the harness reflects FFI dispatch + per-call counter
instrumentation.

## Cross-bundle context

Across both bundles, the algorithms shipped this round produce
per-call times comfortably below the millisecond budget the
hilavitkutin plan stage needs. Specifically:

- RCM at the conventional 64-node default-W adjacency: under 25us
  per call on this hardware (M-series ARM, 2026 baseline).
- Fiedler + spectral bisection at the same N: under 100us per call,
  including 50 power-iteration steps.

These bounds clear the budget that hilavitkutin's plan stage uses
for graph analysis during scheduler bring-up (sub-millisecond per
plan compilation). The choice of W (Bits<64> vs wider) and operator
(dense vs sparse Laplacian) become consumer-side decisions per the
arvo-toolbox-not-policer principle, with these numbers as one
reference point.

## Per-bundle artefacts

For each `(bench, n)` pair, the harness produces:

- `mock/benches/<bench>_n<N>.csv` raw sample data
- `mock/benches/<bench>_n<N>.meta.json` environment metadata
- `mock/benches/<bench>_n<N>_findings.md` rendered statistical
  report

These artefacts regenerate on every `cargo run --release -p
arvo-benches` from `mock/benches/`. The findings file in
`mock/research/findings_graph_spectral_202605111719.md` (this
document) summarises the per-bundle headlines; the per-N findings
carry the per-pass detail.

## Scope and follow-ups

In scope here (landed):

- Routine impls for RCM and Fiedler over the canonical 64-wide
  adjacency / dense Laplacian shape.
- N in {16, 32, 64}.
- Harness integration via `Routine` + `bench_variant` + bench.toml +
  orchestrator dispatch.
- Standalone-binary parallel measurements for audit trail and quick
  sanity checks.

Deferred to follow-up bench rounds:

- N in {128, 512, 2048} via Bits<256> / WideBits<BYTES> BitMatrix
  capacities (needs wider-W BitMatrix landing on the bench side).
- CSR-driven path at the same N range (needs CSR fixture builder).
- SparseLaplacian vs dense Matrix head-to-head (needs sparse
  fixture builder).
- Routine impls for block_diagonal, dulmage_mendelsohn,
  power_iteration, spectral_bisection-only.
- Cross-variant comparison axis (multiple RCM / Fiedler impls
  benchmarked against each other).
- mockspace upstream: optional `wrap = "ident::path"` arg on
  `bench_variant` for const-tag bridging, so Cap-typed const
  generics flow through without the per-N verbosity used here.

The follow-ups are natural rounds once the wider-W and CSR fixture
infrastructure lands. They do not block this round's lock criteria:
the source impls shipped here are correct under the test suite, the
harness integration is canonical, and the bench numbers are reproducible
on demand.
