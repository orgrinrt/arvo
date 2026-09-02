# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (1.36 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround-alias at 1.21 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.93)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.93, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 1214.8 ns median (-10.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.12x (fastest 1214.8 ns, slowest 1360.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1435ns | 1434ns | 1330ns | 1437ns | 1534ns | base |
| wide-rung-ragged | 1320ns | 1291ns | 1246ns | 1304ns | 1439ns | -8.05% |
| wide-rung-ragged-overread | 1375ns | 1390ns | 1243ns | 1362ns | 1546ns | -4.18% |
| wide-rung-wordround | 1401ns | 1344ns | 1241ns | 1364ns | 1671ns | -2.37% |
| wide-rung-wordround-alias | 1340ns | 1278ns | 1245ns | 1303ns | 1544ns | -6.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1362ns | 1265ns | 1456ns | base | 1.503 |
| wide-rung-ragged | 1254ns | 1183ns | 1369ns | -7.94% | 1.633 |
| wide-rung-ragged-overread | 1308ns | 1182ns | 1469ns | -4.02% | 1.566 |
| wide-rung-wordround | 1331ns | 1181ns | 1591ns | -2.26% | 1.538 |
| wide-rung-wordround-alias | 1274ns | 1184ns | 1467ns | -6.50% | 1.608 |

## Performance model

- Peak throughput: **1.734 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.505 | 86.8% |
| wide-rung-ragged | 1.669 | 96.2% |
| wide-rung-ragged-overread | 1.550 | 89.4% |
| wide-rung-wordround | 1.603 | 92.4% |
| wide-rung-wordround-alias | 1.686 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1435ns | 1435ns | base |
| wide-rung-ragged | 1320ns | 1320ns | -8.05% |
| wide-rung-ragged-overread | 1375ns | 1375ns | -4.18% |
| wide-rung-wordround | 1401ns | 1401ns | -2.37% |
| wide-rung-wordround-alias | 1340ns | 1340ns | -6.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1360ns | base | --- | [1358, 1365] | --- | --- | --- | --- |
| wide-rung-ragged | 1227ns | -91.7ns (-6.7%) | [-132, -86]ns | [1224, 1230] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 1321ns | no significant difference | [-170, +13]ns | [1191, 1370] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-wordround | 1278ns | no significant difference | [-84, +5]ns | [1188, 1368] | no | 0.5728 | 0.4296 | 0 |
| wide-rung-wordround-alias | 1215ns | -93.1ns (-6.8%) | [-176, -59]ns | [1205, 1225] | YES | 0.0044 | 0.0022 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 1266ns | -6.5% | +8.0% | -6.8% | -4.4% |
| 2 | 1265ns | -6.3% | +8.4% | -6.5% | -4.6% |
| 3 | 1262ns | -6.9% | +8.4% | -6.2% | -4.3% |
| 4 | 1264ns | -6.6% | +8.4% | -6.6% | -4.6% |
| 5 | 1266ns | -6.7% | +8.3% | -6.7% | -5.0% |
| 6 | 1270ns | -6.5% | +7.6% | -6.9% | -4.9% |
| 7 | 1266ns | -5.9% | +8.5% | -6.2% | -5.1% |
| 8 | 1265ns | -6.2% | +8.3% | -6.6% | -4.7% |
| 9 | 1268ns | -6.6% | +7.8% | -6.9% | -5.2% |
| 10 | 1264ns | -6.3% | +8.3% | -6.3% | -4.7% |
| 11 | 1352ns | -9.7% | -5.9% | -12.3% | +8.7% |
| 12 | 1362ns | -9.8% | -6.4% | -13.2% | +7.3% |
| 13 | 1359ns | -9.9% | -11.2% | -13.3% | +7.6% |
| 14 | 1359ns | -9.7% | -12.7% | -12.6% | +7.7% |
| 15 | 1358ns | -9.5% | -12.3% | -13.0% | +7.2% |
| 16 | 1361ns | -9.8% | -12.5% | -12.3% | +7.6% |
| 17 | 1370ns | -10.3% | -12.4% | -13.0% | +6.9% |
| 18 | 1362ns | -9.6% | -12.7% | -12.7% | +9.0% |
| 19 | 1360ns | -10.0% | -12.7% | -12.7% | +7.2% |
| 20 | 1355ns | -9.7% | -12.8% | -12.4% | +8.2% |
| 21 | 1359ns | -9.7% | -13.1% | +0.6% | -12.8% |
| 22 | 1362ns | -10.1% | -13.1% | +0.4% | -12.9% |
| 23 | 1358ns | -10.3% | -13.1% | +0.4% | -13.0% |
| 24 | 1360ns | -10.3% | -13.1% | +0.4% | -13.0% |
| 25 | 1367ns | -10.2% | -13.7% | -0.3% | -13.5% |
| 26 | 1360ns | -10.0% | -12.3% | +1.1% | -13.0% |
| 27 | 1364ns | -10.0% | -12.6% | +0.3% | -13.0% |
| 28 | 1431ns | -11.4% | -17.1% | -4.3% | -17.2% |
| 29 | 1365ns | -6.9% | -13.2% | -0.1% | -13.2% |
| 30 | 1356ns | -6.2% | -12.6% | +0.9% | -12.4% |
| 31 | 1458ns | -6.0% | +0.1% | +8.6% | -16.2% |
| 32 | 1450ns | -5.6% | +0.9% | +9.6% | -15.8% |
| 33 | 1450ns | -6.1% | +0.8% | +9.9% | -15.6% |
| 34 | 1463ns | -6.9% | -0.1% | +8.5% | -16.2% |
| 35 | 1457ns | -5.9% | +0.8% | +9.0% | -16.3% |
| 36 | 1455ns | -5.8% | +1.2% | +9.3% | -15.8% |
| 37 | 1457ns | -5.8% | +1.2% | +9.1% | -15.9% |
| 38 | 1455ns | -5.8% | +0.8% | +9.5% | -8.3% |
| 39 | 1455ns | -6.1% | +0.9% | +9.5% | -12.3% |
| 40 | 1451ns | -6.1% | +2.1% | +9.1% | -15.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.878 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.923 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.888 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.925 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.828 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 40/40, lost 0/40
- **wide-rung-ragged-overread**: won 21/40, lost 19/40
- **wide-rung-wordround**: won 22/40, lost 17/40
- **wide-rung-wordround-alias**: won 30/40, lost 10/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.6ns | 1362.2ns | 0.2% |  |
| wide-rung-ragged | 2.5ns | 1254.0ns | 0.2% |  |
| wide-rung-ragged-overread | 2.5ns | 1307.5ns | 0.2% |  |
| wide-rung-wordround | 2.6ns | 1331.5ns | 0.2% |  |
| wide-rung-wordround-alias | 2.9ns | 1273.7ns | 0.2% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1264.9-1456.3 ns)
   1264.9 |#########################
   1274.5 |
   1284.0 |
   1293.6 |
   1303.2 |
   1312.7 |
   1322.3 |
   1331.9 |
   1341.5 |
   1351.0 |########################################
   1360.6 |#############################
   1370.2 |
   1379.7 |
   1389.3 |
   1398.9 |
   1408.4 |
   1418.0 |
   1427.6 |###
   1437.2 |
   1446.7 |#####################
  (3 below, 4 above range)

