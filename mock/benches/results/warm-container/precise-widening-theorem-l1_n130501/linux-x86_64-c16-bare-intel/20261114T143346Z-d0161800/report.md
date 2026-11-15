# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-container-plusone, warm-container-headroom) are a dead heat (<1%)

warm-container-plusone (2.61 us) and warm-container-headroom (2.63 us) differ by 0.68%, inside the noise, even though the wider field spreads 42.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.11 us) is smaller than the fastest variant's own run-to-run std-dev (2.49 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-plusone, warm-container-headroom, warm-container-native, warm-container-minimum} vs {warm-container-kernel, warm-container-lanes-deferred} (28% apart)

The field splits into a fast tier {warm-container-plusone, warm-container-headroom, warm-container-native, warm-container-minimum} and a slow tier {warm-container-kernel, warm-container-lanes-deferred} with a 28% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-plusone is inconsistent: worst-20% is 4.0x its best-20%

warm-container-plusone's best 20% of batches run at 2.12 us but its worst 20% at 8.43 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-33 ns, 1.27%)

warm-container-plusone differs from baseline warm-container-headroom by -33 ns (1.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-plusone** at 2608.6 ns median (-0.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.43x (fastest 2608.6 ns, slowest 3719.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4087ns | 2690ns | 2664ns | 2932ns | 8971ns | base |
| warm-container-kernel | 3643ns | 3760ns | 1581ns | 3869ns | 5028ns | -10.84% |
| warm-container-lanes-deferred | 3656ns | 3907ns | 1381ns | 3960ns | 5022ns | -10.52% |
| warm-container-minimum | 4367ns | 2853ns | 2376ns | 3332ns | 9464ns | +6.86% |
| warm-container-native | 4388ns | 2835ns | 2822ns | 3227ns | 9434ns | +7.37% |
| warm-container-plusone | 4260ns | 2672ns | 2174ns | 3476ns | 8696ns | +4.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3976ns | 2600ns | 8692ns | base | 4.121 |
| warm-container-kernel | 3455ns | 1503ns | 4737ns | -13.10% | 4.742 |
| warm-container-lanes-deferred | 3470ns | 1314ns | 4741ns | -12.74% | 4.722 |
| warm-container-minimum | 4257ns | 2323ns | 9192ns | +7.07% | 3.849 |
| warm-container-native | 4279ns | 2759ns | 9169ns | +7.63% | 3.829 |
| warm-container-plusone | 4145ns | 2123ns | 8427ns | +4.26% | 3.952 |

## Performance model

- Peak throughput: **12.465 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.238 | 50.0% |
| warm-container-kernel | 4.577 | 36.7% |
| warm-container-lanes-deferred | 4.405 | 35.3% |
| warm-container-minimum | 5.873 | 47.1% |
| warm-container-native | 5.910 | 47.4% |
| warm-container-plusone | 6.281 | 50.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4087ns | 4087ns | base |
| warm-container-kernel | 3643ns | 3643ns | -10.84% |
| warm-container-lanes-deferred | 3656ns | 3656ns | -10.52% |
| warm-container-minimum | 4367ns | 4367ns | +6.86% |
| warm-container-native | 4388ns | 4388ns | +7.37% |
| warm-container-plusone | 4260ns | 4260ns | +4.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2626ns | base | --- | [2610, 2643] | --- | --- | --- | --- |
| warm-container-kernel | 3580ns | no significant difference | [-1376, +779]ns | [3218, 4170] | no | 0.3352 | 0.2682 | 0 |
| warm-container-lanes-deferred | 3719ns | no significant difference | [-1367, +1027]ns | [3467, 4037] | no | 0.4296 | 0.4296 | 0 |
| warm-container-minimum | 2790ns | +169.1ns (+6.4%) | [+144, +184]ns | [2775, 2833] | YES | 0.0161 | 0.0064 | 0 |
| warm-container-native | 2772ns | +158.5ns (+6.0%) | [+141, +162]ns | [2764, 2780] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2609ns | -33.3ns (-1.3%) | [-471, -1]ns | [2594, 3844] | YES (adj: no) | 0.0641 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2626ns | +41.9% | +55.6% | -13.7% | +5.7% | -18.8% |
| 1 | 2 | 2617ns | +21.3% | +37.7% | +7.1% | +6.0% | +106.1% |
| 1 | 3 | 2627ns | +63.1% | +40.2% | +7.8% | +5.3% | -0.9% |
| 1 | 4 | 2970ns | +50.5% | +5.2% | -6.1% | -6.9% | -11.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.068 | ok |
| warm-container-kernel | 0.069 | ok |
| warm-container-lanes-deferred | -0.086 | ok |
| warm-container-minimum | -0.062 | ok |
| warm-container-native | 0.299 | moderate+ |
| warm-container-plusone | -0.499 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 0/4, lost 4/4
- **warm-container-lanes-deferred**: won 0/4, lost 4/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 1/4, lost 3/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 36.9ns | 3976.0ns | 0.9% |  |
| warm-container-kernel | 69.8ns | 3455.0ns | 2.0% |  |
| warm-container-lanes-deferred | 68.8ns | 3469.6ns | 2.0% |  |
| warm-container-minimum | 37.1ns | 4257.1ns | 0.9% |  |
| warm-container-native | 37.3ns | 4279.3ns | 0.9% |  |
| warm-container-plusone | 40.4ns | 4145.5ns | 1.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2599.7-8691.9 ns)
   2599.7 |########################################
   2904.3 |#
   3208.9 |#
   3513.5 |
   3818.1 |
   4122.7 |
   4427.3 |
   4732.0 |
   5036.6 |
   5341.2 |
   5645.8 |
   5950.4 |
   6255.0 |
   6559.6 |
   6864.2 |
   7168.9 |#
   7473.5 |
   7778.1 |###
   8082.7 |#
   8387.3 |#
  (3 below, 4 above range)

warm-container-kernel (n=40, range 1503.5-4736.8 ns)
   1503.5 |
   1665.2 |
   1826.8 |
   1988.5 |
   2150.2 |####
   2311.8 |
   2473.5 |####
   2635.2 |
   2796.8 |####
   2958.5 |########
   3120.2 |######################
   3281.8 |#############
   3443.5 |####
   3605.2 |#############
   3766.8 |####
   3928.5 |
   4090.2 |#############
   4251.8 |####
   4413.5 |########################################
   4575.2 |
  (6 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1314.3-4740.7 ns)
   1314.3 |
   1485.7 |
   1657.0 |######
   1828.3 |
   1999.6 |
   2170.9 |
   2342.2 |
   2513.6 |
   2684.9 |
   2856.2 |
   3027.5 |######
   3198.8 |######
   3370.1 |########################################
   3541.5 |##########################
   3712.8 |#############
   3884.1 |#################################
   4055.4 |####################
   4226.7 |######
   4398.0 |##########################
   4569.3 |#############
  (7 below, 3 above range)

warm-container-minimum (n=40, range 2322.7-9191.9 ns)
   2322.7 |
   2666.2 |########################################
   3009.6 |##
   3353.1 |##
   3696.6 |
   4040.0 |
   4383.5 |
   4726.9 |##
   5070.4 |##
   5413.8 |
   5757.3 |
   6100.8 |
   6444.2 |
   6787.7 |
   7131.1 |
   7474.6 |
   7818.0 |##
   8161.5 |##
   8505.0 |##
   8848.4 |######
  (7 below, 3 above range)

warm-container-native (n=40, range 2759.3-9169.4 ns)
   2759.3 |########################################
   3079.8 |
   3400.3 |
   3720.8 |
   4041.3 |
   4361.8 |
   4682.3 |
   5002.8 |
   5323.3 |#
   5643.8 |#
   5964.3 |
   6284.8 |#
   6605.3 |
   6925.9 |
   7246.4 |
   7566.9 |
   7887.4 |
   8207.9 |##
   8528.4 |#
   8848.9 |#
  (2 below, 4 above range)

warm-container-plusone (n=40, range 2122.9-8426.6 ns)
   2122.9 |################
   2438.1 |########################################
   2753.3 |
   3068.4 |
   3383.6 |
   3698.8 |#####
   4014.0 |
   4329.2 |
   4644.4 |##
   4959.5 |
   5274.7 |
   5589.9 |
   5905.1 |##
   6220.3 |##
   6535.5 |#####
   6850.6 |
   7165.8 |##
   7481.0 |
   7796.2 |#####
   8111.4 |##
  (4 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=62.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-container-kernel**: CV=33.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=34.1% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-minimum**: CV=62.9% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-native**: CV=60.6% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-container-plusone**: CV=60.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.0x (possible bimodal distribution)
