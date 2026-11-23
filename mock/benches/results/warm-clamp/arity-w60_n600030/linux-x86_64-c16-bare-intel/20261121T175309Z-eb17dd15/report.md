# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 22% faster than the next best (warm-clamp-minimum)

warm-clamp-min-lanes (1.98 us) leads warm-clamp-minimum (2.42 us) by 22%, a clear separation rather than a photo finish. CV 116.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes is fastest but the noisiest (CV 116.4%)

warm-clamp-min-lanes wins on median (1.98 us) yet has the highest variance (CV 116.4%), while warm-clamp-minimum is the steadiest (CV 12.5%, 2.42 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.79)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.07 us) is smaller than the fastest variant's own run-to-run std-dev (2.30 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-min-lanes is inconsistent: worst-20% is 3.8x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.88 us but its worst 20% at 7.21 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 1975.8 ns median (-26.8% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.54x (fastest 1975.8 ns, slowest 3044.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112891686 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2805ns | 2943ns | 1951ns | 2878ns | 3442ns | base |
| warm-clamp-accfit | 2715ns | 2723ns | 1813ns | 2776ns | 3430ns | -3.23% |
| warm-clamp-accfit-dyn | 2872ns | 2945ns | 2253ns | 2905ns | 3390ns | +2.36% |
| warm-clamp-head | 4082ns | 3127ns | 2383ns | 3271ns | 8217ns | +45.53% |
| warm-clamp-min-lanes | 3802ns | 2047ns | 1945ns | 3180ns | 7523ns | +35.53% |
| warm-clamp-minimum | 2715ns | 2637ns | 2358ns | 2648ns | 3271ns | -3.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2563ns | 1789ns | 3123ns | base | 3.196 |
| warm-clamp-accfit | 2478ns | 1664ns | 3112ns | -3.33% | 3.306 |
| warm-clamp-accfit-dyn | 2607ns | 2068ns | 3043ns | +1.72% | 3.142 |
| warm-clamp-head | 3957ns | 2305ns | 7940ns | +54.40% | 2.070 |
| warm-clamp-min-lanes | 3658ns | 1876ns | 7207ns | +42.72% | 2.239 |
| warm-clamp-minimum | 2479ns | 2164ns | 2976ns | -3.28% | 3.304 |

## Performance model

- Peak throughput: **4.924 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.034 | 61.6% |
| warm-clamp-accfit | 3.281 | 66.6% |
| warm-clamp-accfit-dyn | 3.031 | 61.6% |
| warm-clamp-head | 2.691 | 54.6% |
| warm-clamp-min-lanes | 4.146 | 84.2% |
| warm-clamp-minimum | 3.387 | 68.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 2805ns | 2805ns | base |
| warm-clamp-accfit | 2715ns | 2715ns | -3.23% |
| warm-clamp-accfit-dyn | 2872ns | 2872ns | +2.36% |
| warm-clamp-head | 4082ns | 4082ns | +45.53% |
| warm-clamp-min-lanes | 3802ns | 3802ns | +35.53% |
| warm-clamp-minimum | 2715ns | 2715ns | -3.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2700ns | base | --- | [2494, 2743] | --- | --- | --- | --- |
| warm-clamp-accfit | 2497ns | no significant difference | [-240, +309]ns | [2322, 2707] | no | 1.0000 | 0.6358 | 0 |
| warm-clamp-accfit-dyn | 2703ns | no significant difference | [-241, +303]ns | [2506, 2724] | no | 1.0000 | 0.8746 | 0 |
| warm-clamp-head | 3044ns | +276.6ns (+10.2%) | [+101, +551]ns | [3033, 3057] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1976ns | no significant difference | [-519, +1626]ns | [1973, 4438] | no | 1.0000 | 0.6358 | 0 |
| warm-clamp-minimum | 2419ns | no significant difference | [-263, +153]ns | [2315, 2497] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2947ns | -25.9% | -8.2% | +3.5% | -36.1% | -23.5% |
| 1 | 2 | 2176ns | +28.6% | +24.3% | +3.3% | -4.1% | +8.3% |
| 1 | 3 | 2700ns | -7.5% | -12.4% | +12.7% | -23.2% | -11.9% |
| 1 | 4 | 2704ns | +9.3% | +2.5% | -7.6% | -30.4% | +6.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.536 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.403 | moderate- |
| warm-clamp-accfit-dyn | -0.500 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.787 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.250 | moderate- |
| warm-clamp-minimum | -0.013 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 70.0ns | 2563.2ns | 2.7% |  |
| warm-clamp-accfit | 67.8ns | 2477.8ns | 2.7% |  |
| warm-clamp-accfit-dyn | 72.4ns | 2607.1ns | 2.8% |  |
| warm-clamp-head | 34.5ns | 3957.5ns | 0.9% |  |
| warm-clamp-min-lanes | 37.7ns | 3658.3ns | 1.0% |  |
| warm-clamp-minimum | 70.1ns | 2479.2ns | 2.8% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1788.9-3123.4 ns)
   1788.9 |
   1855.6 |
   1922.3 |
   1989.1 |####
   2055.8 |####
   2122.5 |#################
   2189.2 |
   2256.0 |########
   2322.7 |####
   2389.4 |########
   2456.1 |#############
   2522.9 |
   2589.6 |
   2656.3 |########################################
   2723.0 |#################
   2789.8 |####
   2856.5 |####
   2923.2 |######################
   2989.9 |####
   3056.7 |
  (2 below, 3 above range)

