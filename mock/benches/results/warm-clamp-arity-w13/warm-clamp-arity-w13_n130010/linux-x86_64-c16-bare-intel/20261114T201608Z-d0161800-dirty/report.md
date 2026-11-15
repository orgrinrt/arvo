# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (4.46 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 3.21 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head dominates: 11% faster than the next best (warm-clamp-minimum)

warm-clamp-head (3.21 us) leads warm-clamp-minimum (3.56 us) by 11%, a clear separation rather than a photo finish. CV 11.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum beats baseline by 21% (significant)

warm-clamp-minimum is -923 ns (21%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.55)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-acc64 is inconsistent: worst-20% is 4.3x its best-20%

warm-clamp-acc64's best 20% of batches run at 1.44 us but its worst 20% at 6.26 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 3207.2 ns median (-28.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.39x (fastest 3207.2 ns, slowest 4459.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4010ns | 4662ns | 1505ns | 3998ns | 6555ns | base |
| warm-clamp-accfit | 3580ns | 3920ns | 1719ns | 3902ns | 4475ns | -10.74% |
| warm-clamp-accfit-dyn | 3425ns | 3826ns | 1202ns | 3739ns | 4707ns | -14.59% |
| warm-clamp-head | 3335ns | 3431ns | 2793ns | 3323ns | 3912ns | -16.85% |
| warm-clamp-min-lanes | 3819ns | 4370ns | 1418ns | 3968ns | 5775ns | -4.76% |
| warm-clamp-minimum | 3462ns | 3755ns | 1265ns | 3766ns | 4747ns | -13.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3828ns | 1440ns | 6259ns | base | 2.140 |
| warm-clamp-accfit | 3370ns | 1627ns | 4167ns | -11.96% | 2.431 |
| warm-clamp-accfit-dyn | 3234ns | 1139ns | 4412ns | -15.51% | 2.533 |
| warm-clamp-head | 3098ns | 2604ns | 3595ns | -19.06% | 2.644 |
| warm-clamp-min-lanes | 3648ns | 1359ns | 5493ns | -4.68% | 2.245 |
| warm-clamp-minimum | 3263ns | 1196ns | 4432ns | -14.74% | 2.510 |

## Performance model

- Peak throughput: **7.194 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 1.837 | 25.5% |
| warm-clamp-accfit | 2.212 | 30.7% |
| warm-clamp-accfit-dyn | 2.259 | 31.4% |
| warm-clamp-head | 2.554 | 35.5% |
| warm-clamp-min-lanes | 1.955 | 27.2% |
| warm-clamp-minimum | 2.304 | 32.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 4010ns | 4010ns | base |
| warm-clamp-accfit | 3580ns | 3580ns | -10.74% |
| warm-clamp-accfit-dyn | 3425ns | 3425ns | -14.59% |
| warm-clamp-head | 3335ns | 3335ns | -16.85% |
| warm-clamp-min-lanes | 3819ns | 3819ns | -4.76% |
| warm-clamp-minimum | 3462ns | 3462ns | -13.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4459ns | base | --- | [2477, 4878] | --- | --- | --- | --- |
| warm-clamp-accfit | 3703ns | -876.1ns (-19.6%) | [-1009, -303]ns | [3413, 3856] | YES | 0.0276 | 0.0166 | 0 |
| warm-clamp-accfit-dyn | 3627ns | -808.9ns (-18.1%) | [-1124, -297]ns | [3349, 3871] | YES | 0.0111 | 0.0022 | 0 |
| warm-clamp-head | 3207ns | no significant difference | [-1632, +224]ns | [2939, 3235] | no | 0.4296 | 0.4296 | 0 |
| warm-clamp-min-lanes | 4191ns | -224.9ns (-5.0%) | [-546, -54]ns | [3437, 4699] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-clamp-minimum | 3555ns | -922.7ns (-20.7%) | [-1269, -349]ns | [3176, 4086] | YES | 0.0161 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4534ns | -18.1% | -18.7% | -35.3% | -7.6% | -24.7% |
| 1 | 2 | 3900ns | -7.4% | -50.2% | -15.2% | -6.0% | +5.2% |
| 1 | 3 | 4726ns | -22.2% | -25.7% | -31.6% | -40.2% | -21.3% |
| 1 | 4 | 3247ns | +20.1% | +15.2% | -8.4% | +49.2% | +8.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.554 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.032 | ok |
| warm-clamp-accfit-dyn | -0.371 | moderate- |
| warm-clamp-head | -0.268 | moderate- |
| warm-clamp-min-lanes | -0.391 | moderate- |
| warm-clamp-minimum | -0.397 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 61.7ns | 3827.6ns | 1.6% |  |
| warm-clamp-accfit | 70.0ns | 3369.7ns | 2.1% |  |
| warm-clamp-accfit-dyn | 66.9ns | 3233.8ns | 2.1% |  |
| warm-clamp-head | 82.7ns | 3097.9ns | 2.7% |  |
| warm-clamp-min-lanes | 58.2ns | 3648.5ns | 1.6% |  |
| warm-clamp-minimum | 67.4ns | 3263.3ns | 2.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1439.8-6258.9 ns)
   1439.8 |################################
   1680.8 |
   1921.7 |
   2162.7 |
   2403.6 |########################################
   2644.6 |
   2885.5 |
   3126.5 |########
   3367.5 |
   3608.4 |
   3849.4 |########
   4090.3 |########
   4331.3 |################
   4572.2 |################################
   4813.2 |################################
   5054.1 |################################
   5295.1 |########
   5536.0 |########
   5777.0 |
   6018.0 |########
  (7 below, 4 above range)

