# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d64 shows warm-up / thermal drift (autocorr +0.88)

bitpack-contend-d64's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-contend-d64** at 1442.3 ns median (-3.7% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.52x (fastest 1442.3 ns, slowest 2191.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1573ns | 1566ns | 1434ns | 1562ns | 1744ns | base |
| bitpack-contend-d16-control | 1659ns | 1689ns | 1519ns | 1677ns | 1745ns | +5.50% |
| bitpack-contend-d32 | 1573ns | 1538ns | 1389ns | 1533ns | 1878ns | +0.03% |
| bitpack-contend-d64 | 1565ns | 1538ns | 1471ns | 1546ns | 1714ns | -0.49% |
| bitpack-contend-packed | 2317ns | 2255ns | 2149ns | 2288ns | 2574ns | +47.36% |
| bitpack-contend-packed-simd | 2190ns | 2094ns | 2021ns | 2107ns | 2611ns | +39.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 1505ns | 1370ns | 1671ns | base | 10.889 |
| bitpack-contend-d16-control | 1589ns | 1450ns | 1672ns | +5.59% | 10.313 |
| bitpack-contend-d32 | 1504ns | 1330ns | 1797ns | -0.04% | 10.893 |
| bitpack-contend-d64 | 1464ns | 1377ns | 1596ns | -2.67% | 11.188 |
| bitpack-contend-packed | 2250ns | 2083ns | 2503ns | +49.54% | 7.281 |
| bitpack-contend-packed-simd | 2110ns | 1946ns | 2517ns | +40.22% | 7.766 |

## Performance model