wide-rung-ragged (n=40, range 1182.7-1369.0 ns)
   1182.7 |################
   1192.0 |##
   1201.3 |
   1210.7 |##
   1220.0 |########################################
   1229.3 |##
   1238.6 |
   1247.9 |
   1257.2 |
   1266.5 |########
   1275.9 |
   1285.2 |
   1294.5 |
   1303.8 |
   1313.1 |
   1322.4 |
   1331.7 |
   1341.1 |
   1350.4 |
   1359.7 |#############
  (3 below, 5 above range)

wide-rung-ragged-overread (n=40, range 1182.2-1469.5 ns)
   1182.2 |########################################
   1196.6 |######
   1211.0 |
   1225.3 |
   1239.7 |
   1254.1 |
   1268.4 |######
   1282.8 |
   1297.1 |
   1311.5 |
   1325.9 |
   1340.2 |
   1354.6 |#############
   1368.9 |####################
   1383.3 |
   1397.7 |
   1412.0 |
   1426.4 |
   1440.8 |
   1455.1 |#######################
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 1180.9-1590.7 ns)
   1180.9 |########################################
   1201.4 |
   1221.9 |
   1242.4 |
   1262.9 |
   1283.4 |
   1303.9 |
   1324.4 |
   1344.9 |#######
   1365.3 |#################
   1385.8 |
   1406.3 |
   1426.8 |
   1447.3 |
   1467.8 |
   1488.3 |
   1508.8 |
   1529.3 |
   1549.7 |
   1570.2 |###############
  (4 below, 4 above range)

wide-rung-wordround-alias (n=40, range 1184.1-1467.1 ns)
   1184.1 |############################
   1198.3 |########################################
   1212.4 |################################
   1226.6 |
   1240.7 |
   1254.9 |
   1269.0 |####
   1283.2 |
   1297.3 |
   1311.5 |
   1325.6 |####
   1339.8 |
   1353.9 |
   1368.1 |
   1382.2 |
   1396.4 |
   1410.5 |
   1424.7 |
   1438.8 |
   1453.0 |################################
  (3 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.92 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.93 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.83 (measurement drift or warm-up artifact)
