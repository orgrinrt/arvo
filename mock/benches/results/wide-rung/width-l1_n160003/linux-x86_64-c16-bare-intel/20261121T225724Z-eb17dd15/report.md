# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged is fastest but the noisiest (CV 65.4%)

wide-rung-ragged wins on median (3.30 us) yet has the highest variance (CV 65.4%), while wide-rung-ragged-overread is the steadiest (CV 29.2%, 4.32 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (wide-rung-ragged, wide-rung-wordround) are a dead heat (<1%)

wide-rung-ragged (3.30 us) and wide-rung-wordround (3.32 us) differ by 0.73%, inside the noise, even though the wider field spreads 30.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.02 us) is smaller than the fastest variant's own run-to-run std-dev (2.16 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wide-rung-wordround is inconsistent: worst-20% is 2.7x its best-20%

wide-rung-wordround's best 20% of batches run at 3.17 us but its worst 20% at 8.50 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-ragged** at 3298.2 ns median (-16.7% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.31x (fastest 3298.2 ns, slowest 4316.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112636733 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4289ns | 4040ns | 3310ns | 4042ns | 6009ns | base |
| wide-rung-ragged | 4355ns | 3353ns | 3339ns | 3566ns | 7740ns | +1.55% |
| wide-rung-ragged-overread | 4665ns | 4382ns | 4372ns | 4383ns | 5806ns | +8.77% |
| wide-rung-wordround | 4635ns | 3395ns | 3238ns | 3738ns | 8725ns | +8.08% |
| wide-rung-wordround-alias | 4634ns | 3954ns | 3153ns | 3895ns | 8333ns | +8.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4196ns | 3243ns | 5854ns | base | 1.952 |
| wide-rung-ragged | 4277ns | 3287ns | 7570ns | +1.93% | 1.915 |
| wide-rung-ragged-overread | 4589ns | 4309ns | 5683ns | +9.37% | 1.785 |
| wide-rung-wordround | 4531ns | 3172ns | 8501ns | +7.99% | 1.808 |
| wide-rung-wordround-alias | 4529ns | 3088ns | 8113ns | +7.93% | 1.809 |

## Performance model

- Peak throughput: **2.653 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 2.070 | 78.0% |
| wide-rung-ragged | 2.484 | 93.6% |
| wide-rung-ragged-overread | 1.898 | 71.5% |
| wide-rung-wordround | 2.466 | 92.9% |
| wide-rung-wordround-alias | 2.115 | 79.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4289ns | 4289ns | base |
| wide-rung-ragged | 4355ns | 4355ns | +1.55% |
| wide-rung-ragged-overread | 4665ns | 4665ns | +8.77% |
| wide-rung-wordround | 4635ns | 4635ns | +8.08% |
| wide-rung-wordround-alias | 4634ns | 4634ns | +8.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 3958ns | base | --- | [3955, 3968] | --- | --- | --- | --- |
| wide-rung-ragged | 3298ns | no significant difference | [-661, +50]ns | [3290, 4015] | no | 0.6358 | 0.6358 | 0 |
| wide-rung-ragged-overread | 4316ns | +357.6ns (+9.0%) | [+353, +367]ns | [4312, 4323] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 3322ns | -99.4ns (-2.5%) | [-768, -71]ns | [3184, 3873] | YES | 0.0002 | 0.0002 | 0 |
| wide-rung-wordround-alias | 3873ns | -94.7ns (-2.4%) | [-155, -82]ns | [3776, 3882] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 3960ns | +1.6% | +9.1% | -19.5% | -4.7% |
| 1 | 2 | 3965ns | -17.0% | +8.8% | -2.3% | -2.1% |
| 1 | 3 | 3962ns | -16.8% | +8.8% | -19.8% | -1.9% |
| 1 | 4 | 3245ns | +1.3% | +33.0% | -1.7% | -4.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.081 | ok |
| wide-rung-ragged | -0.085 | ok |
| wide-rung-ragged-overread | -0.013 | ok |
| wide-rung-wordround | -0.420 | moderate- |
| wide-rung-wordround-alias | -0.121 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 0/4, lost 4/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 24.8ns | 4195.9ns | 0.6% |  |
| wide-rung-ragged | 25.8ns | 4276.8ns | 0.6% |  |
| wide-rung-ragged-overread | 24.7ns | 4589.1ns | 0.5% |  |
| wide-rung-wordround | 27.5ns | 4531.1ns | 0.6% |  |
| wide-rung-wordround-alias | 27.6ns | 4528.6ns | 0.6% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 3243.5-5853.6 ns)
   3243.5 |#####
   3374.0 |
   3504.5 |
   3635.0 |
   3765.5 |
   3896.0 |########################################
   4026.5 |
   4157.0 |
   4287.5 |
   4418.0 |
   4548.5 |
   4679.0 |
   4809.5 |
   4940.0 |
   5070.6 |
   5201.1 |
   5331.6 |
   5462.1 |
   5592.6 |
   5723.1 |
  (4 below, 3 above range)

wide-rung-ragged (n=40, range 3286.6-7570.5 ns)
   3286.6 |########################################
   3500.8 |
   3715.0 |
   3929.1 |#################
   4143.3 |
   4357.5 |
   4571.7 |
   4785.9 |
   5000.1 |
   5214.3 |#
   5428.5 |#
   5642.7 |
   5856.9 |
   6071.1 |
   6285.3 |
   6499.5 |
   6713.7 |
   6927.9 |
   7142.1 |
   7356.3 |
  (4 below, 4 above range)

wide-rung-ragged-overread (n=40, range 4309.4-5683.1 ns)
   4309.4 |########################################
   4378.1 |
   4446.8 |
   4515.5 |
   4584.2 |
   4652.9 |
   4721.6 |
   4790.2 |
   4858.9 |
   4927.6 |
   4996.3 |
   5065.0 |
   5133.7 |
   5202.4 |
   5271.0 |
   5339.7 |
   5408.4 |
   5477.1 |
   5545.8 |
   5614.5 |
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 3171.9-8500.8 ns)
   3171.9 |########################################
   3438.3 |##
   3704.8 |######################
   3971.2 |
   4237.7 |
   4504.1 |
   4770.6 |
   5037.0 |
   5303.5 |##
   5569.9 |
   5836.4 |#####
   6102.8 |
   6369.3 |
   6635.7 |##
   6902.2 |
   7168.6 |
   7435.0 |
   7701.5 |
   7967.9 |
   8234.4 |##
  (4 below, 5 above range)

wide-rung-wordround-alias (n=40, range 3087.6-8113.0 ns)
   3087.6 |##########
   3338.8 |
   3590.1 |####################
   3841.4 |########################################
   4092.6 |
   4343.9 |
   4595.2 |
   4846.5 |#####
   5097.7 |
   5349.0 |
   5600.3 |
   5851.5 |
   6102.8 |
   6354.1 |
   6605.4 |
   6856.6 |
   7107.9 |##
   7359.2 |
   7610.4 |
   7861.7 |
  (5 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=33.4% (high variance, measurements may be unstable)
- **wide-rung-ragged**: CV=50.4% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=27.4% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=47.6% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=50.9% (high variance, measurements may be unstable)
