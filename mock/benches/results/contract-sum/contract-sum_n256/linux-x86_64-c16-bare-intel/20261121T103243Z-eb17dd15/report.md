# The sum through the Sum contract against Num::add, an ad hoc disassembly reading

2 variants, 40 samples per variant.
Baseline: **contract_sum_concrete**

## Highlights

Baseline for all deltas below: **contract_sum_concrete**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (contract_sum_concrete)

The baseline contract_sum_concrete is the fastest (48 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### contract_sum_generic is inconsistent: worst-20% is 2.1x its best-20%

contract_sum_generic's best 20% of batches run at 47 ns but its worst 20% at 99 ns (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (contract_sum_concrete) is the fastest** at 47.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 47.5 ns, slowest 47.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The sum through the Sum contract against Num::add, an ad hoc disassembly reading |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x18ea08ee541a9b81 |
| Counter frequency | 2112811718 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| contract_sum_concrete | 418ns | 335ns | 299ns | 347ns | 749ns | base |
| contract_sum_generic | 399ns | 336ns | 330ns | 367ns | 565ns | -4.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| contract_sum_concrete | 49ns | 42ns | 54ns | base | 5.271 |
| contract_sum_generic | 60ns | 47ns | 99ns | +23.09% | 4.282 |

## Performance model

- Peak throughput: **6.048 Gops/s** (contract_sum_concrete; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| contract_sum_concrete | 5.389 | 89.1% |
| contract_sum_generic | 5.384 | 89.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| contract_sum_concrete | 418ns | 418ns | base |
| contract_sum_generic | 399ns | 399ns | -4.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| contract_sum_concrete | 48ns | base | --- | [47, 49] | --- | --- | --- | --- |
| contract_sum_generic | 48ns | +4.9ns (+10.3%) | [+0, +7]ns | [47, 54] | YES | 0.0095 | 0.0095 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | contract_sum_concrete | contract_sum_generic |
|---|---|---|---|
| 1 | 1 | 47ns | +6.8% |
| 1 | 2 | 50ns | +27.0% |
| 1 | 3 | 54ns | -12.5% |
| 1 | 4 | 42ns | +12.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| contract_sum_concrete | -0.390 | moderate- |
| contract_sum_generic | -0.273 | moderate- |

**Consistency summary:**

- **contract_sum_generic**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| contract_sum_concrete | 118.4ns | 48.6ns | 243.9% | HIGH |
| contract_sum_generic | 120.2ns | 59.8ns | 201.1% | HIGH |

## Distribution (algo ns)

```
contract_sum_concrete (n=40, range 42.3-54.2 ns)
     42.3 |########
     42.9 |
     43.5 |####
     44.1 |
     44.7 |
     45.3 |
     45.9 |
     46.5 |################
     47.1 |########################################
     47.7 |
     48.3 |########
     48.8 |########
     49.4 |########
     50.0 |
     50.6 |
     51.2 |
     51.8 |
     52.4 |
     53.0 |####
     53.6 |############################
  (5 below, 4 above range)

contract_sum_generic (n=40, range 47.0-98.8 ns)
     47.0 |########################################
     49.6 |
     52.2 |#######
     54.8 |###
     57.4 |
     60.0 |###
     62.5 |###############
     65.1 |
     67.7 |
     70.3 |
     72.9 |
     75.5 |
     78.1 |
     80.7 |
     83.3 |
     85.9 |
     88.5 |
     91.1 |
     93.7 |
     96.2 |
  (2 below, 1 above range)

```

## Diagnostics

- **contract_sum_concrete**: bridge=218.9% of algo (FFI overhead may distort results)
- **contract_sum_generic**: CV=78.0% (high variance, measurements may be unstable)
- **contract_sum_generic**: bridge=212.4% of algo (FFI overhead may distort results)
