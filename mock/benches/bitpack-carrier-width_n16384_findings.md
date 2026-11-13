# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d64 is fastest but the noisiest (CV 16.5%)

bitpack-carrier-d64 wins on median (1.32 us) yet has the highest variance (CV 16.5%), while bitpack-carrier-d32 is the steadiest (CV 4.0%, 1.36 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-carrier-d16 shows warm-up / thermal drift (autocorr +0.85)

bitpack-carrier-d16's per-pass series has lag-1 autocorrelation +0.85, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-carrier-d64, bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control} vs {bitpack-carrier-packed-simd, bitpack-carrier-packed} (41% apart)

The field splits into a fast tier {bitpack-carrier-d64, bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control} and a slow tier {bitpack-carrier-packed-simd, bitpack-carrier-packed} with a 41% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader bitpack-carrier-d64 vs stability leader bitpack-carrier-d32 (+3% speed for 4.1x steadier)

bitpack-carrier-d64 is fastest (1.32 us, CV 16.5%); bitpack-carrier-d32 gives up 3.0% median for 4.1x lower variance (CV 4.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bitpack-carrier-d16-control's edge over baseline is significant but tiny (23 ns, 1.67%)

bitpack-carrier-d16-control differs from baseline bitpack-carrier-d16 by 23 ns (1.67%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bitpack-carrier-d64** at 1316.9 ns median (-4.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.60x (fastest 1316.9 ns, slowest 2102.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1547ns | 1446ns | 1432ns | 1466ns | 1907ns | base |
| bitpack-carrier-d16-control | 1473ns | 1456ns | 1432ns | 1454ns | 1571ns | -4.78% |
| bitpack-carrier-d32 | 1439ns | 1417ns | 1412ns | 1421ns | 1517ns | -7.03% |
| bitpack-carrier-d64 | 1567ns | 1407ns | 1376ns | 1505ns | 1946ns | +1.28% |
| bitpack-carrier-packed | 2206ns | 2165ns | 2110ns | 2166ns | 2424ns | +42.61% |
| bitpack-carrier-packed-simd | 2158ns | 2044ns | 1983ns | 2087ns | 2547ns | +39.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1481ns | 1373ns | 1824ns | base | 11.061 |
| bitpack-carrier-d16-control | 1410ns | 1372ns | 1502ns | -4.79% | 11.617 |
| bitpack-carrier-d32 | 1376ns | 1353ns | 1448ns | -7.10% | 11.907 |
| bitpack-carrier-d64 | 1469ns | 1292ns | 1824ns | -0.83% | 11.154 |
| bitpack-carrier-packed | 2143ns | 2049ns | 2355ns | +44.69% | 7.645 |
| bitpack-carrier-packed-simd | 2077ns | 1911ns | 2453ns | +40.23% | 7.888 |

## Performance model

- Peak throughput: **12.685 Gops/s** (bitpack-carrier-d64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 11.839 | 93.3% |
| bitpack-carrier-d16-control | 11.738 | 92.5% |
| bitpack-carrier-d32 | 12.084 | 95.3% |
| bitpack-carrier-d64 | 12.441 | 98.1% |
| bitpack-carrier-packed | 7.793 | 61.4% |
| bitpack-carrier-packed-simd | 8.333 | 65.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 1547ns | 1547ns | base |
| bitpack-carrier-d16-control | 1473ns | 1473ns | -4.78% |
| bitpack-carrier-d32 | 1439ns | 1439ns | -7.03% |
| bitpack-carrier-d64 | 1567ns | 1567ns | +1.28% |
| bitpack-carrier-packed | 2206ns | 2206ns | +42.61% |
| bitpack-carrier-packed-simd | 2158ns | 2158ns | +39.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1384ns | base | --- | [1375, 1396] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 1396ns | no significant difference | [-20, +17]ns | [1376, 1402] | no | 0.6358 | 0.6358 | 0 |
| bitpack-carrier-d32 | 1356ns | -20.2ns (-1.5%) | [-42, -16]ns | [1354, 1360] | YES | 0.0107 | 0.0064 | 0 |
| bitpack-carrier-d64 | 1317ns | -72.3ns (-5.2%) | [-80, -54]ns | [1314, 1541] | YES | 0.0207 | 0.0166 | 0 |
| bitpack-carrier-packed | 2102ns | +686.0ns (+49.6%) | [+670, +699]ns | [2081, 2118] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 1966ns | +572.3ns (+41.4%) | [+554, +665]ns | [1932, 2051] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 1372ns | +1.7% | -1.3% | -5.6% | +48.9% | +85.0% |
| 2 | 1374ns | +4.2% | +0.2% | -5.5% | +48.6% | +84.4% |
| 3 | 1372ns | +1.8% | -0.8% | -6.1% | +48.8% | +84.8% |
| 4 | 1376ns | +2.9% | -0.2% | -6.0% | +49.9% | +84.3% |
| 5 | 1373ns | +1.8% | +0.9% | -5.8% | +48.7% | +84.5% |
| 6 | 1375ns | +1.6% | +1.1% | -5.9% | +48.6% | +84.4% |
| 7 | 1373ns | +2.9% | +0.1% | -5.9% | +48.7% | +43.2% |
| 8 | 1372ns | +1.7% | +0.2% | -6.1% | +53.7% | +37.6% |
| 9 | 1374ns | +3.4% | +0.1% | -6.2% | +54.7% | +37.5% |
| 10 | 1372ns | +3.8% | +0.3% | -4.3% | +51.7% | +37.7% |
| 11 | 1808ns | -18.4% | -9.3% | +4.4% | +15.1% | +6.5% |
| 12 | 1809ns | -22.7% | -25.1% | +4.7% | +14.8% | +6.5% |
| 13 | 1809ns | -21.5% | -24.9% | +4.7% | +21.4% | +6.5% |
| 14 | 1873ns | -25.3% | -27.6% | +1.1% | +25.9% | +2.7% |
| 15 | 1873ns | -25.4% | -27.7% | -6.0% | +26.2% | +2.9% |
| 16 | 1802ns | -20.1% | -24.9% | -3.9% | +30.9% | +6.9% |
| 17 | 1809ns | -22.2% | -25.1% | -1.4% | +37.6% | +6.3% |
| 18 | 1809ns | -23.9% | -25.1% | -4.2% | +30.4% | +6.5% |
| 19 | 1719ns | -19.9% | -21.4% | +1.4% | +36.9% | +12.0% |
| 20 | 1375ns | +0.2% | -1.5% | +26.3% | +71.8% | +39.9% |
| 21 | 1374ns | -0.2% | +0.8% | -4.3% | +53.8% | +59.1% |
| 22 | 1374ns | +0.1% | +1.7% | -4.0% | +54.0% | +58.9% |
| 23 | 1375ns | -0.2% | -1.3% | -4.2% | +54.1% | +58.8% |
| 24 | 1375ns | -0.1% | -1.3% | -4.2% | +56.6% | +58.5% |
| 25 | 1373ns | +0.1% | -1.1% | -3.9% | +53.9% | +59.0% |
| 26 | 1373ns | -0.0% | -1.4% | +8.3% | +55.4% | +58.7% |
| 27 | 1511ns | -9.3% | -10.3% | +5.9% | +42.4% | +47.4% |
| 28 | 1374ns | -0.0% | -1.5% | +16.3% | +55.1% | +41.0% |
| 29 | 1375ns | -0.2% | -1.5% | +15.9% | +53.6% | +39.8% |
| 30 | 1375ns | -0.2% | -1.6% | +16.4% | +53.6% | +45.8% |
| 31 | 1395ns | +7.1% | -3.0% | -6.0% | +49.1% | +40.2% |
| 32 | 1396ns | -1.6% | -3.0% | -5.8% | +50.1% | +41.3% |
| 33 | 1392ns | -1.3% | -2.6% | -5.6% | +49.5% | +43.0% |
| 34 | 1397ns | -1.5% | -3.1% | +4.4% | +48.9% | +41.2% |
| 35 | 1397ns | -1.5% | -3.1% | -5.8% | +48.9% | +40.0% |
| 36 | 1394ns | +0.8% | -2.9% | -5.8% | +49.3% | +40.3% |
| 37 | 1396ns | -1.5% | -3.1% | -5.9% | +49.6% | +40.8% |
| 38 | 1394ns | +2.1% | -2.9% | -5.7% | +50.2% | +40.6% |
| 39 | 1394ns | +19.5% | +5.8% | -5.9% | +49.3% | +41.8% |
| 40 | 1395ns | +19.4% | +10.1% | -5.6% | +49.1% | +50.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | 0.854 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d16-control | 0.479 | moderate+ |
| bitpack-carrier-d32 | 0.155 | ok |
| bitpack-carrier-d64 | 0.801 | HIGH+ (drift/warm-up) |
| bitpack-carrier-packed | 0.843 | HIGH+ (drift/warm-up) |
| bitpack-carrier-packed-simd | 0.805 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 19/40, lost 16/40
- **bitpack-carrier-d32**: won 29/40, lost 9/40
- **bitpack-carrier-d64**: won 28/40, lost 12/40
- **bitpack-carrier-packed**: won 0/40, lost 40/40
- **bitpack-carrier-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 2.1ns | 1481.2ns | 0.1% |  |
| bitpack-carrier-d16-control | 1.9ns | 1410.3ns | 0.1% |  |
| bitpack-carrier-d32 | 2.0ns | 1376.0ns | 0.1% |  |
| bitpack-carrier-d64 | 2.2ns | 1468.9ns | 0.2% |  |
| bitpack-carrier-packed | 1.9ns | 2143.2ns | 0.1% |  |
| bitpack-carrier-packed-simd | 2.3ns | 2077.1ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 1372.6-1824.0 ns)
   1372.6 |########################################
   1395.2 |############
   1417.8 |
   1440.3 |
   1462.9 |
   1485.5 |
   1508.0 |##
   1530.6 |
   1553.2 |
   1575.7 |
   1598.3 |
   1620.9 |
   1643.4 |
   1666.0 |
   1688.6 |
   1711.1 |##
   1733.7 |
   1756.3 |
   1778.8 |
   1801.4 |############
  (4 below, 2 above range)

bitpack-carrier-d16-control (n=40, range 1372.4-1502.4 ns)
   1372.4 |########################################
   1378.9 |
   1385.4 |
   1391.9 |################
   1398.4 |#####
   1404.9 |#####
   1411.4 |#####
   1417.9 |########
   1424.4 |#####
   1430.9 |
   1437.4 |##
   1443.9 |
   1450.4 |
   1456.9 |
   1463.4 |
   1469.9 |##
   1476.4 |
   1482.9 |
   1489.4 |##
   1495.9 |
  (3 below, 2 above range)

bitpack-carrier-d32 (n=40, range 1353.0-1448.1 ns)
   1353.0 |########################################
   1357.8 |#####
   1362.5 |
   1367.3 |
   1372.0 |#########
   1376.8 |#
   1381.5 |###
   1386.3 |#
   1391.0 |
   1395.8 |#
   1400.5 |
   1405.3 |
   1410.0 |
   1414.8 |
   1419.6 |
   1424.3 |
   1429.1 |
   1433.8 |
   1438.6 |
   1443.3 |
  (3 below, 3 above range)

bitpack-carrier-d64 (n=40, range 1291.6-1824.4 ns)
   1291.6 |########################################
   1318.2 |####
   1344.9 |
   1371.5 |
   1398.1 |
   1424.8 |
   1451.4 |##
   1478.0 |##
   1504.7 |
   1531.3 |
   1558.0 |
   1584.6 |########
   1611.2 |
   1637.9 |
   1664.5 |
   1691.2 |
   1717.8 |########
   1744.4 |##
   1771.1 |##
   1797.7 |
  (3 below, 4 above range)

bitpack-carrier-packed (n=40, range 2049.0-2355.1 ns)
   2049.0 |###
   2064.3 |###
   2079.6 |########################################
   2094.9 |#######
   2110.2 |#########################
   2125.5 |#######
   2140.8 |#######
   2156.1 |
   2171.4 |
   2186.7 |###
   2202.0 |
   2217.3 |
   2232.6 |
   2247.9 |
   2263.2 |
   2278.5 |
   2293.8 |
   2309.1 |
   2324.4 |
   2339.7 |###
  (6 below, 6 above range)

bitpack-carrier-packed-simd (n=40, range 1910.6-2452.7 ns)
   1910.6 |########################################
   1937.7 |#############
   1964.8 |####################
   1991.9 |###
   2019.0 |
   2046.1 |
   2073.2 |###
   2100.3 |
   2127.4 |
   2154.6 |######
   2181.7 |#############
   2208.8 |###
   2235.9 |
   2263.0 |
   2290.1 |
   2317.2 |
   2344.3 |
   2371.4 |
   2398.5 |
   2425.6 |
  (3 below, 6 above range)

```

## Diagnostics

- **bitpack-carrier-d16**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **bitpack-carrier-d64**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **bitpack-carrier-packed**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **bitpack-carrier-packed-simd**: autocorrelation=0.81 (measurement drift or warm-up artifact)
