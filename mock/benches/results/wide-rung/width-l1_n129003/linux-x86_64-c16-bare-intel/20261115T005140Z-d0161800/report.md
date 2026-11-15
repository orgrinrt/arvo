# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (4.64 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 4.45 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (wide-rung-ragged-overread, wide-rung-wordround) are a dead heat (<1%)

wide-rung-ragged-overread (4.45 us) and wide-rung-wordround (4.49 us) differ by 0.91%, inside the noise, even though the wider field spreads 4.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged shows alternating (throttle bounce) (autocorr -0.63)

wide-rung-ragged's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (186 ns) is smaller than the fastest variant's own run-to-run std-dev (1.27 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-ragged-overread vs stability leader wide-rung-ragged (+2% speed for 4.1x steadier)

wide-rung-ragged-overread is fastest (4.45 us, CV 28.6%); wide-rung-ragged gives up 1.6% median for 4.1x lower variance (CV 7.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-wordround is inconsistent: worst-20% is 2.0x its best-20%

wide-rung-wordround's best 20% of batches run at 3.70 us but its worst 20% at 7.50 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 4.2% of the fastest

All 5 variants sit between 4.45 us and 4.64 us - a 4.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 4451.4 ns median (-4.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.04x (fastest 4451.4 ns, slowest 4637.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 6013ns | 4722ns | 4676ns | 5307ns | 9468ns | base |
| wide-rung-ragged | 4647ns | 4588ns | 4574ns | 4591ns | 4888ns | -22.71% |
| wide-rung-ragged-overread | 4605ns | 4520ns | 3697ns | 4465ns | 5934ns | -23.42% |
| wide-rung-wordround | 5003ns | 4576ns | 3766ns | 4522ns | 7681ns | -16.81% |
| wide-rung-wordround-alias | 4986ns | 4608ns | 4551ns | 4601ns | 6574ns | -17.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 5896ns | 4590ns | 9263ns | base | 1.389 |
| wide-rung-ragged | 4573ns | 4507ns | 4786ns | -22.44% | 1.791 |
| wide-rung-ragged-overread | 4528ns | 3644ns | 5816ns | -23.20% | 1.809 |
| wide-rung-wordround | 4902ns | 3696ns | 7498ns | -16.86% | 1.671 |
| wide-rung-wordround-alias | 4886ns | 4467ns | 6424ns | -17.12% | 1.676 |

## Performance model

- Peak throughput: **2.248 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.766 | 78.6% |
| wide-rung-ragged | 1.811 | 80.6% |
| wide-rung-ragged-overread | 1.840 | 81.9% |
| wide-rung-wordround | 1.824 | 81.1% |
| wide-rung-wordround-alias | 1.814 | 80.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 6013ns | 6013ns | base |
| wide-rung-ragged | 4647ns | 4647ns | -22.71% |
| wide-rung-ragged-overread | 4605ns | 4605ns | -23.42% |
| wide-rung-wordround | 5003ns | 5003ns | -16.81% |
| wide-rung-wordround-alias | 4986ns | 4986ns | -17.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4638ns | base | --- | [4627, 4640] | --- | --- | --- | --- |
| wide-rung-ragged | 4523ns | -110.6ns (-2.4%) | [-126, -99]ns | [4520, 4527] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 4451ns | -182.7ns (-3.9%) | [-193, -171]ns | [4442, 4457] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 4492ns | -166.7ns (-3.6%) | [-234, -95]ns | [4409, 4528] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 4515ns | -122.8ns (-2.6%) | [-165, -88]ns | [4493, 4535] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 7418ns | -39.0% | -50.8% | -50.2% | -39.6% |
| 1 | 2 | 4633ns | -2.3% | -3.7% | -2.2% | -3.2% |
| 1 | 3 | 4590ns | -1.6% | -2.9% | -1.4% | -1.3% |
| 1 | 4 | 4633ns | -2.4% | -4.0% | -4.8% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.075 | ok |
| wide-rung-ragged | -0.629 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | -0.088 | ok |
| wide-rung-wordround | -0.121 | ok |
| wide-rung-wordround-alias | 0.299 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 31.5ns | 5896.0ns | 0.5% |  |
| wide-rung-ragged | 24.8ns | 4572.9ns | 0.5% |  |
| wide-rung-ragged-overread | 24.9ns | 4528.1ns | 0.5% |  |
| wide-rung-wordround | 26.5ns | 4901.7ns | 0.5% |  |
| wide-rung-wordround-alias | 26.3ns | 4886.5ns | 0.5% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 4590.3-9263.4 ns)
   4590.3 |########################################
   4824.0 |
   5057.6 |
   5291.3 |
   5524.9 |
   5758.6 |
   5992.2 |
   6225.9 |
   6459.6 |
   6693.2 |
   6926.9 |
   7160.5 |
   7394.2 |##################
   7627.8 |
   7861.5 |
   8095.1 |
   8328.8 |
   8562.4 |
   8796.1 |
   9029.8 |
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 4507.2-4786.4 ns)
   4507.2 |#################################
   4521.1 |########################################
   4535.1 |###########
   4549.1 |
   4563.0 |
   4577.0 |
   4591.0 |
   4604.9 |
   4618.9 |
   4632.8 |
   4646.8 |
   4660.8 |
   4674.7 |
   4688.7 |
   4702.7 |
   4716.6 |
   4730.6 |
   4744.5 |
   4758.5 |
   4772.5 |
  (1 below, 1 above range)

wide-rung-ragged-overread (n=40, range 3644.0-5815.7 ns)
   3644.0 |###########
   3752.6 |
   3861.2 |#
   3969.8 |
   4078.4 |
   4186.9 |
   4295.5 |
   4404.1 |########################################
   4512.7 |
   4621.3 |
   4729.9 |
   4838.4 |
   4947.0 |
   5055.6 |
   5164.2 |
   5272.8 |
   5381.3 |
   5489.9 |
   5598.5 |
   5707.1 |
  (1 below, 2 above range)

wide-rung-wordround (n=40, range 3696.1-7497.5 ns)
   3696.1 |###########
   3886.2 |
   4076.2 |
   4266.3 |####################
   4456.4 |########################################
   4646.4 |
   4836.5 |
   5026.6 |
   5216.7 |
   5406.7 |
   5596.8 |
   5786.9 |
   5977.0 |
   6167.0 |
   6357.1 |
   6547.2 |
   6737.2 |
   6927.3 |
   7117.4 |
   7307.5 |
  (4 below, 4 above range)

wide-rung-wordround-alias (n=40, range 4467.0-6424.1 ns)
   4467.0 |########################################
   4564.9 |
   4662.7 |
   4760.6 |
   4858.5 |
   4956.3 |
   5054.2 |
   5152.0 |
   5249.9 |#
   5347.7 |
   5445.6 |
   5543.4 |
   5641.3 |
   5739.1 |
   5837.0 |
   5934.8 |
   6032.7 |
   6130.6 |
   6228.4 |
   6326.3 |
  (3 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=37.4% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=28.1% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=41.7% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=38.5% (high variance, measurements may be unstable)
