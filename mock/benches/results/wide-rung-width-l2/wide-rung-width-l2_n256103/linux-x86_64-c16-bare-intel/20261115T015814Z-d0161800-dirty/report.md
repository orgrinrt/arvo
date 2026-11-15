# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (1.50 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround at 1.49 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-align16 shows alternating (throttle bounce) (autocorr -0.78)

wide-rung-align16's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5.90 us) is smaller than the fastest variant's own run-to-run std-dev (8.84 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.4% of the fastest

All 5 variants sit between 1.49 ms and 1.50 ms - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-wordround** at 1490071.8 ns median (-0.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 1490071.8 ns, slowest 1495972.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1498436ns | 1496390ns | 1486639ns | 1495373ns | 1519425ns | base |
| wide-rung-ragged | 1456014ns | 1495765ns | 1380074ns | 1455046ns | 1534860ns | -2.83% |
| wide-rung-ragged-overread | 1484787ns | 1491437ns | 1385777ns | 1486144ns | 1579726ns | -0.91% |
| wide-rung-wordround | 1493686ns | 1490551ns | 1486393ns | 1491298ns | 1508141ns | -0.32% |
| wide-rung-wordround-alias | 1502333ns | 1491336ns | 1377171ns | 1500497ns | 1633004ns | +0.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1498029ns | 1486242ns | 1518979ns | base | 1.225 |
| wide-rung-ragged | 1455447ns | 1379552ns | 1534276ns | -2.84% | 1.261 |
| wide-rung-ragged-overread | 1484270ns | 1385266ns | 1579139ns | -0.92% | 1.236 |
| wide-rung-wordround | 1493225ns | 1485965ns | 1507476ns | -0.32% | 1.229 |
| wide-rung-wordround-alias | 1501780ns | 1376638ns | 1632536ns | +0.25% | 1.222 |

## Performance model

- Peak throughput: **1.333 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.227 | 92.0% |
| wide-rung-ragged | 1.227 | 92.1% |
| wide-rung-ragged-overread | 1.231 | 92.3% |
| wide-rung-wordround | 1.231 | 92.4% |
| wide-rung-wordround-alias | 1.231 | 92.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1498436ns | 1498436ns | base |
| wide-rung-ragged | 1456014ns | 1456014ns | -2.83% |
| wide-rung-ragged-overread | 1484787ns | 1484787ns | -0.91% |
| wide-rung-wordround | 1493686ns | 1493686ns | -0.32% |
| wide-rung-wordround-alias | 1502333ns | 1502333ns | +0.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1495973ns | base | --- | [1487386, 1501111] | --- | --- | --- | --- |
| wide-rung-ragged | 1495172ns | no significant difference | [-102377, +326]ns | [1384353, 1495685] | no | 0.1614 | 0.0807 | 0 |
| wide-rung-ragged-overread | 1490978ns | no significant difference | [-5502, +955]ns | [1486627, 1495859] | no | 0.3576 | 0.2682 | 0 |
| wide-rung-wordround | 1490072ns | -4929.6ns (-0.3%) | [-11401, -275]ns | [1489094, 1491586] | YES (adj: no) | 0.1539 | 0.0385 | 0 |
| wide-rung-wordround-alias | 1490695ns | no significant difference | [-13140, +1504]ns | [1487471, 1496870] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1486689ns | +0.8% | +0.0% | -0.0% | +9.6% |
| 1 | 2 | 1506893ns | -8.3% | -8.0% | -1.2% | -0.8% |
| 1 | 3 | 1487159ns | -6.9% | +3.3% | +0.9% | +0.0% |
| 1 | 4 | 1500711ns | -0.4% | -0.3% | -0.7% | -8.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.784 | HIGH- (thermal bounce) |
| wide-rung-ragged | -0.250 | moderate- |
| wide-rung-ragged-overread | -0.419 | moderate- |
| wide-rung-wordround | -0.180 | ok |
| wide-rung-wordround-alias | 0.030 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 2/4, lost 1/4
- **wide-rung-wordround**: won 2/4, lost 1/4
- **wide-rung-wordround-alias**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 96.9ns | 1498029.4ns | 0.0% |  |
| wide-rung-ragged | 124.9ns | 1455446.6ns | 0.0% |  |
| wide-rung-ragged-overread | 70.0ns | 1484269.7ns | 0.0% |  |
| wide-rung-wordround | 78.2ns | 1493225.1ns | 0.0% |  |
| wide-rung-wordround-alias | 72.9ns | 1501780.4ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1486241.7-1518978.7 ns)
  1486241.7 |########################################
  1487878.6 |##########
  1489515.4 |
  1491152.3 |###
  1492789.1 |
  1494426.0 |
  1496062.8 |
  1497699.7 |
  1499336.5 |####################
  1500973.4 |######
  1502610.2 |######
  1504247.1 |###
  1505883.9 |#############
  1507520.8 |###
  1509157.6 |###
  1510794.5 |###
  1512431.3 |
  1514068.2 |###
  1515705.0 |
  1517341.9 |
  (4 below, 1 above range)

wide-rung-ragged (n=40, range 1379552.1-1534276.4 ns)
  1379552.1 |########################################
  1387288.4 |###
  1395024.6 |
  1402760.8 |
  1410497.0 |
  1418233.2 |
  1425969.4 |
  1433705.6 |
  1441441.9 |
  1449178.1 |
  1456914.3 |
  1464650.5 |
  1472386.7 |
  1480122.9 |
  1487859.1 |################################
  1495595.4 |#############################
  1503331.6 |#######
  1511067.8 |
  1518804.0 |
  1526540.2 |###
  (5 below, 3 above range)

wide-rung-ragged-overread (n=40, range 1385265.6-1579138.6 ns)
  1385265.6 |###########
  1394959.2 |
  1404652.9 |
  1414346.5 |
  1424040.2 |
  1433733.8 |
  1443427.5 |##
  1453121.1 |
  1462814.8 |
  1472508.4 |
  1482202.1 |############################
  1491895.7 |########################################
  1501589.4 |##
  1511283.0 |
  1520976.7 |
  1530670.3 |
  1540364.0 |
  1550057.6 |
  1559751.3 |
  1569444.9 |##
  (5 below, 4 above range)

wide-rung-wordround (n=40, range 1485965.0-1507475.6 ns)
  1485965.0 |#############
  1487040.5 |#############
  1488116.0 |########################################
  1489191.6 |########################################
  1490267.1 |#################################
  1491342.6 |######
  1492418.2 |
  1493493.7 |#############
  1494569.2 |######
  1495644.8 |#############
  1496720.3 |######
  1497795.8 |######
  1498871.3 |
  1499946.9 |
  1501022.4 |######
  1502097.9 |#############
  1503173.5 |
  1504249.0 |
  1505324.5 |
  1506400.0 |
  (5 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1376637.9-1632535.9 ns)
  1376637.9 |########################
  1389432.8 |
  1402227.7 |
  1415022.6 |
  1427817.5 |
  1440612.4 |
  1453407.3 |
  1466202.2 |
  1478997.1 |########################################
  1491792.0 |################################
  1504586.9 |####
  1517381.8 |
  1530176.7 |
  1542971.6 |
  1555766.5 |
  1568561.4 |
  1581356.3 |
  1594151.2 |
  1606946.1 |
  1619741.0 |################################
  (4 below, 3 above range)

```
