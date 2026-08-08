# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged shows warm-up / thermal drift (autocorr +0.66)

wide-rung-ragged's per-pass series has lag-1 autocorrelation +0.66, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (106 ns) is smaller than the fastest variant's own run-to-run std-dev (156 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.3% of the fastest

All 5 variants sit between 8.39 us and 8.50 us - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged's edge over baseline is significant but tiny (35 ns, 0.41%)

wide-rung-ragged differs from baseline wide-rung-align16 by 35 ns (0.41%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround** at 8393.5 ns median (-1.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 8393.5 ns, slowest 8499.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 8544ns | 8559ns | 8421ns | 8523ns | 8733ns | base |
| wide-rung-ragged | 8584ns | 8564ns | 8425ns | 8550ns | 8847ns | +0.47% |
| wide-rung-ragged-overread | 8531ns | 8469ns | 8426ns | 8500ns | 8725ns | -0.16% |
| wide-rung-wordround | 8541ns | 8458ns | 8415ns | 8485ns | 8835ns | -0.04% |
| wide-rung-wordround-alias | 8527ns | 8509ns | 8420ns | 8507ns | 8696ns | -0.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 8469ns | 8354ns | 8638ns | base | 1.209 |
| wide-rung-ragged | 8516ns | 8361ns | 8771ns | +0.56% | 1.202 |
| wide-rung-ragged-overread | 8462ns | 8362ns | 8655ns | -0.08% | 1.210 |
| wide-rung-wordround | 8470ns | 8347ns | 8758ns | +0.01% | 1.209 |
| wide-rung-wordround-alias | 8458ns | 8350ns | 8626ns | -0.13% | 1.211 |

## Performance model

- Peak throughput: **1.227 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 10240

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.207 | 98.4% |
| wide-rung-ragged | 1.205 | 98.2% |
| wide-rung-ragged-overread | 1.219 | 99.4% |
| wide-rung-wordround | 1.220 | 99.4% |
| wide-rung-wordround-alias | 1.215 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 8544ns | 8544ns | base |
| wide-rung-ragged | 8584ns | 8584ns | +0.47% |
| wide-rung-ragged-overread | 8531ns | 8531ns | -0.16% |
| wide-rung-wordround | 8541ns | 8541ns | -0.04% |
| wide-rung-wordround-alias | 8527ns | 8527ns | -0.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 8481ns | base | --- | [8386, 8504] | --- | --- | --- | --- |
| wide-rung-ragged | 8499ns | +19.0ns (+0.2%) | [+4, +69]ns | [8432, 8512] | YES | 0.0135 | 0.0034 | 1 |
| wide-rung-ragged-overread | 8401ns | no significant difference | [-9, +14]ns | [8385, 8498] | no | 0.5728 | 0.4296 | 0 |
| wide-rung-wordround | 8394ns | -7.1ns (-0.1%) | [-22, -1]ns | [8364, 8474] | YES (adj: no) | 0.1065 | 0.0533 | 1 |
| wide-rung-wordround-alias | 8427ns | no significant difference | [-19, +9]ns | [8391, 8490] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 8503ns | -0.0% | +0.0% | -0.0% | -0.2% |
| 2 | 8499ns | +0.0% | +2.0% | -0.2% | +0.3% |
| 3 | 8496ns | +0.1% | +0.1% | -0.3% | -0.0% |
| 4 | 8492ns | +0.0% | +0.2% | -0.2% | +0.0% |
| 5 | 8499ns | +0.0% | +0.1% | -0.3% | +0.1% |
| 6 | 8565ns | +0.8% | -0.4% | -0.7% | -0.8% |
| 7 | 8531ns | -0.3% | -0.0% | +0.8% | -0.5% |
| 8 | 8508ns | +0.1% | +0.0% | +3.4% | +0.0% |
| 9 | 8515ns | -0.1% | -0.1% | +3.2% | -0.2% |
| 10 | 8496ns | +0.4% | +0.5% | +3.3% | -0.1% |
| 11 | 8787ns | +1.2% | +0.1% | -5.0% | -5.0% |
| 12 | 8516ns | +3.3% | +3.4% | -1.3% | -2.0% |
| 13 | 8356ns | +3.6% | +2.4% | +0.9% | +0.1% |
| 14 | 8354ns | +4.5% | +0.1% | +0.9% | +0.0% |
| 15 | 8352ns | +2.8% | +0.5% | -0.1% | +1.2% |
| 16 | 8356ns | +1.8% | +1.5% | +0.7% | +1.5% |
| 17 | 8361ns | +1.9% | +0.3% | +2.0% | +1.5% |
| 18 | 8385ns | +1.2% | -0.2% | -0.4% | +1.5% |
| 19 | 8354ns | +2.3% | +1.3% | -0.1% | +1.9% |
| 20 | 8398ns | +2.2% | -0.4% | -0.0% | +2.8% |
| 21 | 8366ns | +1.0% | -0.1% | -0.3% | +0.6% |
| 22 | 8353ns | +0.1% | +0.1% | +0.4% | -0.1% |
| 23 | 8355ns | +0.0% | +0.0% | -0.1% | -0.0% |
| 24 | 8367ns | +0.6% | -0.1% | -0.2% | -0.2% |
| 25 | 8419ns | -0.7% | -0.7% | -0.8% | -0.8% |
| 26 | 8386ns | +0.4% | +0.2% | +0.0% | -0.4% |
| 27 | 8387ns | +0.2% | -0.2% | +0.1% | +0.1% |
| 28 | 8370ns | +0.8% | +0.2% | -0.0% | +0.3% |
| 29 | 8352ns | +0.2% | +0.3% | -0.1% | +0.1% |
| 30 | 8372ns | +0.6% | +0.4% | -0.3% | +0.5% |
| 31 | 8805ns | +0.2% | +0.2% | -0.1% | +0.0% |
| 32 | 8470ns | +4.1% | +0.3% | +3.9% | +3.9% |
| 33 | 8512ns | +3.5% | -0.9% | +3.2% | +2.3% |
| 34 | 8469ns | +0.1% | -0.9% | +3.0% | -0.5% |
| 35 | 8504ns | -1.6% | -1.3% | -1.7% | -1.4% |
| 36 | 8528ns | -2.0% | -1.7% | -2.1% | -1.7% |
| 37 | 8613ns | -2.9% | -2.7% | -2.9% | -2.5% |
| 38 | 8596ns | -2.4% | -2.2% | -2.5% | -2.0% |
| 39 | 8567ns | -2.5% | -2.2% | -2.4% | -2.2% |
| 40 | 8641ns | -2.9% | -2.8% | -3.1% | -3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.457 | moderate+ |
| wide-rung-ragged | 0.663 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.535 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.661 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.648 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 9/40, lost 22/40
- **wide-rung-ragged-overread**: won 15/40, lost 17/40
- **wide-rung-wordround**: won 18/40, lost 12/40
- **wide-rung-wordround-alias**: won 16/40, lost 15/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.3ns | 8468.9ns | 0.0% |  |
| wide-rung-ragged | 2.7ns | 8516.5ns | 0.0% |  |
| wide-rung-ragged-overread | 2.6ns | 8462.4ns | 0.0% |  |
| wide-rung-wordround | 2.4ns | 8469.6ns | 0.0% |  |
| wide-rung-wordround-alias | 2.5ns | 8457.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 8353.9-8638.2 ns)
   8353.9 |########################################
   8368.1 |#############
   8382.3 |####################
   8396.6 |######
   8410.8 |######
   8425.0 |
   8439.2 |
   8453.4 |
   8467.6 |#############
   8481.8 |#############
   8496.0 |########################################
   8510.2 |####################
   8524.5 |#############
   8538.7 |
   8552.9 |#############
   8567.1 |
   8581.3 |
   8595.5 |######
   8609.7 |######
   8624.0 |
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 8360.6-8771.1 ns)
   8360.6 |#################
   8381.2 |###########
   8401.7 |#################
   8422.2 |###########
   8442.7 |#####
   8463.3 |#####
   8483.8 |########################################
   8504.3 |######################
   8524.8 |#####
   8545.4 |#####
   8565.9 |#####
   8586.4 |#####
   8606.9 |
   8627.5 |#####
   8648.0 |#####
   8668.5 |
   8689.0 |
   8709.6 |
   8730.1 |#####
   8750.6 |
  (5 below, 5 above range)

wide-rung-ragged-overread (n=40, range 8362.3-8655.3 ns)
   8362.3 |########################################
   8376.9 |######################
   8391.6 |########################################
   8406.2 |
   8420.9 |
   8435.5 |#####
   8450.2 |#####
   8464.8 |#####
   8479.5 |#####
   8494.1 |######################
   8508.8 |###########
   8523.4 |###########
   8538.1 |###########
   8552.7 |
   8567.4 |
   8582.1 |
   8596.7 |
   8611.4 |
   8626.0 |
   8640.7 |
  (4 below, 4 above range)

wide-rung-wordround (n=40, range 8347.3-8758.3 ns)
   8347.3 |########################################
   8367.8 |##############
   8388.4 |##########
   8408.9 |###
   8429.5 |#######
   8450.0 |
   8470.6 |##############
   8491.1 |#######
   8511.7 |###
   8532.2 |
   8552.8 |
   8573.4 |
   8593.9 |###
   8614.5 |
   8635.0 |
   8655.6 |
   8676.1 |
   8696.7 |
   8717.2 |###
   8737.8 |
  (4 below, 6 above range)

wide-rung-wordround-alias (n=40, range 8349.7-8625.7 ns)
   8349.7 |#################
   8363.5 |###########
   8377.3 |#################
   8391.1 |#################
   8404.9 |#####
   8418.7 |#################
   8432.5 |
   8446.3 |#####
   8460.1 |
   8473.9 |###########
   8487.7 |########################################
   8501.5 |######################
   8515.3 |#####
   8529.1 |
   8542.9 |
   8556.7 |
   8570.5 |
   8584.3 |
   8598.1 |
   8611.9 |
  (6 below, 4 above range)

```

## Diagnostics

- **wide-rung-ragged**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.65 (measurement drift or warm-up artifact)
