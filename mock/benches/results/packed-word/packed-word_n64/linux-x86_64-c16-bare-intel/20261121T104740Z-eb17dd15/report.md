# A packed word of named fields, through Word and by hand on the host integer

2 variants, 40 samples per variant.
Baseline: **packed_word_host**

## Highlights

Baseline for all deltas below: **packed_word_host**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (10 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (packed_word_host)

The baseline packed_word_host is the fastest (66 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### packed_word_word is inconsistent: worst-20% is 2.3x its best-20%

packed_word_word's best 20% of batches run at 60 ns but its worst 20% at 138 ns (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### packed_word_word's edge over baseline is significant but tiny (1 ns, 1.13%)

packed_word_word differs from baseline packed_word_host by 1 ns (1.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (packed_word_host) is the fastest** at 66.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 66.5 ns, slowest 67.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A packed word of named fields, through Word and by hand on the host integer |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x18ea09b1d1fc878d |
| Counter frequency | 2112871953 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| packed_word_host | 375ns | 320ns | 306ns | 341ns | 545ns | base |
| packed_word_word | 376ns | 321ns | 287ns | 347ns | 550ns | +0.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| packed_word_host | 71ns | 63ns | 87ns | base | 0.896 |
| packed_word_word | 81ns | 60ns | 138ns | +13.72% | 0.788 |

## Performance model

- Peak throughput: **1.064 Gops/s** (packed_word_word; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| packed_word_host | 0.963 | 90.5% |
| packed_word_word | 0.955 | 89.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| packed_word_host | 375ns | 375ns | base |
| packed_word_word | 376ns | 376ns | +0.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| packed_word_host | 66ns | base | --- | [66, 67] | --- | --- | --- | --- |
| packed_word_word | 67ns | +0.8ns (+1.1%) | [+0, +1]ns | [67, 75] | YES | 0.0139 | 0.0139 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | packed_word_host | packed_word_word |
|---|---|---|---|
| 1 | 1 | 85ns | +0.9% |
| 1 | 2 | 66ns | -9.3% |
| 1 | 3 | 65ns | +1.8% |
| 1 | 4 | 67ns | +6.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| packed_word_host | -0.058 | ok |
| packed_word_word | -0.319 | moderate- |

**Consistency summary:**

- **packed_word_word**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| packed_word_host | 111.6ns | 71.4ns | 156.4% | HIGH |
| packed_word_word | 97.4ns | 81.2ns | 120.0% | HIGH |

## Distribution (algo ns)

```
packed_word_host (n=40, range 62.8-87.3 ns)
     62.8 |
     64.0 |
     65.3 |########################################
     66.5 |#################
     67.7 |
     68.9 |
     70.2 |
     71.4 |##
     72.6 |
     73.8 |
     75.1 |#####
     76.3 |
     77.5 |
     78.7 |
     80.0 |
     81.2 |
     82.4 |
     83.6 |
     84.9 |######################
     86.1 |
  (4 below, 1 above range)

packed_word_word (n=40, range 60.2-137.5 ns)
     60.2 |##############
     64.0 |########################################
     67.9 |##
     71.8 |########
     75.6 |########
     79.5 |
     83.4 |######################
     87.2 |
     91.1 |
     95.0 |
     98.8 |
    102.7 |
    106.6 |
    110.4 |
    114.3 |
    118.2 |
    122.0 |
    125.9 |
    129.8 |
    133.6 |
  (5 below, 1 above range)

```

## Diagnostics

- **packed_word_host**: bridge=137.2% of algo (FFI overhead may distort results)
- **packed_word_word**: CV=82.8% (high variance, measurements may be unstable)
- **packed_word_word**: bridge=137.4% of algo (FFI overhead may distort results)
