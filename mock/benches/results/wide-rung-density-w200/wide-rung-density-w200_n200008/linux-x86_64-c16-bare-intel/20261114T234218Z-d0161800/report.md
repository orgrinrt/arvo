# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-align16 dominates: 16% faster than the next best (wide-rung-ragged)

wide-rung-align16 (11.81 us) leads wide-rung-ragged (13.65 us) by 16%, a clear separation rather than a photo finish. CV 10.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (wide-rung-align16)

The baseline wide-rung-align16 is the fastest (11.81 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (wide-rung-align16) is the fastest** at 11813.5 ns median
- 4 variants significantly slower than baseline
- Spread: 1.25x (fastest 11813.5 ns, slowest 14810.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 12578ns | 11896ns | 11814ns | 12129ns | 14687ns | base |
| wide-rung-ragged | 13678ns | 13729ns | 12210ns | 13671ns | 15168ns | +8.75% |
| wide-rung-ragged-overread | 14244ns | 14909ns | 12188ns | 14689ns | 14967ns | +13.25% |
| wide-rung-wordround | 13962ns | 14595ns | 11945ns | 14386ns | 14708ns | +11.01% |
| wide-rung-wordround-alias | 14034ns | 14704ns | 11989ns | 14471ns | 14767ns | +11.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 12491ns | 11737ns | 14583ns | base | 1.476 |
| wide-rung-ragged | 13589ns | 12135ns | 15069ns | +8.79% | 1.356 |
| wide-rung-ragged-overread | 14153ns | 12111ns | 14870ns | +13.30% | 1.302 |
| wide-rung-wordround | 13873ns | 11866ns | 14624ns | +11.06% | 1.329 |
| wide-rung-wordround-alias | 13950ns | 11916ns | 14673ns | +11.68% | 1.321 |

## Performance model

- Peak throughput: **1.570 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 18432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.560 | 99.4% |
| wide-rung-ragged | 1.351 | 86.0% |
| wide-rung-ragged-overread | 1.245 | 79.3% |
| wide-rung-wordround | 1.272 | 81.0% |
| wide-rung-wordround-alias | 1.261 | 80.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 12578ns | 12578ns | base |
| wide-rung-ragged | 13678ns | 13678ns | +8.75% |
| wide-rung-ragged-overread | 14244ns | 14244ns | +13.25% |
| wide-rung-wordround | 13962ns | 13962ns | +11.01% |
| wide-rung-wordround-alias | 14034ns | 14034ns | +11.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 11814ns | base | --- | [11771, 11924] | --- | --- | --- | --- |
| wide-rung-ragged | 13645ns | +462.4ns (+3.9%) | [+411, +564]ns | [12217, 14906] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 14810ns | +1681.3ns (+14.2%) | [+400, +3012]ns | [14800, 14832] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 14496ns | +1298.2ns (+11.0%) | [+63, +2719]ns | [14417, 14566] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 14618ns | +1487.2ns (+12.6%) | [+201, +2731]ns | [14579, 14627] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 11756ns | +27.1% | +26.3% | +24.3% | +1.5% |
| 1 | 2 | 14438ns | +3.7% | +2.9% | +0.2% | +1.4% |
| 1 | 3 | 11805ns | +3.4% | +25.4% | +0.6% | +23.9% |
| 1 | 4 | 11844ns | +3.0% | +2.3% | +22.8% | +23.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.438 | moderate- |
| wide-rung-ragged | 0.248 | moderate+ |
| wide-rung-ragged-overread | -0.070 | ok |
| wide-rung-wordround | -0.390 | moderate- |
| wide-rung-wordround-alias | -0.088 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 0/4, lost 4/4
- **wide-rung-ragged-overread**: won 0/4, lost 4/4
- **wide-rung-wordround**: won 0/4, lost 4/4
- **wide-rung-wordround-alias**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 22.2ns | 12491.3ns | 0.2% |  |
| wide-rung-ragged | 22.8ns | 13589.5ns | 0.2% |  |
| wide-rung-ragged-overread | 25.0ns | 14152.5ns | 0.2% |  |
| wide-rung-wordround | 24.2ns | 13872.7ns | 0.2% |  |
| wide-rung-wordround-alias | 24.1ns | 13949.9ns | 0.2% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 11737.5-14583.0 ns)
  11737.5 |########################################
  11879.8 |#########
  12022.0 |
  12164.3 |
  12306.6 |
  12448.9 |
  12591.1 |
  12733.4 |
  12875.7 |
  13018.0 |
  13160.3 |
  13302.5 |
  13444.8 |
  13587.1 |
  13729.4 |
  13871.6 |
  14013.9 |
  14156.2 |
  14298.5 |##########
  14440.7 |#
  (3 below, 3 above range)

wide-rung-ragged (n=40, range 12134.9-15069.3 ns)
  12134.9 |########################################
  12281.6 |########
  12428.4 |
  12575.1 |
  12721.8 |
  12868.5 |
  13015.2 |
  13162.0 |
  13308.7 |
  13455.4 |
  13602.1 |
  13748.8 |
  13895.5 |
  14042.3 |
  14189.0 |
  14335.7 |
  14482.4 |
  14629.1 |
  14775.9 |######################
  14922.6 |#########################
  (3 below, 3 above range)

wide-rung-ragged-overread (n=40, range 12111.4-14869.7 ns)
  12111.4 |##########
  12249.4 |
  12387.3 |
  12525.2 |
  12663.1 |
  12801.0 |
  12938.9 |
  13076.8 |
  13214.8 |
  13352.7 |
  13490.6 |
  13628.5 |
  13766.4 |
  13904.3 |
  14042.2 |
  14180.2 |
  14318.1 |
  14456.0 |
  14593.9 |
  14731.8 |########################################
  (3 below, 4 above range)

wide-rung-wordround (n=40, range 11866.2-14624.4 ns)
  11866.2 |#############
  12004.1 |
  12142.0 |
  12279.9 |
  12417.8 |
  12555.7 |
  12693.6 |
  12831.5 |
  12969.5 |
  13107.4 |
  13245.3 |
  13383.2 |
  13521.1 |
  13659.0 |
  13796.9 |
  13934.8 |
  14072.8 |
  14210.7 |
  14348.6 |####################
  14486.5 |########################################
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 11916.0-14673.0 ns)
  11916.0 |########
  12053.9 |
  12191.7 |
  12329.6 |
  12467.4 |
  12605.3 |
  12743.1 |
  12881.0 |
  13018.8 |
  13156.7 |
  13294.5 |
  13432.4 |
  13570.2 |
  13708.1 |
  13845.9 |
  13983.8 |
  14121.6 |
  14259.5 |
  14397.3 |
  14535.2 |########################################
  (4 below, 2 above range)

```
