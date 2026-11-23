# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum dominates: 19% faster than the next best (warm-container-plusone)

warm-container-minimum (2.33 us) leads warm-container-plusone (2.78 us) by 19%, a clear separation rather than a photo finish. CV 120.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-minimum is fastest but the noisiest (CV 120.1%)

warm-container-minimum wins on median (2.33 us) yet has the highest variance (CV 120.1%), while warm-container-lanes-deferred is the steadiest (CV 29.0%, 3.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-native shows alternating (throttle bounce) (autocorr -0.73)

warm-container-native's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.18 us) is smaller than the fastest variant's own run-to-run std-dev (2.80 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-container-minimum is inconsistent: worst-20% is 3.9x its best-20%

warm-container-minimum's best 20% of batches run at 2.32 us but its worst 20% at 9.14 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-47 ns, 1.67%)

warm-container-plusone differs from baseline warm-container-headroom by -47 ns (1.67%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-minimum** at 2328.8 ns median (-17.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.51x (fastest 2328.8 ns, slowest 3507.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112763739 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4222ns | 2898ns | 2495ns | 3348ns | 8571ns | base |
| warm-container-kernel | 3358ns | 3496ns | 1531ns | 3506ns | 4742ns | -20.47% |
| warm-container-lanes-deferred | 3500ns | 3703ns | 1668ns | 3725ns | 4656ns | -17.11% |
| warm-container-minimum | 4276ns | 2380ns | 2368ns | 3205ns | 9397ns | +1.27% |
| warm-container-native | 4206ns | 2840ns | 2329ns | 3450ns | 8348ns | -0.39% |
| warm-container-plusone | 4308ns | 2839ns | 2326ns | 3438ns | 8900ns | +2.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4120ns | 2440ns | 8333ns | base | 3.977 |
| warm-container-kernel | 3175ns | 1452ns | 4463ns | -22.93% | 5.160 |
| warm-container-lanes-deferred | 3302ns | 1582ns | 4391ns | -19.85% | 4.961 |
| warm-container-minimum | 4172ns | 2316ns | 9137ns | +1.25% | 3.928 |
| warm-container-native | 4103ns | 2277ns | 8115ns | -0.42% | 3.994 |
| warm-container-plusone | 4203ns | 2274ns | 8653ns | +2.01% | 3.898 |

## Performance model

- Peak throughput: **11.286 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.779 | 51.2% |
| warm-container-kernel | 4.938 | 43.8% |
| warm-container-lanes-deferred | 4.671 | 41.4% |
| warm-container-minimum | 7.036 | 62.3% |
| warm-container-native | 5.901 | 52.3% |
| warm-container-plusone | 5.903 | 52.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4222ns | 4222ns | base |
| warm-container-kernel | 3358ns | 3358ns | -20.47% |
| warm-container-lanes-deferred | 3500ns | 3500ns | -17.11% |
| warm-container-minimum | 4276ns | 4276ns | +1.27% |
| warm-container-native | 4206ns | 4206ns | -0.39% |
| warm-container-plusone | 4308ns | 4308ns | +2.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2835ns | base | --- | [2832, 2852] | --- | --- | --- | --- |
| warm-container-kernel | 3318ns | no significant difference | [-1159, +297]ns | [3041, 3504] | no | 1.0000 | 1.0000 | 0 |
| warm-container-lanes-deferred | 3508ns | no significant difference | [-1678, +689]ns | [3214, 3776] | no | 0.7948 | 0.6358 | 0 |
| warm-container-minimum | 2329ns | -208.7ns (-7.4%) | [-511, -27]ns | [2319, 2870] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-container-native | 2776ns | -52.5ns (-1.9%) | [-76, -43]ns | [2774, 3254] | YES | 0.0017 | 0.0007 | 0 |
| warm-container-plusone | 2776ns | -47.5ns (-1.7%) | [-58, -40]ns | [2774, 2792] | YES | 0.0009 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2840ns | +31.9% | +28.2% | -18.2% | -2.1% | -2.1% |
| 1 | 2 | 2838ns | +16.9% | +46.3% | -18.3% | -18.4% | -1.7% |
| 1 | 3 | 2837ns | +12.6% | +16.3% | +1.1% | -2.0% | -2.0% |
| 1 | 4 | 2315ns | -15.9% | +43.6% | +0.1% | -1.7% | -1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.081 | ok |
| warm-container-kernel | 0.037 | ok |
| warm-container-lanes-deferred | -0.130 | ok |
| warm-container-minimum | -0.424 | moderate- |
| warm-container-native | -0.726 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.070 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-lanes-deferred**: won 0/4, lost 4/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 34.2ns | 4120.0ns | 0.8% |  |
| warm-container-kernel | 67.8ns | 3175.1ns | 2.1% |  |
| warm-container-lanes-deferred | 70.8ns | 3302.3ns | 2.1% |  |
| warm-container-minimum | 34.7ns | 4171.6ns | 0.8% |  |
| warm-container-native | 34.4ns | 4102.5ns | 0.8% |  |
| warm-container-plusone | 35.5ns | 4202.7ns | 0.8% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2439.9-8333.0 ns)
   2439.9 |
   2734.6 |########################################
   3029.2 |
   3323.9 |
   3618.5 |#
   3913.2 |
   4207.8 |
   4502.5 |
   4797.1 |
   5091.8 |
   5386.4 |#
   5681.1 |#
   5975.7 |
   6270.4 |
   6565.0 |#
   6859.7 |#
   7154.3 |#
   7449.0 |#
   7743.6 |
   8038.3 |
  (6 below, 5 above range)

warm-container-kernel (n=40, range 1451.8-4462.6 ns)
   1451.8 |
   1602.3 |
   1752.8 |
   1903.4 |
   2053.9 |
   2204.4 |######
   2355.0 |######
   2505.5 |
   2656.1 |######
   2806.6 |
   2957.2 |########################################
   3107.7 |#################################
   3258.2 |######
   3408.8 |########################################
   3559.3 |######
   3709.9 |#################################
   3860.4 |
   4010.9 |#############
   4161.5 |######
   4312.0 |######
  (6 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1581.6-4391.4 ns)
   1581.6 |######
   1722.1 |
   1862.6 |######
   2003.1 |######
   2143.6 |
   2284.1 |
   2424.6 |
   2565.0 |
   2705.5 |
   2846.0 |######
   2986.5 |####################
   3127.0 |#################################
   3267.5 |#############
   3408.0 |##########################
   3548.5 |######
   3688.9 |########################################
   3829.4 |######
   3969.9 |
   4110.4 |########################################
   4250.9 |######
  (4 below, 3 above range)

warm-container-minimum (n=40, range 2315.9-9137.0 ns)
   2315.9 |########################################
   2657.0 |#############
   2998.0 |##
   3339.1 |
   3680.1 |
   4021.2 |
   4362.2 |
   4703.3 |####
   5044.4 |
   5385.4 |
   5726.5 |
   6067.5 |
   6408.6 |
   6749.6 |
   7090.7 |
   7431.7 |####
   7772.8 |##
   8113.8 |##
   8454.9 |##
   8795.9 |##
  (3 below, 4 above range)

warm-container-native (n=40, range 2277.1-8115.3 ns)
   2277.1 |####################
   2569.0 |########################################
   2860.9 |
   3152.8 |##
   3444.7 |
   3736.6 |##
   4028.5 |
   4320.4 |
   4612.4 |##
   4904.3 |
   5196.2 |
   5488.1 |
   5780.0 |
   6071.9 |
   6363.8 |
   6655.7 |##
   6947.6 |#####
   7239.5 |##
   7531.5 |#####
   7823.4 |#####
  (5 below, 3 above range)

warm-container-plusone (n=40, range 2274.0-8652.7 ns)
   2274.0 |####
   2592.9 |########################################
   2911.8 |
   3230.8 |
   3549.7 |
   3868.6 |
   4187.6 |##
   4506.5 |
   4825.4 |
   5144.4 |
   5463.3 |##
   5782.2 |
   6101.2 |
   6420.1 |
   6739.1 |
   7058.0 |##
   7376.9 |
   7695.9 |####
   8014.8 |####
   8333.7 |####
  (6 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=56.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-kernel**: CV=32.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=30.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=67.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-native**: CV=57.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=60.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
