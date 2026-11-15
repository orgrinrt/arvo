# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 11% faster than the next best (warm-container-lanes-deferred)

warm-container-kernel (2.66 us) leads warm-container-lanes-deferred (2.95 us) by 11%, a clear separation rather than a photo finish. CV 13.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 43% (significant)

warm-container-kernel is -1.97 us (43%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-native shows alternating (throttle bounce) (autocorr -0.52)

warm-container-native's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-lanes-deferred, warm-container-native, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (47% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-lanes-deferred, warm-container-native, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 47% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-plusone is inconsistent: worst-20% is 3.8x its best-20%

warm-container-plusone's best 20% of batches run at 1.58 us but its worst 20% at 5.96 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 2663.5 ns median (-42.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.77x (fastest 2663.5 ns, slowest 4703.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4172ns | 4832ns | 1652ns | 4348ns | 6163ns | base |
| warm-container-kernel | 2831ns | 2839ns | 2348ns | 2813ns | 3366ns | -32.15% |
| warm-container-lanes-deferred | 3217ns | 3172ns | 1966ns | 3258ns | 4343ns | -22.89% |
| warm-container-minimum | 3237ns | 3332ns | 2305ns | 3316ns | 3932ns | -22.40% |
| warm-container-native | 3252ns | 3299ns | 2251ns | 3324ns | 4036ns | -22.05% |
| warm-container-plusone | 4081ns | 4945ns | 1651ns | 4152ns | 6298ns | -2.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3965ns | 1576ns | 5821ns | base | 8.265 |
| warm-container-kernel | 2644ns | 2205ns | 3132ns | -33.30% | 12.392 |
| warm-container-lanes-deferred | 3007ns | 1846ns | 4045ns | -24.15% | 10.897 |
| warm-container-minimum | 3030ns | 2165ns | 3650ns | -23.58% | 10.815 |
| warm-container-native | 3041ns | 2114ns | 3740ns | -23.30% | 10.776 |
| warm-container-plusone | 3882ns | 1576ns | 5956ns | -2.09% | 8.442 |

## Performance model

- Peak throughput: **20.793 Gops/s** (warm-container-plusone; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.139 | 34.3% |
| warm-container-kernel | 12.303 | 59.2% |
| warm-container-lanes-deferred | 11.108 | 53.4% |
| warm-container-minimum | 10.459 | 50.3% |
| warm-container-native | 10.584 | 50.9% |
| warm-container-plusone | 6.967 | 33.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4172ns | 4172ns | base |
| warm-container-kernel | 2831ns | 2831ns | -32.15% |
| warm-container-lanes-deferred | 3217ns | 3217ns | -22.89% |
| warm-container-minimum | 3237ns | 3237ns | -22.40% |
| warm-container-native | 3252ns | 3252ns | -22.05% |
| warm-container-plusone | 4081ns | 4081ns | -2.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4590ns | base | --- | [3594, 4946] | --- | --- | --- | --- |
| warm-container-kernel | 2664ns | -1970.1ns (-42.9%) | [-2269, -1175]ns | [2488, 2696] | YES | 0.0107 | 0.0064 | 0 |
| warm-container-lanes-deferred | 2950ns | -1234.7ns (-26.9%) | [-1799, -596]ns | [2882, 3212] | YES | 0.0107 | 0.0064 | 0 |
| warm-container-minimum | 3133ns | -1516.8ns (-33.0%) | [-1760, -988]ns | [2962, 3222] | YES | 0.0107 | 0.0064 | 0 |
| warm-container-native | 3096ns | -1339.7ns (-29.2%) | [-1771, -652]ns | [2859, 3448] | YES | 0.0207 | 0.0166 | 0 |
| warm-container-plusone | 4704ns | no significant difference | [-158, +286]ns | [1738, 5200] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4457ns | -44.2% | -15.5% | -38.5% | -40.5% | -22.3% |
| 1 | 2 | 4427ns | -43.4% | -27.9% | -25.4% | -20.4% | +6.3% |
| 1 | 3 | 3741ns | -19.4% | -15.4% | -14.0% | -18.7% | +29.0% |
| 1 | 4 | 5097ns | -47.5% | -47.5% | -37.6% | -30.6% | -8.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.497 | moderate- |
| warm-container-kernel | -0.134 | ok |
| warm-container-lanes-deferred | 0.023 | ok |
| warm-container-minimum | -0.227 | moderate- |
| warm-container-native | -0.518 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.046 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 56.5ns | 3964.6ns | 1.4% |  |
| warm-container-kernel | 68.2ns | 2644.3ns | 2.6% |  |
| warm-container-lanes-deferred | 76.5ns | 3006.9ns | 2.5% |  |
| warm-container-minimum | 74.4ns | 3029.8ns | 2.5% |  |
| warm-container-native | 74.9ns | 3040.8ns | 2.5% |  |
| warm-container-plusone | 56.4ns | 3881.7ns | 1.5% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1576.0-5820.6 ns)
   1576.0 |########################################
   1788.2 |######
   2000.5 |
   2212.7 |######
   2424.9 |######
   2637.1 |
   2849.4 |######
   3061.6 |
   3273.8 |
   3486.1 |
   3698.3 |
   3910.5 |
   4122.8 |#############
   4335.0 |##########################
   4547.2 |######
   4759.4 |########################################
   4971.7 |######
   5183.9 |##########################
   5396.1 |####################
   5608.4 |#############
  (4 below, 3 above range)

warm-container-kernel (n=40, range 2204.9-3132.3 ns)
   2204.9 |
   2251.3 |
   2297.6 |####################
   2344.0 |###############
   2390.4 |#####
   2436.7 |#####
   2483.1 |########################################
   2529.5 |
   2575.8 |
   2622.2 |##########
   2668.6 |###################################
   2714.9 |#####
   2761.3 |#####
   2807.7 |#####
   2854.1 |#####
   2900.4 |####################
   2946.8 |
   2993.2 |
   3039.5 |
   3085.9 |#####
  (1 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1846.0-4045.0 ns)
   1846.0 |
   1955.9 |
   2065.9 |
   2175.8 |
   2285.8 |
   2395.7 |####################
   2505.7 |#####
   2615.6 |####################
   2725.6 |
   2835.5 |########################################
   2945.5 |##########
   3055.4 |###############
   3165.4 |##########
   3275.3 |
   3385.3 |##############################
   3495.2 |#####
   3605.2 |#####
   3715.1 |
   3825.1 |#####
   3935.0 |#####
  (3 below, 3 above range)

warm-container-minimum (n=40, range 2165.1-3649.8 ns)
   2165.1 |
   2239.3 |
   2313.6 |
   2387.8 |
   2462.0 |#############
   2536.3 |######
   2610.5 |
   2684.7 |##########################
   2759.0 |#############
   2833.2 |
   2907.5 |########################################
   2981.7 |#############
   3055.9 |######
   3130.2 |####################
   3204.4 |#################################
   3278.6 |
   3352.9 |#############
   3427.1 |
   3501.4 |########################################
   3575.6 |#############
  (2 below, 2 above range)

warm-container-native (n=40, range 2114.4-3739.9 ns)
   2114.4 |
   2195.7 |
   2277.0 |####
   2358.2 |
   2439.5 |#############
   2520.8 |####
   2602.1 |####
   2683.3 |######################
   2764.6 |####
   2845.9 |####
   2927.2 |#################
   3008.4 |
   3089.7 |########
   3171.0 |#############
   3252.2 |####
   3333.5 |####
   3414.8 |
   3496.1 |########################################
   3577.3 |####
   3658.6 |
  (2 below, 4 above range)

warm-container-plusone (n=40, range 1575.9-5956.2 ns)
   1575.9 |########################################
   1794.9 |
   2013.9 |
   2232.9 |
   2451.9 |
   2671.0 |###
   2890.0 |
   3109.0 |
   3328.0 |
   3547.0 |
   3766.0 |
   3985.1 |
   4204.1 |
   4423.1 |##############
   4642.1 |#######
   4861.1 |#######
   5080.1 |##################
   5299.2 |#######
   5518.2 |##############
   5737.2 |#######
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=41.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=28.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=20.5% (high variance, measurements may be unstable)
- **warm-container-native**: CV=21.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=47.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
