# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head dominates: 26% faster than the next best (warm-clamp-accfit)

warm-clamp-head (2.50 us) leads warm-clamp-accfit (3.15 us) by 26%, a clear separation rather than a photo finish. CV 9.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 22% (significant)

warm-clamp-head is -762 ns (22%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.67)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.2x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.28 us but its worst 20% at 5.34 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 2497.6 ns median (-26.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.74x (fastest 2497.6 ns, slowest 4356.8 ns)

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
| warm-clamp-acc64 | 3464ns | 3613ns | 2656ns | 3517ns | 4113ns | base |
| warm-clamp-accfit | 3426ns | 3401ns | 2619ns | 3425ns | 4234ns | -1.11% |
| warm-clamp-accfit-dyn | 3417ns | 3657ns | 2251ns | 3568ns | 4132ns | -1.36% |
| warm-clamp-head | 2741ns | 2723ns | 2388ns | 2723ns | 3148ns | -20.88% |
| warm-clamp-min-lanes | 3904ns | 4548ns | 1338ns | 4188ns | 5619ns | +12.69% |
| warm-clamp-minimum | 3396ns | 4114ns | 1301ns | 3486ns | 5220ns | -1.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3258ns | 2505ns | 3838ns | base | 2.515 |
| warm-clamp-accfit | 3184ns | 2471ns | 3862ns | -2.28% | 2.573 |
| warm-clamp-accfit-dyn | 3213ns | 2125ns | 3860ns | -1.37% | 2.549 |
| warm-clamp-head | 2504ns | 2189ns | 2852ns | -23.13% | 3.271 |
| warm-clamp-min-lanes | 3729ns | 1282ns | 5339ns | +14.46% | 2.197 |
| warm-clamp-minimum | 3228ns | 1242ns | 4938ns | -0.91% | 2.538 |

## Performance model

- Peak throughput: **6.598 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.404 | 36.4% |
| warm-clamp-accfit | 2.599 | 39.4% |
| warm-clamp-accfit-dyn | 2.387 | 36.2% |
| warm-clamp-head | 3.280 | 49.7% |
| warm-clamp-min-lanes | 1.880 | 28.5% |
| warm-clamp-minimum | 2.086 | 31.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3464ns | 3464ns | base |
| warm-clamp-accfit | 3426ns | 3426ns | -1.11% |
| warm-clamp-accfit-dyn | 3417ns | 3417ns | -1.36% |
| warm-clamp-head | 2741ns | 2741ns | -20.88% |
| warm-clamp-min-lanes | 3904ns | 3904ns | +12.69% |
| warm-clamp-minimum | 3396ns | 3396ns | -1.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3408ns | base | --- | [3124, 3440] | --- | --- | --- | --- |
| warm-clamp-accfit | 3152ns | no significant difference | [-87, +68]ns | [3030, 3428] | no | 0.3352 | 0.2682 | 0 |
| warm-clamp-accfit-dyn | 3432ns | no significant difference | [-138, +244]ns | [2997, 3692] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-head | 2498ns | -762.4ns (-22.4%) | [-960, -687]ns | [2356, 2603] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 4357ns | +1090.8ns (+32.0%) | [+755, +1233]ns | [3946, 4596] | YES | 0.0161 | 0.0064 | 0 |
| warm-clamp-minimum | 3928ns | no significant difference | [-701, +802]ns | [1640, 4355] | no | 0.3352 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3749ns | -25.1% | -6.3% | -33.4% | +26.8% | +0.1% |
| 1 | 2 | 3145ns | -0.2% | -10.5% | -16.8% | +15.2% | +27.6% |
| 1 | 3 | 3125ns | +14.0% | +13.6% | -30.7% | +41.3% | +25.7% |
| 1 | 4 | 3370ns | +0.2% | +11.2% | -21.6% | +19.4% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.164 | ok |
| warm-clamp-accfit | 0.186 | ok |
| warm-clamp-accfit-dyn | -0.197 | ok |
| warm-clamp-head | -0.635 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.670 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.108 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 73.8ns | 3257.7ns | 2.3% |  |
| warm-clamp-accfit | 82.9ns | 3183.5ns | 2.6% |  |
| warm-clamp-accfit-dyn | 72.4ns | 3213.2ns | 2.3% |  |
| warm-clamp-head | 68.3ns | 2504.3ns | 2.7% |  |
| warm-clamp-min-lanes | 59.1ns | 3728.9ns | 1.6% |  |
| warm-clamp-minimum | 56.5ns | 3228.2ns | 1.8% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2505.4-3837.8 ns)
   2505.4 |#####
   2572.0 |
   2638.6 |#####
   2705.2 |#####
   2771.8 |
   2838.5 |###########
   2905.1 |###########
   2971.7 |#####
   3038.3 |#####
   3105.0 |########################################
   3171.6 |
   3238.2 |
   3304.8 |#####
   3371.5 |########################################
   3438.1 |#################
   3504.7 |
   3571.3 |
   3638.0 |###########
   3704.6 |######################
   3771.2 |#################
  (2 below, 2 above range)

