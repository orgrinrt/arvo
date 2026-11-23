# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-acc64 dominates: 10% faster than the next best (warm-clamp-min-lanes)

warm-clamp-acc64 (2.18 us) leads warm-clamp-min-lanes (2.40 us) by 10%, a clear separation rather than a photo finish. CV 87.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-acc64 is fastest but the noisiest (CV 87.2%)

warm-clamp-acc64 wins on median (2.18 us) yet has the highest variance (CV 87.2%), while warm-clamp-minimum is the steadiest (CV 8.1%, 2.88 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.78)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.74 us) is smaller than the fastest variant's own run-to-run std-dev (1.90 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (2.18 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-clamp-acc64, warm-clamp-min-lanes, warm-clamp-head, warm-clamp-minimum} vs {warm-clamp-accfit-dyn, warm-clamp-accfit} (29% apart)

The field splits into a fast tier {warm-clamp-acc64, warm-clamp-min-lanes, warm-clamp-head, warm-clamp-minimum} and a slow tier {warm-clamp-accfit-dyn, warm-clamp-accfit} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-acc64 is inconsistent: worst-20% is 4.4x its best-20%

warm-clamp-acc64's best 20% of batches run at 1.28 us but its worst 20% at 5.60 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 2176.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.80x (fastest 2176.2 ns, slowest 3917.3 ns)

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
| warm-clamp-acc64 | 3365ns | 2269ns | 1333ns | 3204ns | 5882ns | base |
| warm-clamp-accfit | 3779ns | 4085ns | 1515ns | 3943ns | 5554ns | +12.30% |
| warm-clamp-accfit-dyn | 3643ns | 3862ns | 1484ns | 3667ns | 5729ns | +8.23% |
| warm-clamp-head | 4109ns | 2715ns | 2220ns | 3589ns | 7558ns | +22.10% |
| warm-clamp-min-lanes | 2714ns | 2622ns | 2249ns | 2678ns | 3286ns | -19.37% |
| warm-clamp-minimum | 3085ns | 3104ns | 2692ns | 3074ns | 3511ns | -8.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3216ns | 1277ns | 5599ns | base | 2.547 |
| warm-clamp-accfit | 3611ns | 1451ns | 5272ns | +12.28% | 2.269 |
| warm-clamp-accfit-dyn | 3485ns | 1423ns | 5473ns | +8.37% | 2.351 |
| warm-clamp-head | 3975ns | 2154ns | 7289ns | +23.61% | 2.061 |
| warm-clamp-min-lanes | 2507ns | 2086ns | 3024ns | -22.05% | 3.268 |
| warm-clamp-minimum | 2829ns | 2496ns | 3150ns | -12.03% | 2.895 |

## Performance model

- Peak throughput: **6.415 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.764 | 58.7% |
| warm-clamp-accfit | 2.091 | 32.6% |
| warm-clamp-accfit-dyn | 2.214 | 34.5% |
| warm-clamp-head | 3.111 | 48.5% |
| warm-clamp-min-lanes | 3.417 | 53.3% |
| warm-clamp-minimum | 2.847 | 44.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3365ns | 3365ns | base |
| warm-clamp-accfit | 3779ns | 3779ns | +12.30% |
| warm-clamp-accfit-dyn | 3643ns | 3643ns | +8.23% |
| warm-clamp-head | 4109ns | 4109ns | +22.10% |
| warm-clamp-min-lanes | 2714ns | 2714ns | -19.37% |
| warm-clamp-minimum | 3085ns | 3085ns | -8.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2176ns | base | --- | [1558, 5165] | --- | --- | --- | --- |
| warm-clamp-accfit | 3917ns | no significant difference | [-97, +324]ns | [3498, 4660] | no | 1.0000 | 0.6358 | 0 |
| warm-clamp-accfit-dyn | 3701ns | +206.4ns (+9.5%) | [+12, +396]ns | [3186, 4301] | YES | 0.0415 | 0.0166 | 0 |
| warm-clamp-head | 2633ns | +715.5ns (+32.9%) | [+656, +1380]ns | [2165, 4471] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2398ns | no significant difference | [-2357, +857]ns | [2256, 2661] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-minimum | 2878ns | no significant difference | [-1867, +1057]ns | [2752, 2878] | no | 1.0000 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2168ns | +93.7% | +70.7% | +66.6% | +21.7% | +25.9% |
| 1 | 2 | 2318ns | +57.9% | +61.4% | -6.6% | -4.5% | +13.8% |
| 1 | 3 | 2893ns | +62.1% | +13.4% | -25.3% | -0.5% | -0.5% |
| 1 | 4 | 2430ns | +80.2% | +30.6% | +8.9% | -8.9% | +19.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.105 | ok |
| warm-clamp-accfit | -0.313 | moderate- |
| warm-clamp-accfit-dyn | 0.268 | moderate+ |
| warm-clamp-head | -0.165 | ok |
| warm-clamp-min-lanes | -0.784 | HIGH- (thermal bounce) |
| warm-clamp-minimum | 0.125 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 0/4, lost 4/4
- **warm-clamp-accfit-dyn**: won 0/4, lost 4/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 51.4ns | 3216.0ns | 1.6% |  |
| warm-clamp-accfit | 60.0ns | 3611.0ns | 1.7% |  |
| warm-clamp-accfit-dyn | 55.1ns | 3485.1ns | 1.6% |  |
| warm-clamp-head | 36.4ns | 3975.5ns | 0.9% |  |
| warm-clamp-min-lanes | 74.6ns | 2506.9ns | 3.0% |  |
| warm-clamp-minimum | 86.0ns | 2829.3ns | 3.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1277.1-5598.9 ns)
   1277.1 |########################################
   1493.2 |###################################
   1709.2 |
   1925.3 |
   2141.4 |
   2357.5 |
   2573.6 |#####
   2789.7 |
   3005.8 |#####
   3221.9 |
   3438.0 |
   3654.1 |#####
   3870.1 |
   4086.2 |
   4302.3 |#####
   4518.4 |#####
   4734.5 |
   4950.6 |#####
   5166.7 |####################
   5382.8 |###################################
  (5 below, 3 above range)

