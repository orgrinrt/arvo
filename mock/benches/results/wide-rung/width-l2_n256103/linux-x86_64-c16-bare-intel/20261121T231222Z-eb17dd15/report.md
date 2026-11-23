# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (1.50 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 1.49 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-wordround-alias shows alternating (throttle bounce) (autocorr -0.51)

wide-rung-wordround-alias's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (10.12 us) is smaller than the fastest variant's own run-to-run std-dev (88.52 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-ragged-overread vs stability leader wide-rung-wordround (+1% speed for 1.5x steadier)

wide-rung-ragged-overread is fastest (1.49 ms, CV 6.0%); wide-rung-wordround gives up 0.6% median for 1.5x lower variance (CV 3.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.7% of the fastest

All 5 variants sit between 1.49 ms and 1.50 ms - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 1486949.1 ns median (-0.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.01x (fastest 1486949.1 ns, slowest 1497072.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112691900 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1503303ns | 1497778ns | 1383096ns | 1498885ns | 1636762ns | base |
| wide-rung-ragged | 1541855ns | 1496151ns | 1489269ns | 1521495ns | 1655520ns | +2.56% |
| wide-rung-ragged-overread | 1467336ns | 1487591ns | 1374631ns | 1451648ns | 1607102ns | -2.39% |
| wide-rung-wordround | 1474613ns | 1495964ns | 1379543ns | 1487389ns | 1531358ns | -1.91% |
| wide-rung-wordround-alias | 1489861ns | 1491950ns | 1376567ns | 1477646ns | 1639799ns | -0.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1502666ns | 1382507ns | 1636129ns | base | 1.221 |
| wide-rung-ragged | 1541274ns | 1488798ns | 1654804ns | +2.57% | 1.191 |
| wide-rung-ragged-overread | 1466831ns | 1374184ns | 1606566ns | -2.38% | 1.251 |
| wide-rung-wordround | 1474088ns | 1379168ns | 1530781ns | -1.90% | 1.245 |
| wide-rung-wordround-alias | 1489268ns | 1376092ns | 1639268ns | -0.89% | 1.232 |

## Performance model

- Peak throughput: **1.335 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.226 | 91.8% |
| wide-rung-ragged | 1.227 | 91.9% |
| wide-rung-ragged-overread | 1.234 | 92.4% |
| wide-rung-wordround | 1.227 | 91.9% |
| wide-rung-wordround-alias | 1.231 | 92.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1503303ns | 1503303ns | base |
| wide-rung-ragged | 1541855ns | 1541855ns | +2.56% |
| wide-rung-ragged-overread | 1467336ns | 1467336ns | -2.39% |
| wide-rung-wordround | 1474613ns | 1474613ns | -1.91% |
| wide-rung-wordround-alias | 1489861ns | 1489861ns | -0.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1497073ns | base | --- | [1490951, 1501286] | --- | --- | --- | --- |
| wide-rung-ragged | 1495639ns | no significant difference | [-1410, +107014]ns | [1493546, 1503200] | no | 0.5728 | 0.4296 | 0 |
| wide-rung-ragged-overread | 1486949ns | -14124.3ns (-0.9%) | [-105902, -5406]ns | [1388649, 1489674] | YES | 0.0129 | 0.0064 | 0 |
| wide-rung-wordround | 1495274ns | no significant difference | [-5059, +3083]ns | [1491868, 1496605] | no | 1.0000 | 1.0000 | 0 |
| wide-rung-wordround-alias | 1491190ns | -79827.7ns (-5.3%) | [-110332, -8989]ns | [1437310, 1494921] | YES | 0.0027 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1490783ns | -0.1% | -6.8% | +0.1% | -7.4% |
| 1 | 2 | 1383525ns | +7.9% | +7.5% | +8.3% | +18.5% |
| 1 | 3 | 1592284ns | +3.8% | +2.1% | -13.3% | -6.2% |
| 1 | 4 | 1500523ns | -0.1% | -8.4% | -0.1% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.452 | moderate- |
| wide-rung-ragged | -0.377 | moderate- |
| wide-rung-ragged-overread | -0.335 | moderate- |
| wide-rung-wordround | -0.460 | moderate- |
| wide-rung-wordround-alias | -0.511 | HIGH- (thermal bounce) |

**Consistency summary:**

- **wide-rung-ragged**: won 0/4, lost 2/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 1/4, lost 2/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 75.8ns | 1502666.2ns | 0.0% |  |
| wide-rung-ragged | 94.5ns | 1541274.3ns | 0.0% |  |
| wide-rung-ragged-overread | 89.3ns | 1466831.0ns | 0.0% |  |
| wide-rung-wordround | 72.7ns | 1474088.4ns | 0.0% |  |
| wide-rung-wordround-alias | 75.0ns | 1489267.6ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1382506.9-1636129.3 ns)
  1382506.9 |##############
  1395188.0 |
  1407869.1 |
  1420550.3 |
  1433231.4 |
  1445912.5 |
  1458593.6 |
  1471274.7 |
  1483955.9 |########################################
  1496637.0 |########################################
  1509318.1 |###
  1521999.2 |###
  1534680.4 |###
  1547361.5 |
  1560042.6 |
  1572723.7 |
  1585404.8 |
  1598086.0 |
  1610767.1 |
  1623448.2 |
  (4 below, 7 above range)

wide-rung-ragged (n=40, range 1488797.9-1654804.4 ns)
  1488797.9 |########################################
  1497098.2 |#################
  1505398.6 |
  1513698.9 |
  1521999.2 |
  1530299.5 |
  1538599.9 |
  1546900.2 |
  1555200.5 |
  1563500.8 |
  1571801.2 |
  1580101.5 |
  1588401.8 |
  1596702.1 |
  1605002.5 |
  1613302.8 |
  1621603.1 |##
  1629903.4 |
  1638203.8 |##
  1646504.1 |#################
  (5 below, 3 above range)

wide-rung-ragged-overread (n=40, range 1374183.8-1606565.6 ns)
  1374183.8 |################################
  1385802.9 |##################
  1397422.0 |
  1409041.1 |
  1420660.2 |
  1432279.3 |
  1443898.4 |
  1455517.5 |
  1467136.6 |
  1478755.6 |########################################
  1490374.7 |##############
  1501993.8 |###
  1513612.9 |
  1525232.0 |###
  1536851.1 |
  1548470.2 |
  1560089.3 |
  1571708.3 |
  1583327.4 |
  1594946.5 |
  (3 below, 6 above range)

wide-rung-wordround (n=40, range 1379168.1-1530781.1 ns)
  1379168.1 |#############
  1386748.7 |
  1394329.4 |
  1401910.0 |
  1409490.7 |##
  1417071.3 |
  1424652.0 |
  1432232.6 |
  1439813.3 |
  1447393.9 |
  1454974.6 |
  1462555.2 |
  1470135.9 |
  1477716.5 |
  1485297.2 |##################
  1492877.8 |########################################
  1500458.5 |################
  1508039.1 |
  1515619.8 |
  1523200.4 |
  (4 below, 2 above range)

wide-rung-wordround-alias (n=40, range 1376092.3-1639267.5 ns)
  1376092.3 |#################################
  1389251.0 |###
  1402409.8 |
  1415568.6 |
  1428727.3 |
  1441886.1 |
  1455044.9 |
  1468203.6 |
  1481362.4 |########################################
  1494521.1 |#############
  1507679.9 |
  1520838.7 |
  1533997.4 |
  1547156.2 |
  1560315.0 |
  1573473.7 |
  1586632.5 |
  1599791.2 |
  1612950.0 |
  1626108.8 |####################
  (3 below, 4 above range)

```
