# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.07 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-minimum at 2.12 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-minimum is fastest but the noisiest (CV 112.1%)

warm-clamp-minimum wins on median (2.12 us) yet has the highest variance (CV 112.1%), while warm-clamp-accfit-dyn is the steadiest (CV 20.8%, 2.66 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-accfit shows alternating (throttle bounce) (autocorr -0.73)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (953 ns) is smaller than the fastest variant's own run-to-run std-dev (2.38 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-acc64 is inconsistent: worst-20% is 5.4x its best-20%

warm-clamp-acc64's best 20% of batches run at 1.03 us but its worst 20% at 5.55 us (5.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 2120.7 ns median (-31.0% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.45x (fastest 2120.7 ns, slowest 3073.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112729874 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3010ns | 3227ns | 1086ns | 2692ns | 5888ns | base |
| warm-clamp-accfit | 2691ns | 2836ns | 1484ns | 2819ns | 3517ns | -10.59% |
| warm-clamp-accfit-dyn | 2753ns | 2848ns | 1896ns | 2816ns | 3422ns | -8.54% |
| warm-clamp-head | 2668ns | 2772ns | 1843ns | 2710ns | 3368ns | -11.37% |
| warm-clamp-min-lanes | 4006ns | 2392ns | 1958ns | 3353ns | 8015ns | +33.10% |
| warm-clamp-minimum | 3840ns | 2178ns | 1956ns | 3159ns | 7768ns | +27.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2854ns | 1034ns | 5555ns | base | 2.870 |
| warm-clamp-accfit | 2502ns | 1385ns | 3258ns | -12.33% | 3.274 |
| warm-clamp-accfit-dyn | 2560ns | 1771ns | 3168ns | -10.31% | 3.200 |
| warm-clamp-head | 2438ns | 1692ns | 3058ns | -14.58% | 3.360 |
| warm-clamp-min-lanes | 3890ns | 1906ns | 7763ns | +36.28% | 2.106 |
| warm-clamp-minimum | 3727ns | 1904ns | 7506ns | +30.59% | 2.198 |

## Performance model

- Peak throughput: **7.922 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.665 | 33.6% |
| warm-clamp-accfit | 3.096 | 39.1% |
| warm-clamp-accfit-dyn | 3.083 | 38.9% |
| warm-clamp-head | 3.219 | 40.6% |
| warm-clamp-min-lanes | 3.518 | 44.4% |
| warm-clamp-minimum | 3.863 | 48.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3010ns | 3010ns | base |
| warm-clamp-accfit | 2691ns | 2691ns | -10.59% |
| warm-clamp-accfit-dyn | 2753ns | 2753ns | -8.54% |
| warm-clamp-head | 2668ns | 2668ns | -11.37% |
| warm-clamp-min-lanes | 4006ns | 4006ns | +33.10% |
| warm-clamp-minimum | 3840ns | 3840ns | +27.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3074ns | base | --- | [1261, 3621] | --- | --- | --- | --- |
| warm-clamp-accfit | 2646ns | no significant difference | [-1017, +604]ns | [2464, 2721] | no | 0.2564 | 0.1539 | 0 |
| warm-clamp-accfit-dyn | 2657ns | no significant difference | [-1030, +1022]ns | [2477, 2726] | no | 0.4296 | 0.4296 | 0 |
| warm-clamp-head | 2545ns | no significant difference | [-1386, +931]ns | [2316, 2608] | no | 0.4296 | 0.4296 | 0 |
| warm-clamp-min-lanes | 2328ns | +1071.1ns (+34.8%) | [+647, +1296]ns | [2327, 4995] | YES | 0.0005 | 0.0002 | 0 |
| warm-clamp-minimum | 2121ns | +873.5ns (+28.4%) | [+708, +1067]ns | [1907, 3907] | YES | 0.0002 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1036ns | +182.1% | +169.7% | +173.0% | +124.8% | +84.3% |
| 1 | 2 | 3080ns | -17.9% | -5.3% | -18.3% | -24.0% | -6.3% |
| 1 | 3 | 1267ns | +131.2% | +98.2% | +107.1% | +173.2% | +50.5% |
| 1 | 4 | 3371ns | -26.2% | -27.6% | -37.2% | -43.3% | -43.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.672 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.726 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | 0.184 | ok |
| warm-clamp-head | -0.164 | ok |
| warm-clamp-min-lanes | -0.529 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.419 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 2/4, lost 2/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 53.4ns | 2854.3ns | 1.9% |  |
| warm-clamp-accfit | 67.0ns | 2502.3ns | 2.7% |  |
| warm-clamp-accfit-dyn | 67.9ns | 2560.0ns | 2.7% |  |
| warm-clamp-head | 82.1ns | 2438.1ns | 3.4% |  |
| warm-clamp-min-lanes | 39.4ns | 3890.0ns | 1.0% |  |
| warm-clamp-minimum | 36.9ns | 3727.4ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1034.1-5554.9 ns)
   1034.1 |############################
   1260.1 |########################################
   1486.2 |#####
   1712.2 |
   1938.2 |
   2164.3 |
   2390.3 |
   2616.4 |
   2842.4 |#####
   3068.4 |######################
   3294.5 |#####
   3520.5 |###########
   3746.6 |#################
   3972.6 |
   4198.6 |
   4424.7 |###########
   4650.7 |#####
   4876.7 |
   5102.8 |#################
   5328.8 |
  (6 below, 4 above range)

