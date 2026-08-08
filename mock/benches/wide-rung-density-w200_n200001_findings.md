# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-wordround-alias, wide-rung-wordround) are a dead heat (<1%)

wide-rung-wordround-alias (4.65 us) and wide-rung-wordround (4.68 us) differ by 0.71%, inside the noise, even though the wider field spreads 6.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.68)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.68, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-wordround's edge over baseline is significant but tiny (-39 ns, 0.81%)

wide-rung-wordround differs from baseline wide-rung-align16 by -39 ns (0.81%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 4650.0 ns median (-3.7% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.07x (fastest 4650.0 ns, slowest 4960.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4894ns | 4896ns | 4735ns | 4894ns | 5051ns | base |
| wide-rung-ragged | 5057ns | 5028ns | 5003ns | 5041ns | 5160ns | +3.35% |
| wide-rung-ragged-overread | 5033ns | 5015ns | 5011ns | 5025ns | 5079ns | +2.85% |
| wide-rung-wordround | 4756ns | 4751ns | 4701ns | 4751ns | 4829ns | -2.81% |
| wide-rung-wordround-alias | 4724ns | 4711ns | 4683ns | 4712ns | 4802ns | -3.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4826ns | 4666ns | 4984ns | base | 0.849 |
| wide-rung-ragged | 4993ns | 4939ns | 5095ns | +3.47% | 0.820 |
| wide-rung-ragged-overread | 4970ns | 4947ns | 5016ns | +2.98% | 0.824 |
| wide-rung-wordround | 4690ns | 4635ns | 4760ns | -2.82% | 0.873 |
| wide-rung-wordround-alias | 4660ns | 4621ns | 4734ns | -3.43% | 0.879 |

## Performance model

- Peak throughput: **0.886 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.849 | 95.7% |
| wide-rung-ragged | 0.826 | 93.2% |
| wide-rung-ragged-overread | 0.827 | 93.3% |
| wide-rung-wordround | 0.875 | 98.7% |
| wide-rung-wordround-alias | 0.881 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4894ns | 4894ns | base |
| wide-rung-ragged | 5057ns | 5057ns | +3.35% |
| wide-rung-ragged-overread | 5033ns | 5033ns | +2.85% |
| wide-rung-wordround | 4756ns | 4756ns | -2.81% |
| wide-rung-wordround-alias | 4724ns | 4724ns | -3.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4827ns | base | --- | [4774, 4868] | --- | --- | --- | --- |
| wide-rung-ragged | 4961ns | +157.2ns (+3.3%) | [+100, +244]ns | [4945, 5019] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 4951ns | +122.5ns (+2.5%) | [+82, +177]ns | [4950, 4961] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 4683ns | -150.8ns (-3.1%) | [-179, -122]ns | [4668, 4700] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 4650ns | -189.5ns (-3.9%) | [-220, -127]ns | [4640, 4657] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 4953ns | -0.3% | +1.4% | -5.3% | -4.7% |
| 2 | 4835ns | +2.4% | +2.4% | -3.0% | -4.0% |
| 3 | 4857ns | +1.7% | +1.9% | -3.8% | -4.2% |
| 4 | 5019ns | -1.5% | -1.5% | -6.3% | -7.3% |
| 5 | 5044ns | -2.0% | -1.9% | -6.8% | -8.0% |
| 6 | 4852ns | +1.9% | +2.1% | -3.5% | -4.6% |
| 7 | 4830ns | +2.3% | +2.4% | -3.2% | -4.6% |
| 8 | 4718ns | +4.8% | +4.9% | -0.4% | -2.0% |
| 9 | 5003ns | -1.2% | +0.5% | -5.0% | -5.2% |
| 10 | 4811ns | +2.8% | +2.9% | -2.8% | -3.2% |
| 11 | 4879ns | +2.9% | +1.4% | -5.0% | -4.4% |
| 12 | 4824ns | +5.7% | +2.7% | -3.8% | -3.9% |
| 13 | 4975ns | +2.9% | -0.5% | -6.7% | -6.6% |
| 14 | 4832ns | +5.6% | +2.5% | -3.7% | -4.1% |
| 15 | 4821ns | +5.1% | +2.7% | -3.7% | -4.0% |
| 16 | 4824ns | +7.2% | +2.6% | -2.7% | -3.9% |
| 17 | 4954ns | -0.1% | -0.1% | -5.0% | -6.6% |
| 18 | 4914ns | +1.0% | +0.7% | -3.7% | -6.0% |
| 19 | 4971ns | +1.5% | -0.5% | -6.2% | -7.1% |
| 20 | 4777ns | +5.1% | +3.7% | -2.9% | -2.9% |
| 21 | 4767ns | +3.6% | +4.9% | -2.4% | +1.8% |
| 22 | 4655ns | +6.2% | +7.4% | -0.5% | +0.4% |
| 23 | 4730ns | +4.5% | +5.7% | -1.5% | -1.6% |
| 24 | 4951ns | -0.2% | +1.0% | -5.3% | -5.6% |
| 25 | 4939ns | -0.0% | +1.2% | -5.7% | -5.8% |
| 26 | 4822ns | +2.5% | +3.7% | -3.3% | -3.4% |
| 27 | 4619ns | +7.8% | +8.3% | +0.1% | +0.1% |
| 28 | 4682ns | +7.2% | +6.9% | -1.4% | -0.8% |
| 29 | 4720ns | +6.7% | +6.0% | -0.8% | -1.1% |
| 30 | 4649ns | +8.6% | +8.4% | +2.3% | -0.0% |
| 31 | 4917ns | +0.8% | +0.7% | -2.7% | -5.3% |
| 32 | 4906ns | +0.7% | +0.9% | -3.1% | -3.6% |
| 33 | 4911ns | +1.3% | +1.1% | -2.8% | -5.7% |
| 34 | 4659ns | +6.7% | +6.2% | +2.0% | -0.3% |
| 35 | 4661ns | +9.4% | +6.1% | +1.7% | +1.4% |
| 36 | 4740ns | +6.0% | +4.4% | +0.5% | -1.7% |
| 37 | 4686ns | +7.8% | +5.6% | -0.1% | -1.3% |
| 38 | 4842ns | +3.6% | +2.2% | -3.4% | -2.8% |
| 39 | 4771ns | +5.2% | +3.7% | -1.5% | -2.4% |
| 40 | 4713ns | +5.1% | +6.5% | +0.3% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.360 | moderate+ |
| wide-rung-ragged | 0.558 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.394 | moderate+ |
| wide-rung-wordround | 0.677 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.023 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 6/40, lost 33/40
- **wide-rung-ragged-overread**: won 5/40, lost 35/40
- **wide-rung-wordround**: won 33/40, lost 5/40
- **wide-rung-wordround-alias**: won 34/40, lost 5/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.3ns | 4825.8ns | 0.0% |  |
| wide-rung-ragged | 2.3ns | 4993.1ns | 0.0% |  |
| wide-rung-ragged-overread | 2.1ns | 4969.5ns | 0.0% |  |
| wide-rung-wordround | 2.4ns | 4689.7ns | 0.1% |  |
| wide-rung-wordround-alias | 2.4ns | 4660.2ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 4665.5-4983.6 ns)
   4665.5 |
   4681.4 |################
   4697.3 |
   4713.2 |########################
   4729.1 |################
   4745.0 |
   4760.9 |########################
   4776.9 |
   4792.8 |
   4808.7 |########################################
   4824.6 |########################
   4840.5 |################
   4856.4 |########
   4872.3 |########
   4888.2 |
   4904.1 |################################
   4920.0 |
   4935.9 |################
   4951.8 |################
   4967.7 |################
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 4939.1-5094.7 ns)
   4939.1 |########################################
   4946.9 |######
   4954.6 |###
   4962.4 |###
   4970.2 |#########
   4978.0 |
   4985.8 |
   4993.6 |
   5001.3 |
   5009.1 |
   5016.9 |###############
   5024.7 |###
   5032.5 |###
   5040.3 |######
   5048.0 |###
   5055.8 |
   5063.6 |###
   5071.4 |
   5079.2 |
   5087.0 |
  (4 below, 5 above range)

wide-rung-ragged-overread (n=40, range 4946.5-5015.9 ns)
   4946.5 |#################################
   4950.0 |########################################
   4953.4 |###
   4956.9 |
   4960.4 |
   4963.9 |###
   4967.3 |
   4970.8 |
   4974.3 |
   4977.7 |
   4981.2 |
   4984.7 |
   4988.1 |
   4991.6 |
   4995.1 |###
   4998.6 |#############
   5002.0 |#############
   5005.5 |
   5009.0 |
   5012.4 |
  (3 below, 4 above range)

wide-rung-wordround (n=40, range 4634.6-4760.4 ns)
   4634.6 |####################
   4640.9 |##############################
   4647.2 |####################
   4653.5 |####################
   4659.8 |####################
   4666.1 |##########
   4672.4 |##############################
   4678.7 |##############################
   4684.9 |##############################
   4691.2 |##########
   4697.5 |########################################
   4703.8 |##########
   4710.1 |
   4716.4 |
   4722.7 |##########
   4729.0 |##########
   4735.3 |
   4741.6 |##########
   4747.8 |##############################
   4754.1 |##########
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 4621.2-4734.1 ns)
   4621.2 |########################
   4626.9 |################################
   4632.5 |########################
   4638.2 |########################
   4643.8 |################################
   4649.5 |########################################
   4655.1 |################################
   4660.8 |################
   4666.4 |
   4672.0 |################
   4677.7 |
   4683.3 |
   4689.0 |
   4694.6 |
   4700.3 |########
   4705.9 |
   4711.6 |
   4717.2 |################
   4722.8 |########
   4728.5 |########
  (3 below, 2 above range)

```

## Diagnostics

- **wide-rung-ragged**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.68 (measurement drift or warm-up artifact)
