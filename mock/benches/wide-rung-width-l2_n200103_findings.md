# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 2.4% of the fastest

All 5 variants sit between 1.32 ms and 1.35 ms - a 2.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 1317445.6 ns median (-0.4% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 1317445.6 ns, slowest 1349340.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1324989ns | 1323188ns | 1320274ns | 1323788ns | 1333310ns | base |
| wide-rung-ragged | 1353056ns | 1350560ns | 1348381ns | 1350587ns | 1365136ns | +2.12% |
| wide-rung-ragged-overread | 1320422ns | 1318838ns | 1315816ns | 1319386ns | 1328138ns | -0.34% |
| wide-rung-wordround | 1322405ns | 1321870ns | 1318150ns | 1321749ns | 1328629ns | -0.20% |
| wide-rung-wordround-alias | 1327147ns | 1324801ns | 1320297ns | 1324910ns | 1340707ns | +0.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1323770ns | 1319061ns | 1332134ns | base | 1.386 |
| wide-rung-ragged | 1351880ns | 1347172ns | 1363973ns | +2.12% | 1.357 |
| wide-rung-ragged-overread | 1319179ns | 1314608ns | 1326969ns | -0.35% | 1.391 |
| wide-rung-wordround | 1321100ns | 1316848ns | 1327324ns | -0.20% | 1.389 |
| wide-rung-wordround-alias | 1325955ns | 1319081ns | 1339590ns | +0.17% | 1.384 |

## Performance model

- Peak throughput: **1.396 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.388 | 99.4% |
| wide-rung-ragged | 1.360 | 97.4% |
| wide-rung-ragged-overread | 1.393 | 99.8% |
| wide-rung-wordround | 1.390 | 99.5% |
| wide-rung-wordround-alias | 1.386 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1324989ns | 1324989ns | base |
| wide-rung-ragged | 1353056ns | 1353056ns | +2.12% |
| wide-rung-ragged-overread | 1320422ns | 1320422ns | -0.34% |
| wide-rung-wordround | 1322405ns | 1322405ns | -0.20% |
| wide-rung-wordround-alias | 1327147ns | 1327147ns | +0.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1322162ns | base | --- | [1320921, 1324344] | --- | --- | --- | --- |
| wide-rung-ragged | 1349341ns | +27307.5ns (+2.1%) | [+25896, +28163]ns | [1348214, 1350475] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 1317446ns | -4419.2ns (-0.3%) | [-5444, -4047]ns | [1316808, 1319244] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-wordround | 1320593ns | -1401.0ns (-0.1%) | [-3416, -630]ns | [1319043, 1321817] | YES | 0.0221 | 0.0166 | 0 |
| wide-rung-wordround-alias | 1323701ns | no significant difference | [-1638, +4316]ns | [1321224, 1325783] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 1319676ns | +2.3% | -0.1% | +1.2% | +0.4% |
| 2 | 1322118ns | +2.0% | -0.4% | -0.2% | +0.6% |
| 3 | 1320814ns | +1.9% | -0.3% | -0.1% | +0.5% |
| 4 | 1321029ns | +2.0% | +0.7% | -0.1% | +0.8% |
| 5 | 1326996ns | +1.5% | -0.0% | -0.4% | +0.3% |
| 6 | 1321896ns | +4.2% | -0.3% | +0.0% | +0.4% |
| 7 | 1322206ns | +2.7% | -0.5% | +0.2% | +0.3% |
| 8 | 1320535ns | +2.8% | +0.8% | +0.3% | +0.7% |
| 9 | 1320200ns | +2.3% | +0.7% | -0.1% | +0.6% |
| 10 | 1320754ns | +2.0% | -0.3% | +0.0% | +1.7% |
| 11 | 1319209ns | +2.2% | -0.4% | -0.2% | +0.2% |
| 12 | 1346651ns | +0.1% | -1.9% | -2.2% | -2.2% |
| 13 | 1319505ns | +2.1% | +0.1% | -0.3% | -0.1% |
| 14 | 1319962ns | +2.2% | -0.4% | +0.1% | +0.0% |
| 15 | 1319526ns | +2.1% | -0.4% | +0.0% | +1.5% |
| 16 | 1319222ns | +2.1% | -0.3% | -0.0% | +0.4% |
| 17 | 1320606ns | +2.1% | -0.3% | -0.2% | +0.9% |
| 18 | 1318868ns | +4.0% | -0.2% | -0.2% | +2.1% |
| 19 | 1316518ns | +2.4% | +0.1% | +0.1% | +0.4% |
| 20 | 1320027ns | +2.1% | +0.5% | -0.1% | +0.3% |
| 21 | 1324115ns | +1.9% | -0.2% | -0.5% | -0.0% |
| 22 | 1327341ns | +1.6% | -0.4% | -0.6% | -0.4% |
| 23 | 1325529ns | +3.3% | -0.5% | +1.0% | -0.4% |
| 24 | 1326720ns | +1.8% | -0.6% | -0.4% | -0.4% |
| 25 | 1324426ns | +2.1% | -0.3% | -0.4% | -0.3% |
| 26 | 1331421ns | +1.4% | -1.1% | -1.0% | -0.7% |
| 27 | 1324610ns | +2.0% | -0.4% | -0.5% | -0.4% |
| 28 | 1326662ns | +2.0% | -0.7% | -0.7% | -0.1% |
| 29 | 1327108ns | +1.8% | -0.3% | -0.3% | -0.1% |
| 30 | 1324427ns | +2.1% | -0.3% | -0.6% | +2.8% |
| 31 | 1321196ns | +3.9% | -0.4% | +0.3% | -0.0% |
| 32 | 1322671ns | +2.2% | -0.6% | +0.0% | +0.1% |
| 33 | 1321625ns | +2.1% | -0.3% | -0.0% | -0.3% |
| 34 | 1340321ns | +0.7% | -1.7% | -1.4% | -1.5% |
| 35 | 1330516ns | +1.4% | -1.1% | -0.6% | -0.8% |
| 36 | 1322604ns | +2.0% | -0.6% | -0.1% | -0.2% |
| 37 | 1322748ns | +1.9% | -0.3% | -0.0% | -0.2% |
| 38 | 1324262ns | +1.8% | +0.1% | -0.2% | -0.3% |
| 39 | 1321136ns | +2.1% | -0.4% | +0.2% | +0.0% |
| 40 | 1325035ns | +1.7% | -0.8% | -0.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.079 | ok |
| wide-rung-ragged | 0.049 | ok |
| wide-rung-ragged-overread | 0.270 | moderate+ |
| wide-rung-wordround | 0.078 | ok |
| wide-rung-wordround-alias | 0.098 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 0/40, lost 40/40
- **wide-rung-ragged-overread**: won 31/40, lost 6/40
- **wide-rung-wordround**: won 21/40, lost 7/40
- **wide-rung-wordround-alias**: won 14/40, lost 19/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 10.3ns | 1323769.8ns | 0.0% |  |
| wide-rung-ragged | 11.6ns | 1351879.8ns | 0.0% |  |
| wide-rung-ragged-overread | 8.6ns | 1319179.0ns | 0.0% |  |
| wide-rung-wordround | 11.1ns | 1321099.6ns | 0.0% |  |
| wide-rung-wordround-alias | 11.0ns | 1325955.1ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1319060.6-1332134.2 ns)
  1319060.6 |########################################
  1319714.3 |########################
  1320368.0 |################################
  1321021.7 |################################
  1321675.3 |########################
  1322329.0 |########################
  1322982.7 |
  1323636.4 |################
  1324290.1 |########################
  1324943.7 |################
  1325597.4 |
  1326251.1 |################
  1326904.8 |########################
  1327558.5 |
  1328212.1 |
  1328865.8 |
  1329519.5 |
  1330173.2 |########
  1330826.9 |########
  1331480.6 |
  (2 below, 2 above range)

