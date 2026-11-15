# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head dominates: 23% faster than the next best (warm-clamp-accfit)

warm-clamp-head (2.54 us) leads warm-clamp-accfit (3.12 us) by 23%, a clear separation rather than a photo finish. CV 8.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.64)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-minimum is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-minimum's best 20% of batches run at 1.58 us but its worst 20% at 6.13 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 2543.8 ns median (-23.7% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.84x (fastest 2543.8 ns, slowest 4689.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3284ns | 3542ns | 1595ns | 3523ns | 4259ns | base |
| warm-clamp-accfit | 3217ns | 3314ns | 2130ns | 3311ns | 4021ns | -2.05% |
| warm-clamp-accfit-dyn | 3361ns | 3633ns | 1731ns | 3629ns | 4186ns | +2.33% |
| warm-clamp-head | 2870ns | 2783ns | 2581ns | 2834ns | 3264ns | -12.63% |
| warm-clamp-min-lanes | 4126ns | 4883ns | 1605ns | 4297ns | 6133ns | +25.62% |
| warm-clamp-minimum | 3703ns | 4007ns | 1644ns | 3476ns | 6443ns | +12.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3081ns | 1501ns | 3967ns | base | 2.659 |
| warm-clamp-accfit | 3024ns | 2009ns | 3754ns | -1.85% | 2.709 |
| warm-clamp-accfit-dyn | 3151ns | 1625ns | 3886ns | +2.30% | 2.599 |
| warm-clamp-head | 2610ns | 2354ns | 2934ns | -15.29% | 3.139 |
| warm-clamp-min-lanes | 3948ns | 1539ns | 5842ns | +28.17% | 2.075 |
| warm-clamp-minimum | 3543ns | 1579ns | 6129ns | +14.99% | 2.312 |

## Performance model

- Peak throughput: **5.458 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.457 | 45.0% |
| warm-clamp-accfit | 2.622 | 48.0% |
| warm-clamp-accfit-dyn | 2.391 | 43.8% |
| warm-clamp-head | 3.220 | 59.0% |
| warm-clamp-min-lanes | 1.747 | 32.0% |
| warm-clamp-minimum | 2.128 | 39.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3284ns | 3284ns | base |
| warm-clamp-accfit | 3217ns | 3217ns | -2.05% |
| warm-clamp-accfit-dyn | 3361ns | 3361ns | +2.33% |
| warm-clamp-head | 2870ns | 2870ns | -12.63% |
| warm-clamp-min-lanes | 4126ns | 4126ns | +25.62% |
| warm-clamp-minimum | 3703ns | 3703ns | +12.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3335ns | base | --- | [3094, 3465] | --- | --- | --- | --- |
| warm-clamp-accfit | 3124ns | -71.3ns (-2.1%) | [-173, -1]ns | [2928, 3242] | YES | 0.0158 | 0.0095 | 1 |
| warm-clamp-accfit-dyn | 3426ns | no significant difference | [-77, +238]ns | [3148, 3653] | no | 0.2682 | 0.2682 | 0 |
| warm-clamp-head | 2544ns | -632.2ns (-19.0%) | [-880, -494]ns | [2494, 2680] | YES | 0.0002 | 0.0000 | 0 |
| warm-clamp-min-lanes | 4689ns | +1203.2ns (+36.1%) | [+847, +1516]ns | [2960, 5183] | YES | 0.0017 | 0.0007 | 0 |
| warm-clamp-minimum | 3849ns | +750.4ns (+22.5%) | [+244, +1178]ns | [1748, 4451] | YES | 0.0481 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3773ns | -9.6% | -9.3% | -29.0% | +24.0% | -9.1% |
| 1 | 2 | 3465ns | -10.9% | +3.2% | -27.4% | +42.0% | -18.7% |
| 1 | 3 | 3304ns | -1.6% | +6.3% | -20.2% | -29.3% | -41.9% |
| 1 | 4 | 2936ns | -1.0% | +11.3% | -14.5% | +63.5% | +34.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.166 | ok |
| warm-clamp-accfit | -0.341 | moderate- |
| warm-clamp-accfit-dyn | -0.113 | ok |
| warm-clamp-head | -0.637 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.466 | moderate- |
| warm-clamp-minimum | -0.381 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 1/4, lost 3/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 1/4, lost 3/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 70.5ns | 3080.7ns | 2.3% |  |
| warm-clamp-accfit | 70.2ns | 3023.8ns | 2.3% |  |
| warm-clamp-accfit-dyn | 71.5ns | 3151.4ns | 2.3% |  |
| warm-clamp-head | 71.2ns | 2609.7ns | 2.7% |  |
| warm-clamp-min-lanes | 59.0ns | 3948.4ns | 1.5% |  |
| warm-clamp-minimum | 51.0ns | 3542.5ns | 1.4% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1500.8-3967.2 ns)
   1500.8 |
   1624.1 |#####
   1747.4 |
   1870.7 |
   1994.1 |
   2117.4 |#####
   2240.7 |
   2364.0 |
   2487.3 |
   2610.7 |
   2734.0 |
   2857.3 |##############################
   2980.6 |#####
   3103.9 |#########################
   3227.3 |##########
   3350.6 |##############################
   3473.9 |
   3597.2 |
   3720.5 |########################################
   3843.9 |##########
  (5 below, 3 above range)

