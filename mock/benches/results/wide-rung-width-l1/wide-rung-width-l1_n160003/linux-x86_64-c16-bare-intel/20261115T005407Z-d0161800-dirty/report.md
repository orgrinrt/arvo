# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged shows alternating (throttle bounce) (autocorr -0.61)

wide-rung-ragged's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (304 ns) is smaller than the fastest variant's own run-to-run std-dev (1.78 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-ragged vs stability leader wide-rung-wordround-alias (+8% speed for 1.5x steadier)

wide-rung-ragged is fastest (4.03 us, CV 44.2%); wide-rung-wordround-alias gives up 7.5% median for 1.5x lower variance (CV 30.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-align16 is inconsistent: worst-20% is 2.2x its best-20%

wide-rung-align16's best 20% of batches run at 4.32 us but its worst 20% at 9.33 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (-18 ns, 0.42%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by -18 ns (0.42%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged** at 4029.8 ns median (-7.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.08x (fastest 4029.8 ns, slowest 4334.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 5442ns | 4416ns | 4403ns | 4416ns | 9556ns | base |
| wide-rung-ragged | 4769ns | 4093ns | 4081ns | 4100ns | 7463ns | -12.36% |
| wide-rung-ragged-overread | 4808ns | 4380ns | 3604ns | 4383ns | 7285ns | -11.65% |
| wide-rung-wordround | 5066ns | 4418ns | 4405ns | 4417ns | 7670ns | -6.91% |
| wide-rung-wordround-alias | 4687ns | 4415ns | 4391ns | 4418ns | 5789ns | -13.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 5331ns | 4322ns | 9330ns | base | 1.537 |
| wide-rung-ragged | 4682ns | 4018ns | 7290ns | -12.18% | 1.750 |
| wide-rung-ragged-overread | 4730ns | 3551ns | 7142ns | -11.27% | 1.732 |
| wide-rung-wordround | 4962ns | 4323ns | 7483ns | -6.91% | 1.651 |
| wide-rung-wordround-alias | 4592ns | 4310ns | 5642ns | -13.86% | 1.784 |

## Performance model

- Peak throughput: **2.307 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.890 | 81.9% |
| wide-rung-ragged | 2.033 | 88.1% |
| wide-rung-ragged-overread | 1.898 | 82.3% |
| wide-rung-wordround | 1.891 | 82.0% |
| wide-rung-wordround-alias | 1.890 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 5442ns | 5442ns | base |
| wide-rung-ragged | 4769ns | 4769ns | -12.36% |
| wide-rung-ragged-overread | 4808ns | 4808ns | -11.65% |
| wide-rung-wordround | 5066ns | 5066ns | -6.91% |
| wide-rung-wordround-alias | 4687ns | 4687ns | -13.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4333ns | base | --- | [4328, 4340] | --- | --- | --- | --- |
| wide-rung-ragged | 4030ns | -305.8ns (-7.1%) | [-311, -301]ns | [4020, 4033] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 4316ns | -18.1ns (-0.4%) | [-75, -13]ns | [4315, 4326] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-wordround | 4333ns | no significant difference | [-9, +6]ns | [4329, 4341] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-wordround-alias | 4334ns | no significant difference | [-12, +11]ns | [4332, 4343] | no | 0.5728 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 4334ns | -7.1% | -0.4% | -0.1% | +0.1% |
| 1 | 2 | 4330ns | -6.5% | -0.2% | +0.3% | +0.2% |
| 1 | 3 | 4331ns | -7.2% | -18.3% | +0.1% | +0.2% |
| 1 | 4 | 4346ns | -7.3% | -0.6% | -0.3% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.132 | ok |
| wide-rung-ragged | -0.613 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | -0.422 | moderate- |
| wide-rung-wordround | -0.412 | moderate- |
| wide-rung-wordround-alias | -0.066 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 2/4, lost 1/4
- **wide-rung-wordround-alias**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 29.4ns | 5331.0ns | 0.6% |  |
| wide-rung-ragged | 28.0ns | 4681.6ns | 0.6% |  |
| wide-rung-ragged-overread | 26.6ns | 4729.9ns | 0.6% |  |
| wide-rung-wordround | 27.5ns | 4962.3ns | 0.6% |  |
| wide-rung-wordround-alias | 25.3ns | 4592.3ns | 0.6% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 4321.5-9330.5 ns)
   4321.5 |########################################
   4572.0 |
   4822.4 |
   5072.9 |
   5323.3 |
   5573.8 |
   5824.2 |
   6074.7 |#
   6325.1 |
   6575.6 |
   6826.0 |
   7076.4 |
   7326.9 |
   7577.3 |
   7827.8 |#
   8078.2 |
   8328.7 |
   8579.1 |
   8829.6 |
   9080.0 |
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 4017.8-7289.8 ns)
   4017.8 |########################################
   4181.4 |#
   4345.0 |
   4508.6 |
   4672.2 |
   4835.8 |
   4999.4 |
   5163.0 |
   5326.6 |
   5490.2 |
   5653.8 |
   5817.4 |
   5981.0 |
   6144.6 |
   6308.2 |
   6471.8 |
   6635.4 |
   6799.0 |
   6962.6 |
   7126.2 |
  (5 below, 5 above range)

wide-rung-ragged-overread (n=40, range 3550.9-7142.0 ns)
   3550.9 |#
   3730.4 |
   3910.0 |
   4089.6 |
   4269.1 |########################################
   4448.7 |
   4628.2 |#
   4807.8 |
   4987.4 |
   5166.9 |
   5346.5 |
   5526.0 |
   5705.6 |
   5885.1 |
   6064.7 |
   6244.3 |
   6423.8 |
   6603.4 |
   6782.9 |
   6962.5 |
  (7 below, 4 above range)

wide-rung-wordround (n=40, range 4323.1-7483.3 ns)
   4323.1 |########################################
   4481.1 |
   4639.1 |
   4797.1 |
   4955.1 |
   5113.2 |
   5271.2 |
   5429.2 |
   5587.2 |#
   5745.2 |
   5903.2 |
   6061.2 |
   6219.2 |
   6377.2 |
   6535.3 |
   6693.3 |
   6851.3 |
   7009.3 |#
   7167.3 |
   7325.3 |
  (5 below, 3 above range)

wide-rung-wordround-alias (n=40, range 4310.2-5642.2 ns)
   4310.2 |########################################
   4376.8 |
   4443.4 |
   4510.0 |
   4576.6 |
   4643.2 |
   4709.8 |
   4776.4 |
   4843.0 |
   4909.6 |
   4976.2 |#
   5042.8 |
   5109.4 |
   5176.0 |
   5242.6 |
   5309.2 |
   5375.8 |
   5442.4 |
   5509.0 |
   5575.6 |
  (5 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=48.9% (high variance, measurements may be unstable)
- **wide-rung-ragged**: CV=38.0% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=37.0% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=39.4% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=28.4% (high variance, measurements may be unstable)