warm-clamp-accfit (n=40, range 1627.2-4167.2 ns)
   1627.2 |
   1754.2 |
   1881.2 |
   2008.2 |
   2135.2 |
   2262.2 |
   2389.2 |
   2516.2 |
   2643.2 |
   2770.2 |###
   2897.2 |
   3024.2 |
   3151.2 |###
   3278.2 |#############
   3405.2 |################
   3532.2 |
   3659.2 |##########################
   3786.2 |###
   3913.2 |###
   4040.2 |########################################
  (6 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 1138.8-4411.9 ns)
   1138.8 |##########
   1302.4 |
   1466.1 |
   1629.7 |#####
   1793.4 |
   1957.1 |
   2120.7 |
   2284.4 |
   2448.0 |
   2611.7 |#####
   2775.3 |
   2939.0 |
   3102.6 |##########
   3266.3 |#########################
   3429.9 |##########
   3593.6 |####################
   3757.2 |####################
   3920.9 |#####
   4084.6 |########################################
   4248.2 |#####
  (6 below, 3 above range)

warm-clamp-head (n=40, range 2604.2-3594.7 ns)
   2604.2 |
   2653.7 |############################
   2703.2 |
   2752.8 |
   2802.3 |####
   2851.8 |
   2901.3 |########################
   2950.9 |
   3000.4 |########
   3049.9 |####
   3099.5 |
   3149.0 |####
   3198.5 |########################################
   3248.0 |########
   3297.6 |############
   3347.1 |
   3396.6 |
   3446.1 |
   3495.7 |
   3545.2 |########
  (2 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1359.4-5492.6 ns)
   1359.4 |########################################
   1566.0 |
   1772.7 |
   1979.3 |
   2186.0 |
   2392.7 |
   2599.3 |
   2806.0 |
   3012.6 |####
   3219.3 |
   3426.0 |
   3632.6 |
   3839.3 |########
   4045.9 |######################
   4252.6 |########
   4459.3 |#############
   4665.9 |########
   4872.6 |########
   5079.2 |#############
   5285.9 |######################
  (4 below, 2 above range)

warm-clamp-minimum (n=40, range 1196.1-4432.4 ns)
   1196.1 |
   1357.9 |
   1519.7 |#####
   1681.5 |
   1843.3 |
   2005.1 |
   2167.0 |
   2328.8 |#####
   2490.6 |
   2652.4 |
   2814.2 |#####
   2976.0 |###########
   3137.8 |#################
   3299.7 |############################
   3461.5 |
   3623.3 |#################
   3785.1 |###########
   3946.9 |######################
   4108.7 |########################################
   4270.5 |
  (7 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=48.1% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=29.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: CV=36.5% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=45.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=35.4% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
