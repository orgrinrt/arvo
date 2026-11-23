# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum dominates: 26% faster than the next best (warm-container-native)

warm-container-minimum (2.19 us) leads warm-container-native (2.76 us) by 26%, a clear separation rather than a photo finish. CV 121.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-minimum is fastest but the noisiest (CV 121.1%)

warm-container-minimum wins on median (2.19 us) yet has the highest variance (CV 121.1%), while warm-container-lanes-deferred is the steadiest (CV 32.1%, 3.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.53 us) is smaller than the fastest variant's own run-to-run std-dev (2.65 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-container-kernel is inconsistent: worst-20% is 4.4x its best-20%

warm-container-kernel's best 20% of batches run at 1.04 us but its worst 20% at 4.53 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-native's edge over baseline is significant but tiny (-36 ns, 1.28%)

warm-container-native differs from baseline warm-container-headroom by -36 ns (1.28%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-minimum** at 2189.8 ns median (-21.0% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.70x (fastest 2189.8 ns, slowest 3723.3 ns)

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
| warm-container-headroom | 4163ns | 2836ns | 2453ns | 3146ns | 8924ns | base |
| warm-container-kernel | 3301ns | 3929ns | 1092ns | 3537ns | 4802ns | -20.71% |
| warm-container-lanes-deferred | 3437ns | 3437ns | 1659ns | 3567ns | 4826ns | -17.44% |
| warm-container-minimum | 3926ns | 2242ns | 2202ns | 2837ns | 8917ns | -5.70% |
| warm-container-native | 3900ns | 2824ns | 2331ns | 2903ns | 8459ns | -6.33% |
| warm-container-plusone | 4313ns | 2898ns | 2464ns | 3301ns | 9200ns | +3.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4059ns | 2398ns | 8667ns | base | 4.037 |
| warm-container-kernel | 3132ns | 1040ns | 4526ns | -22.84% | 5.232 |
| warm-container-lanes-deferred | 3264ns | 1579ns | 4573ns | -19.59% | 5.020 |
| warm-container-minimum | 3823ns | 2151ns | 8651ns | -5.82% | 4.286 |
| warm-container-native | 3801ns | 2278ns | 8216ns | -6.35% | 4.310 |
| warm-container-plusone | 4208ns | 2410ns | 8944ns | +3.67% | 3.894 |

## Performance model

- Peak throughput: **15.759 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.908 | 37.5% |
| warm-container-kernel | 4.400 | 27.9% |
| warm-container-lanes-deferred | 5.012 | 31.8% |
| warm-container-minimum | 7.482 | 47.5% |
| warm-container-native | 5.932 | 37.6% |
| warm-container-plusone | 5.779 | 36.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4163ns | 4163ns | base |
| warm-container-kernel | 3301ns | 3301ns | -20.71% |
| warm-container-lanes-deferred | 3437ns | 3437ns | -17.44% |
| warm-container-minimum | 3926ns | 3926ns | -5.70% |
| warm-container-native | 3900ns | 3900ns | -6.33% |
| warm-container-plusone | 4313ns | 4313ns | +3.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2773ns | base | --- | [2765, 2796] | --- | --- | --- | --- |
| warm-container-kernel | 3723ns | no significant difference | [-1640, +554]ns | [3192, 4038] | no | 0.6358 | 0.6358 | 0 |
| warm-container-lanes-deferred | 3269ns | no significant difference | [-1043, +340]ns | [3010, 3729] | no | 0.5370 | 0.4296 | 0 |
| warm-container-minimum | 2190ns | -170.9ns (-6.2%) | [-585, -119]ns | [2180, 2641] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2762ns | -35.5ns (-1.3%) | [-262, -10]ns | [2749, 2806] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-container-plusone | 2835ns | +39.9ns (+1.4%) | [+9, +91]ns | [2787, 2840] | YES | 0.0415 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2307ns | +64.8% | +84.7% | -5.7% | +22.2% | +22.9% |
| 1 | 2 | 2785ns | +19.9% | +6.9% | -21.5% | -1.2% | +1.7% |
| 1 | 3 | 2772ns | +47.4% | +22.3% | -4.7% | -0.6% | -16.2% |
| 1 | 4 | 2773ns | +39.3% | +48.1% | -21.4% | -17.5% | +2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.104 | ok |
| warm-container-kernel | -0.410 | moderate- |
| warm-container-lanes-deferred | -0.296 | moderate- |
| warm-container-minimum | -0.406 | moderate- |
| warm-container-native | -0.061 | ok |
| warm-container-plusone | -0.415 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 0/4, lost 4/4
- **warm-container-lanes-deferred**: won 0/4, lost 4/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 34.3ns | 4058.8ns | 0.8% |  |
| warm-container-kernel | 60.8ns | 3131.6ns | 1.9% |  |
| warm-container-lanes-deferred | 63.3ns | 3263.9ns | 1.9% |  |
| warm-container-minimum | 34.2ns | 3822.8ns | 0.9% |  |
| warm-container-native | 32.2ns | 3801.1ns | 0.8% |  |
| warm-container-plusone | 35.0ns | 4208.0ns | 0.8% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2398.1-8666.6 ns)
   2398.1 |
   2711.5 |########################################
   3025.0 |
   3338.4 |
   3651.8 |
   3965.2 |
   4278.7 |
   4592.1 |
   4905.5 |
   5218.9 |
   5532.3 |
   5845.8 |#
   6159.2 |
   6472.6 |#
   6786.0 |#
   7099.4 |#
   7412.9 |###
   7726.3 |
   8039.7 |
   8353.1 |
  (6 below, 4 above range)

warm-container-kernel (n=40, range 1039.7-4526.0 ns)
   1039.7 |######
   1214.0 |#############
   1388.3 |
   1562.6 |######
   1736.9 |
   1911.3 |######
   2085.6 |
   2259.9 |
   2434.2 |
   2608.5 |
   2782.8 |
   2957.2 |######
   3131.5 |####################
   3305.8 |#############
   3480.1 |######
   3654.4 |#################################
   3828.7 |######
   4003.1 |########################################
   4177.4 |######
   4351.7 |########################################
  (7 below, 2 above range)

warm-container-lanes-deferred (n=40, range 1579.3-4573.1 ns)
   1579.3 |######
   1729.0 |
   1878.7 |
   2028.4 |
   2178.1 |######
   2327.8 |######
   2477.4 |
   2627.1 |
   2776.8 |##########################
   2926.5 |##########################
   3076.2 |##########################
   3225.9 |######
   3375.6 |####################
   3525.3 |######
   3675.0 |#############
   3824.7 |######
   3974.4 |####################
   4124.1 |
   4273.7 |######
   4423.4 |########################################
  (5 below, 2 above range)

warm-container-minimum (n=40, range 2150.6-8650.7 ns)
   2150.6 |########################################
   2475.6 |##################
   2800.6 |##
   3125.6 |
   3450.6 |
   3775.6 |
   4100.6 |
   4425.6 |
   4750.6 |
   5075.6 |
   5400.6 |
   5725.6 |
   6050.6 |
   6375.6 |
   6700.6 |##
   7025.6 |##
   7350.6 |##
   7675.6 |
   8000.6 |####
   8325.6 |##
  (4 below, 4 above range)

warm-container-native (n=40, range 2277.7-8216.1 ns)
   2277.7 |########
   2574.6 |########################################
   2871.5 |
   3168.4 |
   3465.3 |
   3762.3 |
   4059.2 |#
   4356.1 |
   4653.0 |#
   4950.0 |
   5246.9 |
   5543.8 |
   5840.7 |
   6137.6 |
   6434.6 |#
   6731.5 |
   7028.4 |
   7325.3 |#
   7622.2 |
   7919.2 |
  (3 below, 5 above range)

warm-container-plusone (n=40, range 2409.6-8943.8 ns)
   2409.6 |
   2736.3 |########################################
   3063.0 |#
   3389.7 |
   3716.4 |
   4043.2 |
   4369.9 |
   4696.6 |
   5023.3 |
   5350.0 |###
   5676.7 |
   6003.4 |
   6330.1 |
   6656.8 |#
   6983.5 |#
   7310.2 |#
   7636.9 |#
   7963.6 |
   8290.3 |#
   8617.0 |
  (6 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=62.0% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-kernel**: CV=42.5% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=32.1% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=69.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-native**: CV=62.3% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=61.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.7x (possible bimodal distribution)
