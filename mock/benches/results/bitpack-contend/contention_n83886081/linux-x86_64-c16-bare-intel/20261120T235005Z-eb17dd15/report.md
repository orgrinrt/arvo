# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed beats baseline by 27% (significant)

bitpack-contend-packed is -436.65 us (27%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d64 is an outlier: 2.5x slower than the field

bitpack-contend-d64 (2.88 ms) is 2.5x the fastest (1.15 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32} vs {bitpack-contend-d64} (76% apart)

The field splits into a fast tier {bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32} and a slow tier {bitpack-contend-d64} with a 76% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-packed** at 1152541.0 ns median (-27.9% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.50x (fastest 1152541.0 ns, slowest 2880393.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112542308 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1532196ns | 1599817ns | 1418440ns | 1528488ns | 1657076ns | base |
| bitpack-contend-d16-control | 1436304ns | 1422832ns | 1417815ns | 1425040ns | 1488584ns | -6.26% |
| bitpack-contend-d32 | 1741938ns | 1634095ns | 1592334ns | 1724934ns | 1942557ns | +13.69% |
| bitpack-contend-d64 | 2897879ns | 2881843ns | 2833073ns | 2881900ns | 3010624ns | +89.13% |
| bitpack-contend-packed | 1187222ns | 1153093ns | 1140067ns | 1168911ns | 1289312ns | -22.51% |
| bitpack-contend-packed-simd | 1180716ns | 1165472ns | 1151831ns | 1170652ns | 1239794ns | -22.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 1531407ns | 1417607ns | 1656290ns | base | 5.478 |
| bitpack-contend-d16-control | 1435526ns | 1416854ns | 1487866ns | -6.26% | 5.844 |
| bitpack-contend-d32 | 1740765ns | 1591004ns | 1941369ns | +13.67% | 4.819 |
| bitpack-contend-d64 | 2896459ns | 2831587ns | 3009277ns | +89.14% | 2.896 |
| bitpack-contend-packed | 1186702ns | 1139513ns | 1288868ns | -22.51% | 7.069 |
| bitpack-contend-packed-simd | 1180321ns | 1151437ns | 1239387ns | -22.93% | 7.107 |

## Performance model

- Peak throughput: **7.362 Gops/s** (bitpack-contend-packed; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 5.246 | 71.3% |
| bitpack-contend-d16-control | 5.898 | 80.1% |
| bitpack-contend-d32 | 5.137 | 69.8% |
| bitpack-contend-d64 | 2.912 | 39.6% |
| bitpack-contend-packed | 7.278 | 98.9% |
| bitpack-contend-packed-simd | 7.200 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 1532196ns | 1532196ns | base |
| bitpack-contend-d16-control | 1436304ns | 1436304ns | -6.26% |
| bitpack-contend-d32 | 1741938ns | 1741938ns | +13.69% |
| bitpack-contend-d64 | 2897879ns | 2897879ns | +89.13% |
| bitpack-contend-packed | 1187222ns | 1187222ns | -22.51% |
| bitpack-contend-packed-simd | 1180716ns | 1180716ns | -22.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1599119ns | base | --- | [1426452, 1609686] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 1422225ns | -162581.3ns (-10.2%) | [-178760, -8765]ns | [1421561, 1428750] | YES | 0.0007 | 0.0007 | 0 |
| bitpack-contend-d32 | 1632909ns | +197105.3ns (+12.3%) | [+172215, +290190]ns | [1627084, 1907640] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 2880394ns | +1306697.9ns (+81.7%) | [+1250467, +1434516]ns | [2855248, 2903589] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 1152541ns | -436651.6ns (-27.3%) | [-461341, -266166]ns | [1145246, 1160947] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 1165030ns | -338309.6ns (-21.2%) | [-453202, -255555]ns | [1155035, 1185977] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1508845ns | -5.8% | +8.0% | +88.9% | -23.3% | -21.0% |
| 1 | 2 | 1418678ns | -0.1% | +12.2% | +108.3% | -9.3% | -16.1% |
| 1 | 3 | 1661668ns | -14.4% | +13.0% | +74.6% | -31.4% | -30.5% |
| 1 | 4 | 1610667ns | -11.1% | +19.4% | +76.3% | -29.0% | -28.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.071 | ok |
| bitpack-contend-d16-control | 0.037 | ok |
| bitpack-contend-d32 | 0.245 | moderate+ |
| bitpack-contend-d64 | -0.259 | moderate- |
| bitpack-contend-packed | -0.369 | moderate- |
| bitpack-contend-packed-simd | 0.277 | moderate+ |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 3/4, lost 0/4
- **bitpack-contend-d32**: won 0/4, lost 4/4
- **bitpack-contend-d64**: won 0/4, lost 4/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 149.3ns | 1531407.0ns | 0.0% |  |
| bitpack-contend-d16-control | 159.0ns | 1435526.3ns | 0.0% |  |
| bitpack-contend-d32 | 211.9ns | 1740765.3ns | 0.0% |  |
| bitpack-contend-d64 | 259.3ns | 2896458.7ns | 0.0% |  |
| bitpack-contend-packed | 56.6ns | 1186701.9ns | 0.0% |  |
| bitpack-contend-packed-simd | 90.2ns | 1180320.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1417606.7-1656290.1 ns)
  1417606.7 |########################################
  1429540.9 |######
  1441475.0 |
  1453409.2 |
  1465343.4 |
  1477277.6 |###
  1489211.7 |
  1501145.9 |
  1513080.1 |
  1525014.3 |
  1536948.4 |
  1548882.6 |
  1560816.8 |
  1572750.9 |
  1584685.1 |###
  1596619.3 |##########
  1608553.5 |####################################
  1620487.6 |
  1632421.8 |
  1644356.0 |
  (4 below, 6 above range)

bitpack-contend-d16-control (n=40, range 1416854.5-1487866.4 ns)
  1416854.5 |#############################
  1420405.1 |########################################
  1423955.7 |#######
  1427506.3 |#####################
  1431056.9 |#######
  1434607.5 |##############
  1438158.1 |
  1441708.6 |
  1445259.2 |
  1448809.8 |
  1452360.4 |
  1455911.0 |
  1459461.6 |
  1463012.2 |
  1466562.8 |
  1470113.4 |
  1473664.0 |
  1477214.6 |
  1480765.2 |
  1484315.8 |
  (4 below, 3 above range)

bitpack-contend-d32 (n=40, range 1591003.9-1941369.0 ns)
  1591003.9 |################
  1608522.1 |################
  1626040.4 |########################################
  1643558.6 |
  1661076.9 |
  1678595.2 |
  1696113.4 |####
  1713631.7 |
  1731149.9 |
  1748668.2 |
  1766186.4 |
  1783704.7 |
  1801223.0 |####
  1818741.2 |
  1836259.5 |
  1853777.7 |
  1871296.0 |
  1888814.3 |####
  1906332.5 |################
  1923850.8 |####################
  (5 below, 5 above range)

bitpack-contend-d64 (n=40, range 2831587.4-3009277.0 ns)
  2831587.4 |########################################
  2840471.9 |########################
  2849356.4 |########################
  2858240.8 |################
  2867125.3 |########################
  2876009.8 |
  2884894.3 |################################
  2893778.8 |################
  2902663.2 |################
  2911547.7 |########
  2920432.2 |################
  2929316.7 |################
  2938201.2 |########
  2947085.6 |
  2955970.1 |########
  2964854.6 |########
  2973739.1 |
  2982623.5 |
  2991508.0 |
  3000392.5 |
  (4 below, 4 above range)

bitpack-contend-packed (n=40, range 1139513.0-1288868.5 ns)
  1139513.0 |########################################
  1146980.8 |############################
  1154448.6 |####################
  1161916.3 |####
  1169384.1 |####
  1176851.9 |
  1184319.7 |
  1191787.4 |
  1199255.2 |
  1206723.0 |
  1214190.8 |
  1221658.5 |
  1229126.3 |
  1236594.1 |
  1244061.8 |
  1251529.6 |
  1258997.4 |
  1266465.2 |####
  1273932.9 |
  1281400.7 |########################
  (5 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 1151437.1-1239387.1 ns)
  1151437.1 |########################################
  1155834.6 |##################
  1160232.1 |
  1164629.6 |
  1169027.1 |###
  1173424.6 |#######
  1177822.1 |###
  1182219.6 |##########
  1186617.1 |#######
  1191014.6 |##################
  1195412.1 |###
  1199809.6 |
  1204207.1 |
  1208604.6 |
  1213002.1 |
  1217399.6 |
  1221797.1 |
  1226194.6 |
  1230592.1 |
  1234989.6 |
  (4 below, 5 above range)

```
