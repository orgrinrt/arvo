# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround is fastest but the noisiest (CV 5.7%)

wide-rung-wordround wins on median (1.31 ms) yet has the highest variance (CV 5.7%), while wide-rung-ragged is the steadiest (CV 3.8%, 1.50 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: wide-rung-wordround** at 1306807.1 ns median (-8.3% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.15x (fastest 1306807.1 ns, slowest 1499412.6 ns)

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
| wide-rung-align16 | 1470882ns | 1424997ns | 1404048ns | 1458936ns | 1573553ns | base |
| wide-rung-ragged | 1537893ns | 1500139ns | 1483978ns | 1527813ns | 1622047ns | +4.56% |
| wide-rung-ragged-overread | 1537596ns | 1492858ns | 1486021ns | 1522936ns | 1633149ns | +4.54% |
| wide-rung-wordround | 1349167ns | 1307411ns | 1290134ns | 1333323ns | 1455734ns | -8.27% |
| wide-rung-wordround-alias | 1363414ns | 1353800ns | 1298156ns | 1353850ns | 1457366ns | -7.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1470354ns | 1403522ns | 1573008ns | base | 1.248 |
| wide-rung-ragged | 1537354ns | 1483515ns | 1621563ns | +4.56% | 1.194 |
| wide-rung-ragged-overread | 1537093ns | 1485514ns | 1632714ns | +4.54% | 1.194 |
| wide-rung-wordround | 1348533ns | 1289639ns | 1455005ns | -8.29% | 1.361 |
| wide-rung-wordround-alias | 1362854ns | 1297672ns | 1456900ns | -7.31% | 1.346 |

## Performance model

- Peak throughput: **1.423 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.288 | 90.5% |
| wide-rung-ragged | 1.224 | 86.0% |
| wide-rung-ragged-overread | 1.230 | 86.4% |
| wide-rung-wordround | 1.404 | 98.7% |
| wide-rung-wordround-alias | 1.356 | 95.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1470882ns | 1470882ns | base |
| wide-rung-ragged | 1537893ns | 1537893ns | +4.56% |
| wide-rung-ragged-overread | 1537596ns | 1537596ns | +4.54% |
| wide-rung-wordround | 1349167ns | 1349167ns | -8.27% |
| wide-rung-wordround-alias | 1363414ns | 1363414ns | -7.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1424516ns | base | --- | [1418184, 1507818] | --- | --- | --- | --- |
| wide-rung-ragged | 1499413ns | +80985.9ns (+5.7%) | [+32728, +103770]ns | [1493033, 1553736] | YES (adj: no) | 0.0807 | 0.0807 | 0 |
| wide-rung-ragged-overread | 1492255ns | +65371.1ns (+4.6%) | [+61635, +75834]ns | [1490085, 1527514] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 1306807ns | -118409.5ns (-8.3%) | [-130741, -103454]ns | [1303555, 1394590] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 1353189ns | -109350.1ns (-7.7%) | [-119373, -101317]ns | [1300492, 1403660] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1569868ns | -4.9% | +3.9% | -17.7% | -9.9% |
| 1 | 2 | 1424516ns | +13.6% | +4.4% | -8.4% | -1.5% |
| 1 | 3 | 1427134ns | +8.7% | +4.6% | -5.4% | -8.9% |
| 1 | 4 | 1403666ns | +5.7% | +6.2% | -0.1% | -7.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.065 | ok |
| wide-rung-ragged | -0.271 | moderate- |
| wide-rung-ragged-overread | -0.111 | ok |
| wide-rung-wordround | 0.252 | moderate+ |
| wide-rung-wordround-alias | 0.274 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 1/4, lost 3/4
- **wide-rung-ragged-overread**: won 0/4, lost 4/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 127.4ns | 1470354.2ns | 0.0% |  |
| wide-rung-ragged | 89.8ns | 1537354.5ns | 0.0% |  |
| wide-rung-ragged-overread | 131.1ns | 1537093.1ns | 0.0% |  |
| wide-rung-wordround | 81.7ns | 1348532.8ns | 0.0% |  |
| wide-rung-wordround-alias | 91.5ns | 1362854.1ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1403522.0-1573008.2 ns)
  1403522.0 |################################
  1411996.3 |################
  1420470.6 |####################
  1428945.0 |########
  1437419.3 |####
  1445893.6 |####
  1454367.9 |
  1462842.2 |
  1471316.5 |
  1479790.8 |
  1488265.1 |
  1496739.4 |
  1505213.8 |
  1513688.1 |
  1522162.4 |
  1530636.7 |
  1539111.0 |
  1547585.3 |
  1556059.6 |####
  1564533.9 |########################################
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 1483515.2-1621563.4 ns)
  1483515.2 |######################
  1490417.6 |########################################
  1497320.0 |#################
  1504222.5 |
  1511124.9 |
  1518027.3 |
  1524929.7 |
  1531832.1 |
  1538734.5 |
  1545636.9 |#############
  1552539.3 |#################
  1559441.7 |
  1566344.1 |
  1573246.5 |
  1580148.9 |
  1587051.3 |
  1593953.7 |
  1600856.1 |
  1607758.6 |
  1614661.0 |########################################
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 1485514.1-1632714.5 ns)
  1485514.1 |########################################
  1492874.1 |######
  1500234.1 |
  1507594.1 |
  1514954.1 |
  1522314.2 |
  1529674.2 |
  1537034.2 |
  1544394.2 |
  1551754.3 |##
  1559114.3 |
  1566474.3 |
  1573834.3 |
  1581194.3 |
  1588554.4 |
  1595914.4 |
  1603274.4 |
  1610634.4 |
  1617994.5 |##
  1625354.5 |################
  (4 below, 4 above range)

wide-rung-wordround (n=40, range 1289638.9-1455004.9 ns)
  1289638.9 |########################
  1297907.2 |####################################
  1306175.5 |####################
  1314443.8 |########
  1322712.1 |
  1330980.4 |
  1339248.7 |
  1347517.0 |
  1355785.3 |
  1364053.6 |
  1372321.9 |
  1380590.2 |
  1388858.5 |####
  1397126.8 |########################################
  1405395.1 |####
  1413663.4 |
  1421931.7 |####
  1430200.0 |
  1438468.3 |
  1446736.6 |
  (3 below, 2 above range)

wide-rung-wordround-alias (n=40, range 1297672.3-1456899.7 ns)
  1297672.3 |########################################
  1305633.6 |
  1313595.0 |
  1321556.4 |
  1329517.8 |
  1337479.1 |
  1345440.5 |
  1353401.9 |
  1361363.2 |
  1369324.6 |
  1377286.0 |
  1385247.3 |
  1393208.7 |
  1401170.1 |#######################
  1409131.5 |##############
  1417092.8 |##
  1425054.2 |
  1433015.6 |
  1440976.9 |##
  1448938.3 |
  (3 below, 2 above range)

```
