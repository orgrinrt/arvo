# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.00 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 2.60 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-accfit shows alternating (throttle bounce) (autocorr -0.71)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (403 ns) is smaller than the fastest variant's own run-to-run std-dev (2.38 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader warm-clamp-head vs stability leader warm-clamp-minimum (+8% speed for 11.8x steadier)

warm-clamp-head is fastest (2.60 us, CV 91.7%); warm-clamp-minimum gives up 8.2% median for 11.8x lower variance (CV 7.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 2.52 us but its worst 20% at 9.75 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-clamp-accfit's edge over baseline is significant but tiny (-17 ns, 0.55%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by -17 ns (0.55%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-head** at 2597.9 ns median (-13.4% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.16x (fastest 2597.9 ns, slowest 3001.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4476ns | 3064ns | 3058ns | 3119ns | 9964ns | base |
| warm-clamp-accfit | 4720ns | 3057ns | 3040ns | 3687ns | 9499ns | +5.47% |
| warm-clamp-accfit-dyn | 4712ns | 3058ns | 2569ns | 3657ns | 10019ns | +5.27% |
| warm-clamp-head | 4066ns | 2681ns | 2669ns | 2985ns | 8704ns | -9.16% |
| warm-clamp-min-lanes | 2887ns | 2860ns | 2512ns | 2885ns | 3267ns | -35.50% |
| warm-clamp-minimum | 3001ns | 3036ns | 2679ns | 2994ns | 3343ns | -32.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 4373ns | 2995ns | 9708ns | base | 1.873 |
| warm-clamp-accfit | 4606ns | 2974ns | 9232ns | +5.33% | 1.778 |
| warm-clamp-accfit-dyn | 4602ns | 2515ns | 9755ns | +5.23% | 1.780 |
| warm-clamp-head | 3927ns | 2586ns | 8372ns | -10.21% | 2.086 |
| warm-clamp-min-lanes | 2662ns | 2325ns | 2981ns | -39.12% | 3.077 |
| warm-clamp-minimum | 2763ns | 2479ns | 3032ns | -36.83% | 2.965 |

## Performance model

- Peak throughput: **3.524 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.729 | 77.5% |
| warm-clamp-accfit | 2.738 | 77.7% |
| warm-clamp-accfit-dyn | 2.735 | 77.6% |
| warm-clamp-head | 3.153 | 89.5% |
| warm-clamp-min-lanes | 3.089 | 87.7% |
| warm-clamp-minimum | 2.914 | 82.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 4476ns | 4476ns | base |
| warm-clamp-accfit | 4720ns | 4720ns | +5.47% |
| warm-clamp-accfit-dyn | 4712ns | 4712ns | +5.27% |
| warm-clamp-head | 4066ns | 4066ns | -9.16% |
| warm-clamp-min-lanes | 2887ns | 2887ns | -35.50% |
| warm-clamp-minimum | 3001ns | 3001ns | -32.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3001ns | base | --- | [2996, 3008] | --- | --- | --- | --- |
| warm-clamp-accfit | 2992ns | -16.6ns (-0.6%) | [-21, -6]ns | [2982, 2996] | YES | 0.0008 | 0.0007 | 0 |
| warm-clamp-accfit-dyn | 2995ns | no significant difference | [-18, +1]ns | [2994, 3006] | no | 0.0807 | 0.0807 | 0 |
| warm-clamp-head | 2598ns | -408.9ns (-13.6%) | [-411, -397]ns | [2586, 2632] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2652ns | -544.4ns (-18.1%) | [-610, -379]ns | [2577, 2832] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 2811ns | -380.5ns (-12.7%) | [-473, -314]ns | [2637, 2877] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3002ns | -0.2% | -18.3% | -12.1% | -12.2% | -12.1% |
| 1 | 2 | 3001ns | -0.5% | -0.1% | +32.4% | -4.2% | -4.1% |
| 1 | 3 | 3002ns | -0.3% | -0.0% | -13.8% | -19.0% | -4.1% |
| 1 | 4 | 3002ns | -0.4% | -0.2% | -13.8% | -12.1% | -12.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.024 | ok |
| warm-clamp-accfit | -0.711 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.081 | ok |
| warm-clamp-head | -0.395 | moderate- |
| warm-clamp-min-lanes | -0.511 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.250 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 0/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 35.7ns | 4373.1ns | 0.8% |  |
| warm-clamp-accfit | 36.7ns | 4606.3ns | 0.8% |  |
| warm-clamp-accfit-dyn | 37.7ns | 4601.9ns | 0.8% |  |
| warm-clamp-head | 36.4ns | 3926.7ns | 0.9% |  |
| warm-clamp-min-lanes | 79.8ns | 2662.4ns | 3.0% |  |
| warm-clamp-minimum | 85.2ns | 2762.5ns | 3.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2994.9-9707.8 ns)
   2994.9 |########################################
   3330.6 |
   3666.2 |
   4001.9 |#
   4337.5 |
   4673.1 |
   5008.8 |
   5344.4 |
   5680.1 |
   6015.7 |
   6351.4 |
   6687.0 |
   7022.6 |
   7358.3 |
   7693.9 |
   8029.6 |
   8365.2 |
   8700.8 |####
   9036.5 |
   9372.1 |#
  (2 below, 4 above range)

