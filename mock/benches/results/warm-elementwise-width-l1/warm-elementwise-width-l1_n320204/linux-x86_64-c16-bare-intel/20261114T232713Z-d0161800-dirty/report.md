# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-container-headroom, warm-container-plusone) are a dead heat (<1%)

warm-container-headroom (2.31 us) and warm-container-plusone (2.33 us) differ by 0.81%, inside the noise, even though the wider field spreads 50.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.67)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.17 us) is smaller than the fastest variant's own run-to-run std-dev (2.24 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-container-headroom)

The baseline warm-container-headroom is the fastest (2.31 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-container-headroom, warm-container-plusone} vs {warm-container-native, warm-container-minimum, warm-container-kernel} (37% apart)

The field splits into a fast tier {warm-container-headroom, warm-container-plusone} and a slow tier {warm-container-native, warm-container-minimum, warm-container-kernel} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-headroom is inconsistent: worst-20% is 3.7x its best-20%

warm-container-headroom's best 20% of batches run at 2.04 us but its worst 20% at 7.55 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (12 ns, 0.53%)

warm-container-plusone differs from baseline warm-container-headroom by 12 ns (0.53%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (warm-container-headroom) is the fastest** at 2309.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.51x (fastest 2309.9 ns, slowest 3483.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3846ns | 2392ns | 2117ns | 3084ns | 7860ns | base |
| warm-container-kernel | 3466ns | 3710ns | 2185ns | 3625ns | 4271ns | -9.87% |
| warm-container-minimum | 3427ns | 3703ns | 2076ns | 3579ns | 4323ns | -10.89% |
| warm-container-native | 3364ns | 3391ns | 2487ns | 3417ns | 4083ns | -12.53% |
| warm-container-plusone | 4306ns | 2410ns | 2391ns | 3609ns | 8311ns | +11.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3700ns | 2044ns | 7546ns | base | 11.069 |
| warm-container-kernel | 3240ns | 2048ns | 3934ns | -12.46% | 12.644 |
| warm-container-minimum | 3205ns | 1943ns | 4014ns | -13.40% | 12.781 |
| warm-container-native | 3148ns | 2336ns | 3787ns | -14.93% | 13.011 |
| warm-container-plusone | 4149ns | 2310ns | 7980ns | +12.12% | 9.872 |

## Performance model

- Peak throughput: **21.076 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.732 | 84.1% |
| warm-container-kernel | 11.760 | 55.8% |
| warm-container-minimum | 11.775 | 55.9% |
| warm-container-native | 12.864 | 61.0% |
| warm-container-plusone | 17.590 | 83.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3846ns | 3846ns | base |
| warm-container-kernel | 3466ns | 3466ns | -9.87% |
| warm-container-minimum | 3427ns | 3427ns | -10.89% |
| warm-container-native | 3364ns | 3364ns | -12.53% |
| warm-container-plusone | 4306ns | 4306ns | +11.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2310ns | base | --- | [2309, 2466] | --- | --- | --- | --- |
| warm-container-kernel | 3483ns | no significant difference | [-786, +1044]ns | [3248, 3524] | no | 0.4296 | 0.4296 | 0 |
| warm-container-minimum | 3478ns | no significant difference | [-1338, +895]ns | [3182, 3480] | no | 0.4296 | 0.4296 | 0 |
| warm-container-native | 3184ns | no significant difference | [-228, +804]ns | [2999, 3479] | no | 0.1614 | 0.0807 | 0 |
| warm-container-plusone | 2329ns | +12.3ns (+0.5%) | [+0, +130]ns | [2310, 4315] | YES | 0.0027 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 2310ns | +56.2% | +46.5% | +42.6% | +0.3% |
| 1 | 2 | 2241ns | +43.0% | +53.1% | +13.7% | +3.9% |
| 1 | 3 | 3233ns | +7.9% | +1.1% | +7.6% | -4.4% |
| 1 | 4 | 2299ns | +44.4% | +51.3% | +46.4% | +1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.439 | moderate- |
| warm-container-kernel | -0.675 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.654 | HIGH- (thermal bounce) |
| warm-container-native | -0.388 | moderate- |
| warm-container-plusone | -0.396 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 0/4, lost 4/4
- **warm-container-minimum**: won 0/4, lost 4/4
- **warm-container-native**: won 0/4, lost 4/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 39.6ns | 3700.4ns | 1.1% |  |
| warm-container-kernel | 81.8ns | 3239.5ns | 2.5% |  |
| warm-container-minimum | 82.6ns | 3204.6ns | 2.6% |  |
| warm-container-native | 90.2ns | 3148.0ns | 2.9% |  |
| warm-container-plusone | 44.4ns | 4149.1ns | 1.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2044.3-7545.7 ns)
   2044.3 |########################################
   2319.4 |######
   2594.4 |
   2869.5 |
   3144.6 |
   3419.6 |
   3694.7 |
   3969.8 |####
   4244.9 |
   4519.9 |
   4795.0 |
   5070.1 |
   5345.2 |
   5620.2 |
   5895.3 |##
   6170.4 |####
   6445.4 |##
   6720.5 |##
   6995.6 |##
   7270.7 |##
  (5 below, 4 above range)

warm-container-kernel (n=40, range 2048.2-3934.2 ns)
   2048.2 |
   2142.5 |
   2236.8 |
   2331.1 |
   2425.4 |
   2519.7 |
   2614.0 |
   2708.3 |
   2802.6 |
   2896.9 |##############
   2991.2 |
   3085.5 |###########
   3179.8 |#####
   3274.1 |
   3368.4 |#####
   3462.7 |########################################
   3557.0 |
   3651.3 |#####
   3745.6 |#####
   3839.9 |########
  (4 below, 2 above range)

warm-container-minimum (n=40, range 1943.4-4013.9 ns)
   1943.4 |
   2047.0 |
   2150.5 |
   2254.0 |
   2357.5 |
   2461.0 |
   2564.6 |
   2668.1 |
   2771.6 |
   2875.1 |####################
   2978.6 |###############
   3082.2 |###############
   3185.7 |###############
   3289.2 |##########
   3392.7 |########################################
   3496.3 |#########################
   3599.8 |#####
   3703.3 |#####
   3806.8 |#####
   3910.3 |##########
  (4 below, 3 above range)

warm-container-native (n=40, range 2336.1-3787.4 ns)
   2336.1 |
   2408.7 |
   2481.3 |#################
   2553.8 |#####
   2626.4 |###########
   2699.0 |
   2771.5 |
   2844.1 |###########
   2916.7 |###########
   2989.2 |###########
   3061.8 |###########
   3134.3 |######################
   3206.9 |###########
   3279.5 |###########
   3352.0 |
   3424.6 |######################
   3497.2 |########################################
   3569.7 |
   3642.3 |#####
   3714.9 |
  (3 below, 3 above range)

warm-container-plusone (n=40, range 2309.6-7980.0 ns)
   2309.6 |########################################
   2593.2 |
   2876.7 |
   3160.2 |
   3443.7 |
   3727.2 |#
   4010.8 |#
   4294.3 |#
   4577.8 |
   4861.3 |
   5144.8 |
   5428.3 |
   5711.9 |
   5995.4 |
   6278.9 |#
   6562.4 |###
   6845.9 |#
   7129.5 |###
   7413.0 |#
   7696.5 |###
  (3 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=60.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-kernel**: CV=23.6% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=25.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=58.5% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.5x (possible bimodal distribution)