warm-clamp-accfit (n=40, range 1384.6-3257.9 ns)
   1384.6 |
   1478.2 |
   1571.9 |
   1665.6 |
   1759.2 |
   1852.9 |
   1946.6 |
   2040.2 |
   2133.9 |
   2227.6 |#################################
   2321.2 |
   2414.9 |#################################
   2508.6 |##########################
   2602.2 |########################################
   2695.9 |#############
   2789.6 |######
   2883.2 |##########################
   2976.9 |#############
   3070.6 |
   3164.2 |##########################
  (5 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1771.1-3168.2 ns)
   1771.1 |########
   1840.9 |
   1910.8 |
   1980.6 |
   2050.5 |
   2120.4 |
   2190.2 |
   2260.1 |########################################
   2329.9 |################
   2399.8 |########################
   2469.6 |########################
   2539.5 |########
   2609.4 |########################################
   2679.2 |################################
   2749.1 |################
   2818.9 |########
   2888.8 |################
   2958.6 |########################
   3028.5 |
   3098.4 |
  (3 below, 5 above range)

warm-clamp-head (n=40, range 1692.2-3058.3 ns)
   1692.2 |
   1760.5 |
   1828.8 |
   1897.1 |####################
   1965.4 |
   2033.8 |
   2102.1 |#########################
   2170.4 |#####
   2238.7 |
   2307.0 |####################
   2375.3 |
   2443.6 |#####
   2511.9 |########################################
   2580.2 |####################
   2648.5 |#####
   2716.8 |
   2785.1 |####################
   2853.4 |##########
   2921.7 |
   2990.0 |
  (2 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1906.0-7763.1 ns)
   1906.0 |############################
   2198.8 |########################################
   2491.7 |
   2784.5 |
   3077.4 |
   3370.3 |
   3663.1 |
   3956.0 |
   4248.8 |
   4541.7 |
   4834.5 |#####
   5127.4 |
   5420.2 |##
   5713.1 |##
   6006.0 |#####
   6298.8 |##
   6591.7 |##
   6884.5 |#####
   7177.4 |
   7470.2 |#####
  (1 below, 3 above range)

warm-clamp-minimum (n=40, range 1904.2-7506.3 ns)
   1904.2 |########################################
   2184.3 |###########
   2464.4 |
   2744.5 |
   3024.6 |
   3304.7 |##
   3584.8 |
   3864.9 |
   4145.0 |##
   4425.1 |
   4705.2 |
   4985.4 |
   5265.5 |
   5545.6 |##
   5825.7 |##
   6105.8 |
   6385.9 |##
   6666.0 |####
   6946.1 |####
   7226.2 |####
  (2 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=62.0% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 5.4x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=28.5% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: CV=21.6% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=21.9% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=61.3% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=63.8% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
