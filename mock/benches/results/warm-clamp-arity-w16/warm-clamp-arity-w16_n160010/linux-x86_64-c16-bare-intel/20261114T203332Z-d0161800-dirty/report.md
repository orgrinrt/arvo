# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (4.26 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-minimum at 2.70 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-accfit-dyn beats baseline by 33% (significant)

warm-clamp-accfit-dyn is -1.40 us (33%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is fastest but the noisiest (CV 38.0%)

warm-clamp-minimum wins on median (2.70 us) yet has the highest variance (CV 38.0%), while warm-clamp-accfit-dyn is the steadiest (CV 7.2%, 2.84 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.58)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-head, warm-clamp-min-lanes} vs {warm-clamp-acc64} (41% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-head, warm-clamp-min-lanes} and a slow tier {warm-clamp-acc64} with a 41% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-clamp-minimum vs stability leader warm-clamp-accfit-dyn (+5% speed for 5.3x steadier)

warm-clamp-minimum is fastest (2.70 us, CV 38.0%); warm-clamp-accfit-dyn gives up 5.0% median for 5.3x lower variance (CV 7.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-clamp-acc64 is inconsistent: worst-20% is 3.7x its best-20%

warm-clamp-acc64's best 20% of batches run at 1.42 us but its worst 20% at 5.20 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 2704.0 ns median (-36.6% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 1.58x (fastest 2704.0 ns, slowest 4262.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3723ns | 4461ns | 1478ns | 3883ns | 5486ns | base |
| warm-clamp-accfit | 2973ns | 3101ns | 2337ns | 3011ns | 3496ns | -20.14% |
| warm-clamp-accfit-dyn | 3024ns | 3080ns | 2691ns | 3019ns | 3372ns | -18.78% |
| warm-clamp-head | 3093ns | 3144ns | 2475ns | 3124ns | 3618ns | -16.92% |
| warm-clamp-min-lanes | 3276ns | 3217ns | 2873ns | 3221ns | 3840ns | -12.02% |
| warm-clamp-minimum | 3029ns | 2959ns | 1437ns | 3036ns | 4600ns | -18.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3550ns | 1419ns | 5203ns | base | 2.307 |
| warm-clamp-accfit | 2744ns | 2166ns | 3187ns | -22.72% | 2.986 |
| warm-clamp-accfit-dyn | 2773ns | 2484ns | 3041ns | -21.91% | 2.955 |
| warm-clamp-head | 2874ns | 2306ns | 3330ns | -19.04% | 2.850 |
| warm-clamp-min-lanes | 3063ns | 2706ns | 3533ns | -13.72% | 2.674 |
| warm-clamp-minimum | 2813ns | 1344ns | 4286ns | -20.77% | 2.912 |

## Performance model

- Peak throughput: **6.094 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 1.922 | 31.5% |
| warm-clamp-accfit | 2.851 | 46.8% |
| warm-clamp-accfit-dyn | 2.885 | 47.3% |
| warm-clamp-head | 2.789 | 45.8% |
| warm-clamp-min-lanes | 2.705 | 44.4% |
| warm-clamp-minimum | 3.030 | 49.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3723ns | 3723ns | base |
| warm-clamp-accfit | 2973ns | 2973ns | -20.14% |
| warm-clamp-accfit-dyn | 3024ns | 3024ns | -18.78% |
| warm-clamp-head | 3093ns | 3093ns | -16.92% |
| warm-clamp-min-lanes | 3276ns | 3276ns | -12.02% |
| warm-clamp-minimum | 3029ns | 3029ns | -18.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4263ns | base | --- | [2855, 4478] | --- | --- | --- | --- |
| warm-clamp-accfit | 2874ns | -1274.8ns (-29.9%) | [-1568, -502]ns | [2648, 2874] | YES | 0.0276 | 0.0166 | 0 |
| warm-clamp-accfit-dyn | 2839ns | -1399.8ns (-32.8%) | [-1678, -287]ns | [2688, 2857] | YES | 0.0276 | 0.0166 | 0 |
| warm-clamp-head | 2937ns | -1041.4ns (-24.4%) | [-1279, -340]ns | [2728, 3143] | YES | 0.0385 | 0.0385 | 0 |
| warm-clamp-min-lanes | 3028ns | -975.2ns (-22.9%) | [-1185, -60]ns | [2868, 3153] | YES | 0.0385 | 0.0385 | 0 |
| warm-clamp-minimum | 2704ns | -593.4ns (-13.9%) | [-1531, -364]ns | [2545, 2979] | YES | 0.0111 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3391ns | -14.3% | -22.9% | -13.3% | -10.7% | -1.8% |
| 1 | 2 | 4332ns | -33.7% | -36.1% | -28.9% | -31.2% | -9.0% |
| 1 | 3 | 4315ns | -33.4% | -33.7% | -26.0% | -23.5% | -39.4% |
| 1 | 4 | 3913ns | -33.3% | -27.4% | -41.0% | -28.5% | -33.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.200 | ok |
| warm-clamp-accfit | -0.060 | ok |
| warm-clamp-accfit-dyn | 0.175 | ok |
| warm-clamp-head | -0.224 | moderate- |
| warm-clamp-min-lanes | -0.581 | HIGH- (thermal bounce) |
| warm-clamp-minimum | 0.011 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 59.3ns | 3550.5ns | 1.7% |  |
| warm-clamp-accfit | 82.1ns | 2743.6ns | 3.0% |  |
| warm-clamp-accfit-dyn | 84.2ns | 2772.5ns | 3.0% |  |
| warm-clamp-head | 76.6ns | 2874.3ns | 2.7% |  |
| warm-clamp-min-lanes | 74.1ns | 3063.3ns | 2.4% |  |
| warm-clamp-minimum | 73.4ns | 2813.1ns | 2.6% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1418.5-5202.9 ns)
   1418.5 |########################################
   1607.7 |
   1797.0 |
   1986.2 |
   2175.4 |
   2364.6 |
   2553.8 |#####
   2743.0 |##########
   2932.3 |
   3121.5 |
   3310.7 |
   3499.9 |
   3689.1 |
   3878.3 |####################
   4067.6 |#####
   4256.8 |####################
   4446.0 |####################
   4635.2 |##########
   4824.4 |####################
   5013.6 |#####
  (4 below, 5 above range)