warm-clamp-accfit (n=40, range 1451.4-5272.5 ns)
   1451.4 |########################################
   1642.5 |########
   1833.5 |
   2024.6 |
   2215.6 |
   2406.7 |########
   2597.7 |
   2788.8 |
   2979.8 |
   3170.9 |
   3361.9 |################################
   3553.0 |########################
   3744.1 |########
   3935.1 |################
   4126.2 |########
   4317.2 |########
   4508.3 |################################
   4699.3 |########################
   4890.4 |################
   5081.4 |################################
  (5 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1423.4-5473.3 ns)
   1423.4 |########################################
   1625.9 |
   1828.4 |####
   2030.9 |####
   2233.4 |
   2435.9 |
   2638.4 |
   2840.9 |####
   3043.4 |
   3245.9 |####
   3448.4 |#################
   3650.8 |########
   3853.3 |#############
   4055.8 |########
   4258.3 |########
   4460.8 |########
   4663.3 |########
   4865.8 |####
   5068.3 |
   5270.8 |####
  (2 below, 6 above range)

warm-clamp-head (n=40, range 2153.8-7289.2 ns)
   2153.8 |########################################
   2410.6 |#################
   2667.4 |
   2924.1 |
   3180.9 |
   3437.7 |
   3694.4 |########
   3951.2 |
   4208.0 |
   4464.7 |
   4721.5 |
   4978.3 |##
   5235.0 |
   5491.8 |
   5748.6 |##
   6005.3 |##
   6262.1 |##
   6518.9 |#####
   6775.6 |########
   7032.4 |
  (3 below, 5 above range)

warm-clamp-min-lanes (n=40, range 2085.8-3024.1 ns)
   2085.8 |
   2132.7 |######
   2179.6 |########################################
   2226.5 |#############
   2273.4 |
   2320.4 |######
   2367.3 |########################################
   2414.2 |######
   2461.1 |
   2508.0 |
   2554.9 |######
   2601.9 |#############
   2648.8 |######
   2695.7 |
   2742.6 |######
   2789.5 |######
   2836.4 |########################################
   2883.4 |
   2930.3 |######
   2977.2 |######
  (6 below, 3 above range)

warm-clamp-minimum (n=40, range 2495.7-3150.1 ns)
   2495.7 |###
   2528.4 |
   2561.1 |###
   2593.9 |######
   2626.6 |###
   2659.3 |######
   2692.0 |######
   2724.7 |###
   2757.5 |###
   2790.2 |
   2822.9 |
   2855.6 |########################################
   2888.3 |
   2921.0 |##################
   2953.8 |
   2986.5 |
   3019.2 |
   3051.9 |###
   3084.6 |
   3117.4 |###
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=59.0% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=40.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=43.3% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-head**: CV=54.1% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.4x (possible bimodal distribution)
