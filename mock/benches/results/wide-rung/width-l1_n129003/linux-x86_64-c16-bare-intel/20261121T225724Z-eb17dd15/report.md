# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround dominates: 14% faster than the next best (wide-rung-align16)

wide-rung-wordround (3.70 us) leads wide-rung-align16 (4.21 us) by 14%, a clear separation rather than a photo finish. CV 49.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-wordround is fastest but the noisiest (CV 49.1%)

wide-rung-wordround wins on median (3.70 us) yet has the highest variance (CV 49.1%), while wide-rung-align16 is the steadiest (CV 10.1%, 4.21 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-align16 shows alternating (throttle bounce) (autocorr -0.75)

wide-rung-align16's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (811 ns) is smaller than the fastest variant's own run-to-run std-dev (1.82 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wide-rung-wordround-alias is inconsistent: worst-20% is 1.9x its best-20%

wide-rung-wordround-alias's best 20% of batches run at 3.61 us but its worst 20% at 6.75 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-wordround** at 3704.8 ns median (-12.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.22x (fastest 3704.8 ns, slowest 4516.2 ns)

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
| wide-rung-align16 | 4288ns | 4303ns | 3848ns | 4285ns | 4736ns | base |
| wide-rung-ragged | 4431ns | 4579ns | 3737ns | 4207ns | 5796ns | +3.33% |
| wide-rung-ragged-overread | 4582ns | 4521ns | 3689ns | 4218ns | 6568ns | +6.85% |
| wide-rung-wordround | 4299ns | 3772ns | 3674ns | 3768ns | 6516ns | +0.26% |
| wide-rung-wordround-alias | 4694ns | 4597ns | 3679ns | 4292ns | 6917ns | +9.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4209ns | 3782ns | 4644ns | base | 1.946 |
| wide-rung-ragged | 4362ns | 3686ns | 5686ns | +3.63% | 1.878 |
| wide-rung-ragged-overread | 4506ns | 3637ns | 6422ns | +7.04% | 1.818 |
| wide-rung-wordround | 4215ns | 3606ns | 6369ns | +0.14% | 1.943 |
| wide-rung-wordround-alias | 4602ns | 3611ns | 6753ns | +9.32% | 1.780 |

## Performance model

- Peak throughput: **2.271 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.947 | 85.7% |
| wide-rung-ragged | 1.814 | 79.9% |
| wide-rung-ragged-overread | 1.838 | 80.9% |
| wide-rung-wordround | 2.211 | 97.3% |
| wide-rung-wordround-alias | 1.814 | 79.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4288ns | 4288ns | base |
| wide-rung-ragged | 4431ns | 4431ns | +3.33% |
| wide-rung-ragged-overread | 4582ns | 4582ns | +6.85% |
| wide-rung-wordround | 4299ns | 4299ns | +0.26% |
| wide-rung-wordround-alias | 4694ns | 4694ns | +9.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4208ns | base | --- | [3791, 4624] | --- | --- | --- | --- |
| wide-rung-ragged | 4516ns | -98.8ns (-2.3%) | [-100, -96]ns | [3696, 4523] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 4457ns | -152.7ns (-3.6%) | [-163, -136]ns | [3650, 4459] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| wide-rung-wordround | 3705ns | -90.3ns (-2.1%) | [-928, -81]ns | [3704, 3712] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 4515ns | no significant difference | [-105, +44]ns | [3706, 4520] | no | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 3785ns | -2.6% | +17.9% | -2.1% | -2.2% |
| 1 | 2 | 4623ns | -2.2% | -3.6% | -22.0% | -21.8% |
| 1 | 3 | 3783ns | -2.5% | -3.9% | -1.9% | +19.6% |
| 1 | 4 | 4632ns | -2.2% | -21.3% | -19.9% | -2.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.748 | HIGH- (thermal bounce) |
| wide-rung-ragged | -0.748 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | 0.249 | moderate+ |
| wide-rung-wordround | -0.396 | moderate- |
| wide-rung-wordround-alias | 0.221 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 3/4, lost 1/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 21.5ns | 4209.4ns | 0.5% |  |
| wide-rung-ragged | 23.0ns | 4362.4ns | 0.5% |  |
| wide-rung-ragged-overread | 24.1ns | 4505.8ns | 0.5% |  |
| wide-rung-wordround | 22.0ns | 4215.4ns | 0.5% |  |
| wide-rung-wordround-alias | 24.7ns | 4601.7ns | 0.5% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 3782.1-4643.7 ns)
   3782.1 |#################################
   3825.2 |
   3868.3 |
   3911.4 |
   3954.4 |
   3997.5 |
   4040.6 |
   4083.7 |
   4126.7 |
   4169.8 |
   4212.9 |
   4256.0 |
   4299.0 |
   4342.1 |
   4385.2 |
   4428.3 |
   4471.4 |
   4514.4 |
   4557.5 |
   4600.6 |########################################
  (4 below, 1 above range)

wide-rung-ragged (n=40, range 3685.7-5686.1 ns)
   3685.7 |########################################
   3785.7 |
   3885.7 |
   3985.8 |
   4085.8 |
   4185.8 |
   4285.8 |
   4385.8 |
   4485.9 |########################################
   4585.9 |
   4685.9 |##
   4785.9 |##
   4885.9 |
   4986.0 |#####
   5086.0 |
   5186.0 |
   5286.0 |
   5386.1 |
   5486.1 |
   5586.1 |
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 3637.4-6422.2 ns)
   3637.4 |#########################
   3776.7 |
   3915.9 |
   4055.1 |
   4194.4 |
   4333.6 |########################################
   4472.8 |####
   4612.1 |
   4751.3 |
   4890.6 |
   5029.8 |
   5169.0 |
   5308.3 |
   5447.5 |
   5586.8 |
   5726.0 |
   5865.2 |
   6004.5 |
   6143.7 |##
   6282.9 |
  (6 below, 3 above range)

wide-rung-wordround (n=40, range 3606.4-6368.6 ns)
   3606.4 |########################################
   3744.6 |#
   3882.7 |
   4020.8 |
   4158.9 |
   4297.0 |
   4435.1 |#
   4573.2 |
   4711.3 |
   4849.4 |
   4987.5 |
   5125.6 |
   5263.7 |
   5401.9 |
   5540.0 |
   5678.1 |
   5816.2 |
   5954.3 |
   6092.4 |
   6230.5 |
  (7 below, 3 above range)

wide-rung-wordround-alias (n=40, range 3611.4-6752.9 ns)
   3611.4 |#########################
   3768.4 |
   3925.5 |
   4082.6 |
   4239.7 |
   4396.7 |########################################
   4553.8 |
   4710.9 |##
   4868.0 |
   5025.1 |
   5182.1 |
   5339.2 |
   5496.3 |
   5653.4 |
   5810.4 |
   5967.5 |
   6124.6 |
   6281.7 |
   6438.7 |
   6595.8 |
  (5 below, 3 above range)

```

## Diagnostics

- **wide-rung-ragged**: CV=24.1% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=34.7% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=43.2% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=37.0% (high variance, measurements may be unstable)
