# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

5 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-packed dominates: 13% faster than the next best (bitpack-carrier-d32)

bitpack-carrier-packed (1.05 ms) leads bitpack-carrier-d32 (1.19 ms) by 13%, a clear separation rather than a photo finish. CV 5.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-packed beats baseline by 32% (significant)

bitpack-carrier-packed is -498.57 us (32%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d32 shows alternating (throttle bounce) (autocorr -0.66)

bitpack-carrier-d32's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-carrier-packed, bitpack-carrier-d32} vs {bitpack-carrier-d16-control, bitpack-carrier-d16, bitpack-carrier-d64} (29% apart)

The field splits into a fast tier {bitpack-carrier-packed, bitpack-carrier-d32} and a slow tier {bitpack-carrier-d16-control, bitpack-carrier-d16, bitpack-carrier-d64} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-carrier-packed** at 1046741.9 ns median (-32.4% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.61x (fastest 1046741.9 ns, slowest 1687750.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1533321ns | 1550922ns | 1399099ns | 1526865ns | 1686910ns | base |
| bitpack-carrier-d16-control | 1517087ns | 1533645ns | 1397112ns | 1523206ns | 1618703ns | -1.06% |
| bitpack-carrier-d32 | 1106787ns | 1188915ns | 892832ns | 1131159ns | 1247625ns | -27.82% |
| bitpack-carrier-d64 | 1688517ns | 1690074ns | 1527111ns | 1694615ns | 1831631ns | +10.12% |
| bitpack-carrier-packed | 1035593ns | 1048564ns | 937780ns | 1042296ns | 1113297ns | -32.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1531248ns | 1397083ns | 1684546ns | base | 2.739 |
| bitpack-carrier-d16-control | 1515041ns | 1395137ns | 1616519ns | -1.06% | 2.768 |
| bitpack-carrier-d32 | 1104775ns | 891189ns | 1245418ns | -27.85% | 3.797 |
| bitpack-carrier-d64 | 1685993ns | 1523746ns | 1829155ns | +10.11% | 2.488 |
| bitpack-carrier-packed | 1033822ns | 936193ns | 1111279ns | -32.49% | 4.057 |

## Performance model

- Peak throughput: **4.706 Gops/s** (bitpack-carrier-d32; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 2.708 | 57.5% |
| bitpack-carrier-d16-control | 2.738 | 58.2% |
| bitpack-carrier-d32 | 3.535 | 75.1% |
| bitpack-carrier-d64 | 2.485 | 52.8% |
| bitpack-carrier-packed | 4.007 | 85.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 1533321ns | 1533321ns | base |
| bitpack-carrier-d16-control | 1517087ns | 1517087ns | -1.06% |
| bitpack-carrier-d32 | 1106787ns | 1106787ns | -27.82% |
| bitpack-carrier-d64 | 1688517ns | 1688517ns | +10.12% |
| bitpack-carrier-packed | 1035593ns | 1035593ns | -32.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1548785ns | base | --- | [1470276, 1574208] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 1531647ns | no significant difference | [-73506, +53853]ns | [1486920, 1551514] | no | 0.6358 | 0.6358 | 0 |
| bitpack-carrier-d32 | 1186656ns | -442115.1ns (-28.5%) | [-521175, -332274]ns | [1053399, 1215880] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 1687751ns | +158325.8ns (+10.2%) | [+88639, +233181]ns | [1663898, 1718901] | YES | 0.0001 | 0.0000 | 0 |
| bitpack-carrier-packed | 1046742ns | -498566.6ns (-32.2%) | [-537132, -445597]ns | [1030023, 1053451] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed |
|---|---|---|---|---|---|---|
| 1 | 1 | 1563724ns | -0.2% | -42.0% | +12.0% | -32.4% |
| 1 | 2 | 1574208ns | -3.2% | -24.6% | +6.4% | -33.6% |
| 1 | 3 | 1559664ns | -6.8% | -39.3% | -2.8% | -38.6% |
| 1 | 4 | 1419328ns | +9.0% | -13.5% | +18.1% | -25.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.026 | ok |
| bitpack-carrier-d16-control | -0.262 | moderate- |
| bitpack-carrier-d32 | -0.659 | HIGH- (thermal bounce) |
| bitpack-carrier-d64 | -0.132 | ok |
| bitpack-carrier-packed | -0.363 | moderate- |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 3/4, lost 1/4
- **bitpack-carrier-d32**: won 4/4, lost 0/4
- **bitpack-carrier-d64**: won 1/4, lost 3/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 743.4ns | 1531248.5ns | 0.0% |  |
| bitpack-carrier-d16-control | 663.4ns | 1515041.1ns | 0.0% |  |
| bitpack-carrier-d32 | 778.6ns | 1104775.0ns | 0.1% |  |
| bitpack-carrier-d64 | 876.7ns | 1685993.1ns | 0.1% |  |
| bitpack-carrier-packed | 568.7ns | 1033822.1ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 1397083.1-1684546.1 ns)
  1397083.1 |########
  1411456.2 |########################
  1425829.4 |################
  1440202.5 |################
  1454575.7 |########
  1468948.8 |
  1483322.0 |########################
  1497695.1 |
  1512068.3 |########
  1526441.4 |########
  1540814.6 |################
  1555187.8 |################################
  1569560.9 |########################################
  1583934.1 |################
  1598307.2 |########
  1612680.4 |
  1627053.5 |################
  1641426.7 |########
  1655799.8 |
  1670173.0 |
  (5 below, 4 above range)

bitpack-carrier-d16-control (n=40, range 1395136.7-1616519.2 ns)
  1395136.7 |#############
  1406205.8 |#############
  1417274.9 |
  1428344.0 |##########################
  1439413.2 |##########################
  1450482.3 |##########################
  1461551.4 |
  1472620.6 |#############
  1483689.7 |##########################
  1494758.8 |
  1505827.9 |##########################
  1516897.1 |########################################
  1527966.2 |##########################
  1539035.3 |########################################
  1550104.4 |########################################
  1561173.6 |#############
  1572242.7 |########################################
  1583311.8 |##########################
  1594380.9 |#############
  1605450.1 |########################################
  (4 below, 2 above range)

bitpack-carrier-d32 (n=40, range 891189.1-1245418.1 ns)
  891189.1 |#################
  908900.6 |#################
  926612.0 |#####
  944323.5 |#####
  962034.9 |
  979746.4 |#####
  997457.8 |
  1015169.3 |
  1032880.7 |
  1050592.1 |
  1068303.6 |
  1086015.0 |
  1103726.5 |
  1121437.9 |###########
  1139149.4 |
  1156860.8 |#####
  1174572.3 |############################
  1192283.7 |######################
  1209995.2 |############################
  1227706.6 |########################################
  (5 below, 2 above range)

bitpack-carrier-d64 (n=40, range 1523746.3-1829154.9 ns)
  1523746.3 |
  1539016.7 |########
  1554287.1 |
  1569557.6 |
  1584828.0 |
  1600098.4 |
  1615368.8 |########
  1630639.3 |################
  1645909.7 |################################
  1661180.1 |########################
  1676450.6 |########################################
  1691721.0 |########################
  1706991.4 |########################################
  1722261.9 |########
  1737532.3 |########################
  1752802.7 |################
  1768073.1 |
  1783343.6 |########
  1798614.0 |
  1813884.4 |########
  (5 below, 3 above range)

bitpack-carrier-packed (n=40, range 936193.0-1111279.1 ns)
  936193.0 |######
  944947.3 |#############
  953701.6 |#############
  962455.9 |
  971210.2 |######
  979964.5 |
  988718.8 |
  997473.1 |
  1006227.4 |######
  1014981.7 |#############
  1023736.0 |##########################
  1032490.3 |######
  1041244.6 |########################################
  1049998.9 |########################################
  1058753.2 |#############
  1067507.5 |####################
  1076261.9 |
  1085016.2 |#############
  1093770.5 |
  1102524.8 |######
  (3 below, 3 above range)

```
