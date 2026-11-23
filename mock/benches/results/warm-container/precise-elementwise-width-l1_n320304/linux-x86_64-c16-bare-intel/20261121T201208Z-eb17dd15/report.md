# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (348 ns) is smaller than the fastest variant's own run-to-run std-dev (1.01 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-container-kernel is inconsistent: worst-20% is 4.2x its best-20%

warm-container-kernel's best 20% of batches run at 1.35 us but its worst 20% at 5.67 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-2 ns, 0.05%)

warm-container-plusone differs from baseline warm-container-headroom by -2 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-minimum** at 3569.9 ns median (-5.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.10x (fastest 3569.9 ns, slowest 3917.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112749630 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4961ns | 3846ns | 3233ns | 3891ns | 9900ns | base |
| warm-container-kernel | 3717ns | 4102ns | 1409ns | 3749ns | 5931ns | -25.07% |
| warm-container-minimum | 3516ns | 3734ns | 1605ns | 3793ns | 4596ns | -29.12% |
| warm-container-native | 3686ns | 3917ns | 1420ns | 3894ns | 5329ns | -25.70% |
| warm-container-plusone | 4933ns | 3844ns | 3232ns | 3863ns | 9842ns | -0.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4846ns | 3165ns | 9644ns | base | 8.451 |
| warm-container-kernel | 3551ns | 1346ns | 5672ns | -26.74% | 11.536 |
| warm-container-minimum | 3349ns | 1532ns | 4335ns | -30.91% | 12.232 |
| warm-container-native | 3496ns | 1357ns | 5019ns | -27.87% | 11.718 |
| warm-container-plusone | 4818ns | 3163ns | 9585ns | -0.59% | 8.502 |

## Performance model

- Peak throughput: **30.436 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 10.887 | 35.8% |
| warm-container-kernel | 10.455 | 34.3% |
| warm-container-minimum | 11.474 | 37.7% |
| warm-container-native | 10.941 | 35.9% |
| warm-container-plusone | 10.893 | 35.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4961ns | 4961ns | base |
| warm-container-kernel | 3717ns | 3717ns | -25.07% |
| warm-container-minimum | 3516ns | 3516ns | -29.12% |
| warm-container-native | 3686ns | 3686ns | -25.70% |
| warm-container-plusone | 4933ns | 4933ns | -0.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3762ns | base | --- | [3760, 3774] | --- | --- | --- | --- |
| warm-container-kernel | 3918ns | no significant difference | [-2414, +334]ns | [3360, 4328] | no | 0.8746 | 0.8746 | 0 |
| warm-container-minimum | 3570ns | no significant difference | [-2094, +2]ns | [3469, 3806] | no | 0.1076 | 0.0807 | 0 |
| warm-container-native | 3744ns | -536.2ns (-14.3%) | [-1740, -150]ns | [3243, 4144] | YES (adj: no) | 0.0664 | 0.0166 | 0 |
| warm-container-plusone | 3760ns | -1.7ns (-0.0%) | [-3, -0]ns | [3759, 3771] | YES (adj: no) | 0.1065 | 0.0533 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3086ns | +18.9% | +13.7% | +21.3% | -0.2% |
| 1 | 2 | 3767ns | +13.6% | -5.3% | -5.7% | +0.1% |
| 1 | 3 | 3774ns | +12.1% | +7.5% | +7.2% | -0.3% |
| 1 | 4 | 3767ns | +5.2% | -5.6% | +4.7% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.077 | ok |
| warm-container-kernel | -0.236 | moderate- |
| warm-container-minimum | -0.341 | moderate- |
| warm-container-native | -0.083 | ok |
| warm-container-plusone | -0.089 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 0/4, lost 4/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 1/4, lost 3/4
- **warm-container-plusone**: won 2/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 31.3ns | 4846.5ns | 0.6% |  |
| warm-container-kernel | 60.6ns | 3550.7ns | 1.7% |  |
| warm-container-minimum | 60.2ns | 3348.6ns | 1.8% |  |
| warm-container-native | 66.7ns | 3495.6ns | 1.9% |  |
| warm-container-plusone | 30.9ns | 4817.7ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3165.0-9643.9 ns)
   3165.0 |
   3489.0 |########################################
   3812.9 |
   4136.9 |
   4460.8 |#
   4784.8 |
   5108.7 |
   5432.6 |
   5756.6 |
   6080.5 |
   6404.5 |
   6728.4 |
   7052.4 |#
   7376.3 |
   7700.2 |###
   8024.2 |
   8348.1 |
   8672.1 |#
   8996.0 |
   9320.0 |#
  (7 below, 3 above range)

warm-container-kernel (n=40, range 1345.8-5672.1 ns)
   1345.8 |########################################
   1562.1 |
   1778.4 |
   1994.7 |#####
   2211.0 |
   2427.3 |
   2643.7 |
   2860.0 |
   3076.3 |##########
   3292.6 |##########
   3508.9 |#####
   3725.2 |###############
   3941.5 |#########################
   4157.9 |##########
   4374.2 |#########################
   4590.5 |#####
   4806.8 |##############################
   5023.1 |
   5239.4 |
   5455.7 |
  (3 below, 1 above range)

warm-container-minimum (n=40, range 1531.6-4335.4 ns)
   1531.6 |
   1671.8 |
   1812.0 |
   1952.2 |########
   2092.4 |
   2232.6 |
   2372.8 |
   2512.9 |
   2653.1 |
   2793.3 |########
   2933.5 |
   3073.7 |
   3213.9 |################################
   3354.1 |########################################
   3494.2 |################################
   3634.4 |################################
   3774.6 |########################
   3914.8 |########################################
   4055.0 |################
   4195.2 |################
  (6 below, 3 above range)

warm-container-native (n=40, range 1356.6-5019.5 ns)
   1356.6 |##########
   1539.8 |##########
   1722.9 |##########
   1906.0 |
   2089.2 |
   2272.3 |
   2455.5 |
   2638.6 |##########
   2821.8 |
   3004.9 |####################
   3188.0 |##############################
   3371.2 |####################
   3554.3 |##############################
   3737.5 |####################
   3920.6 |########################################
   4103.8 |####################
   4286.9 |##########
   4470.0 |########################################
   4653.2 |##########
   4836.3 |########################################
  (6 below, 2 above range)

warm-container-plusone (n=40, range 3162.8-9584.6 ns)
   3162.8 |
   3483.9 |########################################
   3805.0 |
   4126.1 |#
   4447.2 |
   4768.3 |
   5089.4 |#
   5410.5 |
   5731.6 |
   6052.6 |
   6373.7 |
   6694.8 |
   7015.9 |
   7337.0 |
   7658.1 |
   7979.2 |#
   8300.3 |
   8621.4 |
   8942.5 |#
   9263.5 |#
  (7 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=53.0% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **warm-container-kernel**: CV=52.0% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-minimum**: CV=30.3% (high variance, measurements may be unstable)
- **warm-container-native**: CV=37.1% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-plusone**: CV=53.5% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.0x (possible bimodal distribution)
