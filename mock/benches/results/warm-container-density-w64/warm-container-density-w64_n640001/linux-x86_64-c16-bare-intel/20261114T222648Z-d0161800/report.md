# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 49% (significant)

warm-container-kernel is -1.31 us (49%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.79)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (660 ns) is smaller than the fastest variant's own run-to-run std-dev (789 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-container-plusone is inconsistent: worst-20% is 4.3x its best-20%

warm-container-plusone's best 20% of batches run at 1.92 us but its worst 20% at 8.18 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 2622.9 ns median (-1.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.25x (fastest 2622.9 ns, slowest 3282.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4215ns | 2747ns | 2088ns | 3631ns | 8095ns | base |
| warm-container-kernel | 2764ns | 2854ns | 1428ns | 2867ns | 3794ns | -34.41% |
| warm-container-minimum | 3155ns | 3125ns | 1434ns | 3003ns | 5331ns | -25.15% |
| warm-container-native | 3377ns | 3511ns | 1251ns | 3461ns | 5250ns | -19.88% |
| warm-container-plusone | 4419ns | 2739ns | 1989ns | 3878ns | 8473ns | +4.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4068ns | 2021ns | 7783ns | base | 4.027 |
| warm-container-kernel | 2530ns | 1313ns | 3449ns | -37.81% | 6.476 |
| warm-container-minimum | 2927ns | 1330ns | 4910ns | -28.06% | 5.598 |
| warm-container-native | 3145ns | 1170ns | 4862ns | -22.69% | 5.210 |
| warm-container-plusone | 4270ns | 1922ns | 8181ns | +4.96% | 3.837 |

## Performance model

- Peak throughput: **14.005 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.163 | 44.0% |
| warm-container-kernel | 6.247 | 44.6% |
| warm-container-minimum | 5.609 | 40.0% |
| warm-container-native | 4.991 | 35.6% |
| warm-container-plusone | 6.167 | 44.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4215ns | 4215ns | base |
| warm-container-kernel | 2764ns | 2764ns | -34.41% |
| warm-container-minimum | 3155ns | 3155ns | -25.15% |
| warm-container-native | 3377ns | 3377ns | -19.88% |
| warm-container-plusone | 4419ns | 4419ns | +4.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2658ns | base | --- | [2652, 5108] | --- | --- | --- | --- |
| warm-container-kernel | 2623ns | -1305.5ns (-49.1%) | [-2275, -44]ns | [2498, 2811] | YES | 0.0257 | 0.0064 | 0 |
| warm-container-minimum | 2921ns | -256.6ns (-9.7%) | [-1503, -21]ns | [2630, 3220] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| warm-container-native | 3283ns | no significant difference | [-1482, +319]ns | [2923, 3976] | no | 0.0807 | 0.0807 | 0 |
| warm-container-plusone | 2657ns | -440.3ns (-16.6%) | [-489, -198]ns | [2180, 5602] | YES (adj: no) | 0.0513 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 2654ns | -49.2% | -25.0% | +56.1% | -17.9% |
| 1 | 2 | 4005ns | -34.5% | -14.7% | -22.0% | -20.7% |
| 1 | 3 | 5108ns | -46.5% | -68.7% | -58.6% | -47.9% |
| 1 | 4 | 2657ns | -1.7% | +6.4% | +46.2% | +91.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.286 | moderate- |
| warm-container-kernel | -0.043 | ok |
| warm-container-minimum | -0.790 | HIGH- (thermal bounce) |
| warm-container-native | -0.246 | moderate- |
| warm-container-plusone | -0.192 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 41.4ns | 4068.1ns | 1.0% |  |
| warm-container-kernel | 69.9ns | 2529.9ns | 2.8% |  |
| warm-container-minimum | 66.5ns | 2926.6ns | 2.3% |  |
| warm-container-native | 68.1ns | 3144.9ns | 2.2% |  |
| warm-container-plusone | 42.9ns | 4269.8ns | 1.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2020.6-7783.5 ns)
   2020.6 |############
   2308.8 |
   2596.9 |########################################
   2885.1 |##
   3173.2 |
   3461.3 |
   3749.5 |
   4037.6 |
   4325.8 |
   4613.9 |
   4902.1 |#####
   5190.2 |
   5478.3 |#######
   5766.5 |#####
   6054.6 |##
   6342.8 |##
   6630.9 |##
   6919.0 |##
   7207.2 |
   7495.3 |##
  (3 below, 3 above range)

warm-container-kernel (n=40, range 1313.1-3448.7 ns)
   1313.1 |
   1419.9 |
   1526.6 |
   1633.4 |
   1740.2 |
   1847.0 |#####
   1953.8 |
   2060.5 |
   2167.3 |
   2274.1 |####################
   2380.9 |####################
   2487.6 |###############
   2594.4 |########################################
   2701.2 |#####
   2808.0 |##############################
   2914.8 |
   3021.5 |####################
   3128.3 |
   3235.1 |
   3341.9 |
  (5 below, 4 above range)

warm-container-minimum (n=40, range 1330.1-4910.4 ns)
   1330.1 |
   1509.2 |########################################
   1688.2 |
   1867.2 |
   2046.2 |
   2225.2 |#####
   2404.2 |
   2583.2 |#################
   2762.2 |######################
   2941.2 |############################
   3120.3 |#################
   3299.3 |#####
   3478.3 |###########
   3657.3 |
   3836.3 |###########
   4015.3 |#####
   4194.3 |
   4373.3 |#####
   4552.3 |
   4731.4 |#####
  (5 below, 4 above range)

warm-container-native (n=40, range 1169.9-4862.1 ns)
   1169.9 |########################################
   1354.5 |
   1539.1 |
   1723.7 |
   1908.3 |
   2093.0 |
   2277.6 |
   2462.2 |
   2646.8 |##########
   2831.4 |###############
   3016.0 |###############
   3200.6 |##########
   3385.2 |##########
   3569.8 |
   3754.4 |###############
   3939.1 |#####
   4123.7 |##############################
   4308.3 |
   4492.9 |##########
   4677.5 |##########
  (3 below, 3 above range)

warm-container-plusone (n=40, range 1921.9-8180.9 ns)
   1921.9 |########################################
   2234.8 |
   2547.8 |############################
   2860.7 |
   3173.7 |
   3486.6 |
   3799.6 |
   4112.5 |####
   4425.5 |####
   4738.4 |
   5051.4 |####
   5364.3 |####
   5677.3 |####
   5990.2 |############
   6303.2 |########
   6616.1 |####
   6929.1 |
   7242.0 |####
   7555.0 |
   7867.9 |########
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=56.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-kernel**: CV=31.2% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=43.9% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-native**: CV=44.0% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-plusone**: CV=57.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.3x (possible bimodal distribution)
