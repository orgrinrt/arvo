# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-headroom is fastest but the noisiest (CV 97.2%)

warm-container-headroom wins on median (2.33 us) yet has the highest variance (CV 97.2%), while warm-container-minimum is the steadiest (CV 20.0%, 2.94 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-headroom, warm-container-plusone) are a dead heat (<1%)

warm-container-headroom (2.33 us) and warm-container-plusone (2.33 us) differ by 0.19%, inside the noise, even though the wider field spreads 51.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.67)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.19 us) is smaller than the fastest variant's own run-to-run std-dev (2.26 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-container-headroom)

The baseline warm-container-headroom is the fastest (2.33 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### warm-container-plusone is inconsistent: worst-20% is 3.7x its best-20%

warm-container-plusone's best 20% of batches run at 1.89 us but its worst 20% at 6.91 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-container-headroom) is the fastest** at 2325.8 ns median
- 1 variant significantly faster than baseline
- Spread: 1.51x (fastest 2325.8 ns, slowest 3510.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112697097 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4073ns | 2414ns | 2400ns | 3353ns | 7903ns | base |
| warm-container-kernel | 3104ns | 3169ns | 1803ns | 3226ns | 4039ns | -23.78% |
| warm-container-minimum | 3388ns | 3140ns | 2707ns | 3252ns | 4477ns | -16.80% |
| warm-container-native | 3414ns | 3738ns | 1564ns | 3704ns | 4396ns | -16.17% |
| warm-container-plusone | 3725ns | 2413ns | 1956ns | 3152ns | 7213ns | -8.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3922ns | 2319ns | 7577ns | base | 10.444 |
| warm-container-kernel | 2907ns | 1694ns | 3756ns | -25.86% | 14.088 |
| warm-container-minimum | 3169ns | 2547ns | 4162ns | -19.19% | 12.925 |
| warm-container-native | 3199ns | 1469ns | 4096ns | -18.43% | 12.805 |
| warm-container-plusone | 3587ns | 1890ns | 6909ns | -8.55% | 11.421 |

## Performance model

- Peak throughput: **27.888 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.611 | 63.1% |
| warm-container-kernel | 13.763 | 49.4% |
| warm-container-minimum | 13.950 | 50.0% |
| warm-container-native | 11.666 | 41.8% |
| warm-container-plusone | 17.578 | 63.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4073ns | 4073ns | base |
| warm-container-kernel | 3104ns | 3104ns | -23.78% |
| warm-container-minimum | 3388ns | 3388ns | -16.80% |
| warm-container-native | 3414ns | 3414ns | -16.17% |
| warm-container-plusone | 3725ns | 3725ns | -8.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2326ns | base | --- | [2319, 3565] | --- | --- | --- | --- |
| warm-container-kernel | 2976ns | no significant difference | [-1470, +468]ns | [2789, 3249] | no | 1.0000 | 0.8746 | 0 |
| warm-container-minimum | 2936ns | no significant difference | [-870, +465]ns | [2859, 3222] | no | 0.1614 | 0.0807 | 0 |
| warm-container-native | 3511ns | no significant difference | [-1358, +922]ns | [3445, 3571] | no | 1.0000 | 1.0000 | 0 |
| warm-container-plusone | 2330ns | -420.5ns (-18.1%) | [-436, -394]ns | [1924, 4030] | YES | 0.0089 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 2320ns | +31.4% | +32.2% | +52.5% | -16.5% |
| 1 | 2 | 2320ns | +51.6% | +75.4% | +54.4% | +1.5% |
| 1 | 3 | 2332ns | +10.3% | +19.2% | +50.6% | -17.7% |
| 1 | 4 | 3565ns | -8.9% | -17.9% | -5.3% | -16.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.076 | ok |
| warm-container-kernel | -0.671 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.363 | moderate- |
| warm-container-native | 0.103 | ok |
| warm-container-plusone | -0.412 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 1/4, lost 3/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 40.8ns | 3921.7ns | 1.0% |  |
| warm-container-kernel | 72.7ns | 2907.5ns | 2.5% |  |
| warm-container-minimum | 76.8ns | 3168.9ns | 2.4% |  |
| warm-container-native | 78.1ns | 3198.7ns | 2.4% |  |
| warm-container-plusone | 37.4ns | 3586.5ns | 1.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2318.6-7576.9 ns)
   2318.6 |########################################
   2581.5 |
   2844.4 |
   3107.3 |
   3370.3 |
   3633.2 |
   3896.1 |
   4159.0 |
   4421.9 |
   4684.9 |#
   4947.8 |
   5210.7 |#
   5473.6 |
   5736.5 |#
   5999.4 |
   6262.4 |#
   6525.3 |###
   6788.2 |#####
   7051.1 |
   7314.0 |#
  (2 below, 4 above range)

warm-container-kernel (n=40, range 1693.7-3756.3 ns)
   1693.7 |
   1796.8 |
   1899.9 |
   2003.1 |
   2106.2 |
   2209.3 |#####
   2312.5 |###########
   2415.6 |#####
   2518.7 |#################
   2621.9 |###########
   2725.0 |#####
   2828.1 |############################
   2931.3 |#################
   3034.4 |
   3137.5 |######################
   3240.7 |###########
   3343.8 |
   3446.9 |########################################
   3550.1 |###########
   3653.2 |
  (4 below, 3 above range)

warm-container-minimum (n=40, range 2546.9-4162.1 ns)
   2546.9 |
   2627.7 |#####
   2708.4 |############################
   2789.2 |###########
   2869.9 |########################################
   2950.7 |#####
   3031.5 |###########
   3112.2 |
   3193.0 |######################
   3273.7 |#####
   3354.5 |
   3435.3 |#####
   3516.0 |#####
   3596.8 |
   3677.5 |###########
   3758.3 |
   3839.0 |
   3919.8 |#####
   4000.6 |###########
   4081.3 |#####
  (6 below, 3 above range)

warm-container-native (n=40, range 1468.7-4096.1 ns)
   1468.7 |
   1600.1 |
   1731.5 |
   1862.8 |
   1994.2 |
   2125.6 |
   2256.9 |
   2388.3 |
   2519.7 |###
   2651.0 |
   2782.4 |
   2913.8 |###
   3045.2 |#######
   3176.5 |##########
   3307.9 |###
   3439.3 |########################################
   3570.6 |#####################
   3702.0 |
   3833.4 |##################
   3964.7 |#######
  (6 below, 2 above range)

warm-container-plusone (n=40, range 1890.1-6909.5 ns)
   1890.1 |########################################
   2141.1 |########################
   2392.1 |
   2643.0 |
   2894.0 |
   3145.0 |
   3395.9 |
   3646.9 |###
   3897.9 |###
   4148.8 |
   4399.8 |
   4650.8 |
   4901.7 |
   5152.7 |
   5403.7 |###
   5654.6 |###
   5905.6 |#########
   6156.6 |######
   6407.6 |#########
   6658.5 |###
  (4 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=57.7% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-container-kernel**: CV=27.1% (high variance, measurements may be unstable)
- **warm-container-native**: CV=30.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=59.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.7x (possible bimodal distribution)
