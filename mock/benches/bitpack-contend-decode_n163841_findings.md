# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed-simd shows warm-up / thermal drift (autocorr +0.82)

bitpack-contend-packed-simd's per-pass series has lag-1 autocorrelation +0.82, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-pipe4, bitpack-contend-pipe2, bitpack-contend-d16-control, bitpack-contend-d16} vs {bitpack-contend-packed-simd} (37% apart)

The field splits into a fast tier {bitpack-contend-pipe4, bitpack-contend-pipe2, bitpack-contend-d16-control, bitpack-contend-d16} and a slow tier {bitpack-contend-packed-simd} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-contend-d16-control's comparison is tie-heavy (10% tied pairs)

10% of paired samples for bitpack-contend-d16-control are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### bitpack-contend-d16-control's edge over baseline is significant but tiny (-16 ns, 1.16%)

bitpack-contend-d16-control differs from baseline bitpack-contend-d16 by -16 ns (1.16%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bitpack-contend-pipe4** at 1151.5 ns median (-18.2% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.68x (fastest 1151.5 ns, slowest 1931.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1550ns | 1469ns | 1436ns | 1499ns | 1818ns | base |
| bitpack-contend-d16-control | 1452ns | 1439ns | 1435ns | 1444ns | 1492ns | -6.34% |
| bitpack-contend-packed-simd | 2117ns | 2006ns | 1999ns | 2024ns | 2512ns | +36.56% |
| bitpack-contend-pipe2 | 1330ns | 1278ns | 1210ns | 1290ns | 1570ns | -14.22% |
| bitpack-contend-pipe4 | 1273ns | 1214ns | 1209ns | 1253ns | 1398ns | -17.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 1483ns | 1376ns | 1733ns | base | 11.046 |
| bitpack-contend-d16-control | 1391ns | 1375ns | 1430ns | -6.25% | 11.782 |
| bitpack-contend-packed-simd | 2040ns | 1926ns | 2420ns | +37.52% | 8.032 |
| bitpack-contend-pipe2 | 1261ns | 1148ns | 1489ns | -15.01% | 12.997 |
| bitpack-contend-pipe4 | 1209ns | 1149ns | 1327ns | -18.50% | 13.553 |

## Performance model

- Peak throughput: **14.268 Gops/s** (bitpack-contend-pipe2; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 11.634 | 81.5% |
| bitpack-contend-d16-control | 11.892 | 83.3% |
| bitpack-contend-packed-simd | 8.484 | 59.5% |
| bitpack-contend-pipe2 | 13.532 | 94.8% |
| bitpack-contend-pipe4 | 14.229 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 1550ns | 1550ns | base |
| bitpack-contend-d16-control | 1452ns | 1452ns | -6.34% |
| bitpack-contend-packed-simd | 2117ns | 2117ns | +36.56% |
| bitpack-contend-pipe2 | 1330ns | 1330ns | -14.22% |
| bitpack-contend-pipe4 | 1273ns | 1273ns | -17.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1408ns | base | --- | [1390, 1444] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 1378ns | no significant difference | [-63, +0]ns | [1377, 1384] | no | 0.1081 | 0.1081 | 1 |
| bitpack-contend-packed-simd | 1931ns | +536.2ns (+38.1%) | [+502, +553]ns | [1930, 1965] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 1211ns | -231.9ns (-16.5%) | [-265, -197]ns | [1180, 1213] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 1151ns | -232.0ns (-16.5%) | [-250, -225]ns | [1151, 1226] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|
| 1 | 1406ns | -2.1% | +39.8% | -18.3% | -7.2% |
| 2 | 1424ns | -3.2% | +36.8% | -19.4% | -8.5% |
| 3 | 1481ns | -7.0% | +30.1% | -22.5% | -11.9% |
| 4 | 1507ns | -8.4% | +28.0% | -23.7% | -13.3% |
| 5 | 1443ns | -4.0% | +33.7% | -20.6% | -9.6% |
| 6 | 1445ns | -4.8% | +33.6% | -18.2% | -9.8% |
| 7 | 1422ns | -3.1% | +40.6% | -18.8% | -8.2% |
| 8 | 1387ns | -0.9% | +44.3% | -17.2% | -5.9% |
| 9 | 1410ns | -2.2% | +39.3% | -18.5% | -7.6% |
| 10 | 1417ns | -2.7% | +36.6% | -18.8% | -7.9% |
| 11 | 1812ns | -24.1% | +6.4% | -23.2% | -36.6% |
| 12 | 1812ns | -24.0% | +7.9% | -23.0% | -28.1% |
| 13 | 1812ns | -24.1% | +10.0% | -23.0% | -23.1% |
| 14 | 1747ns | -21.2% | +16.1% | -20.3% | -20.3% |
| 15 | 1668ns | -17.6% | +21.5% | -16.6% | -26.9% |
| 16 | 1670ns | -17.6% | +17.2% | -16.7% | -31.2% |
| 17 | 1670ns | -17.5% | +15.5% | -16.7% | -31.2% |
| 18 | 1668ns | -17.5% | +15.5% | -5.3% | -30.4% |
| 19 | 1671ns | -17.6% | +15.6% | -7.3% | -26.3% |
| 20 | 1669ns | -17.6% | +15.6% | -16.5% | -31.1% |
| 21 | 1378ns | +1.8% | +40.2% | -15.0% | -16.3% |
| 22 | 1377ns | +1.7% | +40.2% | -10.9% | -16.4% |
| 23 | 1392ns | +0.8% | +38.7% | -16.1% | -17.0% |
| 24 | 1388ns | +0.9% | +38.8% | -15.1% | -17.0% |
| 25 | 1388ns | +0.9% | +39.1% | -12.7% | -17.2% |
| 26 | 1389ns | +0.7% | +38.5% | -12.1% | -17.1% |
| 27 | 1402ns | -0.3% | +41.3% | -13.6% | -18.0% |
| 28 | 1397ns | +3.1% | +38.2% | -14.8% | -17.7% |
| 29 | 1402ns | -0.2% | +37.6% | -16.7% | -18.1% |
| 30 | 1400ns | +0.0% | +37.7% | -16.4% | -17.8% |
| 31 | 1375ns | +0.1% | +84.7% | +31.7% | -16.2% |
| 32 | 1455ns | -5.5% | +74.5% | -16.7% | -21.1% |
| 33 | 1418ns | -2.9% | +79.2% | -14.4% | -18.9% |
| 34 | 1376ns | +0.1% | +84.7% | -12.0% | -16.3% |
| 35 | 1375ns | +0.2% | +84.6% | -11.9% | -16.3% |
| 36 | 1377ns | -0.1% | +84.6% | -11.9% | -16.4% |
| 37 | 1375ns | +0.5% | +52.4% | -12.0% | -16.4% |
| 38 | 1377ns | +2.4% | +39.9% | -12.1% | -16.5% |
| 39 | 1376ns | +7.0% | +40.1% | -12.0% | -16.4% |
| 40 | 1375ns | +9.9% | +40.2% | -11.9% | -16.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.823 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.566 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.823 | HIGH+ (drift/warm-up) |
| bitpack-contend-pipe2 | 0.411 | moderate+ |
| bitpack-contend-pipe4 | 0.773 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 25/40, lost 12/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40
- **bitpack-contend-pipe2**: won 39/40, lost 1/40
- **bitpack-contend-pipe4**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 2.3ns | 1483.3ns | 0.2% |  |
| bitpack-contend-d16-control | 2.1ns | 1390.6ns | 0.2% |  |
| bitpack-contend-packed-simd | 2.2ns | 2039.8ns | 0.1% |  |
| bitpack-contend-pipe2 | 2.0ns | 1260.6ns | 0.2% |  |
| bitpack-contend-pipe4 | 1.8ns | 1208.9ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1375.7-1732.8 ns)
   1375.7 |########################################
   1393.5 |#####################
   1411.4 |##############
   1429.2 |#######
   1447.1 |###
   1465.0 |###
   1482.8 |
   1500.7 |###
   1518.5 |
   1536.4 |
   1554.2 |
   1572.1 |
   1590.0 |
   1607.8 |
   1625.7 |
   1643.5 |
   1661.4 |#####################
   1679.3 |
   1697.1 |
   1715.0 |
  (4 below, 4 above range)

bitpack-contend-d16-control (n=40, range 1375.3-1430.0 ns)
   1375.3 |########################################
   1378.1 |#########
   1380.8 |##
   1383.6 |##
   1386.3 |
   1389.0 |
   1391.8 |
   1394.5 |
   1397.2 |#######
   1400.0 |###########
   1402.7 |##
   1405.4 |
   1408.2 |##
   1410.9 |
   1413.6 |
   1416.4 |
   1419.1 |
   1421.8 |
   1424.6 |
   1427.3 |
  (4 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 1926.5-2420.3 ns)
   1926.5 |########################################
   1951.1 |########
   1975.8 |######
   2000.5 |##
   2025.2 |####
   2049.9 |
   2074.6 |##
   2099.3 |
   2124.0 |
   2148.7 |
   2173.4 |
   2198.1 |
   2222.8 |
   2247.5 |
   2272.2 |
   2296.8 |
   2321.5 |
   2346.2 |
   2370.9 |
   2395.6 |
  (3 below, 6 above range)

bitpack-contend-pipe2 (n=40, range 1148.3-1488.6 ns)
   1148.3 |##################
   1165.3 |#####################
   1182.3 |###
   1199.3 |########################################
   1216.4 |#######
   1233.4 |
   1250.4 |
   1267.4 |
   1284.4 |
   1301.4 |
   1318.5 |
   1335.5 |
   1352.5 |
   1369.5 |
   1386.5 |#############################
   1403.5 |
   1420.6 |
   1437.6 |
   1454.6 |
   1471.6 |
  (4 below, 3 above range)

bitpack-contend-pipe4 (n=40, range 1148.9-1327.1 ns)
   1148.9 |########################################
   1157.8 |##
   1166.7 |
   1175.7 |
   1184.6 |
   1193.5 |
   1202.4 |
   1211.3 |##
   1220.2 |
   1229.1 |##
   1238.0 |
   1246.9 |
   1255.9 |
   1264.8 |
   1273.7 |
   1282.6 |
   1291.5 |
   1300.4 |######################
   1309.3 |
   1318.2 |
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-control**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **bitpack-contend-packed-simd**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **bitpack-contend-pipe4**: autocorrelation=0.77 (measurement drift or warm-up artifact)
