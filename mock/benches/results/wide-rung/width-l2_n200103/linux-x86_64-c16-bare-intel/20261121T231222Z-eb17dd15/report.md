# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround shows alternating (throttle bounce) (autocorr -0.78)

wide-rung-wordround's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader wide-rung-wordround-alias vs stability leader wide-rung-wordround (+7% speed for 1.4x steadier)

wide-rung-wordround-alias is fastest (1.26 ms, CV 4.2%); wide-rung-wordround gives up 7.0% median for 1.4x lower variance (CV 2.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 1256394.4 ns median (-7.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.09x (fastest 1256394.4 ns, slowest 1367498.6 ns)

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
| wide-rung-align16 | 1386753ns | 1356004ns | 1340705ns | 1366583ns | 1493308ns | base |
| wide-rung-ragged | 1348591ns | 1367922ns | 1270328ns | 1362596ns | 1384838ns | -2.75% |
| wide-rung-ragged-overread | 1374124ns | 1353885ns | 1314989ns | 1371088ns | 1442368ns | -0.91% |
| wide-rung-wordround | 1357117ns | 1345294ns | 1340105ns | 1345391ns | 1409305ns | -2.14% |
| wide-rung-wordround-alias | 1280149ns | 1256870ns | 1239514ns | 1261285ns | 1377377ns | -7.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1386154ns | 1340028ns | 1492746ns | base | 1.324 |
| wide-rung-ragged | 1348111ns | 1269855ns | 1384292ns | -2.74% | 1.361 |
| wide-rung-ragged-overread | 1373616ns | 1314531ns | 1441949ns | -0.90% | 1.336 |
| wide-rung-wordround | 1356473ns | 1339480ns | 1408671ns | -2.14% | 1.353 |
| wide-rung-wordround-alias | 1279678ns | 1239039ns | 1376835ns | -7.68% | 1.434 |

## Performance model

- Peak throughput: **1.481 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.354 | 91.4% |
| wide-rung-ragged | 1.342 | 90.6% |
| wide-rung-ragged-overread | 1.356 | 91.6% |
| wide-rung-wordround | 1.365 | 92.1% |
| wide-rung-wordround-alias | 1.461 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1386753ns | 1386753ns | base |
| wide-rung-ragged | 1348591ns | 1348591ns | -2.75% |
| wide-rung-ragged-overread | 1374124ns | 1374124ns | -0.91% |
| wide-rung-wordround | 1357117ns | 1357117ns | -2.14% |
| wide-rung-wordround-alias | 1280149ns | 1280149ns | -7.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1355538ns | base | --- | [1349658, 1360455] | --- | --- | --- | --- |
| wide-rung-ragged | 1367499ns | no significant difference | [-86139, +24875]ns | [1353292, 1370886] | no | 1.0000 | 0.8746 | 0 |
| wide-rung-ragged-overread | 1353345ns | no significant difference | [-26644, +51416]ns | [1341190, 1407067] | no | 1.0000 | 1.0000 | 0 |
| wide-rung-wordround | 1344659ns | -4781.4ns (-0.4%) | [-9163, -3451]ns | [1342738, 1346222] | YES | 0.0044 | 0.0022 | 0 |
| wide-rung-wordround-alias | 1256394ns | -96632.8ns (-7.1%) | [-105892, -83476]ns | [1253886, 1260510] | YES | 0.0002 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1340832ns | +2.2% | -1.9% | +0.1% | -6.2% |
| 1 | 2 | 1355474ns | +1.8% | +6.3% | -0.6% | +0.8% |
| 1 | 3 | 1491583ns | -9.3% | -10.0% | -10.0% | -16.2% |
| 1 | 4 | 1356578ns | -6.3% | +3.6% | -0.8% | -8.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.331 | moderate- |
| wide-rung-ragged | 0.081 | ok |
| wide-rung-ragged-overread | -0.721 | HIGH- (thermal bounce) |
| wide-rung-wordround | -0.785 | HIGH- (thermal bounce) |
| wide-rung-wordround-alias | -0.340 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 3/4, lost 0/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 90.4ns | 1386154.1ns | 0.0% |  |
| wide-rung-ragged | 107.5ns | 1348111.1ns | 0.0% |  |
| wide-rung-ragged-overread | 81.6ns | 1373615.7ns | 0.0% |  |
| wide-rung-wordround | 80.8ns | 1356472.8ns | 0.0% |  |
| wide-rung-wordround-alias | 81.0ns | 1279678.4ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1340027.6-1492745.8 ns)
  1340027.6 |############################
  1347663.5 |############################
  1355299.4 |########################################
  1362935.3 |####
  1370571.2 |####
  1378207.2 |
  1385843.1 |
  1393479.0 |
  1401114.9 |
  1408750.8 |
  1416386.7 |
  1424022.6 |
  1431658.6 |
  1439294.5 |
  1446930.4 |
  1454566.3 |
  1462202.2 |
  1469838.1 |
  1477474.0 |
  1485109.9 |########################
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 1269854.8-1384291.9 ns)
  1269854.8 |######################
  1275576.7 |
  1281298.5 |
  1287020.4 |
  1292742.2 |
  1298464.1 |
  1304185.9 |
  1309907.8 |
  1315629.7 |####
  1321351.5 |
  1327073.4 |
  1332795.2 |
  1338517.1 |
  1344238.9 |
  1349960.8 |###################################
  1355682.6 |########
  1361404.5 |
  1367126.3 |########################################
  1372848.2 |#############
  1378570.0 |###############################
  (3 below, 2 above range)

wide-rung-ragged-overread (n=40, range 1314530.9-1441948.5 ns)
  1314530.9 |##############################
  1320901.8 |
  1327272.7 |
  1333643.5 |##########
  1340014.4 |########################################
  1346385.3 |
  1352756.2 |
  1359127.1 |##########
  1365497.9 |
  1371868.8 |#####
  1378239.7 |
  1384610.6 |
  1390981.5 |
  1397352.4 |
  1403723.2 |#########################
  1410094.1 |##########
  1416465.0 |
  1422835.9 |
  1429206.8 |
  1435577.6 |###################################
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 1339479.9-1408671.4 ns)
  1339479.9 |####################################
  1342939.5 |########################################
  1346399.1 |####################
  1349858.6 |##########
  1353318.2 |###
  1356777.8 |
  1360237.4 |
  1363696.9 |
  1367156.5 |
  1370616.1 |
  1374075.7 |
  1377535.2 |
  1380994.8 |
  1384454.4 |
  1387913.9 |###
  1391373.5 |
  1394833.1 |
  1398292.7 |
  1401752.2 |
  1405211.8 |
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1239038.8-1376835.2 ns)
  1239038.8 |########################
  1245928.6 |#########
  1252818.5 |########################################
  1259708.3 |############
  1266598.1 |
  1273487.9 |###
  1280377.7 |
  1287267.6 |
  1294157.4 |
  1301047.2 |
  1307937.0 |
  1314826.8 |
  1321716.7 |
  1328606.5 |
  1335496.3 |
  1342386.1 |
  1349275.9 |
  1356165.8 |###
  1363055.6 |###############
  1369945.4 |###
  (2 below, 2 above range)

```
