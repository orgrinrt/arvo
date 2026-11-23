# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum dominates: 11% faster than the next best (warm-clamp-accfit)

warm-clamp-minimum (2.18 us) leads warm-clamp-accfit (2.43 us) by 11%, a clear separation rather than a photo finish. CV 95.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum is fastest but the noisiest (CV 95.2%)

warm-clamp-minimum wins on median (2.18 us) yet has the highest variance (CV 95.2%), while warm-clamp-accfit is the steadiest (CV 9.2%, 2.43 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-accfit shows alternating (throttle bounce) (autocorr -0.64)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.62 us) is smaller than the fastest variant's own run-to-run std-dev (2.08 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64} vs {warm-clamp-min-lanes} (45% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64} and a slow tier {warm-clamp-min-lanes} with a 45% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.1x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.25 us but its worst 20% at 5.12 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-clamp-accfit-dyn's edge over baseline is significant but tiny (-46 ns, 1.77%)

warm-clamp-accfit-dyn differs from baseline warm-clamp-acc64 by -46 ns (1.77%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-minimum** at 2183.6 ns median (-16.8% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.74x (fastest 2183.6 ns, slowest 3800.4 ns)

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
| warm-clamp-acc64 | 2799ns | 2816ns | 2093ns | 2823ns | 3436ns | base |
| warm-clamp-accfit | 2683ns | 2608ns | 2397ns | 2644ns | 3087ns | -4.15% |
| warm-clamp-accfit-dyn | 2892ns | 2798ns | 2109ns | 2878ns | 3719ns | +3.33% |
| warm-clamp-head | 2789ns | 2865ns | 2165ns | 2850ns | 3228ns | -0.38% |
| warm-clamp-min-lanes | 3606ns | 3962ns | 1299ns | 3781ns | 5387ns | +28.81% |
| warm-clamp-minimum | 3489ns | 2245ns | 1974ns | 2785ns | 7113ns | +24.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2595ns | 1949ns | 3159ns | base | 3.157 |
| warm-clamp-accfit | 2488ns | 2230ns | 2836ns | -4.09% | 3.292 |
| warm-clamp-accfit-dyn | 2678ns | 1956ns | 3436ns | +3.20% | 3.059 |
| warm-clamp-head | 2513ns | 1957ns | 2884ns | -3.15% | 3.260 |
| warm-clamp-min-lanes | 3446ns | 1245ns | 5124ns | +32.82% | 2.377 |
| warm-clamp-minimum | 3386ns | 1917ns | 6881ns | +30.50% | 2.420 |

## Performance model

- Peak throughput: **6.579 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.121 | 47.4% |
| warm-clamp-accfit | 3.375 | 51.3% |
| warm-clamp-accfit-dyn | 3.149 | 47.9% |
| warm-clamp-head | 3.161 | 48.0% |
| warm-clamp-min-lanes | 2.156 | 32.8% |
| warm-clamp-minimum | 3.752 | 57.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 2799ns | 2799ns | base |
| warm-clamp-accfit | 2683ns | 2683ns | -4.15% |
| warm-clamp-accfit-dyn | 2892ns | 2892ns | +3.33% |
| warm-clamp-head | 2789ns | 2789ns | -0.38% |
| warm-clamp-min-lanes | 3606ns | 3606ns | +28.81% |
| warm-clamp-minimum | 3489ns | 3489ns | +24.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2625ns | base | --- | [2452, 2764] | --- | --- | --- | --- |
| warm-clamp-accfit | 2427ns | -163.9ns (-6.2%) | [-236, -46]ns | [2319, 2595] | YES | 0.0415 | 0.0166 | 0 |
| warm-clamp-accfit-dyn | 2601ns | -46.5ns (-1.8%) | [-164, -10]ns | [2437, 2868] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-clamp-head | 2592ns | no significant difference | [-81, +210]ns | [2395, 2706] | no | 0.6358 | 0.6358 | 0 |
| warm-clamp-min-lanes | 3800ns | +1227.8ns (+46.8%) | [+1088, +1333]ns | [3454, 4073] | YES | 0.0034 | 0.0007 | 0 |
| warm-clamp-minimum | 2184ns | no significant difference | [-423, +72]ns | [1986, 2357] | no | 0.1009 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2677ns | -8.8% | -14.9% | -2.3% | +51.9% | -11.9% |
| 1 | 2 | 2626ns | -5.0% | -4.9% | +8.5% | -52.4% | -24.1% |
| 1 | 3 | 2943ns | -21.4% | -6.3% | -32.5% | +38.5% | -34.9% |
| 1 | 4 | 2276ns | +12.9% | +38.7% | +15.0% | +59.3% | +66.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.499 | moderate- |
| warm-clamp-accfit | -0.639 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | 0.224 | moderate+ |
| warm-clamp-head | -0.480 | moderate- |
| warm-clamp-min-lanes | -0.541 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.162 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 1/4, lost 3/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 69.9ns | 2594.5ns | 2.7% |  |
| warm-clamp-accfit | 67.7ns | 2488.5ns | 2.7% |  |
| warm-clamp-accfit-dyn | 72.8ns | 2677.7ns | 2.7% |  |
| warm-clamp-head | 73.4ns | 2512.7ns | 2.9% |  |
| warm-clamp-min-lanes | 52.3ns | 3446.1ns | 1.5% |  |
| warm-clamp-minimum | 33.3ns | 3385.7ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1948.8-3158.9 ns)
   1948.8 |
   2009.3 |
   2069.8 |##########
   2130.3 |#####
   2190.8 |#####
   2251.3 |###############
   2311.8 |
   2372.3 |#####
   2432.8 |########################################
   2493.3 |
   2553.8 |##########
   2614.4 |###############
   2674.9 |###############
   2735.4 |
   2795.9 |#####
   2856.4 |#########################
   2916.9 |#####
   2977.4 |##########
   3037.9 |
   3098.4 |#####
  (2 below, 4 above range)

warm-clamp-accfit (n=40, range 2230.0-2836.4 ns)
   2230.0 |#################
   2260.3 |###########
   2290.6 |########################################
   2320.9 |#####
   2351.3 |
   2381.6 |###########
   2411.9 |######################
   2442.2 |#####
   2472.5 |#####
   2502.9 |#####
   2533.2 |#####
   2563.5 |#####
   2593.8 |#####
   2624.2 |######################
   2654.5 |#####
   2684.8 |#####
   2715.1 |
   2745.4 |
   2775.8 |#####
   2806.1 |#####
  (2 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 1956.3-3435.6 ns)
   1956.3 |
   2030.3 |####################
   2104.2 |##############################
   2178.2 |
   2252.2 |####################
   2326.1 |##############################
   2400.1 |##############################
   2474.1 |########################################
   2548.0 |####################
   2622.0 |
   2696.0 |##############################
   2769.9 |##########
   2843.9 |##############################
   2917.9 |
   2991.8 |##########
   3065.8 |
   3139.8 |########################################
   3213.7 |##########
   3287.7 |##########
   3361.7 |
  (2 below, 5 above range)

warm-clamp-head (n=40, range 1956.6-2884.2 ns)
   1956.6 |
   2003.0 |###############
   2049.3 |#####
   2095.7 |
   2142.1 |
   2188.5 |
   2234.9 |#####
   2281.2 |
   2327.6 |
   2374.0 |#########################
   2420.4 |#####
   2466.7 |
   2513.1 |
   2559.5 |##############################
   2605.9 |###############
   2652.3 |#####
   2698.6 |
   2745.0 |#####
   2791.4 |
   2837.8 |########################################
  (5 below, 5 above range)

warm-clamp-min-lanes (n=40, range 1245.1-5124.1 ns)
   1245.1 |#############
   1439.1 |
   1633.0 |
   1827.0 |
   2020.9 |######
   2214.8 |
   2408.8 |######
   2602.7 |
   2796.7 |
   2990.6 |
   3184.6 |######
   3378.5 |####################
   3572.5 |#################################
   3766.4 |########################################
   3960.4 |
   4154.3 |#################################
   4348.3 |######
   4542.2 |#############
   4736.2 |####################
   4930.1 |
  (7 below, 3 above range)

warm-clamp-minimum (n=40, range 1917.3-6881.4 ns)
   1917.3 |########################################
   2165.5 |###########################
   2413.7 |
   2661.9 |
   2910.2 |
   3158.4 |
   3406.6 |
   3654.8 |
   3903.0 |
   4151.2 |
   4399.4 |
   4647.6 |
   4895.8 |
   5144.0 |
   5392.2 |###
   5640.4 |#########
   5888.6 |###
   6136.8 |######
   6385.0 |###
   6633.2 |###
  (6 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit-dyn**: CV=20.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=40.0% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=61.4% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