warm-clamp-accfit (n=40, range 2471.3-3862.2 ns)
   2471.3 |#####
   2540.9 |
   2610.4 |#####
   2680.0 |######################
   2749.5 |#####
   2819.0 |
   2888.6 |##################################
   2958.1 |
   3027.7 |
   3097.2 |########################################
   3166.7 |###########
   3236.3 |###########
   3305.8 |
   3375.4 |#################
   3444.9 |#################
   3514.5 |
   3584.0 |#####
   3653.5 |###########
   3723.1 |###########
   3792.6 |###########
  (1 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 2125.4-3859.7 ns)
   2125.4 |
   2212.1 |
   2298.8 |
   2385.6 |
   2472.3 |##########
   2559.0 |#####
   2645.7 |##########
   2732.4 |#####
   2819.1 |#####
   2905.9 |####################
   2992.6 |#####
   3079.3 |#####
   3166.0 |
   3252.7 |#####
   3339.4 |###############
   3426.2 |####################
   3512.9 |
   3599.6 |##########
   3686.3 |########################################
   3773.0 |####################
  (3 below, 2 above range)

warm-clamp-head (n=40, range 2189.4-2851.6 ns)
   2189.4 |######
   2222.5 |
   2255.6 |
   2288.7 |#################################
   2321.8 |#############
   2354.9 |######
   2388.1 |####################
   2421.2 |
   2454.3 |
   2487.4 |#################################
   2520.5 |#############
   2553.6 |######
   2586.7 |
   2619.8 |######
   2652.9 |
   2686.0 |########################################
   2719.1 |#############
   2752.2 |
   2785.4 |
   2818.5 |######
  (6 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1281.8-5339.4 ns)
   1281.8 |#####
   1484.7 |#################
   1687.6 |
   1890.5 |
   2093.4 |
   2296.2 |
   2499.1 |
   2702.0 |#####
   2904.9 |
   3107.8 |
   3310.6 |
   3513.5 |
   3716.4 |
   3919.3 |############################
   4122.2 |#################
   4325.0 |###########
   4527.9 |########################################
   4730.8 |###########
   4933.7 |#################
   5136.6 |#####
  (7 below, 5 above range)

warm-clamp-minimum (n=40, range 1241.5-4937.5 ns)
   1241.5 |########################################
   1426.3 |
   1611.1 |
   1795.9 |####
   1980.7 |
   2165.5 |
   2350.3 |####
   2535.1 |
   2719.9 |
   2904.7 |
   3089.5 |
   3274.3 |####
   3459.1 |####
   3643.9 |
   3828.7 |############
   4013.5 |############
   4198.3 |########
   4383.1 |############
   4567.9 |################
   4752.7 |################
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit-dyn**: CV=21.8% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=41.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=47.7% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
