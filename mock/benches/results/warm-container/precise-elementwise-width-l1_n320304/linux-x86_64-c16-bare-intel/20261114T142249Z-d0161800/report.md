# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-container-headroom, warm-container-plusone) are a dead heat (<1%)

warm-container-headroom (3.76 us) and warm-container-plusone (3.78 us) differ by 0.48%, inside the noise, even though the wider field spreads 19.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (747 ns) is smaller than the fastest variant's own run-to-run std-dev (2.37 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-container-headroom)

The baseline warm-container-headroom is the fastest (3.76 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### warm-container-native is inconsistent: worst-20% is 4.2x its best-20%

warm-container-native's best 20% of batches run at 1.35 us but its worst 20% at 5.70 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (10 ns, 0.27%)

warm-container-plusone differs from baseline warm-container-headroom by 10 ns (0.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (warm-container-headroom) is the fastest** at 3761.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.20x (fastest 3761.4 ns, slowest 4508.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4761ns | 3844ns | 3833ns | 3845ns | 8436ns | base |
| warm-container-kernel | 3796ns | 4258ns | 1409ns | 3878ns | 5936ns | -20.27% |
| warm-container-minimum | 3540ns | 4226ns | 1251ns | 3742ns | 5225ns | -25.65% |
| warm-container-native | 4182ns | 4723ns | 1409ns | 4491ns | 6032ns | -12.15% |
| warm-container-plusone | 5894ns | 3863ns | 3840ns | 4372ns | 12513ns | +23.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4648ns | 3750ns | 8200ns | base | 8.813 |
| warm-container-kernel | 3612ns | 1346ns | 5612ns | -22.28% | 11.340 |
| warm-container-minimum | 3369ns | 1195ns | 4942ns | -27.52% | 12.159 |
| warm-container-native | 3980ns | 1345ns | 5698ns | -14.37% | 10.292 |
| warm-container-plusone | 5758ns | 3757ns | 12208ns | +23.90% | 7.113 |

## Performance model

- Peak throughput: **34.284 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 10.890 | 31.8% |
| warm-container-kernel | 10.066 | 29.4% |
| warm-container-minimum | 10.143 | 29.6% |
| warm-container-native | 9.086 | 26.5% |
| warm-container-plusone | 10.837 | 31.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4761ns | 4761ns | base |
| warm-container-kernel | 3796ns | 3796ns | -20.27% |
| warm-container-minimum | 3540ns | 3540ns | -25.65% |
| warm-container-native | 4182ns | 4182ns | -12.15% |
| warm-container-plusone | 5894ns | 5894ns | +23.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3761ns | base | --- | [3759, 3768] | --- | --- | --- | --- |
| warm-container-kernel | 4069ns | no significant difference | [-2394, +347]ns | [3497, 4409] | no | 0.8746 | 0.8746 | 0 |
| warm-container-minimum | 4038ns | no significant difference | [-2404, +270]ns | [3627, 4280] | no | 0.8478 | 0.6358 | 0 |
| warm-container-native | 4508ns | no significant difference | [-1108, +980]ns | [4038, 4846] | no | 0.8478 | 0.4296 | 0 |
| warm-container-plusone | 3780ns | +10.0ns (+0.3%) | [+1, +23]ns | [3769, 3800] | YES (adj: no) | 0.1539 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3760ns | -23.4% | -54.4% | +30.3% | +25.9% |
| 1 | 2 | 3762ns | +9.8% | +6.0% | +20.1% | +0.1% |
| 1 | 3 | 3764ns | +6.1% | +16.7% | -4.4% | +0.8% |
| 1 | 4 | 3766ns | +12.7% | +5.2% | +5.4% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.290 | moderate+ |
| warm-container-kernel | -0.133 | ok |
| warm-container-minimum | -0.010 | ok |
| warm-container-native | 0.182 | ok |
| warm-container-plusone | -0.104 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 1/4, lost 3/4
- **warm-container-plusone**: won 0/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 30.6ns | 4647.6ns | 0.7% |  |
| warm-container-kernel | 66.2ns | 3612.1ns | 1.8% |  |
| warm-container-minimum | 61.9ns | 3368.8ns | 1.8% |  |
| warm-container-native | 72.6ns | 3979.7ns | 1.8% |  |
| warm-container-plusone | 37.6ns | 5758.3ns | 0.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3750.0-8200.1 ns)
   3750.0 |########################################
   3972.5 |
   4195.0 |
   4417.5 |
   4640.0 |
   4862.5 |
   5085.0 |
   5307.5 |
   5530.0 |
   5752.6 |#
   5975.1 |#
   6197.6 |
   6420.1 |
   6642.6 |
   6865.1 |
   7087.6 |
   7310.1 |
   7532.6 |
   7755.1 |
   7977.6 |
  (3 below, 4 above range)

warm-container-kernel (n=40, range 1346.5-5611.9 ns)
   1346.5 |########################################
   1559.8 |
   1773.0 |
   1986.3 |
   2199.6 |
   2412.8 |
   2626.1 |
   2839.4 |
   3052.7 |########
   3265.9 |
   3479.2 |
   3692.5 |####
   3905.7 |##########################
   4119.0 |########
   4332.3 |###############################
   4545.5 |
   4758.8 |##########################
   4972.1 |
   5185.3 |
   5398.6 |####
  (3 below, 3 above range)

warm-container-minimum (n=40, range 1194.7-4941.9 ns)
   1194.7 |########################################
   1382.1 |
   1569.4 |
   1756.8 |
   1944.2 |
   2131.5 |#####
   2318.9 |
   2506.2 |
   2693.6 |
   2880.9 |
   3068.3 |
   3255.6 |
   3443.0 |#####
   3630.4 |#################
   3817.7 |###########
   4005.1 |##################################
   4192.4 |###########
   4379.8 |######################
   4567.1 |#####
   4754.5 |############################
  (5 below, 3 above range)

warm-container-native (n=40, range 1345.5-5698.2 ns)
   1345.5 |#############
   1563.1 |
   1780.7 |
   1998.4 |
   2216.0 |
   2433.6 |###
   2651.3 |######
   2868.9 |###
   3086.6 |
   3304.2 |
   3521.8 |
   3739.5 |###
   3957.1 |######
   4174.7 |######
   4392.4 |#############
   4610.0 |###
   4827.6 |########################################
   5045.3 |###
   5262.9 |######
   5480.6 |###
  (4 below, 2 above range)

warm-container-plusone (n=40, range 3757.3-12208.4 ns)
   3757.3 |########################################
   4179.9 |#
   4602.5 |
   5025.0 |
   5447.6 |#
   5870.1 |
   6292.7 |#
   6715.2 |#
   7137.8 |
   7560.3 |#
   7982.9 |
   8405.5 |
   8828.0 |
   9250.6 |
   9673.1 |
  10095.7 |#
  10518.2 |
  10940.8 |###
  11363.3 |
  11785.9 |#
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=50.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=45.5% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-minimum**: CV=43.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-native**: CV=40.1% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-plusone**: CV=58.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.2x (possible bimodal distribution)
