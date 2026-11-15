# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 68% (significant)

warm-container-native is -2.91 us (68%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 3.2x slower than the field

warm-container-plusone (4.31 us) is 3.2x the fastest (1.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-native, warm-container-kernel) are a dead heat (<1%)

warm-container-native (1.35 us) and warm-container-kernel (1.35 us) differ by 0.09%, inside the noise, even though the wider field spreads 218.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-plusone shows warm-up / thermal drift (autocorr +0.60)

warm-container-plusone's per-pass series has lag-1 autocorrelation +0.60, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-kernel, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (211% apart)

The field splits into a fast tier {warm-container-native, warm-container-kernel, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 211% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.2x the fastest

Fastest warm-container-native (1.35 us) to slowest warm-container-plusone (4.31 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-plusone's edge over baseline is significant but tiny (39 ns, 0.91%)

warm-container-plusone differs from baseline warm-container-headroom by 39 ns (0.91%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-native** at 1352.1 ns median (-68.3% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.19x (fastest 1352.1 ns, slowest 4308.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4399ns | 4355ns | 4349ns | 4358ns | 4573ns | base |
| warm-container-kernel | 1427ns | 1414ns | 1409ns | 1417ns | 1471ns | -67.57% |
| warm-container-minimum | 1448ns | 1438ns | 1429ns | 1438ns | 1496ns | -67.09% |
| warm-container-native | 1429ns | 1414ns | 1409ns | 1420ns | 1478ns | -67.51% |
| warm-container-plusone | 4446ns | 4408ns | 4353ns | 4425ns | 4602ns | +1.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4308ns | 4260ns | 4476ns | base | 3.804 |
| warm-container-kernel | 1366ns | 1350ns | 1410ns | -68.30% | 11.997 |
| warm-container-minimum | 1385ns | 1367ns | 1431ns | -67.86% | 11.833 |
| warm-container-native | 1367ns | 1348ns | 1412ns | -68.27% | 11.986 |
| warm-container-plusone | 4348ns | 4263ns | 4494ns | +0.94% | 3.768 |

## Performance model

- Peak throughput: **12.155 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.841 | 31.6% |
| warm-container-kernel | 12.107 | 99.6% |
| warm-container-minimum | 11.930 | 98.2% |
| warm-container-native | 12.117 | 99.7% |
| warm-container-plusone | 3.803 | 31.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4399ns | 4399ns | base |
| warm-container-kernel | 1427ns | 1427ns | -67.57% |
| warm-container-minimum | 1448ns | 1448ns | -67.09% |
| warm-container-native | 1429ns | 1429ns | -67.51% |
| warm-container-plusone | 4446ns | 4446ns | +1.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4265ns | base | --- | [4264, 4269] | --- | --- | --- | --- |
| warm-container-kernel | 1353ns | -2911.9ns (-68.3%) | [-2915, -2907]ns | [1352, 1358] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1373ns | -2890.9ns (-67.8%) | [-2894, -2888]ns | [1373, 1377] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1352ns | -2915.2ns (-68.3%) | [-2916, -2910]ns | [1350, 1370] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4308ns | +27.3ns (+0.6%) | [+3, +94]ns | [4271, 4381] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 4260ns | -67.8% | -67.9% | -68.2% | +2.2% |
| 2 | 4280ns | -68.0% | -67.8% | -68.5% | +4.3% |
| 3 | 4264ns | -66.9% | -68.3% | -68.3% | +5.3% |
| 4 | 4265ns | -66.6% | -67.1% | -68.4% | +5.2% |
| 5 | 4265ns | -67.1% | -65.6% | -65.7% | +3.9% |
| 6 | 4260ns | -68.3% | -67.8% | -67.8% | +6.0% |
| 7 | 4263ns | -68.3% | -67.9% | -67.8% | +4.9% |
| 8 | 4260ns | -68.2% | -67.8% | -67.8% | +2.8% |
| 9 | 4261ns | -68.4% | -67.8% | -67.9% | +1.1% |
| 10 | 4276ns | -68.3% | -67.9% | -68.2% | +6.1% |
| 11 | 4275ns | -68.5% | -67.3% | -67.9% | +2.5% |
| 12 | 4261ns | -68.2% | -67.9% | -67.8% | -0.0% |
| 13 | 4265ns | -68.2% | -67.8% | -67.8% | +0.0% |
| 14 | 4262ns | -68.3% | -67.8% | -63.5% | +0.4% |
| 15 | 4261ns | -67.9% | -67.8% | -67.6% | +0.1% |
| 16 | 4260ns | -68.2% | -67.8% | -67.1% | +0.0% |
| 17 | 4454ns | -67.5% | -69.2% | -69.2% | -4.2% |
| 18 | 4480ns | -69.8% | -69.3% | -69.3% | -4.9% |
| 19 | 4320ns | -68.7% | -68.3% | -68.3% | -1.3% |
| 20 | 4312ns | -68.6% | -68.2% | -68.2% | -1.2% |
| 21 | 4259ns | -68.1% | -67.8% | -68.3% | +1.8% |
| 22 | 4264ns | -68.3% | -67.5% | -68.3% | +3.8% |
| 23 | 4265ns | -67.5% | -67.6% | -68.4% | +4.5% |
| 24 | 4523ns | -70.0% | -69.5% | -70.2% | -1.6% |
| 25 | 5097ns | -73.5% | -72.6% | -73.5% | -15.8% |
| 26 | 4267ns | -68.3% | -66.7% | -68.4% | -0.0% |
| 27 | 4267ns | -68.3% | -67.7% | -68.3% | +0.4% |
| 28 | 4284ns | -68.4% | -68.0% | -68.5% | -0.3% |
| 29 | 4263ns | -68.3% | -67.9% | -68.4% | +0.2% |
| 30 | 4262ns | -67.9% | -67.8% | -68.3% | +0.1% |
| 31 | 4273ns | -68.4% | -66.1% | -68.4% | +0.8% |
| 32 | 4268ns | -68.4% | -67.9% | -68.3% | +0.9% |
| 33 | 4265ns | -68.4% | -67.8% | -68.4% | +0.0% |
| 34 | 4263ns | -68.3% | -66.7% | -68.4% | +0.0% |
| 35 | 4273ns | -66.9% | -67.8% | -68.4% | +2.2% |
| 36 | 4267ns | -68.3% | -65.0% | -68.4% | +2.9% |
| 37 | 4270ns | -67.3% | -67.7% | -68.0% | -0.1% |
| 38 | 4265ns | -67.3% | -67.8% | -68.3% | +0.1% |
| 39 | 4340ns | -68.9% | -68.3% | -69.0% | +1.9% |
| 40 | 4263ns | -68.1% | -67.7% | -68.4% | +6.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.251 | moderate+ |
| warm-container-kernel | 0.169 | ok |
| warm-container-minimum | -0.042 | ok |
| warm-container-native | 0.176 | ok |
| warm-container-plusone | 0.604 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 8/40, lost 25/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.2ns | 4307.5ns | 0.1% |  |
| warm-container-kernel | 2.3ns | 1365.6ns | 0.2% |  |
| warm-container-minimum | 2.6ns | 1384.6ns | 0.2% |  |
| warm-container-native | 2.6ns | 1366.9ns | 0.2% |  |
| warm-container-plusone | 2.6ns | 4348.1ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 4260.3-4476.3 ns)
   4260.3 |########################################
   4271.1 |########
   4281.9 |#
   4292.7 |
   4303.5 |#
   4314.3 |#
   4325.1 |
   4335.9 |#
   4346.7 |
   4357.5 |
   4368.3 |
   4379.1 |
   4389.9 |
   4400.7 |
   4411.5 |
   4422.3 |
   4433.1 |
   4443.9 |#
   4454.7 |
   4465.5 |
  (3 below, 3 above range)

warm-container-kernel (n=40, range 1349.5-1409.6 ns)
   1349.5 |########################################
   1352.5 |##################
   1355.5 |#####
   1358.5 |#####
   1361.5 |
   1364.5 |##
   1367.5 |##
   1370.5 |#####
   1373.5 |
   1376.6 |
   1379.6 |
   1382.6 |
   1385.6 |##
   1388.6 |
   1391.6 |
   1394.6 |#####
   1397.6 |
   1400.6 |##
   1403.6 |
   1406.6 |
  (2 below, 4 above range)

warm-container-minimum (n=40, range 1366.9-1430.9 ns)
   1366.9 |##################
   1370.1 |########################################
   1373.3 |#############################
   1376.5 |##############
   1379.7 |###
   1382.9 |#######
   1386.1 |
   1389.3 |
   1392.5 |
   1395.7 |#######
   1398.9 |
   1402.1 |###
   1405.3 |
   1408.5 |
   1411.7 |
   1414.9 |
   1418.1 |#######
   1421.3 |
   1424.5 |
   1427.7 |
  (1 below, 3 above range)

warm-container-native (n=40, range 1348.0-1412.3 ns)
   1348.0 |########################################
   1351.2 |################
   1354.4 |######
   1357.6 |###
   1360.8 |
   1364.1 |###
   1367.3 |######
   1370.5 |#######################
   1373.7 |######
   1376.9 |
   1380.1 |###
   1383.4 |
   1386.6 |
   1389.8 |
   1393.0 |
   1396.2 |
   1399.5 |###
   1402.7 |
   1405.9 |
   1409.1 |
  (4 below, 2 above range)

warm-container-plusone (n=40, range 4263.0-4494.1 ns)
   4263.0 |########################################
   4274.5 |######
   4286.1 |###
   4297.6 |######
   4309.2 |###
   4320.8 |
   4332.3 |###
   4343.9 |###
   4355.4 |
   4367.0 |###
   4378.5 |######
   4390.1 |###
   4401.7 |
   4413.2 |###
   4424.8 |######
   4436.3 |
   4447.9 |######
   4459.4 |###
   4471.0 |###
   4482.6 |######
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-plusone**: autocorrelation=0.60 (measurement drift or warm-up artifact)
