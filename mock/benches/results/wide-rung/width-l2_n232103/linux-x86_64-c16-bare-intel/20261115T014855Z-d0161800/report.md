# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround shows alternating (throttle bounce) (autocorr -0.76)

wide-rung-wordround's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 1383381.9 ns median (-7.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.09x (fastest 1383381.9 ns, slowest 1505513.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1504719ns | 1498472ns | 1391148ns | 1504343ns | 1619421ns | base |
| wide-rung-ragged | 1539833ns | 1506105ns | 1448379ns | 1539384ns | 1632637ns | +2.33% |
| wide-rung-ragged-overread | 1509212ns | 1484839ns | 1404602ns | 1491873ns | 1665839ns | +0.30% |
| wide-rung-wordround | 1458635ns | 1490494ns | 1373030ns | 1455697ns | 1553053ns | -3.06% |
| wide-rung-wordround-alias | 1410415ns | 1383895ns | 1373007ns | 1390794ns | 1506686ns | -6.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1504168ns | 1390734ns | 1618911ns | base | 1.220 |
| wide-rung-ragged | 1539265ns | 1447801ns | 1632115ns | +2.33% | 1.192 |
| wide-rung-ragged-overread | 1508574ns | 1403905ns | 1665143ns | +0.29% | 1.216 |
| wide-rung-wordround | 1458014ns | 1372572ns | 1552375ns | -3.07% | 1.259 |
| wide-rung-wordround-alias | 1409882ns | 1372458ns | 1506108ns | -6.27% | 1.302 |

## Performance model

- Peak throughput: **1.337 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.225 | 91.6% |
| wide-rung-ragged | 1.219 | 91.2% |
| wide-rung-ragged-overread | 1.236 | 92.5% |
| wide-rung-wordround | 1.232 | 92.1% |
| wide-rung-wordround-alias | 1.326 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1504719ns | 1504719ns | base |
| wide-rung-ragged | 1539833ns | 1539833ns | +2.33% |
| wide-rung-ragged-overread | 1509212ns | 1509212ns | +0.30% |
| wide-rung-wordround | 1458635ns | 1458635ns | -3.06% |
| wide-rung-wordround-alias | 1410415ns | 1410415ns | -6.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1497936ns | base | --- | [1494494, 1513624] | --- | --- | --- | --- |
| wide-rung-ragged | 1505514ns | no significant difference | [-5152, +60804]ns | [1497709, 1617809] | no | 0.5728 | 0.4296 | 0 |
| wide-rung-ragged-overread | 1484331ns | no significant difference | [-83978, +77053]ns | [1433033, 1497553] | no | 0.6358 | 0.6358 | 0 |
| wide-rung-wordround | 1489764ns | -20299.4ns (-1.4%) | [-119073, -12181]ns | [1376835, 1497428] | YES | 0.0129 | 0.0064 | 0 |
| wide-rung-wordround-alias | 1383382ns | -105682.2ns (-7.1%) | [-122020, -27743]ns | [1374226, 1388919] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1509399ns | -4.1% | +10.6% | -9.1% | -8.0% |
| 1 | 2 | 1514134ns | +7.9% | -5.6% | -0.9% | -1.8% |
| 1 | 3 | 1494922ns | +0.2% | -3.2% | -7.9% | -8.2% |
| 1 | 4 | 1376337ns | +17.6% | +7.8% | +8.3% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.018 | ok |
| wide-rung-ragged | -0.661 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | -0.178 | ok |
| wide-rung-wordround | -0.760 | HIGH- (thermal bounce) |
| wide-rung-wordround-alias | -0.340 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 1/4, lost 3/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 72.2ns | 1504167.6ns | 0.0% |  |
| wide-rung-ragged | 74.0ns | 1539265.1ns | 0.0% |  |
| wide-rung-ragged-overread | 97.7ns | 1508573.9ns | 0.0% |  |
| wide-rung-wordround | 70.1ns | 1458014.0ns | 0.0% |  |
| wide-rung-wordround-alias | 56.1ns | 1409881.6ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1390733.6-1618911.1 ns)
  1390733.6 |
  1402142.5 |
  1413551.4 |
  1424960.2 |
  1436369.1 |
  1447778.0 |
  1459186.9 |
  1470595.7 |
  1482004.6 |######
  1493413.5 |########################################
  1504822.4 |########################
  1516231.2 |#########
  1527640.1 |
  1539049.0 |
  1550457.8 |###
  1561866.7 |
  1573275.6 |
  1584684.5 |
  1596093.3 |
  1607502.2 |###
  (7 below, 5 above range)

wide-rung-ragged (n=40, range 1447801.2-1632115.2 ns)
  1447801.2 |##########################
  1457016.9 |
  1466232.6 |
  1475448.3 |
  1484664.0 |
  1493879.7 |########################################
  1503095.4 |#################
  1512311.1 |
  1521526.8 |
  1530742.5 |
  1539958.2 |
  1549173.9 |
  1558389.6 |
  1567605.3 |
  1576821.0 |####
  1586036.7 |
  1595252.4 |
  1604468.1 |####
  1613683.8 |###################################
  1622899.5 |####
  (4 below, 6 above range)

wide-rung-ragged-overread (n=40, range 1403904.6-1665143.5 ns)
  1403904.6 |
  1416966.5 |###################################
  1430028.5 |########
  1443090.4 |
  1456152.4 |
  1469214.3 |####
  1482276.3 |########################################
  1495338.2 |########
  1508400.1 |
  1521462.1 |
  1534524.0 |
  1547586.0 |####
  1560647.9 |####
  1573709.9 |####
  1586771.8 |
  1599833.8 |
  1612895.7 |########
  1625957.7 |####
  1639019.6 |
  1652081.5 |
  (5 below, 7 above range)

wide-rung-wordround (n=40, range 1372572.0-1552375.3 ns)
  1372572.0 |####################################
  1381562.2 |####
  1390552.4 |
  1399542.5 |
  1408532.7 |
  1417522.8 |
  1426513.0 |
  1435503.2 |
  1444493.3 |
  1453483.5 |
  1462473.7 |
  1471463.8 |
  1480454.0 |############
  1489444.1 |########################################
  1498434.3 |############################
  1507424.5 |####
  1516414.6 |
  1525404.8 |
  1534395.0 |
  1543385.1 |
  (6 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1372458.2-1506108.3 ns)
  1372458.2 |########################################
  1379140.8 |##
  1385823.3 |########################
  1392505.8 |##
  1399188.3 |
  1405870.8 |
  1412553.3 |
  1419235.8 |
  1425918.3 |
  1432600.8 |
  1439283.3 |
  1445965.8 |
  1452648.3 |
  1459330.8 |
  1466013.3 |
  1472695.8 |
  1479378.3 |##
  1486060.8 |#####################
  1492743.3 |
  1499425.8 |
  (4 below, 1 above range)

```
