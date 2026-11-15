# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum dominates: 37% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-minimum (2.04 us) leads warm-clamp-accfit-dyn (2.78 us) by 37%, a clear separation rather than a photo finish. CV 98.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum beats baseline by 27% (significant)

warm-clamp-minimum is -828 ns (27%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is fastest but the noisiest (CV 98.4%)

warm-clamp-minimum wins on median (2.04 us) yet has the highest variance (CV 98.4%), while warm-clamp-accfit-dyn is the steadiest (CV 8.4%, 2.78 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.88 us) is smaller than the fastest variant's own run-to-run std-dev (2.01 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.5x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.19 us but its worst 20% at 5.39 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 2038.3 ns median (-33.2% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.92x (fastest 2038.3 ns, slowest 3917.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3211ns | 3281ns | 2372ns | 3282ns | 3835ns | base |
| warm-clamp-accfit | 3192ns | 3275ns | 2351ns | 3221ns | 3943ns | -0.59% |
| warm-clamp-accfit-dyn | 3029ns | 3012ns | 2672ns | 3029ns | 3385ns | -5.66% |
| warm-clamp-head | 3091ns | 3127ns | 2537ns | 3106ns | 3600ns | -3.74% |
| warm-clamp-min-lanes | 3535ns | 4094ns | 1240ns | 3585ns | 5678ns | +10.09% |
| warm-clamp-minimum | 3221ns | 2109ns | 1745ns | 2396ns | 7173ns | +0.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2934ns | 2172ns | 3420ns | base | 2.792 |
| warm-clamp-accfit | 2954ns | 2188ns | 3614ns | +0.69% | 2.773 |
| warm-clamp-accfit-dyn | 2803ns | 2484ns | 3091ns | -4.47% | 2.923 |
| warm-clamp-head | 2780ns | 2294ns | 3201ns | -5.24% | 2.946 |
| warm-clamp-min-lanes | 3370ns | 1186ns | 5389ns | +14.87% | 2.431 |
| warm-clamp-minimum | 3101ns | 1686ns | 6887ns | +5.69% | 2.642 |

## Performance model

- Peak throughput: **6.907 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.686 | 38.9% |
| warm-clamp-accfit | 2.687 | 38.9% |
| warm-clamp-accfit-dyn | 2.944 | 42.6% |
| warm-clamp-head | 2.901 | 42.0% |
| warm-clamp-min-lanes | 2.091 | 30.3% |
| warm-clamp-minimum | 4.019 | 58.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3211ns | 3211ns | base |
| warm-clamp-accfit | 3192ns | 3192ns | -0.59% |
| warm-clamp-accfit-dyn | 3029ns | 3029ns | -5.66% |
| warm-clamp-head | 3091ns | 3091ns | -3.74% |
| warm-clamp-min-lanes | 3535ns | 3535ns | +10.09% |
| warm-clamp-minimum | 3221ns | 3221ns | +0.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3050ns | base | --- | [3042, 3096] | --- | --- | --- | --- |
| warm-clamp-accfit | 3048ns | no significant difference | [-115, +48]ns | [2842, 3062] | no | 0.2682 | 0.2682 | 0 |
| warm-clamp-accfit-dyn | 2783ns | -234.5ns (-7.7%) | [-289, -100]ns | [2718, 2983] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2824ns | -207.3ns (-6.8%) | [-239, -159]ns | [2795, 2875] | YES | 0.0161 | 0.0064 | 0 |
| warm-clamp-min-lanes | 3918ns | +829.1ns (+27.2%) | [+62, +1185]ns | [2742, 4326] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-clamp-minimum | 2038ns | -828.4ns (-27.2%) | [-1017, -189]ns | [1999, 2118] | YES (adj: no) | 0.1009 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3044ns | +9.8% | -12.4% | -18.3% | +23.2% | -46.0% |
| 1 | 2 | 3050ns | -8.1% | -0.3% | -5.5% | +29.0% | -34.5% |
| 1 | 3 | 3049ns | -3.3% | -7.4% | -7.4% | +26.8% | -30.3% |
| 1 | 4 | 3052ns | -0.1% | -10.9% | -7.2% | +32.9% | -33.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.094 | ok |
| warm-clamp-accfit | -0.338 | moderate- |
| warm-clamp-accfit-dyn | -0.387 | moderate- |
| warm-clamp-head | -0.206 | moderate- |
| warm-clamp-min-lanes | -0.235 | moderate- |
| warm-clamp-minimum | 0.072 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 94.9ns | 2934.0ns | 3.2% |  |
| warm-clamp-accfit | 83.1ns | 2954.2ns | 2.8% |  |
| warm-clamp-accfit-dyn | 80.0ns | 2802.9ns | 2.9% |  |
| warm-clamp-head | 87.1ns | 2780.3ns | 3.1% |  |
| warm-clamp-min-lanes | 56.0ns | 3370.4ns | 1.7% |  |
| warm-clamp-minimum | 36.6ns | 3100.8ns | 1.2% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2171.8-3419.8 ns)
   2171.8 |
   2234.2 |
   2296.6 |
   2359.0 |
   2421.4 |
   2483.8 |
   2546.2 |#####
   2608.6 |
   2671.0 |
   2733.4 |##############
   2795.8 |
   2858.2 |#####
   2920.6 |##
   2983.0 |##
   3045.4 |########################################
   3107.8 |###########
   3170.2 |
   3232.6 |##
   3295.0 |
   3357.4 |###########
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 2187.9-3614.2 ns)
   2187.9 |
   2259.2 |
   2330.5 |
   2401.8 |
   2473.2 |
   2544.5 |################
   2615.8 |###
   2687.1 |
   2758.4 |################
   2829.7 |######
   2901.0 |######
   2972.4 |
   3043.7 |########################################
   3115.0 |###
   3186.3 |##########
   3257.6 |
   3328.9 |###
   3400.3 |##########
   3471.6 |
   3542.9 |
  (2 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 2484.4-3090.7 ns)
   2484.4 |
   2514.7 |##############################
   2545.0 |###############
   2575.3 |
   2605.6 |##########
   2635.9 |
   2666.2 |#####
   2696.6 |
   2726.9 |
   2757.2 |##############################
   2787.5 |###############
   2817.8 |
   2848.1 |#####
   2878.4 |#####
   2908.8 |#####
   2939.1 |
   2969.4 |
   2999.7 |
   3030.0 |########################################
   3060.3 |###############
  (2 below, 3 above range)