warm-clamp-accfit (n=40, range 2166.5-3186.9 ns)
   2166.5 |
   2217.5 |
   2268.5 |###
   2319.5 |
   2370.6 |######
   2421.6 |###
   2472.6 |
   2523.6 |
   2574.7 |#####################
   2625.7 |#########
   2676.7 |###
   2727.7 |
   2778.7 |
   2829.8 |########################################
   2880.8 |######
   2931.8 |######
   2982.8 |
   3033.8 |###
   3084.9 |
   3135.9 |###
  (2 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 2483.8-3040.6 ns)
   2483.8 |
   2511.7 |#####
   2539.5 |##########
   2567.3 |#####
   2595.2 |#########################
   2623.0 |
   2650.8 |
   2678.7 |##########
   2706.5 |
   2734.4 |##########
   2762.2 |
   2790.0 |
   2817.9 |#########################
   2845.7 |########################################
   2873.5 |#####
   2901.4 |####################
   2929.2 |
   2957.1 |#####
   2984.9 |
   3012.7 |
  (4 below, 4 above range)

warm-clamp-head (n=40, range 2306.4-3329.9 ns)
   2306.4 |###############
   2357.6 |
   2408.8 |
   2459.9 |####################
   2511.1 |
   2562.3 |
   2613.4 |
   2664.6 |##########
   2715.8 |####################
   2767.0 |#####
   2818.1 |##########
   2869.3 |
   2920.5 |####################
   2971.7 |#####
   3022.8 |#####
   3074.0 |
   3125.2 |##########
   3176.4 |#####
   3227.5 |########################################
   3278.7 |#####
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 2706.3-3533.2 ns)
   2706.3 |
   2747.6 |
   2789.0 |########################################
   2830.3 |####
   2871.7 |####
   2913.0 |####
   2954.3 |####
   2995.7 |##########################
   3037.0 |#############
   3078.4 |
   3119.7 |
   3161.1 |####
   3202.4 |
   3243.8 |########
   3285.1 |#############
   3326.5 |########
   3367.8 |####
   3409.2 |####
   3450.5 |
   3491.9 |
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 1344.3-4286.2 ns)
   1344.3 |#####
   1491.4 |
   1638.5 |
   1785.6 |
   1932.7 |
   2079.8 |#####
   2226.8 |###########
   2373.9 |#################
   2521.0 |##################################
   2668.1 |########################################
   2815.2 |#####
   2962.3 |#####
   3109.4 |
   3256.5 |#################
   3403.6 |
   3550.7 |#################
   3697.8 |#####
   3844.9 |
   3992.0 |#################
   4139.1 |
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=42.5% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=36.6% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.2x (possible bimodal distribution)
