# The sum through the Sum contract against Num::add, an ad hoc disassembly reading

2 variants, 40 samples per variant.
Baseline: **contract_sum_concrete**

## Highlights

Baseline for all deltas below: **contract_sum_concrete**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (contract_sum_concrete) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline contract_sum_concrete has the worst median (54 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest contract_sum_generic at 54 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.2% (fastest 54 ns to slowest 54 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (10 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### contract_sum_concrete is inconsistent: worst-20% is 2.0x its best-20%

contract_sum_concrete's best 20% of batches run at 37 ns but its worst 20% at 76 ns (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: contract_sum_generic** at 54.1 ns median (-0.2% vs baseline)
- Spread: 1.00x (fastest 54.1 ns, slowest 54.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The sum through the Sum contract against Num::add, an ad hoc disassembly reading |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x18ea08e0e768fda1 |
| Counter frequency | 2112811718 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| contract_sum_concrete | 424ns | 378ns | 264ns | 406ns | 636ns | base |
| contract_sum_generic | 418ns | 377ns | 293ns | 388ns | 631ns | -1.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| contract_sum_concrete | 56ns | 37ns | 76ns | base | 1.150 |
| contract_sum_generic | 55ns | 42ns | 71ns | -0.64% | 1.158 |

## Performance model

- Peak throughput: **1.711 Gops/s** (contract_sum_concrete; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| contract_sum_concrete | 1.181 | 69.0% |
| contract_sum_generic | 1.183 | 69.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| contract_sum_concrete | 424ns | 424ns | base |
| contract_sum_generic | 418ns | 418ns | -1.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| contract_sum_concrete | 54ns | base | --- | [54, 54] | --- | --- | --- | --- |
| contract_sum_generic | 54ns | no significant difference | [-0, +4]ns | [54, 54] | no | 0.3368 | 0.3368 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | contract_sum_concrete | contract_sum_generic |
|---|---|---|---|
| 1 | 1 | 54ns | +27.5% |
| 1 | 2 | 38ns | +11.2% |
| 1 | 3 | 76ns | -28.4% |
| 1 | 4 | 54ns | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| contract_sum_concrete | -0.498 | moderate- |
| contract_sum_generic | -0.469 | moderate- |

**Consistency summary:**

- **contract_sum_generic**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| contract_sum_concrete | 126.8ns | 55.6ns | 227.9% | HIGH |
| contract_sum_generic | 117.6ns | 55.3ns | 212.7% | HIGH |

## Distribution (algo ns)

```
contract_sum_concrete (n=40, range 37.4-75.8 ns)
     37.4 |####################
     39.3 |
     41.3 |
     43.2 |
     45.1 |
     47.0 |
     48.9 |
     50.9 |
     52.8 |########################################
     54.7 |
     56.6 |##
     58.5 |
     60.5 |
     62.4 |##
     64.3 |
     66.2 |
     68.2 |
     70.1 |
     72.0 |
     73.9 |#############
  (1 below, 4 above range)

contract_sum_generic (n=40, range 42.1-70.8 ns)
     42.1 |###########
     43.6 |
     45.0 |
     46.4 |#####
     47.9 |
     49.3 |
     50.7 |
     52.2 |#####
     53.6 |########################################
     55.0 |##
     56.5 |
     57.9 |
     59.3 |
     60.7 |
     62.2 |####################
     63.6 |
     65.0 |
     66.5 |
     67.9 |
     69.3 |
  (5 below, 5 above range)

```

## Diagnostics

- **contract_sum_concrete**: CV=24.4% (high variance, measurements may be unstable)
- **contract_sum_concrete**: bridge=211.7% of algo (FFI overhead may distort results)
- **contract_sum_generic**: bridge=212.6% of algo (FFI overhead may distort results)
