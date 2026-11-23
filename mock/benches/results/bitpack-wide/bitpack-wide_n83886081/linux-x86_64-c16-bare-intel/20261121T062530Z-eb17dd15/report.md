# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-wide-pipe4 dominates: 24% faster than the next best (bitpack-wide-d16-control)

bitpack-wide-pipe4 (1.14 ms) leads bitpack-wide-d16-control (1.42 ms) by 24%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-pipe4 shows alternating (throttle bounce) (autocorr -0.50)

bitpack-wide-pipe4's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-wide-pipe4** at 1141850.3 ns median (-19.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.24x (fastest 1141850.3 ns, slowest 1420302.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112815569 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 1465994ns | 1420710ns | 1415814ns | 1436356ns | 1605088ns | base |
| bitpack-wide-d16-control | 1431666ns | 1420113ns | 1417792ns | 1420157ns | 1480064ns | -2.34% |
| bitpack-wide-d16-padal | 1429635ns | 1421284ns | 1418365ns | 1421389ns | 1465644ns | -2.48% |
| bitpack-wide-pipe4 | 1164065ns | 1142238ns | 1136131ns | 1143366ns | 1254099ns | -20.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 1465248ns | 1414860ns | 1604396ns | base | 5.725 |
| bitpack-wide-d16-control | 1430934ns | 1416999ns | 1479321ns | -2.34% | 5.862 |
| bitpack-wide-d16-padal | 1428834ns | 1417550ns | 1464888ns | -2.49% | 5.871 |
| bitpack-wide-pipe4 | 1163637ns | 1135680ns | 1253674ns | -20.58% | 7.209 |

## Performance model

- Peak throughput: **7.386 Gops/s** (bitpack-wide-pipe4; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 5.907 | 80.0% |
| bitpack-wide-d16-control | 5.910 | 80.0% |
| bitpack-wide-d16-padal | 5.906 | 80.0% |
| bitpack-wide-pipe4 | 7.347 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 1465994ns | 1465994ns | base |
| bitpack-wide-d16-control | 1431666ns | 1431666ns | -2.34% |
| bitpack-wide-d16-padal | 1429635ns | 1429635ns | -2.48% |
| bitpack-wide-pipe4 | 1164065ns | 1164065ns | -20.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 1420026ns | base | --- | [1418039, 1422817] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 1419454ns | no significant difference | [-5091, +2546]ns | [1418577, 1420318] | no | 0.8746 | 0.8746 | 0 |
| bitpack-wide-d16-padal | 1420303ns | no significant difference | [-2578, +3283]ns | [1419192, 1421384] | no | 0.8746 | 0.8746 | 0 |
| bitpack-wide-pipe4 | 1141850ns | -279788.0ns (-19.7%) | [-280788, -276540]ns | [1140008, 1143061] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|---|
| 1 | 1 | 1418039ns | +0.0% | +0.0% | -19.4% |
| 1 | 2 | 1414806ns | +0.3% | +0.5% | -19.7% |
| 1 | 3 | 1420468ns | +0.0% | +0.1% | -18.6% |
| 1 | 4 | 1603694ns | -11.4% | -11.5% | -28.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | -0.064 | ok |
| bitpack-wide-d16-control | 0.173 | ok |
| bitpack-wide-d16-padal | -0.284 | moderate- |
| bitpack-wide-pipe4 | -0.502 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 1/4, lost 1/4
- **bitpack-wide-d16-padal**: won 1/4, lost 1/4
- **bitpack-wide-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 78.2ns | 1465247.8ns | 0.0% |  |
| bitpack-wide-d16-control | 108.7ns | 1430934.4ns | 0.0% |  |
| bitpack-wide-d16-padal | 106.6ns | 1428834.2ns | 0.0% |  |
| bitpack-wide-pipe4 | 71.7ns | 1163636.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 1414860.0-1604396.0 ns)
  1414860.0 |########################################
  1424336.8 |#####
  1433813.6 |
  1443290.4 |
  1452767.2 |
  1462244.0 |
  1471720.8 |
  1481197.6 |
  1490674.4 |
  1500151.2 |
  1509628.0 |
  1519104.8 |
  1528581.6 |
  1538058.4 |
  1547535.2 |
  1557012.0 |
  1566488.8 |
  1575965.6 |
  1585442.4 |
  1594919.2 |###########
  (6 below, 4 above range)

bitpack-wide-d16-control (n=40, range 1416998.8-1479320.8 ns)
  1416998.8 |########################################
  1420114.9 |###################
  1423231.0 |###
  1426347.1 |
  1429463.2 |#
  1432579.3 |
  1435695.4 |
  1438811.5 |
  1441927.6 |
  1445043.7 |
  1448159.8 |
  1451275.9 |
  1454392.0 |
  1457508.1 |
  1460624.2 |
  1463740.3 |
  1466856.4 |
  1469972.5 |
  1473088.6 |
  1476204.7 |
  (3 below, 3 above range)

bitpack-wide-d16-padal (n=40, range 1417550.1-1464888.3 ns)
  1417550.1 |########################################
  1419917.0 |############################
  1422283.9 |###########
  1424650.8 |##############
  1427017.7 |
  1429384.6 |
  1431751.5 |
  1434118.5 |
  1436485.4 |
  1438852.3 |
  1441219.2 |#####
  1443586.1 |
  1445953.0 |
  1448319.9 |
  1450686.9 |
  1453053.8 |
  1455420.7 |
  1457787.6 |
  1460154.5 |
  1462521.4 |
  (4 below, 1 above range)

bitpack-wide-pipe4 (n=40, range 1135679.9-1253674.4 ns)
  1135679.9 |########################################
  1141579.6 |##########################
  1147479.3 |#####
  1153379.0 |
  1159278.8 |##
  1165178.5 |
  1171078.2 |
  1176977.9 |
  1182877.7 |
  1188777.4 |##
  1194677.1 |
  1200576.8 |
  1206476.6 |
  1212376.3 |
  1218276.0 |
  1224175.7 |
  1230075.5 |
  1235975.2 |
  1241874.9 |
  1247774.6 |#####
  (4 below, 5 above range)

```
