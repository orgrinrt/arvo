# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-headroom dominates: 13% faster than the next best (warm-container-minimum)

warm-container-headroom (2.32 us) leads warm-container-minimum (2.61 us) by 13%, a clear separation rather than a photo finish. CV 106.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-headroom is fastest but the noisiest (CV 106.5%)

warm-container-headroom wins on median (2.32 us) yet has the highest variance (CV 106.5%), while warm-container-kernel is the steadiest (CV 24.2%, 3.04 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (989 ns) is smaller than the fastest variant's own run-to-run std-dev (2.47 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-container-headroom)

The baseline warm-container-headroom is the fastest (2.32 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### warm-container-plusone is inconsistent: worst-20% is 3.8x its best-20%

warm-container-plusone's best 20% of batches run at 2.31 us but its worst 20% at 8.69 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-container-headroom) is the fastest** at 2315.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.43x (fastest 2315.6 ns, slowest 3304.4 ns)

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
| Counter frequency | 2112735203 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4243ns | 2367ns | 2309ns | 3587ns | 8146ns | base |
| warm-container-kernel | 3208ns | 3199ns | 2115ns | 3268ns | 4120ns | -24.40% |
| warm-container-lanes-deferred | 3359ns | 3407ns | 2050ns | 3474ns | 4324ns | -20.83% |
| warm-container-minimum | 4135ns | 2672ns | 2181ns | 3458ns | 8121ns | -2.54% |
| warm-container-native | 4529ns | 3383ns | 2201ns | 4335ns | 7441ns | +6.74% |
| warm-container-plusone | 4295ns | 2895ns | 2363ns | 3392ns | 8936ns | +1.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4138ns | 2258ns | 7909ns | base | 3.959 |
| warm-container-kernel | 3032ns | 2005ns | 3872ns | -26.73% | 5.404 |
| warm-container-lanes-deferred | 3175ns | 1944ns | 4071ns | -23.27% | 5.160 |
| warm-container-minimum | 4027ns | 2129ns | 7873ns | -2.69% | 4.069 |
| warm-container-native | 4413ns | 2150ns | 7227ns | +6.65% | 3.713 |
| warm-container-plusone | 4190ns | 2311ns | 8687ns | +1.25% | 3.911 |

## Performance model

- Peak throughput: **8.427 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.075 | 84.0% |
| warm-container-kernel | 5.398 | 64.1% |
| warm-container-lanes-deferred | 5.069 | 60.2% |
| warm-container-minimum | 6.280 | 74.5% |
| warm-container-native | 4.958 | 58.8% |
| warm-container-plusone | 5.784 | 68.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4243ns | 4243ns | base |
| warm-container-kernel | 3208ns | 3208ns | -24.40% |
| warm-container-lanes-deferred | 3359ns | 3359ns | -20.83% |
| warm-container-minimum | 4135ns | 4135ns | -2.54% |
| warm-container-native | 4529ns | 4529ns | +6.74% |
| warm-container-plusone | 4295ns | 4295ns | +1.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2316ns | base | --- | [2282, 5404] | --- | --- | --- | --- |
| warm-container-kernel | 3035ns | no significant difference | [-1958, +568]ns | [2964, 3214] | no | 0.8746 | 0.8746 | 0 |
| warm-container-lanes-deferred | 3232ns | no significant difference | [-2018, +675]ns | [3024, 3462] | no | 0.8746 | 0.8746 | 0 |
| warm-container-minimum | 2609ns | no significant difference | [-146, +296]ns | [2595, 4399] | no | 0.8746 | 0.8746 | 0 |
| warm-container-native | 3304ns | no significant difference | [-120, +331]ns | [2641, 6252] | no | 0.8746 | 0.6358 | 0 |
| warm-container-plusone | 2833ns | +224.6ns (+9.7%) | [+41, +545]ns | [2822, 2840] | YES | 0.0009 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2308ns | +58.1% | +30.4% | +14.9% | +56.5% | +0.5% |
| 1 | 2 | 2265ns | +31.0% | +68.6% | +58.4% | +134.0% | +25.2% |
| 1 | 3 | 2322ns | +37.2% | +52.5% | +12.0% | +23.6% | +22.5% |
| 1 | 4 | 2337ns | +27.9% | +32.2% | -8.4% | +21.7% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.066 | ok |
| warm-container-kernel | -0.327 | moderate- |
| warm-container-lanes-deferred | -0.296 | moderate- |
| warm-container-minimum | -0.102 | ok |
| warm-container-native | -0.182 | ok |
| warm-container-plusone | -0.250 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 0/4, lost 4/4
- **warm-container-lanes-deferred**: won 0/4, lost 4/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 0/4, lost 4/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 35.9ns | 4138.1ns | 0.9% |  |
| warm-container-kernel | 65.5ns | 3031.9ns | 2.2% |  |
| warm-container-lanes-deferred | 68.0ns | 3175.2ns | 2.1% |  |
| warm-container-minimum | 37.4ns | 4026.7ns | 0.9% |  |
| warm-container-native | 40.8ns | 4413.2ns | 0.9% |  |
| warm-container-plusone | 35.8ns | 4189.6ns | 0.9% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2257.7-7909.1 ns)
   2257.7 |########################################
   2540.3 |
   2822.8 |
   3105.4 |
   3388.0 |
   3670.5 |
   3953.1 |
   4235.7 |#
   4518.2 |
   4800.8 |
   5083.4 |
   5366.0 |
   5648.5 |
   5931.1 |
   6213.7 |###
   6496.2 |#
   6778.8 |###
   7061.4 |#
   7343.9 |#####
   7626.5 |###
  (3 below, 3 above range)

warm-container-kernel (n=40, range 2004.6-3871.7 ns)
   2004.6 |
   2097.9 |
   2191.3 |#####
   2284.7 |
   2378.0 |
   2471.4 |
   2564.7 |
   2658.1 |#####
   2751.4 |##################################
   2844.8 |###########
   2938.1 |########################################
   3031.5 |#####
   3124.8 |############################
   3218.2 |#####
   3311.6 |###########
   3404.9 |######################
   3498.3 |#####
   3591.6 |#####
   3685.0 |#####
   3778.3 |#####
  (3 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1944.2-4071.3 ns)
   1944.2 |
   2050.6 |
   2156.9 |
   2263.3 |
   2369.6 |
   2476.0 |
   2582.4 |
   2688.7 |####################
   2795.1 |######
   2901.4 |########################################
   3007.8 |######
   3114.1 |####################
   3220.5 |####################
   3326.8 |#############
   3433.2 |#################################
   3539.6 |#############
   3645.9 |
   3752.3 |#############
   3858.6 |####################
   3965.0 |
  (4 below, 5 above range)

warm-container-minimum (n=40, range 2129.3-7872.8 ns)
   2129.3 |####################
   2416.4 |########################################
   2703.6 |
   2990.8 |
   3278.0 |
   3565.1 |
   3852.3 |##
   4139.5 |
   4426.7 |
   4713.9 |#####
   5001.0 |
   5288.2 |
   5575.4 |
   5862.6 |########
   6149.7 |#####
   6436.9 |##
   6724.1 |##
   7011.3 |##
   7298.4 |
   7585.6 |
  (4 below, 4 above range)

warm-container-native (n=40, range 2149.8-7227.0 ns)
   2149.8 |##############################
   2403.7 |########################################
   2657.5 |#####
   2911.4 |#####
   3165.2 |
   3419.1 |#####
   3672.9 |#####
   3926.8 |
   4180.7 |
   4434.5 |
   4688.4 |
   4942.2 |#####
   5196.1 |
   5449.9 |
   5703.8 |#####
   5957.7 |#####
   6211.5 |##########
   6465.4 |###############
   6719.2 |####################
   6973.1 |###############
  (4 below, 3 above range)

warm-container-plusone (n=40, range 2311.2-8687.2 ns)
   2311.2 |####################
   2630.0 |########################################
   2948.8 |
   3267.6 |
   3586.4 |
   3905.2 |
   4224.0 |
   4542.8 |
   4861.6 |##
   5180.4 |
   5499.2 |
   5818.0 |##
   6136.8 |
   6455.6 |
   6774.4 |##
   7093.2 |#####
   7412.0 |
   7730.8 |#####
   8049.6 |
   8368.4 |##
  (4 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=59.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-kernel**: CV=24.3% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: CV=25.6% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=57.6% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-native**: CV=48.6% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-plusone**: CV=61.0% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
