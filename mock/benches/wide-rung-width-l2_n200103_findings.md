# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 2.1% of the fastest

All 5 variants sit between 1.32 ms and 1.35 ms - a 2.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 1323812.1 ns median (-0.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 1323812.1 ns, slowest 1352073.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1342181ns | 1327182ns | 1319501ns | 1329660ns | 1402426ns | base |
| wide-rung-ragged | 1356441ns | 1353359ns | 1347951ns | 1353287ns | 1374392ns | +1.06% |
| wide-rung-ragged-overread | 1327149ns | 1324885ns | 1319914ns | 1325388ns | 1339667ns | -1.12% |
| wide-rung-wordround | 1337017ns | 1329244ns | 1323302ns | 1329528ns | 1373198ns | -0.38% |
| wide-rung-wordround-alias | 1328244ns | 1326747ns | 1320305ns | 1327319ns | 1338960ns | -1.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1340878ns | 1318245ns | 1400863ns | base | 1.369 |
| wide-rung-ragged | 1355296ns | 1346985ns | 1373065ns | +1.08% | 1.354 |
| wide-rung-ragged-overread | 1325997ns | 1318770ns | 1338437ns | -1.11% | 1.384 |
| wide-rung-wordround | 1335669ns | 1322034ns | 1371650ns | -0.39% | 1.374 |
| wide-rung-wordround-alias | 1326896ns | 1319004ns | 1337482ns | -1.04% | 1.383 |

## Performance model

- Peak throughput: **1.392 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.384 | 99.4% |
| wide-rung-ragged | 1.357 | 97.5% |
| wide-rung-ragged-overread | 1.386 | 99.6% |
| wide-rung-wordround | 1.382 | 99.3% |
| wide-rung-wordround-alias | 1.384 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1342181ns | 1342181ns | base |
| wide-rung-ragged | 1356441ns | 1356441ns | +1.06% |
| wide-rung-ragged-overread | 1327149ns | 1327149ns | -1.12% |
| wide-rung-wordround | 1337017ns | 1337017ns | -0.38% |
| wide-rung-wordround-alias | 1328244ns | 1328244ns | -1.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1326119ns | base | --- | [1322178, 1331810] | --- | --- | --- | --- |
| wide-rung-ragged | 1352074ns | +26467.5ns (+2.0%) | [+19378, +30229]ns | [1349745, 1354245] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 1323812ns | no significant difference | [-5449, +920]ns | [1321690, 1325850] | no | 0.4296 | 0.4296 | 0 |
| wide-rung-wordround | 1328056ns | no significant difference | [-4379, +5496]ns | [1325906, 1329618] | no | 0.4296 | 0.4296 | 0 |
| wide-rung-wordround-alias | 1325396ns | no significant difference | [-5225, +277]ns | [1324108, 1328486] | no | 0.1614 | 0.0807 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 1329198ns | +1.6% | -0.5% | -0.1% | -0.6% |
| 2 | 1329146ns | +1.6% | +1.0% | +1.5% | +0.1% |
| 3 | 1654572ns | -18.0% | -20.3% | -20.0% | -19.5% |
| 4 | 1378826ns | -0.5% | -4.0% | -3.8% | -2.8% |
| 5 | 1374717ns | +0.8% | -3.2% | -2.9% | -3.0% |
| 6 | 1371337ns | -1.4% | -3.5% | -3.2% | -3.0% |
| 7 | 1364017ns | -1.1% | -3.0% | -1.8% | -2.5% |
| 8 | 1361107ns | -1.1% | -1.5% | -2.7% | -0.1% |
| 9 | 1349501ns | -0.2% | -2.1% | -1.6% | -1.5% |
| 10 | 1327015ns | +1.5% | -0.2% | -0.2% | -0.1% |
| 11 | 1318805ns | +6.6% | +0.5% | +0.0% | +0.4% |
| 12 | 1322458ns | +2.4% | -0.2% | +1.8% | -0.3% |
| 13 | 1322927ns | +2.4% | +0.3% | +2.0% | -0.1% |
| 14 | 1322033ns | +3.0% | -0.2% | +1.0% | -0.1% |
| 15 | 1321054ns | +2.3% | +0.0% | +0.2% | +0.5% |
| 16 | 1332972ns | +2.0% | -0.2% | -0.0% | -0.4% |
| 17 | 1320519ns | +2.7% | +1.9% | +0.4% | +0.3% |
| 18 | 1321014ns | +2.2% | +0.2% | +0.1% | +0.0% |
| 19 | 1322323ns | +2.3% | +0.2% | +0.0% | +0.2% |
| 20 | 1343166ns | +1.9% | -1.1% | -1.9% | -1.9% |
| 21 | 1320266ns | +2.2% | +0.4% | +0.3% | -0.2% |
| 22 | 1316116ns | +2.5% | +1.3% | +0.9% | +0.2% |
| 23 | 1319468ns | +2.7% | +0.9% | +0.3% | -0.0% |
| 24 | 1317804ns | +2.4% | +0.9% | +0.6% | +0.8% |
| 25 | 1319502ns | +2.5% | +0.1% | +0.6% | +0.7% |
| 26 | 1317722ns | +4.3% | -0.0% | +0.8% | +0.5% |
| 27 | 1318435ns | +2.5% | +0.1% | +0.3% | +0.8% |
| 28 | 1318107ns | +2.6% | +0.1% | +0.8% | +0.5% |
| 29 | 1352830ns | +0.1% | -2.5% | -1.8% | -1.9% |
| 30 | 1320874ns | +2.7% | -0.2% | +0.2% | +0.2% |
| 31 | 1339349ns | +0.6% | -1.3% | -0.5% | -0.0% |
| 32 | 1326888ns | +1.5% | +0.7% | +17.3% | +0.5% |
| 33 | 1338962ns | +0.7% | -1.3% | -0.7% | -0.6% |
| 34 | 1342342ns | +0.7% | -1.6% | -0.9% | -1.3% |
| 35 | 1330649ns | +1.2% | +1.2% | -0.5% | -0.1% |
| 36 | 1345847ns | +0.1% | -1.6% | -1.3% | -1.3% |
| 37 | 1327891ns | +1.5% | -0.1% | +0.7% | -0.3% |
| 38 | 1325211ns | +2.0% | -0.3% | +2.6% | -0.6% |
| 39 | 1325350ns | +2.1% | +0.1% | +0.8% | -0.1% |
| 40 | 1324801ns | +2.3% | -0.1% | +0.4% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.163 | ok |
| wide-rung-ragged | 0.061 | ok |
| wide-rung-ragged-overread | -0.065 | ok |
| wide-rung-wordround | 0.001 | ok |
| wide-rung-wordround-alias | 0.342 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 6/40, lost 34/40
- **wide-rung-ragged-overread**: won 22/40, lost 13/40
- **wide-rung-wordround**: won 15/40, lost 20/40
- **wide-rung-wordround-alias**: won 20/40, lost 13/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 30.7ns | 1340878.0ns | 0.0% |  |
| wide-rung-ragged | 24.1ns | 1355296.3ns | 0.0% |  |
| wide-rung-ragged-overread | 25.4ns | 1325996.5ns | 0.0% |  |
| wide-rung-wordround | 36.2ns | 1335669.2ns | 0.0% |  |
| wide-rung-wordround-alias | 26.0ns | 1326896.4ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1318244.8-1400863.4 ns)
  1318244.8 |########################################
  1322375.7 |##################
  1326506.6 |##################
  1330637.5 |#######
  1334768.5 |
  1338899.4 |##########
  1343030.3 |#######
  1347161.3 |###
  1351292.2 |###
  1355423.1 |
  1359554.1 |###
  1363685.0 |###
  1367815.9 |###
  1371946.9 |###
  1376077.8 |###
  1380208.7 |
  1384339.7 |
  1388470.6 |
  1392601.5 |
  1396732.5 |
  (4 below, 1 above range)