warm-clamp-head (n=40, range 2293.5-3200.9 ns)
   2293.5 |
   2338.9 |########
   2384.2 |
   2429.6 |####
   2475.0 |
   2520.3 |########
   2565.7 |############
   2611.1 |####
   2656.4 |
   2701.8 |####
   2747.2 |####
   2792.5 |########################################
   2837.9 |####################
   2883.3 |
   2928.7 |################
   2974.0 |
   3019.4 |
   3064.8 |####
   3110.1 |############
   3155.5 |
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1186.0-5388.9 ns)
   1186.0 |####################
   1396.2 |########################################
   1606.3 |
   1816.5 |
   2026.6 |
   2236.8 |
   2446.9 |
   2657.0 |##############################
   2867.2 |
   3077.3 |
   3287.4 |##########
   3497.6 |
   3707.7 |##############################
   3917.9 |########################################
   4128.0 |####################
   4338.2 |##############################
   4548.3 |##########
   4758.4 |####################
   4968.6 |####################
   5178.7 |########################################
  (7 below, 2 above range)

warm-clamp-minimum (n=40, range 1685.8-6886.6 ns)
   1685.8 |
   1945.8 |########################################
   2205.9 |
   2465.9 |
   2725.9 |
   2986.0 |
   3246.0 |#
   3506.1 |
   3766.1 |
   4026.2 |
   4286.2 |###
   4546.2 |
   4806.3 |
   5066.3 |
   5326.4 |
   5586.4 |#
   5846.5 |
   6106.5 |
   6366.6 |#
   6626.6 |###
  (7 below, 4 above range)

```

## Diagnostics

- **warm-clamp-min-lanes**: CV=48.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=64.7% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.1x (possible bimodal distribution)
