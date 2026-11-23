# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head dominates: 13% faster than the next best (warm-clamp-accfit)

warm-clamp-head (2.63 us) leads warm-clamp-accfit (2.98 us) by 13%, a clear separation rather than a photo finish. CV 19.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.59)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-minimum is inconsistent: worst-20% is 4.3x its best-20%

warm-clamp-minimum's best 20% of batches run at 1.08 us but its worst 20% at 4.70 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 2629.2 ns median (-16.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.38x (fastest 2629.2 ns, slowest 3630.4 ns)

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
| warm-clamp-acc64 | 3209ns | 3353ns | 1909ns | 3369ns | 4028ns | base |
| warm-clamp-accfit | 3148ns | 3180ns | 2649ns | 3157ns | 3620ns | -1.88% |
| warm-clamp-accfit-dyn | 3263ns | 3579ns | 2114ns | 3364ns | 4108ns | +1.70% |
| warm-clamp-head | 2818ns | 2897ns | 2021ns | 2857ns | 3499ns | -12.17% |
| warm-clamp-min-lanes | 3459ns | 3362ns | 2650ns | 3389ns | 4476ns | +7.80% |
| warm-clamp-minimum | 3301ns | 3818ns | 1139ns | 3461ns | 4984ns | +2.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2995ns | 1789ns | 3727ns | base | 2.735 |
| warm-clamp-accfit | 2941ns | 2485ns | 3355ns | -1.79% | 2.785 |
| warm-clamp-accfit-dyn | 3047ns | 1982ns | 3807ns | +1.75% | 2.688 |
| warm-clamp-head | 2546ns | 1837ns | 3131ns | -15.00% | 3.218 |
| warm-clamp-min-lanes | 3263ns | 2507ns | 4225ns | +8.95% | 2.511 |
| warm-clamp-minimum | 3127ns | 1082ns | 4698ns | +4.40% | 2.620 |

## Performance model

- Peak throughput: **7.571 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.608 | 34.4% |
| warm-clamp-accfit | 2.747 | 36.3% |
| warm-clamp-accfit-dyn | 2.444 | 32.3% |
| warm-clamp-head | 3.116 | 41.2% |
| warm-clamp-min-lanes | 2.576 | 34.0% |
| warm-clamp-minimum | 2.257 | 29.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3209ns | 3209ns | base |
| warm-clamp-accfit | 3148ns | 3148ns | -1.88% |
| warm-clamp-accfit-dyn | 3263ns | 3263ns | +1.70% |
| warm-clamp-head | 2818ns | 2818ns | -12.17% |
| warm-clamp-min-lanes | 3459ns | 3459ns | +7.80% |
| warm-clamp-minimum | 3301ns | 3301ns | +2.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3142ns | base | --- | [3038, 3342] | --- | --- | --- | --- |
| warm-clamp-accfit | 2982ns | no significant difference | [-349, +34]ns | [2799, 3058] | no | 0.2564 | 0.1539 | 0 |
| warm-clamp-accfit-dyn | 3352ns | no significant difference | [-338, +91]ns | [2827, 3356] | no | 0.3352 | 0.2682 | 0 |
| warm-clamp-head | 2629ns | -572.3ns (-18.2%) | [-637, -514]ns | [2414, 2684] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 3180ns | no significant difference | [-99, +205]ns | [2961, 3361] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-minimum | 3630ns | +360.4ns (+11.5%) | [+255, +481]ns | [3175, 3843] | YES | 0.0005 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2933ns | +0.1% | +14.4% | -1.9% | +41.4% | +31.5% |
| 1 | 2 | 3272ns | -14.5% | -6.4% | -17.7% | -9.5% | +12.6% |
| 1 | 3 | 3038ns | +4.6% | -12.9% | -25.9% | +5.7% | +11.4% |
| 1 | 4 | 3299ns | -10.7% | +4.3% | -21.7% | -9.8% | +24.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.593 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.470 | moderate- |
| warm-clamp-accfit-dyn | -0.349 | moderate- |
| warm-clamp-head | -0.000 | ok |
| warm-clamp-min-lanes | -0.233 | moderate- |
| warm-clamp-minimum | -0.396 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 2/4, lost 2/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 76.7ns | 2994.9ns | 2.6% |  |
| warm-clamp-accfit | 75.0ns | 2941.3ns | 2.5% |  |
| warm-clamp-accfit-dyn | 78.5ns | 3047.3ns | 2.6% |  |
| warm-clamp-head | 76.0ns | 2545.6ns | 3.0% |  |
| warm-clamp-min-lanes | 67.1ns | 3262.8ns | 2.1% |  |
| warm-clamp-minimum | 58.2ns | 3126.7ns | 1.9% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1789.3-3727.2 ns)
   1789.3 |
   1886.2 |
   1983.1 |
   2080.0 |
   2176.9 |
   2273.8 |
   2370.7 |###
   2467.6 |
   2564.5 |###
   2661.4 |
   2758.3 |#####################
   2855.2 |
   2952.1 |##################
   3049.0 |##########
   3145.9 |###
   3242.8 |#######
   3339.7 |########################################
   3436.6 |###
   3533.4 |###
   3630.3 |
  (4 below, 4 above range)