- Peak throughput: **12.319 Gops/s** (bitpack-contend-d32; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 10.941 | 88.8% |
| bitpack-contend-d16-control | 10.122 | 82.2% |
| bitpack-contend-d32 | 11.151 | 90.5% |
| bitpack-contend-d64 | 11.360 | 92.2% |
| bitpack-contend-packed | 7.475 | 60.7% |
| bitpack-contend-packed-simd | 8.136 | 66.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 1573ns | 1573ns | base |
| bitpack-contend-d16-control | 1659ns | 1659ns | +5.50% |
| bitpack-contend-d32 | 1573ns | 1573ns | +0.03% |
| bitpack-contend-d64 | 1565ns | 1565ns | -0.49% |
| bitpack-contend-packed | 2317ns | 2317ns | +47.36% |
| bitpack-contend-packed-simd | 2190ns | 2190ns | +39.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1498ns | base | --- | [1407, 1561] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 1619ns | +100.8ns (+6.7%) | [+45, +110]ns | [1562, 1669] | YES | 0.0000 | 0.0000 | 2 |
| bitpack-contend-d32 | 1469ns | no significant difference | [-25, +141]ns | [1381, 1538] | no | 0.8746 | 0.8746 | 0 |
| bitpack-contend-d64 | 1442ns | -111.9ns (-7.5%) | [-175, -16]ns | [1388, 1494] | YES | 0.0207 | 0.0166 | 0 |
| bitpack-contend-packed | 2192ns | +692.7ns (+46.3%) | [+615, +748]ns | [2152, 2305] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 2014ns | +610.4ns (+40.8%) | [+574, +646]ns | [1977, 2029] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 1673ns | -0.2% | -20.5% | -11.0% | +41.2% | +17.1% |
| 2 | 1672ns | -0.1% | -20.5% | -10.6% | +41.4% | +17.2% |
| 3 | 1670ns | +0.1% | -20.3% | -10.7% | +41.6% | +17.2% |
| 4 | 1670ns | +0.0% | -20.2% | -10.5% | +41.5% | +17.3% |
| 5 | 1671ns | +0.0% | -20.4% | -10.6% | +34.6% | +17.3% |
| 6 | 1670ns | +0.1% | -20.4% | -10.6% | +31.3% | +17.3% |
| 7 | 1668ns | +0.0% | -20.2% | -10.5% | +31.4% | +17.4% |
| 8 | 1671ns | -0.1% | -20.3% | -10.6% | +31.2% | +17.2% |
| 9 | 1670ns | +0.0% | -20.5% | -10.6% | +31.4% | +17.3% |
| 10 | 1669ns | -0.0% | -20.2% | -10.5% | +31.4% | +17.3% |
| 11 | 1405ns | +3.3% | -1.1% | -0.8% | +48.0% | +42.4% |
| 12 | 1418ns | +2.0% | -4.9% | -1.6% | +52.7% | +35.6% |
| 13 | 1409ns | +3.1% | -3.0% | -1.6% | +51.2% | +37.5% |
| 14 | 1395ns | +3.9% | -0.3% | -0.7% | +53.6% | +43.3% |
| 15 | 1352ns | +7.1% | +3.8% | +2.2% | +57.3% | +47.0% |
| 16 | 1385ns | +4.8% | -0.2% | +0.6% | +52.4% | +43.1% |
| 17 | 1421ns | +2.1% | -3.0% | -2.4% | +50.5% | +40.4% |
| 18 | 1437ns | +0.9% | -2.4% | -3.1% | +44.4% | +33.2% |
| 19 | 1393ns | +4.1% | -1.0% | -4.0% | +56.3% | +42.0% |
| 20 | 1404ns | +3.8% | -1.0% | -2.0% | +54.6% | +40.8% |
| 21 | 1561ns | +6.9% | +20.3% | -10.8% | +44.1% | +62.7% |
| 22 | 1558ns | +7.3% | +14.5% | -11.0% | +40.6% | +62.9% |
| 23 | 1561ns | +6.9% | +14.6% | -11.3% | +32.8% | +62.8% |
| 24 | 1565ns | +6.8% | +14.1% | -11.4% | +38.1% | +62.2% |
| 25 | 1560ns | +7.0% | +14.4% | -11.2% | +36.7% | +62.9% |
| 26 | 1559ns | +7.3% | +14.5% | -11.3% | +36.3% | +62.9% |
| 27 | 1560ns | +7.2% | +14.4% | -11.1% | +37.3% | +62.8% |
| 28 | 1560ns | +7.1% | +14.3% | -11.2% | +31.6% | +51.2% |
| 29 | 1562ns | +7.0% | +5.1% | -11.3% | +31.0% | +49.7% |
| 30 | 1561ns | +6.9% | +5.1% | -11.2% | +34.8% | +50.2% |
| 31 | 1376ns | +13.4% | +11.6% | +15.8% | +83.6% | +47.6% |
| 32 | 1395ns | +11.8% | +10.2% | +14.2% | +81.5% | +45.4% |
| 33 | 1420ns | +10.2% | +8.3% | +12.3% | +78.3% | +42.8% |
| 34 | 1396ns | +12.1% | +10.1% | +14.3% | +81.2% | +45.3% |
| 35 | 1400ns | +11.5% | +9.8% | +13.9% | +80.2% | +45.2% |
| 36 | 1416ns | +10.3% | +8.4% | +12.8% | +78.8% | +43.3% |
| 37 | 1350ns | +16.2% | +13.9% | +18.2% | +83.9% | +50.3% |
| 38 | 1350ns | +15.9% | +13.8% | +18.0% | +74.8% | +50.2% |
| 39 | 1396ns | +12.1% | +10.4% | +14.4% | +69.2% | +45.3% |
| 40 | 1358ns | +15.1% | +13.3% | +17.8% | +73.9% | +49.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.827 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.832 | HIGH+ (drift/warm-up) |
| bitpack-contend-d32 | 0.867 | HIGH+ (drift/warm-up) |
| bitpack-contend-d64 | 0.876 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed | 0.834 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.873 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 2/40, lost 31/40
- **bitpack-contend-d32**: won 19/40, lost 21/40
- **bitpack-contend-d64**: won 28/40, lost 12/40
- **bitpack-contend-packed**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 2.1ns | 1504.7ns | 0.1% |  |
| bitpack-contend-d16-control | 2.5ns | 1588.7ns | 0.2% |  |
| bitpack-contend-d32 | 2.4ns | 1504.1ns | 0.2% |  |
| bitpack-contend-d64 | 2.3ns | 1464.4ns | 0.2% |  |
| bitpack-contend-packed | 2.2ns | 2250.1ns | 0.1% |  |
| bitpack-contend-packed-simd | 2.3ns | 2109.8ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1369.7-1670.8 ns)
   1369.7 |####
   1384.8 |########################
   1399.8 |################
   1414.9 |################
   1429.9 |####
   1445.0 |
   1460.0 |
   1475.1 |
   1490.2 |
   1505.2 |
   1520.3 |
   1535.3 |
   1550.4 |########################################
   1565.4 |
   1580.5 |
   1595.5 |
   1610.6 |
   1625.6 |
   1640.7 |
   1655.7 |########################
  (4 below, 4 above range)

