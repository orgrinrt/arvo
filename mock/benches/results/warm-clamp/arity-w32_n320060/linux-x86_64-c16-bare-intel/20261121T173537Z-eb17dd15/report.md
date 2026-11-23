# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum is fastest but the noisiest (CV 77.4%)

warm-clamp-minimum wins on median (2.46 us) yet has the highest variance (CV 77.4%), while warm-clamp-accfit-dyn is the steadiest (CV 7.6%, 2.81 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.74)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.69 us) is smaller than the fastest variant's own run-to-run std-dev (1.90 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head} vs {warm-clamp-min-lanes} (47% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head} and a slow tier {warm-clamp-min-lanes} with a 47% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-min-lanes is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.41 us but its worst 20% at 5.50 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 2461.6 ns median (-11.0% vs baseline)
- Spread: 1.69x (fastest 2461.6 ns, slowest 4149.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112820894 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2908ns | 2978ns | 2280ns | 2948ns | 3419ns | base |
| warm-clamp-accfit | 2832ns | 2943ns | 1269ns | 2921ns | 4129ns | -2.62% |
| warm-clamp-accfit-dyn | 3077ns | 3012ns | 2745ns | 3078ns | 3407ns | +5.80% |
| warm-clamp-head | 3052ns | 3110ns | 2538ns | 3040ns | 3599ns | +4.92% |
| warm-clamp-min-lanes | 3631ns | 4329ns | 1470ns | 3634ns | 5780ns | +24.83% |
| warm-clamp-minimum | 3621ns | 2555ns | 1814ns | 3137ns | 6882ns | +24.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2695ns | 2122ns | 3139ns | base | 3.040 |
| warm-clamp-accfit | 2625ns | 1181ns | 3804ns | -2.58% | 3.121 |
| warm-clamp-accfit-dyn | 2854ns | 2555ns | 3127ns | +5.90% | 2.871 |
| warm-clamp-head | 2756ns | 2298ns | 3222ns | +2.28% | 2.973 |
| warm-clamp-min-lanes | 3470ns | 1409ns | 5500ns | +28.77% | 2.361 |
| warm-clamp-minimum | 3487ns | 1753ns | 6607ns | +29.40% | 2.350 |

## Performance model

- Peak throughput: **6.938 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.960 | 42.7% |
| warm-clamp-accfit | 2.987 | 43.0% |
| warm-clamp-accfit-dyn | 2.920 | 42.1% |
| warm-clamp-head | 2.905 | 41.9% |
| warm-clamp-min-lanes | 1.974 | 28.5% |
| warm-clamp-minimum | 3.328 | 48.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 2908ns | 2908ns | base |
| warm-clamp-accfit | 2832ns | 2832ns | -2.62% |
| warm-clamp-accfit-dyn | 3077ns | 3077ns | +5.80% |
| warm-clamp-head | 3052ns | 3052ns | +4.92% |
| warm-clamp-min-lanes | 3631ns | 3631ns | +24.83% |
| warm-clamp-minimum | 3621ns | 3621ns | +24.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2767ns | base | --- | [2559, 2855] | --- | --- | --- | --- |
| warm-clamp-accfit | 2743ns | no significant difference | [-39, +141]ns | [2525, 3048] | no | 1.0000 | 0.8746 | 0 |
| warm-clamp-accfit-dyn | 2805ns | no significant difference | [-12, +256]ns | [2763, 3038] | no | 1.0000 | 0.4296 | 0 |
| warm-clamp-head | 2820ns | no significant difference | [-215, +348]ns | [2595, 2824] | no | 1.0000 | 0.6358 | 0 |
| warm-clamp-min-lanes | 4149ns | no significant difference | [-835, +2050]ns | [1515, 4826] | no | 0.7693 | 0.1539 | 0 |
| warm-clamp-minimum | 2462ns | no significant difference | [-438, +1150]ns | [2112, 3714] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2364ns | +0.1% | +28.5% | +21.5% | +75.5% | +22.3% |
| 1 | 2 | 3065ns | -72.1% | -9.2% | -23.3% | -8.3% | +65.7% |
| 1 | 3 | 2586ns | +17.9% | +8.0% | +9.1% | +45.0% | -4.8% |
| 1 | 4 | 2836ns | -2.3% | -1.8% | -8.5% | +58.1% | -25.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.661 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.298 | moderate- |
| warm-clamp-accfit-dyn | -0.103 | ok |
| warm-clamp-head | -0.737 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.211 | moderate- |
| warm-clamp-minimum | -0.204 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 1/4, lost 3/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 75.5ns | 2694.5ns | 2.8% |  |
| warm-clamp-accfit | 73.6ns | 2625.1ns | 2.8% |  |
| warm-clamp-accfit-dyn | 78.4ns | 2853.6ns | 2.7% |  |
| warm-clamp-head | 85.2ns | 2755.9ns | 3.1% |  |
| warm-clamp-min-lanes | 53.8ns | 3469.8ns | 1.6% |  |
| warm-clamp-minimum | 40.7ns | 3486.7ns | 1.2% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2122.2-3138.6 ns)
   2122.2 |
   2173.0 |##########
   2223.8 |
   2274.6 |
   2325.5 |#########################
   2376.3 |##########
   2427.1 |
   2477.9 |
   2528.8 |#########################
   2579.6 |###############
   2630.4 |
   2681.2 |
   2732.0 |#########################
   2782.9 |##########
   2833.7 |##########
   2884.5 |
   2935.3 |
   2986.1 |#####
   3037.0 |########################################
   3087.8 |##########
  (1 below, 2 above range)

