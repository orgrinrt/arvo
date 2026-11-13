# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

5 variants, 20 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-packed dominates: 14% faster than the next best (bitpack-carrier-d32)

bitpack-carrier-packed (1.06 ms) leads bitpack-carrier-d32 (1.21 ms) by 14%, a clear separation rather than a photo finish. CV 4.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-packed beats baseline by 30% (significant)

bitpack-carrier-packed is -457.48 us (30%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-packed shows alternating (throttle bounce) (autocorr -0.50)

bitpack-carrier-packed's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-carrier-packed** at 1059414.8 ns median (-31.3% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.61x (fastest 1059414.8 ns, slowest 1709658.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1518922ns | 1544565ns | 1401668ns | 1527002ns | 1611938ns | base |
| bitpack-carrier-d16-control | 1529603ns | 1538127ns | 1421209ns | 1533227ns | 1627124ns | +0.70% |
| bitpack-carrier-d32 | 1231858ns | 1211791ns | 1176239ns | 1221718ns | 1317898ns | -18.90% |
| bitpack-carrier-d64 | 1733684ns | 1711889ns | 1653252ns | 1718488ns | 1859702ns | +14.14% |
| bitpack-carrier-packed | 1064326ns | 1061323ns | 1005149ns | 1057543ns | 1143852ns | -29.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1516646ns | 1399978ns | 1609443ns | base | 2.766 |
| bitpack-carrier-d16-control | 1527454ns | 1419443ns | 1624805ns | +0.71% | 2.746 |
| bitpack-carrier-d32 | 1229722ns | 1174053ns | 1315717ns | -18.92% | 3.411 |
| bitpack-carrier-d64 | 1731503ns | 1651283ns | 1857485ns | +14.17% | 2.422 |
| bitpack-carrier-packed | 1062409ns | 1003478ns | 1141546ns | -29.95% | 3.948 |

## Performance model

- Peak throughput: **4.180 Gops/s** (bitpack-carrier-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 2.720 | 65.1% |
| bitpack-carrier-d16-control | 2.731 | 65.3% |
| bitpack-carrier-d32 | 3.468 | 83.0% |
| bitpack-carrier-d64 | 2.453 | 58.7% |
| bitpack-carrier-packed | 3.959 | 94.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 1518922ns | 1518922ns | base |
| bitpack-carrier-d16-control | 1529603ns | 1529603ns | +0.70% |
| bitpack-carrier-d32 | 1231858ns | 1231858ns | -18.90% |
| bitpack-carrier-d64 | 1733684ns | 1733684ns | +14.14% |
| bitpack-carrier-packed | 1064326ns | 1064326ns | -29.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 1542288ns | base | --- | [1472658, 1563861] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 1536070ns | no significant difference | [-36182, +50128]ns | [1481448, 1569958] | no | 1.0000 | 1.0000 | 0 |
| bitpack-carrier-d32 | 1209584ns | -277046.0ns (-18.0%) | [-348600, -218501]ns | [1189425, 1256594] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 1709658ns | +198358.5ns (+12.9%) | [+154663, +253383]ns | [1686729, 1740254] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 1059415ns | -457480.9ns (-29.7%) | [-500457, -405630]ns | [1022863, 1080548] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed |
|---|---|---|---|---|---|---|
| 1 | 1 | 1507064ns | -1.6% | -20.1% | +11.5% | -32.3% |
| 1 | 2 | 1544593ns | +2.2% | -19.1% | +15.0% | -29.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.500 | moderate- |
| bitpack-carrier-d16-control | -0.500 | moderate- |
| bitpack-carrier-d32 | -0.500 | moderate- |
| bitpack-carrier-d64 | -0.500 | moderate- |
| bitpack-carrier-packed | -0.500 | moderate- |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 1/2, lost 1/2
- **bitpack-carrier-d32**: won 2/2, lost 0/2
- **bitpack-carrier-d64**: won 0/2, lost 2/2
- **bitpack-carrier-packed**: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 788.2ns | 1516645.9ns | 0.1% |  |
| bitpack-carrier-d16-control | 843.3ns | 1527453.6ns | 0.1% |  |
| bitpack-carrier-d32 | 869.7ns | 1229722.0ns | 0.1% |  |
| bitpack-carrier-d64 | 872.6ns | 1731503.1ns | 0.1% |  |
| bitpack-carrier-packed | 522.5ns | 1062408.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=20, range 1399978.3-1609442.9 ns)
  1399978.3 |#############
  1410451.5 |
  1420924.7 |
  1431398.0 |
  1441871.2 |
  1452344.4 |##########################
  1462817.6 |#############
  1473290.9 |#############
  1483764.1 |#############
  1494237.3 |
  1504710.6 |
  1515183.8 |#############
  1525657.0 |
  1536130.2 |########################################
  1546603.5 |#############
  1557076.7 |##########################
  1567549.9 |##########################
  1578023.2 |
  1588496.4 |#############
  1598969.6 |
  (2 below, 2 above range)

bitpack-carrier-d16-control (n=20, range 1419442.7-1624804.9 ns)
  1419442.7 |####################
  1429710.8 |########################################
  1439978.9 |
  1450247.0 |
  1460515.1 |####################
  1470783.2 |####################
  1481051.3 |####################
  1491319.4 |####################
  1501587.5 |
  1511855.7 |
  1522123.8 |########################################
  1532391.9 |####################
  1542660.0 |####################
  1552928.1 |
  1563196.2 |########################################
  1573464.3 |####################
  1583732.4 |####################
  1594000.5 |####################
  1604268.7 |########################################
  1614536.8 |
  (1 below, 1 above range)

bitpack-carrier-d32 (n=20, range 1174053.2-1315716.8 ns)
  1174053.2 |#############
  1181136.4 |########################################
  1188219.5 |#############
  1195302.7 |##########################
  1202385.9 |#############
  1209469.1 |#############
  1216552.3 |
  1223635.4 |##########################
  1230718.6 |
  1237801.8 |#############
  1244885.0 |
  1251968.2 |
  1259051.3 |
  1266134.5 |#############
  1273217.7 |
  1280300.9 |#############
  1287384.1 |#############
  1294467.2 |#############
  1301550.4 |
  1308633.6 |
  (2 below, 2 above range)

bitpack-carrier-d64 (n=20, range 1651283.3-1857484.7 ns)
  1651283.3 |##########################
  1661593.3 |
  1671903.4 |#############
  1682213.5 |########################################
  1692523.6 |#############
  1702833.6 |#############
  1713143.7 |#############
  1723453.8 |##########################
  1733763.8 |#############
  1744073.9 |#############
  1754384.0 |
  1764694.1 |
  1775004.1 |
  1785314.2 |
  1795624.3 |#############
  1805934.3 |#############
  1816244.4 |
  1826554.5 |
  1836864.6 |#############
  1847174.6 |
  (2 below, 2 above range)

bitpack-carrier-packed (n=20, range 1003477.6-1141546.2 ns)
  1003477.6 |#############
  1010381.0 |#############
  1017284.5 |########################################
  1024187.9 |
  1031091.3 |
  1037994.8 |
  1044898.2 |##########################
  1051801.6 |#############
  1058705.1 |#############
  1065608.5 |##########################
  1072511.9 |#############
  1079415.3 |#############
  1086318.8 |#############
  1093222.2 |
  1100125.6 |
  1107029.1 |
  1113932.5 |
  1120835.9 |#############
  1127739.4 |#############
  1134642.8 |
  (2 below, 2 above range)

```
