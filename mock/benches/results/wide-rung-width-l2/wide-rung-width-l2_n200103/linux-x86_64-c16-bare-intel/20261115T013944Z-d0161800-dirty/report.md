# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged is fastest but the noisiest (CV 6.5%)

wide-rung-ragged wins on median (1.35 ms) yet has the highest variance (CV 6.5%), while wide-rung-align16 is the steadiest (CV 0.7%, 1.39 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-ragged shows alternating (throttle bounce) (autocorr -0.56)

wide-rung-ragged's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (48.02 us) is smaller than the fastest variant's own run-to-run std-dev (87.52 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-ragged vs stability leader wide-rung-align16 (+3% speed for 9.4x steadier)

wide-rung-ragged is fastest (1.35 ms, CV 6.5%); wide-rung-align16 gives up 3.1% median for 9.4x lower variance (CV 0.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 3.6% of the fastest

All 5 variants sit between 1.35 ms and 1.39 ms - a 3.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged** at 1345411.6 ns median (-3.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.04x (fastest 1345411.6 ns, slowest 1393432.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1393347ns | 1388257ns | 1384215ns | 1391423ns | 1408251ns | base |
| wide-rung-ragged | 1315737ns | 1345863ns | 1218491ns | 1305487ns | 1443733ns | -5.57% |
| wide-rung-ragged-overread | 1365572ns | 1361810ns | 1321466ns | 1365564ns | 1409703ns | -1.99% |
| wide-rung-wordround | 1410732ns | 1385719ns | 1320693ns | 1402806ns | 1524548ns | +1.25% |
| wide-rung-wordround-alias | 1399440ns | 1393946ns | 1387421ns | 1394438ns | 1426463ns | +0.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1392760ns | 1383651ns | 1407679ns | base | 1.318 |
| wide-rung-ragged | 1315269ns | 1218007ns | 1443278ns | -5.56% | 1.395 |
| wide-rung-ragged-overread | 1365162ns | 1321164ns | 1409210ns | -1.98% | 1.344 |
| wide-rung-wordround | 1410127ns | 1320135ns | 1523901ns | +1.25% | 1.301 |
| wide-rung-wordround-alias | 1398896ns | 1386893ns | 1425868ns | +0.44% | 1.312 |

## Performance model

- Peak throughput: **1.507 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.323 | 87.8% |
| wide-rung-ragged | 1.364 | 90.5% |
| wide-rung-ragged-overread | 1.348 | 89.5% |
| wide-rung-wordround | 1.325 | 87.9% |
| wide-rung-wordround-alias | 1.317 | 87.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1393347ns | 1393347ns | base |
| wide-rung-ragged | 1315737ns | 1315737ns | -5.57% |
| wide-rung-ragged-overread | 1365572ns | 1365572ns | -1.99% |
| wide-rung-wordround | 1410732ns | 1410732ns | +1.25% |
| wide-rung-wordround-alias | 1399440ns | 1399440ns | +0.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1387479ns | base | --- | [1386057, 1397318] | --- | --- | --- | --- |
| wide-rung-ragged | 1345412ns | -38852.1ns (-2.8%) | [-140775, -35726]ns | [1249818, 1348689] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 1361338ns | -26901.2ns (-1.9%) | [-32053, -22570]ns | [1357548, 1363565] | YES | 0.0332 | 0.0166 | 0 |
| wide-rung-wordround | 1385149ns | -12477.2ns (-0.9%) | [-15736, -1227]ns | [1375765, 1386410] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| wide-rung-wordround-alias | 1393433ns | no significant difference | [-1730, +8095]ns | [1391395, 1397179] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1386132ns | -12.1% | -2.0% | -1.1% | +0.1% |
| 1 | 2 | 1384149ns | -2.6% | +1.7% | +10.0% | +0.6% |
| 1 | 3 | 1390637ns | -10.1% | -2.0% | -0.4% | +0.9% |
| 1 | 4 | 1403382ns | -1.2% | -5.8% | -1.7% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.142 | ok |
| wide-rung-ragged | -0.564 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | -0.051 | ok |
| wide-rung-wordround | -0.436 | moderate- |
| wide-rung-wordround-alias | -0.106 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 3/4, lost 1/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 112.0ns | 1392760.2ns | 0.0% |  |
| wide-rung-ragged | 89.9ns | 1315269.1ns | 0.0% |  |
| wide-rung-ragged-overread | 89.0ns | 1365162.2ns | 0.0% |  |
| wide-rung-wordround | 101.6ns | 1410127.4ns | 0.0% |  |
| wide-rung-wordround-alias | 111.4ns | 1398895.9ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1383651.3-1407679.0 ns)
  1383651.3 |########################################
  1384852.7 |########################################
  1386054.1 |########################################
  1387255.5 |################################
  1388456.8 |
  1389658.2 |########
  1390859.6 |
  1392061.0 |
  1393262.4 |########
  1394463.8 |########
  1395665.1 |
  1396866.5 |
  1398067.9 |
  1399269.3 |################
  1400470.7 |########################
  1401672.1 |########
  1402873.4 |########################
  1404074.8 |
  1405276.2 |########
  1406477.6 |########
  (4 below, 3 above range)

wide-rung-ragged (n=40, range 1218006.7-1443277.8 ns)
  1218006.7 |##################
  1229270.3 |
  1240533.8 |################################
  1251797.4 |###
  1263060.9 |
  1274324.5 |
  1285588.0 |
  1296851.6 |
  1308115.1 |
  1319378.7 |
  1330642.2 |
  1341905.8 |########################################
  1353169.3 |
  1364432.9 |
  1375696.4 |##################
  1386960.0 |###
  1398223.5 |
  1409487.1 |
  1420750.6 |
  1432014.2 |
  (4 below, 4 above range)

wide-rung-ragged-overread (n=40, range 1321164.1-1409210.4 ns)
  1321164.1 |########################
  1325566.4 |
  1329968.7 |
  1334371.0 |
  1338773.3 |
  1343175.6 |
  1347578.0 |
  1351980.3 |####
  1356382.6 |############################
  1360784.9 |####################################
  1365187.2 |####
  1369589.6 |
  1373991.9 |
  1378394.2 |
  1382796.5 |
  1387198.8 |
  1391601.1 |
  1396003.5 |
  1400405.8 |
  1404808.1 |########################################
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 1320134.8-1523900.8 ns)
  1320134.8 |
  1330323.1 |
  1340511.4 |
  1350699.7 |
  1360888.0 |############################
  1371076.3 |########################
  1381264.6 |########################################
  1391452.9 |####
  1401641.2 |
  1411829.5 |
  1422017.8 |
  1432206.1 |
  1442394.4 |
  1452582.7 |
  1462771.0 |
  1472959.3 |####
  1483147.6 |
  1493335.9 |
  1503524.2 |
  1513712.5 |################################
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1386893.4-1425868.4 ns)
  1386893.4 |########################################
  1388842.1 |####################
  1390790.9 |########################################
  1392739.6 |##########################
  1394688.4 |######
  1396637.1 |##########################
  1398585.9 |####################
  1400534.6 |######
  1402483.4 |####################
  1404432.1 |#############
  1406380.9 |
  1408329.7 |
  1410278.4 |######
  1412227.2 |
  1414175.9 |
  1416124.7 |
  1418073.4 |
  1420022.2 |
  1421970.9 |
  1423919.7 |
  (4 below, 2 above range)

```
