# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-wide-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-wide-d16 has the worst median (1.45 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-wide-pipe4 at 1.14 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-wide-pipe4 is fastest but the noisiest (CV 49.1%)

bitpack-wide-pipe4 wins on median (1.14 ms) yet has the highest variance (CV 49.1%), while bitpack-wide-d16-control is the steadiest (CV 15.9%, 1.23 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-wide-d16-control shows warm-up / thermal drift (autocorr +0.78)

bitpack-wide-d16-control's per-pass series has lag-1 autocorrelation +0.78, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (309.75 us) is smaller than the fastest variant's own run-to-run std-dev (561.10 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bitpack-wide-pipe4 vs stability leader bitpack-wide-d16-control (+8% speed for 3.1x steadier)

bitpack-wide-pipe4 is fastest (1.14 ms, CV 49.1%); bitpack-wide-d16-control gives up 7.7% median for 3.1x lower variance (CV 15.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bitpack-wide-pipe4 is inconsistent: worst-20% is 2.1x its best-20%

bitpack-wide-pipe4's best 20% of batches run at 1.01 ms but its worst 20% at 2.11 ms (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-wide-pipe4** at 1141873.5 ns median (-21.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.27x (fastest 1141873.5 ns, slowest 1451618.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 1502078ns | 1454595ns | 1191772ns | 1442715ns | 1990471ns | base |
| bitpack-wide-d16-control | 1326022ns | 1231503ns | 1162459ns | 1267987ns | 1663690ns | -11.72% |
| bitpack-wide-d16-padal | 1259244ns | 1182845ns | 1126740ns | 1190841ns | 1596957ns | -16.17% |
| bitpack-wide-pipe4 | 1356792ns | 1143603ns | 1009652ns | 1219652ns | 2115350ns | -9.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 1499239ns | 1189842ns | 1985683ns | base | 22.381 |
| bitpack-wide-d16-control | 1324163ns | 1160328ns | 1661870ns | -11.68% | 25.340 |
| bitpack-wide-d16-padal | 1256660ns | 1125120ns | 1591222ns | -16.18% | 26.701 |
| bitpack-wide-pipe4 | 1354819ns | 1007829ns | 2113069ns | -9.63% | 24.767 |

## Performance model

- Peak throughput: **33.294 Gops/s** (bitpack-wide-pipe4; best 20% batches)
- Ops per call: 33554432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 23.115 | 69.4% |
| bitpack-wide-d16-control | 27.282 | 81.9% |
| bitpack-wide-d16-padal | 28.410 | 85.3% |
| bitpack-wide-pipe4 | 29.385 | 88.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 1502078ns | 1502078ns | base |
| bitpack-wide-d16-control | 1326022ns | 1326022ns | -11.72% |
| bitpack-wide-d16-padal | 1259244ns | 1259244ns | -16.17% |
| bitpack-wide-pipe4 | 1356792ns | 1356792ns | -9.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 1451619ns | base | --- | [1339525, 1512518] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 1229919ns | -102073.3ns (-7.0%) | [-136027, -30406]ns | [1196840, 1289015] | YES | 0.0064 | 0.0064 | 0 |
| bitpack-wide-d16-padal | 1181066ns | -216435.6ns (-14.9%) | [-315429, -116482]ns | [1160296, 1212795] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-wide-pipe4 | 1141874ns | -185154.3ns (-12.8%) | [-288670, -92351]ns | [1113130, 1286678] | YES | 0.0001 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|
| 1 | 1129605ns | +3.2% | +24.4% | -12.7% |
| 2 | 1212550ns | -3.7% | +4.6% | -13.4% |
| 3 | 1227679ns | +3.3% | -5.2% | -20.8% |
| 4 | 1152574ns | +3.0% | +1.2% | -10.5% |
| 5 | 1201838ns | -0.6% | -5.1% | +142.8% |
| 6 | 1327525ns | -1.9% | -8.5% | +208.1% |
| 7 | 1496554ns | -7.0% | -20.2% | -19.0% |
| 8 | 1509948ns | -18.4% | -13.4% | -26.9% |
| 9 | 3154450ns | -62.5% | -57.7% | -67.3% |
| 10 | 2619284ns | -55.6% | -54.0% | -60.1% |
| 11 | 1676878ns | +0.4% | -33.6% | -11.0% |
| 12 | 1723234ns | -5.8% | -31.1% | +4.9% |
| 13 | 1663537ns | -3.5% | -24.8% | -4.0% |
| 14 | 1734222ns | -7.1% | -35.3% | -9.1% |
| 15 | 1658748ns | +5.9% | -27.5% | -3.4% |
| 16 | 1588873ns | +8.9% | -16.0% | -2.8% |
| 17 | 1541497ns | +2.1% | -25.6% | +1.3% |
| 18 | 1632251ns | +0.1% | -25.8% | -0.5% |
| 19 | 1655111ns | -0.2% | -31.7% | -6.0% |
| 20 | 1649237ns | -2.8% | -32.7% | +2.5% |
| 21 | 1343617ns | -10.1% | -15.7% | -21.8% |
| 22 | 1485644ns | -16.7% | -22.1% | -23.5% |
| 23 | 1221058ns | +4.5% | -2.8% | -7.0% |
| 24 | 1184540ns | +1.3% | -5.7% | -1.9% |
| 25 | 1249609ns | -8.4% | -9.2% | +16.7% |
| 26 | 1429092ns | -18.3% | -19.9% | -17.7% |
| 27 | 1515088ns | -18.1% | -14.7% | -25.3% |
| 28 | 1508011ns | -22.5% | -23.4% | -22.3% |
| 29 | 1597295ns | -27.2% | -26.4% | -25.0% |
| 30 | 1308236ns | -8.0% | -12.2% | -14.2% |
| 31 | 1454166ns | -19.6% | -19.6% | -21.4% |
| 32 | 1391139ns | -17.9% | -12.0% | -30.0% |
| 33 | 1453848ns | -10.1% | -1.5% | -26.9% |
| 34 | 1434207ns | -11.5% | -18.6% | -31.4% |
| 35 | 1449390ns | -8.8% | +123.2% | -27.0% |
| 36 | 1262851ns | -2.8% | +10.1% | -15.5% |
| 37 | 1374171ns | -13.3% | -8.5% | -24.5% |
| 38 | 1202893ns | +2.1% | +1.9% | -6.0% |
| 39 | 1335433ns | -10.2% | -12.7% | -14.6% |
| 40 | 1213678ns | -3.3% | -5.5% | +12.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | 0.569 | HIGH+ (drift/warm-up) |
| bitpack-wide-d16-control | 0.782 | HIGH+ (drift/warm-up) |
| bitpack-wide-d16-padal | 0.058 | ok |
| bitpack-wide-pipe4 | 0.426 | moderate+ |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 29/40, lost 10/40
- **bitpack-wide-d16-padal**: won 34/40, lost 6/40
- **bitpack-wide-pipe4**: won 33/40, lost 7/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 115.8ns | 1499239.0ns | 0.0% |  |
| bitpack-wide-d16-control | 24.0ns | 1324162.7ns | 0.0% |  |
| bitpack-wide-d16-padal | 33.8ns | 1256659.7ns | 0.0% |  |
| bitpack-wide-pipe4 | 43.3ns | 1354819.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 1189841.9-1985683.1 ns)
  1189841.9 |########################################
  1229634.0 |#############
  1269426.0 |######
  1309218.1 |####################
  1349010.2 |######
  1388802.2 |######
  1428594.3 |#################################
  1468386.3 |####################
  1508178.4 |####################
  1547970.4 |
  1587762.5 |#############
  1627554.6 |#################################
  1667346.6 |######
  1707138.7 |#############
  1746930.7 |
  1786722.8 |
  1826514.8 |
  1866306.9 |
  1906099.0 |
  1945891.0 |
  (3 below, 2 above range)

bitpack-wide-d16-control (n=40, range 1160328.3-1661869.8 ns)
  1160328.3 |########################################
  1185405.4 |###############################
  1210482.4 |#############
  1235559.5 |########
  1260636.6 |#############
  1285713.7 |########
  1310790.7 |####
  1335867.8 |
  1360944.9 |
  1386022.0 |####
  1411099.0 |
  1436176.1 |
  1461253.2 |
  1486330.3 |
  1511407.3 |
  1536484.4 |
  1561561.5 |####
  1586638.6 |########
  1611715.6 |#############
  1636792.7 |####
  (2 below, 3 above range)

bitpack-wide-d16-padal (n=40, range 1125120.1-1591222.4 ns)
  1125120.1 |###################################
  1148425.2 |########################################
  1171730.3 |####################
  1195035.4 |####################
  1218340.5 |##########
  1241645.7 |##########
  1264950.8 |#####
  1288255.9 |##########
  1311561.0 |##########
  1334866.1 |
  1358171.2 |
  1381476.4 |#####
  1404781.5 |#####
  1428086.6 |#####
  1451391.7 |
  1474696.8 |
  1498002.0 |
  1521307.1 |
  1544612.2 |
  1567917.3 |
  (4 below, 1 above range)

bitpack-wide-pipe4 (n=40, range 1007829.0-2113069.0 ns)
  1007829.0 |###################################
  1063091.0 |########
  1118353.0 |########################################
  1173615.0 |#############
  1228877.0 |
  1284139.0 |
  1339401.0 |####
  1394663.0 |
  1449925.0 |########
  1505187.0 |########
  1560449.0 |#################
  1615711.0 |####
  1670973.0 |####
  1726235.0 |
  1781497.0 |####
  1836759.0 |
  1892021.0 |
  1947283.0 |
  2002545.0 |
  2057807.0 |
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-wide-d16**: CV=24.5% (high variance, measurements may be unstable)
- **bitpack-wide-d16**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **bitpack-wide-d16-control**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **bitpack-wide-d16-padal**: CV=26.0% (high variance, measurements may be unstable)
- **bitpack-wide-pipe4**: CV=41.4% (high variance, measurements may be unstable)
