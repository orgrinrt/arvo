# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround is fastest but the noisiest (CV 47.3%)

wide-rung-wordround wins on median (4.64 us) yet has the highest variance (CV 47.3%), while wide-rung-ragged is the steadiest (CV 22.8%, 4.72 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-wordround shows alternating (throttle bounce) (autocorr -0.75)

wide-rung-wordround's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (116 ns) is smaller than the fastest variant's own run-to-run std-dev (2.19 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-wordround vs stability leader wide-rung-ragged (+2% speed for 2.1x steadier)

wide-rung-wordround is fastest (4.64 us, CV 47.3%); wide-rung-ragged gives up 1.8% median for 2.1x lower variance (CV 22.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-wordround is inconsistent: worst-20% is 1.8x its best-20%

wide-rung-wordround's best 20% of batches run at 4.57 us but its worst 20% at 8.08 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 2.5% of the fastest

All 5 variants sit between 4.64 us and 4.75 us - a 2.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-wordround's edge over baseline is significant but tiny (-32 ns, 0.69%)

wide-rung-wordround differs from baseline wide-rung-align16 by -32 ns (0.69%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround** at 4638.6 ns median (-0.6% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.02x (fastest 4638.6 ns, slowest 4754.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 5201ns | 4751ns | 4737ns | 4750ns | 7018ns | base |
| wide-rung-ragged | 5050ns | 4808ns | 4784ns | 4808ns | 6042ns | -2.91% |
| wide-rung-ragged-overread | 5048ns | 4841ns | 4667ns | 4839ns | 6057ns | -2.94% |
| wide-rung-wordround | 5416ns | 4721ns | 4659ns | 4714ns | 8278ns | +4.13% |
| wide-rung-wordround-alias | 5069ns | 4721ns | 4665ns | 4719ns | 6522ns | -2.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 5105ns | 4655ns | 6874ns | base | 1.203 |
| wide-rung-ragged | 4953ns | 4702ns | 5895ns | -2.98% | 1.240 |
| wide-rung-ragged-overread | 4953ns | 4585ns | 5919ns | -2.99% | 1.240 |
| wide-rung-wordround | 5309ns | 4574ns | 8083ns | +3.98% | 1.157 |
| wide-rung-wordround-alias | 4974ns | 4583ns | 6375ns | -2.58% | 1.235 |

## Performance model

- Peak throughput: **1.343 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 6144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.316 | 98.0% |
| wide-rung-ragged | 1.301 | 96.8% |
| wide-rung-ragged-overread | 1.292 | 96.2% |
| wide-rung-wordround | 1.325 | 98.6% |
| wide-rung-wordround-alias | 1.324 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 5201ns | 5201ns | base |
| wide-rung-ragged | 5050ns | 5050ns | -2.91% |
| wide-rung-ragged-overread | 5048ns | 5048ns | -2.94% |
| wide-rung-wordround | 5416ns | 5416ns | +4.13% |
| wide-rung-wordround-alias | 5069ns | 5069ns | -2.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4668ns | base | --- | [4658, 4671] | --- | --- | --- | --- |
| wide-rung-ragged | 4723ns | +56.1ns (+1.2%) | [+51, +65]ns | [4715, 4728] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 4754ns | +82.5ns (+1.8%) | [+58, +104]ns | [4740, 4769] | YES | 0.0022 | 0.0022 | 0 |
| wide-rung-wordround | 4639ns | -32.2ns (-0.7%) | [-62, -18]ns | [4608, 4648] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 4640ns | -32.1ns (-0.7%) | [-49, -20]ns | [4636, 4646] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 4670ns | +0.9% | -1.7% | -1.1% | -0.6% |
| 1 | 2 | 4663ns | +1.0% | +2.2% | -0.5% | -0.5% |
| 1 | 3 | 4663ns | +1.5% | +2.4% | -1.0% | -0.4% |
| 1 | 4 | 4669ns | +1.2% | +1.5% | -0.7% | -1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.248 | moderate- |
| wide-rung-ragged | 0.071 | ok |
| wide-rung-ragged-overread | -0.086 | ok |
| wide-rung-wordround | -0.747 | HIGH- (thermal bounce) |
| wide-rung-wordround-alias | -0.183 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 0/4, lost 4/4
- **wide-rung-ragged-overread**: won 1/4, lost 3/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 26.4ns | 5105.4ns | 0.5% |  |
| wide-rung-ragged | 26.2ns | 4953.1ns | 0.5% |  |
| wide-rung-ragged-overread | 26.1ns | 4953.0ns | 0.5% |  |
| wide-rung-wordround | 28.6ns | 5308.7ns | 0.5% |  |
| wide-rung-wordround-alias | 26.4ns | 4973.8ns | 0.5% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 4655.3-6874.1 ns)
   4655.3 |########################################
   4766.2 |
   4877.2 |
   4988.1 |
   5099.0 |
   5210.0 |
   5320.9 |
   5431.9 |
   5542.8 |
   5653.7 |
   5764.7 |
   5875.6 |
   5986.6 |
   6097.5 |
   6208.4 |
   6319.4 |
   6430.3 |
   6541.2 |#
   6652.2 |
   6763.1 |#
  (4 below, 2 above range)

wide-rung-ragged (n=40, range 4701.9-5895.0 ns)
   4701.9 |########################################
   4761.5 |####
   4821.2 |
   4880.8 |#
   4940.5 |
   5000.1 |
   5059.8 |
   5119.5 |
   5179.1 |
   5238.8 |
   5298.4 |
   5358.1 |
   5417.8 |
   5477.4 |
   5537.1 |
   5596.7 |
   5656.4 |
   5716.0 |
   5775.7 |
   5835.4 |
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 4585.0-5919.2 ns)
   4585.0 |###########
   4651.8 |###
   4718.5 |########################################
   4785.2 |#####
   4851.9 |#
   4918.6 |
   4985.3 |
   5052.0 |
   5118.7 |
   5185.4 |
   5252.1 |
   5318.9 |
   5385.6 |
   5452.3 |
   5519.0 |
   5585.7 |
   5652.4 |
   5719.1 |
   5785.8 |
   5852.5 |
  (1 below, 2 above range)

wide-rung-wordround (n=40, range 4573.5-8083.2 ns)
   4573.5 |########################################
   4749.0 |
   4924.5 |
   5100.0 |#
   5275.5 |
   5450.9 |
   5626.4 |
   5801.9 |
   5977.4 |
   6152.9 |
   6328.4 |
   6503.8 |
   6679.3 |
   6854.8 |
   7030.3 |#
   7205.8 |
   7381.3 |
   7556.8 |
   7732.2 |
   7907.7 |
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 4582.8-6375.1 ns)
   4582.8 |########################################
   4672.4 |
   4762.0 |
   4851.7 |#
   4941.3 |
   5030.9 |
   5120.5 |
   5210.1 |
   5299.7 |
   5389.4 |
   5479.0 |
   5568.6 |
   5658.2 |
   5747.8 |
   5837.4 |
   5927.1 |
   6016.7 |
   6106.3 |
   6195.9 |
   6285.5 |
  (4 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=30.1% (high variance, measurements may be unstable)
- **wide-rung-ragged**: CV=21.7% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=22.2% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=41.3% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=30.9% (high variance, measurements may be unstable)
