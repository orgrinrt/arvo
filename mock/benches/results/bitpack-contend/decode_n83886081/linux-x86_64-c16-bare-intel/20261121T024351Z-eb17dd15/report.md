# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (bitpack-contend-packed-simd, bitpack-contend-pipe2) are a dead heat (<1%)

bitpack-contend-packed-simd (1.16 ms) and bitpack-contend-pipe2 (1.16 ms) differ by 0.18%, inside the noise, even though the wider field spreads 23.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-pipe2 shows alternating (throttle bounce) (autocorr -0.54)

bitpack-contend-pipe2's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-contend-packed-simd** at 1155169.4 ns median (-18.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.23x (fastest 1155169.4 ns, slowest 1424206.0 ns)

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
| Counter frequency | 2112791945 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1437680ns | 1422199ns | 1418120ns | 1423323ns | 1500313ns | base |
| bitpack-contend-d16-control | 1505179ns | 1424855ns | 1419727ns | 1445009ns | 1771143ns | +4.69% |
| bitpack-contend-packed-simd | 1181548ns | 1155644ns | 1148054ns | 1166046ns | 1261549ns | -17.82% |
| bitpack-contend-pipe2 | 1185137ns | 1157681ns | 1145436ns | 1170987ns | 1267289ns | -17.57% |
| bitpack-contend-pipe4 | 1185473ns | 1170684ns | 1150836ns | 1173868ns | 1254925ns | -17.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 1436818ns | 1417063ns | 1499554ns | base | 5.838 |
| bitpack-contend-d16-control | 1504452ns | 1418997ns | 1770458ns | +4.71% | 5.576 |
| bitpack-contend-packed-simd | 1181133ns | 1147694ns | 1261174ns | -17.80% | 7.102 |
| bitpack-contend-pipe2 | 1184697ns | 1144996ns | 1266907ns | -17.55% | 7.081 |
| bitpack-contend-pipe4 | 1184997ns | 1150370ns | 1254497ns | -17.53% | 7.079 |

## Performance model

- Peak throughput: **7.326 Gops/s** (bitpack-contend-pipe2; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 5.902 | 80.6% |
| bitpack-contend-d16-control | 5.890 | 80.4% |
| bitpack-contend-packed-simd | 7.262 | 99.1% |
| bitpack-contend-pipe2 | 7.249 | 98.9% |
| bitpack-contend-pipe4 | 7.168 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 1437680ns | 1437680ns | base |
| bitpack-contend-d16-control | 1505179ns | 1505179ns | +4.69% |
| bitpack-contend-packed-simd | 1181548ns | 1181548ns | -17.82% |
| bitpack-contend-pipe2 | 1185137ns | 1185137ns | -17.57% |
| bitpack-contend-pipe4 | 1185473ns | 1185473ns | -17.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1421333ns | base | --- | [1419895, 1425556] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 1424206ns | +2561.7ns (+0.2%) | [+743, +4794]ns | [1422531, 1432267] | YES | 0.0385 | 0.0385 | 0 |
| bitpack-contend-packed-simd | 1155169ns | -266623.1ns (-18.8%) | [-270769, -263824]ns | [1153753, 1160892] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 1157218ns | -264211.7ns (-18.6%) | [-269930, -249021]ns | [1153438, 1179310] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 1170267ns | -258711.9ns (-18.2%) | [-269233, -238825]ns | [1160254, 1188649] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 1 | 1418045ns | +0.3% | -19.0% | -17.5% | -17.1% |
| 1 | 2 | 1429017ns | +18.1% | -19.0% | -19.9% | -18.8% |
| 1 | 3 | 1421868ns | -0.1% | -11.5% | -11.0% | -19.0% |
| 1 | 4 | 1420803ns | +0.2% | -18.7% | -18.7% | -16.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.483 | moderate- |
| bitpack-contend-d16-control | -0.420 | moderate- |
| bitpack-contend-packed-simd | -0.363 | moderate- |
| bitpack-contend-pipe2 | -0.539 | HIGH- (thermal bounce) |
| bitpack-contend-pipe4 | -0.307 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 0/4, lost 3/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe2**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 167.8ns | 1436817.9ns | 0.0% |  |
| bitpack-contend-d16-control | 159.2ns | 1504452.0ns | 0.0% |  |
| bitpack-contend-packed-simd | 94.9ns | 1181133.4ns | 0.0% |  |
| bitpack-contend-pipe2 | 74.7ns | 1184697.5ns | 0.0% |  |
| bitpack-contend-pipe4 | 85.9ns | 1184996.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1417063.2-1499554.5 ns)
  1417063.2 |########################################
  1421187.8 |################
  1425312.3 |#####################
  1429436.9 |#####
  1433561.5 |
  1437686.0 |
  1441810.6 |##
  1445935.2 |
  1450059.7 |
  1454184.3 |
  1458308.8 |
  1462433.4 |
  1466558.0 |
  1470682.5 |
  1474807.1 |
  1478931.6 |##
  1483056.2 |
  1487180.8 |
  1491305.3 |
  1495429.9 |
  (4 below, 3 above range)

bitpack-contend-d16-control (n=40, range 1418997.5-1770457.8 ns)
  1418997.5 |########################################
  1436570.5 |
  1454143.5 |#
  1471716.5 |#
  1489289.5 |#
  1506862.5 |
  1524435.6 |
  1542008.6 |#
  1559581.6 |
  1577154.6 |
  1594727.6 |####
  1612300.7 |
  1629873.7 |
  1647446.7 |
  1665019.7 |
  1682592.7 |###
  1700165.7 |
  1717738.8 |
  1735311.8 |
  1752884.8 |
  (2 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 1147693.7-1261174.4 ns)
  1147693.7 |##############################
  1153367.8 |########################################
  1159041.8 |##########
  1164715.8 |###
  1170389.9 |
  1176063.9 |
  1181737.9 |###
  1187412.0 |
  1193086.0 |
  1198760.0 |
  1204434.0 |
  1210108.1 |
  1215782.1 |
  1221456.1 |
  1227130.2 |
  1232804.2 |
  1238478.2 |
  1244152.3 |
  1249826.3 |
  1255500.3 |#######################
  (4 below, 3 above range)

bitpack-contend-pipe2 (n=40, range 1144996.2-1266906.7 ns)
  1144996.2 |####################
  1151091.7 |########################################
  1157187.3 |########
  1163282.8 |########
  1169378.3 |####
  1175473.8 |####
  1181569.4 |########
  1187664.9 |####
  1193760.4 |####
  1199855.9 |
  1205951.5 |
  1212047.0 |
  1218142.5 |
  1224238.0 |
  1230333.6 |####
  1236429.1 |
  1242524.6 |
  1248620.1 |
  1254715.7 |
  1260811.2 |########################
  (5 below, 3 above range)

bitpack-contend-pipe4 (n=40, range 1150370.1-1254497.0 ns)
  1150370.1 |########################################
  1155576.4 |####################
  1160782.8 |###############
  1165989.1 |####################
  1171195.5 |
  1176401.8 |
  1181608.1 |###############
  1186814.5 |###############
  1192020.8 |#########################
  1197227.2 |
  1202433.5 |
  1207639.9 |
  1212846.2 |
  1218052.6 |#####
  1223258.9 |
  1228465.2 |
  1233671.6 |
  1238877.9 |
  1244084.3 |
  1249290.6 |
  (3 below, 6 above range)

```
