# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-align16, wide-rung-wordround-alias) are a dead heat (<1%)

wide-rung-align16 (1.25 ms) and wide-rung-wordround-alias (1.25 ms) differ by 0.03%, inside the noise, even though the wider field spreads 7.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (wide-rung-align16)

The baseline wide-rung-align16 is the fastest (1.25 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (wide-rung-align16) is the fastest** at 1253845.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.08x (fastest 1253845.2 ns, slowest 1349232.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1259683ns | 1254941ns | 1251027ns | 1255727ns | 1280210ns | base |
| wide-rung-ragged | 1376037ns | 1350439ns | 1345148ns | 1351702ns | 1479933ns | +9.24% |
| wide-rung-ragged-overread | 1267234ns | 1263202ns | 1258456ns | 1263298ns | 1287822ns | +0.60% |
| wide-rung-wordround | 1257149ns | 1256233ns | 1250421ns | 1255708ns | 1268198ns | -0.20% |
| wide-rung-wordround-alias | 1259511ns | 1255581ns | 1249690ns | 1256013ns | 1279830ns | -0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1258422ns | 1249768ns | 1278886ns | base | 1.458 |
| wide-rung-ragged | 1374713ns | 1343979ns | 1477967ns | +9.24% | 1.335 |
| wide-rung-ragged-overread | 1266073ns | 1257340ns | 1286692ns | +0.61% | 1.449 |
| wide-rung-wordround | 1255839ns | 1249147ns | 1266830ns | -0.21% | 1.461 |
| wide-rung-wordround-alias | 1258237ns | 1248470ns | 1278541ns | -0.01% | 1.458 |

## Performance model

- Peak throughput: **1.470 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.464 | 99.6% |
| wide-rung-ragged | 1.360 | 92.5% |
| wide-rung-ragged-overread | 1.454 | 98.9% |
| wide-rung-wordround | 1.462 | 99.5% |
| wide-rung-wordround-alias | 1.463 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1259683ns | 1259683ns | base |
| wide-rung-ragged | 1376037ns | 1376037ns | +9.24% |
| wide-rung-ragged-overread | 1267234ns | 1267234ns | +0.60% |
| wide-rung-wordround | 1257149ns | 1257149ns | -0.20% |
| wide-rung-wordround-alias | 1259511ns | 1259511ns | -0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1253845ns | base | --- | [1251984, 1255799] | --- | --- | --- | --- |
| wide-rung-ragged | 1349232ns | +94449.2ns (+7.5%) | [+91392, +97055]ns | [1348577, 1351779] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 1261802ns | +8019.4ns (+0.6%) | [+5712, +9836]ns | [1259875, 1263177] | YES | 0.0014 | 0.0007 | 0 |
| wide-rung-wordround | 1254780ns | no significant difference | [-2451, +3039]ns | [1252047, 1256319] | no | 1.0000 | 1.0000 | 0 |
| wide-rung-wordround-alias | 1254229ns | no significant difference | [-4165, +1145]ns | [1251938, 1257011] | no | 0.2051 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 1254328ns | +8.4% | +0.7% | +0.2% | -0.3% |
| 2 | 1281896ns | +5.2% | -1.3% | -1.4% | -2.7% |
| 3 | 1255840ns | +7.5% | +0.7% | -0.0% | -0.4% |
| 4 | 1257175ns | +7.3% | +0.4% | -0.4% | -0.2% |
| 5 | 1254832ns | +7.2% | +0.6% | +0.3% | -0.2% |
| 6 | 1253863ns | +7.6% | +0.7% | +1.9% | -0.2% |
| 7 | 1251977ns | +7.3% | +1.2% | +0.2% | +0.6% |
| 8 | 1254147ns | +7.7% | +0.4% | -0.2% | -0.3% |
| 9 | 1253232ns | +7.9% | +1.0% | -0.3% | -0.1% |
| 10 | 1253598ns | +7.9% | +2.3% | -0.2% | -0.1% |
| 11 | 1254895ns | +7.7% | +0.5% | -0.5% | +3.0% |
| 12 | 1265318ns | +6.6% | -0.6% | -1.5% | -0.1% |
| 13 | 1266807ns | +6.5% | -0.9% | -0.7% | -0.6% |
| 14 | 1267156ns | +6.9% | -0.8% | -1.5% | -1.0% |
| 15 | 1325565ns | +4.6% | -5.1% | -5.6% | -5.3% |
| 16 | 1273410ns | +5.9% | -1.3% | +0.9% | -1.1% |
| 17 | 1261050ns | +7.0% | -0.3% | -0.1% | -0.3% |
| 18 | 1257493ns | +7.4% | +0.4% | -0.5% | -0.3% |
| 19 | 1264792ns | +6.7% | -0.5% | -1.2% | -0.6% |
| 20 | 1284058ns | +5.1% | +0.5% | -2.3% | -1.7% |
| 21 | 1251659ns | +9.4% | +1.4% | +0.3% | +6.9% |
| 22 | 1251672ns | +8.8% | +0.8% | +0.1% | -0.4% |
| 23 | 1250575ns | +8.3% | +0.9% | +0.4% | +0.3% |
| 24 | 1249205ns | +9.6% | +0.6% | +0.6% | +0.2% |
| 25 | 1250604ns | +7.7% | +8.0% | +0.6% | +1.7% |
| 26 | 1248958ns | +7.8% | +3.0% | +0.7% | +1.1% |
| 27 | 1250890ns | +54.9% | +1.9% | +0.3% | +0.8% |
| 28 | 1255757ns | +31.5% | +0.3% | +0.1% | +0.2% |
| 29 | 1252378ns | +9.9% | +1.4% | +0.3% | +0.9% |
| 30 | 1251728ns | +9.5% | +1.5% | +0.4% | +0.4% |
| 31 | 1248160ns | +7.7% | +1.2% | +0.5% | +0.2% |
| 32 | 1266878ns | +6.1% | -0.5% | -1.0% | -1.1% |
| 33 | 1253828ns | +7.3% | +0.6% | +0.7% | -0.6% |
| 34 | 1250495ns | +7.6% | +0.7% | -0.0% | -0.1% |
| 35 | 1251992ns | +7.4% | +0.6% | -0.0% | -0.2% |
| 36 | 1250352ns | +7.5% | +1.3% | +0.1% | +1.9% |
| 37 | 1251685ns | +7.3% | +0.6% | +0.2% | +0.2% |
| 38 | 1255998ns | +7.0% | +0.3% | -0.4% | -0.1% |
| 39 | 1252857ns | +8.6% | +0.8% | -0.2% | -0.4% |
| 40 | 1249795ns | +8.1% | +0.8% | +1.4% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.322 | moderate+ |
| wide-rung-ragged | 0.382 | moderate+ |
| wide-rung-ragged-overread | 0.166 | ok |
| wide-rung-wordround | 0.075 | ok |
| wide-rung-wordround-alias | -0.004 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 0/40, lost 40/40
- **wide-rung-ragged-overread**: won 9/40, lost 31/40
- **wide-rung-wordround**: won 17/40, lost 18/40
- **wide-rung-wordround-alias**: won 22/40, lost 14/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 20.1ns | 1258422.4ns | 0.0% |  |
| wide-rung-ragged | 48.4ns | 1374712.5ns | 0.0% |  |
| wide-rung-ragged-overread | 17.1ns | 1266073.2ns | 0.0% |  |
| wide-rung-wordround | 15.8ns | 1255839.4ns | 0.0% |  |
| wide-rung-wordround-alias | 19.6ns | 1258237.1ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1249768.0-1278885.8 ns)
  1249768.0 |##################################
  1251223.9 |########################################
  1252679.8 |############################
  1254135.6 |######################
  1255591.5 |#################
  1257047.4 |###########
  1258503.3 |
  1259959.2 |#####
  1261415.1 |
  1262871.0 |
  1264326.9 |###########
  1265782.8 |#################
  1267238.7 |
  1268694.5 |
  1270150.4 |
  1271606.3 |
  1273062.2 |#####
  1274518.1 |
  1275974.0 |
  1277429.9 |
  (3 below, 3 above range)

wide-rung-ragged (n=40, range 1343979.0-1477966.9 ns)
  1343979.0 |########################################
  1350678.4 |##############
  1357377.8 |######
  1364077.2 |####
  1370776.6 |####
  1377476.0 |
  1384175.4 |##
  1390874.8 |
  1397574.2 |
  1404273.5 |
  1410972.9 |
  1417672.3 |
  1424371.7 |
  1431071.1 |
  1437770.5 |
  1444469.9 |
  1451169.3 |
  1457868.7 |
  1464568.1 |
  1471267.5 |
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 1257339.9-1286692.1 ns)
  1257339.9 |############################
  1258807.5 |########################################
  1260275.1 |######################
  1261742.7 |##################################
  1263210.3 |#####
  1264678.0 |#################
  1266145.6 |###########
  1267613.2 |
  1269080.8 |#################
  1270548.4 |
  1272016.0 |
  1273483.6 |#####
  1274951.2 |
  1276418.9 |
  1277886.5 |
  1279354.1 |
  1280821.7 |
  1282289.3 |#####
  1283756.9 |
  1285224.5 |#####
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 1249146.9-1266830.3 ns)
  1249146.9 |########
  1250031.1 |########################################
  1250915.3 |########################
  1251799.4 |################
  1252683.6 |########
  1253567.8 |################
  1254451.9 |########################################
  1255336.1 |########################
  1256220.3 |################
  1257104.4 |################################
  1257988.6 |################
  1258872.8 |########
  1259757.0 |
  1260641.1 |
  1261525.3 |
  1262409.5 |########
  1263293.6 |
  1264177.8 |########
  1265062.0 |
  1265946.1 |
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1248469.5-1278540.8 ns)
  1248469.5 |########################
  1249973.1 |########################################
  1251476.6 |########################################
  1252980.2 |################################
  1254483.8 |########################
  1255987.3 |########################
  1257490.9 |########################
  1258994.5 |########
  1260498.0 |################
  1262001.6 |########
  1263505.1 |################
  1265008.7 |
  1266512.3 |
  1268015.8 |
  1269519.4 |
  1271023.0 |########
  1272526.5 |########
  1274030.1 |
  1275533.6 |
  1277037.2 |
  (4 below, 2 above range)

```
