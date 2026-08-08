# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.80)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (53 ns) is smaller than the fastest variant's own run-to-run std-dev (57 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.6% of the fastest

All 5 variants sit between 8.20 us and 8.26 us - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged's edge over baseline is significant but tiny (6 ns, 0.07%)

wide-rung-ragged differs from baseline wide-rung-align16 by 6 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged** at 8203.3 ns median (-0.1% vs baseline)
- Spread: 1.01x (fastest 8203.3 ns, slowest 8255.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 8476ns | 8276ns | 8251ns | 8381ns | 8987ns | base |
| wide-rung-ragged | 8296ns | 8265ns | 8260ns | 8273ns | 8403ns | -2.12% |
| wide-rung-ragged-overread | 8500ns | 8282ns | 8266ns | 8334ns | 9230ns | +0.28% |
| wide-rung-wordround | 8387ns | 8315ns | 8251ns | 8350ns | 8635ns | -1.05% |
| wide-rung-wordround-alias | 8356ns | 8290ns | 8252ns | 8306ns | 8612ns | -1.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 8410ns | 8188ns | 8908ns | base | 1.218 |
| wide-rung-ragged | 8232ns | 8198ns | 8339ns | -2.11% | 1.244 |
| wide-rung-ragged-overread | 8433ns | 8205ns | 9148ns | +0.27% | 1.214 |
| wide-rung-wordround | 8322ns | 8189ns | 8567ns | -1.04% | 1.230 |
| wide-rung-wordround-alias | 8292ns | 8189ns | 8545ns | -1.40% | 1.235 |

## Performance model

- Peak throughput: **1.251 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 10240

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.247 | 99.7% |
| wide-rung-ragged | 1.248 | 99.8% |
| wide-rung-ragged-overread | 1.246 | 99.6% |
| wide-rung-wordround | 1.240 | 99.2% |
| wide-rung-wordround-alias | 1.245 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 8476ns | 8476ns | base |
| wide-rung-ragged | 8296ns | 8296ns | -2.12% |
| wide-rung-ragged-overread | 8500ns | 8500ns | +0.28% |
| wide-rung-wordround | 8387ns | 8387ns | -1.05% |
| wide-rung-wordround-alias | 8356ns | 8356ns | -1.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 8213ns | base | --- | [8194, 8333] | --- | --- | --- | --- |
| wide-rung-ragged | 8203ns | no significant difference | [-96, +11]ns | [8201, 8218] | no | 1.0000 | 0.8746 | 0 |
| wide-rung-ragged-overread | 8219ns | no significant difference | [-62, +16]ns | [8208, 8350] | no | 1.0000 | 0.6358 | 0 |
| wide-rung-wordround | 8256ns | no significant difference | [-113, +4]ns | [8204, 8371] | no | 1.0000 | 0.4296 | 0 |
| wide-rung-wordround-alias | 8224ns | no significant difference | [-57, +17]ns | [8198, 8269] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 8185ns | +0.2% | +0.6% | +2.6% | +7.7% |
| 2 | 8214ns | +0.2% | -0.1% | -0.3% | +0.7% |
| 3 | 8188ns | +0.4% | +0.2% | +0.4% | +1.9% |
| 4 | 8196ns | +0.0% | +0.1% | -0.1% | +2.4% |
| 5 | 8192ns | +0.1% | +0.2% | +1.5% | +0.4% |
| 6 | 8194ns | +0.1% | +0.1% | +0.0% | +2.8% |
| 7 | 8185ns | +0.2% | +0.3% | +0.9% | +0.1% |
| 8 | 8317ns | +0.7% | -1.1% | -0.8% | -1.2% |
| 9 | 8187ns | +0.1% | +0.2% | +0.1% | +0.3% |
| 10 | 8212ns | -0.2% | -0.1% | +0.6% | +0.6% |
| 11 | 8796ns | -6.5% | -3.5% | -4.2% | -6.9% |
| 12 | 8798ns | -6.1% | -0.9% | -4.5% | -6.9% |
| 13 | 8947ns | -8.3% | -1.4% | -3.7% | -6.2% |
| 14 | 8899ns | -7.9% | -1.2% | -3.3% | -8.0% |
| 15 | 8802ns | -6.8% | -3.5% | -3.4% | -7.0% |
| 16 | 8805ns | -6.9% | -3.5% | -3.0% | -6.0% |
| 17 | 8897ns | -7.8% | -4.4% | -1.9% | -7.7% |
| 18 | 8987ns | -7.0% | -1.1% | -5.0% | -8.5% |
| 19 | 8985ns | -6.9% | -1.7% | -4.7% | -8.5% |
| 20 | 8872ns | -7.3% | +36.7% | -5.2% | -7.6% |
| 21 | 8193ns | +2.0% | +0.2% | +1.9% | +7.7% |
| 22 | 8192ns | +2.0% | +0.2% | +2.5% | +0.9% |
| 23 | 8192ns | +2.0% | +0.2% | +3.1% | -0.0% |
| 24 | 8255ns | -0.4% | -0.6% | +1.5% | -0.7% |
| 25 | 8188ns | +0.1% | +0.3% | +1.7% | +0.1% |
| 26 | 8194ns | +0.1% | +0.1% | +2.1% | +1.2% |
| 27 | 8192ns | +0.1% | +0.2% | +0.1% | +1.2% |
| 28 | 8238ns | -0.5% | -0.1% | -0.3% | -0.5% |
| 29 | 8245ns | -0.6% | +1.7% | -0.7% | -0.7% |
| 30 | 8194ns | +0.1% | +0.2% | +0.1% | +0.0% |
| 31 | 8511ns | -3.6% | -1.7% | -3.4% | -0.5% |
| 32 | 8520ns | -3.7% | -1.8% | -3.9% | -0.5% |
| 33 | 8868ns | -7.5% | -5.6% | -7.6% | -4.4% |
| 34 | 8196ns | +1.0% | +0.5% | -0.1% | +3.4% |
| 35 | 8350ns | -1.7% | -1.7% | -1.9% | -1.3% |
| 36 | 8197ns | +0.0% | +0.1% | -0.1% | +0.9% |
| 37 | 8189ns | +0.2% | +1.8% | +0.7% | +0.1% |
| 38 | 8238ns | -0.3% | +0.2% | +0.1% | -0.2% |
| 39 | 8193ns | +0.3% | +0.2% | -0.1% | +0.1% |
| 40 | 8191ns | +0.3% | +0.2% | -0.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.758 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.371 | moderate+ |
| wide-rung-ragged-overread | 0.143 | ok |
| wide-rung-wordround | 0.801 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.090 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 19/40, lost 17/40
- **wide-rung-ragged-overread**: won 17/40, lost 22/40
- **wide-rung-wordround**: won 18/40, lost 12/40
- **wide-rung-wordround-alias**: won 19/40, lost 14/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.3ns | 8409.9ns | 0.0% |  |
| wide-rung-ragged | 2.5ns | 8232.3ns | 0.0% |  |
| wide-rung-ragged-overread | 2.5ns | 8432.7ns | 0.0% |  |
| wide-rung-wordround | 2.2ns | 8322.1ns | 0.0% |  |
| wide-rung-wordround-alias | 2.4ns | 8291.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 8188.0-8907.6 ns)
   8188.0 |########################################
   8224.0 |##########
   8260.0 |
   8296.0 |##
   8331.9 |##
   8367.9 |
   8403.9 |
   8439.9 |
   8475.8 |##
   8511.8 |##
   8547.8 |
   8583.8 |
   8619.7 |
   8655.7 |
   8691.7 |
   8727.7 |
   8763.7 |#####
   8799.6 |#####
   8835.6 |##
   8871.6 |#######
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 8198.0-8339.0 ns)
   8198.0 |########################################
   8205.1 |##
   8212.1 |##
   8219.2 |##########
   8226.2 |##
   8233.3 |
   8240.3 |
   8247.4 |
   8254.4 |##
   8261.5 |
   8268.5 |
   8275.6 |##
   8282.6 |
   8289.7 |
   8296.7 |
   8303.8 |
   8310.8 |
   8317.9 |
   8324.9 |
   8332.0 |
  (4 below, 6 above range)

