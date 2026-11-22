# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (1.65 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-nolaw at 1.40 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-nolaw beats baseline by 22% (significant)

satfold-nolaw is -355 ns (22%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-neon shows alternating (throttle bounce) (autocorr -0.76)

satfold-neon's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### satfold-lanes4-idx is inconsistent: worst-20% is 1.5x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.32 us but its worst 20% at 2.03 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-nolaw** at 1397.2 ns median (-15.3% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 1.18x (fastest 1397.2 ns, slowest 1648.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112766038 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 1967ns | 1891ns | 1819ns | 1923ns | 2246ns | base |
| satfold-lanes16 | 1941ns | 1825ns | 1740ns | 1896ns | 2276ns | -1.32% |
| satfold-lanes16-constl | 1774ns | 1712ns | 1457ns | 1745ns | 2178ns | -9.80% |
| satfold-lanes4-idx | 1854ns | 1823ns | 1518ns | 1799ns | 2355ns | -5.73% |
| satfold-lanes64 | 1937ns | 1872ns | 1779ns | 1901ns | 2201ns | -1.52% |
| satfold-neon | 1721ns | 1792ns | 1434ns | 1707ns | 2051ns | -12.48% |
| satfold-neon8 | 1670ns | 1656ns | 1517ns | 1661ns | 1853ns | -15.07% |
| satfold-nolaw | 1664ns | 1643ns | 1366ns | 1650ns | 2005ns | -15.38% |
| satfold-seq | 1822ns | 1821ns | 1637ns | 1831ns | 1977ns | -7.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 1698ns | 1582ns | 1901ns | base | 19.293 |
| satfold-lanes16 | 1680ns | 1514ns | 1942ns | -1.11% | 19.510 |
| satfold-lanes16-constl | 1538ns | 1267ns | 1885ns | -9.44% | 21.305 |
| satfold-lanes4-idx | 1603ns | 1320ns | 2034ns | -5.61% | 20.439 |
| satfold-lanes64 | 1677ns | 1547ns | 1870ns | -1.25% | 19.537 |
| satfold-neon | 1489ns | 1249ns | 1760ns | -12.34% | 22.010 |
| satfold-neon8 | 1446ns | 1319ns | 1593ns | -14.87% | 22.664 |
| satfold-nolaw | 1450ns | 1197ns | 1738ns | -14.61% | 22.595 |
| satfold-seq | 1586ns | 1427ns | 1704ns | -6.60% | 20.657 |

## Performance model

- Peak throughput: **27.386 Gops/s** (satfold-nolaw; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 19.872 | 72.6% |
| satfold-lanes16 | 20.634 | 75.3% |
| satfold-lanes16-constl | 21.918 | 80.0% |
| satfold-lanes4-idx | 20.679 | 75.5% |
| satfold-lanes64 | 20.021 | 73.1% |
| satfold-neon | 21.249 | 77.6% |
| satfold-neon8 | 22.723 | 83.0% |
| satfold-nolaw | 23.452 | 85.6% |
| satfold-seq | 20.543 | 75.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 1967ns | 1967ns | base |
| satfold-lanes16 | 1941ns | 1941ns | -1.32% |
| satfold-lanes16-constl | 1774ns | 1774ns | -9.80% |
| satfold-lanes4-idx | 1854ns | 1854ns | -5.73% |
| satfold-lanes64 | 1937ns | 1937ns | -1.52% |
| satfold-neon | 1721ns | 1721ns | -12.48% |
| satfold-neon8 | 1670ns | 1670ns | -15.07% |
| satfold-nolaw | 1664ns | 1664ns | -15.38% |
| satfold-seq | 1822ns | 1822ns | -7.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 1649ns | base | --- | [1585, 1761] | --- | --- | --- | --- |
| satfold-lanes16 | 1588ns | no significant difference | [-47, +3]ns | [1586, 1749] | no | 0.7267 | 0.6358 | 0 |
| satfold-lanes16-constl | 1495ns | -237.9ns (-14.4%) | [-307, -87]ns | [1449, 1601] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| satfold-lanes4-idx | 1585ns | -143.2ns (-8.7%) | [-184, -48]ns | [1453, 1588] | YES | 0.0442 | 0.0166 | 0 |
| satfold-lanes64 | 1637ns | no significant difference | [-35, +2]ns | [1586, 1741] | no | 1.0000 | 1.0000 | 0 |
| satfold-neon | 1542ns | -219.5ns (-13.3%) | [-282, -56]ns | [1322, 1584] | YES | 0.0007 | 0.0002 | 0 |
| satfold-neon8 | 1442ns | -263.3ns (-16.0%) | [-315, -210]ns | [1395, 1465] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 1397ns | -355.4ns (-21.6%) | [-416, -95]ns | [1329, 1595] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| satfold-seq | 1595ns | -116.7ns (-7.1%) | [-167, -50]ns | [1595, 1596] | YES (adj: no) | 0.0513 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1585ns | +0.0% | -18.4% | -8.4% | +11.0% | -16.6% | -10.5% | +4.8% | +0.7% |
| 1 | 2 | 1699ns | +3.8% | +5.5% | -6.6% | -5.8% | -5.4% | -15.5% | -21.8% | -14.6% |
| 1 | 3 | 1719ns | +1.7% | -15.7% | -20.2% | -6.6% | -27.8% | -19.8% | -7.2% | -7.2% |
| 1 | 4 | 1712ns | -7.3% | -6.5% | +15.7% | -5.8% | -4.1% | -7.4% | -28.4% | -6.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.024 | ok |
| satfold-lanes16 | -0.251 | moderate- |
| satfold-lanes16-constl | -0.658 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.375 | moderate- |
| satfold-lanes64 | -0.128 | ok |
| satfold-neon | -0.764 | HIGH- (thermal bounce) |
| satfold-neon8 | -0.318 | moderate- |
| satfold-nolaw | -0.581 | HIGH- (thermal bounce) |
| satfold-seq | -0.419 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 1/4, lost 2/4
- **satfold-lanes16-constl**: won 3/4, lost 1/4
- **satfold-lanes4-idx**: won 3/4, lost 1/4
- **satfold-lanes64**: won 3/4, lost 1/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 3/4, lost 1/4
- **satfold-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 92.3ns | 1698.4ns | 5.4% | HIGH |
| satfold-lanes16 | 93.3ns | 1679.6ns | 5.6% | HIGH |
| satfold-lanes16-constl | 83.5ns | 1538.0ns | 5.4% | HIGH |
| satfold-lanes4-idx | 87.4ns | 1603.2ns | 5.5% | HIGH |
| satfold-lanes64 | 90.9ns | 1677.2ns | 5.4% | HIGH |
| satfold-neon | 82.7ns | 1488.8ns | 5.6% | HIGH |
| satfold-neon8 | 79.3ns | 1445.8ns | 5.5% | HIGH |
| satfold-nolaw | 78.8ns | 1450.2ns | 5.4% | HIGH |
| satfold-seq | 85.7ns | 1586.3ns | 5.4% | HIGH |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 1582.3-1900.8 ns)
   1582.3 |########################################
   1598.2 |
   1614.2 |###
   1630.1 |######
   1646.0 |
   1661.9 |###
   1677.9 |###
   1693.8 |
   1709.7 |###
   1725.6 |
   1741.6 |
   1757.5 |###########################
   1773.4 |###
   1789.3 |
   1805.2 |######
   1821.2 |###
   1837.1 |###
   1853.0 |
   1868.9 |
   1884.9 |
  (4 below, 3 above range)

satfold-lanes16 (n=40, range 1513.7-1941.9 ns)
   1513.7 |
   1535.1 |##
   1556.6 |
   1578.0 |########################################
   1599.4 |
   1620.8 |##
   1642.2 |#####
   1663.6 |##
   1685.0 |
   1706.4 |
   1727.8 |##
   1749.2 |#################
   1770.7 |##
   1792.1 |
   1813.5 |
   1834.9 |
   1856.3 |
   1877.7 |
   1899.1 |
   1920.5 |
  (4 below, 6 above range)

satfold-lanes16-constl (n=40, range 1267.0-1885.3 ns)
   1267.0 |
   1297.9 |####################
   1328.8 |##########################
   1359.7 |######
   1390.6 |
   1421.5 |####################
   1452.5 |##########################
   1483.4 |
   1514.3 |######
   1545.2 |
   1576.1 |########################################
   1607.0 |######
   1638.0 |####################
   1668.9 |
   1699.8 |######
   1730.7 |
   1761.6 |####################
   1792.5 |######
   1823.5 |
   1854.4 |
  (5 below, 4 above range)

satfold-lanes4-idx (n=40, range 1320.1-2034.0 ns)
   1320.1 |############
   1355.8 |########
   1391.5 |
   1427.2 |############################
   1462.9 |########
   1498.6 |
   1534.3 |####
   1570.0 |########################################
   1605.7 |####
   1641.4 |####
   1677.1 |
   1712.8 |
   1748.5 |########
   1784.2 |########
   1819.9 |
   1855.6 |
   1891.3 |
   1927.0 |
   1962.6 |########
   1998.3 |
  (3 below, 4 above range)

satfold-lanes64 (n=40, range 1546.9-1870.5 ns)
   1546.9 |
   1563.1 |
   1579.2 |########################################
   1595.4 |
   1611.6 |#####
   1627.8 |########
   1644.0 |##
   1660.1 |
   1676.3 |
   1692.5 |#####
   1708.7 |##
   1724.8 |
   1741.0 |
   1757.2 |#####################
   1773.4 |
   1789.6 |
   1805.7 |
   1821.9 |##
   1838.1 |##
   1854.3 |#####
  (2 below, 2 above range)

satfold-neon (n=40, range 1248.8-1760.1 ns)
   1248.8 |####
   1274.4 |
   1299.9 |########################################
   1325.5 |
   1351.1 |
   1376.6 |
   1402.2 |
   1427.8 |############
   1453.3 |
   1478.9 |####
   1504.4 |
   1530.0 |
   1555.6 |
   1581.1 |########################################
   1606.7 |########
   1632.3 |
   1657.8 |
   1683.4 |####
   1709.0 |
   1734.5 |####
  (5 below, 6 above range)

satfold-neon8 (n=40, range 1319.3-1593.1 ns)
   1319.3 |###############################
   1333.0 |
   1346.7 |
   1360.4 |####
   1374.1 |####
   1387.7 |########
   1401.4 |
   1415.1 |
   1428.8 |########################################
   1442.5 |########
   1456.2 |
   1469.9 |
   1483.6 |########
   1497.3 |
   1511.0 |####
   1524.6 |
   1538.3 |####
   1552.0 |####
   1565.7 |
   1579.4 |###################################
  (4 below, 1 above range)

satfold-nolaw (n=40, range 1196.5-1738.0 ns)
   1196.5 |
   1223.6 |##########################
   1250.7 |######
   1277.7 |
   1304.8 |####################
   1331.9 |###
   1359.0 |
   1386.0 |
   1413.1 |
   1440.2 |###
   1467.3 |
   1494.4 |
   1521.4 |
   1548.5 |
   1575.6 |########################################
   1602.7 |
   1629.7 |######
   1656.8 |
   1683.9 |
   1711.0 |###
  (3 below, 4 above range)

satfold-seq (n=40, range 1427.5-1704.1 ns)
   1427.5 |#
   1441.3 |#######
   1455.1 |
   1469.0 |
   1482.8 |
   1496.6 |
   1510.5 |#
   1524.3 |
   1538.1 |
   1552.0 |
   1565.8 |
   1579.6 |
   1593.4 |########################################
   1607.3 |#
   1621.1 |
   1634.9 |#########
   1648.8 |###
   1662.6 |
   1676.4 |
   1690.3 |#
  (2 below, 2 above range)

```
