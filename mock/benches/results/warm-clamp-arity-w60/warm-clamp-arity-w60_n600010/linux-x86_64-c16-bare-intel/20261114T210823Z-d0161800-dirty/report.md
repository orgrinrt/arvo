# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 24% faster than the next best (warm-clamp-head)

warm-clamp-min-lanes (2.46 us) leads warm-clamp-head (3.06 us) by 24%, a clear separation rather than a photo finish. CV 100.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes beats baseline by 20% (significant)

warm-clamp-min-lanes is -630 ns (20%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is fastest but the noisiest (CV 100.7%)

warm-clamp-min-lanes wins on median (2.46 us) yet has the highest variance (CV 100.7%), while warm-clamp-accfit-dyn is the steadiest (CV 81.3%, 3.10 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (653 ns) is smaller than the fastest variant's own run-to-run std-dev (2.48 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-head is inconsistent: worst-20% is 3.7x its best-20%

warm-clamp-head's best 20% of batches run at 2.42 us but its worst 20% at 8.99 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-clamp-accfit's edge over baseline is significant but tiny (12 ns, 0.38%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by 12 ns (0.38%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 2463.3 ns median (-20.5% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.26x (fastest 2463.3 ns, slowest 3115.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4561ns | 3185ns | 2628ns | 3518ns | 9622ns | base |
| warm-clamp-accfit | 4578ns | 3202ns | 2765ns | 3537ns | 9515ns | +0.39% |
| warm-clamp-accfit-dyn | 4361ns | 3186ns | 2572ns | 3303ns | 9324ns | -4.38% |
| warm-clamp-head | 4494ns | 3147ns | 2492ns | 3558ns | 9305ns | -1.45% |
| warm-clamp-min-lanes | 4107ns | 2550ns | 2371ns | 3160ns | 8684ns | -9.94% |
| warm-clamp-minimum | 4477ns | 3183ns | 2611ns | 3408ns | 9550ns | -1.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 4426ns | 2556ns | 9305ns | base | 1.851 |
| warm-clamp-accfit | 4446ns | 2691ns | 9208ns | +0.44% | 1.843 |
| warm-clamp-accfit-dyn | 4232ns | 2501ns | 9015ns | -4.38% | 1.936 |
| warm-clamp-head | 4358ns | 2418ns | 8994ns | -1.53% | 1.880 |
| warm-clamp-min-lanes | 3954ns | 2285ns | 8326ns | -10.67% | 2.072 |
| warm-clamp-minimum | 4345ns | 2540ns | 9236ns | -1.83% | 1.885 |

## Performance model

- Peak throughput: **3.585 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.643 | 73.7% |
| warm-clamp-accfit | 2.629 | 73.3% |
| warm-clamp-accfit-dyn | 2.641 | 73.7% |
| warm-clamp-head | 2.674 | 74.6% |
| warm-clamp-min-lanes | 3.326 | 92.8% |
| warm-clamp-minimum | 2.647 | 73.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 4561ns | 4561ns | base |
| warm-clamp-accfit | 4578ns | 4578ns | +0.39% |
| warm-clamp-accfit-dyn | 4361ns | 4361ns | -4.38% |
| warm-clamp-head | 4494ns | 4494ns | -1.45% |
| warm-clamp-min-lanes | 4107ns | 4107ns | -9.94% |
| warm-clamp-minimum | 4477ns | 4477ns | -1.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3100ns | base | --- | [3093, 3113] | --- | --- | --- | --- |
| warm-clamp-accfit | 3116ns | +11.8ns (+0.4%) | [+0, +28]ns | [3104, 3127] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-clamp-accfit-dyn | 3102ns | no significant difference | [-42, +5]ns | [3090, 3122] | no | 0.1009 | 0.0807 | 0 |
| warm-clamp-head | 3063ns | -38.7ns (-1.2%) | [-61, -30]ns | [3061, 3080] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2463ns | -630.0ns (-20.3%) | [-649, -425]ns | [2463, 2476] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 3095ns | no significant difference | [-17, +4]ns | [3087, 3107] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2534ns | +2.0% | -2.2% | +47.4% | -2.8% | -0.2% |
| 1 | 2 | 3107ns | +0.1% | -0.0% | -1.4% | -20.5% | +0.0% |
| 1 | 3 | 3100ns | +0.5% | +0.8% | -1.0% | +16.7% | -0.4% |
| 1 | 4 | 3113ns | +0.3% | -0.3% | -1.5% | -20.7% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.087 | ok |
| warm-clamp-accfit | -0.072 | ok |
| warm-clamp-accfit-dyn | -0.069 | ok |
| warm-clamp-head | -0.090 | ok |
| warm-clamp-min-lanes | -0.412 | moderate- |
| warm-clamp-minimum | -0.099 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 0/4, lost 4/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 1/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 35.0ns | 4426.0ns | 0.8% |  |
| warm-clamp-accfit | 34.7ns | 4445.5ns | 0.8% |  |
| warm-clamp-accfit-dyn | 36.2ns | 4232.0ns | 0.9% |  |
| warm-clamp-head | 37.4ns | 4358.1ns | 0.9% |  |
| warm-clamp-min-lanes | 42.4ns | 3953.8ns | 1.1% |  |
| warm-clamp-minimum | 34.1ns | 4344.8ns | 0.8% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2555.9-9304.6 ns)
   2555.9 |#
   2893.3 |########################################
   3230.8 |
   3568.2 |
   3905.6 |
   4243.1 |
   4580.5 |
   4917.9 |
   5255.4 |
   5592.8 |
   5930.2 |
   6267.7 |
   6605.1 |#
   6942.5 |
   7280.0 |#
   7617.4 |#
   7954.8 |#
   8292.3 |
   8629.7 |#
   8967.1 |
  (7 below, 5 above range)

warm-clamp-accfit (n=40, range 2691.0-9208.1 ns)
   2691.0 |
   3016.9 |########################################
   3342.7 |#
   3668.6 |
   3994.4 |#
   4320.3 |
   4646.2 |
   4972.0 |
   5297.9 |
   5623.7 |#
   5949.6 |
   6275.4 |
   6601.3 |
   6927.1 |#
   7253.0 |
   7578.9 |
   7904.7 |###
   8230.6 |
   8556.4 |
   8882.3 |#####
  (6 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 2500.7-9015.2 ns)
   2500.7 |#
   2826.5 |########################################
   3152.2 |
   3477.9 |#
   3803.6 |
   4129.3 |
   4455.1 |
   4780.8 |
   5106.5 |#
   5432.2 |
   5758.0 |
   6083.7 |#
   6409.4 |
   6735.1 |
   7060.8 |#
   7386.6 |
   7712.3 |
   8038.0 |#
   8363.7 |
   8689.5 |#
  (7 below, 4 above range)

warm-clamp-head (n=40, range 2418.2-8994.3 ns)
   2418.2 |
   2747.0 |########################################
   3075.8 |######
   3404.6 |
   3733.4 |##
   4062.2 |
   4391.0 |
   4719.8 |
   5048.6 |
   5377.4 |##
   5706.2 |
   6035.0 |##
   6363.9 |####
   6692.7 |##
   7021.5 |####
   7350.3 |
   7679.1 |
   8007.9 |
   8336.7 |
   8665.5 |
  (5 below, 4 above range)

warm-clamp-min-lanes (n=40, range 2285.2-8326.2 ns)
   2285.2 |########################################
   2587.3 |
   2889.3 |
   3191.4 |
   3493.4 |
   3795.5 |
   4097.5 |
   4399.6 |
   4701.6 |
   5003.7 |#
   5305.7 |#
   5607.8 |
   5909.8 |
   6211.9 |
   6513.9 |#
   6816.0 |#
   7118.0 |#
   7420.1 |###
   7722.1 |#
   8024.2 |#
  (5 below, 3 above range)

warm-clamp-minimum (n=40, range 2540.5-9235.6 ns)
   2540.5 |#
   2875.2 |########################################
   3210.0 |
   3544.7 |
   3879.5 |
   4214.3 |
   4549.0 |#
   4883.8 |
   5218.5 |
   5553.3 |
   5888.0 |
   6222.8 |
   6557.5 |#
   6892.3 |
   7227.0 |
   7561.8 |#
   7896.5 |
   8231.3 |
   8566.1 |#
   8900.8 |#
  (7 below, 5 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=59.6% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=57.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=59.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-head**: CV=60.7% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=62.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=59.0% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
