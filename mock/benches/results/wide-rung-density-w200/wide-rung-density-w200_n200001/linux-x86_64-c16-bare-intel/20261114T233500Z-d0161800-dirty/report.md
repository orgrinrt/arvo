# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-ragged, wide-rung-align16) are a dead heat (<1%)

wide-rung-ragged (4.11 us) and wide-rung-align16 (4.13 us) differ by 0.50%, inside the noise, even though the wider field spreads 7.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (318 ns) is smaller than the fastest variant's own run-to-run std-dev (1.99 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-ragged vs stability leader wide-rung-ragged-overread (+8% speed for 1.0x steadier)

wide-rung-ragged is fastest (4.11 us, CV 48.4%); wide-rung-ragged-overread gives up 7.7% median for 1.0x lower variance (CV 47.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-align16 is inconsistent: worst-20% is 2.4x its best-20%

wide-rung-align16's best 20% of batches run at 3.46 us but its worst 20% at 8.26 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### wide-rung-ragged's edge over baseline is significant but tiny (-15 ns, 0.36%)

wide-rung-ragged differs from baseline wide-rung-align16 by -15 ns (0.36%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged** at 4105.9 ns median (-0.5% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.08x (fastest 4105.9 ns, slowest 4423.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4926ns | 4210ns | 3529ns | 4208ns | 8477ns | base |
| wide-rung-ragged | 4720ns | 4189ns | 3429ns | 4189ns | 7602ns | -4.18% |
| wide-rung-ragged-overread | 5084ns | 4505ns | 3676ns | 4507ns | 8223ns | +3.21% |
| wide-rung-wordround | 5165ns | 4223ns | 4114ns | 4221ns | 9045ns | +4.84% |
| wide-rung-wordround-alias | 5167ns | 4211ns | 4111ns | 4210ns | 9094ns | +4.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4817ns | 3457ns | 8256ns | base | 0.850 |
| wide-rung-ragged | 4617ns | 3362ns | 7400ns | -4.16% | 0.887 |
| wide-rung-ragged-overread | 4985ns | 3609ns | 8045ns | +3.50% | 0.822 |
| wide-rung-wordround | 5052ns | 4032ns | 8812ns | +4.88% | 0.811 |
| wide-rung-wordround-alias | 5055ns | 4029ns | 8865ns | +4.93% | 0.810 |

## Performance model

- Peak throughput: **1.218 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.993 | 81.5% |
| wide-rung-ragged | 0.998 | 81.9% |
| wide-rung-ragged-overread | 0.926 | 76.0% |
| wide-rung-wordround | 0.990 | 81.2% |
| wide-rung-wordround-alias | 0.992 | 81.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4926ns | 4926ns | base |
| wide-rung-ragged | 4720ns | 4720ns | -4.18% |
| wide-rung-ragged-overread | 5084ns | 5084ns | +3.21% |
| wide-rung-wordround | 5165ns | 5165ns | +4.84% |
| wide-rung-wordround-alias | 5167ns | 5167ns | +4.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4126ns | base | --- | [4113, 4132] | --- | --- | --- | --- |
| wide-rung-ragged | 4106ns | -14.9ns (-0.4%) | [-24, -5]ns | [4102, 4111] | YES | 0.0030 | 0.0022 | 0 |
| wide-rung-ragged-overread | 4424ns | +294.6ns (+7.1%) | [+282, +305]ns | [4419, 4431] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 4138ns | +21.8ns (+0.5%) | [+11, +34]ns | [4131, 4145] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 4128ns | +17.4ns (+0.4%) | [+8, +47]ns | [4116, 4145] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 4130ns | -0.6% | +7.2% | +0.4% | +0.3% |
| 1 | 2 | 4120ns | -0.2% | +7.4% | +0.5% | +0.8% |
| 1 | 3 | 4130ns | -0.6% | +7.0% | +0.2% | -0.3% |
| 1 | 4 | 3366ns | -0.1% | +7.3% | +19.8% | +19.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.091 | ok |
| wide-rung-ragged | -0.078 | ok |
| wide-rung-ragged-overread | -0.077 | ok |
| wide-rung-wordround | -0.079 | ok |
| wide-rung-wordround-alias | 0.106 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 0/4
- **wide-rung-ragged-overread**: won 0/4, lost 4/4
- **wide-rung-wordround**: won 0/4, lost 4/4
- **wide-rung-wordround-alias**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 29.1ns | 4816.9ns | 0.6% |  |
| wide-rung-ragged | 28.0ns | 4616.7ns | 0.6% |  |
| wide-rung-ragged-overread | 27.4ns | 4985.4ns | 0.5% |  |
| wide-rung-wordround | 30.3ns | 5052.0ns | 0.6% |  |
| wide-rung-wordround-alias | 30.6ns | 5054.6ns | 0.6% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 3456.8-8256.1 ns)
   3456.8 |
   3696.8 |
   3936.8 |########################################
   4176.7 |
   4416.7 |
   4656.7 |#
   4896.6 |
   5136.6 |
   5376.5 |
   5616.5 |
   5856.5 |
   6096.4 |
   6336.4 |
   6576.3 |
   6816.3 |
   7056.3 |#
   7296.2 |
   7536.2 |
   7776.1 |#
   8016.1 |
  (7 below, 4 above range)

wide-rung-ragged (n=40, range 3362.4-7400.2 ns)
   3362.4 |####
   3564.3 |
   3766.2 |
   3968.1 |########################################
   4170.0 |
   4371.8 |
   4573.7 |
   4775.6 |
   4977.5 |
   5179.4 |
   5381.3 |
   5583.2 |
   5785.1 |
   5987.0 |#
   6188.9 |
   6390.8 |
   6592.7 |
   6794.5 |
   6996.4 |
   7198.3 |
  (5 below, 4 above range)

wide-rung-ragged-overread (n=40, range 3609.4-8045.0 ns)
   3609.4 |####
   3831.1 |
   4052.9 |
   4274.7 |########################################
   4496.5 |
   4718.3 |
   4940.0 |
   5161.8 |
   5383.6 |
   5605.4 |
   5827.2 |#
   6049.0 |
   6270.7 |
   6492.5 |
   6714.3 |
   6936.1 |#
   7157.9 |
   7379.6 |
   7601.4 |#
   7823.2 |
  (5 below, 3 above range)

wide-rung-wordround (n=40, range 4032.5-8812.0 ns)
   4032.5 |########################################
   4271.5 |
   4510.4 |
   4749.4 |#
   4988.4 |
   5227.4 |
   5466.3 |
   5705.3 |
   5944.3 |
   6183.3 |
   6422.2 |
   6661.2 |
   6900.2 |
   7139.1 |
   7378.1 |
   7617.1 |
   7856.1 |
   8095.0 |
   8334.0 |
   8573.0 |
  (5 below, 6 above range)

wide-rung-wordround-alias (n=40, range 4028.7-8864.8 ns)
   4028.7 |########################################
   4270.6 |
   4512.4 |
   4754.2 |
   4996.0 |##
   5237.8 |
   5479.6 |
   5721.4 |
   5963.2 |
   6205.0 |
   6446.8 |
   6688.6 |
   6930.4 |
   7172.2 |#
   7414.0 |
   7655.8 |
   7897.6 |
   8139.4 |
   8381.2 |
   8623.0 |
  (6 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=43.5% (high variance, measurements may be unstable)
- **wide-rung-ragged**: CV=43.1% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=42.3% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=44.2% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=49.2% (high variance, measurements may be unstable)