warm-clamp-accfit (n=40, range 2008.6-3753.7 ns)
   2008.6 |
   2095.9 |
   2183.1 |
   2270.4 |#####
   2357.6 |#####
   2444.9 |
   2532.1 |
   2619.4 |#####
   2706.7 |#####
   2793.9 |#####
   2881.2 |########################################
   2968.4 |#####
   3055.7 |########################################
   3142.9 |#################
   3230.2 |#####
   3317.4 |###########
   3404.7 |######################
   3491.9 |#####
   3579.2 |
   3666.5 |######################
  (3 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1624.6-3886.1 ns)
   1624.6 |#####
   1737.7 |
   1850.7 |
   1963.8 |
   2076.9 |
   2190.0 |
   2303.0 |
   2416.1 |##########
   2529.2 |
   2642.3 |
   2755.3 |
   2868.4 |###############
   2981.5 |
   3094.6 |#########################
   3207.6 |
   3320.7 |#########################
   3433.8 |####################
   3546.9 |##########
   3659.9 |########################################
   3773.0 |###############
  (4 below, 3 above range)

warm-clamp-head (n=40, range 2353.6-2934.1 ns)
   2353.6 |#####
   2382.6 |
   2411.7 |#####
   2440.7 |#####
   2469.7 |########################################
   2498.7 |###############
   2527.8 |##########
   2556.8 |
   2585.8 |#####
   2614.8 |
   2643.9 |
   2672.9 |###################################
   2701.9 |##########
   2730.9 |##########
   2760.0 |
   2789.0 |##########
   2818.0 |
   2847.0 |
   2876.1 |
   2905.1 |
  (5 below, 5 above range)

warm-clamp-min-lanes (n=40, range 1539.3-5841.9 ns)
   1539.3 |########################################
   1754.4 |
   1969.5 |####
   2184.7 |
   2399.8 |
   2614.9 |
   2830.1 |####
   3045.2 |####
   3260.3 |
   3475.5 |
   3690.6 |
   3905.7 |####
   4120.9 |
   4336.0 |####
   4551.1 |################
   4766.3 |################
   4981.4 |####
   5196.5 |########
   5411.7 |################
   5626.8 |################
  (2 below, 4 above range)

warm-clamp-minimum (n=40, range 1578.8-6129.1 ns)
   1578.8 |########################################
   1806.3 |##
   2033.8 |##
   2261.3 |
   2488.9 |
   2716.4 |
   2943.9 |
   3171.4 |
   3398.9 |
   3626.4 |##
   3854.0 |##########
   4081.5 |#####
   4309.0 |
   4536.5 |##
   4764.0 |
   4991.5 |##
   5219.1 |
   5446.6 |########
   5674.1 |##
   5901.6 |################
  (2 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=29.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: CV=22.7% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: CV=28.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=44.4% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=53.0% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
