# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (4.05 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-accfit at 2.34 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-accfit is fastest but the noisiest (CV 102.2%)

warm-clamp-accfit wins on median (2.34 us) yet has the highest variance (CV 102.2%), while warm-clamp-head is the steadiest (CV 24.4%, 3.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-clamp-accfit, warm-clamp-minimum) are a dead heat (<1%)

warm-clamp-accfit (2.34 us) and warm-clamp-minimum (2.35 us) differ by 0.05%, inside the noise, even though the wider field spreads 72.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.79)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.71 us) is smaller than the fastest variant's own run-to-run std-dev (2.40 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-clamp-accfit, warm-clamp-minimum, warm-clamp-accfit-dyn, warm-clamp-min-lanes} vs {warm-clamp-head, warm-clamp-acc64} (39% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-minimum, warm-clamp-accfit-dyn, warm-clamp-min-lanes} and a slow tier {warm-clamp-head, warm-clamp-acc64} with a 39% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-acc64 is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-acc64's best 20% of batches run at 1.30 us but its worst 20% at 5.03 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit** at 2344.2 ns median (-42.1% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 1.73x (fastest 2344.2 ns, slowest 4050.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3501ns | 4240ns | 1358ns | 3612ns | 5310ns | base |
| warm-clamp-accfit | 4116ns | 2408ns | 2180ns | 3392ns | 8221ns | +17.56% |
| warm-clamp-accfit-dyn | 4288ns | 2492ns | 2062ns | 3770ns | 8068ns | +22.48% |
| warm-clamp-head | 3515ns | 3702ns | 2123ns | 3635ns | 4547ns | +0.40% |
| warm-clamp-min-lanes | 4385ns | 2591ns | 2044ns | 3967ns | 7981ns | +25.27% |
| warm-clamp-minimum | 4084ns | 2409ns | 1967ns | 3615ns | 7606ns | +16.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3335ns | 1296ns | 5034ns | base | 2.457 |
| warm-clamp-accfit | 3994ns | 2123ns | 7951ns | +19.79% | 2.051 |
| warm-clamp-accfit-dyn | 4161ns | 2008ns | 7788ns | +24.79% | 1.969 |
| warm-clamp-head | 3319ns | 2011ns | 4268ns | -0.48% | 2.469 |
| warm-clamp-min-lanes | 4260ns | 1991ns | 7719ns | +27.75% | 1.923 |
| warm-clamp-minimum | 3965ns | 1916ns | 7362ns | +18.90% | 2.066 |

## Performance model

- Peak throughput: **6.321 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.023 | 32.0% |
| warm-clamp-accfit | 3.495 | 55.3% |
| warm-clamp-accfit-dyn | 3.375 | 53.4% |
| warm-clamp-head | 2.336 | 37.0% |
| warm-clamp-min-lanes | 3.245 | 51.3% |
| warm-clamp-minimum | 3.493 | 55.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3501ns | 3501ns | base |
| warm-clamp-accfit | 4116ns | 4116ns | +17.56% |
| warm-clamp-accfit-dyn | 4288ns | 4288ns | +22.48% |
| warm-clamp-head | 3515ns | 3515ns | +0.40% |
| warm-clamp-min-lanes | 4385ns | 4385ns | +25.27% |
| warm-clamp-minimum | 4084ns | 4084ns | +16.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4050ns | base | --- | [1924, 4499] | --- | --- | --- | --- |
| warm-clamp-accfit | 2344ns | +962.1ns (+23.8%) | [+536, +1034]ns | [2337, 4139] | YES | 0.0080 | 0.0064 | 0 |
| warm-clamp-accfit-dyn | 2427ns | +961.8ns (+23.7%) | [+554, +1131]ns | [2343, 5360] | YES | 0.0017 | 0.0007 | 0 |
| warm-clamp-head | 3507ns | -489.3ns (-12.1%) | [-608, -189]ns | [3039, 3835] | YES | 0.0385 | 0.0385 | 0 |
| warm-clamp-min-lanes | 2525ns | +1013.8ns (+25.0%) | [+584, +1175]ns | [2370, 5800] | YES | 0.0002 | 0.0000 | 0 |
| warm-clamp-minimum | 2346ns | +808.2ns (+20.0%) | [+566, +977]ns | [2339, 5154] | YES | 0.0080 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3743ns | -37.4% | -37.3% | -12.1% | -36.6% | -48.6% |
| 1 | 2 | 3201ns | +29.3% | -26.8% | +24.3% | +35.7% | -26.9% |
| 1 | 3 | 4420ns | -47.1% | +21.3% | -34.7% | +20.8% | +16.6% |
| 1 | 4 | 4413ns | -46.9% | -49.4% | -13.1% | -46.1% | -46.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.018 | ok |
| warm-clamp-accfit | -0.418 | moderate- |
| warm-clamp-accfit-dyn | -0.437 | moderate- |
| warm-clamp-head | -0.788 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.268 | moderate- |
| warm-clamp-minimum | -0.306 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 2/4, lost 2/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 58.5ns | 3334.6ns | 1.8% |  |
| warm-clamp-accfit | 40.6ns | 3994.5ns | 1.0% |  |
| warm-clamp-accfit-dyn | 41.9ns | 4161.1ns | 1.0% |  |
| warm-clamp-head | 68.0ns | 3318.5ns | 2.0% |  |
| warm-clamp-min-lanes | 43.4ns | 4260.1ns | 1.0% |  |
| warm-clamp-minimum | 40.3ns | 3965.0ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1296.1-5033.5 ns)
   1296.1 |########################################
   1482.9 |####
   1669.8 |####
   1856.7 |####
   2043.6 |
   2230.4 |
   2417.3 |
   2604.2 |
   2791.1 |####
   2977.9 |
   3164.8 |
   3351.7 |####
   3538.5 |####
   3725.4 |####
   3912.3 |####
   4099.2 |########
   4286.0 |############
   4472.9 |################
   4659.8 |####
   4846.7 |################################
  (2 below, 2 above range)

warm-clamp-accfit (n=40, range 2123.4-7951.3 ns)
   2123.4 |########################################
   2414.8 |##
   2706.2 |##
   2997.6 |
   3289.0 |
   3580.4 |
   3871.8 |
   4163.1 |
   4454.5 |
   4745.9 |
   5037.3 |
   5328.7 |####
   5620.1 |
   5911.5 |####
   6202.9 |
   6494.3 |####
   6785.7 |
   7077.1 |##
   7368.5 |####
   7659.9 |##
  (4 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 2008.2-7788.1 ns)
   2008.2 |##
   2297.2 |########################################
   2586.2 |
   2875.2 |
   3164.1 |
   3453.1 |
   3742.1 |
   4031.1 |##
   4320.1 |
   4609.1 |
   4898.1 |
   5187.1 |##########
   5476.1 |##
   5765.1 |########
   6054.1 |##
   6343.1 |#####
   6632.1 |##
   6921.1 |
   7210.1 |
   7499.1 |##
  (6 below, 4 above range)

warm-clamp-head (n=40, range 2011.3-4268.3 ns)
   2011.3 |
   2124.2 |
   2237.0 |
   2349.9 |
   2462.7 |######
   2575.6 |######
   2688.4 |
   2801.3 |####################
   2914.1 |##########################
   3027.0 |####################
   3139.8 |######
   3252.7 |#############
   3365.5 |
   3478.4 |#################################
   3591.2 |######
   3704.0 |
   3816.9 |#################################
   3929.7 |
   4042.6 |######
   4155.4 |########################################
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1991.3-7718.7 ns)
   1991.3 |
   2277.7 |########################################
   2564.1 |###
   2850.4 |###
   3136.8 |
   3423.2 |
   3709.6 |
   3995.9 |
   4282.3 |
   4568.7 |
   4855.0 |
   5141.4 |######
   5427.8 |###
   5714.1 |############
   6000.5 |######
   6286.9 |
   6573.3 |######
   6859.6 |######
   7146.0 |
   7432.4 |###
  (7 below, 4 above range)

warm-clamp-minimum (n=40, range 1915.8-7361.9 ns)
   1915.8 |############
   2188.1 |########################################
   2460.4 |
   2732.7 |
   3005.0 |
   3277.3 |
   3549.6 |
   3821.9 |###
   4094.2 |
   4366.5 |
   4638.8 |###
   4911.1 |######
   5183.4 |#########
   5455.7 |###
   5728.0 |###
   6000.3 |###
   6272.6 |###
   6544.9 |######
   6817.2 |###
   7089.5 |###
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=46.8% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=60.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=56.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-head**: CV=25.8% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=54.5% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=54.6% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
