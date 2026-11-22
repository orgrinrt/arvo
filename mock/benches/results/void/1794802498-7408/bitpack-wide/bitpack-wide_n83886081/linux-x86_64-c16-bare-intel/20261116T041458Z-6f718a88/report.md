# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-wide-pipe4 dominates: 20% faster than the next best (bitpack-wide-d16-control)

bitpack-wide-pipe4 (1.18 ms) leads bitpack-wide-d16-control (1.42 ms) by 20%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-d16-control shows alternating (throttle bounce) (autocorr -0.77)

bitpack-wide-d16-control's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-wide-pipe4** at 1180952.1 ns median (-16.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.20x (fastest 1180952.1 ns, slowest 1421536.1 ns)

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
| Counter frequency | 2112726093 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 1488548ns | 1421014ns | 1415901ns | 1422508ns | 1759315ns | base |
| bitpack-wide-d16-control | 1458659ns | 1420128ns | 1417915ns | 1425162ns | 1599896ns | -2.01% |
| bitpack-wide-d16-padal | 1536217ns | 1422277ns | 1418153ns | 1488813ns | 1796495ns | +3.20% |
| bitpack-wide-pipe4 | 1175378ns | 1181457ns | 1146644ns | 1177012ns | 1199210ns | -21.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 1487670ns | 1414858ns | 1758521ns | base | 5.639 |
| bitpack-wide-d16-control | 1457937ns | 1417092ns | 1599140ns | -2.00% | 5.754 |
| bitpack-wide-d16-padal | 1535592ns | 1417498ns | 1795959ns | +3.22% | 5.463 |
| bitpack-wide-pipe4 | 1174839ns | 1146190ns | 1198565ns | -21.03% | 7.140 |

## Performance model

- Peak throughput: **7.319 Gops/s** (bitpack-wide-pipe4; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 5.907 | 80.7% |
| bitpack-wide-d16-control | 5.910 | 80.7% |
| bitpack-wide-d16-padal | 5.901 | 80.6% |
| bitpack-wide-pipe4 | 7.103 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 1488548ns | 1488548ns | base |
| bitpack-wide-d16-control | 1458659ns | 1458659ns | -2.01% |
| bitpack-wide-d16-padal | 1536217ns | 1536217ns | +3.20% |
| bitpack-wide-pipe4 | 1175378ns | 1175378ns | -21.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 1420047ns | base | --- | [1417205, 1425621] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 1419449ns | no significant difference | [-5572, +3144]ns | [1418357, 1421042] | no | 0.4296 | 0.4296 | 0 |
| bitpack-wide-d16-padal | 1421536ns | +2021.5ns (+0.1%) | [+596, +188572]ns | [1419323, 1603166] | YES (adj: no) | 0.0577 | 0.0385 | 0 |
| bitpack-wide-pipe4 | 1180952ns | -241168.4ns (-17.0%) | [-278383, -235747]ns | [1173751, 1181940] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|---|
| 1 | 1 | 1676557ns | -15.4% | -15.4% | -29.5% |
| 1 | 2 | 1415056ns | +0.4% | +13.4% | -16.5% |
| 1 | 3 | 1426286ns | -0.5% | +19.1% | -19.6% |
| 1 | 4 | 1417205ns | +0.2% | +0.1% | -16.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | -0.110 | ok |
| bitpack-wide-d16-control | -0.774 | HIGH- (thermal bounce) |
| bitpack-wide-d16-padal | -0.270 | moderate- |
| bitpack-wide-pipe4 | -0.455 | moderate- |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 2/4, lost 2/4
- **bitpack-wide-d16-padal**: won 1/4, lost 2/4
- **bitpack-wide-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 69.6ns | 1487669.9ns | 0.0% |  |
| bitpack-wide-d16-control | 57.1ns | 1457937.2ns | 0.0% |  |
| bitpack-wide-d16-padal | 38.1ns | 1535591.8ns | 0.0% |  |
| bitpack-wide-pipe4 | 65.6ns | 1174839.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 1414857.6-1758520.6 ns)
  1414857.6 |########################################
  1432040.7 |#
  1449223.9 |
  1466407.0 |
  1483590.2 |
  1500773.3 |
  1517956.5 |
  1535139.6 |
  1552322.8 |#
  1569505.9 |
  1586689.1 |
  1603872.2 |
  1621055.4 |
  1638238.6 |
  1655421.7 |
  1672604.9 |####
  1689788.0 |
  1706971.2 |
  1724154.3 |
  1741337.5 |#
  (3 below, 3 above range)

bitpack-wide-d16-control (n=40, range 1417091.6-1599140.0 ns)
  1417091.6 |########################################
  1426194.0 |
  1435296.4 |
  1444398.8 |
  1453501.2 |
  1462603.7 |
  1471706.1 |
  1480808.5 |
  1489910.9 |
  1499013.3 |
  1508115.8 |
  1517218.2 |
  1526320.6 |#
  1535423.0 |
  1544525.4 |#
  1553627.9 |
  1562730.3 |
  1571832.7 |
  1580935.1 |
  1590037.5 |
  (2 below, 7 above range)

bitpack-wide-d16-padal (n=40, range 1417498.2-1795959.5 ns)
  1417498.2 |########################################
  1436421.3 |
  1455344.3 |
  1474267.4 |
  1493190.4 |
  1512113.5 |
  1531036.6 |
  1549959.6 |
  1568882.7 |##
  1587805.8 |#####################
  1606728.8 |
  1625651.9 |
  1644575.0 |
  1663498.0 |
  1682421.1 |##
  1701344.1 |##
  1720267.2 |
  1739190.3 |
  1758113.3 |
  1777036.4 |
  (4 below, 4 above range)

bitpack-wide-pipe4 (n=40, range 1146190.0-1198564.8 ns)
  1146190.0 |################
  1148808.7 |
  1151427.5 |
  1154046.2 |
  1156665.0 |###
  1159283.7 |
  1161902.4 |
  1164521.2 |######
  1167139.9 |
  1169758.7 |###
  1172377.4 |######
  1174996.1 |######
  1177614.9 |###
  1180233.6 |########################################
  1182852.4 |####################
  1185471.1 |###
  1188089.8 |
  1190708.6 |
  1193327.3 |
  1195946.1 |
  (4 below, 3 above range)

```
