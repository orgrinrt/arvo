# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-contend-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-contend-d16 has the worst median (1.60 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-contend-pipe2 at 1.15 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-contend-pipe2 beats baseline by 23% (significant)

bitpack-contend-pipe2 is -366.85 us (23%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-contend-pipe2, bitpack-contend-packed-simd) are a dead heat (<1%)

bitpack-contend-pipe2 (1.15 ms) and bitpack-contend-packed-simd (1.16 ms) differ by 0.69%, inside the noise, even though the wider field spreads 39.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-pipe4 shows alternating (throttle bounce) (autocorr -0.55)

bitpack-contend-pipe4's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader bitpack-contend-pipe2 vs stability leader bitpack-contend-packed-simd (+1% speed for 1.1x steadier)

bitpack-contend-pipe2 is fastest (1.15 ms, CV 4.7%); bitpack-contend-packed-simd gives up 0.7% median for 1.1x lower variance (CV 4.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-contend-pipe2** at 1149002.5 ns median (-28.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.40x (fastest 1149002.5 ns, slowest 1603363.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112755232 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1533225ns | 1604058ns | 1418353ns | 1544649ns | 1613825ns | base |
| bitpack-contend-d16-control | 1478349ns | 1426137ns | 1421641ns | 1452322ns | 1613141ns | -3.58% |
| bitpack-contend-packed-simd | 1181929ns | 1157350ns | 1148617ns | 1165693ns | 1263950ns | -22.91% |
| bitpack-contend-pipe2 | 1182224ns | 1149519ns | 1140702ns | 1169291ns | 1262546ns | -22.89% |
| bitpack-contend-pipe4 | 1228256ns | 1261766ns | 1149610ns | 1235073ns | 1286450ns | -19.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 1532563ns | 1417675ns | 1613185ns | base | 5.474 |
| bitpack-contend-d16-control | 1477714ns | 1421031ns | 1612544ns | -3.58% | 5.677 |
| bitpack-contend-packed-simd | 1181527ns | 1148193ns | 1263622ns | -22.91% | 7.100 |
| bitpack-contend-pipe2 | 1181769ns | 1140251ns | 1262095ns | -22.89% | 7.098 |
| bitpack-contend-pipe4 | 1227875ns | 1149208ns | 1286105ns | -19.88% | 6.832 |

## Performance model

- Peak throughput: **7.357 Gops/s** (bitpack-contend-pipe2; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 5.232 | 71.1% |
| bitpack-contend-d16-control | 5.885 | 80.0% |
| bitpack-contend-packed-simd | 7.251 | 98.6% |
| bitpack-contend-pipe2 | 7.301 | 99.2% |
| bitpack-contend-pipe4 | 6.650 | 90.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 1533225ns | 1533225ns | base |
| bitpack-contend-d16-control | 1478349ns | 1478349ns | -3.58% |
| bitpack-contend-packed-simd | 1181929ns | 1181929ns | -22.91% |
| bitpack-contend-pipe2 | 1182224ns | 1182224ns | -22.89% |
| bitpack-contend-pipe4 | 1228256ns | 1228256ns | -19.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1603363ns | base | --- | [1420284, 1606214] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 1425512ns | no significant difference | [-25272, +6221]ns | [1423707, 1431102] | no | 0.2682 | 0.2682 | 0 |
| bitpack-contend-packed-simd | 1156881ns | -341155.0ns (-21.3%) | [-374589, -263474]ns | [1156268, 1157923] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 1149003ns | -366847.1ns (-22.9%) | [-458562, -280497]ns | [1143649, 1151418] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 1261403ns | -323566.5ns (-20.2%) | [-329605, -265830]ns | [1153994, 1272141] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 1 | 1419883ns | +0.5% | -18.5% | -19.7% | -11.2% |
| 1 | 2 | 1604229ns | +0.5% | -21.3% | -28.7% | -24.4% |
| 1 | 3 | 1608525ns | -11.5% | -28.6% | -28.4% | -20.1% |
| 1 | 4 | 1418038ns | +0.4% | -18.4% | -11.1% | -18.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.250 | moderate- |
| bitpack-contend-d16-control | -0.408 | moderate- |
| bitpack-contend-packed-simd | -0.431 | moderate- |
| bitpack-contend-pipe2 | -0.027 | ok |
| bitpack-contend-pipe4 | -0.552 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 3/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe2**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 95.9ns | 1532563.1ns | 0.0% |  |
| bitpack-contend-d16-control | 88.8ns | 1477713.8ns | 0.0% |  |
| bitpack-contend-packed-simd | 55.0ns | 1181526.7ns | 0.0% |  |
| bitpack-contend-pipe2 | 47.0ns | 1181768.7ns | 0.0% |  |
| bitpack-contend-pipe4 | 45.8ns | 1227874.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1417674.6-1613185.2 ns)
  1417674.6 |############################
  1427450.2 |
  1437225.7 |
  1447001.2 |
  1456776.8 |
  1466552.3 |##
  1476327.8 |
  1486103.3 |
  1495878.9 |
  1505654.4 |
  1515429.9 |
  1525205.5 |
  1534981.0 |
  1544756.5 |
  1554532.1 |
  1564307.6 |##
  1574083.1 |
  1583858.7 |
  1593634.2 |#######
  1603409.7 |########################################
  (3 below, 3 above range)

bitpack-contend-d16-control (n=40, range 1421031.4-1612543.5 ns)
  1421031.4 |########################################
  1430607.0 |#####
  1440182.6 |
  1449758.2 |
  1459333.8 |
  1468909.4 |
  1478485.0 |
  1488060.7 |
  1497636.3 |#
  1507211.9 |
  1516787.5 |
  1526363.1 |
  1535938.7 |
  1545514.3 |
  1555089.9 |
  1564665.5 |
  1574241.1 |
  1583816.7 |
  1593392.3 |#
  1602967.9 |##############
  (3 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 1148192.7-1263622.1 ns)
  1148192.7 |###############
  1153964.1 |########################################
  1159735.6 |####
  1165507.1 |
  1171278.5 |
  1177050.0 |
  1182821.5 |
  1188592.9 |
  1194364.4 |
  1200135.9 |
  1205907.4 |
  1211678.8 |
  1217450.3 |
  1223221.8 |
  1228993.2 |
  1234764.7 |
  1240536.2 |
  1246307.7 |
  1252079.1 |
  1257850.6 |#############
  (3 below, 4 above range)

bitpack-contend-pipe2 (n=40, range 1140251.2-1262095.4 ns)
  1140251.2 |########################################
  1146343.4 |#########################
  1152435.6 |
  1158527.8 |
  1164620.0 |
  1170712.2 |
  1176804.5 |
  1182896.7 |
  1188988.9 |
  1195081.1 |
  1201173.3 |
  1207265.5 |
  1213357.7 |
  1219449.9 |
  1225542.1 |##
  1231634.3 |
  1237726.6 |
  1243818.8 |
  1249911.0 |##
  1256003.2 |######################
  (4 below, 3 above range)

bitpack-contend-pipe4 (n=40, range 1149207.7-1286105.4 ns)
  1149207.7 |########################################
  1156052.6 |
  1162897.5 |
  1169742.3 |
  1176587.2 |
  1183432.1 |
  1190277.0 |
  1197121.9 |
  1203966.8 |
  1210811.6 |
  1217656.5 |
  1224501.4 |
  1231346.3 |
  1238191.2 |
  1245036.1 |
  1251880.9 |
  1258725.8 |################################
  1265570.7 |#######
  1272415.6 |##############
  1279260.5 |#############################
  (4 below, 2 above range)

```