bitpack-contend-d16-control (n=40, range 1449.6-1671.8 ns)
   1449.6 |#################
   1460.7 |
   1471.8 |
   1482.9 |
   1494.0 |
   1505.1 |
   1516.2 |
   1527.4 |
   1538.5 |
   1549.6 |##
   1560.7 |######################
   1571.8 |
   1582.9 |
   1594.0 |
   1605.2 |
   1616.3 |
   1627.4 |
   1638.5 |
   1649.6 |
   1660.7 |########################################
  (3 below, 4 above range)

bitpack-contend-d32 (n=40, range 1330.0-1797.1 ns)
   1330.0 |##########################
   1353.4 |####
   1376.7 |##########################
   1400.1 |########
   1423.4 |
   1446.8 |
   1470.1 |
   1493.5 |
   1516.8 |########################################
   1540.2 |####
   1563.5 |
   1586.9 |
   1610.2 |
   1633.6 |########
   1656.9 |
   1680.3 |
   1703.6 |
   1727.0 |
   1750.4 |
   1773.7 |###############################
  (5 below, 1 above range)

bitpack-contend-d64 (n=40, range 1377.4-1596.0 ns)
   1377.4 |########################################
   1388.3 |###############
   1399.3 |
   1410.2 |
   1421.1 |
   1432.1 |
   1443.0 |
   1453.9 |
   1464.8 |
   1475.8 |
   1486.7 |##############################
   1497.6 |
   1508.6 |
   1519.5 |
   1530.4 |
   1541.4 |
   1552.3 |
   1563.2 |
   1574.2 |
   1585.1 |#####################
  (2 below, 3 above range)

bitpack-contend-packed (n=40, range 2083.3-2503.0 ns)
   2083.3 |#####
   2104.3 |#####
   2125.2 |########################################
   2146.2 |###########
   2167.2 |###########
   2188.2 |##################################
   2209.2 |
   2230.2 |###########
   2251.2 |
   2272.2 |
   2293.1 |
   2314.1 |
   2335.1 |
   2356.1 |########################################
   2377.1 |
   2398.1 |
   2419.1 |
   2440.1 |
   2461.1 |
   2482.0 |#####
  (5 below, 6 above range)

bitpack-contend-packed-simd (n=40, range 1945.6-2517.0 ns)
   1945.6 |########################################
   1974.2 |############################
   2002.8 |####################################
   2031.3 |####
   2059.9 |
   2088.5 |
   2117.0 |
   2145.6 |
   2174.2 |
   2202.8 |
   2231.3 |
   2259.9 |
   2288.5 |
   2317.0 |########
   2345.6 |####
   2374.2 |
   2402.8 |
   2431.3 |
   2459.9 |
   2488.5 |
  (3 below, 7 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-control**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **bitpack-contend-d32**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **bitpack-contend-d64**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **bitpack-contend-packed**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **bitpack-contend-packed-simd**: autocorrelation=0.87 (measurement drift or warm-up artifact)