warm-clamp-accfit (n=40, range 2974.3-9231.9 ns)
   2974.3 |########################################
   3287.2 |
   3600.0 |
   3912.9 |#
   4225.8 |#
   4538.7 |#
   4851.6 |
   5164.4 |
   5477.3 |
   5790.2 |
   6103.1 |
   6416.0 |
   6728.8 |
   7041.7 |
   7354.6 |
   7667.5 |
   7980.4 |#
   8293.2 |#####
   8606.1 |#
   8919.0 |#
  (3 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 2515.1-9754.6 ns)
   2515.1 |
   2877.1 |########################################
   3239.1 |
   3601.0 |
   3963.0 |
   4325.0 |#
   4687.0 |
   5048.9 |
   5410.9 |
   5772.9 |#
   6134.9 |
   6496.8 |
   6858.8 |
   7220.8 |#
   7582.8 |
   7944.7 |#
   8306.7 |###
   8668.7 |
   9030.7 |#
   9392.6 |#
  (7 below, 4 above range)

warm-clamp-head (n=40, range 2585.8-8372.1 ns)
   2585.8 |########################################
   2875.2 |
   3164.5 |
   3453.8 |
   3743.1 |
   4032.4 |
   4321.7 |
   4611.0 |
   4900.3 |
   5189.6 |#
   5479.0 |
   5768.3 |
   6057.6 |
   6346.9 |
   6636.2 |#
   6925.5 |##
   7214.8 |
   7504.1 |#
   7793.5 |
   8082.8 |#
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 2324.9-2981.3 ns)
   2324.9 |
   2357.8 |####
   2390.6 |######################
   2423.4 |########
   2456.2 |####
   2489.0 |####
   2521.8 |####
   2554.7 |
   2587.5 |######################
   2620.3 |########
   2653.1 |########
   2685.9 |
   2718.7 |####
   2751.6 |
   2784.4 |########
   2817.2 |
   2850.0 |########################################
   2882.8 |
   2915.6 |########
   2948.5 |####
  (3 below, 2 above range)

warm-clamp-minimum (n=40, range 2479.5-3031.9 ns)
   2479.5 |
   2507.1 |###
   2534.7 |######
   2562.3 |
   2590.0 |#####################
   2617.6 |
   2645.2 |#########
   2672.8 |
   2700.5 |###
   2728.1 |
   2755.7 |###
   2783.3 |###
   2810.9 |###
   2838.6 |
   2866.2 |########################################
   2893.8 |###
   2921.4 |######
   2949.0 |###
   2976.7 |
   3004.3 |
  (4 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=61.7% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=56.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=62.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-head**: CV=60.7% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.2x (possible bimodal distribution)
