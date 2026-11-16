# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (quantiser-radix10) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline quantiser-radix10 has the worst median (4.62 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest quantiser-radix2 at 3.42 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### quantiser-radix2 dominates: 35% faster than the next best (quantiser-radix10)

quantiser-radix2 (3.42 us) leads quantiser-radix10 (4.62 us) by 35%, a clear separation rather than a photo finish. CV 93.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix2 is fastest but the noisiest (CV 93.7%)

quantiser-radix2 wins on median (3.42 us) yet has the highest variance (CV 93.7%), while quantiser-radix10 is the steadiest (CV 71.0%, 4.62 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### quantiser-radix10 shows alternating (throttle bounce) (autocorr -0.73)

quantiser-radix10's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.20 us) is smaller than the fastest variant's own run-to-run std-dev (3.20 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### quantiser-radix2 is inconsistent: worst-20% is 3.6x its best-20%

quantiser-radix2's best 20% of batches run at 3.11 us but its worst 20% at 11.23 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: quantiser-radix2** at 3421.8 ns median (-26.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.35x (fastest 3421.8 ns, slowest 4623.4 ns)

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
| Counter frequency | 2112854571 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 5813ns | 4689ns | 3792ns | 4541ns | 11650ns | base |
| quantiser-radix2 | 5317ns | 3483ns | 3166ns | 3973ns | 11500ns | -8.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 5711ns | 3738ns | 11387ns | base | 0.045 |
| quantiser-radix2 | 5210ns | 3110ns | 11226ns | -8.78% | 0.049 |

## Performance model

- Peak throughput: **0.082 Gops/s** (quantiser-radix2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.055 | 67.3% |
| quantiser-radix2 | 0.075 | 90.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 5813ns | 5813ns | base |
| quantiser-radix2 | 5317ns | 5317ns | -8.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 4623ns | base | --- | [4197, 4746] | --- | --- | --- | --- |
| quantiser-radix2 | 3422ns | -597.4ns (-12.9%) | [-1399, -236]ns | [3213, 3915] | YES | 0.0064 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|---|
| 1 | 1 | 3849ns | +1.7% |
| 1 | 2 | 4623ns | -32.5% |
| 1 | 3 | 3719ns | -14.3% |
| 1 | 4 | 4777ns | -32.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | -0.732 | HIGH- (thermal bounce) |
| quantiser-radix2 | -0.164 | ok |

**Consistency summary:**

- **quantiser-radix2**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 29.4ns | 5711.5ns | 0.5% |  |
| quantiser-radix2 | 32.0ns | 5210.2ns | 0.6% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 3737.5-11387.3 ns)
   3737.5 |##################
   4120.0 |##
   4502.5 |########################################
   4885.0 |
   5267.5 |##
   5650.0 |
   6032.5 |##
   6415.0 |
   6797.5 |
   7179.9 |
   7562.4 |
   7944.9 |
   8327.4 |
   8709.9 |
   9092.4 |
   9474.9 |
   9857.4 |
  10239.9 |
  10622.4 |
  11004.8 |##
  (6 below, 5 above range)

quantiser-radix2 (n=40, range 3110.4-11225.6 ns)
   3110.4 |########################################
   3516.1 |##############
   3921.9 |####
   4327.7 |##
   4733.4 |
   5139.2 |##
   5545.0 |##
   5950.7 |
   6356.5 |
   6762.2 |
   7168.0 |
   7573.8 |
   7979.5 |
   8385.3 |
   8791.0 |##
   9196.8 |
   9602.6 |##
  10008.3 |##
  10414.1 |####
  10819.9 |
  (3 below, 4 above range)

```

## Diagnostics

- **quantiser-radix10**: CV=57.5% (high variance, measurements may be unstable)
- **quantiser-radix10**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **quantiser-radix2**: CV=61.5% (high variance, measurements may be unstable)
- **quantiser-radix2**: worst_20/best_20 = 3.6x (possible bimodal distribution)