wide-rung-ragged-overread (n=40, range 8204.8-9148.4 ns)
   8204.8 |########################################
   8251.9 |##
   8299.1 |##
   8346.3 |########
   8393.5 |
   8440.7 |
   8487.9 |########
   8535.0 |
   8582.2 |
   8629.4 |
   8676.6 |##
   8723.8 |
   8771.0 |##
   8818.1 |####
   8865.3 |##
   8912.5 |
   8959.7 |
   9006.9 |
   9054.1 |
   9101.2 |
  (5 below, 1 above range)

wide-rung-wordround (n=40, range 8189.4-8566.9 ns)
   8189.4 |########################################
   8208.3 |##########
   8227.2 |#######
   8246.0 |##########
   8264.9 |
   8283.8 |
   8302.7 |###
   8321.5 |###
   8340.4 |###
   8359.3 |#######
   8378.1 |###
   8397.0 |##########
   8415.9 |###
   8434.8 |###
   8453.6 |
   8472.5 |
   8491.4 |###
   8510.3 |
   8529.1 |#######
   8548.0 |###
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 8189.2-8545.4 ns)
   8189.2 |########################################
   8207.0 |####################
   8224.8 |######
   8242.6 |###
   8260.4 |#############
   8278.2 |######
   8296.0 |
   8313.9 |
   8331.7 |###
   8349.5 |
   8367.3 |
   8385.1 |######
   8402.9 |
   8420.7 |###
   8438.5 |
   8456.4 |###
   8474.2 |##########
   8492.0 |
   8509.8 |
   8527.6 |
  (3 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.80 (measurement drift or warm-up artifact)
