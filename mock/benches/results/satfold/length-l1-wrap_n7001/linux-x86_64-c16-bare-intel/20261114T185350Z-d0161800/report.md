# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (13 ns) is smaller than the fastest variant's own run-to-run std-dev (188 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader satfold-neon8 vs stability leader satfold-lanes4-idx (+0% speed for 1.6x steadier)

satfold-neon8 is fastest (1.59 us, CV 11.8%); satfold-lanes4-idx gives up 0.1% median for 1.6x lower variance (CV 7.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-neon is inconsistent: worst-20% is 1.6x its best-20%

satfold-neon's best 20% of batches run at 1.20 us but its worst 20% at 1.90 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.8% of the fastest

All 9 variants sit between 1.59 us and 1.60 us - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### satfold-nolaw's edge over baseline is significant but tiny (10 ns, 0.65%)

satfold-nolaw differs from baseline satfold-iterfold by 10 ns (0.65%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-neon8** at 1585.2 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 1585.2 ns, slowest 1598.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 1804ns | 1821ns | 1503ns | 1808ns | 2092ns | base |
| satfold-lanes16 | 1877ns | 1824ns | 1571ns | 1841ns | 2293ns | +4.06% |
| satfold-lanes16-constl | 1778ns | 1826ns | 1462ns | 1810ns | 1995ns | -1.47% |
| satfold-lanes4-idx | 1770ns | 1820ns | 1565ns | 1771ns | 1972ns | -1.87% |
| satfold-lanes64 | 1880ns | 1833ns | 1661ns | 1861ns | 2159ns | +4.23% |
| satfold-neon | 1857ns | 1827ns | 1379ns | 1870ns | 2296ns | +2.93% |
| satfold-neon8 | 1818ns | 1821ns | 1479ns | 1826ns | 2132ns | +0.78% |
| satfold-nolaw | 1855ns | 1821ns | 1513ns | 1867ns | 2162ns | +2.84% |
| satfold-seq | 1797ns | 1822ns | 1425ns | 1806ns | 2143ns | -0.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 1556ns | 1311ns | 1773ns | base | 21.061 |
| satfold-lanes16 | 1621ns | 1368ns | 1963ns | +4.20% | 20.211 |
| satfold-lanes16-constl | 1542ns | 1278ns | 1709ns | -0.91% | 21.253 |
| satfold-lanes4-idx | 1532ns | 1366ns | 1679ns | -1.54% | 21.389 |
| satfold-lanes64 | 1624ns | 1445ns | 1851ns | +4.41% | 20.171 |
| satfold-neon | 1580ns | 1198ns | 1903ns | +1.53% | 20.743 |
| satfold-neon8 | 1573ns | 1290ns | 1830ns | +1.10% | 20.832 |
| satfold-nolaw | 1611ns | 1327ns | 1833ns | +3.53% | 20.343 |
| satfold-seq | 1558ns | 1250ns | 1827ns | +0.14% | 21.031 |

## Performance model

- Peak throughput: **27.357 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 20.663 | 75.5% |
| satfold-lanes16 | 20.641 | 75.5% |
| satfold-lanes16-constl | 20.497 | 74.9% |
| satfold-lanes4-idx | 20.649 | 75.5% |
| satfold-lanes64 | 20.649 | 75.5% |
| satfold-neon | 20.656 | 75.5% |
| satfold-neon8 | 20.671 | 75.6% |
| satfold-nolaw | 20.535 | 75.1% |
| satfold-seq | 20.533 | 75.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 1804ns | 1804ns | base |
| satfold-lanes16 | 1877ns | 1877ns | +4.06% |
| satfold-lanes16-constl | 1778ns | 1778ns | -1.47% |
| satfold-lanes4-idx | 1770ns | 1770ns | -1.87% |
| satfold-lanes64 | 1880ns | 1880ns | +4.23% |
| satfold-neon | 1857ns | 1857ns | +2.93% |
| satfold-neon8 | 1818ns | 1818ns | +0.78% |
| satfold-nolaw | 1855ns | 1855ns | +2.84% |
| satfold-seq | 1797ns | 1797ns | -0.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 1586ns | base | --- | [1585, 1587] | --- | --- | --- | --- |
| satfold-lanes16 | 1588ns | no significant difference | [-43, +121]ns | [1585, 1642] | no | 0.5364 | 0.2682 | 0 |
| satfold-lanes16-constl | 1599ns | no significant difference | [-26, +14]ns | [1589, 1601] | no | 0.5364 | 0.2682 | 0 |
| satfold-lanes4-idx | 1587ns | no significant difference | [-31, +3]ns | [1464, 1588] | no | 0.9996 | 0.8746 | 0 |
| satfold-lanes64 | 1587ns | no significant difference | [-1, +152]ns | [1586, 1635] | no | 0.5364 | 0.1539 | 0 |
| satfold-neon | 1586ns | no significant difference | [-2, +52]ns | [1554, 1669] | no | 1.0000 | 1.0000 | 1 |
| satfold-neon8 | 1585ns | no significant difference | [-1, +1]ns | [1585, 1587] | no | 0.9996 | 0.6358 | 0 |
| satfold-nolaw | 1596ns | +10.3ns (+0.6%) | [+9, +76]ns | [1595, 1711] | YES | 0.0001 | 0.0000 | 0 |
| satfold-seq | 1596ns | no significant difference | [-53, +10]ns | [1595, 1596] | no | 0.9996 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1326ns | +10.0% | +0.2% | +3.3% | +10.5% | -7.9% | -0.4% | +0.3% | -6.2% |
| 1 | 2 | 1635ns | -3.0% | -2.1% | -2.9% | -3.0% | +10.6% | -3.1% | +5.8% | -2.4% |
| 1 | 3 | 1586ns | +22.5% | +0.9% | +0.1% | +11.1% | -0.1% | +2.5% | +10.4% | +6.2% |
| 1 | 4 | 1586ns | -6.6% | +0.9% | +0.1% | +1.4% | +2.5% | +0.1% | +0.6% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.220 | moderate- |
| satfold-lanes16 | -0.327 | moderate- |
| satfold-lanes16-constl | -0.086 | ok |
| satfold-lanes4-idx | -0.087 | ok |
| satfold-lanes64 | 0.007 | ok |
| satfold-neon | -0.419 | moderate- |
| satfold-neon8 | -0.014 | ok |
| satfold-nolaw | -0.148 | ok |
| satfold-seq | 0.012 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 2/4
- **satfold-lanes16-constl**: won 1/4, lost 3/4
- **satfold-lanes4-idx**: won 1/4, lost 1/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 1/4, lost 2/4
- **satfold-neon8**: won 2/4, lost 1/4
- **satfold-nolaw**: won 0/4, lost 4/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 83.8ns | 1555.9ns | 5.4% | HIGH |
| satfold-lanes16 | 94.4ns | 1621.3ns | 5.8% | HIGH |
| satfold-lanes16-constl | 83.4ns | 1541.8ns | 5.4% | HIGH |
| satfold-lanes4-idx | 82.8ns | 1532.0ns | 5.4% | HIGH |
| satfold-lanes64 | 87.7ns | 1624.5ns | 5.4% | HIGH |
| satfold-neon | 90.6ns | 1579.7ns | 5.7% | HIGH |
| satfold-neon8 | 84.3ns | 1573.0ns | 5.4% | HIGH |
| satfold-nolaw | 88.9ns | 1610.8ns | 5.5% | HIGH |
| satfold-seq | 84.1ns | 1558.1ns | 5.4% | HIGH |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 1310.5-1772.7 ns)
   1310.5 |########
   1333.6 |
   1356.7 |
   1379.8 |#####
   1402.9 |
   1426.1 |#####
   1449.2 |
   1472.3 |
   1495.4 |#####
   1518.5 |##
   1541.6 |
   1564.7 |########################################
   1587.8 |##############
   1610.9 |##
   1634.0 |##
   1657.1 |##
   1680.3 |
   1703.4 |
   1726.5 |
   1749.6 |#####
  (3 below, 3 above range)

satfold-lanes16 (n=40, range 1367.7-1963.1 ns)
   1367.7 |
   1397.5 |
   1427.3 |#############
   1457.0 |#############
   1486.8 |
   1516.6 |
   1546.3 |
   1576.1 |########################################
   1605.9 |
   1635.6 |######
   1665.4 |######
   1695.2 |######
   1724.9 |
   1754.7 |##########
   1784.5 |
   1814.2 |
   1844.0 |
   1873.8 |###
   1903.5 |
   1933.3 |
  (5 below, 5 above range)

satfold-lanes16-constl (n=40, range 1278.2-1709.4 ns)
   1278.2 |
   1299.8 |
   1321.4 |#########
   1342.9 |
   1364.5 |##
   1386.0 |##
   1407.6 |
   1429.1 |
   1450.7 |##
   1472.3 |
   1493.8 |
   1515.4 |
   1536.9 |
   1558.5 |
   1580.1 |########################################
   1601.6 |#########
   1623.2 |#######
   1644.7 |##
   1666.3 |
   1687.8 |
  (4 below, 4 above range)

satfold-lanes4-idx (n=40, range 1365.8-1679.4 ns)
   1365.8 |
   1381.5 |##
   1397.1 |
   1412.8 |
   1428.5 |############
   1444.2 |#####
   1459.9 |#####
   1475.6 |##
   1491.2 |##
   1506.9 |
   1522.6 |
   1538.3 |##
   1554.0 |
   1569.6 |
   1585.3 |########################################
   1601.0 |
   1616.7 |
   1632.4 |#######
   1648.0 |##
   1663.7 |
  (5 below, 2 above range)

satfold-lanes64 (n=40, range 1445.3-1850.9 ns)
   1445.3 |#####
   1465.6 |##
   1485.9 |
   1506.1 |
   1526.4 |
   1546.7 |
   1567.0 |########################################
   1587.2 |##
   1607.5 |
   1627.8 |###########
   1648.1 |#####
   1668.4 |
   1688.6 |
   1708.9 |##
   1729.2 |
   1749.5 |########
   1769.8 |
   1790.0 |#####
   1810.3 |##
   1830.6 |
  (6 below, 3 above range)

satfold-neon (n=40, range 1197.8-1903.4 ns)
   1197.8 |########################################
   1233.1 |
   1268.4 |
   1303.6 |######
   1338.9 |
   1374.2 |
   1409.5 |####################
   1444.7 |
   1480.0 |######
   1515.3 |######
   1550.6 |########################################
   1585.9 |##########################
   1621.1 |######
   1656.4 |#############
   1691.7 |######
   1727.0 |######
   1762.2 |#############
   1797.5 |####################
   1832.8 |####################
   1868.1 |
  (2 below, 3 above range)

satfold-neon8 (n=40, range 1290.0-1829.5 ns)
   1290.0 |
   1317.0 |#######
   1344.0 |
   1371.0 |##
   1397.9 |
   1424.9 |##
   1451.9 |
   1478.9 |
   1505.8 |##
   1532.8 |
   1559.8 |########################################
   1586.8 |##########
   1613.7 |##
   1640.7 |#######
   1667.7 |
   1694.7 |
   1721.6 |
   1748.6 |#####
   1775.6 |
   1802.6 |##
  (4 below, 3 above range)

satfold-nolaw (n=40, range 1326.9-1832.8 ns)
   1326.9 |###########
   1352.2 |##
   1377.5 |
   1402.8 |
   1428.1 |#####
   1453.4 |##
   1478.7 |
   1504.0 |
   1529.2 |
   1554.5 |
   1579.8 |########################################
   1605.1 |
   1630.4 |##
   1655.7 |
   1681.0 |##
   1706.3 |##
   1731.6 |
   1756.9 |############################
   1782.2 |
   1807.5 |##
  (2 below, 2 above range)

satfold-seq (n=40, range 1249.5-1827.3 ns)
   1249.5 |###
   1278.4 |###
   1307.3 |##########
   1336.2 |
   1365.1 |
   1394.0 |
   1422.8 |
   1451.7 |
   1480.6 |#######
   1509.5 |
   1538.4 |
   1567.3 |########################################
   1596.2 |#####################
   1625.1 |##############
   1654.0 |###
   1682.9 |
   1711.8 |
   1740.7 |
   1769.5 |##########
   1798.4 |
  (5 below, 3 above range)

```
