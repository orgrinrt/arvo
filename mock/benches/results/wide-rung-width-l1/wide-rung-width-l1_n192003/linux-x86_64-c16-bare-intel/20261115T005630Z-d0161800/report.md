# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (4.11 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 4.05 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (59 ns) is smaller than the fastest variant's own run-to-run std-dev (1.82 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-ragged vs stability leader wide-rung-wordround-alias (+1% speed for 1.2x steadier)

wide-rung-ragged is fastest (4.05 us, CV 44.8%); wide-rung-wordround-alias gives up 0.7% median for 1.2x lower variance (CV 38.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-ragged-overread is inconsistent: worst-20% is 2.4x its best-20%

wide-rung-ragged-overread's best 20% of batches run at 3.32 us but its worst 20% at 7.79 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 1.4% of the fastest

All 5 variants sit between 4.05 us and 4.11 us - a 1.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-wordround's edge over baseline is significant but tiny (-37 ns, 0.90%)

wide-rung-wordround differs from baseline wide-rung-align16 by -37 ns (0.90%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged** at 4050.8 ns median (-1.4% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.01x (fastest 4050.8 ns, slowest 4109.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 5058ns | 4192ns | 4110ns | 4195ns | 8594ns | base |
| wide-rung-ragged | 4542ns | 4133ns | 3354ns | 4103ns | 7044ns | -10.21% |
| wide-rung-ragged-overread | 4739ns | 4135ns | 3383ns | 4106ns | 7997ns | -6.30% |
| wide-rung-wordround | 4838ns | 4161ns | 4152ns | 4163ns | 7546ns | -4.36% |
| wide-rung-wordround-alias | 4658ns | 4160ns | 4149ns | 4160ns | 6662ns | -7.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4948ns | 4030ns | 8370ns | base | 1.656 |
| wide-rung-ragged | 4441ns | 3287ns | 6853ns | -10.24% | 1.845 |
| wide-rung-ragged-overread | 4637ns | 3316ns | 7794ns | -6.29% | 1.767 |
| wide-rung-wordround | 4731ns | 4070ns | 7344ns | -4.38% | 1.732 |
| wide-rung-wordround-alias | 4557ns | 4067ns | 6485ns | -7.89% | 1.798 |

## Performance model

- Peak throughput: **2.492 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.993 | 80.0% |
| wide-rung-ragged | 2.022 | 81.1% |
| wide-rung-ragged-overread | 2.021 | 81.1% |
| wide-rung-wordround | 2.009 | 80.6% |
| wide-rung-wordround-alias | 2.009 | 80.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 5058ns | 5058ns | base |
| wide-rung-ragged | 4542ns | 4542ns | -10.21% |
| wide-rung-ragged-overread | 4739ns | 4739ns | -6.30% |
| wide-rung-wordround | 4838ns | 4838ns | -4.36% |
| wide-rung-wordround-alias | 4658ns | 4658ns | -7.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4109ns | base | --- | [4107, 4120] | --- | --- | --- | --- |
| wide-rung-ragged | 4051ns | -68.4ns (-1.7%) | [-428, -58]ns | [4049, 4060] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 4054ns | -61.5ns (-1.5%) | [-75, -51]ns | [4047, 4057] | YES | 0.0166 | 0.0166 | 0 |
| wide-rung-wordround | 4079ns | -37.0ns (-0.9%) | [-43, -21]ns | [4077, 4085] | YES | 0.0030 | 0.0022 | 0 |
| wide-rung-wordround-alias | 4078ns | -33.7ns (-0.8%) | [-43, -26]ns | [4076, 4083] | YES | 0.0004 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 4041ns | -18.7% | +0.4% | +1.1% | +0.7% |
| 1 | 2 | 4114ns | -1.4% | -19.5% | -1.1% | -0.9% |
| 1 | 3 | 4118ns | -1.7% | -1.4% | -1.0% | -0.8% |
| 1 | 4 | 4120ns | -1.5% | -1.8% | -0.9% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.033 | ok |
| wide-rung-ragged | -0.086 | ok |
| wide-rung-ragged-overread | -0.427 | moderate- |
| wide-rung-wordround | -0.341 | moderate- |
| wide-rung-wordround-alias | -0.140 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 3/4, lost 1/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 29.3ns | 4948.0ns | 0.6% |  |
| wide-rung-ragged | 26.2ns | 4441.1ns | 0.6% |  |
| wide-rung-ragged-overread | 27.3ns | 4636.6ns | 0.6% |  |
| wide-rung-wordround | 28.3ns | 4731.1ns | 0.6% |  |
| wide-rung-wordround-alias | 26.9ns | 4557.4ns | 0.6% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 4030.1-8370.2 ns)
   4030.1 |########################################
   4247.1 |#
   4464.1 |
   4681.1 |
   4898.1 |#
   5115.1 |
   5332.1 |
   5549.1 |
   5766.1 |#
   5983.1 |
   6200.1 |
   6417.1 |
   6634.1 |
   6851.1 |
   7068.1 |
   7285.1 |
   7502.1 |
   7719.1 |
   7936.2 |
   8153.2 |
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 3286.9-6852.9 ns)
   3286.9 |####
   3465.2 |
   3643.5 |
   3821.8 |
   4000.1 |########################################
   4178.4 |
   4356.7 |
   4535.0 |
   4713.3 |
   4891.6 |
   5069.9 |
   5248.2 |
   5426.5 |
   5604.8 |#
   5783.1 |
   5961.4 |
   6139.7 |
   6318.0 |#
   6496.3 |
   6674.6 |
  (6 below, 3 above range)

wide-rung-ragged-overread (n=40, range 3315.7-7794.5 ns)
   3315.7 |####
   3539.7 |
   3763.6 |
   3987.5 |########################################
   4211.5 |
   4435.4 |
   4659.3 |#
   4883.3 |
   5107.2 |
   5331.2 |#
   5555.1 |
   5779.0 |
   6003.0 |
   6226.9 |
   6450.8 |
   6674.8 |
   6898.7 |
   7122.7 |
   7346.6 |
   7570.5 |
  (6 below, 4 above range)

wide-rung-wordround (n=40, range 4069.8-7343.5 ns)
   4069.8 |########################################
   4233.5 |
   4397.2 |
   4560.9 |
   4724.5 |
   4888.2 |
   5051.9 |
   5215.6 |
   5379.3 |#
   5543.0 |
   5706.7 |
   5870.4 |
   6034.0 |
   6197.7 |
   6361.4 |
   6525.1 |
   6688.8 |
   6852.5 |
   7016.2 |
   7179.9 |
  (4 below, 4 above range)

wide-rung-wordround-alias (n=40, range 4066.5-6485.0 ns)
   4066.5 |########################################
   4187.4 |
   4308.4 |#
   4429.3 |
   4550.2 |
   4671.1 |
   4792.1 |
   4913.0 |
   5033.9 |
   5154.8 |
   5275.8 |
   5396.7 |
   5517.6 |
   5638.5 |#
   5759.4 |
   5880.4 |
   6001.3 |
   6122.2 |
   6243.1 |
   6364.1 |
  (3 below, 3 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=47.8% (high variance, measurements may be unstable)
- **wide-rung-ragged**: CV=40.9% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=48.5% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=40.3% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=34.0% (high variance, measurements may be unstable)
