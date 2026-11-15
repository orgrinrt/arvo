# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.60 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 2.34 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-accfit-dyn beats baseline by 30% (significant)

warm-clamp-accfit-dyn is -1.08 us (30%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-head is fastest but the noisiest (CV 112.0%)

warm-clamp-head wins on median (2.34 us) yet has the highest variance (CV 112.0%), while warm-clamp-accfit is the steadiest (CV 8.5%, 2.92 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.75)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.26 us) is smaller than the fastest variant's own run-to-run std-dev (2.63 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-acc64 is inconsistent: worst-20% is 4.0x its best-20%

warm-clamp-acc64's best 20% of batches run at 1.06 us but its worst 20% at 4.26 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 2344.1 ns median (-34.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.54x (fastest 2344.1 ns, slowest 3599.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3464ns | 3836ns | 1132ns | 3871ns | 4578ns | base |
| warm-clamp-accfit | 3158ns | 3147ns | 2778ns | 3157ns | 3540ns | -8.85% |
| warm-clamp-accfit-dyn | 2838ns | 2745ns | 2454ns | 2765ns | 3442ns | -18.07% |
| warm-clamp-head | 4359ns | 2407ns | 2396ns | 3547ns | 8758ns | +25.82% |
| warm-clamp-min-lanes | 4186ns | 2455ns | 2008ns | 3658ns | 7948ns | +20.83% |
| warm-clamp-minimum | 4269ns | 2613ns | 2457ns | 3463ns | 8501ns | +23.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3242ns | 1064ns | 4256ns | base | 2.527 |
| warm-clamp-accfit | 2922ns | 2578ns | 3243ns | -9.87% | 2.804 |
| warm-clamp-accfit-dyn | 2583ns | 2243ns | 3102ns | -20.32% | 3.172 |
| warm-clamp-head | 4233ns | 2333ns | 8472ns | +30.58% | 1.935 |
| warm-clamp-min-lanes | 4065ns | 1956ns | 7683ns | +25.40% | 2.015 |
| warm-clamp-minimum | 4131ns | 2393ns | 8192ns | +27.42% | 1.983 |

## Performance model

- Peak throughput: **7.698 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.276 | 29.6% |
| warm-clamp-accfit | 2.802 | 36.4% |
| warm-clamp-accfit-dyn | 3.265 | 42.4% |
| warm-clamp-head | 3.495 | 45.4% |
| warm-clamp-min-lanes | 3.424 | 44.5% |
| warm-clamp-minimum | 3.264 | 42.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3464ns | 3464ns | base |
| warm-clamp-accfit | 3158ns | 3158ns | -8.85% |
| warm-clamp-accfit-dyn | 2838ns | 2838ns | -18.07% |
| warm-clamp-head | 4359ns | 4359ns | +25.82% |
| warm-clamp-min-lanes | 4186ns | 4186ns | +20.83% |
| warm-clamp-minimum | 4269ns | 4269ns | +23.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3599ns | base | --- | [3471, 3892] | --- | --- | --- | --- |
| warm-clamp-accfit | 2923ns | -673.3ns (-18.7%) | [-873, -423]ns | [2801, 3048] | YES | 0.0017 | 0.0007 | 0 |
| warm-clamp-accfit-dyn | 2509ns | -1080.5ns (-30.0%) | [-1272, -638]ns | [2496, 2559] | YES | 0.0009 | 0.0002 | 0 |
| warm-clamp-head | 2344ns | no significant difference | [-971, +1554]ns | [2334, 4293] | no | 0.7948 | 0.6358 | 0 |
| warm-clamp-min-lanes | 2392ns | no significant difference | [-742, +2184]ns | [1966, 5970] | no | 0.2564 | 0.1539 | 0 |
| warm-clamp-minimum | 2510ns | no significant difference | [-961, +1429]ns | [2401, 3228] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3471ns | -9.7% | -17.5% | -32.6% | -43.4% | -21.4% |
| 1 | 2 | 3521ns | -15.7% | -28.9% | -33.7% | +19.0% | -31.8% |
| 1 | 3 | 3681ns | -25.7% | -35.9% | -36.3% | -46.7% | -31.6% |
| 1 | 4 | 3912ns | -24.2% | -36.0% | -30.0% | -21.7% | -38.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.227 | moderate+ |
| warm-clamp-accfit | -0.045 | ok |
| warm-clamp-accfit-dyn | 0.034 | ok |
| warm-clamp-head | -0.064 | ok |
| warm-clamp-min-lanes | -0.748 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.353 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 79.1ns | 3241.7ns | 2.4% |  |
| warm-clamp-accfit | 81.8ns | 2921.6ns | 2.8% |  |
| warm-clamp-accfit-dyn | 88.8ns | 2582.8ns | 3.4% |  |
| warm-clamp-head | 43.0ns | 4233.1ns | 1.0% |  |
| warm-clamp-min-lanes | 40.8ns | 4065.2ns | 1.0% |  |
| warm-clamp-minimum | 45.9ns | 4130.5ns | 1.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1064.2-4256.2 ns)
   1064.2 |####
   1223.8 |
   1383.4 |####
   1543.0 |
   1702.6 |
   1862.2 |
   2021.8 |
   2181.4 |
   2341.0 |
   2500.6 |
   2660.2 |####
   2819.8 |
   2979.4 |
   3139.0 |########
   3298.6 |####
   3458.2 |########################################
   3617.8 |########
   3777.4 |#################
   3937.0 |##########################
   4096.6 |#############
  (6 below, 4 above range)