warm-clamp-accfit (n=40, range 2484.7-3354.6 ns)
   2484.7 |
   2528.1 |#####
   2571.6 |#################
   2615.1 |###########
   2658.6 |
   2702.1 |
   2745.6 |###########
   2789.1 |########################################
   2832.6 |#################
   2876.1 |
   2919.6 |#####
   2963.1 |
   3006.6 |#################
   3050.1 |######################
   3093.6 |#################
   3137.1 |
   3180.6 |
   3224.1 |#################
   3267.6 |
   3311.1 |#####
  (1 below, 6 above range)

warm-clamp-accfit-dyn (n=40, range 1982.1-3807.0 ns)
   1982.1 |
   2073.3 |
   2164.6 |
   2255.8 |
   2347.1 |#####
   2438.3 |##
   2529.6 |#####
   2620.8 |#####
   2712.1 |###########
   2803.3 |##
   2894.6 |
   2985.8 |########
   3077.0 |
   3168.3 |
   3259.5 |##
   3350.8 |########################################
   3442.0 |##
   3533.3 |
   3624.5 |
   3715.8 |###########
  (3 below, 2 above range)

warm-clamp-head (n=40, range 1837.1-3131.2 ns)
   1837.1 |
   1901.8 |
   1966.5 |
   2031.2 |#####
   2095.9 |#####
   2160.6 |
   2225.3 |############################
   2290.0 |
   2354.7 |##################################
   2419.4 |###########
   2484.1 |###########
   2548.8 |
   2613.5 |########################################
   2678.3 |###########
   2743.0 |
   2807.7 |###########
   2872.4 |######################
   2937.1 |###########
   3001.8 |
   3066.5 |
  (2 below, 4 above range)

warm-clamp-min-lanes (n=40, range 2507.4-4224.6 ns)
   2507.4 |
   2593.2 |
   2679.1 |#################
   2765.0 |#################
   2850.8 |###########
   2936.7 |########################################
   3022.5 |#####
   3108.4 |############################
   3194.2 |###########
   3280.1 |#####
   3366.0 |#####
   3451.8 |#################
   3537.7 |
   3623.5 |#####
   3709.4 |#####
   3795.3 |
   3881.1 |
   3967.0 |
   4052.8 |
   4138.7 |########################################
  (2 below, 1 above range)

warm-clamp-minimum (n=40, range 1082.0-4697.5 ns)
   1082.0 |##############################
   1262.8 |########################################
   1443.6 |
   1624.3 |
   1805.1 |
   1985.9 |
   2166.7 |
   2347.4 |
   2528.2 |
   2709.0 |
   2889.8 |##########
   3070.5 |####################
   3251.3 |####################
   3432.1 |##############################
   3612.9 |########################################
   3793.6 |########################################
   3974.4 |####################
   4155.2 |####################
   4336.0 |##########
   4516.7 |########################################
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=26.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: CV=23.3% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=20.5% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: CV=44.0% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.3x (possible bimodal distribution)
