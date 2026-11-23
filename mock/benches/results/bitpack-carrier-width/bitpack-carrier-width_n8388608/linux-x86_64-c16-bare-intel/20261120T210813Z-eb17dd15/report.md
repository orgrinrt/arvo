# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d64 is an outlier: 2.4x slower than the field

bitpack-carrier-d64 (2.85 ms) is 2.4x the fastest (1.17 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-carrier-packed shows alternating (throttle bounce) (autocorr -0.73)

bitpack-carrier-packed's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-carrier-packed, bitpack-carrier-packed-simd, bitpack-carrier-d16, bitpack-carrier-d16-control, bitpack-carrier-d32} vs {bitpack-carrier-d64} (72% apart)

The field splits into a fast tier {bitpack-carrier-packed, bitpack-carrier-packed-simd, bitpack-carrier-d16, bitpack-carrier-d16-control, bitpack-carrier-d32} and a slow tier {bitpack-carrier-d64} with a 72% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-carrier-packed** at 1168220.5 ns median (-17.4% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.44x (fastest 1168220.5 ns, slowest 2854900.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112762950 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1416291ns | 1415828ns | 1411380ns | 1415769ns | 1422769ns | base |
| bitpack-carrier-d16-control | 1460222ns | 1419029ns | 1414126ns | 1432697ns | 1588892ns | +3.10% |
| bitpack-carrier-d32 | 1687542ns | 1656083ns | 1606354ns | 1662844ns | 1842823ns | +19.15% |
| bitpack-carrier-d64 | 2882691ns | 2856519ns | 2802631ns | 2861879ns | 3025187ns | +103.54% |
| bitpack-carrier-packed | 1207993ns | 1168978ns | 1149698ns | 1206376ns | 1271141ns | -14.71% |
| bitpack-carrier-packed-simd | 1201044ns | 1181908ns | 1145772ns | 1191927ns | 1283669ns | -15.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1415433ns | 1410398ns | 1421892ns | base | 5.927 |
| bitpack-carrier-d16-control | 1459559ns | 1413297ns | 1588210ns | +3.12% | 5.747 |
| bitpack-carrier-d32 | 1686397ns | 1605125ns | 1841617ns | +19.14% | 4.974 |
| bitpack-carrier-d64 | 2881122ns | 2801141ns | 3023633ns | +103.55% | 2.912 |
| bitpack-carrier-packed | 1207487ns | 1149218ns | 1270719ns | -14.69% | 6.947 |
| bitpack-carrier-packed-simd | 1200531ns | 1145239ns | 1283129ns | -15.18% | 6.987 |

## Performance model

- Peak throughput: **7.325 Gops/s** (bitpack-carrier-packed-simd; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 5.928 | 80.9% |
| bitpack-carrier-d16-control | 5.914 | 80.7% |
| bitpack-carrier-d32 | 5.068 | 69.2% |
| bitpack-carrier-d64 | 2.938 | 40.1% |
| bitpack-carrier-packed | 7.181 | 98.0% |
| bitpack-carrier-packed-simd | 7.100 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 1416291ns | 1416291ns | base |
| bitpack-carrier-d16-control | 1460222ns | 1460222ns | +3.10% |
| bitpack-carrier-d32 | 1687542ns | 1687542ns | +19.15% |
| bitpack-carrier-d64 | 2882691ns | 2882691ns | +103.54% |
| bitpack-carrier-packed | 1207993ns | 1207993ns | -14.71% |
| bitpack-carrier-packed-simd | 1201044ns | 1201044ns | -15.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1415097ns | base | --- | [1414273, 1415620] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 1418520ns | +4900.9ns (+0.3%) | [+3304, +5945]ns | [1415929, 1420016] | YES | 0.0007 | 0.0007 | 0 |
| bitpack-carrier-d32 | 1655134ns | +240000.9ns (+17.0%) | [+227887, +248225]ns | [1645888, 1663407] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 2854900ns | +1439746.6ns (+101.7%) | [+1410482, +1466571]ns | [2826016, 2880416] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 1168221ns | -245980.2ns (-17.4%) | [-254776, -152265]ns | [1161372, 1264988] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 1181449ns | -238753.6ns (-16.9%) | [-263614, -193543]ns | [1150720, 1228434] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1415620ns | +12.2% | +13.7% | +103.3% | -10.7% | -18.6% |
| 1 | 2 | 1414624ns | -0.1% | +17.4% | +99.4% | -17.8% | -10.4% |
| 1 | 3 | 1411462ns | +0.3% | +30.4% | +99.9% | -10.1% | -18.9% |
| 1 | 4 | 1415363ns | +0.3% | +16.5% | +113.4% | -18.6% | -16.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.326 | moderate- |
| bitpack-carrier-d16-control | -0.100 | ok |
| bitpack-carrier-d32 | -0.257 | moderate- |
| bitpack-carrier-d64 | -0.155 | ok |
| bitpack-carrier-packed | -0.735 | HIGH- (thermal bounce) |
| bitpack-carrier-packed-simd | -0.632 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 0/4, lost 3/4
- **bitpack-carrier-d32**: won 0/4, lost 4/4
- **bitpack-carrier-d64**: won 0/4, lost 4/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4
- **bitpack-carrier-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 225.6ns | 1415432.6ns | 0.0% |  |
| bitpack-carrier-d16-control | 196.1ns | 1459558.6ns | 0.0% |  |
| bitpack-carrier-d32 | 357.4ns | 1686397.4ns | 0.0% |  |
| bitpack-carrier-d64 | 420.9ns | 2881122.1ns | 0.0% |  |
| bitpack-carrier-packed | 170.4ns | 1207486.8ns | 0.0% |  |
| bitpack-carrier-packed-simd | 179.4ns | 1200531.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 1410397.8-1421891.9 ns)
  1410397.8 |#####
  1410972.5 |###########
  1411547.2 |#################
  1412121.9 |#####
  1412696.6 |
  1413271.3 |#####
  1413846.0 |#################
  1414420.7 |######################
  1414995.4 |########################################
  1415570.1 |###########
  1416144.8 |#####
  1416719.5 |###########
  1417294.2 |#####
  1417869.0 |###########
  1418443.7 |#####
  1419018.4 |
  1419593.1 |#####
  1420167.8 |
  1420742.5 |#####
  1421317.2 |
  (4 below, 3 above range)

bitpack-carrier-d16-control (n=40, range 1413296.9-1588209.5 ns)
  1413296.9 |########################################
  1422042.6 |###
  1430788.2 |
  1439533.8 |
  1448279.5 |
  1457025.1 |
  1465770.7 |
  1474516.3 |
  1483262.0 |
  1492007.6 |
  1500753.2 |
  1509498.9 |
  1518244.5 |
  1526990.1 |
  1535735.8 |
  1544481.4 |
  1553227.0 |
  1561972.6 |
  1570718.3 |
  1579463.9 |##########
  (4 below, 4 above range)

bitpack-carrier-d32 (n=40, range 1605125.1-1841617.4 ns)
  1605125.1 |#################
  1616949.7 |########
  1628774.3 |#############
  1640598.9 |#################
  1652423.5 |########################################
  1664248.1 |#############
  1676072.7 |####
  1687897.4 |####
  1699722.0 |
  1711546.6 |
  1723371.2 |
  1735195.8 |
  1747020.4 |
  1758845.1 |
  1770669.7 |
  1782494.3 |
  1794318.9 |
  1806143.5 |
  1817968.1 |
  1829792.7 |######################
  (4 below, 4 above range)

bitpack-carrier-d64 (n=40, range 2801141.1-3023632.9 ns)
  2801141.1 |#################################
  2812265.6 |#################################
  2823390.2 |########################################
  2834514.8 |
  2845639.4 |#############
  2856764.0 |####################
  2867888.6 |####################
  2879013.2 |######
  2890137.8 |
  2901262.4 |#############
  2912387.0 |######
  2923511.6 |######
  2934636.2 |
  2945760.8 |######
  2956885.4 |
  2968010.0 |
  2979134.6 |
  2990259.2 |
  3001383.8 |######
  3012508.4 |####################
  (2 below, 4 above range)

bitpack-carrier-packed (n=40, range 1149217.9-1270718.5 ns)
  1149217.9 |############################
  1155293.0 |########################
  1161368.0 |####################
  1167443.0 |########
  1173518.0 |####
  1179593.1 |
  1185668.1 |
  1191743.1 |
  1197818.2 |
  1203893.2 |
  1209968.2 |
  1216043.2 |
  1222118.3 |
  1228193.3 |
  1234268.3 |
  1240343.4 |
  1246418.4 |
  1252493.4 |
  1258568.5 |################
  1264643.5 |########################################
  (1 below, 4 above range)

bitpack-carrier-packed-simd (n=40, range 1145239.0-1283129.0 ns)
  1145239.0 |########################################
  1152133.5 |####
  1159028.0 |
  1165922.5 |############
  1172817.0 |
  1179711.5 |####################
  1186606.0 |########
  1193500.5 |
  1200395.0 |
  1207289.5 |
  1214184.0 |
  1221078.5 |
  1227973.0 |
  1234867.5 |
  1241762.0 |
  1248656.5 |
  1255551.0 |
  1262445.5 |########################################
  1269340.0 |
  1276234.5 |
  (5 below, 4 above range)

```
