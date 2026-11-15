# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon is fastest but the noisiest (CV 15.7%)

satfold-neon wins on median (1.49 us) yet has the highest variance (CV 15.7%), while satfold-iterfold is the steadiest (CV 2.4%, 1.49 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (satfold-neon, satfold-neon8) are a dead heat (<1%)

satfold-neon (1.49 us) and satfold-neon8 (1.49 us) differ by 0.05%, inside the noise, even though the wider field spreads 11.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-neon shows warm-up / thermal drift (autocorr +0.89)

satfold-neon's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (175 ns) is smaller than the fastest variant's own run-to-run std-dev (233 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader satfold-neon vs stability leader satfold-iterfold (+0% speed for 6.6x steadier)

satfold-neon is fastest (1.49 us, CV 15.7%); satfold-iterfold gives up 0.1% median for 6.6x lower variance (CV 2.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-lanes16's edge over baseline is significant but tiny (3 ns, 0.22%)

satfold-lanes16 differs from baseline satfold-iterfold by 3 ns (0.22%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-neon** at 1487.3 ns median (-0.1% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.12x (fastest 1487.3 ns, slowest 1662.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 1575ns | 1567ns | 1545ns | 1565ns | 1637ns | base |
| satfold-lanes16 | 1629ns | 1572ns | 1543ns | 1584ns | 1849ns | +3.40% |
| satfold-lanes16-constl | 1736ns | 1726ns | 1564ns | 1694ns | 2034ns | +10.21% |
| satfold-lanes4-idx | 1596ns | 1623ns | 1543ns | 1598ns | 1642ns | +1.29% |
| satfold-lanes64 | 1773ns | 1783ns | 1544ns | 1756ns | 2054ns | +12.57% |
| satfold-neon | 1741ns | 1547ns | 1541ns | 1702ns | 2057ns | +10.51% |
| satfold-neon8 | 1602ns | 1552ns | 1543ns | 1571ns | 1751ns | +1.67% |
| satfold-nolaw | 1634ns | 1626ns | 1543ns | 1624ns | 1753ns | +3.71% |
| satfold-seq | 1645ns | 1583ns | 1543ns | 1623ns | 1815ns | +4.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 1496ns | 1467ns | 1556ns | base | 21.897 |
| satfold-lanes16 | 1548ns | 1465ns | 1759ns | +3.43% | 21.171 |
| satfold-lanes16-constl | 1648ns | 1483ns | 1933ns | +10.14% | 19.880 |
| satfold-lanes4-idx | 1515ns | 1465ns | 1559ns | +1.26% | 21.624 |
| satfold-lanes64 | 1681ns | 1466ns | 1950ns | +12.36% | 19.488 |
| satfold-neon | 1673ns | 1482ns | 1977ns | +11.83% | 19.581 |
| satfold-neon8 | 1539ns | 1483ns | 1685ns | +2.84% | 21.292 |
| satfold-nolaw | 1552ns | 1466ns | 1665ns | +3.74% | 21.108 |
| satfold-seq | 1564ns | 1467ns | 1724ns | +4.50% | 20.953 |

## Performance model

- Peak throughput: **22.373 Gops/s** (satfold-lanes16; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 22.004 | 98.3% |
| satfold-lanes16 | 21.949 | 98.1% |
| satfold-lanes16-constl | 19.988 | 89.3% |
| satfold-lanes4-idx | 21.235 | 94.9% |
| satfold-lanes64 | 19.710 | 88.1% |
| satfold-neon | 22.032 | 98.5% |
| satfold-neon8 | 22.020 | 98.4% |
| satfold-nolaw | 21.212 | 94.8% |
| satfold-seq | 21.752 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 1575ns | 1575ns | base |
| satfold-lanes16 | 1629ns | 1629ns | +3.40% |
| satfold-lanes16-constl | 1736ns | 1736ns | +10.21% |
| satfold-lanes4-idx | 1596ns | 1596ns | +1.29% |
| satfold-lanes64 | 1773ns | 1773ns | +12.57% |
| satfold-neon | 1741ns | 1741ns | +10.51% |
| satfold-neon8 | 1602ns | 1602ns | +1.67% |
| satfold-nolaw | 1634ns | 1634ns | +3.71% |
| satfold-seq | 1645ns | 1645ns | +4.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 1489ns | base | --- | [1477, 1491] | --- | --- | --- | --- |
| satfold-lanes16 | 1493ns | no significant difference | [-2, +8]ns | [1480, 1515] | no | 1.0000 | 1.0000 | 1 |
| satfold-lanes16-constl | 1639ns | +93.6ns (+6.3%) | [+21, +125]ns | [1492, 1666] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1543ns | no significant difference | [-1, +39]ns | [1483, 1544] | no | 0.6965 | 0.5224 | 1 |
| satfold-lanes64 | 1662ns | no significant difference | [-1, +290]ns | [1543, 1781] | no | 0.1291 | 0.0807 | 0 |
| satfold-neon | 1487ns | +17.3ns (+1.2%) | [+9, +459]ns | [1484, 1951] | YES | 0.0129 | 0.0064 | 0 |
| satfold-neon8 | 1488ns | +17.9ns (+1.2%) | [+11, +34]ns | [1485, 1506] | YES | 0.0129 | 0.0064 | 0 |
| satfold-nolaw | 1545ns | +52.7ns (+3.5%) | [+20, +64]ns | [1491, 1548] | YES | 0.0007 | 0.0002 | 0 |
| satfold-seq | 1506ns | no significant difference | [-19, +160]ns | [1472, 1638] | no | 0.9996 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1482ns | -0.8% | +0.7% | -1.0% | +30.4% | +0.3% | +0.4% | -1.2% | -0.9% |
| 2 | 1470ns | -0.1% | +1.2% | -0.2% | +40.8% | +1.2% | +1.1% | -0.3% | +0.1% |
| 3 | 1470ns | -0.1% | +4.7% | +5.5% | +31.3% | +0.9% | +1.0% | -0.1% | -0.2% |
| 4 | 1471ns | +0.1% | +0.7% | +0.7% | +31.2% | +0.8% | +1.0% | -0.4% | -0.1% |
| 5 | 1489ns | -1.6% | +0.1% | +6.0% | +29.8% | -0.3% | -0.2% | -1.5% | -1.5% |
| 6 | 1465ns | +0.2% | +0.5% | +0.2% | +31.7% | +1.3% | +1.5% | +0.2% | +10.2% |
| 7 | 1467ns | -0.2% | +1.1% | -0.1% | +32.1% | +1.1% | +1.2% | +0.2% | +21.3% |
| 8 | 1467ns | -0.2% | +1.6% | -0.3% | +31.9% | +1.1% | +2.4% | -0.1% | +21.4% |
| 9 | 1468ns | -0.3% | +10.1% | -0.1% | +31.6% | +1.3% | +1.4% | -0.1% | +21.3% |
| 10 | 1466ns | +0.1% | +1.7% | +0.1% | +31.9% | +1.2% | +1.1% | +0.3% | +21.6% |
| 11 | 1490ns | +1.3% | +11.8% | +3.7% | -1.4% | -0.4% | +13.1% | +11.6% | +0.3% |
| 12 | 1545ns | -3.7% | +7.6% | +0.0% | +0.2% | -3.9% | +9.0% | +7.7% | -3.0% |
| 13 | 1545ns | +13.5% | +7.6% | +0.0% | -3.4% | -3.8% | +9.0% | +7.7% | -2.1% |
| 14 | 1638ns | -8.7% | +1.7% | -5.8% | -5.5% | -9.4% | +2.6% | +1.6% | -9.0% |
| 15 | 1545ns | -2.1% | +7.8% | -0.1% | -0.3% | -4.0% | +9.1% | +7.7% | -1.9% |
| 16 | 1547ns | -3.4% | +8.7% | -0.2% | -0.2% | -3.8% | +9.0% | +7.7% | -3.1% |
| 17 | 1542ns | +0.7% | +7.9% | +0.1% | +0.0% | -3.9% | +9.0% | +8.0% | +1.3% |
| 18 | 1540ns | -1.3% | +8.2% | +0.3% | +0.5% | -3.7% | +9.4% | +8.2% | -3.1% |
| 19 | 1542ns | -3.4% | +8.0% | +0.1% | +0.2% | -3.7% | +9.3% | +7.9% | -1.0% |
| 20 | 1545ns | -3.7% | +8.0% | +4.6% | -4.6% | -4.1% | +8.9% | +7.7% | +0.0% |
| 21 | 1489ns | +0.1% | +29.8% | -0.0% | +19.4% | +31.3% | -0.4% | +3.9% | -1.5% |
| 22 | 1491ns | -0.1% | +29.6% | -0.4% | +19.6% | +31.0% | -0.5% | +3.9% | -1.7% |
| 23 | 1492ns | +0.3% | +29.6% | -0.3% | +19.3% | +31.2% | -0.4% | +3.6% | -1.7% |
| 24 | 1491ns | +14.1% | +29.7% | -1.6% | +19.4% | +31.1% | -0.6% | +3.6% | -1.3% |
| 25 | 1494ns | +8.9% | +29.4% | +1.9% | +19.2% | +30.8% | -0.7% | +9.9% | -1.8% |
| 26 | 1492ns | +0.4% | +29.5% | -1.5% | +19.2% | +30.7% | -0.3% | +3.7% | -1.6% |
| 27 | 1490ns | -1.8% | +29.6% | -1.7% | +19.5% | +31.0% | -0.4% | +10.3% | -1.2% |
| 28 | 1487ns | +7.0% | +29.7% | -1.4% | +19.7% | +31.5% | -0.1% | +4.0% | -1.3% |
| 29 | 1491ns | -0.1% | +29.7% | -1.7% | +19.4% | +31.0% | -0.6% | +3.5% | -1.5% |
| 30 | 1476ns | +2.8% | +30.9% | -0.5% | +20.4% | +32.5% | +0.6% | +4.7% | -0.6% |
| 31 | 1471ns | +21.2% | +0.3% | +5.1% | -0.2% | +33.0% | +2.3% | +1.4% | +13.3% |
| 32 | 1478ns | +20.5% | +1.0% | +4.5% | -0.7% | +32.2% | +1.9% | +0.7% | +12.7% |
| 33 | 1490ns | +19.1% | +0.1% | +3.7% | -1.7% | +31.2% | +1.2% | -0.1% | +11.9% |
| 34 | 1482ns | +20.0% | +0.8% | +4.4% | -0.8% | +43.5% | +1.9% | +0.7% | +12.1% |
| 35 | 1468ns | +21.3% | +1.6% | +5.1% | -0.3% | +33.1% | +2.6% | +1.5% | +13.4% |
| 36 | 1471ns | +16.7% | +1.3% | +4.8% | -0.2% | +21.8% | +2.4% | +1.4% | +13.4% |
| 37 | 1470ns | +5.9% | +1.4% | +5.1% | -0.5% | +0.7% | +2.5% | +1.3% | +13.3% |
| 38 | 1482ns | -1.2% | +0.4% | +4.2% | +4.4% | +0.2% | +0.5% | +8.8% | +12.5% |
| 39 | 1495ns | -1.9% | -0.1% | +3.3% | -1.8% | +0.4% | -0.6% | +3.3% | +11.4% |
| 40 | 1468ns | +0.0% | +1.5% | +5.2% | +0.1% | +2.8% | +0.9% | +5.3% | +13.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.720 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.640 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.846 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.420 | moderate+ |
| satfold-lanes64 | 0.806 | HIGH+ (drift/warm-up) |
| satfold-neon | 0.892 | HIGH+ (drift/warm-up) |
| satfold-neon8 | 0.844 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.759 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.774 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-lanes16**: won 17/40, lost 18/40
- **satfold-lanes16-constl**: won 0/40, lost 37/40
- **satfold-lanes4-idx**: won 13/40, lost 21/40
- **satfold-lanes64**: won 14/40, lost 25/40
- **satfold-neon**: won 11/40, lost 29/40
- **satfold-neon8**: won 11/40, lost 29/40
- **satfold-nolaw**: won 5/40, lost 32/40
- **satfold-seq**: won 21/40, lost 18/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.4ns | 1496.5ns | 0.2% |  |
| satfold-lanes16 | 2.7ns | 1547.8ns | 0.2% |  |
| satfold-lanes16-constl | 2.8ns | 1648.3ns | 0.2% |  |
| satfold-lanes4-idx | 2.6ns | 1515.4ns | 0.2% |  |
| satfold-lanes64 | 3.2ns | 1681.4ns | 0.2% |  |
| satfold-neon | 2.3ns | 1673.5ns | 0.1% |  |
| satfold-neon8 | 1.9ns | 1539.0ns | 0.1% |  |
| satfold-nolaw | 2.3ns | 1552.4ns | 0.1% |  |
| satfold-seq | 2.2ns | 1563.9ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 1467.2-1556.1 ns)
   1467.2 |########################################
   1471.6 |
   1476.1 |########
   1480.5 |#############
   1485.0 |#############
   1489.4 |###################################
   1493.9 |########
   1498.3 |
   1502.8 |
   1507.2 |
   1511.6 |
   1516.1 |
   1520.5 |
   1525.0 |
   1529.4 |
   1533.9 |
   1538.3 |#############
   1542.8 |######################
   1547.2 |
   1551.7 |
  (4 below, 1 above range)

satfold-lanes16 (n=40, range 1464.6-1758.7 ns)
   1464.6 |########################################
   1479.3 |##########################
   1494.0 |#################
   1508.7 |#################
   1523.4 |
   1538.1 |####
   1552.8 |####
   1567.5 |
   1582.2 |####
   1596.9 |
   1611.6 |
   1626.3 |####
   1641.1 |
   1655.8 |
   1670.5 |
   1685.2 |
   1699.9 |####
   1714.6 |####
   1729.3 |
   1744.0 |####
  (5 below, 5 above range)

satfold-lanes16-constl (n=40, range 1483.3-1933.4 ns)
   1483.3 |########################################
   1505.8 |
   1528.3 |##
   1550.8 |
   1573.4 |
   1595.9 |##
   1618.4 |
   1640.9 |#####
   1663.4 |######################
   1685.9 |
   1708.4 |
   1730.9 |
   1753.4 |
   1775.9 |
   1798.4 |
   1820.9 |
   1843.4 |
   1865.9 |
   1888.4 |
   1910.9 |#################
  (4 below, 4 above range)

satfold-lanes4-idx (n=40, range 1465.4-1559.2 ns)
   1465.4 |#############################
   1470.1 |
   1474.7 |
   1479.4 |##
   1484.1 |########
   1488.8 |
   1493.5 |
   1498.2 |
   1502.9 |
   1507.6 |
   1512.3 |
   1517.0 |
   1521.7 |##
   1526.4 |
   1531.1 |
   1535.8 |
   1540.4 |########################################
   1545.1 |##########
   1549.8 |##
   1554.5 |
  (2 below, 2 above range)

satfold-lanes64 (n=40, range 1466.4-1950.3 ns)
   1466.4 |###################################
   1490.6 |####
   1514.8 |
   1539.0 |###################################
   1563.2 |
   1587.4 |
   1611.6 |
   1635.8 |
   1660.0 |
   1684.2 |
   1708.4 |
   1732.6 |
   1756.8 |###############################
   1780.9 |#############
   1805.1 |
   1829.3 |
   1853.5 |
   1877.7 |
   1901.9 |
   1926.1 |########################################
  (3 below, 1 above range)

satfold-neon (n=40, range 1482.1-1977.3 ns)
   1482.1 |########################################
   1506.9 |##
   1531.7 |
   1556.4 |
   1581.2 |
   1605.9 |
   1630.7 |
   1655.4 |
   1680.2 |
   1704.9 |
   1729.7 |
   1754.5 |
   1779.2 |##
   1804.0 |
   1828.7 |
   1853.5 |
   1878.2 |
   1903.0 |
   1927.8 |####
   1952.5 |########################
  (3 below, 1 above range)

satfold-neon8 (n=40, range 1483.0-1684.6 ns)
   1483.0 |########################################
   1493.1 |##
   1503.1 |###############
   1513.2 |
   1523.3 |
   1533.4 |
   1543.5 |
   1553.5 |
   1563.6 |
   1573.7 |
   1583.8 |
   1593.9 |
   1603.9 |
   1614.0 |
   1624.1 |
   1634.2 |
   1644.2 |
   1654.3 |
   1664.4 |
   1674.5 |###########
  (4 below, 5 above range)

satfold-nolaw (n=40, range 1466.3-1664.6 ns)
   1466.3 |############################
   1476.3 |
   1486.2 |########################################
   1496.1 |
   1506.0 |
   1515.9 |
   1525.8 |
   1535.7 |############################
   1545.7 |############################
   1555.6 |
   1565.5 |
   1575.4 |
   1585.3 |
   1595.2 |
   1605.1 |#####
   1615.1 |
   1625.0 |
   1634.9 |###########
   1644.8 |
   1654.7 |########################################
  (5 below, 3 above range)

satfold-seq (n=40, range 1467.0-1724.2 ns)
   1467.0 |########################################
   1479.8 |######
   1492.7 |##########
   1505.6 |######
   1518.4 |###
   1531.3 |
   1544.2 |###
   1557.0 |###
   1569.9 |
   1582.7 |
   1595.6 |
   1608.5 |###
   1621.3 |
   1634.2 |
   1647.0 |
   1659.9 |#################################
   1672.8 |
   1685.6 |
   1698.5 |
   1711.3 |
  (3 below, 4 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.72 (measurement drift or warm-up artifact)
- **satfold-lanes16**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **satfold-lanes64**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **satfold-nolaw**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.77 (measurement drift or warm-up artifact)
