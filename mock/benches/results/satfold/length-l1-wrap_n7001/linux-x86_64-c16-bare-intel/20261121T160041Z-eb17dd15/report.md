# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes16 shows alternating (throttle bounce) (autocorr -0.79)

satfold-lanes16's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (190 ns) is smaller than the fastest variant's own run-to-run std-dev (220 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### satfold-seq is inconsistent: worst-20% is 2.2x its best-20%

satfold-seq's best 20% of batches run at 1.23 us but its worst 20% at 2.64 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### satfold-lanes16's edge over baseline is significant but tiny (-16 ns, 1.03%)

satfold-lanes16 differs from baseline satfold-iterfold by -16 ns (1.03%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-lanes16** at 1408.5 ns median (-11.6% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.14x (fastest 1408.5 ns, slowest 1598.8 ns)

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
| Counter frequency | 2112745822 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 1816ns | 1820ns | 1562ns | 1805ns | 2101ns | base |
| satfold-lanes16 | 1616ns | 1635ns | 1270ns | 1627ns | 1929ns | -10.99% |
| satfold-lanes16-constl | 1890ns | 1827ns | 1781ns | 1848ns | 2124ns | +4.10% |
| satfold-lanes4-idx | 1734ns | 1818ns | 1522ns | 1745ns | 1915ns | -4.49% |
| satfold-lanes64 | 1739ns | 1678ns | 1507ns | 1709ns | 2058ns | -4.25% |
| satfold-neon | 1767ns | 1813ns | 1453ns | 1723ns | 2215ns | -2.67% |
| satfold-neon8 | 1759ns | 1818ns | 1351ns | 1777ns | 2113ns | -3.15% |
| satfold-nolaw | 1681ns | 1654ns | 1494ns | 1639ns | 1994ns | -7.45% |
| satfold-seq | 1934ns | 1660ns | 1403ns | 1742ns | 3037ns | +6.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 1574ns | 1370ns | 1781ns | base | 20.824 |
| satfold-lanes16 | 1407ns | 1112ns | 1666ns | -10.60% | 23.294 |
| satfold-lanes16-constl | 1643ns | 1561ns | 1828ns | +4.43% | 19.942 |
| satfold-lanes4-idx | 1511ns | 1334ns | 1643ns | -3.96% | 21.682 |
| satfold-lanes64 | 1516ns | 1319ns | 1795ns | -3.68% | 21.620 |
| satfold-neon | 1538ns | 1273ns | 1924ns | -2.23% | 21.300 |
| satfold-neon8 | 1530ns | 1182ns | 1824ns | -2.79% | 21.421 |
| satfold-nolaw | 1463ns | 1302ns | 1738ns | -7.01% | 22.395 |
| satfold-seq | 1682ns | 1228ns | 2644ns | +6.91% | 19.479 |

## Performance model

- Peak throughput: **29.462 Gops/s** (satfold-lanes16; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 20.562 | 69.8% |
| satfold-lanes16 | 23.265 | 79.0% |
| satfold-lanes16-constl | 20.495 | 69.6% |
| satfold-lanes4-idx | 20.927 | 71.0% |
| satfold-lanes64 | 22.588 | 76.7% |
| satfold-neon | 21.395 | 72.6% |
| satfold-neon8 | 20.571 | 69.8% |
| satfold-nolaw | 22.607 | 76.7% |
| satfold-seq | 22.589 | 76.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 1816ns | 1816ns | base |
| satfold-lanes16 | 1616ns | 1616ns | -10.99% |
| satfold-lanes16-constl | 1890ns | 1890ns | +4.10% |
| satfold-lanes4-idx | 1734ns | 1734ns | -4.49% |
| satfold-lanes64 | 1739ns | 1739ns | -4.25% |
| satfold-neon | 1767ns | 1767ns | -2.67% |
| satfold-neon8 | 1759ns | 1759ns | -3.15% |
| satfold-nolaw | 1681ns | 1681ns | -7.45% |
| satfold-seq | 1934ns | 1934ns | +6.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 1594ns | base | --- | [1593, 1594] | --- | --- | --- | --- |
| satfold-lanes16 | 1408ns | -16.4ns (-1.0%) | [-121, -0]ns | [1238, 1593] | YES (adj: no) | 0.1420 | 0.0533 | 1 |
| satfold-lanes16-constl | 1599ns | +93.0ns (+5.8%) | [+3, +154]ns | [1598, 1602] | YES | 0.0178 | 0.0022 | 0 |
| satfold-lanes4-idx | 1566ns | no significant difference | [-62, +2]ns | [1450, 1596] | no | 1.0000 | 1.0000 | 0 |
| satfold-lanes64 | 1451ns | no significant difference | [-130, +1]ns | [1447, 1594] | no | 0.4490 | 0.3368 | 1 |
| satfold-neon | 1532ns | -105.4ns (-6.6%) | [-145, -1]ns | [1362, 1594] | YES (adj: no) | 0.1420 | 0.0385 | 0 |
| satfold-neon8 | 1593ns | no significant difference | [-184, +78]ns | [1448, 1595] | no | 0.2462 | 0.1539 | 0 |
| satfold-nolaw | 1449ns | no significant difference | [-144, +0]ns | [1329, 1452] | no | 0.2462 | 0.1539 | 0 |
| satfold-seq | 1451ns | no significant difference | [-146, +120]ns | [1448, 1595] | no | 0.4910 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1380ns | -6.6% | +15.7% | -2.2% | -3.7% | -3.8% | -11.2% | -3.7% | +5.0% |
| 1 | 2 | 1594ns | +0.0% | +11.4% | +0.2% | +0.1% | +17.6% | +11.1% | +2.2% | +54.1% |
| 1 | 3 | 1707ns | -33.3% | -6.3% | -15.0% | -18.1% | -6.6% | -13.8% | -22.1% | -14.0% |
| 1 | 4 | 1594ns | -0.0% | +0.3% | +0.1% | +1.5% | -16.7% | +1.6% | -11.4% | -16.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.039 | ok |
| satfold-lanes16 | -0.787 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.420 | moderate- |
| satfold-lanes4-idx | -0.550 | HIGH- (thermal bounce) |
| satfold-lanes64 | -0.621 | HIGH- (thermal bounce) |
| satfold-neon | -0.299 | moderate- |
| satfold-neon8 | -0.564 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.629 | HIGH- (thermal bounce) |
| satfold-seq | -0.323 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 0/4
- **satfold-lanes16-constl**: won 1/4, lost 3/4
- **satfold-lanes4-idx**: won 2/4, lost 2/4
- **satfold-lanes64**: won 2/4, lost 1/4
- **satfold-neon**: won 3/4, lost 1/4
- **satfold-neon8**: won 2/4, lost 2/4
- **satfold-nolaw**: won 3/4, lost 1/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 84.5ns | 1573.5ns | 5.4% | HIGH |
| satfold-lanes16 | 77.4ns | 1406.7ns | 5.5% | HIGH |
| satfold-lanes16-constl | 88.1ns | 1643.2ns | 5.4% | HIGH |
| satfold-lanes4-idx | 80.5ns | 1511.3ns | 5.3% | HIGH |
| satfold-lanes64 | 80.5ns | 1515.6ns | 5.3% | HIGH |
| satfold-neon | 81.9ns | 1538.4ns | 5.3% | HIGH |
| satfold-neon8 | 82.4ns | 1529.7ns | 5.4% | HIGH |
| satfold-nolaw | 78.4ns | 1463.2ns | 5.4% | HIGH |
| satfold-seq | 90.4ns | 1682.3ns | 5.4% | HIGH |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 1369.8-1780.9 ns)
   1369.8 |#####
   1390.4 |
   1410.9 |
   1431.5 |################
   1452.0 |
   1472.6 |
   1493.1 |
   1513.7 |##
   1534.2 |
   1554.8 |
   1575.4 |########################################
   1595.9 |##
   1616.5 |##
   1637.0 |#####
   1657.6 |#####
   1678.1 |
   1698.7 |
   1719.2 |
   1739.8 |
   1760.3 |########
  (4 below, 3 above range)

satfold-lanes16 (n=40, range 1112.2-1665.9 ns)
   1112.2 |####################
   1139.9 |
   1167.6 |###
   1195.3 |
   1223.0 |################
   1250.6 |
   1278.3 |
   1306.0 |#############
   1333.7 |
   1361.4 |###
   1389.0 |
   1416.7 |
   1444.4 |###
   1472.1 |###
   1499.8 |
   1527.5 |
   1555.1 |
   1582.8 |########################################
   1610.5 |
   1638.2 |#############
  (3 below, 2 above range)

satfold-lanes16-constl (n=40, range 1560.5-1828.1 ns)
   1560.5 |
   1573.9 |
   1587.3 |########################################
   1600.7 |###
   1614.1 |
   1627.4 |#
   1640.8 |###
   1654.2 |
   1667.6 |
   1681.0 |
   1694.3 |#
   1707.7 |
   1721.1 |
   1734.5 |
   1747.9 |
   1761.2 |#
   1774.6 |#####
   1788.0 |
   1801.4 |
   1814.7 |
  (2 below, 4 above range)

satfold-lanes4-idx (n=40, range 1334.4-1643.5 ns)
   1334.4 |
   1349.8 |
   1365.3 |##
   1380.7 |##
   1396.2 |
   1411.7 |
   1427.1 |##
   1442.6 |#####################
   1458.0 |
   1473.5 |
   1488.9 |
   1504.4 |##
   1519.8 |
   1535.3 |##
   1550.8 |
   1566.2 |
   1581.7 |########################################
   1597.1 |#####
   1612.6 |
   1628.0 |##
  (7 below, 2 above range)

satfold-lanes64 (n=40, range 1319.0-1795.0 ns)
   1319.0 |########################################
   1342.8 |
   1366.6 |####
   1390.4 |
   1414.2 |########
   1438.0 |###################################
   1461.8 |####
   1485.6 |########
   1509.4 |
   1533.2 |
   1557.0 |
   1580.8 |########################################
   1604.6 |
   1628.4 |####
   1652.2 |
   1676.0 |
   1699.8 |
   1723.6 |
   1747.4 |####
   1771.2 |########
  (1 below, 3 above range)

satfold-neon (n=40, range 1272.6-1924.1 ns)
   1272.6 |
   1305.2 |########################################
   1337.8 |#####
   1370.3 |##########
   1402.9 |
   1435.5 |###############
   1468.1 |#####
   1500.6 |
   1533.2 |
   1565.8 |########################################
   1598.3 |#####
   1630.9 |##########
   1663.5 |
   1696.1 |
   1728.6 |#####
   1761.2 |###############
   1793.8 |
   1826.3 |
   1858.9 |
   1891.5 |
  (5 below, 5 above range)

satfold-neon8 (n=40, range 1181.9-1824.4 ns)
   1181.9 |
   1214.0 |#####################
   1246.2 |
   1278.3 |
   1310.4 |
   1342.5 |###
   1374.6 |
   1406.8 |
   1438.9 |##############
   1471.0 |###
   1503.1 |
   1535.3 |
   1567.4 |########################################
   1599.5 |
   1631.6 |#######
   1663.7 |###
   1695.9 |
   1728.0 |###
   1760.1 |##################
   1792.2 |
  (4 below, 4 above range)

satfold-nolaw (n=40, range 1302.5-1738.4 ns)
   1302.5 |
   1324.3 |########################################
   1346.1 |
   1367.9 |#####
   1389.7 |
   1411.5 |##
   1433.3 |######################
   1455.1 |
   1476.9 |
   1498.7 |#####
   1520.4 |
   1542.2 |
   1564.0 |
   1585.8 |##############
   1607.6 |##
   1629.4 |
   1651.2 |##
   1673.0 |
   1694.8 |##
   1716.6 |
  (2 below, 3 above range)

satfold-seq (n=40, range 1227.6-2643.8 ns)
   1227.6 |
   1298.4 |#########################
   1369.2 |
   1440.0 |########################################
   1510.8 |
   1581.6 |#################
   1652.4 |##
   1723.3 |
   1794.1 |
   1864.9 |
   1935.7 |##
   2006.5 |
   2077.3 |
   2148.1 |
   2218.9 |#####
   2289.8 |
   2360.6 |##
   2431.4 |
   2502.2 |##
   2573.0 |
  (1 below, 4 above range)

```

## Diagnostics

- **satfold-seq**: CV=32.0% (high variance, measurements may be unstable)