warm-clamp-accfit (n=40, range 2578.2-3243.4 ns)
   2578.2 |################
   2611.5 |
   2644.7 |
   2678.0 |########
   2711.2 |########################
   2744.5 |########
   2777.8 |########################
   2811.0 |################
   2844.3 |########
   2877.5 |################
   2910.8 |########
   2944.1 |
   2977.3 |################
   3010.6 |########
   3043.8 |########################
   3077.1 |########
   3110.4 |########################################
   3143.6 |########################
   3176.9 |################
   3210.1 |
  (5 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 2243.2-3102.3 ns)
   2243.2 |#############
   2286.1 |#################
   2329.1 |########
   2372.0 |
   2415.0 |
   2458.0 |#############
   2500.9 |########################################
   2543.9 |######################
   2586.8 |####
   2629.8 |####
   2672.7 |####
   2715.7 |
   2758.7 |########
   2801.6 |########
   2844.6 |
   2887.5 |####
   2930.5 |
   2973.4 |
   3016.4 |
   3059.4 |
  (2 below, 4 above range)

warm-clamp-head (n=40, range 2332.9-8471.9 ns)
   2332.9 |########################################
   2639.8 |
   2946.8 |#
   3253.7 |
   3560.7 |
   3867.7 |
   4174.6 |
   4481.6 |
   4788.5 |
   5095.5 |
   5402.4 |###
   5709.4 |
   6016.3 |
   6323.3 |
   6630.2 |
   6937.2 |###
   7244.1 |#
   7551.1 |#
   7858.0 |###
   8165.0 |#####
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1956.4-7683.1 ns)
   1956.4 |########################################
   2242.7 |###############
   2529.0 |###
   2815.4 |
   3101.7 |
   3388.1 |
   3674.4 |
   3960.7 |######
   4247.1 |
   4533.4 |
   4819.7 |
   5106.1 |
   5392.4 |
   5678.7 |
   5965.1 |#########
   6251.4 |###
   6537.8 |######
   6824.1 |######
   7110.4 |#########
   7396.8 |
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 2393.5-8191.7 ns)
   2393.5 |########################################
   2683.4 |
   2973.3 |#
   3263.2 |#
   3553.1 |
   3843.0 |
   4132.9 |
   4422.9 |
   4712.8 |#
   5002.7 |
   5292.6 |
   5582.5 |
   5872.4 |
   6162.3 |
   6452.2 |###
   6742.2 |
   7032.1 |###
   7322.0 |
   7611.9 |#####
   7901.8 |#
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=35.3% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-head**: CV=62.0% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=59.9% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=59.3% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.4x (possible bimodal distribution)