wide-rung-ragged (n=40, range 1347172.1-1363972.7 ns)
  1347172.1 |##################################
  1348012.2 |########################################
  1348852.2 |##################################
  1349692.2 |#################
  1350532.2 |######################
  1351372.3 |###########
  1352212.3 |###########
  1353052.3 |
  1353892.4 |
  1354732.4 |
  1355572.4 |
  1356412.4 |
  1357252.5 |###########
  1358092.5 |
  1358932.5 |
  1359772.6 |
  1360612.6 |
  1361452.6 |
  1362292.6 |
  1363132.7 |
  (4 below, 4 above range)

wide-rung-ragged-overread (n=40, range 1314607.9-1326969.1 ns)
  1314607.9 |######
  1315225.9 |######
  1315844.0 |##########################
  1316462.0 |##########################
  1317080.1 |########################################
  1317698.2 |
  1318316.2 |####################
  1318934.3 |#############
  1319552.4 |
  1320170.4 |####################
  1320788.5 |#############
  1321406.6 |######
  1322024.6 |
  1322642.7 |######
  1323260.7 |
  1323878.8 |
  1324496.9 |
  1325114.9 |
  1325733.0 |######
  1326351.1 |######
  (6 below, 4 above range)

wide-rung-wordround (n=40, range 1316847.7-1327324.5 ns)
  1316847.7 |####################
  1317371.5 |##############################
  1317895.3 |####################
  1318419.2 |####################
  1318943.0 |########################################
  1319466.9 |##########
  1319990.7 |####################
  1320514.5 |##########
  1321038.4 |##############################
  1321562.2 |########################################
  1322086.1 |####################
  1322609.9 |####################
  1323133.8 |####################
  1323657.6 |##########
  1324181.4 |##########
  1324705.3 |##########
  1325229.1 |##########
  1325753.0 |
  1326276.8 |
  1326800.7 |
  (4 below, 2 above range)

wide-rung-wordround-alias (n=40, range 1319080.6-1339589.8 ns)
  1319080.6 |######################
  1320106.1 |########################################
  1321131.5 |######################
  1322157.0 |#####
  1323182.5 |######################
  1324207.9 |#####
  1325233.4 |######################
  1326258.9 |
  1327284.3 |#################
  1328309.8 |
  1329335.2 |###########
  1330360.7 |#####
  1331386.2 |#####
  1332411.6 |#####
  1333437.1 |
  1334462.5 |
  1335488.0 |
  1336513.5 |
  1337538.9 |
  1338564.4 |#####
  (3 below, 3 above range)

```
