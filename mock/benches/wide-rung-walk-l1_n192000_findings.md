# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-align16 shows warm-up / thermal drift (autocorr +0.89)

wide-rung-align16's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-ragged's edge over baseline is significant but tiny (9 ns, 0.70%)

wide-rung-ragged differs from baseline wide-rung-align16 by 9 ns (0.70%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 1224.8 ns median (-6.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.12x (fastest 1224.8 ns, slowest 1369.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1385ns | 1379ns | 1245ns | 1383ns | 1533ns | base |
| wide-rung-ragged | 1420ns | 1445ns | 1244ns | 1436ns | 1549ns | +2.53% |
| wide-rung-ragged-overread | 1369ns | 1390ns | 1246ns | 1382ns | 1452ns | -1.20% |
| wide-rung-wordround | 1423ns | 1388ns | 1241ns | 1401ns | 1674ns | +2.73% |
| wide-rung-wordround-alias | 1289ns | 1288ns | 1246ns | 1288ns | 1335ns | -6.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1318ns | 1184ns | 1459ns | base | 1.554 |
| wide-rung-ragged | 1349ns | 1183ns | 1473ns | +2.34% | 1.519 |
| wide-rung-ragged-overread | 1302ns | 1185ns | 1380ns | -1.23% | 1.574 |
| wide-rung-wordround | 1353ns | 1181ns | 1590ns | +2.67% | 1.514 |
| wide-rung-wordround-alias | 1225ns | 1183ns | 1269ns | -7.00% | 1.671 |

## Performance model

- Peak throughput: **1.734 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.560 | 90.0% |
| wide-rung-ragged | 1.495 | 86.2% |
| wide-rung-ragged-overread | 1.550 | 89.4% |
| wide-rung-wordround | 1.556 | 89.7% |
| wide-rung-wordround-alias | 1.672 | 96.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1385ns | 1385ns | base |
| wide-rung-ragged | 1420ns | 1420ns | +2.53% |
| wide-rung-ragged-overread | 1369ns | 1369ns | -1.20% |
| wide-rung-wordround | 1423ns | 1423ns | +2.73% |
| wide-rung-wordround-alias | 1289ns | 1289ns | -6.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1313ns | base | --- | [1261, 1362] | --- | --- | --- | --- |
| wide-rung-ragged | 1370ns | +6.2ns (+0.5%) | [+2, +10]ns | [1366, 1374] | YES | 0.0021 | 0.0011 | 1 |
| wide-rung-ragged-overread | 1322ns | no significant difference | [-88, +3]ns | [1270, 1368] | no | 0.4296 | 0.4296 | 0 |
| wide-rung-wordround | 1316ns | +6.7ns (+0.5%) | [+4, +10]ns | [1268, 1369] | YES | 0.0030 | 0.0022 | 0 |
| wide-rung-wordround-alias | 1225ns | -91.5ns (-7.0%) | [-138, -56]ns | [1206, 1235] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 1265ns | +8.1% | +8.1% | +0.3% | -4.7% |
| 2 | 1261ns | +8.9% | +8.5% | +0.9% | -4.4% |
| 3 | 1263ns | +8.7% | +8.1% | +0.4% | -4.4% |
| 4 | 1260ns | +8.7% | +8.6% | +0.8% | -4.3% |
| 5 | 1270ns | +8.2% | +8.0% | -0.4% | -4.7% |
| 6 | 1262ns | +9.4% | +8.5% | +0.7% | -4.6% |
| 7 | 1265ns | +8.7% | +8.8% | +0.4% | -4.7% |
| 8 | 1268ns | +8.5% | +8.1% | +0.1% | -5.1% |
| 9 | 1258ns | +9.1% | +9.3% | +0.6% | -4.4% |
| 10 | 1259ns | +8.8% | +8.6% | +0.5% | +0.8% |
| 11 | 1180ns | +0.4% | +0.1% | +0.6% | +4.2% |
| 12 | 1182ns | +0.0% | +0.6% | +0.8% | +5.0% |
| 13 | 1188ns | -0.7% | -0.4% | -0.4% | -0.5% |
| 14 | 1185ns | -0.4% | +0.1% | -0.8% | +0.1% |
| 15 | 1188ns | -0.3% | -0.5% | -0.6% | -0.6% |
| 16 | 1185ns | -0.1% | +0.3% | -0.5% | -0.3% |
| 17 | 1188ns | +0.1% | -0.1% | -0.7% | +0.2% |
| 18 | 1181ns | +0.4% | +0.4% | +0.5% | +0.2% |
| 19 | 1185ns | +0.1% | +0.2% | -0.2% | -0.4% |
| 20 | 1189ns | -0.4% | +2.8% | -0.7% | -0.7% |
| 21 | 1458ns | +0.6% | -6.0% | +8.9% | -13.1% |
| 22 | 1458ns | +4.9% | -6.0% | +9.0% | -12.8% |
| 23 | 1455ns | +0.7% | -6.2% | +9.0% | -12.7% |
| 24 | 1450ns | +1.0% | -1.8% | +9.3% | -12.9% |
| 25 | 1462ns | +0.0% | -6.3% | +8.6% | -13.1% |
| 26 | 1458ns | +0.4% | -6.2% | +8.9% | -13.1% |
| 27 | 1469ns | -0.1% | -6.2% | +8.8% | -13.9% |
| 28 | 1458ns | +0.3% | -5.8% | +9.2% | -12.9% |
| 29 | 1455ns | +0.7% | -5.8% | +9.3% | -13.0% |
| 30 | 1455ns | +0.0% | -5.9% | +9.2% | -12.7% |
| 31 | 1363ns | -0.1% | -6.6% | +0.3% | -10.2% |
| 32 | 1358ns | +0.9% | -6.8% | +0.5% | -9.6% |
| 33 | 1357ns | +0.5% | -6.8% | +0.8% | -9.7% |
| 34 | 1359ns | +0.5% | -6.6% | +0.1% | -10.1% |
| 35 | 1361ns | +0.4% | -6.5% | +0.2% | -9.7% |
| 36 | 1366ns | +0.3% | -7.0% | +0.2% | -10.3% |
| 37 | 1387ns | -1.0% | -7.8% | -1.3% | -10.4% |
| 38 | 1384ns | -1.2% | -7.9% | -1.0% | -11.4% |
| 39 | 1356ns | +0.7% | -6.1% | +3.9% | -9.8% |
| 40 | 1359ns | +0.7% | -6.7% | +0.7% | -8.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.890 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.847 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.835 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.874 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.726 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 9/40, lost 26/40
- **wide-rung-ragged-overread**: won 23/40, lost 16/40
- **wide-rung-wordround**: won 10/40, lost 30/40
- **wide-rung-wordround-alias**: won 34/40, lost 5/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.7ns | 1317.7ns | 0.2% |  |
| wide-rung-ragged | 2.6ns | 1348.6ns | 0.2% |  |
| wide-rung-ragged-overread | 2.3ns | 1301.5ns | 0.2% |  |
| wide-rung-wordround | 2.9ns | 1352.9ns | 0.2% |  |
| wide-rung-wordround-alias | 2.5ns | 1225.5ns | 0.2% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1184.2-1459.0 ns)
   1184.2 |###################################
   1198.0 |
   1211.7 |
   1225.4 |
   1239.2 |
   1252.9 |########################################
   1266.7 |##########
   1280.4 |
   1294.1 |
   1307.9 |
   1321.6 |
   1335.4 |
   1349.1 |##############################
   1362.8 |##########
   1376.6 |##########
   1390.3 |
   1404.1 |
   1417.8 |
   1431.5 |
   1445.3 |########################################
  (3 below, 2 above range)

wide-rung-ragged (n=40, range 1183.1-1473.0 ns)
   1183.1 |#######################
   1197.6 |
   1212.1 |
   1226.6 |
   1241.1 |
   1255.6 |
   1270.1 |
   1284.6 |
   1299.1 |
   1313.6 |
   1328.1 |
   1342.6 |
   1357.1 |########################################
   1371.6 |##########################
   1386.0 |
   1400.5 |
   1415.0 |
   1429.5 |
   1444.0 |###
   1458.5 |##########################
  (3 below, 1 above range)

wide-rung-ragged-overread (n=40, range 1185.1-1380.0 ns)
   1185.1 |####################
   1194.9 |
   1204.6 |
   1214.4 |###
   1224.1 |
   1233.9 |
   1243.6 |
   1253.3 |
   1263.1 |####################
   1272.8 |#############
   1282.6 |
   1292.3 |
   1302.1 |
   1311.8 |
   1321.5 |
   1331.3 |
   1341.0 |
   1350.8 |
   1360.5 |########################################
   1370.3 |#######################
  (3 below, 1 above range)

wide-rung-wordround (n=40, range 1181.1-1590.2 ns)
   1181.1 |########################
   1201.6 |
   1222.0 |
   1242.5 |
   1262.9 |########################################
   1283.4 |
   1303.8 |
   1324.3 |
   1344.7 |############
   1365.2 |########################
   1385.7 |
   1406.1 |####
   1426.6 |
   1447.0 |
   1467.5 |
   1487.9 |
   1508.4 |
   1528.8 |
   1549.3 |
   1569.7 |############################
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1183.2-1269.2 ns)
   1183.2 |################
   1187.5 |########
   1191.8 |
   1196.1 |
   1200.4 |########################
   1204.7 |########################################
   1209.0 |########
   1213.3 |
   1217.6 |########
   1221.9 |################################
   1226.2 |################################
   1230.5 |
   1234.8 |
   1239.1 |########################
   1243.4 |
   1247.7 |
   1252.0 |
   1256.3 |
   1260.6 |########
   1264.9 |########################################
  (5 below, 5 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.73 (measurement drift or warm-up artifact)