warm-clamp-accfit (n=40, range 1180.8-3804.3 ns)
   1180.8 |
   1312.0 |
   1443.1 |
   1574.3 |
   1705.5 |
   1836.7 |
   1967.8 |
   2099.0 |#############
   2230.2 |########
   2361.4 |########
   2492.6 |##########################
   2623.7 |####
   2754.9 |#################
   2886.1 |####
   3017.3 |########################################
   3148.4 |####
   3279.6 |########
   3410.8 |
   3542.0 |
   3673.2 |
  (6 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 2555.2-3126.7 ns)
   2555.2 |####################
   2583.8 |
   2612.4 |#####
   2641.0 |#####
   2669.5 |#####
   2698.1 |
   2726.7 |
   2755.3 |##############################
   2783.8 |####################
   2812.4 |#####
   2841.0 |
   2869.6 |
   2898.1 |##########
   2926.7 |
   2955.3 |
   2983.9 |
   3012.4 |########################################
   3041.0 |#####
   3069.6 |###############
   3098.2 |##########
  (4 below, 2 above range)

warm-clamp-head (n=40, range 2298.2-3222.1 ns)
   2298.2 |
   2344.4 |##################
   2390.6 |###
   2436.7 |
   2482.9 |
   2529.1 |##################
   2575.3 |
   2621.5 |###
   2667.7 |
   2713.9 |
   2760.1 |###
   2806.3 |########################################
   2852.5 |##############
   2898.7 |###
   2944.9 |
   2991.1 |
   3037.3 |###
   3083.5 |
   3129.7 |##########
   3175.9 |###
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1408.8-5499.7 ns)
   1408.8 |########################################
   1613.3 |
   1817.9 |
   2022.4 |
   2227.0 |
   2431.5 |###
   2636.1 |
   2840.6 |
   3045.2 |
   3249.7 |###
   3454.3 |
   3658.8 |
   3863.3 |
   4067.9 |############
   4272.4 |###
   4477.0 |###
   4681.5 |######
   4886.1 |############
   5090.6 |
   5295.2 |########################
  (3 below, 2 above range)

warm-clamp-minimum (n=40, range 1752.8-6607.0 ns)
   1752.8 |#################
   1995.5 |########################################
   2238.2 |########################################
   2480.9 |#################
   2723.7 |
   2966.4 |
   3209.1 |#####
   3451.8 |
   3694.5 |
   3937.2 |#####
   4179.9 |#####
   4422.6 |
   4665.3 |
   4908.0 |###########
   5150.7 |
   5393.4 |
   5636.1 |#################
   5878.8 |
   6121.5 |#####
   6364.3 |###########
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-accfit**: CV=36.7% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=50.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=54.6% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
