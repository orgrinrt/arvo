# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (5.25 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround at 5.24 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (8 ns) is smaller than the fastest variant's own run-to-run std-dev (66 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.1% of the fastest

All 5 variants sit between 5.24 us and 5.25 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged's edge over baseline is significant but tiny (-2 ns, 0.04%)

wide-rung-ragged differs from baseline wide-rung-align16 by -2 ns (0.04%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround** at 5243.1 ns median (-0.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.00x (fastest 5243.1 ns, slowest 5250.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 5333ns | 5316ns | 5308ns | 5319ns | 5400ns | base |
| wide-rung-ragged | 5362ns | 5310ns | 5305ns | 5317ns | 5553ns | +0.54% |
| wide-rung-ragged-overread | 5318ns | 5308ns | 5303ns | 5309ns | 5359ns | -0.28% |
| wide-rung-wordround | 5339ns | 5307ns | 5302ns | 5316ns | 5446ns | +0.11% |
| wide-rung-wordround-alias | 5378ns | 5317ns | 5302ns | 5332ns | 5593ns | +0.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 5266ns | 5243ns | 5332ns | base | 1.556 |
| wide-rung-ragged | 5295ns | 5241ns | 5478ns | +0.54% | 1.547 |
| wide-rung-ragged-overread | 5253ns | 5240ns | 5292ns | -0.26% | 1.560 |
| wide-rung-wordround | 5273ns | 5239ns | 5373ns | +0.13% | 1.553 |
| wide-rung-wordround-alias | 5311ns | 5239ns | 5516ns | +0.84% | 1.543 |

## Performance model

- Peak throughput: **1.564 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.560 | 99.8% |
| wide-rung-ragged | 1.562 | 99.9% |
| wide-rung-ragged-overread | 1.562 | 99.9% |
| wide-rung-wordround | 1.562 | 99.9% |
| wide-rung-wordround-alias | 1.560 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 5333ns | 5333ns | base |
| wide-rung-ragged | 5362ns | 5362ns | +0.54% |
| wide-rung-ragged-overread | 5318ns | 5318ns | -0.28% |
| wide-rung-wordround | 5339ns | 5339ns | +0.11% |
| wide-rung-wordround-alias | 5378ns | 5378ns | +0.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 5251ns | base | --- | [5247, 5260] | --- | --- | --- | --- |
| wide-rung-ragged | 5246ns | no significant difference | [-7, +1]ns | [5244, 5247] | no | 0.2661 | 0.1996 | 1 |
| wide-rung-ragged-overread | 5243ns | -6.4ns (-0.1%) | [-15, -3]ns | [5242, 5245] | YES | 0.0027 | 0.0007 | 0 |
| wide-rung-wordround | 5243ns | -5.4ns (-0.1%) | [-10, -0]ns | [5242, 5249] | YES (adj: no) | 0.1065 | 0.0533 | 1 |
| wide-rung-wordround-alias | 5250ns | no significant difference | [-5, +11]ns | [5245, 5282] | no | 0.7493 | 0.7493 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 5252ns | -0.2% | -0.2% | +1.0% | -0.1% |
| 2 | 5252ns | -0.2% | -0.2% | -0.2% | -0.0% |
| 3 | 5260ns | -0.3% | -0.4% | -0.4% | +18.9% |
| 4 | 5275ns | -0.6% | -0.6% | -0.6% | +5.7% |
| 5 | 5260ns | -0.3% | -0.3% | -0.4% | +3.0% |
| 6 | 5262ns | -0.3% | -0.4% | +0.9% | +1.7% |
| 7 | 5261ns | -0.3% | +0.5% | -0.3% | +1.7% |
| 8 | 5351ns | -2.0% | -2.0% | -1.0% | -0.3% |
| 9 | 5266ns | +0.8% | -0.5% | -0.5% | -0.2% |
| 10 | 5249ns | -0.1% | -0.1% | +1.1% | -0.1% |
| 11 | 5409ns | -3.1% | -2.8% | +2.5% | -3.1% |
| 12 | 5245ns | -0.0% | +1.5% | +2.8% | +0.0% |
| 13 | 5248ns | +0.8% | +0.8% | +4.9% | +0.1% |
| 14 | 5243ns | +3.6% | +0.1% | +0.2% | -0.1% |
| 15 | 5279ns | +19.1% | -0.7% | -0.7% | -0.8% |
| 16 | 5250ns | +3.4% | -0.1% | +1.1% | +1.0% |
| 17 | 5252ns | +2.0% | +0.9% | -0.2% | +0.2% |
| 18 | 5242ns | +1.7% | +0.1% | +0.1% | -0.0% |
| 19 | 5245ns | +1.5% | -0.1% | -0.2% | -0.2% |
| 20 | 5261ns | +1.7% | -0.3% | -0.3% | -0.4% |
| 21 | 5248ns | -0.2% | -0.1% | +1.2% | +1.4% |
| 22 | 5242ns | +0.3% | -0.1% | +0.0% | +1.6% |
| 23 | 5241ns | +0.0% | -0.0% | -0.0% | +1.5% |
| 24 | 5243ns | +0.0% | +0.7% | +0.4% | +1.5% |
| 25 | 5249ns | -0.1% | -0.1% | -0.1% | +4.1% |
| 26 | 5246ns | +0.0% | -0.0% | -0.1% | +1.3% |
| 27 | 5254ns | -0.1% | -0.2% | +1.0% | -0.1% |
| 28 | 5240ns | +0.1% | +0.1% | -0.0% | +0.1% |
| 29 | 5245ns | -0.0% | +1.2% | -0.1% | +2.5% |
| 30 | 5245ns | +1.4% | -0.1% | +0.0% | -0.0% |
| 31 | 5246ns | -0.1% | -0.1% | -0.1% | +0.2% |
| 32 | 5456ns | -3.8% | -3.8% | -3.0% | -3.9% |
| 33 | 5295ns | -1.0% | -1.0% | -1.0% | -0.7% |
| 34 | 5308ns | -1.3% | -0.4% | -1.0% | -1.2% |
| 35 | 5264ns | +0.4% | -0.4% | -0.4% | -0.4% |
| 36 | 5281ns | -0.7% | -0.7% | -0.8% | -0.7% |
| 37 | 5251ns | -0.1% | -0.1% | -0.2% | -0.1% |
| 38 | 5246ns | -0.1% | -0.0% | -0.1% | -0.1% |
| 39 | 5245ns | -0.0% | -0.1% | -0.1% | +0.0% |
| 40 | 5247ns | -0.0% | -0.2% | -0.1% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.050 | ok |
| wide-rung-ragged | 0.308 | moderate+ |
| wide-rung-ragged-overread | 0.098 | ok |
| wide-rung-wordround | 0.391 | moderate+ |
| wide-rung-wordround-alias | 0.282 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 18/40, lost 13/40
- **wide-rung-ragged-overread**: won 23/40, lost 7/40
- **wide-rung-wordround**: won 21/40, lost 11/40
- **wide-rung-wordround-alias**: won 14/40, lost 17/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.6ns | 5266.4ns | 0.0% |  |
| wide-rung-ragged | 2.3ns | 5294.9ns | 0.0% |  |
| wide-rung-ragged-overread | 2.2ns | 5252.5ns | 0.0% |  |
| wide-rung-wordround | 2.3ns | 5273.3ns | 0.0% |  |
| wide-rung-wordround-alias | 2.5ns | 5310.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 5242.7-5331.8 ns)
   5242.7 |########################################
   5247.2 |#####################
   5251.6 |##############
   5256.1 |#######
   5260.5 |##############
   5265.0 |###
   5269.4 |
   5273.9 |###
   5278.3 |#######
   5282.8 |
   5287.3 |
   5291.7 |###
   5296.2 |
   5300.6 |
   5305.1 |###
   5309.5 |
   5314.0 |
   5318.4 |
   5322.9 |
   5327.4 |
  (4 below, 3 above range)

wide-rung-ragged (n=40, range 5241.0-5478.4 ns)
   5241.0 |########################################
   5252.9 |#
   5264.7 |
   5276.6 |#
   5288.5 |#
   5300.3 |#
   5312.2 |###
   5324.1 |#
   5335.9 |
   5347.8 |###
   5359.7 |
   5371.5 |
   5383.4 |
   5395.3 |
   5407.2 |
   5419.0 |
   5430.9 |###
   5442.8 |
   5454.6 |
   5466.5 |
  (4 below, 1 above range)

wide-rung-ragged-overread (n=40, range 5239.9-5291.8 ns)
   5239.9 |########################################
   5242.5 |##############################
   5245.1 |###############
   5247.7 |
   5250.3 |
   5252.9 |
   5255.5 |###
   5258.0 |
   5260.6 |
   5263.2 |
   5265.8 |
   5268.4 |
   5271.0 |
   5273.6 |
   5276.2 |###
   5278.8 |
   5281.4 |
   5284.0 |
   5286.6 |###
   5289.2 |######
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 5238.9-5373.0 ns)
   5238.9 |########################################
   5245.6 |######
   5252.3 |##
   5259.0 |##
   5265.7 |
   5272.4 |
   5279.1 |
   5285.8 |
   5292.6 |##
   5299.3 |##
   5306.0 |############
   5312.7 |
   5319.4 |
   5326.1 |
   5332.8 |
   5339.5 |
   5346.2 |
   5352.9 |
   5359.6 |
   5366.3 |
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 5239.3-5516.3 ns)
   5239.3 |########################################
   5253.1 |########
   5267.0 |
   5280.8 |
   5294.7 |##
   5308.5 |######
   5322.4 |######
   5336.2 |
   5350.1 |####
   5363.9 |##
   5377.8 |
   5391.6 |
   5405.5 |##
   5419.3 |
   5433.2 |
   5447.0 |
   5460.9 |##
   5474.7 |
   5488.6 |
   5502.4 |
  (4 below, 2 above range)

```
