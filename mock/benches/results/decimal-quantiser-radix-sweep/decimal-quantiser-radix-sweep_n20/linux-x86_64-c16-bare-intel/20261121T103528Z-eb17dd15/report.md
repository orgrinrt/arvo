# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-radix10 dominates: 71% faster than the next best (quantiser-radix2)

quantiser-radix10 (7.26 us) leads quantiser-radix2 (12.42 us) by 71%, a clear separation rather than a photo finish. CV 15.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-radix10)

The baseline quantiser-radix10 is the fastest (7.26 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### quantiser-radix2 is inconsistent: worst-20% is 1.8x its best-20%

quantiser-radix2's best 20% of batches run at 10.11 us but its worst 20% at 18.47 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-radix10) is the fastest** at 7261.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.71x (fastest 7261.9 ns, slowest 12417.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Quantiser cost by radix: decimal32 against binary32, exponent spread swept |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112757813 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 6910ns | 7330ns | 6002ns | 6761ns | 8267ns | base |
| quantiser-radix2 | 13148ns | 12501ns | 10184ns | 12330ns | 18565ns | +90.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 6837ns | 5946ns | 8160ns | base | 0.037 |
| quantiser-radix2 | 13070ns | 10114ns | 18469ns | +91.17% | 0.020 |

## Performance model

- Peak throughput: **0.043 Gops/s** (quantiser-radix10; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.035 | 81.9% |
| quantiser-radix2 | 0.021 | 47.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 6910ns | 6910ns | base |
| quantiser-radix2 | 13148ns | 13148ns | +90.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 7262ns | base | --- | [5988, 7292] | --- | --- | --- | --- |
| quantiser-radix2 | 12417ns | +5189.0ns (+71.5%) | [+5135, +5305]ns | [12382, 12478] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|---|
| 1 | 1 | 7302ns | +72.1% |
| 1 | 2 | 5951ns | +70.0% |
| 1 | 3 | 5983ns | +107.0% |
| 1 | 4 | 7292ns | +71.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | -0.250 | moderate- |
| quantiser-radix2 | -0.415 | moderate- |

**Consistency summary:**

- **quantiser-radix2**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 21.8ns | 6837.0ns | 0.3% |  |
| quantiser-radix2 | 22.9ns | 13070.3ns | 0.2% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 5945.6-8159.7 ns)
   5945.6 |########################################
   6056.3 |
   6167.0 |
   6277.7 |
   6388.4 |
   6499.1 |
   6609.8 |
   6720.6 |
   6831.3 |
   6942.0 |
   7052.7 |
   7163.4 |#####
   7274.1 |########################################
   7384.8 |##
   7495.5 |##
   7606.2 |
   7716.9 |
   7827.6 |
   7938.3 |
   8049.0 |
  (4 below, 2 above range)

quantiser-radix2 (n=40, range 10114.2-18469.4 ns)
  10114.2 |#########
  10531.9 |
  10949.7 |
  11367.5 |
  11785.2 |
  12203.0 |########################################
  12620.7 |####
  13038.5 |
  13456.3 |
  13874.0 |
  14291.8 |
  14709.5 |
  15127.3 |
  15545.1 |
  15962.8 |
  16380.6 |
  16798.4 |
  17216.1 |
  17633.9 |
  18051.6 |
  (4 below, 2 above range)

```

## Diagnostics

- **quantiser-radix2**: CV=40.9% (high variance, measurements may be unstable)