warm-clamp-accfit (n=40, range 1663.5-3111.6 ns)
   1663.5 |
   1736.0 |
   1808.4 |
   1880.8 |
   1953.2 |
   2025.6 |########################
   2098.0 |########################
   2170.4 |########
   2242.8 |########
   2315.2 |########################################
   2387.6 |################
   2460.0 |########################
   2532.4 |################
   2604.8 |
   2677.2 |########################################
   2749.6 |
   2822.0 |########
   2894.4 |################################
   2966.8 |################
   3039.2 |########
  (3 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 2067.6-3042.8 ns)
   2067.6 |
   2116.4 |
   2165.1 |####
   2213.9 |####
   2262.7 |
   2311.4 |#############
   2360.2 |########
   2408.9 |####
   2457.7 |#################
   2506.5 |#################
   2555.2 |
   2604.0 |####
   2652.8 |
   2701.5 |########################################
   2750.3 |
   2799.0 |####
   2847.8 |########
   2896.6 |
   2945.3 |##########################
   2994.1 |
  (2 below, 3 above range)

warm-clamp-head (n=40, range 2304.8-7939.6 ns)
   2304.8 |##############
   2586.6 |
   2868.3 |########################################
   3150.0 |
   3431.8 |##
   3713.5 |
   3995.2 |
   4277.0 |
   4558.7 |
   4840.5 |##
   5122.2 |
   5403.9 |
   5685.7 |
   5967.4 |####
   6249.2 |
   6530.9 |##
   6812.6 |
   7094.4 |##
   7376.1 |
   7657.8 |##
  (6 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1875.6-7207.1 ns)
   1875.6 |########################################
   2142.2 |####
   2408.8 |
   2675.3 |
   2941.9 |
   3208.5 |
   3475.1 |
   3741.6 |
   4008.2 |##
   4274.8 |
   4541.4 |
   4808.0 |##
   5074.5 |
   5341.1 |
   5607.7 |##
   5874.3 |
   6140.8 |####
   6407.4 |######
   6674.0 |####
   6940.6 |##
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 2164.4-2976.0 ns)
   2164.4 |####################
   2205.0 |#####
   2245.6 |
   2286.1 |########################################
   2326.7 |#####
   2367.3 |#####
   2407.9 |##########
   2448.5 |
   2489.0 |##############################
   2529.6 |#####
   2570.2 |
   2610.8 |#####
   2651.4 |#####
   2691.9 |##########
   2732.5 |#####
   2773.1 |#####
   2813.7 |
   2854.3 |
   2894.8 |
   2935.4 |##########
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=20.8% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: CV=22.9% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=54.7% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=62.8% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.8x (possible bimodal distribution)