wide-rung-ragged (n=40, range 1346985.5-1373065.0 ns)
  1346985.5 |################################
  1348289.4 |################################
  1349593.4 |########################
  1350897.4 |########################################
  1352201.4 |################################
  1353505.3 |########################
  1354809.3 |################
  1356113.3 |########################
  1357417.3 |
  1358721.3 |########
  1360025.2 |
  1361329.2 |########
  1362633.2 |
  1363937.2 |
  1365241.1 |
  1366545.1 |
  1367849.1 |########
  1369153.1 |
  1370457.0 |
  1371761.0 |########
  (5 below, 3 above range)

wide-rung-ragged-overread (n=40, range 1318769.9-1338437.4 ns)
  1318769.9 |################################
  1319753.3 |################
  1320736.6 |########################################
  1321720.0 |################
  1322703.4 |########################
  1323686.8 |########################
  1324670.1 |################################
  1325653.5 |################
  1326636.9 |########
  1327620.3 |########
  1328603.7 |
  1329587.0 |########
  1330570.4 |########################
  1331553.8 |
  1332537.2 |########
  1333520.5 |
  1334503.9 |
  1335487.3 |
  1336470.7 |########
  1337454.1 |
  (3 below, 4 above range)

wide-rung-wordround (n=40, range 1322034.3-1371649.9 ns)
  1322034.3 |########################################
  1324515.1 |################
  1326995.9 |################################
  1329476.6 |############
  1331957.4 |########
  1334438.2 |############
  1336919.0 |########
  1339399.8 |
  1341880.5 |
  1344361.3 |####
  1346842.1 |
  1349322.9 |########
  1351803.6 |
  1354284.4 |
  1356765.2 |####
  1359246.0 |
  1361726.8 |
  1364207.5 |
  1366688.3 |
  1369169.1 |
  (3 below, 1 above range)

wide-rung-wordround-alias (n=40, range 1319004.5-1337481.6 ns)
  1319004.5 |################
  1319928.3 |########
  1320852.2 |################################
  1321776.1 |
  1322699.9 |################
  1323623.8 |########################################
  1324547.6 |################
  1325471.5 |########
  1326395.3 |########
  1327319.2 |########################
  1328243.1 |########################
  1329166.9 |################
  1330090.8 |########################
  1331014.6 |########
  1331938.5 |########
  1332862.4 |################
  1333786.2 |
  1334710.1 |
  1335633.9 |
  1336557.8 |
  (4 below, 3 above range)

```
